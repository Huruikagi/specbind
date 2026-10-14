//! Render the complete release-finalization plan before any mutation.

use std::path::Path;

use super::{
    FinalizeIssues, issue, log_failure, one_issue,
    persistence::{read_optional_log, read_regular},
};
use crate::{
    artifacts::{self, ArtifactKind},
    config::ProjectLanguage,
    domain::spec::Spec,
    release,
    release_log::{self, LogUpdate, ValidatedLogEntries},
    release_readiness::ReleaseReadiness,
    roadmap,
    schema::{runtime, spec::v1::SpecDocument},
};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct FinalizationPlan {
    pub(super) readiness: ReleaseReadiness,
    pub(super) specs: Vec<SpecPlan>,
    pub(super) roadmap_archive: String,
    pub(super) review_archive: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SpecPlan {
    pub(super) spec: String,
    pub(super) spec_yaml: String,
    pub(super) log_path: String,
    pub(super) log_update: LogUpdate,
    pub(super) cleanup: Vec<String>,
}

pub(super) fn build_plan(
    specbind_root: &Path,
    language: ProjectLanguage,
    date: time::Date,
    readiness: ReleaseReadiness,
    entries: &ValidatedLogEntries,
) -> Result<FinalizationPlan, FinalizeIssues> {
    let archives =
        release::archive_targets(&readiness.version).map_err(|error| FinalizeIssues {
            issues: error
                .issues
                .into_iter()
                .map(|value| issue(value.code, value.path, value.message))
                .collect(),
        })?;
    let roadmap_relative = format!("../../{}", archives.roadmap);
    let mut specs = Vec::new();
    for spec in &readiness.specs {
        let spec_path = format!("specs/{spec}/spec.yaml");
        let source = read_regular(specbind_root, &spec_path, "RELEASE_SPEC_READ_FAILED")?;
        let mut wire = runtime::load_spec(&source).map_err(|error| {
            one_issue(
                "RELEASE_SPEC_STRUCTURAL_INVALID",
                Some(spec_path.clone()),
                error.to_string(),
            )
        })?;
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
        let existing_log = read_optional_log(specbind_root, &log_path, language)?;
        let summary = entries.summary(spec).ok_or_else(|| {
            one_issue(
                "LOG_ENTRY_SET_MISMATCH",
                Some(spec.clone()),
                "participating Spec summary disappeared after validation",
            )
        })?;
        let log_update = release_log::update_log(
            &existing_log,
            language,
            date,
            &readiness.version,
            &readiness.milestone_id,
            &roadmap_relative,
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
        let mut cleanup = inventory
            .artifacts
            .into_iter()
            .filter(|artifact| {
                matches!(artifact.kind, ArtifactKind::Brief | ArtifactKind::Research)
            })
            .map(|artifact| artifact.path.to_string())
            .collect::<Vec<_>>();
        let tasks = format!("specs/{spec}/tasks.yaml");
        if specbind_root.join(&tasks).exists() {
            cleanup.push(tasks);
        }
        cleanup.sort();
        specs.push(SpecPlan {
            spec: spec.clone(),
            spec_yaml,
            log_path,
            log_update,
            cleanup,
        });
    }
    Ok(FinalizationPlan {
        review_archive: (!specs.is_empty()
            || roadmap::parse(&read_regular(
                specbind_root,
                "steering/roadmap.md",
                "RELEASE_ROADMAP_READ_FAILED",
            )?)
            .map_err(|error| one_issue("RELEASE_ROADMAP_INVALID", None, error.to_string()))?
            .has_shared_changes())
        .then_some(archives.cross_spec_review),
        roadmap_archive: archives.roadmap,
        readiness,
        specs,
    })
}

fn render_yaml(value: &SpecDocument, path: &str) -> Result<String, FinalizeIssues> {
    let mut rendered = serde_saphyr::to_string(value).map_err(|error| {
        one_issue(
            "RELEASE_SPEC_SERIALIZE_FAILED",
            Some(path.to_owned()),
            error.to_string(),
        )
    })?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}
