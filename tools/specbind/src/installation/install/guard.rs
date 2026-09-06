//! Repository-state guard for replacement or retirement plans.

use std::path::Path;

use crate::repository;

use super::{InstallIssues, PlanEntry, one_issue};

/// Decision 0077 permits creating new files in a repository without a commit.
/// Replacement or removal requires a commit and either a clean worktree or an
/// exact partial product-asset output that this plan can safely resume.
pub(super) fn require_replaceable_repository(
    project_root: &Path,
    entries: &[PlanEntry],
) -> Result<(), InstallIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))?;
    if !committed {
        return Err(one_issue(
            "INSTALL_COMMIT_REQUIRED",
            None,
            "replacing or removing an existing product-managed file requires at least one commit",
        ));
    }
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))?;
    if status.is_empty() || is_exact_partial_install(project_root, &status, entries) {
        Ok(())
    } else {
        Err(one_issue(
            "INSTALL_REPOSITORY_DIRTY",
            None,
            "replacing or removing an existing product-managed file requires a clean repository",
        ))
    }
}

/// Accepts only dirty paths whose current bytes are exact product-managed
/// outputs from this plan. This lets a failed multi-asset refresh converge on
/// retry without admitting unrelated or project-owned worktree changes.
fn is_exact_partial_install(project_root: &Path, status: &[u8], entries: &[PlanEntry]) -> bool {
    let mut remaining = status;
    while !remaining.is_empty() {
        if remaining.len() < 4 || remaining[2] != b' ' {
            return false;
        }
        // Rename and copy records carry a second NUL-delimited path. Neither is
        // a state produced by installation, so fail closed instead of parsing
        // it as a separate status record.
        if matches!(remaining[0], b'R' | b'C') || matches!(remaining[1], b'R' | b'C') {
            return false;
        }
        // Installation never stages files. A tracked replacement therefore
        // appears only in the worktree column, while a newly created asset is
        // untracked. Even byte-identical staged content is user-owned index
        // state and must keep blocking the refresh.
        if !matches!((remaining[0], remaining[1]), (b' ', b'M') | (b'?', b'?')) {
            return false;
        }
        let Some(end) = remaining[3..].iter().position(|byte| *byte == 0) else {
            return false;
        };
        let path = &remaining[3..3 + end];
        let Some(entry) = entries
            .iter()
            .find(|entry| entry.path.as_bytes() == path && entry.resume_content.is_some())
        else {
            return false;
        };
        let Some(expected) = entry.resume_content.as_deref() else {
            return false;
        };
        if !matches!(
            std::fs::read(project_root.join(&entry.path)),
            Ok(current) if current == expected.as_bytes()
        ) {
            return false;
        }
        remaining = &remaining[3 + end + 1..];
    }
    true
}
