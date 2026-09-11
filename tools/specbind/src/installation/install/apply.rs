//! Guarded installation plan application.

use std::{collections::BTreeSet, fs, path::Path};

use crate::guarded_fs;

use super::{InstallInputs, InstallIssues, InstallOutcome, PlanAction, PlanEntry, one_issue, plan};

/// Applies a freshly computed plan, writing the Roadmap-style config last.
///
/// # Errors
///
/// Returns planning, race, or guarded-write diagnostics. A failure may leave
/// earlier product-managed assets written; a later run recognizes their exact
/// current-binary output and continues the remaining plan.
pub fn apply(project_root: &Path, inputs: &InstallInputs) -> Result<InstallOutcome, InstallIssues> {
    let plan = plan(project_root, inputs)?;
    let unchanged = plan
        .entries
        .iter()
        .all(|entry| entry.action == PlanAction::Keep);
    if unchanged {
        return Ok(InstallOutcome {
            plan,
            unchanged: true,
        });
    }
    apply_entries(project_root, &plan.entries)?;
    Ok(InstallOutcome {
        plan,
        unchanged: false,
    })
}

pub(super) fn apply_entries(
    project_root: &Path,
    entries: &[PlanEntry],
) -> Result<(), InstallIssues> {
    // Assets first, configuration last: a project only claims the selected
    // capability state once the matching assets exist or have been retired.
    let ordered = entries
        .iter()
        .filter(|entry| entry.category != "config")
        .chain(entries.iter().filter(|entry| entry.category == "config"))
        .collect::<Vec<_>>();
    let mut applied = Vec::new();
    for (index, entry) in ordered.iter().enumerate() {
        if entry.action == PlanAction::Keep {
            continue;
        }
        if let Err(error) = apply_entry(project_root, entry) {
            let mut confirmed_applied = applied.clone();
            let pending = if matches_desired_state(project_root, entry) {
                confirmed_applied.push(*entry);
                &ordered[index + 1..]
            } else {
                &ordered[index..]
            };
            return Err(with_partial_apply_context(
                error,
                &confirmed_applied,
                pending,
            ));
        }
        applied.push(*entry);
    }
    Ok(())
}

fn apply_entry(project_root: &Path, entry: &PlanEntry) -> Result<(), InstallIssues> {
    let target = project_root.join(&entry.path);
    if entry.action == PlanAction::Remove {
        verify_expected_state(&target, entry)?;
        fs::remove_file(&target).map_err(|error| {
            one_issue(
                "INSTALL_WRITE_FAILED",
                Some(entry.path.clone()),
                error.to_string(),
            )
        })?;
        if let Some(parent) = target.parent() {
            match fs::remove_dir(parent) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::DirectoryNotEmpty => {}
                Err(error) => {
                    return Err(one_issue(
                        "INSTALL_WRITE_FAILED",
                        Some(entry.path.clone()),
                        error.to_string(),
                    ));
                }
            }
        }
        return Ok(());
    }
    let Some(content) = entry.content.as_deref() else {
        return Ok(());
    };
    verify_expected_state(&target, entry)?;
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            one_issue(
                "INSTALL_WRITE_FAILED",
                Some(entry.path.clone()),
                error.to_string(),
            )
        })?;
    }
    guarded_fs::replace_optional(&target, content.as_bytes()).map_err(|error| {
        one_issue(
            "INSTALL_WRITE_FAILED",
            Some(entry.path.clone()),
            error.to_string(),
        )
    })?;
    Ok(())
}

fn with_partial_apply_context(
    mut error: InstallIssues,
    applied: &[&PlanEntry],
    pending: &[&PlanEntry],
) -> InstallIssues {
    if applied.is_empty() {
        return error;
    }
    error.issues.extend(applied.iter().map(|entry| {
        super::issue(
            "INSTALL_APPLIED_BEFORE_FAILURE",
            Some(entry.path.clone()),
            format!(
                "planned action `{}` was applied before installation stopped",
                entry.action.name()
            ),
        )
    }));
    error.issues.extend(
        pending
            .iter()
            .filter(|entry| entry.action != PlanAction::Keep)
            .map(|entry| {
                super::issue(
                    "INSTALL_PENDING_AFTER_FAILURE",
                    Some(entry.path.clone()),
                    format!(
                        "planned action `{}` remains pending after installation stopped",
                        entry.action.name()
                    ),
                )
            }),
    );

    let directing_packages = applied
        .iter()
        .filter_map(|entry| {
            entry
                .path
                .split_once("/skills/sb-configure/")
                .map(|(root, _)| format!("{root}/skills/sb-configure"))
        })
        .collect::<BTreeSet<_>>();
    if directing_packages.is_empty() {
        error.issues.push(super::issue(
            "INSTALL_DIRECTING_SKILL_UNCHANGED",
            None,
            "no sb-configure package target was applied before installation stopped",
        ));
    } else {
        error.issues.push(super::issue(
            "INSTALL_DIRECTING_SKILL_CHANGED",
            None,
            format!(
                "sb-configure package content changed before installation stopped: {}; reload sb-configure after recovery",
                directing_packages.into_iter().collect::<Vec<_>>().join(", ")
            ),
        ));
    }
    error.issues.push(super::issue(
        "INSTALL_RECOVERY_REQUIRED",
        None,
        "correct the reported filesystem problem, rerun `specbind install`, then run `specbind install --dry-run` and inspect the repository diff",
    ));
    error
}

fn matches_desired_state(project_root: &Path, entry: &PlanEntry) -> bool {
    let target = project_root.join(&entry.path);
    match entry.action {
        PlanAction::Create | PlanAction::Replace => entry
            .content
            .as_ref()
            .is_some_and(|content| fs::read(target).is_ok_and(|bytes| bytes == content.as_bytes())),
        PlanAction::Remove => fs::symlink_metadata(target)
            .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound),
        PlanAction::Keep => true,
    }
}

/// Fails closed when the filesystem no longer matches the planned action.
fn verify_expected_state(target: &Path, entry: &PlanEntry) -> Result<(), InstallIssues> {
    let present = match fs::symlink_metadata(target) {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(one_issue(
                "INSTALL_TARGET_UNREADABLE",
                Some(entry.path.clone()),
                error.to_string(),
            ));
        }
    };
    if entry.action == PlanAction::Keep {
        return Ok(());
    }
    if let Some(expected) = &entry.expected_current {
        // An in-place edit leaves the file present either way, so presence
        // proves nothing. Compare the bytes the plan actually read.
        return match fs::read(target) {
            Ok(current) if current == expected.as_bytes() => Ok(()),
            Ok(_) => Err(one_issue(
                "INSTALL_TARGET_CHANGED",
                Some(entry.path.clone()),
                "installation target changed after the plan was computed",
            )),
            Err(error) => Err(one_issue(
                "INSTALL_TARGET_UNREADABLE",
                Some(entry.path.clone()),
                error.to_string(),
            )),
        };
    }
    let expected = match entry.action {
        PlanAction::Create => false,
        PlanAction::Replace | PlanAction::Remove => true,
        PlanAction::Keep => return Ok(()),
    };
    if present == expected {
        Ok(())
    } else {
        Err(one_issue(
            "INSTALL_TARGET_CHANGED",
            Some(entry.path.clone()),
            "installation target changed after the plan was computed",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn replace(path: &str, content: &str, expected_current: Option<&str>) -> PlanEntry {
        PlanEntry {
            action: PlanAction::Replace,
            path: path.to_owned(),
            category: "skill",
            detail: None,
            content: Some(content.to_owned()),
            expected_current: expected_current.map(str::to_owned),
            resume_content: None,
        }
    }

    #[test]
    fn reports_applied_pending_directing_skill_and_recovery_after_partial_failure() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join(".agents/skills/sb-configure")).unwrap();
        fs::write(
            root.path().join(".agents/skills/sb-configure/SKILL.md"),
            "old\n",
        )
        .unwrap();
        fs::create_dir(root.path().join("blocked")).unwrap();
        fs::write(root.path().join("later.md"), "old\n").unwrap();
        let entries = [
            replace(
                ".agents/skills/sb-configure/SKILL.md",
                "updated\n",
                Some("old\n"),
            ),
            replace("blocked", "pending\n", None),
            replace("later.md", "later\n", Some("old\n")),
        ];

        let error = apply_entries(root.path(), &entries).unwrap_err();

        assert_eq!(
            error
                .issues
                .iter()
                .map(|issue| (issue.code, issue.path.as_deref()))
                .collect::<Vec<_>>(),
            [
                ("INSTALL_WRITE_FAILED", Some("blocked")),
                (
                    "INSTALL_APPLIED_BEFORE_FAILURE",
                    Some(".agents/skills/sb-configure/SKILL.md")
                ),
                ("INSTALL_PENDING_AFTER_FAILURE", Some("blocked")),
                ("INSTALL_PENDING_AFTER_FAILURE", Some("later.md")),
                ("INSTALL_DIRECTING_SKILL_CHANGED", None),
                ("INSTALL_RECOVERY_REQUIRED", None),
            ]
        );
        assert_eq!(
            fs::read_to_string(root.path().join(".agents/skills/sb-configure/SKILL.md")).unwrap(),
            "updated\n"
        );
        assert_eq!(
            fs::read_to_string(root.path().join("later.md")).unwrap(),
            "old\n"
        );
    }
}
