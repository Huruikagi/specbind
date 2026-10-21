//! Apply a complete reverse-establishment plan in the established write order.

use std::{fs, path::Path};

use super::{FinalizeIssues, one_issue, plan::FinalizationPlan};
use crate::{guarded_fs, release_log::LogUpdate};

pub(super) fn apply_plan(
    specbind_root: &Path,
    plan: &FinalizationPlan,
) -> Result<(), FinalizeIssues> {
    ensure_baselines_directory(specbind_root)?;
    for spec in &plan.specs {
        if let LogUpdate::Updated(content) = &spec.log_update {
            guarded_fs::replace_optional(&specbind_root.join(&spec.log_path), content.as_bytes())
                .map_err(|error| {
                one_issue(
                    "ADOPTION_LOG_WRITE_FAILED",
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
                    "ADOPTION_SPEC_WRITE_FAILED",
                    Some(spec_path),
                    error.to_string(),
                )
            })?;
    }
    move_regular(
        specbind_root,
        "state/contract-review.md",
        &plan.review_archive,
    )?;
    move_regular(specbind_root, "steering/roadmap.md", &plan.roadmap_archive)?;
    let dossier = "adoption/reverse-discovery.yaml";
    if specbind_root.join(dossier).exists() {
        remove_regular(specbind_root, dossier)?;
    }
    Ok(())
}

fn ensure_baselines_directory(specbind_root: &Path) -> Result<(), FinalizeIssues> {
    let path = specbind_root.join("baselines");
    match fs::symlink_metadata(&path) {
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() => {
            Err(one_issue(
                "ADOPTION_ARCHIVE_ROOT_INVALID",
                Some("baselines".to_owned()),
                "baseline archive root must be a regular non-symlink directory",
            ))
        }
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => fs::create_dir(&path)
            .map_err(|error| {
                one_issue(
                    "ADOPTION_ARCHIVE_ROOT_CREATE_FAILED",
                    Some("baselines".to_owned()),
                    error.to_string(),
                )
            }),
        Err(error) => Err(one_issue(
            "ADOPTION_ARCHIVE_ROOT_INVALID",
            Some("baselines".to_owned()),
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
    let metadata = fs::symlink_metadata(&source_path).map_err(|error| {
        one_issue(
            "ADOPTION_ARCHIVE_SOURCE_INVALID",
            Some(source.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "ADOPTION_ARCHIVE_SOURCE_INVALID",
            Some(source.to_owned()),
            "archive source must be a regular non-symlink file",
        ));
    }
    fs::rename(source_path, specbind_root.join(destination)).map_err(|error| {
        one_issue(
            "ADOPTION_ARCHIVE_MOVE_FAILED",
            Some(destination.to_owned()),
            error.to_string(),
        )
    })
}

pub(super) fn remove_regular(specbind_root: &Path, relative: &str) -> Result<(), FinalizeIssues> {
    let path = specbind_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative.to_owned()),
            "cleanup target must be a regular non-symlink file",
        ));
    }
    fs::remove_file(path).map_err(|error| {
        one_issue(
            "ADOPTION_CLEANUP_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::super::plan::SpecPlan;
    use super::*;

    fn fixture() -> (tempfile::TempDir, FinalizationPlan) {
        let root = tempfile::tempdir().unwrap();
        let mut specs = Vec::new();
        for name in ["alpha", "beta"] {
            fs::create_dir_all(root.path().join(format!("specs/{name}"))).unwrap();
            fs::write(
                root.path().join(format!("specs/{name}/spec.yaml")),
                "before",
            )
            .unwrap();
            let brief = format!("specs/{name}/brief.md");
            fs::write(root.path().join(&brief), "brief").unwrap();
            specs.push(SpecPlan {
                spec: name.to_owned(),
                spec_yaml: "after".to_owned(),
                log_path: format!("specs/{name}/log.md"),
                log_update: LogUpdate::Updated("baseline log".to_owned()),
                cleanup: vec![brief],
            });
        }
        for (directory, file) in [
            ("state", "contract-review.md"),
            ("steering", "roadmap.md"),
            ("adoption", "reverse-discovery.yaml"),
        ] {
            fs::create_dir(root.path().join(directory)).unwrap();
            fs::write(root.path().join(directory).join(file), file).unwrap();
        }
        (
            root,
            FinalizationPlan {
                version: "v1".to_owned(),
                specs,
                roadmap_archive: "baselines/v1-roadmap.md".to_owned(),
                review_archive: "baselines/v1-contract-review.md".to_owned(),
            },
        )
    }

    #[test]
    fn later_log_failure_stops_before_cleanup_spec_writes_and_archival() {
        let (root, plan) = fixture();
        fs::create_dir(root.path().join("specs/beta/log.md")).unwrap();

        let error = apply_plan(root.path(), &plan).unwrap_err();
        assert_eq!(error.issues[0].code, "ADOPTION_LOG_WRITE_FAILED");
        assert_eq!(error.issues[0].path.as_deref(), Some("specs/beta/log.md"));
        assert_eq!(
            fs::read_to_string(root.path().join("specs/alpha/log.md")).unwrap(),
            "baseline log"
        );
        for name in ["alpha", "beta"] {
            assert_eq!(
                fs::read_to_string(root.path().join(format!("specs/{name}/spec.yaml"))).unwrap(),
                "before"
            );
            assert!(root.path().join(format!("specs/{name}/brief.md")).is_file());
        }
        assert!(root.path().join("state/contract-review.md").is_file());
        assert!(root.path().join("steering/roadmap.md").is_file());
        assert!(
            root.path()
                .join("adoption/reverse-discovery.yaml")
                .is_file()
        );
        assert!(!root.path().join(&plan.review_archive).exists());
        assert!(!root.path().join(&plan.roadmap_archive).exists());
    }

    #[test]
    fn review_archive_failure_keeps_roadmap_and_adoption_record() {
        let (root, plan) = fixture();
        let review = root.path().join("state/contract-review.md");
        fs::remove_file(&review).unwrap();
        fs::create_dir(review).unwrap();

        let error = apply_plan(root.path(), &plan).unwrap_err();
        assert_eq!(error.issues[0].code, "ADOPTION_ARCHIVE_SOURCE_INVALID");
        assert_eq!(
            error.issues[0].path.as_deref(),
            Some("state/contract-review.md")
        );
        for name in ["alpha", "beta"] {
            assert_eq!(
                fs::read_to_string(root.path().join(format!("specs/{name}/spec.yaml"))).unwrap(),
                "after"
            );
            assert_eq!(
                fs::read_to_string(root.path().join(format!("specs/{name}/log.md"))).unwrap(),
                "baseline log"
            );
            assert!(!root.path().join(format!("specs/{name}/brief.md")).exists());
        }
        assert!(root.path().join("steering/roadmap.md").is_file());
        assert!(
            root.path()
                .join("adoption/reverse-discovery.yaml")
                .is_file()
        );
        assert!(!root.path().join(&plan.roadmap_archive).exists());
    }
}
