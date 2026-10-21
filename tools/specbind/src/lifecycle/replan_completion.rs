//! Read-only mechanical evidence for delegated completion withdrawal.

use std::{collections::BTreeMap, fs, path::Path};

use crate::{
    artifacts,
    cross_spec_review::ReviewFreshnessStatus,
    freshness::FreshnessStatus,
    milestone_status::{self, MilestoneHealth},
    repository, roadmap,
    schema::{
        runtime,
        spec::v1::{SpecDocument, WorkflowState},
    },
    spec_status::{self, ConsistencyHealth},
};

#[derive(Debug, thiserror::Error)]
#[error("{code}: {message}")]
pub struct ReplanError {
    pub code: &'static str,
    pub message: String,
}

fn error(code: &'static str, message: impl Into<String>) -> ReplanError {
    ReplanError {
        code,
        message: message.into(),
    }
}

fn git(project: &Path, args: &[&str]) -> Result<String, ReplanError> {
    repository::output(project, args).map_err(|e| error("REPLAN_GIT_FAILED", e.to_string()))
}

fn clean_head(project: &Path) -> Result<String, ReplanError> {
    if !git(
        project,
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )?
    .is_empty()
    {
        return Err(error(
            "REPLAN_WORKTREE_DIRTY",
            "a clean committed checkout is required",
        ));
    }
    Ok(git(project, &["rev-parse", "HEAD"])?.trim().to_owned())
}

fn exact_revision(project: &Path, revision: &str) -> Result<(), ReplanError> {
    if !matches!(revision.len(), 40 | 64) || !revision.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(error(
            "REPLAN_REVISION_INVALID",
            "use the captured full commit ID",
        ));
    }
    if git(
        project,
        &["rev-parse", "--verify", &format!("{revision}^{{commit}}")],
    )?
    .trim()
        != revision
    {
        return Err(error(
            "REPLAN_REVISION_INVALID",
            "revision must name that exact commit",
        ));
    }
    Ok(())
}

fn ancestor(project: &Path, before: &str, after: &str) -> Result<(), ReplanError> {
    if !repository::predicate(project, &["merge-base", "--is-ancestor", before, after])
        .map_err(|e| error("REPLAN_GIT_FAILED", e.to_string()))?
    {
        return Err(error(
            "REPLAN_HISTORY_CHANGED",
            "captured revisions must remain ancestors in order",
        ));
    }
    Ok(())
}

struct Snapshot {
    _directory: tempfile::TempDir,
    roadmap: roadmap::RoadmapDocument,
    specs: BTreeMap<String, SpecDocument>,
    status: BTreeMap<String, spec_status::SpecStatusModel>,
    milestone: milestone_status::MilestoneStatusModel,
}

fn snapshot(project: &Path, relative: &Path, revision: &str) -> Result<Snapshot, ReplanError> {
    let paths = git(
        project,
        &[
            "ls-tree",
            "-r",
            revision,
            "--",
            &relative.to_string_lossy(),
            ".specbind.json",
        ],
    )?;
    if paths
        .lines()
        .any(|line| line.starts_with("120000 ") || line.starts_with("160000 "))
    {
        return Err(error(
            "REPLAN_SNAPSHOT_UNSUPPORTED",
            "historical SpecBind inputs must be ordinary tracked files",
        ));
    }
    let directory = repository::snapshot::at(project, revision)
        .map_err(|e| error("REPLAN_SNAPSHOT_FAILED", e.to_string()))?;
    let root = directory.path().join(relative);
    let source = fs::read_to_string(root.join("steering/roadmap.md"))
        .map_err(|e| error("REPLAN_INPUT_INVALID", e.to_string()))?;
    let roadmap =
        roadmap::parse(&source).map_err(|e| error("REPLAN_INPUT_INVALID", e.to_string()))?;
    if !roadmap.reverse_specs.is_empty() {
        return Err(error(
            "REPLAN_SCOPE_INVALID",
            "only an active delivery milestone is eligible",
        ));
    }
    let milestone = milestone_status::resolve(directory.path(), &root)
        .map_err(|e| error("REPLAN_STATE_INVALID", format!("{:?}", e.diagnostics)))?
        .ok_or_else(|| error("REPLAN_SCOPE_INVALID", "active milestone required"))?;
    let mut specs = BTreeMap::new();
    let mut status = BTreeMap::new();
    for id in roadmap.spec_ids() {
        let resolved = artifacts::resolve_spec(&root, &id);
        if !resolved.issues.is_empty() {
            return Err(error(
                "REPLAN_INPUT_INVALID",
                format!("{:?}", resolved.issues),
            ));
        }
        specs.insert(
            id.clone(),
            resolved
                .wire
                .ok_or_else(|| error("REPLAN_INPUT_INVALID", &id))?,
        );
        status.insert(
            id.clone(),
            spec_status::resolve(directory.path(), &root, &id)
                .map_err(|e| error("REPLAN_INPUT_INVALID", format!("{:?}", e.issues)))?,
        );
    }
    Ok(Snapshot {
        _directory: directory,
        roadmap,
        specs,
        status,
        milestone,
    })
}

fn baseline(snapshot: &Snapshot) -> Result<Vec<String>, ReplanError> {
    if snapshot.milestone.health != MilestoneHealth::Consistent
        || snapshot.milestone.review_status != ReviewFreshnessStatus::Fresh
    {
        return Err(error(
            "REPLAN_BASELINE_INVALID",
            "baseline requires consistent state and fresh Contract Review",
        ));
    }
    Ok(snapshot
        .roadmap
        .spec_ids()
        .into_iter()
        .filter(|id| {
            let state = &snapshot.status[id];
            state.declared_state == Some(WorkflowState::ReleaseReady)
                && state.freshness.completion.status == FreshnessStatus::Fresh
        })
        .collect())
}

fn relative_root<'a>(project: &Path, root: &'a Path) -> Result<&'a Path, ReplanError> {
    root.strip_prefix(project)
        .map_err(|e| error("REPLAN_ROOT_INVALID", e.to_string()))
}

fn render(value: &serde_json::Value) -> Result<String, ReplanError> {
    serde_json::to_string_pretty(value)
        .map(|s| format!("{s}\n"))
        .map_err(|e| error("REPLAN_OUTPUT_FAILED", e.to_string()))
}

/// Captures a reproducible clean baseline before authorized Plan dispatch.
/// # Errors
/// Rejects dirty, inconsistent, missing or unsupported historical inputs.
pub fn capture(project: &Path, root: &Path) -> Result<String, ReplanError> {
    let head = clean_head(project)?;
    let snapshot = snapshot(project, relative_root(project, root)?, &head)?;
    let completed = baseline(&snapshot)?;
    verify_live(project, root, &snapshot, &[])?;
    if clean_head(project)? != head {
        return Err(error("REPLAN_HEAD_CHANGED", "HEAD changed during capture"));
    }
    render(
        &serde_json::json!({"schemaVersion":1,"result":"REPLAN_BASELINE_CAPTURED",
        "baselineRevision":head,"milestoneId":snapshot.roadmap.milestone_id,"freshCompletions":completed}),
    )
}

/// Checks all withdrawal targets and every subsequent checkpoint against fixed revisions.
/// # Errors
/// Rejects missing history, changed authority, other inconsistencies or unexpected commits.
pub fn check(
    project: &Path,
    root: &Path,
    baseline_revision: &str,
    recovery_revision: &str,
) -> Result<String, ReplanError> {
    let head = clean_head(project)?;
    for revision in [baseline_revision, recovery_revision] {
        exact_revision(project, revision)?;
    }
    ancestor(project, baseline_revision, recovery_revision)?;
    ancestor(project, recovery_revision, &head)?;
    let relative = relative_root(project, root)?;
    let before = snapshot(project, relative, baseline_revision)?;
    let after = snapshot(project, relative, recovery_revision)?;
    let fresh = baseline(&before)?;
    if before.roadmap != after.roadmap {
        return Err(error(
            "REPLAN_SCOPE_CHANGED",
            "milestone identity, scope, dependencies or release binding changed",
        ));
    }
    if git(
        project,
        &["show", &format!("{baseline_revision}:.specbind.json")],
    )? != git(
        project,
        &["show", &format!("{recovery_revision}:.specbind.json")],
    )? {
        return Err(error(
            "REPLAN_SCOPE_CHANGED",
            "project configuration changed",
        ));
    }
    let eligible = eligible_targets(&before, &after, &fresh)?;
    let withdrawn = check_withdrawals(
        project,
        relative,
        recovery_revision,
        &head,
        &eligible,
        &after,
    )?;
    verify_live(project, root, &after, &eligible[..withdrawn.len()])?;
    if clean_head(project)? != head {
        return Err(error("REPLAN_HEAD_CHANGED", "HEAD changed during check"));
    }
    let remaining = &eligible[withdrawn.len()..];
    let changed = git(
        project,
        &[
            "log",
            "--format=",
            "--name-only",
            "-z",
            &format!("{baseline_revision}..{recovery_revision}"),
            "--",
        ],
    )?;
    let changed: std::collections::BTreeSet<_> =
        changed.split('\0').filter(|s| !s.is_empty()).collect();
    render(
        &serde_json::json!({"schemaVersion":1,"result":"REPLAN_COMPLETION_CHECKED",
        "baselineRevision":baseline_revision,"recoveryRevision":recovery_revision,"currentRevision":head,
        "milestoneId":after.roadmap.milestone_id,"eligible":eligible,"remaining":remaining,"withdrawn":withdrawn,
        "recoveryChangedPaths":changed,"requiresAuthorityAndDeltaReview":true}),
    )
}

fn eligible_targets(
    before: &Snapshot,
    after: &Snapshot,
    fresh: &[String],
) -> Result<Vec<String>, ReplanError> {
    if after.milestone.review_status != ReviewFreshnessStatus::Fresh {
        return Err(error(
            "REPLAN_REVIEW_REQUIRED",
            "recovery Contract Review must be fresh",
        ));
    }
    let mut eligible = Vec::new();
    for id in after.roadmap.spec_ids() {
        let old = before.specs[&id]
            .active_change
            .0
            .as_ref()
            .ok_or_else(|| error("REPLAN_BASELINE_INVALID", &id))?;
        let new = after.specs[&id]
            .active_change
            .0
            .as_ref()
            .ok_or_else(|| error("REPLAN_STATE_INVALID", &id))?;
        if old.requirement_ids != new.requirement_ids
            || old
                .gate_evidence
                .as_ref()
                .and_then(|g| g.requirements.as_ref())
                != new
                    .gate_evidence
                    .as_ref()
                    .and_then(|g| g.requirements.as_ref())
        {
            return Err(error("REPLAN_REQUIREMENTS_CHANGED", &id));
        }
        let state = &after.status[&id];
        if state.health == ConsistencyHealth::Consistent {
            continue;
        }
        let gates = &state.freshness;
        if !fresh.contains(&id)
            || before.specs[&id] != after.specs[&id]
            || state.declared_state != Some(WorkflowState::ReleaseReady)
            || [
                gates.requirements.status,
                gates.design.status,
                gates.tasks.status,
            ]
            .iter()
            .any(|v| *v != FreshnessStatus::Fresh)
            || gates.completion.status != FreshnessStatus::Stale
            || gates.completion.issues.is_empty()
            || gates
                .completion
                .issues
                .iter()
                .any(|i| i.code != "FRESHNESS_COMPLETION_PROJECT_CHANGED")
            || state
                .diagnostics
                .iter()
                .any(|i| i.code != "FRESHNESS_COMPLETION_PROJECT_CHANGED")
        {
            return Err(error(
                "REPLAN_TARGET_INELIGIBLE",
                format!("{id}: {:?}", state.diagnostics),
            ));
        }
        eligible.push(id);
    }
    for diagnostic in &after.milestone.diagnostics {
        if diagnostic.code != "MILESTONE_SPEC_INCONSISTENT"
            || !eligible
                .iter()
                .any(|id| diagnostic.path.as_deref() == Some(&format!("specs/{id}/spec.yaml")))
        {
            return Err(error("REPLAN_STATE_INVALID", format!("{diagnostic:?}")));
        }
    }
    Ok(eligible)
}

fn verify_live(
    project: &Path,
    root: &Path,
    expected: &Snapshot,
    withdrawn: &[String],
) -> Result<(), ReplanError> {
    let current = milestone_status::resolve(project, root)
        .map_err(|e| error("REPLAN_LIVE_STATE_CHANGED", format!("{:?}", e.diagnostics)))?
        .ok_or_else(|| error("REPLAN_LIVE_STATE_CHANGED", "milestone missing"))?;
    if current.milestone_id != expected.roadmap.milestone_id
        || current.review_status != expected.milestone.review_status
    {
        return Err(error(
            "REPLAN_LIVE_STATE_CHANGED",
            "live milestone or review differs from committed evidence",
        ));
    }
    let remaining_diagnostics: Vec<_> = expected
        .milestone
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            !withdrawn.iter().any(|id| {
                diagnostic.code == "MILESTONE_SPEC_INCONSISTENT"
                    && diagnostic.path.as_deref() == Some(&format!("specs/{id}/spec.yaml"))
            })
        })
        .cloned()
        .collect();
    if current.diagnostics != remaining_diagnostics {
        return Err(error(
            "REPLAN_LIVE_STATE_CHANGED",
            format!("{:?}", current.diagnostics),
        ));
    }
    for (id, wire) in &expected.specs {
        let mut wire = wire.clone();
        if withdrawn.contains(id) {
            let active = wire
                .active_change
                .0
                .as_mut()
                .ok_or_else(|| error("REPLAN_LIVE_STATE_CHANGED", id))?;
            active.state = WorkflowState::Implementation;
            if let Some(gates) = active.gate_evidence.as_mut() {
                gates.completion = None;
            }
        }
        let actual = artifacts::resolve_spec(root, id);
        if !actual.issues.is_empty() || actual.wire.as_ref() != Some(&wire) {
            return Err(error("REPLAN_LIVE_STATE_CHANGED", id));
        }
        let status = spec_status::resolve(project, root, id)
            .map_err(|e| error("REPLAN_LIVE_STATE_CHANGED", format!("{:?}", e.issues)))?;
        if withdrawn.contains(id) {
            if status.health != ConsistencyHealth::Consistent
                || status.freshness.completion.status != FreshnessStatus::NotReached
            {
                return Err(error("REPLAN_LIVE_STATE_CHANGED", id));
            }
        } else if status != expected.status[id] {
            return Err(error("REPLAN_LIVE_STATE_CHANGED", id));
        }
    }
    Ok(())
}

fn check_withdrawals(
    project: &Path,
    relative: &Path,
    recovery: &str,
    head: &str,
    eligible: &[String],
    snapshot: &Snapshot,
) -> Result<Vec<serde_json::Value>, ReplanError> {
    let commits = git(
        project,
        &[
            "rev-list",
            "--reverse",
            "--topo-order",
            &format!("{recovery}..{head}"),
        ],
    )?;
    let mut previous = recovery.to_owned();
    let mut done = Vec::new();
    for commit in commits.lines() {
        let id = eligible.get(done.len()).ok_or_else(|| {
            error(
                "REPLAN_CHECKPOINT_INVALID",
                "unexpected commit after withdrawals",
            )
        })?;
        let parent = git(project, &["show", "-s", "--format=%P", commit])?;
        if parent.trim() != previous {
            return Err(error(
                "REPLAN_CHECKPOINT_INVALID",
                "expected one ordered withdrawal parent",
            ));
        }
        let path = relative
            .join(format!("specs/{id}/spec.yaml"))
            .to_string_lossy()
            .replace('\\', "/");
        let changed = git(
            project,
            &[
                "diff-tree",
                "--no-commit-id",
                "--name-only",
                "-r",
                "-z",
                commit,
                "--",
            ],
        )?;
        if changed != format!("{path}\0") {
            return Err(error(
                "REPLAN_CHECKPOINT_INVALID",
                "checkpoint must change exactly one expected Spec file",
            ));
        }
        let mut expected = snapshot.specs[id].clone();
        let active = expected
            .active_change
            .0
            .as_mut()
            .ok_or_else(|| error("REPLAN_CHECKPOINT_INVALID", id))?;
        active.state = WorkflowState::Implementation;
        if let Some(gates) = active.gate_evidence.as_mut() {
            gates.completion = None;
        }
        let source = git(project, &["show", &format!("{commit}:{path}")])?;
        let actual = runtime::load_spec(&source)
            .map_err(|e| error("REPLAN_CHECKPOINT_INVALID", e.to_string()))?;
        if actual != expected {
            return Err(error(
                "REPLAN_CHECKPOINT_INVALID",
                "checkpoint changes more than completion",
            ));
        }
        done.push(serde_json::json!({"spec":id,"checkpoint":commit}));
        commit.clone_into(&mut previous);
    }
    Ok(done)
}
