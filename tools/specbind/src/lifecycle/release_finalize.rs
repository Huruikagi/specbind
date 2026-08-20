//! Lifecycle service for guarded whole-milestone release finalization.

use std::{fmt, fs, path::Path};

use time::OffsetDateTime;

use crate::{
    artifacts::{self, ArtifactKind},
    config::ProjectLanguage,
    domain::spec::Spec,
    guarded_fs, release,
    release_log::{self, LogUpdate, ValidatedLogEntries},
    release_readiness::{self, ReleaseMutationTarget, ReleaseReadiness},
    repository, roadmap,
    schema::{
        runtime,
        spec::v1::{SpecDocument, WorkflowState},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeOutcome {
    Finalized { version: String, specs: usize },
    AlreadyFinalized { version: String, specs: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FinalizeIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizeIssues {
    pub issues: Vec<FinalizeIssue>,
}

#[derive(Debug, PartialEq, Eq)]
struct FinalizationPlan {
    readiness: ReleaseReadiness,
    specs: Vec<SpecPlan>,
    roadmap_archive: String,
    review_archive: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct SpecPlan {
    spec: String,
    spec_yaml: String,
    log_path: String,
    log_update: LogUpdate,
    cleanup: Vec<String>,
}

impl fmt::Display for FinalizeIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "release finalization has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for FinalizeIssues {}

/// Rechecks current readiness and applies the ordered release-finalization transition.
///
/// # Errors
///
/// Returns input, readiness, log, serialization, path, race, or filesystem diagnostics.
pub fn finalize(
    project_root: &Path,
    specbind_root: &Path,
    language: ProjectLanguage,
    log_entries_json: Option<&str>,
) -> Result<FinalizeOutcome, FinalizeIssues> {
    let initial = match release_readiness::resolve(project_root, specbind_root) {
        Ok(readiness) => readiness,
        Err(error) if error.code == "NO_ACTIVE_MILESTONE" => {
            if let Some(outcome) =
                completed_retry(project_root, specbind_root, language, log_entries_json)?
            {
                return Ok(outcome);
            }
            return Err(readiness_failure(error));
        }
        Err(error) => {
            if let Some(outcome) =
                resume_partial(project_root, specbind_root, language, log_entries_json)?
            {
                return Ok(outcome);
            }
            return Err(readiness_failure(error));
        }
    };
    let entries =
        release_log::validate_input(log_entries_json, &initial.specs).map_err(log_failure)?;
    let date = local_date()?;
    let plan = build_plan(specbind_root, language, date, initial, &entries)?;

    let current = release_readiness::resolve(project_root, specbind_root)
        .map_err(readiness_failure)
        .and_then(|readiness| build_plan(specbind_root, language, date, readiness, &entries))?;
    if current != plan {
        return Err(one_issue(
            "RELEASE_INPUTS_CHANGED",
            None,
            "release inputs changed during guarded finalization",
        ));
    }
    apply_plan(specbind_root, &plan)?;
    verify_final_state(specbind_root, &plan)?;
    Ok(FinalizeOutcome::Finalized {
        version: plan.readiness.version.clone(),
        specs: plan.specs.len(),
    })
}

fn resume_partial(
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

fn local_date() -> Result<time::Date, FinalizeIssues> {
    OffsetDateTime::now_local()
        .map(OffsetDateTime::date)
        .map_err(|error| {
            one_issue(
                "RELEASE_LOCAL_DATE_FAILED",
                None,
                format!("cannot resolve host local date: {error}"),
            )
        })
}

fn completed_retry(
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
) -> Result<(), FinalizeIssues> {
    let archives = release::archive_targets(version).map_err(|error| FinalizeIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    })?;
    if !specs.is_empty() {
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

fn build_plan(
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
        review_archive: (!specs.is_empty()).then_some(archives.cross_spec_review),
        roadmap_archive: archives.roadmap,
        readiness,
        specs,
    })
}

fn apply_plan(specbind_root: &Path, plan: &FinalizationPlan) -> Result<(), FinalizeIssues> {
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

fn verify_final_state(specbind_root: &Path, plan: &FinalizationPlan) -> Result<(), FinalizeIssues> {
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

fn read_regular(
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

fn readiness_failure(error: release_readiness::ReleaseReadinessFailure) -> FinalizeIssues {
    FinalizeIssues {
        issues: error
            .diagnostics
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    }
}

fn log_failure(error: Vec<release_log::LogIssue>) -> FinalizeIssues {
    FinalizeIssues {
        issues: error
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> FinalizeIssues {
    FinalizeIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> FinalizeIssue {
    FinalizeIssue {
        code,
        path,
        message: message.into(),
    }
}
