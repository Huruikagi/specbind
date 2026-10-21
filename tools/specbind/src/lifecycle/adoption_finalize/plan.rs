//! Build every reverse-establishment output before any mutation.

use std::{fs, path::Path};

use time::OffsetDateTime;

use super::{FinalizeIssues, issue, one_issue, read_regular};
use crate::{
    artifacts::{self, ArtifactKind},
    config::ProjectLanguage,
    domain::spec::Spec,
    guarded_fs,
    milestone_status::MilestoneStatusModel,
    release_log::{self, LogEntryKind, LogUpdate},
    schema::{runtime, spec::v1::SpecDocument},
};

pub(super) struct FinalizationPlan {
    pub(super) version: String,
    pub(super) specs: Vec<SpecPlan>,
    pub(super) roadmap_archive: String,
    pub(super) review_archive: String,
}

pub(super) struct SpecPlan {
    pub(super) spec: String,
    pub(super) spec_yaml: String,
    pub(super) log_path: String,
    pub(super) log_update: LogUpdate,
    pub(super) cleanup: Vec<String>,
}

#[allow(
    clippy::too_many_lines,
    reason = "all reverse Spec outputs are planned before persistence"
)]
pub(super) fn build_plan(
    specbind_root: &Path,
    language: ProjectLanguage,
    log_entries_json: Option<&str>,
    status: &MilestoneStatusModel,
) -> Result<FinalizationPlan, FinalizeIssues> {
    let version = status.baseline_version.clone().ok_or_else(|| {
        one_issue(
            "ADOPTION_REVERSE_REQUIRED",
            Some("steering/roadmap.md".to_owned()),
            "reverse milestone lost its baseline version",
        )
    })?;
    let specs = status
        .items
        .iter()
        .filter_map(|item| item.id.strip_prefix("spec:").map(str::to_owned))
        .collect::<Vec<_>>();
    let entries = release_log::validate_input(log_entries_json, &specs).map_err(log_failure)?;
    let date = OffsetDateTime::now_local()
        .map(OffsetDateTime::date)
        .map_err(|error| one_issue("ADOPTION_LOCAL_DATE_FAILED", None, error.to_string()))?;
    let roadmap_archive = format!("baselines/{version}-roadmap.md");
    let review_archive = format!("baselines/{version}-contract-review.md");
    ensure_archive_targets(specbind_root, &roadmap_archive, &review_archive)?;

    let mut plans = Vec::new();
    for spec in &specs {
        let spec_path = format!("specs/{spec}/spec.yaml");
        let source = read_regular(specbind_root, &spec_path)?;
        let mut wire = runtime::load_spec(&source).map_err(|error| {
            one_issue(
                "ADOPTION_SPEC_INVALID",
                Some(spec_path.clone()),
                error.to_string(),
            )
        })?;
        let establishment = wire.establishment.as_ref().ok_or_else(|| {
            one_issue(
                "ADOPTION_PROVENANCE_MISSING",
                Some(spec_path.clone()),
                "reverse Spec must retain establishment provenance",
            )
        })?;
        if establishment.milestone_id.0 != status.milestone_id
            || establishment.source_revision.0 != status.baseline_revision
            || establishment.baseline_version.0 != version
        {
            return Err(one_issue(
                "ADOPTION_PROVENANCE_MISMATCH",
                Some(spec_path),
                "Spec establishment provenance does not match the active reverse milestone",
            ));
        }
        wire.active_change.0 = None;
        Spec::try_from(wire.clone()).map_err(|error| FinalizeIssues {
            issues: error
                .issues
                .into_iter()
                .map(|value| issue(value.code, Some(spec_path.clone()), value.message))
                .collect(),
        })?;
        let spec_yaml = render_yaml(&wire, &spec_path)?;
        let log_path = format!("specs/{spec}/log.md");
        let existing = read_optional_log(specbind_root, &log_path, language)?;
        let summary = entries.summary(spec).ok_or_else(|| {
            one_issue(
                "LOG_ENTRY_SET_MISMATCH",
                Some(spec.clone()),
                "validated reverse Spec summary disappeared before finalization",
            )
        })?;
        let log_update = release_log::update_log_entry(
            &existing,
            language,
            date,
            LogEntryKind::Baseline,
            &version,
            &status.milestone_id,
            &format!("../../{roadmap_archive}"),
            summary,
            &log_path,
        )
        .map_err(log_failure)?;
        let inventory = artifacts::discover_spec(specbind_root, spec);
        if !inventory.issues.is_empty() {
            return Err(FinalizeIssues {
                issues: inventory
                    .issues
                    .into_iter()
                    .map(|value| {
                        issue(
                            value.code,
                            value.path.map(|path| path.to_string()),
                            value.message,
                        )
                    })
                    .collect(),
            });
        }
        if specbind_root
            .join(format!("specs/{spec}/tasks.yaml"))
            .exists()
        {
            return Err(one_issue(
                "ADOPTION_TASKS_FORBIDDEN",
                Some(format!("specs/{spec}/tasks.yaml")),
                "reverse establishment must not produce Tasks",
            ));
        }
        let cleanup = inventory
            .artifacts
            .into_iter()
            .filter(|artifact| {
                matches!(artifact.kind, ArtifactKind::Brief | ArtifactKind::Research)
            })
            .map(|artifact| artifact.path.to_string())
            .collect();
        plans.push(SpecPlan {
            spec: spec.clone(),
            spec_yaml,
            log_path,
            log_update,
            cleanup,
        });
    }

    Ok(FinalizationPlan {
        version,
        specs: plans,
        roadmap_archive,
        review_archive,
    })
}

fn ensure_archive_targets(
    specbind_root: &Path,
    roadmap: &str,
    review: &str,
) -> Result<(), FinalizeIssues> {
    for relative in [roadmap, review] {
        if specbind_root.join(relative).exists() {
            return Err(one_issue(
                "ADOPTION_ARCHIVE_OCCUPIED",
                Some(relative.to_owned()),
                "baseline archive destination must be absent",
            ));
        }
    }
    Ok(())
}

fn read_optional_log(
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
        Ok(_) => read_regular(specbind_root, relative),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(release_log::empty_log(language))
        }
        Err(error) => Err(one_issue(
            "ADOPTION_LOG_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )),
    }
}

fn render_yaml(value: &SpecDocument, path: &str) -> Result<String, FinalizeIssues> {
    let mut rendered = serde_saphyr::to_string(value).map_err(|error| {
        one_issue(
            "ADOPTION_SPEC_SERIALIZE_FAILED",
            Some(path.to_owned()),
            error.to_string(),
        )
    })?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}

fn log_failure(errors: Vec<release_log::LogIssue>) -> FinalizeIssues {
    FinalizeIssues {
        issues: errors
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    }
}
