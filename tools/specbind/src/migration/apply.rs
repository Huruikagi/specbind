//! Guarded deterministic application and final cc-sdd source retirement.

use std::{fs, path::Path};

use super::{
    LEGACY_CONFIG, LEGACY_SKILLS, MigrationIssues, MigrationOutcome, finding, inventory, one_issue,
};
use crate::{
    guarded_fs,
    install::{self, PlanAction},
    migration_resolution, repository,
};

pub(super) fn apply(project_root: &Path) -> Result<MigrationOutcome, MigrationIssues> {
    let plan = super::plan(project_root)?;
    if !plan.findings.is_empty() {
        return Err(one_issue(
            "MIGRATION_PLAN_CHANGED",
            None,
            "the freshly recomputed migration plan contains findings",
        ));
    }
    let inputs = inventory::install_inputs(plan.language, &plan.agents).ok_or_else(|| {
        one_issue(
            "MIGRATION_PLAN_CHANGED",
            None,
            "the freshly recomputed plan has no complete installation inputs",
        )
    })?;
    let install_plan = install::plan(project_root, &inputs).map_err(render_install_issues)?;
    let legacy_assets = plan
        .actions
        .iter()
        .filter(|action| action.kind == "remove-after-cutover")
        .filter_map(|action| action.source.clone())
        .collect::<Vec<_>>();
    let legacy_config = inventory::path_exists(&project_root.join(LEGACY_CONFIG))?;
    let resolution_state =
        inventory::path_exists(&project_root.join(migration_resolution::STATE_RELATIVE))?;
    let mut cleanup_targets = legacy_assets.clone();
    cleanup_targets.push(plan.legacy_root.clone());
    if legacy_config {
        cleanup_targets.push(LEGACY_CONFIG.to_owned());
    }
    if resolution_state {
        cleanup_targets.push(migration_resolution::STATE_RELATIVE.to_owned());
    }
    require_apply_repository(project_root, &cleanup_targets)?;
    let installed_files = install_plan
        .entries
        .iter()
        .filter(|entry| entry.action != PlanAction::Keep)
        .count();
    install::apply(project_root, &inputs).map_err(render_install_issues)?;
    for relative in &legacy_assets {
        remove_legacy_asset(project_root, relative)?;
    }
    remove_cleanup_target(project_root, &plan.legacy_root)?;
    if legacy_config {
        remove_cleanup_target(project_root, LEGACY_CONFIG)?;
    }
    if resolution_state {
        remove_cleanup_target(project_root, migration_resolution::STATE_RELATIVE)?;
    }
    Ok(MigrationOutcome {
        installed_files,
        removed_legacy_assets: legacy_assets.len(),
        removed_legacy_root: plan.legacy_root,
        removed_legacy_config: legacy_config,
        removed_resolution_state: resolution_state,
    })
}

fn render_install_issues(error: install::InstallIssues) -> MigrationIssues {
    MigrationIssues {
        issues: error
            .issues
            .into_iter()
            .map(|issue| finding(issue.code, issue.path, issue.message))
            .collect(),
    }
}

fn require_apply_repository(
    project_root: &Path,
    cleanup_targets: &[String],
) -> Result<(), MigrationIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("MIGRATION_GIT_FAILED", None, error.to_string()))?;
    if !committed {
        return Err(one_issue(
            "MIGRATION_COMMIT_REQUIRED",
            None,
            "cc-sdd migration apply requires at least one commit",
        ));
    }
    for relative in cleanup_targets {
        require_cleanup_target_tracked(project_root, relative)?;
    }
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("MIGRATION_GIT_FAILED", None, error.to_string()))?;
    if !status.is_empty() {
        return Err(one_issue(
            "MIGRATION_REPOSITORY_DIRTY",
            None,
            "cc-sdd migration apply requires a clean repository",
        ));
    }
    Ok(())
}

fn require_cleanup_target_tracked(
    project_root: &Path,
    relative: &str,
) -> Result<(), MigrationIssues> {
    let path = project_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup targets must not contain links or reparse points",
        ));
    }
    if metadata.is_file() {
        return require_file_tracked(project_root, relative);
    }
    if !metadata.is_dir() {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must be a regular file or directory",
        ));
    }
    for entry in fs::read_dir(&path).map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })? {
        let entry = entry.map_err(|error| {
            one_issue(
                "MIGRATION_CLEANUP_TARGET_CHANGED",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })?;
        let child = entry
            .path()
            .strip_prefix(project_root)
            .expect("cleanup target stays below project root")
            .to_string_lossy()
            .replace('\\', "/");
        require_cleanup_target_tracked(project_root, &child)?;
    }
    Ok(())
}

fn require_file_tracked(project_root: &Path, relative: &str) -> Result<(), MigrationIssues> {
    let tracked = repository::output_bytes(project_root, &["ls-files", "-z", "--", relative])
        .map_err(|error| {
            one_issue(
                "MIGRATION_GIT_FAILED",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })?;
    if tracked.is_empty() {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNTRACKED",
            Some(relative.to_owned()),
            "final cutover deletes only Git-tracked files",
        ));
    }
    Ok(())
}

fn remove_legacy_asset(project_root: &Path, relative: &str) -> Result<(), MigrationIssues> {
    let known = [".agents/skills", ".claude/skills"].iter().any(|root| {
        LEGACY_SKILLS
            .iter()
            .any(|name| relative == format!("{root}/{name}"))
    });
    if !known {
        return Err(one_issue(
            "MIGRATION_LEGACY_ASSET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target is not an exact known skill path",
        ));
    }
    let path = project_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_ASSET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "MIGRATION_LEGACY_ASSET_CHANGED",
            Some(relative.to_owned()),
            "legacy cleanup target is no longer a regular non-symlink directory",
        ));
    }
    let status = repository::path_status(project_root, relative).map_err(|error| {
        one_issue(
            "MIGRATION_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if !status.is_empty() {
        return Err(one_issue(
            "MIGRATION_LEGACY_ASSET_DIRTY",
            Some(relative.to_owned()),
            "legacy cleanup target changed after the recovery boundary was established",
        ));
    }
    fs::remove_dir_all(&path).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_ASSET_REMOVE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

fn remove_cleanup_target(project_root: &Path, relative: &str) -> Result<(), MigrationIssues> {
    require_cleanup_target_tracked(project_root, relative)?;
    let status = repository::path_status(project_root, relative).map_err(|error| {
        one_issue(
            "MIGRATION_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if !status.is_empty() {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            "legacy cleanup target changed after the recovery boundary was established",
        ));
    }
    let canonical_project = project_root
        .canonicalize()
        .map_err(|error| one_issue("MIGRATION_GIT_FAILED", None, error.to_string()))?;
    let path = project_root.join(relative);
    let canonical = path.canonicalize().map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if canonical == canonical_project || !canonical.starts_with(&canonical_project) {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must stay below the project root",
        ));
    }
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must not be a link or reparse point",
        ));
    }
    if metadata.is_dir() {
        fs::remove_dir_all(path)
    } else if metadata.is_file() {
        fs::remove_file(path)
    } else {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must be a regular file or directory",
        ));
    }
    .map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_REMOVE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}
