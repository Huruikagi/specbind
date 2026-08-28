//! Repository-state guard for replacement plans.

use std::path::Path;

use crate::repository;

use super::{InstallIssues, one_issue};

/// Decision 0077 permits creating new files in a repository without a commit,
/// but any replacement of an existing file requires a committed clean state.
pub(super) fn require_replaceable_repository(project_root: &Path) -> Result<(), InstallIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))?;
    if !committed {
        return Err(one_issue(
            "INSTALL_COMMIT_REQUIRED",
            None,
            "replacing an existing file requires at least one commit",
        ));
    }
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))?;
    if status.is_empty() {
        Ok(())
    } else {
        Err(one_issue(
            "INSTALL_REPOSITORY_DIRTY",
            None,
            "replacing an existing file requires a clean repository",
        ))
    }
}
