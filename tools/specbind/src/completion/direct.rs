use std::path::Path;

use crate::{
    roadmap::{self, Dependency, DirectCompletionEdit, DirectStatus},
    spec_status::{self, ConsistencyHealth},
};

use super::{
    CompletionIssues, DirectCompleteOutcome, DirectPreflightOutcome, finish_issues,
    guard::{RoadmapGuard, clean_head, read_roadmap, valid_id, validate_revision},
    issue, one_issue, persist_regular, roadmap_failure,
};

/// Begins the clean-revision handshake for one pending Direct Roadmap item.
///
/// # Errors
///
/// Returns Roadmap identity, dependency, or Git diagnostics.
pub fn direct_preflight(
    project_root: &Path,
    specbind_root: &Path,
    canonical_direct: &str,
) -> Result<DirectPreflightOutcome, CompletionIssues> {
    if direct_completed(specbind_root, canonical_direct)? {
        return Ok(DirectPreflightOutcome::AlreadyCompleted);
    }
    let revision = clean_head(project_root)?;
    direct_guard(project_root, specbind_root, canonical_direct, &revision)?;
    Ok(DirectPreflightOutcome::Ready {
        implementation_revision: revision,
    })
}

/// Records one Direct item complete after independently rechecking its revision.
///
/// # Errors
///
/// Returns Roadmap, dependency, revision, race, serialization, or write diagnostics.
pub fn direct_complete(
    project_root: &Path,
    specbind_root: &Path,
    canonical_direct: &str,
    implementation_revision: &str,
) -> Result<DirectCompleteOutcome, CompletionIssues> {
    if direct_completed(specbind_root, canonical_direct)? {
        return Ok(DirectCompleteOutcome::AlreadyCompleted);
    }
    validate_revision(implementation_revision)?;
    let initial = direct_guard(
        project_root,
        specbind_root,
        canonical_direct,
        implementation_revision,
    )?;
    let current = direct_guard(
        project_root,
        specbind_root,
        canonical_direct,
        implementation_revision,
    )?;
    if initial.source != current.source || initial.roadmap != current.roadmap {
        return Err(one_issue(
            "DIRECT_COMPLETION_INPUTS_CHANGED",
            Some("steering/roadmap.md".to_owned()),
            "Roadmap changed during guarded Direct completion",
        ));
    }
    match roadmap::complete_direct(&current.source, canonical_direct).map_err(roadmap_failure)? {
        DirectCompletionEdit::NoChange => Ok(DirectCompleteOutcome::AlreadyCompleted),
        DirectCompletionEdit::Updated(rendered) => {
            persist_regular(
                &specbind_root.join("steering/roadmap.md"),
                rendered.as_bytes(),
                "DIRECT_COMPLETION_WRITE_FAILED",
                "steering/roadmap.md",
            )?;
            Ok(DirectCompleteOutcome::Recorded)
        }
    }
}

fn direct_completed(
    specbind_root: &Path,
    canonical_direct: &str,
) -> Result<bool, CompletionIssues> {
    if !valid_id(canonical_direct) {
        return Err(one_issue(
            "DIRECT_COMPLETION_TARGET_INVALID",
            Some(format!("direct:{canonical_direct}")),
            "Direct completion requires a canonical Direct ID",
        ));
    }
    let guard = read_roadmap(specbind_root)?;
    guard
        .roadmap
        .direct_changes
        .iter()
        .find(|item| item.id == canonical_direct)
        .map(|item| item.status == Some(DirectStatus::Completed))
        .ok_or_else(|| {
            one_issue(
                "DIRECT_COMPLETION_NOT_FOUND",
                Some("steering/roadmap.md".to_owned()),
                format!("Direct item {canonical_direct} is not in the active Roadmap"),
            )
        })
}

fn direct_guard(
    project_root: &Path,
    specbind_root: &Path,
    canonical_direct: &str,
    implementation_revision: &str,
) -> Result<RoadmapGuard, CompletionIssues> {
    if !valid_id(canonical_direct) {
        return Err(one_issue(
            "DIRECT_COMPLETION_TARGET_INVALID",
            Some(format!("direct:{canonical_direct}")),
            "Direct completion requires a canonical Direct ID",
        ));
    }
    validate_revision(implementation_revision)?;
    let current_revision = clean_head(project_root)?;
    let guard = read_roadmap(specbind_root)?;
    let Some(item) = guard
        .roadmap
        .direct_changes
        .iter()
        .find(|item| item.id == canonical_direct)
    else {
        return Err(one_issue(
            "DIRECT_COMPLETION_NOT_FOUND",
            Some("steering/roadmap.md".to_owned()),
            format!("Direct item {canonical_direct} is not in the active Roadmap"),
        ));
    };
    if item.status == Some(DirectStatus::Completed) {
        return Ok(guard);
    }
    let mut issues = Vec::new();
    if current_revision != implementation_revision {
        issues.push(issue(
            "DIRECT_COMPLETION_REVISION_CHANGED",
            None,
            "current HEAD does not match the Direct implementation revision",
        ));
    }
    for dependency in &item.depends_on {
        match dependency {
            Dependency::Direct(value) => {
                let complete = guard
                    .roadmap
                    .direct_changes
                    .iter()
                    .find(|item| item.id == value.direct)
                    .is_some_and(|item| item.status == Some(DirectStatus::Completed));
                if !complete {
                    issues.push(issue(
                        "DIRECT_COMPLETION_DEPENDENCY_PENDING",
                        Some("steering/roadmap.md".to_owned()),
                        format!("dependency direct:{} is not completed", value.direct),
                    ));
                }
            }
            Dependency::Spec(value) => {
                let complete = spec_status::resolve(project_root, specbind_root, &value.spec)
                    .is_ok_and(|model| {
                        model.health == ConsistencyHealth::Consistent
                            && model.milestone_id.as_deref() == Some(&guard.roadmap.milestone_id)
                            && model.task_model.as_ref().is_some_and(|tasks| {
                                tasks.completed == tasks.total()
                                    && tasks.pending == 0
                                    && tasks.blocked == 0
                            })
                    });
                if !complete {
                    issues.push(issue(
                        "DIRECT_COMPLETION_DEPENDENCY_PENDING",
                        Some("steering/roadmap.md".to_owned()),
                        format!(
                            "dependency spec:{} is not implementation-complete",
                            value.spec
                        ),
                    ));
                }
            }
        }
    }
    finish_issues(issues)?;
    Ok(guard)
}
