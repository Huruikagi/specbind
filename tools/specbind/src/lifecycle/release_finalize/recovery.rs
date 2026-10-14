//! Recognize interrupted finalization and verify already-completed retries.

use std::{fs, path::Path};

use super::{
    FinalizeIssues, FinalizeOutcome, issue, local_date, log_failure, one_issue,
    persistence::{apply_plan, read_regular, verify_final_state},
    plan::{FinalizationPlan, build_plan},
};
use crate::{
    artifacts::{self, ArtifactKind},
    config::ProjectLanguage,
    release,
    release_log::{self, LogUpdate, ValidatedLogEntries},
    release_readiness::{ReleaseMutationTarget, ReleaseReadiness},
    repository, roadmap,
    schema::{runtime, spec::v1::WorkflowState},
};

pub(super) fn resume_partial(
    project_root: &Path,
    specbind_root: &Path,
    language: ProjectLanguage,
    input: Option<&str>,
) -> Result<Option<FinalizeOutcome>, FinalizeIssues> {
    if !specbind_root.join("steering/roadmap.md").exists() {
        return Ok(None);
    }
    let source = read_regular(
        specbind_root,
        "steering/roadmap.md",
        "RELEASE_ROADMAP_READ_FAILED",
    )?;
    let Ok(roadmap) = roadmap::parse(&source) else {
        return Ok(None);
    };
    let Some(version) = roadmap.target_release.clone() else {
        return Ok(None);
    };
    let specs = roadmap.spec_ids();
    let entries = release_log::validate_input(input, &specs).map_err(log_failure)?;
    let readiness = ReleaseReadiness {
        milestone_id: roadmap.milestone_id.clone(),
        version,
        specs,
        direct_changes: roadmap.direct_changes.len(),
        mutation_targets: Vec::<ReleaseMutationTarget>::new(),
    };
    let plan = build_plan(specbind_root, language, local_date()?, readiness, &entries)?;
    if !validate_partial_plan(project_root, specbind_root, &roadmap, &plan)? {
        return Ok(None);
    }
    apply_plan(specbind_root, &plan)?;
    verify_final_state(specbind_root, &plan)?;
    Ok(Some(FinalizeOutcome::Finalized {
        version: plan.readiness.version.clone(),
        specs: plan.specs.len(),
    }))
}

fn validate_partial_plan(
    project_root: &Path,
    specbind_root: &Path,
    roadmap: &roadmap::RoadmapDocument,
    plan: &FinalizationPlan,
) -> Result<bool, FinalizeIssues> {
    if roadmap
        .direct_changes
        .iter()
        .any(|item| item.status != Some(roadmap::DirectStatus::Completed))
        || git_path_dirty(project_root, specbind_root, "steering/roadmap.md")?
        || specbind_root.join(&plan.roadmap_archive).exists()
    {
        return Ok(false);
    }
    let mut partial_marker = false;
    for spec in &plan.specs {
        let spec_path = format!("specs/{}/spec.yaml", spec.spec);
        let source = read_regular(specbind_root, &spec_path, "RELEASE_FINAL_STATE_INVALID")?;
        let wire = runtime::load_spec(&source).map_err(|error| {
            one_issue(
                "RELEASE_FINAL_STATE_INVALID",
                Some(spec_path.clone()),
                error.to_string(),
            )
        })?;
        match wire.active_change.0.as_ref() {
            Some(active)
                if active.state == WorkflowState::ReleaseReady
                    && active.milestone_id.0 == plan.readiness.milestone_id =>
            {
                if git_path_dirty(project_root, specbind_root, &spec_path)? {
                    return Ok(false);
                }
            }
            None if source == spec.spec_yaml => {
                partial_marker |= git_path_dirty(project_root, specbind_root, &spec_path)?;
            }
            _ => return Ok(false),
        }
        match &spec.log_update {
            LogUpdate::Updated(_) => {
                if specbind_root.join(&spec.log_path).exists()
                    && git_path_dirty(project_root, specbind_root, &spec.log_path)?
                {
                    return Ok(false);
                }
            }
            LogUpdate::Unchanged => {
                partial_marker |= git_path_dirty(project_root, specbind_root, &spec.log_path)?;
            }
        }
        for relative in &spec.cleanup {
            if specbind_root.join(relative).exists()
                && git_path_dirty(project_root, specbind_root, relative)?
            {
                return Ok(false);
            }
        }
    }
    if let Some(review_archive) = &plan.review_archive {
        let active = specbind_root.join("state/contract-review.md").exists();
        let archived = specbind_root.join(review_archive).exists();
        match (active, archived) {
            (true, false) => {
                if git_path_dirty(project_root, specbind_root, "state/contract-review.md")? {
                    return Ok(false);
                }
            }
            (false, true) => {
                let review =
                    read_regular(specbind_root, review_archive, "RELEASE_FINAL_STATE_INVALID")?;
                if !review.contains(&format!("milestone_id: {}", plan.readiness.milestone_id)) {
                    return Ok(false);
                }
                partial_marker = true;
            }
            _ => return Ok(false),
        }
    }
    Ok(partial_marker)
}

pub(super) fn completed_retry(
    project_root: &Path,
    specbind_root: &Path,
    language: ProjectLanguage,
    input: Option<&str>,
) -> Result<Option<FinalizeOutcome>, FinalizeIssues> {
    let releases = specbind_root.join("releases");
    let entries = match fs::read_dir(&releases) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(one_issue(
                "RELEASE_ARCHIVE_ROOT_UNAVAILABLE",
                Some("releases".to_owned()),
                error.to_string(),
            ));
        }
    };
    let mut candidates = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| {
            one_issue(
                "RELEASE_ARCHIVE_ROOT_UNAVAILABLE",
                Some("releases".to_owned()),
                error.to_string(),
            )
        })?;
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Some(version) = name.strip_suffix("-roadmap.md") else {
            continue;
        };
        if !release::valid_version(version) {
            continue;
        }
        let relative = format!("releases/{name}");
        if !git_path_dirty(project_root, specbind_root, &relative)? {
            continue;
        }
        let source = read_regular(specbind_root, &relative, "RELEASE_ARCHIVE_READ_FAILED")?;
        let Ok(roadmap) = roadmap::parse(&source) else {
            continue;
        };
        if roadmap.target_release.as_deref() == Some(version) {
            candidates.push((version.to_owned(), roadmap));
        }
    }
    if candidates.is_empty() {
        return Ok(None);
    }
    if candidates.len() > 1 {
        return Err(one_issue(
            "RELEASE_RETRY_AMBIGUOUS",
            Some("releases".to_owned()),
            "multiple uncommitted Roadmap archives could represent the finalize retry",
        ));
    }
    let (version, roadmap) = candidates.pop().expect("one candidate remains");
    let specs = roadmap.spec_ids();
    let log_entries = release_log::validate_input(input, &specs).map_err(log_failure)?;
    verify_completed_retry(
        specbind_root,
        language,
        &version,
        &roadmap.milestone_id,
        &specs,
        &log_entries,
        roadmap.has_shared_changes(),
    )?;
    Ok(Some(FinalizeOutcome::AlreadyFinalized {
        version,
        specs: specs.len(),
    }))
}

fn verify_completed_retry(
    specbind_root: &Path,
    language: ProjectLanguage,
    version: &str,
    milestone_id: &str,
    specs: &[String],
    entries: &ValidatedLogEntries,
    shared_changes: bool,
) -> Result<(), FinalizeIssues> {
    let archives = release::archive_targets(version).map_err(|error| FinalizeIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    })?;
    if !specs.is_empty() || shared_changes {
        let review = read_regular(
            specbind_root,
            &archives.cross_spec_review,
            "RELEASE_FINAL_STATE_INVALID",
        )?;
        if !review.contains(&format!("milestone_id: {milestone_id}"))
            || specbind_root.join("state/contract-review.md").exists()
        {
            return Err(one_issue(
                "RELEASE_FINAL_STATE_INVALID",
                Some(archives.cross_spec_review),
                "finalized contract review does not match the archived milestone",
            ));
        }
    }
    for spec in specs {
        let spec_path = format!("specs/{spec}/spec.yaml");
        let source = read_regular(specbind_root, &spec_path, "RELEASE_FINAL_STATE_INVALID")?;
        let wire = runtime::load_spec(&source).map_err(|error| {
            one_issue(
                "RELEASE_FINAL_STATE_INVALID",
                Some(spec_path.clone()),
                error.to_string(),
            )
        })?;
        if wire.active_change.0.is_some() {
            return Err(one_issue(
                "RELEASE_FINAL_STATE_INVALID",
                Some(spec_path),
                "finalized participating Spec must be idle",
            ));
        }
        let inventory = artifacts::discover_spec(specbind_root, spec);
        if inventory
            .artifacts
            .iter()
            .any(|artifact| matches!(artifact.kind, ArtifactKind::Brief | ArtifactKind::Research))
            || specbind_root
                .join(format!("specs/{spec}/tasks.yaml"))
                .exists()
        {
            return Err(one_issue(
                "RELEASE_FINAL_STATE_INVALID",
                Some(format!("specs/{spec}")),
                "finalized participating Spec retains milestone-local artifacts",
            ));
        }
        let log_path = format!("specs/{spec}/log.md");
        let log = read_regular(specbind_root, &log_path, "RELEASE_FINAL_STATE_INVALID")?;
        let summary = entries.summary(spec).ok_or_else(|| {
            one_issue(
                "LOG_ENTRY_SET_MISMATCH",
                Some(spec.clone()),
                "finalized participating Spec has no requested summary",
            )
        })?;
        release_log::verify_entry(
            &log,
            language,
            version,
            milestone_id,
            &format!("../../{}", archives.roadmap),
            summary,
            &log_path,
        )
        .map_err(log_failure)?;
    }
    Ok(())
}

fn git_path_dirty(
    project_root: &Path,
    specbind_root: &Path,
    relative: &str,
) -> Result<bool, FinalizeIssues> {
    let root_relative = specbind_root.strip_prefix(project_root).map_err(|error| {
        one_issue(
            "RELEASE_PROJECT_ROOT_INVALID",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    let path = root_relative
        .join(relative)
        .to_string_lossy()
        .replace('\\', "/");
    repository::output_bytes(
        project_root,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--",
            &path,
        ],
    )
    .map(|output| !output.is_empty())
    .map_err(|error| {
        one_issue(
            "RELEASE_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}
