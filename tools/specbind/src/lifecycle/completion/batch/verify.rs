//! Read-only proof of one narrow metadata checkpoint for a supplied batch.

use std::{collections::BTreeSet, path::Path};

use crate::{
    freshness::FreshnessStatus,
    schema::{runtime, spec::v1::WorkflowState},
    spec_status,
};

use super::{
    BatchReport, CompletionIssues, ValidatedCandidate, clean_head, git, one_issue, parse,
    read_regular, report, same_clean_head, spec_path,
};

/// Verifies a clean single checkpoint and fresh evidence for every candidate.
/// # Errors
/// Refuses moved, split, mixed or incomplete checkpoints and stale evidence.
pub fn verify(project: &Path, root: &Path, source: &str) -> Result<BatchReport, CompletionIssues> {
    let input = parse(source)?;
    let head = clean_head(project)?;
    if git(project, &["show", "-s", "--format=%P", &head])?.trim() != input.revision {
        return Err(one_issue(
            "COMPLETION_BATCH_CHECKPOINT_INVALID",
            None,
            "HEAD must be one single-parent completion checkpoint after the implementation revision",
        ));
    }
    let mut result = report(project, root, input.revision.clone(), &input.ids)?;
    let changed = git(
        project,
        &[
            "diff-tree",
            "--no-commit-id",
            "--name-only",
            "--no-renames",
            "-r",
            "-z",
            &head,
            "--",
        ],
    )?;
    let changed: BTreeSet<_> = changed.split('\0').filter(|s| !s.is_empty()).collect();
    if changed != result.paths.iter().map(String::as_str).collect() {
        return Err(one_issue(
            "COMPLETION_BATCH_CHECKPOINT_INVALID",
            None,
            "checkpoint must change exactly the supplied Spec metadata paths",
        ));
    }
    for ((id, candidate), path) in input.ids.iter().zip(&input.candidates).zip(&result.paths) {
        verify_target(project, root, &input.revision, &head, id, path, candidate)?;
    }
    same_clean_head(project, &head)?;
    result.result = "COMPLETION_BATCH_VERIFIED";
    result.accepted = input.ids;
    result.pending.clear();
    Ok(result)
}

fn verify_target(
    project: &Path,
    root: &Path,
    revision: &str,
    head: &str,
    id: &str,
    path: &str,
    candidate: &ValidatedCandidate,
) -> Result<(), CompletionIssues> {
    let invalid = || {
        one_issue(
            "COMPLETION_BATCH_CHECKPOINT_INVALID",
            Some(path.to_owned()),
            "checkpoint must preserve earlier state and record only the supplied completion evidence",
        )
    };
    let old_mode = git(
        project,
        &["ls-tree", "--format=%(objectmode)", revision, "--", path],
    )?;
    let mode = git(
        project,
        &["ls-tree", "--format=%(objectmode)", head, "--", path],
    )?;
    if old_mode != mode || !matches!(mode.trim(), "100644" | "100755") {
        return Err(invalid());
    }
    let before = git(project, &["show", &format!("{revision}:{path}")])?;
    let mut expected = runtime::load_spec(&before).map_err(|_| invalid())?;
    let actual = runtime::load_spec(&read_regular(&root.join(spec_path(id)), &spec_path(id))?)
        .map_err(|_| invalid())?;
    let active = expected.active_change.0.as_mut().ok_or_else(invalid)?;
    let evidence = active.gate_evidence.as_mut().ok_or_else(invalid)?;
    if active.state != WorkflowState::Implementation || evidence.completion.is_some() {
        return Err(invalid());
    }
    let completion = actual
        .active_change
        .0
        .as_ref()
        .and_then(|a| a.gate_evidence.as_ref())
        .and_then(|g| g.completion.as_ref())
        .ok_or_else(invalid)?;
    if completion.implementation_revision.0 != revision
        || completion.mechanical_checks != candidate.mechanical_checks
    {
        return Err(invalid());
    }
    active.state = WorkflowState::ReleaseReady;
    evidence.completion = Some(completion.clone());
    if actual != expected {
        return Err(invalid());
    }
    let status = spec_status::resolve(project, root, id).map_err(|_| invalid())?;
    if status.health != spec_status::ConsistencyHealth::Consistent
        || status.freshness.completion.status != FreshnessStatus::Fresh
    {
        return Err(one_issue(
            "COMPLETION_BATCH_NOT_FRESH",
            Some(path.to_owned()),
            "accepted completion is not fresh after its checkpoint",
        ));
    }
    Ok(())
}
