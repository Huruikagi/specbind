use super::{
    Path, RemovalAction, RemovalEntry, RemovalIssues, fs, guarded_fs, one_issue,
    project_instructions, repository,
};

pub(super) fn require_commit(project_root: &Path) -> Result<(), RemovalIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("REMOVAL_GIT_FAILED", None, error.to_string()))?;
    if committed {
        Ok(())
    } else {
        Err(one_issue(
            "REMOVAL_COMMIT_REQUIRED",
            None,
            "removal requires at least one Git commit as its recovery boundary",
        ))
    }
}

pub(super) fn validate_repository_state(
    project_root: &Path,
    entries: &[RemovalEntry],
    removable_tree: Option<&str>,
) -> Result<(), RemovalIssues> {
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("REMOVAL_GIT_FAILED", None, error.to_string()))?;
    let allowed_files = entries
        .iter()
        .filter(|entry| matches!(entry.action, RemovalAction::Remove | RemovalAction::Absent))
        .map(|entry| entry.path.as_str())
        .collect::<Vec<_>>();
    for record in status
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        if record.len() < 4 {
            return Err(one_issue(
                "REMOVAL_REPOSITORY_DIRTY",
                None,
                "Git returned an unreadable status record",
            ));
        }
        let code = &record[..2];
        let path = std::str::from_utf8(&record[3..]).map_err(|_| {
            one_issue(
                "REMOVAL_REPOSITORY_DIRTY",
                None,
                "dirty repository path is not UTF-8",
            )
        })?;
        let within_tree = removable_tree.is_some_and(|root| {
            path == root
                || path
                    .strip_prefix(root)
                    .is_some_and(|rest| rest.starts_with('/'))
        });
        if code == b" D" && (allowed_files.contains(&path) || within_tree) {
            continue;
        }
        let completed_instruction = entries.iter().any(|entry| {
            entry.path == path
                && entry.category == "project-instructions"
                && entry.action == RemovalAction::Absent
        });
        if code == b" M" && completed_instruction {
            continue;
        }
        return Err(one_issue(
            "REMOVAL_REPOSITORY_DIRTY",
            Some(path.to_owned()),
            "repository changes outside an already-removed exact target must be resolved first",
        ));
    }
    Ok(())
}

pub(super) fn validate_tree(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    validate_path_chain(project_root, relative)?;
    let root = project_root.join(relative);
    let metadata = fs::symlink_metadata(&root).map_err(|error| {
        one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "REMOVAL_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "configured specDir must remain a regular non-link directory",
        ));
    }
    validate_directory(project_root, &root)?;
    Ok(())
}

pub(super) fn completed_instruction_removal(
    project_root: &Path,
    relative: &str,
    current: &[u8],
) -> Result<bool, RemovalIssues> {
    let revision_path = format!("HEAD:{relative}");
    let Ok(head) = repository::output_bytes(project_root, &["show", &revision_path]) else {
        return Ok(false);
    };
    let head = String::from_utf8(head).map_err(|_| {
        one_issue(
            "REMOVAL_TARGET_NOT_UTF8",
            Some(relative.to_owned()),
            "committed agent instruction file must be UTF-8",
        )
    })?;
    let Some(expected) = project_instructions::remove(&head)
        .map_err(|error| one_issue(error.code, Some(relative.to_owned()), error.message))?
    else {
        return Ok(false);
    };
    Ok(expected.as_bytes() == current)
}

fn validate_directory(project_root: &Path, directory: &Path) -> Result<(), RemovalIssues> {
    for child in fs::read_dir(directory)
        .map_err(|error| one_issue("REMOVAL_TARGET_UNREADABLE", None, error.to_string()))?
    {
        let child = child
            .map_err(|error| one_issue("REMOVAL_TARGET_UNREADABLE", None, error.to_string()))?;
        let path = child.path();
        let relative = path
            .strip_prefix(project_root)
            .expect("walked removal target stays below project root")
            .to_string_lossy()
            .replace('\\', "/");
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            one_issue(
                "REMOVAL_TARGET_UNREADABLE",
                Some(relative.clone()),
                error.to_string(),
            )
        })?;
        if guarded_fs::is_link_like(&metadata) {
            return Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative),
                "knowledge removal rejects links, junctions, and reparse points",
            ));
        }
        validate_not_ignored(project_root, &relative)?;
        if metadata.is_dir() {
            validate_directory(project_root, &path)?;
        } else if metadata.is_file() {
            validate_tracked(project_root, &relative)?;
        } else {
            return Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative),
                "knowledge removal accepts only regular files and directories",
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_path_chain(
    project_root: &Path,
    relative: &str,
) -> Result<(), RemovalIssues> {
    let mut current = project_root.to_path_buf();
    for component in relative.split('/') {
        current.push(component);
        let metadata = fs::symlink_metadata(&current).map_err(|error| {
            one_issue(
                "REMOVAL_TARGET_UNREADABLE",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })?;
        if guarded_fs::is_link_like(&metadata) {
            return Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative.to_owned()),
                "removal target traversal rejects links, junctions, and reparse points",
            ));
        }
    }
    Ok(())
}

pub(super) fn validate_tracked_not_ignored(
    project_root: &Path,
    relative: &str,
) -> Result<(), RemovalIssues> {
    validate_tracked(project_root, relative)?;
    validate_not_ignored(project_root, relative)
}

fn validate_tracked(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    let tracked = repository::predicate(
        project_root,
        &["ls-files", "--error-unmatch", "--", relative],
    )
    .map_err(|error| {
        one_issue(
            "REMOVAL_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if tracked {
        Ok(())
    } else {
        Err(one_issue(
            "REMOVAL_TARGET_UNTRACKED",
            Some(relative.to_owned()),
            "removal deletes only Git-tracked targets",
        ))
    }
}

fn validate_not_ignored(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    let ignored = repository::predicate(
        project_root,
        &["check-ignore", "--no-index", "-q", "--", relative],
    )
    .map_err(|error| {
        one_issue(
            "REMOVAL_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if ignored {
        Err(one_issue(
            "REMOVAL_TARGET_IGNORED",
            Some(relative.to_owned()),
            "ignored targets are not safe removal inputs",
        ))
    } else {
        Ok(())
    }
}

pub(super) fn revalidate_file(
    project_root: &Path,
    entry: &RemovalEntry,
) -> Result<(), RemovalIssues> {
    validate_path_chain(project_root, &entry.path)?;
    let path = project_root.join(&entry.path);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            "target is no longer a regular non-link file",
        ));
    }
    let current = fs::read(path).map_err(|error| {
        one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            error.to_string(),
        )
    })?;
    if entry.expected_current.as_deref() != Some(current.as_slice()) {
        return Err(one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            "target content changed after planning",
        ));
    }
    Ok(())
}
