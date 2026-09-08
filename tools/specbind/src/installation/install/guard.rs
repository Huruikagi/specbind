//! Repository-state guard for replacement or retirement plans.

use std::{collections::BTreeSet, path::Path};

use crate::repository;

use super::{InstallIssues, PlanEntry, one_issue};

/// Decision 0077 permits creating new files in a repository without a commit.
/// Replacement or removal requires a commit and clean mutation targets. Dirty
/// paths outside the plan are reported for preservation rather than blocking
/// the refresh. An exact partial product-asset output can also be resumed.
pub(super) fn inspect_install_repository(
    project_root: &Path,
    entries: &[PlanEntry],
    require_existing_commit: bool,
) -> Result<Vec<String>, InstallIssues> {
    if require_existing_commit {
        require_commit(project_root)?;
    }
    let status = repository_status(project_root)?;
    inspect_status(project_root, &status, entries)
}

/// Adoption retirement is part of a revision-bound finalization workflow and
/// retains its complete clean-worktree requirement.
pub(super) fn require_clean_replaceable_repository(
    project_root: &Path,
) -> Result<(), InstallIssues> {
    require_commit(project_root)?;
    let status = repository_status(project_root)?;
    if status.is_empty() {
        Ok(())
    } else {
        Err(repository_not_clean())
    }
}

fn require_commit(project_root: &Path) -> Result<(), InstallIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))?;
    if !committed {
        return Err(one_issue(
            "INSTALL_COMMIT_REQUIRED",
            None,
            "replacing or removing an existing product-managed file requires at least one commit",
        ));
    }
    Ok(())
}

fn repository_status(project_root: &Path) -> Result<Vec<u8>, InstallIssues> {
    repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))
}

fn inspect_status(
    project_root: &Path,
    status: &[u8],
    entries: &[PlanEntry],
) -> Result<Vec<String>, InstallIssues> {
    let mut remaining = status;
    let mut unrelated = BTreeSet::new();
    let mut conflicts = Vec::new();
    while !remaining.is_empty() {
        if remaining.len() < 4 || remaining[2] != b' ' {
            return Err(repository_unclassified());
        }
        let (x, y) = (remaining[0], remaining[1]);
        let Some(end) = remaining[3..].iter().position(|byte| *byte == 0) else {
            return Err(repository_unclassified());
        };
        let first = &remaining[3..3 + end];
        remaining = &remaining[3 + end + 1..];
        let mut paths = vec![first];
        if matches!((x, y), (b'R' | b'C', _) | (_, b'R' | b'C')) {
            let Some(second_end) = remaining.iter().position(|byte| *byte == 0) else {
                return Err(repository_unclassified());
            };
            paths.push(&remaining[..second_end]);
            remaining = &remaining[second_end + 1..];
        }

        if paths.len() == 1
            && (is_exact_partial_install(project_root, x, y, first, entries)
                || is_safe_instruction_append(project_root, first, entries))
        {
            continue;
        }

        let mut touched_managed_target = false;
        for path in &paths {
            if entries.iter().any(|entry| {
                entry.path.as_bytes() == *path
                    && (entry.action != super::PlanAction::Keep || entry.resume_content.is_some())
            }) {
                touched_managed_target = true;
                conflicts.push(String::from_utf8_lossy(path).into_owned());
            }
        }
        if !touched_managed_target {
            unrelated.extend(
                paths
                    .into_iter()
                    .map(|path| String::from_utf8_lossy(path).into_owned()),
            );
        }
    }

    if conflicts.is_empty() {
        Ok(unrelated.into_iter().collect())
    } else {
        Err(InstallIssues {
            issues: conflicts
                .into_iter()
                .map(|path| {
                    repository_dirty(Some(path))
                        .issues
                        .into_iter()
                        .next()
                        .expect("one issue")
                })
                .collect(),
        })
    }
}

fn is_exact_partial_install(
    project_root: &Path,
    x: u8,
    y: u8,
    path: &[u8],
    entries: &[PlanEntry],
) -> bool {
    // Installation never stages files. A tracked replacement therefore
    // appears only in the worktree column, while a newly created asset is
    // untracked. Even byte-identical staged content is user-owned index state
    // and must keep blocking the refresh.
    if !matches!((x, y), (b' ', b'M') | (b'?', b'?')) {
        return false;
    }
    let Some(entry) = entries
        .iter()
        .find(|entry| entry.path.as_bytes() == path && entry.resume_content.is_some())
    else {
        return false;
    };
    let Some(expected) = entry.resume_content.as_deref() else {
        return false;
    };
    matches!(
        std::fs::read(project_root.join(&entry.path)),
        Ok(current) if current == expected.as_bytes()
    )
}

fn is_safe_instruction_append(project_root: &Path, path: &[u8], entries: &[PlanEntry]) -> bool {
    let Some(entry) = entries.iter().find(|entry| {
        entry.path.as_bytes() == path
            && entry.action == super::PlanAction::Create
            && entry.category == "project-instructions"
    }) else {
        return false;
    };
    let Some(expected) = entry.expected_current.as_deref() else {
        return false;
    };
    matches!(
        std::fs::read(project_root.join(&entry.path)),
        Ok(current) if current == expected.as_bytes()
    )
}

fn repository_dirty(path: Option<String>) -> InstallIssues {
    one_issue(
        "INSTALL_REPOSITORY_DIRTY",
        path,
        "mutating a product-managed target requires that target to be Git-clean",
    )
}

fn repository_not_clean() -> InstallIssues {
    one_issue(
        "INSTALL_REPOSITORY_DIRTY",
        None,
        "this operation requires a clean repository",
    )
}

fn repository_unclassified() -> InstallIssues {
    one_issue(
        "INSTALL_REPOSITORY_DIRTY",
        None,
        "repository changes could not be classified against the installation targets",
    )
}
