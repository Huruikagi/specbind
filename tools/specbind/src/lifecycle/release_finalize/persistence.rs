//! Ordered guarded writes, final-state verification, and regular-file access.

use std::{fs, path::Path};

use super::{FinalizeIssues, one_issue, plan::FinalizationPlan};
use crate::{
    config::ProjectLanguage,
    guarded_fs,
    release_log::{self, LogUpdate},
    schema::runtime,
};

pub(super) fn apply_plan(
    specbind_root: &Path,
    plan: &FinalizationPlan,
) -> Result<(), FinalizeIssues> {
    ensure_release_directory(specbind_root)?;
    for spec in &plan.specs {
        if let LogUpdate::Updated(content) = &spec.log_update {
            guarded_fs::replace_optional(&specbind_root.join(&spec.log_path), content.as_bytes())
                .map_err(|error| {
                one_issue(
                    "RELEASE_LOG_WRITE_FAILED",
                    Some(spec.log_path.clone()),
                    error.to_string(),
                )
            })?;
        }
    }
    for spec in &plan.specs {
        for relative in &spec.cleanup {
            remove_regular(specbind_root, relative)?;
        }
        let spec_path = format!("specs/{}/spec.yaml", spec.spec);
        guarded_fs::replace_existing(&specbind_root.join(&spec_path), spec.spec_yaml.as_bytes())
            .map_err(|error| {
                one_issue(
                    "RELEASE_SPEC_WRITE_FAILED",
                    Some(spec_path),
                    error.to_string(),
                )
            })?;
    }
    if let Some(review_archive) = &plan.review_archive {
        move_regular(specbind_root, "state/contract-review.md", review_archive)?;
    }
    move_regular(specbind_root, "steering/roadmap.md", &plan.roadmap_archive)
}

pub(super) fn verify_final_state(
    specbind_root: &Path,
    plan: &FinalizationPlan,
) -> Result<(), FinalizeIssues> {
    if specbind_root.join("steering/roadmap.md").exists()
        || !specbind_root.join(&plan.roadmap_archive).is_file()
    {
        return Err(one_issue(
            "RELEASE_FINAL_STATE_INVALID",
            Some(plan.roadmap_archive.clone()),
            "Roadmap archive is not the finalization completion marker",
        ));
    }
    if let Some(review_archive) = &plan.review_archive
        && (specbind_root.join("state/contract-review.md").exists()
            || !specbind_root.join(review_archive).is_file())
    {
        return Err(one_issue(
            "RELEASE_FINAL_STATE_INVALID",
            Some(review_archive.clone()),
            "contract review did not reach its release archive",
        ));
    }
    for spec in &plan.specs {
        let spec_path = format!("specs/{}/spec.yaml", spec.spec);
        let source = read_regular(specbind_root, &spec_path, "RELEASE_FINAL_STATE_INVALID")?;
        let wire = runtime::load_spec(&source).map_err(|error| {
            one_issue(
                "RELEASE_SPEC_STRUCTURAL_INVALID",
                Some(spec_path.clone()),
                error.to_string(),
            )
        })?;
        if wire.active_change.0.is_some()
            || spec
                .cleanup
                .iter()
                .any(|path| specbind_root.join(path).exists())
        {
            return Err(one_issue(
                "RELEASE_FINAL_STATE_INVALID",
                Some(spec_path),
                "participating Spec did not reach the idle released state",
            ));
        }
        if let LogUpdate::Updated(expected) = &spec.log_update {
            let actual =
                read_regular(specbind_root, &spec.log_path, "RELEASE_FINAL_STATE_INVALID")?;
            if actual != *expected {
                return Err(one_issue(
                    "RELEASE_FINAL_STATE_INVALID",
                    Some(spec.log_path.clone()),
                    "participating Spec log does not match the rendered release entry",
                ));
            }
        }
    }
    Ok(())
}

fn ensure_release_directory(specbind_root: &Path) -> Result<(), FinalizeIssues> {
    let path = specbind_root.join("releases");
    match fs::symlink_metadata(&path) {
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() => {
            Err(one_issue(
                "RELEASE_ARCHIVE_ROOT_INVALID",
                Some("releases".to_owned()),
                "release archive root must be a regular non-symlink directory",
            ))
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&path)
            .map_err(|error| {
                one_issue(
                    "RELEASE_ARCHIVE_ROOT_CREATE_FAILED",
                    Some("releases".to_owned()),
                    error.to_string(),
                )
            }),
        Err(error) => Err(one_issue(
            "RELEASE_ARCHIVE_ROOT_UNAVAILABLE",
            Some("releases".to_owned()),
            error.to_string(),
        )),
    }
}

fn move_regular(
    specbind_root: &Path,
    source: &str,
    destination: &str,
) -> Result<(), FinalizeIssues> {
    let source_path = specbind_root.join(source);
    let destination_path = specbind_root.join(destination);
    let metadata = match fs::symlink_metadata(&source_path) {
        Ok(metadata) => metadata,
        Err(error)
            if error.kind() == std::io::ErrorKind::NotFound && destination_path.is_file() =>
        {
            return Ok(());
        }
        Err(error) => {
            return Err(one_issue(
                "RELEASE_MOVE_SOURCE_INVALID",
                Some(source.to_owned()),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "RELEASE_MOVE_SOURCE_INVALID",
            Some(source.to_owned()),
            "release archive source must be a regular non-symlink file",
        ));
    }
    if destination_path.exists() {
        return Err(one_issue(
            "RELEASE_DESTINATION_OCCUPIED",
            Some(destination.to_owned()),
            "release archive destination must be absent",
        ));
    }
    fs::rename(source_path, destination_path).map_err(|error| {
        one_issue(
            "RELEASE_ARCHIVE_MOVE_FAILED",
            Some(destination.to_owned()),
            error.to_string(),
        )
    })
}

fn remove_regular(specbind_root: &Path, relative: &str) -> Result<(), FinalizeIssues> {
    let path = specbind_root.join(relative);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(one_issue(
                "RELEASE_CLEANUP_FAILED",
                Some(relative.to_owned()),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "RELEASE_CLEANUP_FAILED",
            Some(relative.to_owned()),
            "release cleanup target must be a regular non-symlink file",
        ));
    }
    fs::remove_file(path).map_err(|error| {
        one_issue(
            "RELEASE_CLEANUP_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

pub(super) fn read_optional_log(
    specbind_root: &Path,
    relative: &str,
    language: ProjectLanguage,
) -> Result<String, FinalizeIssues> {
    match fs::symlink_metadata(specbind_root.join(relative)) {
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_file() => {
            Err(one_issue(
                "LOG_PROFILE_INVALID",
                Some(relative.to_owned()),
                "log.md must be a regular non-symlink file",
            ))
        }
        Ok(_) => read_regular(specbind_root, relative, "RELEASE_LOG_READ_FAILED"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(release_log::empty_log(language))
        }
        Err(error) => Err(one_issue(
            "RELEASE_LOG_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )),
    }
}

pub(super) fn read_regular(
    specbind_root: &Path,
    relative: &str,
    code: &'static str,
) -> Result<String, FinalizeIssues> {
    let path = specbind_root.join(relative);
    let metadata = fs::symlink_metadata(&path)
        .map_err(|error| one_issue(code, Some(relative.to_owned()), error.to_string()))?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            code,
            Some(relative.to_owned()),
            "release target must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(path)
        .map_err(|error| one_issue(code, Some(relative.to_owned()), error.to_string()))
}
