//! Guarded installation plan application.

use std::{fs, path::Path};

use crate::guarded_fs;

use super::{InstallInputs, InstallIssues, InstallOutcome, PlanAction, PlanEntry, one_issue, plan};

/// Applies a freshly computed plan, writing the Roadmap-style config last.
///
/// # Errors
///
/// Returns planning, race, or guarded-write diagnostics. A failure may leave
/// earlier assets written; a later run converges because missing defaults are
/// created and existing project files are kept.
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
    // Assets first, configuration last: a project only claims to be installed
    // once the assets its skills read actually exist.
    let ordered = plan
        .entries
        .iter()
        .filter(|entry| entry.category != "config")
        .chain(
            plan.entries
                .iter()
                .filter(|entry| entry.category == "config"),
        );
    for entry in ordered {
        let Some(content) = entry.content.as_deref() else {
            continue;
        };
        let target = project_root.join(&entry.path);
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
    }
    Ok(InstallOutcome {
        plan,
        unchanged: false,
    })
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
        PlanAction::Replace => true,
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
