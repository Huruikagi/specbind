//! Read-only version-range catalog and raw project probes (Decision 0211).

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use semver::{Version, VersionReq};
use serde::Serialize;
use walkdir::WalkDir;

use crate::{artifacts::split_frontmatter, description::Description, guarded_fs};

const COVERAGE: &str = "1.5.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Classification {
    Required,
    Recommended,
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Handler {
    DeterministicCli,
    AgentProcedure,
    ManualStop,
}

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub id: &'static str,
    pub boundary: &'static str,
    pub source: &'static str,
    pub classification: Classification,
    pub handler: Handler,
    pub procedure: &'static str,
    pub preview: &'static str,
    pub verification: &'static str,
    pub idempotency: &'static str,
    pub artifact_type: &'static str,
    pub roots: &'static [&'static str],
}

const CATALOG: &[Entry] = &[
    Entry {
        id: "design-description",
        boundary: "1.5.0",
        source: ">=1.0.0, <1.5.0",
        classification: Classification::Recommended,
        handler: Handler::AgentProcedure,
        procedure: "sb-configure/references/migrations/descriptions.md",
        preview: "Read each target and propose its durable technical responsibility; preview template and live edits separately.",
        verification: "Re-run this migration plan and artifact list for every affected Spec; inspect freshness and approval separately.",
        idempotency: "Edit only pending targets; retain existing valid descriptions and never rewrite lifecycle evidence.",
        artifact_type: "SpecBind Design",
        roots: &["specs", "settings/templates/specs"],
    },
    Entry {
        id: "requirements-description",
        boundary: "1.5.0",
        source: ">=1.0.0, <1.5.0",
        classification: Classification::Recommended,
        handler: Handler::AgentProcedure,
        procedure: "sb-configure/references/migrations/descriptions.md",
        preview: "Read complete Requirements and propose the current Spec responsibility; obtain reconciliation authority.",
        verification: "Re-run this migration plan and spec list; inspect freshness and approval separately.",
        idempotency: "Edit only pending targets; retain Requirement identities and existing valid descriptions.",
        artifact_type: "SpecBind Requirements",
        roots: &["specs", "settings/templates/specs"],
    },
    Entry {
        id: "steering-description",
        boundary: "1.5.0",
        source: ">=1.0.0, <1.5.0",
        classification: Classification::Recommended,
        handler: Handler::AgentProcedure,
        procedure: "sb-configure/references/migrations/descriptions.md",
        preview: "Read each target and propose its recurring project-wide responsibility without overlapping other Steering.",
        verification: "Re-run this migration plan and steering list; verify changed templates separately.",
        idempotency: "Edit only pending targets; preserve selectors, paths, and existing valid descriptions.",
        artifact_type: "SpecBind Steering",
        roots: &["steering", "settings/templates/steering"],
    },
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pending,
    Complete,
    NotApplicable,
    Blocked,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedEntry {
    pub id: &'static str,
    pub boundary: &'static str,
    pub source: &'static str,
    pub classification: Classification,
    pub handler: Handler,
    pub procedure: &'static str,
    pub preview: &'static str,
    pub verification: &'static str,
    pub idempotency: &'static str,
    pub status: Status,
    pub targets: Vec<String>,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub from: String,
    pub to: String,
    pub entries: Vec<PlannedEntry>,
    pub required_remaining: usize,
}

/// Computes the same catalog against fresh project state on every invocation.
///
/// # Errors
/// Returns invalid version, unsupported interval, catalog, or raw root faults.
pub fn plan(project_root: &Path, from: &str, to: Option<&str>) -> Result<Plan, String> {
    let to = to.unwrap_or(env!("CARGO_PKG_VERSION"));
    let selected = select(CATALOG, from, to, COVERAGE)?;
    let entries: Vec<_> = if selected.is_empty() {
        vec![]
    } else {
        let root = raw_root(project_root)?;
        selected.iter().map(|entry| probe(&root, entry)).collect()
    };
    let required_remaining = entries
        .iter()
        .filter(|entry| {
            entry.classification == Classification::Required
                && matches!(entry.status, Status::Pending | Status::Blocked)
        })
        .count();
    Ok(Plan {
        from: from.to_owned(),
        to: to.to_owned(),
        entries,
        required_remaining,
    })
}

fn version(input: &str) -> Result<Version, String> {
    let mut parsed =
        Version::parse(input).map_err(|error| format!("invalid SemVer {input}: {error}"))?;
    parsed.build = semver::BuildMetadata::EMPTY;
    Ok(parsed)
}

fn select<'a>(
    catalog: &'a [Entry],
    from: &str,
    to: &str,
    coverage: &str,
) -> Result<Vec<&'a Entry>, String> {
    validate_catalog(catalog)?;
    let from = version(from)?;
    let to = version(to)?;
    if from > to {
        return Err("downgrade migration plans are unsupported".to_owned());
    }
    if to > version(coverage)? {
        return Err(format!(
            "target exceeds migration catalog coverage {coverage}"
        ));
    }
    if from == to {
        return Ok(vec![]);
    }
    let mut entries = Vec::new();
    for entry in catalog {
        let boundary = version(entry.boundary)?;
        let minimum = source_minimum(entry)?;
        if from < boundary && boundary <= to && minimum <= from {
            entries.push(entry);
        }
    }
    for major in (from.major + 1)..=to.major {
        if !entries.iter().any(|entry| {
            version(entry.boundary).is_ok_and(|boundary| boundary.major == major)
                && entry.classification == Classification::Required
        }) {
            return Err(format!("no catalog route covers major boundary {major}"));
        }
    }
    entries.sort_by(|left, right| {
        version(left.boundary)
            .unwrap()
            .cmp(&version(right.boundary).unwrap())
            .then(left.id.cmp(right.id))
    });
    Ok(entries)
}

fn validate_catalog(catalog: &[Entry]) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    for entry in catalog {
        let boundary = version(entry.boundary)?;
        let minimum = source_minimum(entry)?;
        if !crate::artifacts::canonical_id(entry.id)
            || !ids.insert(entry.id)
            || [
                entry.procedure,
                entry.preview,
                entry.verification,
                entry.idempotency,
                entry.artifact_type,
            ]
            .iter()
            .any(|value| value.trim().is_empty())
            || entry.roots.is_empty()
            || entry
                .roots
                .iter()
                .any(|root| crate::config::validate_spec_dir(root).is_err())
            || (entry.classification == Classification::Required
                && (boundary.minor != 0 || boundary.patch != 0 || minimum.major >= boundary.major))
        {
            return Err(format!("invalid migration catalog entry {}", entry.id));
        }
    }
    Ok(())
}

fn source_minimum(entry: &Entry) -> Result<Version, String> {
    let range = VersionReq::parse(entry.source).map_err(|error| error.to_string())?;
    let [lower, upper] = range.comparators.as_slice() else {
        return Err("source range must be >=full-version, <boundary".to_owned());
    };
    let full = |comparator: &semver::Comparator| -> Result<Version, String> {
        Ok(Version {
            major: comparator.major,
            minor: comparator
                .minor
                .ok_or("source range requires a minor version")?,
            patch: comparator
                .patch
                .ok_or("source range requires a patch version")?,
            pre: comparator.pre.clone(),
            build: semver::BuildMetadata::EMPTY,
        })
    };
    let minimum = full(lower)?;
    let maximum = full(upper)?;
    if lower.op != semver::Op::GreaterEq
        || upper.op != semver::Op::Less
        || minimum >= maximum
        || maximum != version(entry.boundary)?
    {
        return Err(
            "source range must be a non-empty >=full-version, <boundary interval".to_owned(),
        );
    }
    Ok(minimum)
}

fn regular(path: &Path, directory: bool) -> Result<(), String> {
    let metadata =
        fs::symlink_metadata(path).map_err(|error| format!("{}: {error}", path.display()))?;
    if if directory {
        guarded_fs::is_regular_dir(&metadata)
    } else {
        guarded_fs::is_regular_file(&metadata)
    } {
        Ok(())
    } else {
        Err(format!(
            "{} must be a regular non-link {}",
            path.display(),
            if directory { "directory" } else { "file" }
        ))
    }
}

fn raw_root(project_root: &Path) -> Result<PathBuf, String> {
    let config_path = project_root.join(".specbind.json");
    regular(&config_path, false)?;
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(config_path).map_err(|error| error.to_string())?)
            .map_err(|error| error.to_string())?;
    let spec_dir = value
        .get("specDir")
        .and_then(serde_json::Value::as_str)
        .ok_or("raw .specbind.json requires a string specDir")?;
    crate::config::validate_spec_dir(spec_dir).map_err(|error| error.to_string())?;
    let mut root = project_root.to_path_buf();
    for part in spec_dir.split('/') {
        root.push(part);
        regular(&root, true)?;
    }
    Ok(root)
}

fn probe(root: &Path, entry: &Entry) -> PlannedEntry {
    let mut targets = BTreeSet::new();
    let mut diagnostics = BTreeSet::new();
    let mut found = false;
    for relative in entry.roots {
        let directory = match probe_directory(root, relative) {
            Ok(Some(directory)) => directory,
            Ok(None) => continue,
            Err(error) => {
                diagnostics.insert(error);
                continue;
            }
        };
        let mut walk = WalkDir::new(&directory)
            .follow_links(false)
            .sort_by_file_name()
            .into_iter();
        while let Some(item) = walk.next() {
            let item = match item {
                Ok(item) => item,
                Err(error) => {
                    diagnostics.insert(error.to_string());
                    continue;
                }
            };
            let path = item.path();
            if item.file_type().is_dir() {
                if let Err(error) = regular(path, true) {
                    diagnostics.insert(error);
                    walk.skip_current_dir();
                }
                continue;
            }
            if let Err(error) = regular(path, false) {
                diagnostics.insert(error);
                continue;
            }
            if path.extension().is_none_or(|extension| extension != "md") {
                continue;
            }
            match inspect_description(path, entry.artifact_type) {
                Ok(None) => {}
                Ok(Some(description)) => {
                    found = true;
                    let relative = path
                        .strip_prefix(root)
                        .expect("walk remains under root")
                        .to_string_lossy()
                        .replace('\\', "/");
                    match description {
                        Description::Missing => {
                            targets.insert(relative);
                        }
                        Description::Present(_) => {}
                        Description::Invalid | Description::Unavailable => {
                            diagnostics.insert(format!(
                                "{relative}: {}",
                                crate::description::INVALID_MESSAGE
                            ));
                        }
                    }
                }
                Err(error) => {
                    diagnostics.insert(format!("{}: {error}", path.display()));
                }
            }
        }
    }
    let status = if !diagnostics.is_empty() {
        Status::Blocked
    } else if !targets.is_empty() {
        Status::Pending
    } else if found {
        Status::Complete
    } else {
        Status::NotApplicable
    };
    PlannedEntry {
        id: entry.id,
        boundary: entry.boundary,
        source: entry.source,
        classification: entry.classification,
        handler: entry.handler,
        procedure: entry.procedure,
        preview: entry.preview,
        verification: entry.verification,
        idempotency: entry.idempotency,
        status,
        targets: targets.into_iter().collect(),
        diagnostics: diagnostics.into_iter().collect(),
    }
}

fn probe_directory(root: &Path, relative: &str) -> Result<Option<PathBuf>, String> {
    let mut directory = root.to_path_buf();
    for part in relative.split('/') {
        directory.push(part);
        if fs::symlink_metadata(&directory)
            .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
        {
            return Ok(None);
        }
        regular(&directory, true)?;
    }
    Ok(Some(directory))
}

fn inspect_description(path: &Path, artifact_type: &str) -> Result<Option<Description>, String> {
    let content = fs::read_to_string(path).map_err(|error| error.to_string())?;
    // Non-OKF auxiliary Markdown such as historical logs is not a candidate.
    if !content.starts_with("---\n") && !content.starts_with("---\r\n") {
        return Ok(None);
    }
    let (frontmatter, _) = split_frontmatter(&content)?;
    let value: serde_json::Value =
        serde_saphyr::from_str(frontmatter).map_err(|error| error.to_string())?;
    let mapping = value.as_object().ok_or("Front Matter must be a mapping")?;
    if mapping.get("type").and_then(serde_json::Value::as_str) != Some(artifact_type) {
        return Ok(None);
    }
    Ok(Some(Description::from_mapping(mapping)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_boundaries_prereleases_build_metadata_and_empty_intervals() {
        for (from, to, count) in [
            ("1.4.4", "1.5.0", 3),
            ("1.4.4", "1.5.0-rc.1", 0),
            ("1.5.0-rc.1", "1.5.0", 3),
            ("1.5.0", "1.5.0+build", 0),
            ("1.4.4+old", "1.5.0+new", 3),
        ] {
            assert_eq!(select(CATALOG, from, to, COVERAGE).unwrap().len(), count);
        }
        for (from, to) in [
            ("v1.4.4", "1.5.0"),
            ("1.5.0", "1.4.4"),
            ("1.4.4", "1.6.0"),
            ("0.9.0", "1.5.0"),
            ("1.04.4", "1.5.0"),
        ] {
            assert!(
                select(CATALOG, from, to, COVERAGE).is_err(),
                "{from} -> {to}"
            );
        }
    }

    #[test]
    fn synthetic_major_routes_are_ordered_and_use_old_format_probes() {
        let mut manual = CATALOG[0];
        manual.id = "major-manual";
        manual.boundary = "2.0.0";
        manual.source = ">=1.0.0, <2.0.0";
        manual.classification = Classification::Required;
        manual.handler = Handler::ManualStop;
        let mut deterministic = manual;
        deterministic.id = "major-cli";
        deterministic.handler = Handler::DeterministicCli;
        let entries = [manual, deterministic];
        let selected = select(&entries, "1.5.0", "2.0.0", "2.0.0").unwrap();
        assert_eq!(
            selected.iter().map(|entry| entry.id).collect::<Vec<_>>(),
            ["major-cli", "major-manual"]
        );
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join("specs/old")).unwrap();
        fs::write(
            root.path().join("specs/old/design.md"),
            "---\ntype: SpecBind Design\n---\nOld unsupported body\n",
        )
        .unwrap();
        for entry in selected {
            assert!(matches!(probe(root.path(), entry).status, Status::Pending));
        }
    }

    #[test]
    fn catalog_rejects_duplicate_ids_missing_procedures_and_required_minor_rewrites() {
        assert!(validate_catalog(&[CATALOG[0], CATALOG[0]]).is_err());
        let mut entry = CATALOG[0];
        entry.procedure = "";
        assert!(validate_catalog(&[entry]).is_err());
        entry = CATALOG[0];
        entry.classification = Classification::Required;
        assert!(validate_catalog(&[entry]).is_err());
        entry = CATALOG[0];
        entry.source = "not semver";
        assert!(validate_catalog(&[entry]).is_err());
        entry.source = ">=2.0.0, <1.5.0";
        assert!(validate_catalog(&[entry]).is_err());
        entry.source = ">=1.0.0, <1.6.0";
        assert!(validate_catalog(&[entry]).is_err());
        for entry in CATALOG {
            let (owner, resource) = entry.procedure.split_once('/').unwrap();
            let skill = crate::skill::find(owner).expect("catalog procedure owner exists");
            assert!(
                skill
                    .resources()
                    .iter()
                    .any(|item| item.relative_path == resource)
            );
        }
    }
}
