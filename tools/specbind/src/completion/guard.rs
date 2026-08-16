use std::path::Path;

use crate::{
    artifacts,
    cross_spec_review::{self, ReviewBoundary},
    freshness::{self, FreshnessStatus},
    milestone_status::{self, MilestoneHealth},
    repository,
    roadmap::{self, DirectStatus, RoadmapDocument},
    schema::spec::v1::WorkflowState,
    spec_status::{self, ConsistencyHealth},
};

use super::{
    CompletionIssue, CompletionIssues, finish_issues, from_discovery, issue, one_issue,
    read_regular, roadmap_failure, spec_path,
};

pub(super) struct SpecGuard {
    pub(super) source: String,
    pub(super) wire: crate::schema::spec::v1::SpecDocument,
}

pub(super) struct RoadmapGuard {
    pub(super) source: String,
    pub(super) roadmap: RoadmapDocument,
}

pub(super) fn spec_guard(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    implementation_revision: &str,
    allow_pending_metadata: bool,
) -> Result<SpecGuard, CompletionIssues> {
    validate_revision(implementation_revision)?;
    let mut issues = Vec::new();
    validate_completion_checkout(
        project_root,
        specbind_root,
        implementation_revision,
        allow_pending_metadata,
        &mut issues,
    )?;
    let roadmap_guard = read_roadmap(specbind_root)?;
    validate_completion_milestone(
        project_root,
        specbind_root,
        canonical_spec,
        &roadmap_guard.roadmap,
        &mut issues,
    )?;
    validate_participating_specs(
        project_root,
        specbind_root,
        &roadmap_guard.roadmap,
        &mut issues,
    );
    validate_completion_review(project_root, specbind_root, canonical_spec, &mut issues);

    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(wire) = resolution.wire else {
        issues.extend(resolution.issues.into_iter().map(from_discovery));
        return finish_issues(issues).map(|()| unreachable!());
    };
    if wire
        .active_change
        .0
        .as_ref()
        .is_none_or(|active| active.state != WorkflowState::Implementation)
    {
        issues.push(issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion acceptance requires the target Spec in implementation",
        ));
    }
    let source = read_regular(
        &specbind_root.join(spec_path(canonical_spec)),
        &spec_path(canonical_spec),
    )?;
    finish_issues(issues)?;
    Ok(SpecGuard { source, wire })
}

fn validate_completion_checkout(
    project_root: &Path,
    specbind_root: &Path,
    implementation_revision: &str,
    allow_pending_metadata: bool,
    issues: &mut Vec<CompletionIssue>,
) -> Result<(), CompletionIssues> {
    if head_revision(project_root)? != implementation_revision {
        issues.push(issue(
            "COMPLETION_REVISION_CHANGED",
            None,
            "current HEAD does not match the completion implementation revision",
        ));
    }
    if allow_pending_metadata {
        issues.extend(
            freshness::assess_pending_completion_mutations(
                project_root,
                specbind_root,
                implementation_revision,
            )
            .issues
            .into_iter()
            .map(|value| issue(value.code, Some(value.path), value.message)),
        );
    } else if !worktree_status(project_root)?.is_empty() {
        issues.push(issue(
            "COMPLETION_WORKTREE_DIRTY",
            None,
            "completion preflight requires a clean worktree",
        ));
    }
    Ok(())
}

fn validate_completion_milestone(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    roadmap: &RoadmapDocument,
    issues: &mut Vec<CompletionIssue>,
) -> Result<(), CompletionIssues> {
    if !roadmap.spec_ids().iter().any(|spec| spec == canonical_spec) {
        issues.push(issue(
            "SPEC_COMPLETION_NOT_IN_MILESTONE",
            Some(spec_path(canonical_spec)),
            "completion target must participate in the active Roadmap",
        ));
    }
    let milestone = milestone_status::resolve(project_root, specbind_root).map_err(|error| {
        CompletionIssues {
            issues: error
                .diagnostics
                .into_iter()
                .map(|value| issue(value.code, value.path, value.message))
                .collect(),
        }
    })?;
    match milestone {
        Some(model) if model.milestone_id == roadmap.milestone_id => {
            if model.health != MilestoneHealth::Consistent {
                issues.extend(
                    model
                        .diagnostics
                        .into_iter()
                        .map(|value| issue(value.code, value.path, value.message)),
                );
            }
        }
        Some(_) => issues.push(issue(
            "COMPLETION_MILESTONE_CHANGED",
            Some("steering/roadmap.md".to_owned()),
            "resolved milestone identity changed during completion",
        )),
        None => issues.push(issue(
            "COMPLETION_MILESTONE_MISSING",
            Some("steering/roadmap.md".to_owned()),
            "completion requires an active Roadmap",
        )),
    }
    if roadmap
        .direct_changes
        .iter()
        .any(|item| item.status != Some(DirectStatus::Completed))
    {
        issues.push(issue(
            "COMPLETION_MILESTONE_NOT_CONVERGED",
            Some("steering/roadmap.md".to_owned()),
            "every Direct item must be completed before Spec validation",
        ));
    }
    Ok(())
}

fn validate_participating_specs(
    project_root: &Path,
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    issues: &mut Vec<CompletionIssue>,
) {
    for spec in roadmap.spec_ids() {
        match spec_status::resolve(project_root, specbind_root, &spec) {
            Ok(model) => {
                if model.milestone_id.as_deref() != Some(&roadmap.milestone_id) {
                    issues.push(issue(
                        "COMPLETION_SPEC_MILESTONE_MISMATCH",
                        Some(spec_path(&spec)),
                        "participating Spec milestone does not match the Roadmap",
                    ));
                }
                if model.health != ConsistencyHealth::Consistent {
                    issues.extend(
                        model
                            .diagnostics
                            .into_iter()
                            .map(|value| issue(value.code, value.path, value.message)),
                    );
                }
                if !matches!(
                    model.declared_state,
                    Some(WorkflowState::Implementation | WorkflowState::ReleaseReady)
                ) {
                    issues.push(issue(
                        "COMPLETION_SPEC_STATE_INVALID",
                        Some(spec_path(&spec)),
                        "every participating Spec must be in implementation or release_ready",
                    ));
                }
                if [
                    &model.freshness.requirements,
                    &model.freshness.design,
                    &model.freshness.tasks,
                ]
                .iter()
                .any(|gate| gate.status != FreshnessStatus::Fresh)
                {
                    issues.push(issue(
                        "COMPLETION_SPEC_GATE_STALE",
                        Some(spec_path(&spec)),
                        "every participating Spec requires fresh prior gates",
                    ));
                }
                if !model.task_model.as_ref().is_some_and(|tasks| {
                    tasks.completed == tasks.total() && tasks.pending == 0 && tasks.blocked == 0
                }) {
                    issues.push(issue(
                        "COMPLETION_MILESTONE_NOT_CONVERGED",
                        Some(format!("specs/{spec}/tasks.yaml")),
                        "every participating Spec task plan must be complete and unblocked",
                    ));
                }
            }
            Err(error) => issues.extend(error.issues.into_iter().map(from_discovery)),
        }
    }
}

fn validate_completion_review(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    issues: &mut Vec<CompletionIssue>,
) {
    if let Err(error) = cross_spec_review::require_for_boundary(
        project_root,
        specbind_root,
        ReviewBoundary::ImplementationValidation { canonical_spec },
    ) {
        issues.extend(
            error
                .issues
                .into_iter()
                .map(|value| issue(value.code, value.source, value.message)),
        );
    }
}

pub(super) fn read_roadmap(specbind_root: &Path) -> Result<RoadmapGuard, CompletionIssues> {
    let relative = "steering/roadmap.md";
    let source = read_regular(&specbind_root.join(relative), relative)?;
    let roadmap = roadmap::parse(&source).map_err(roadmap_failure)?;
    Ok(RoadmapGuard { source, roadmap })
}

pub(super) fn clean_head(project_root: &Path) -> Result<String, CompletionIssues> {
    let revision = head_revision(project_root)?;
    if !worktree_status(project_root)?.is_empty() {
        return Err(one_issue(
            "COMPLETION_WORKTREE_DIRTY",
            None,
            "completion preflight requires a clean worktree",
        ));
    }
    Ok(revision)
}

fn head_revision(project_root: &Path) -> Result<String, CompletionIssues> {
    let revision = git_output(project_root, &["rev-parse", "HEAD"])?;
    let revision = revision.trim().to_owned();
    validate_revision(&revision)?;
    Ok(revision)
}

fn worktree_status(project_root: &Path) -> Result<Vec<u8>, CompletionIssues> {
    git_output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
}

pub(super) fn ensure_target_clean(
    project_root: &Path,
    specbind_root: &Path,
    spec_relative: &str,
    issues: &mut Vec<CompletionIssue>,
) {
    let Ok(root_relative) = specbind_root.strip_prefix(project_root) else {
        issues.push(issue(
            "COMPLETION_PROJECT_ROOT_INVALID",
            Some(spec_relative.to_owned()),
            "SpecBind root must be below the Git project root",
        ));
        return;
    };
    let path = root_relative
        .join(spec_relative)
        .to_string_lossy()
        .replace('\\', "/");
    match git_output_bytes(
        project_root,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--",
            &path,
        ],
    ) {
        Ok(output) if output.is_empty() => {}
        Ok(_) => issues.push(issue(
            "SPEC_COMPLETION_TARGET_DIRTY",
            Some(spec_relative.to_owned()),
            "completion invalidation refuses to overwrite a dirty or staged spec.yaml",
        )),
        Err(error) => issues.extend(error.issues),
    }
}

pub(super) fn validate_revision(value: &str) -> Result<(), CompletionIssues> {
    let valid = matches!(value.len(), 40 | 64)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if valid {
        Ok(())
    } else {
        Err(one_issue(
            "COMPLETION_EVIDENCE_INVALID",
            Some("/implementationRevision".to_owned()),
            "implementationRevision must be a full lowercase Git object ID",
        ))
    }
}

pub(super) fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn git_output(project_root: &Path, arguments: &[&str]) -> Result<String, CompletionIssues> {
    repository::output(project_root, arguments)
        .map_err(|error| one_issue("COMPLETION_GIT_FAILED", None, error.to_string()))
}

fn git_output_bytes(project_root: &Path, arguments: &[&str]) -> Result<Vec<u8>, CompletionIssues> {
    repository::output_bytes(project_root, arguments)
        .map_err(|error| one_issue("COMPLETION_GIT_FAILED", None, error.to_string()))
}
