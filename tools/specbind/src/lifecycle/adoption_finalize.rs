//! Guarded finalization for a reverse-established baseline.

use std::{
    fmt, fs,
    path::{Component, Path},
};

use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    artifacts::{self, ArtifactKind},
    config::ProjectLanguage,
    domain::spec::Spec,
    guarded_fs,
    milestone_status::{self, DeliveryStage, MilestoneHealth},
    release_log::{self, LogEntryKind, LogUpdate},
    repository,
    schema::{runtime, spec::v1::SpecDocument},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizeOutcome {
    pub baseline_version: String,
    pub specs: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbandonOutcome {
    pub milestone_id: String,
    pub specs_removed: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FinalizeIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizeIssues {
    pub issues: Vec<FinalizeIssue>,
}

struct SpecPlan {
    spec: String,
    spec_yaml: String,
    log_path: String,
    log_update: LogUpdate,
    cleanup: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
struct AdoptionRecord {
    #[serde(default)]
    suspected_defects: Vec<SuspectedDefect>,
}

#[derive(Debug, Deserialize)]
struct SuspectedDefect {
    destination: Option<String>,
}

impl fmt::Display for FinalizeIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "adoption finalization has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for FinalizeIssues {}

/// Explicitly abandons a clean reverse milestone so urgent ordinary work can start.
///
/// # Errors
///
/// Returns identity, lifecycle, repository, path, or filesystem diagnostics.
pub fn abandon(
    project_root: &Path,
    specbind_root: &Path,
    confirmed_milestone_id: &str,
) -> Result<AbandonOutcome, FinalizeIssues> {
    let roadmap = milestone_status::read_roadmap(specbind_root)
        .map_err(|error| FinalizeIssues {
            issues: error
                .diagnostics
                .into_iter()
                .map(|value| issue(value.code, value.path, value.message))
                .collect(),
        })?
        .ok_or_else(|| one_issue("NO_ACTIVE_MILESTONE", None, "no active milestone exists"))?;
    if roadmap.reverse_specs.is_empty() || roadmap.baseline_version.is_none() {
        return Err(one_issue(
            "ADOPTION_REVERSE_REQUIRED",
            Some("steering/roadmap.md".to_owned()),
            "reverse abandon requires an active reverse milestone",
        ));
    }
    if roadmap.milestone_id != confirmed_milestone_id {
        return Err(one_issue(
            "ADOPTION_ABANDON_ID_MISMATCH",
            None,
            "confirmed milestone ID does not match the active reverse milestone",
        ));
    }
    ensure_clean_repository(project_root)?;
    for item in &roadmap.reverse_specs {
        let spec_path = format!("specs/{}/spec.yaml", item.spec);
        let wire =
            runtime::load_spec(&read_regular(specbind_root, &spec_path)?).map_err(|error| {
                one_issue(
                    "ADOPTION_SPEC_INVALID",
                    Some(spec_path.clone()),
                    error.to_string(),
                )
            })?;
        let matches = wire.establishment.as_ref().is_some_and(|value| {
            value.milestone_id.0 == roadmap.milestone_id
                && value.source_revision.0 == roadmap.baseline_revision
        }) && wire
            .active_change
            .0
            .as_ref()
            .is_some_and(|value| value.milestone_id.0 == roadmap.milestone_id);
        if !matches {
            return Err(one_issue(
                "ADOPTION_ABANDON_SPEC_MISMATCH",
                Some(spec_path),
                "reverse Spec does not match the milestone being abandoned",
            ));
        }
    }
    for item in &roadmap.reverse_specs {
        remove_spec_directory(specbind_root, &item.spec)?;
    }
    for relative in [
        "state/contract-review.md",
        "adoption/reverse-discovery.yaml",
    ] {
        if specbind_root.join(relative).exists() {
            remove_regular(specbind_root, relative)?;
        }
    }
    remove_regular(specbind_root, "steering/roadmap.md")?;
    Ok(AbandonOutcome {
        milestone_id: roadmap.milestone_id,
        specs_removed: roadmap.reverse_specs.len(),
    })
}

/// Finalizes a reverse milestone without creating a product release.
///
/// # Errors
///
/// Returns lifecycle, source-drift, log, path, serialization, or filesystem diagnostics.
#[allow(
    clippy::too_many_lines,
    reason = "the guarded plan is fully built before the ordered finalization mutation"
)]
pub fn finalize(
    project_root: &Path,
    specbind_root: &Path,
    language: ProjectLanguage,
    log_entries_json: Option<&str>,
) -> Result<FinalizeOutcome, FinalizeIssues> {
    let status = milestone_status::resolve(project_root, specbind_root)
        .map_err(|error| FinalizeIssues {
            issues: error
                .diagnostics
                .into_iter()
                .map(|value| issue(value.code, value.path, value.message))
                .collect(),
        })?
        .ok_or_else(|| one_issue("NO_ACTIVE_MILESTONE", None, "no active milestone exists"))?;
    if status.baseline_version.is_none() || status.target_release.is_some() {
        return Err(one_issue(
            "ADOPTION_REVERSE_REQUIRED",
            Some("steering/roadmap.md".to_owned()),
            "adoption finalize requires an unbound reverse milestone",
        ));
    }
    if status.stage != DeliveryStage::AdoptionReady
        || status.health != MilestoneHealth::Consistent
        || !status.current_blockers.is_empty()
    {
        return Err(one_issue(
            "ADOPTION_NOT_READY",
            None,
            "every reverse Spec needs fresh Requirements and Design approval plus a fresh contract review",
        ));
    }
    ensure_clean_repository(project_root)?;
    ensure_source_unchanged(
        project_root,
        specbind_root,
        &status.baseline_revision,
        &status
            .items
            .iter()
            .map(|item| item.id.clone())
            .collect::<Vec<_>>(),
    )?;

    let version = status.baseline_version.clone().ok_or_else(|| {
        one_issue(
            "ADOPTION_REVERSE_REQUIRED",
            Some("steering/roadmap.md".to_owned()),
            "reverse milestone lost its baseline version",
        )
    })?;
    let specs = status
        .items
        .iter()
        .filter_map(|item| item.id.strip_prefix("spec:").map(str::to_owned))
        .collect::<Vec<_>>();
    let entries = release_log::validate_input(log_entries_json, &specs).map_err(log_failure)?;
    let date = OffsetDateTime::now_local()
        .map(OffsetDateTime::date)
        .map_err(|error| one_issue("ADOPTION_LOCAL_DATE_FAILED", None, error.to_string()))?;
    let roadmap_archive = format!("baselines/{version}-roadmap.md");
    let review_archive = format!("baselines/{version}-contract-review.md");
    ensure_archive_targets(specbind_root, &roadmap_archive, &review_archive)?;

    let mut plans = Vec::new();
    for spec in &specs {
        let spec_path = format!("specs/{spec}/spec.yaml");
        let source = read_regular(specbind_root, &spec_path)?;
        let mut wire = runtime::load_spec(&source).map_err(|error| {
            one_issue(
                "ADOPTION_SPEC_INVALID",
                Some(spec_path.clone()),
                error.to_string(),
            )
        })?;
        let establishment = wire.establishment.as_ref().ok_or_else(|| {
            one_issue(
                "ADOPTION_PROVENANCE_MISSING",
                Some(spec_path.clone()),
                "reverse Spec must retain establishment provenance",
            )
        })?;
        if establishment.milestone_id.0 != status.milestone_id
            || establishment.source_revision.0 != status.baseline_revision
            || establishment.baseline_version.0 != version
        {
            return Err(one_issue(
                "ADOPTION_PROVENANCE_MISMATCH",
                Some(spec_path),
                "Spec establishment provenance does not match the active reverse milestone",
            ));
        }
        wire.active_change.0 = None;
        Spec::try_from(wire.clone()).map_err(|error| FinalizeIssues {
            issues: error
                .issues
                .into_iter()
                .map(|value| issue(value.code, Some(spec_path.clone()), value.message))
                .collect(),
        })?;
        let spec_yaml = render_yaml(&wire, &spec_path)?;
        let log_path = format!("specs/{spec}/log.md");
        let existing = read_optional_log(specbind_root, &log_path, language)?;
        let summary = entries.summary(spec).ok_or_else(|| {
            one_issue(
                "LOG_ENTRY_SET_MISMATCH",
                Some(spec.clone()),
                "validated reverse Spec summary disappeared before finalization",
            )
        })?;
        let log_update = release_log::update_log_entry(
            &existing,
            language,
            date,
            LogEntryKind::Baseline,
            &version,
            &status.milestone_id,
            &format!("../../{roadmap_archive}"),
            summary,
            &log_path,
        )
        .map_err(log_failure)?;
        let inventory = artifacts::discover_spec(specbind_root, spec);
        if !inventory.issues.is_empty() {
            return Err(FinalizeIssues {
                issues: inventory
                    .issues
                    .into_iter()
                    .map(|value| {
                        issue(
                            value.code,
                            value.path.map(|path| path.to_string()),
                            value.message,
                        )
                    })
                    .collect(),
            });
        }
        if specbind_root
            .join(format!("specs/{spec}/tasks.yaml"))
            .exists()
        {
            return Err(one_issue(
                "ADOPTION_TASKS_FORBIDDEN",
                Some(format!("specs/{spec}/tasks.yaml")),
                "reverse establishment must not produce Tasks",
            ));
        }
        let cleanup = inventory
            .artifacts
            .into_iter()
            .filter(|artifact| {
                matches!(artifact.kind, ArtifactKind::Brief | ArtifactKind::Research)
            })
            .map(|artifact| artifact.path.to_string())
            .collect();
        plans.push(SpecPlan {
            spec: spec.clone(),
            spec_yaml,
            log_path,
            log_update,
            cleanup,
        });
    }

    ensure_baselines_directory(specbind_root)?;
    for plan in &plans {
        if let LogUpdate::Updated(content) = &plan.log_update {
            guarded_fs::replace_optional(&specbind_root.join(&plan.log_path), content.as_bytes())
                .map_err(|error| {
                one_issue(
                    "ADOPTION_LOG_WRITE_FAILED",
                    Some(plan.log_path.clone()),
                    error.to_string(),
                )
            })?;
        }
    }
    for plan in &plans {
        for relative in &plan.cleanup {
            remove_regular(specbind_root, relative)?;
        }
        let spec_path = format!("specs/{}/spec.yaml", plan.spec);
        guarded_fs::replace_existing(&specbind_root.join(&spec_path), plan.spec_yaml.as_bytes())
            .map_err(|error| {
                one_issue(
                    "ADOPTION_SPEC_WRITE_FAILED",
                    Some(spec_path),
                    error.to_string(),
                )
            })?;
    }
    move_regular(specbind_root, "state/contract-review.md", &review_archive)?;
    move_regular(specbind_root, "steering/roadmap.md", &roadmap_archive)?;
    let dossier = "adoption/reverse-discovery.yaml";
    if specbind_root.join(dossier).exists() {
        remove_regular(specbind_root, dossier)?;
    }
    Ok(FinalizeOutcome {
        baseline_version: version,
        specs: plans.len(),
    })
}

pub(crate) fn ensure_source_unchanged(
    project_root: &Path,
    specbind_root: &Path,
    baseline: &str,
    item_ids: &[String],
) -> Result<(), FinalizeIssues> {
    let root = specbind_root
        .strip_prefix(project_root)
        .map_err(|error| one_issue("ADOPTION_PROJECT_ROOT_INVALID", None, error.to_string()))?;
    let root = root.to_string_lossy().replace('\\', "/");
    let output = repository::output(
        project_root,
        &["diff", "--name-only", &format!("{baseline}..HEAD")],
    )
    .map_err(|error| one_issue("ADOPTION_GIT_FAILED", None, error.to_string()))?;
    let allowed_specs = item_ids
        .iter()
        .filter_map(|item| item.strip_prefix("spec:"))
        .map(|spec| format!("{root}/specs/{spec}/"))
        .collect::<Vec<_>>();
    let mut allowed_exact = vec![
        format!("{root}/steering/roadmap.md"),
        format!("{root}/state/contract-review.md"),
        format!("{root}/adoption/reverse-discovery.yaml"),
    ];
    allowed_exact.extend(deferred_finding_destinations(
        project_root,
        specbind_root,
        &root,
    )?);
    let changed = output
        .lines()
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .filter(|path| {
            !allowed_exact.iter().any(|allowed| path == allowed)
                && !allowed_specs
                    .iter()
                    .any(|allowed| path.starts_with(allowed))
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if changed.is_empty() {
        Ok(())
    } else {
        Err(one_issue(
            "ADOPTION_SOURCE_REVISION_STALE",
            None,
            format!(
                "source changed after reverse discovery: {}",
                changed.join(", ")
            ),
        ))
    }
}

fn deferred_finding_destinations(
    project_root: &Path,
    specbind_root: &Path,
    specbind_relative: &str,
) -> Result<Vec<String>, FinalizeIssues> {
    let record_path = specbind_root.join("adoption/reverse-discovery.yaml");
    if !record_path.exists() {
        return Ok(Vec::new());
    }
    let source = read_regular(specbind_root, "adoption/reverse-discovery.yaml")?;
    let record: AdoptionRecord = serde_saphyr::from_str(&source).map_err(|error| {
        one_issue(
            "ADOPTION_RECORD_INVALID",
            Some("adoption/reverse-discovery.yaml".to_owned()),
            error.to_string(),
        )
    })?;
    let protected = [
        "adoption",
        "baselines",
        "bin",
        "releases",
        "settings",
        "specs",
        "state",
        "steering",
    ];
    let mut destinations = Vec::new();
    for destination in record
        .suspected_defects
        .into_iter()
        .filter_map(|defect| defect.destination)
    {
        let path = Path::new(&destination);
        let components = path.components().collect::<Vec<_>>();
        if path.is_absolute()
            || components.is_empty()
            || components
                .iter()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(invalid_finding_destination(&destination));
        }
        let absolute = project_root.join(path);
        let relative_to_specbind = absolute
            .strip_prefix(specbind_root)
            .map_err(|_| invalid_finding_destination(&destination))?;
        let first = relative_to_specbind
            .components()
            .next()
            .and_then(|component| component.as_os_str().to_str())
            .ok_or_else(|| invalid_finding_destination(&destination))?;
        if protected.contains(&first) {
            return Err(invalid_finding_destination(&destination));
        }
        let normalized = path.to_string_lossy().replace('\\', "/");
        if normalized == specbind_relative || destinations.contains(&normalized) {
            continue;
        }
        destinations.push(normalized);
    }
    Ok(destinations)
}

fn invalid_finding_destination(destination: &str) -> FinalizeIssues {
    one_issue(
        "ADOPTION_FINDING_DESTINATION_INVALID",
        Some("adoption/reverse-discovery.yaml".to_owned()),
        format!(
            "deferred finding destination must be a project-relative path inside the configured SpecBind root and outside managed lifecycle directories: {destination}"
        ),
    )
}

fn ensure_clean_repository(project_root: &Path) -> Result<(), FinalizeIssues> {
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("ADOPTION_GIT_FAILED", None, error.to_string()))?;
    if status.is_empty() {
        Ok(())
    } else {
        Err(one_issue(
            "ADOPTION_WORKTREE_NOT_CLEAN",
            None,
            "commit or reconcile reverse artifacts before adoption finalize",
        ))
    }
}

fn ensure_archive_targets(
    specbind_root: &Path,
    roadmap: &str,
    review: &str,
) -> Result<(), FinalizeIssues> {
    for relative in [roadmap, review] {
        if specbind_root.join(relative).exists() {
            return Err(one_issue(
                "ADOPTION_ARCHIVE_OCCUPIED",
                Some(relative.to_owned()),
                "baseline archive destination must be absent",
            ));
        }
    }
    Ok(())
}

fn ensure_baselines_directory(specbind_root: &Path) -> Result<(), FinalizeIssues> {
    let path = specbind_root.join("baselines");
    match fs::symlink_metadata(&path) {
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() => {
            Err(one_issue(
                "ADOPTION_ARCHIVE_ROOT_INVALID",
                Some("baselines".to_owned()),
                "baseline archive root must be a regular non-symlink directory",
            ))
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&path)
            .map_err(|error| {
                one_issue(
                    "ADOPTION_ARCHIVE_ROOT_CREATE_FAILED",
                    Some("baselines".to_owned()),
                    error.to_string(),
                )
            }),
        Err(error) => Err(one_issue(
            "ADOPTION_ARCHIVE_ROOT_INVALID",
            Some("baselines".to_owned()),
            error.to_string(),
        )),
    }
}

fn move_regular(
    specbind_root: &Path,
    source: &str,
    destination: &str,
) -> Result<(), FinalizeIssues> {
    let source_path = specbind_root.join(source);
    let metadata = fs::symlink_metadata(&source_path).map_err(|error| {
        one_issue(
            "ADOPTION_ARCHIVE_SOURCE_INVALID",
            Some(source.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "ADOPTION_ARCHIVE_SOURCE_INVALID",
            Some(source.to_owned()),
            "archive source must be a regular non-symlink file",
        ));
    }
    fs::rename(source_path, specbind_root.join(destination)).map_err(|error| {
        one_issue(
            "ADOPTION_ARCHIVE_MOVE_FAILED",
            Some(destination.to_owned()),
            error.to_string(),
        )
    })
}

fn remove_regular(specbind_root: &Path, relative: &str) -> Result<(), FinalizeIssues> {
    let path = specbind_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative.to_owned()),
            "cleanup target must be a regular non-symlink file",
        ));
    }
    fs::remove_file(path).map_err(|error| {
        one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

fn remove_spec_directory(specbind_root: &Path, spec: &str) -> Result<(), FinalizeIssues> {
    let relative = format!("specs/{spec}");
    let path = specbind_root.join(&relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative.clone()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative),
            "reverse Spec root must be a regular non-symlink directory",
        ));
    }
    fs::remove_dir_all(path)
        .map_err(|error| one_issue("ADOPTION_CLEANUP_FAILED", Some(relative), error.to_string()))
}

fn read_optional_log(
    specbind_root: &Path,
    relative: &str,
    language: ProjectLanguage,
) -> Result<String, FinalizeIssues> {
    match fs::symlink_metadata(specbind_root.join(relative)) {
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_file() => {
            Err(one_issue(
                "LOG_PROFILE_INVALID",
                Some(relative.to_owned()),
                "log.md must be a regular non-symlink file",
            ))
        }
        Ok(_) => read_regular(specbind_root, relative),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(release_log::empty_log(language))
        }
        Err(error) => Err(one_issue(
            "ADOPTION_LOG_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )),
    }
}

fn read_regular(specbind_root: &Path, relative: &str) -> Result<String, FinalizeIssues> {
    let path = specbind_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "ADOPTION_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "ADOPTION_READ_FAILED",
            Some(relative.to_owned()),
            "target must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(path).map_err(|error| {
        one_issue(
            "ADOPTION_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

fn render_yaml(value: &SpecDocument, path: &str) -> Result<String, FinalizeIssues> {
    let mut rendered = serde_saphyr::to_string(value).map_err(|error| {
        one_issue(
            "ADOPTION_SPEC_SERIALIZE_FAILED",
            Some(path.to_owned()),
            error.to_string(),
        )
    })?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}

fn log_failure(errors: Vec<release_log::LogIssue>) -> FinalizeIssues {
    FinalizeIssues {
        issues: errors
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> FinalizeIssues {
    FinalizeIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> FinalizeIssue {
    FinalizeIssue {
        code,
        path,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deferred_destination_cannot_exempt_managed_lifecycle_state() {
        let project = tempfile::tempdir().expect("temporary project");
        let specbind = project.path().join(".specbind");
        fs::create_dir_all(specbind.join("adoption")).expect("adoption directory");
        fs::write(
            specbind.join("adoption/reverse-discovery.yaml"),
            "suspected_defects:\n  - destination: .specbind/steering/product.md\n",
        )
        .expect("adoption record");

        let error = deferred_finding_destinations(project.path(), &specbind, ".specbind")
            .expect_err("managed destination must remain protected");

        assert_eq!(error.issues[0].code, "ADOPTION_FINDING_DESTINATION_INVALID");
    }
}
