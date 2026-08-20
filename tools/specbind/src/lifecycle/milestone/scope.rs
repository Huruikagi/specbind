//! Active milestone scope mutation lifecycle.

use std::{fs, path::Path};

use serde::Serialize;
use uuid::Uuid;

use crate::{
    artifacts, cross_spec_review, guarded_fs, repository,
    roadmap::{self, DirectItem, DirectStatus, RoadmapDocument, SpecItem},
    schema::{
        runtime,
        spec::v1::{
            ActiveChange, MilestoneId, Nullable, SchemaVersion, SpecDocument, WorkflowState,
        },
    },
};

use super::{
    ROADMAP_RELATIVE, candidate, ensure_target_clean, issue, one_issue, read_roadmap,
    roadmap_failure,
};

const DEFAULT_BODY: &str = "# Roadmap\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeCounts {
    pub new_specs: usize,
    pub spec_updates: usize,
    pub direct_changes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreateOutcome {
    Created {
        milestone_id: String,
        baseline_revision: String,
        counts: ScopeCounts,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeUpdateOutcome {
    Updated {
        milestone_id: String,
        counts: ScopeCounts,
        review_removed: bool,
    },
    NoChange {
        milestone_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RebaselineOutcome {
    Rebaselined {
        milestone_id: String,
        previous: String,
        baseline_revision: String,
        review_removed: bool,
    },
    NoChange {
        milestone_id: String,
        baseline_revision: String,
    },
}

/// Creates the one active milestone from a confirmed scope.
///
/// # Errors
///
/// Returns scope, active-milestone, repository-cleanliness, Git, participating
/// Spec, serialization, or guarded-write diagnostics without partial mutation.
pub fn create(
    project_root: &Path,
    specbind_root: &Path,
    candidate_json: &str,
) -> Result<CreateOutcome, super::MilestoneIssues> {
    let scope = candidate::parse(candidate_json, "MILESTONE_SCOPE_INVALID")?;
    ensure_no_active_roadmap(specbind_root)?;
    ensure_clean_repository(project_root)?;
    let baseline_revision = head_revision(project_root)?;
    let milestone_id = Uuid::now_v7().hyphenated().to_string();

    let body = scope
        .body
        .clone()
        .unwrap_or_else(|| DEFAULT_BODY.to_owned());
    let rendered = render(
        &milestone_id,
        &baseline_revision,
        None,
        &scope.new_specs,
        &scope.spec_updates,
        &scope.direct_changes,
        &body,
    )?;
    let document = roadmap::parse(&rendered).map_err(roadmap_failure)?;
    validate_participants(specbind_root, &document, &[])?;

    initialize_participants(specbind_root, &document, &[])?;
    write_roadmap(specbind_root, &rendered)?;
    Ok(CreateOutcome::Created {
        counts: counts(&document),
        milestone_id: document.milestone_id,
        baseline_revision,
    })
}

/// Replaces the active milestone's confirmed scope and Markdown body.
///
/// # Errors
///
/// Returns scope, active-milestone, removal-guard, participating Spec,
/// target-path, serialization, or guarded-write diagnostics.
pub fn update_scope(
    project_root: &Path,
    specbind_root: &Path,
    candidate_json: &str,
) -> Result<ScopeUpdateOutcome, super::MilestoneIssues> {
    let scope = candidate::parse(candidate_json, "MILESTONE_SCOPE_INVALID")?;
    let source = read_roadmap(specbind_root, "scope update")?;
    let current = roadmap::parse(&source).map_err(roadmap_failure)?;
    let body = scope
        .body
        .clone()
        .unwrap_or_else(|| existing_body(&source).to_owned());
    let direct_changes = preserve_direct_status(&current, scope.direct_changes.clone());
    let rendered = render(
        &current.milestone_id,
        &current.baseline_revision,
        current.target_release.as_deref(),
        &scope.new_specs,
        &scope.spec_updates,
        &direct_changes,
        &body,
    )?;
    if rendered == source {
        return Ok(ScopeUpdateOutcome::NoChange {
            milestone_id: current.milestone_id,
        });
    }
    let next = roadmap::parse(&rendered).map_err(roadmap_failure)?;
    validate_removals(specbind_root, &current, &next)?;
    let retained = current.spec_ids();
    validate_participants(specbind_root, &next, &retained)?;
    ensure_target_clean(
        project_root,
        specbind_root,
        ROADMAP_RELATIVE,
        "MILESTONE_ROADMAP_DIRTY",
        "scope update",
    )?;
    for spec in next
        .spec_ids()
        .iter()
        .filter(|spec| !retained.contains(spec))
    {
        ensure_target_clean(
            project_root,
            specbind_root,
            &spec_relative(spec),
            "MILESTONE_SPEC_DIRTY",
            "scope update",
        )?;
    }

    let review_removed = current.cross_spec_scope().work_items
        != next.cross_spec_scope().work_items
        && remove_accepted_review(specbind_root)?;
    initialize_participants(specbind_root, &next, &retained)?;
    write_roadmap(specbind_root, &rendered)?;
    Ok(ScopeUpdateOutcome::Updated {
        counts: counts(&next),
        milestone_id: next.milestone_id,
        review_removed,
    })
}

/// Replaces the milestone baseline with an explicit ancestor revision.
///
/// # Errors
///
/// Returns revision-format, active-milestone, repository-cleanliness, ancestry,
/// target-path, serialization, or guarded-write diagnostics.
pub fn rebaseline(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
) -> Result<RebaselineOutcome, super::MilestoneIssues> {
    if !valid_revision(revision) {
        return Err(one_issue(
            "MILESTONE_BASELINE_REVISION_INVALID",
            None,
            "rebaseline requires a full lowercase 40- or 64-character Git object ID",
        ));
    }
    let source = read_roadmap(specbind_root, "rebaseline")?;
    let current = roadmap::parse(&source).map_err(roadmap_failure)?;
    if current.baseline_revision == revision {
        return Ok(RebaselineOutcome::NoChange {
            milestone_id: current.milestone_id,
            baseline_revision: current.baseline_revision.clone(),
        });
    }
    ensure_clean_repository(project_root)?;
    ensure_ancestor_commit(project_root, revision)?;

    let rendered = render(
        &current.milestone_id,
        revision,
        current.target_release.as_deref(),
        &current.new_specs,
        &current.spec_updates,
        &current.direct_changes,
        existing_body(&source),
    )?;
    roadmap::parse(&rendered).map_err(roadmap_failure)?;
    let review_removed = remove_accepted_review(specbind_root)?;
    write_roadmap(specbind_root, &rendered)?;
    Ok(RebaselineOutcome::Rebaselined {
        milestone_id: current.milestone_id,
        previous: current.baseline_revision,
        baseline_revision: revision.to_owned(),
        review_removed,
    })
}

#[derive(Serialize)]
struct RoadmapFrontmatter<'a> {
    #[serde(rename = "type")]
    artifact_type: &'static str,
    milestone_id: &'a str,
    baseline_revision: &'a str,
    target_release: Option<&'a str>,
    work_items: WorkItemsFrontmatter<'a>,
}

#[derive(Serialize)]
struct WorkItemsFrontmatter<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    new_specs: Option<Vec<SpecItemFrontmatter<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec_updates: Option<Vec<SpecItemFrontmatter<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direct_changes: Option<Vec<DirectItemFrontmatter<'a>>>,
}

#[derive(Serialize)]
struct SpecItemFrontmatter<'a> {
    spec: &'a str,
    summary: &'a str,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    depends_on: &'a [roadmap::Dependency],
}

#[derive(Serialize)]
struct DirectItemFrontmatter<'a> {
    id: &'a str,
    summary: &'a str,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    depends_on: &'a [roadmap::Dependency],
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<DirectStatus>,
}

fn render(
    milestone_id: &str,
    baseline_revision: &str,
    target_release: Option<&str>,
    new_specs: &[SpecItem],
    spec_updates: &[SpecItem],
    direct_changes: &[DirectItem],
    body: &str,
) -> Result<String, super::MilestoneIssues> {
    let frontmatter = RoadmapFrontmatter {
        artifact_type: "SpecBind Roadmap",
        milestone_id,
        baseline_revision,
        target_release,
        work_items: WorkItemsFrontmatter {
            new_specs: spec_frontmatter(new_specs),
            spec_updates: spec_frontmatter(spec_updates),
            direct_changes: (!direct_changes.is_empty()).then(|| {
                direct_changes
                    .iter()
                    .map(|item| DirectItemFrontmatter {
                        id: &item.id,
                        summary: &item.summary,
                        depends_on: &item.depends_on,
                        status: item.status,
                    })
                    .collect()
            }),
        },
    };
    let rendered = serde_saphyr::to_string(&frontmatter).map_err(|error| {
        one_issue(
            "MILESTONE_ROADMAP_SERIALIZE_FAILED",
            Some(ROADMAP_RELATIVE.to_owned()),
            error.to_string(),
        )
    })?;
    let mut output = String::from("---\n");
    output.push_str(rendered.trim_end_matches('\n'));
    output.push_str("\n---\n");
    output.push_str(body);
    if !output.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

fn spec_frontmatter(items: &[SpecItem]) -> Option<Vec<SpecItemFrontmatter<'_>>> {
    (!items.is_empty()).then(|| {
        items
            .iter()
            .map(|item| SpecItemFrontmatter {
                spec: &item.spec,
                summary: &item.summary,
                depends_on: &item.depends_on,
            })
            .collect()
    })
}

/// Returns the Markdown body that follows the closing frontmatter delimiter.
/// Returns the authored Markdown body that follows the Front Matter.
///
/// The Roadmap parser does not carry the body, so both scope replacement and the
/// scope read recover it from the original source through this one function.
pub(crate) fn existing_body(source: &str) -> &str {
    let mut offset = 0;
    let mut lines = source.split_inclusive('\n');
    let Some(first) = lines.next() else {
        return "";
    };
    if line_content(first) != "---" {
        return "";
    }
    offset += first.len();
    for line in lines {
        offset += line.len();
        if line_content(line) == "---" {
            return &source[offset..];
        }
    }
    ""
}

fn line_content(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

fn preserve_direct_status(
    current: &RoadmapDocument,
    submitted: Vec<DirectItem>,
) -> Vec<DirectItem> {
    submitted
        .into_iter()
        .map(|mut item| {
            item.status = current
                .direct_changes
                .iter()
                .find(|existing| existing.id == item.id)
                .and_then(|existing| existing.status);
            item
        })
        .collect()
}

fn counts(document: &RoadmapDocument) -> ScopeCounts {
    ScopeCounts {
        new_specs: document.new_specs.len(),
        spec_updates: document.spec_updates.len(),
        direct_changes: document.direct_changes.len(),
    }
}

fn validate_removals(
    specbind_root: &Path,
    current: &RoadmapDocument,
    next: &RoadmapDocument,
) -> Result<(), super::MilestoneIssues> {
    let mut issues = Vec::new();
    let retained = next.spec_ids();
    for spec in current.spec_ids() {
        if retained.contains(&spec) {
            continue;
        }
        let resolution = artifacts::resolve_spec(specbind_root, &spec);
        let active = resolution
            .wire
            .as_ref()
            .is_some_and(|wire| wire.active_change.0.is_some());
        if active || resolution.wire.is_none() {
            issues.push(issue(
                "MILESTONE_SCOPE_SPEC_REMOVAL_BLOCKED",
                Some(spec_relative(&spec)),
                "removing a Spec that still holds an active change requires explicit scope removal",
            ));
        }
    }
    for item in &current.direct_changes {
        let retained = next
            .direct_changes
            .iter()
            .any(|candidate| candidate.id == item.id);
        if !retained && item.status == Some(DirectStatus::Completed) {
            issues.push(issue(
                "MILESTONE_SCOPE_DIRECT_REMOVAL_BLOCKED",
                Some(ROADMAP_RELATIVE.to_owned()),
                format!(
                    "completed Direct item {} cannot be dropped by a scope update",
                    item.id
                ),
            ));
        }
    }
    super::finish_issues(issues)
}

fn validate_participants(
    specbind_root: &Path,
    document: &RoadmapDocument,
    retained: &[String],
) -> Result<(), super::MilestoneIssues> {
    let mut issues = Vec::new();
    for item in &document.new_specs {
        if retained.contains(&item.spec) {
            continue;
        }
        let directory = specbind_root.join(format!("specs/{}", item.spec));
        if fs::symlink_metadata(&directory).is_ok() {
            issues.push(issue(
                "MILESTONE_SPEC_ALREADY_EXISTS",
                Some(format!("specs/{}", item.spec)),
                "a new Spec must not have an existing Spec directory",
            ));
        }
    }
    for item in &document.spec_updates {
        if retained.contains(&item.spec) {
            continue;
        }
        let resolution = artifacts::resolve_spec(specbind_root, &item.spec);
        let Some(wire) = resolution.wire else {
            issues.push(issue(
                "MILESTONE_SPEC_UNAVAILABLE",
                Some(spec_relative(&item.spec)),
                "an updated Spec requires readable existing metadata",
            ));
            continue;
        };
        if wire.active_change.0.is_some() {
            issues.push(issue(
                "MILESTONE_SPEC_ALREADY_ACTIVE",
                Some(spec_relative(&item.spec)),
                "an updated Spec must be idle before joining a milestone",
            ));
        }
        let contract = specbind_root.join(format!("specs/{}/contract.md", item.spec));
        if fs::symlink_metadata(&contract).is_err() {
            issues.push(issue(
                "MILESTONE_SPEC_CONTRACT_MISSING",
                Some(format!("specs/{}/contract.md", item.spec)),
                "an updated Spec requires its Contract as the baseline before-state",
            ));
        }
    }
    super::finish_issues(issues)
}

fn initialize_participants(
    specbind_root: &Path,
    document: &RoadmapDocument,
    retained: &[String],
) -> Result<(), super::MilestoneIssues> {
    for spec in document.spec_ids() {
        if retained.contains(&spec) {
            continue;
        }
        let relative = spec_relative(&spec);
        let path = specbind_root.join(&relative);
        let wire = SpecDocument {
            schema_version: SchemaVersion(1),
            active_change: Nullable(Some(ActiveChange {
                milestone_id: MilestoneId(document.milestone_id.clone()),
                state: WorkflowState::Requirements,
                requirement_ids: Nullable(None),
                gate_evidence: None,
            })),
        };
        let mut rendered = serde_saphyr::to_string(&wire).map_err(|error| {
            one_issue(
                "MILESTONE_SPEC_SERIALIZE_FAILED",
                Some(relative.clone()),
                error.to_string(),
            )
        })?;
        if !rendered.ends_with('\n') {
            rendered.push('\n');
        }
        runtime::load_spec(&rendered).map_err(|error| {
            one_issue(
                "MILESTONE_SPEC_SERIALIZE_FAILED",
                Some(relative.clone()),
                error.to_string(),
            )
        })?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                one_issue(
                    "MILESTONE_SPEC_WRITE_FAILED",
                    Some(relative.clone()),
                    error.to_string(),
                )
            })?;
        }
        guarded_fs::replace_optional(&path, rendered.as_bytes()).map_err(|error| {
            one_issue(
                "MILESTONE_SPEC_WRITE_FAILED",
                Some(relative.clone()),
                error.to_string(),
            )
        })?;
    }
    Ok(())
}

fn write_roadmap(specbind_root: &Path, rendered: &str) -> Result<(), super::MilestoneIssues> {
    let path = specbind_root.join(ROADMAP_RELATIVE);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            one_issue(
                "MILESTONE_ROADMAP_WRITE_FAILED",
                Some(ROADMAP_RELATIVE.to_owned()),
                error.to_string(),
            )
        })?;
    }
    guarded_fs::replace_optional(&path, rendered.as_bytes()).map_err(|error| {
        one_issue(
            "MILESTONE_ROADMAP_WRITE_FAILED",
            Some(ROADMAP_RELATIVE.to_owned()),
            error.to_string(),
        )
    })
}

/// Delegates to the review module, which owns the accepted artifact.
fn remove_accepted_review(specbind_root: &Path) -> Result<bool, super::MilestoneIssues> {
    cross_spec_review::remove_accepted(specbind_root).map_err(|error| super::MilestoneIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, value.source, value.message))
            .collect(),
    })
}

fn ensure_no_active_roadmap(specbind_root: &Path) -> Result<(), super::MilestoneIssues> {
    match fs::symlink_metadata(specbind_root.join(ROADMAP_RELATIVE)) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Ok(_) => Err(one_issue(
            "MILESTONE_ALREADY_ACTIVE",
            Some(ROADMAP_RELATIVE.to_owned()),
            "milestone creation requires no existing active Roadmap",
        )),
        Err(error) => Err(one_issue(
            "MILESTONE_ROADMAP_TARGET_INVALID",
            Some(ROADMAP_RELATIVE.to_owned()),
            error.to_string(),
        )),
    }
}

fn ensure_clean_repository(project_root: &Path) -> Result<(), super::MilestoneIssues> {
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("MILESTONE_GIT_FAILED", None, error.to_string()))?;
    if status.is_empty() {
        Ok(())
    } else {
        Err(one_issue(
            "MILESTONE_REPOSITORY_DIRTY",
            None,
            "the operation requires a clean committed repository",
        ))
    }
}

fn head_revision(project_root: &Path) -> Result<String, super::MilestoneIssues> {
    let revision = repository::output(project_root, &["rev-parse", "HEAD"])
        .map_err(|error| one_issue("MILESTONE_GIT_FAILED", None, error.to_string()))?
        .trim()
        .to_owned();
    if valid_revision(&revision) {
        Ok(revision)
    } else {
        Err(one_issue(
            "MILESTONE_BASELINE_REVISION_INVALID",
            None,
            "milestone creation requires a full lowercase Git commit object ID at HEAD",
        ))
    }
}

fn ensure_ancestor_commit(
    project_root: &Path,
    revision: &str,
) -> Result<(), super::MilestoneIssues> {
    let resolved = repository::output(
        project_root,
        &["rev-parse", "--verify", &format!("{revision}^{{commit}}")],
    )
    .map_err(|error| {
        one_issue(
            "MILESTONE_BASELINE_MISSING",
            None,
            format!("baseline revision must exist in this repository: {error}"),
        )
    })?;
    if resolved.trim() != revision {
        return Err(one_issue(
            "MILESTONE_BASELINE_NOT_EXACT",
            None,
            "baseline revision must resolve to the same full commit object ID",
        ));
    }
    match repository::predicate(
        project_root,
        &["merge-base", "--is-ancestor", revision, "HEAD"],
    ) {
        Ok(true) => Ok(()),
        Ok(false) => Err(one_issue(
            "MILESTONE_BASELINE_NOT_ANCESTOR",
            None,
            "baseline revision must be an ancestor of current HEAD",
        )),
        Err(error) => Err(one_issue("MILESTONE_GIT_FAILED", None, error.to_string())),
    }
}

fn valid_revision(value: &str) -> bool {
    matches!(value.len(), 40 | 64)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn spec_relative(canonical_spec: &str) -> String {
    format!("specs/{canonical_spec}/spec.yaml")
}
