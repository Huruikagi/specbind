//! Read model for release readiness and finalization-target safety.

use std::{collections::BTreeSet, fs, path::Path};

use crate::{
    artifacts::{self, ArtifactKind, DiscoveryIssue},
    cross_spec_review::{self, ReviewBoundary},
    freshness::FreshnessStatus,
    guarded_fs, release, release_log, repository, roadmap,
    roadmap::{DirectStatus, RoadmapDocument},
    schema::spec::v1::WorkflowState,
    spec_status::{self, ConsistencyHealth},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MutationTargetState {
    Existing,
    Absent,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseMutationTarget {
    pub path: String,
    pub state: MutationTargetState,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseDiagnostic {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseReadiness {
    pub milestone_id: String,
    pub version: String,
    pub specs: Vec<String>,
    pub direct_changes: usize,
    pub mutation_targets: Vec<ReleaseMutationTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseReadinessFailure {
    pub code: &'static str,
    pub diagnostics: Vec<ReleaseDiagnostic>,
}

/// Derives current release readiness without persisting evidence or authority.
///
/// # Errors
///
/// Returns active-Roadmap, lifecycle, review, archive, artifact, or target-path
/// diagnostics when project release work must not begin.
pub fn resolve(
    project_root: &Path,
    specbind_root: &Path,
) -> Result<ReleaseReadiness, ReleaseReadinessFailure> {
    let roadmap = read_roadmap(specbind_root)?;
    let mut diagnostics = BTreeSet::new();
    let mut targets = BTreeSet::new();
    targets.insert(existing("steering/roadmap.md"));

    let Some(version) = roadmap.target_release.clone() else {
        diagnostics.insert(diagnostic(
            "RELEASE_VERSION_UNBOUND",
            Some("steering/roadmap.md".to_owned()),
            "release preflight requires a concrete target_release",
        ));
        collect_scope_readiness(
            project_root,
            specbind_root,
            &roadmap,
            &mut targets,
            &mut diagnostics,
        );
        validate_targets(project_root, specbind_root, &targets, &mut diagnostics);
        return Err(blocked(diagnostics));
    };
    if !release::valid_version(&version) {
        return Err(ReleaseReadinessFailure {
            code: "INVALID_RELEASE_VERSION",
            diagnostics: vec![diagnostic(
                "INVALID_RELEASE_VERSION",
                Some("steering/roadmap.md".to_owned()),
                "target_release must match ^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$",
            )],
        });
    }

    collect_scope_readiness(
        project_root,
        specbind_root,
        &roadmap,
        &mut targets,
        &mut diagnostics,
    );
    match release::resolve_available_archive_targets(specbind_root, &version) {
        Ok(archive) => add_archive_targets(&roadmap, &archive, &mut targets),
        Err(error) => {
            diagnostics.extend(
                error
                    .issues
                    .into_iter()
                    .map(|value| diagnostic(value.code, value.path, value.message)),
            );
            if let Ok(archive) = release::archive_targets(&version) {
                add_archive_targets(&roadmap, &archive, &mut targets);
            }
        }
    }
    validate_targets(project_root, specbind_root, &targets, &mut diagnostics);

    if diagnostics.is_empty() {
        let specs = roadmap.spec_ids();
        Ok(ReleaseReadiness {
            milestone_id: roadmap.milestone_id,
            version,
            specs,
            direct_changes: roadmap.direct_changes.len(),
            mutation_targets: targets.into_iter().collect(),
        })
    } else {
        Err(blocked(diagnostics))
    }
}

fn read_roadmap(specbind_root: &Path) -> Result<RoadmapDocument, ReleaseReadinessFailure> {
    let relative = "steering/roadmap.md";
    let path = specbind_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        let (code, message) = if error.kind() == std::io::ErrorKind::NotFound {
            (
                "NO_ACTIVE_MILESTONE",
                "release preflight requires an active Roadmap".to_owned(),
            )
        } else {
            (
                "RELEASE_ROADMAP_READ_FAILED",
                format!("cannot inspect active Roadmap: {error}"),
            )
        };
        ReleaseReadinessFailure {
            code,
            diagnostics: vec![diagnostic(code, Some(relative.to_owned()), message)],
        }
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(ReleaseReadinessFailure {
            code: "RELEASE_PREFLIGHT_BLOCKED",
            diagnostics: vec![diagnostic(
                "RELEASE_ROADMAP_TARGET_INVALID",
                Some(relative.to_owned()),
                "active Roadmap must be a regular non-symlink file",
            )],
        });
    }
    let content = fs::read_to_string(path).map_err(|error| ReleaseReadinessFailure {
        code: "RELEASE_PREFLIGHT_BLOCKED",
        diagnostics: vec![diagnostic(
            "RELEASE_ROADMAP_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )],
    })?;
    roadmap::parse(&content).map_err(|error| {
        let invalid_version = error
            .issues
            .iter()
            .any(|issue| issue.code == "ROADMAP_TARGET_RELEASE_INVALID");
        ReleaseReadinessFailure {
            code: if invalid_version {
                "INVALID_RELEASE_VERSION"
            } else {
                "RELEASE_PREFLIGHT_BLOCKED"
            },
            diagnostics: error
                .issues
                .into_iter()
                .map(|value| diagnostic(value.code, Some(relative.to_owned()), value.message))
                .collect(),
        }
    })
}

fn collect_scope_readiness(
    project_root: &Path,
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    targets: &mut BTreeSet<ReleaseMutationTarget>,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    for item in &roadmap.direct_changes {
        if item.status != Some(DirectStatus::Completed) {
            diagnostics.insert(diagnostic(
                "RELEASE_DIRECT_INCOMPLETE",
                Some("steering/roadmap.md".to_owned()),
                format!("Direct item {} is not completed", item.id),
            ));
        }
    }
    if let Err(error) = cross_spec_review::require_for_boundary(
        project_root,
        specbind_root,
        ReviewBoundary::ReleasePreflight,
    ) {
        diagnostics.extend(
            error
                .issues
                .into_iter()
                .map(|value| diagnostic(value.code, value.source, value.message)),
        );
    }

    let specs = roadmap.spec_ids();
    for spec in &specs {
        collect_spec_readiness(
            project_root,
            specbind_root,
            roadmap,
            spec,
            targets,
            diagnostics,
        );
    }
    if !specs.is_empty() {
        targets.insert(existing("state/contract-review.md"));
    }
    diagnose_unscoped_active_specs(specbind_root, roadmap, diagnostics);
}

fn collect_spec_readiness(
    project_root: &Path,
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    spec: &str,
    targets: &mut BTreeSet<ReleaseMutationTarget>,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    let spec_yaml = format!("specs/{spec}/spec.yaml");
    let tasks_yaml = format!("specs/{spec}/tasks.yaml");
    targets.insert(existing(&spec_yaml));
    let log_path = format!("specs/{spec}/log.md");
    targets.insert(target_for_optional_file(specbind_root, &log_path));
    validate_existing_log(specbind_root, &log_path, diagnostics);
    targets.insert(existing(&tasks_yaml));

    match spec_status::resolve(project_root, specbind_root, spec) {
        Ok(model) => {
            diagnostics.extend(
                model
                    .diagnostics
                    .into_iter()
                    .map(|value| diagnostic(value.code, value.path, value.message)),
            );
            if model.health != ConsistencyHealth::Consistent {
                diagnostics.insert(diagnostic(
                    "RELEASE_SPEC_INCONSISTENT",
                    Some(spec_yaml.clone()),
                    format!("Spec {spec} is inconsistent"),
                ));
            }
            if model.milestone_id.as_deref() != Some(&roadmap.milestone_id) {
                diagnostics.insert(diagnostic(
                    "RELEASE_SPEC_MILESTONE_MISMATCH",
                    Some(spec_yaml.clone()),
                    format!("Spec {spec} does not belong to the active milestone"),
                ));
            }
            if model.declared_state != Some(WorkflowState::ReleaseReady) {
                diagnostics.insert(diagnostic(
                    "RELEASE_SPEC_NOT_VALIDATED",
                    Some(spec_yaml.clone()),
                    format!("Spec {spec} must be release_ready"),
                ));
            }
            for (name, gate) in [
                ("requirements", &model.freshness.requirements),
                ("design", &model.freshness.design),
                ("tasks", &model.freshness.tasks),
                ("completion", &model.freshness.completion),
            ] {
                if gate.status != FreshnessStatus::Fresh {
                    diagnostics.insert(diagnostic(
                        "RELEASE_SPEC_GATE_NOT_FRESH",
                        Some(spec_yaml.clone()),
                        format!("Spec {spec} {name} gate is not fresh"),
                    ));
                }
            }
            if !model.task_model.as_ref().is_some_and(|tasks| {
                tasks.completed == tasks.total() && tasks.pending == 0 && tasks.blocked == 0
            }) {
                diagnostics.insert(diagnostic(
                    "RELEASE_SPEC_TASKS_INCOMPLETE",
                    Some(tasks_yaml),
                    format!("Spec {spec} tasks must be complete and unblocked"),
                ));
            }
        }
        Err(error) => diagnostics.extend(error.issues.into_iter().map(from_discovery)),
    }

    let inventory = artifacts::discover_spec(specbind_root, spec);
    diagnostics.extend(inventory.issues.into_iter().map(from_discovery));
    let mut brief = None;
    for artifact in inventory.artifacts {
        match artifact.kind {
            ArtifactKind::Brief => brief = Some(artifact.path.as_str().to_owned()),
            ArtifactKind::Research => {
                targets.insert(existing(artifact.path.as_str()));
            }
            ArtifactKind::Requirements
            | ArtifactKind::Design
            | ArtifactKind::Contract
            | ArtifactKind::ImplementationNotes => {}
        }
    }
    if let Some(brief) = brief {
        targets.insert(existing(brief));
    } else {
        diagnostics.insert(diagnostic(
            "RELEASE_SPEC_BRIEF_MISSING",
            Some(format!("specs/{spec}")),
            format!("Spec {spec} requires one active Brief"),
        ));
    }
}

fn diagnose_unscoped_active_specs(
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    let scoped = roadmap.spec_ids().into_iter().collect::<BTreeSet<_>>();
    let Ok(entries) = fs::read_dir(specbind_root.join("specs")) else {
        return;
    };
    for entry in entries.flatten() {
        if !entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            continue;
        }
        let Some(spec) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if scoped.contains(&spec) {
            continue;
        }
        let resolution = artifacts::resolve_spec(specbind_root, &spec);
        if resolution
            .wire
            .as_ref()
            .is_some_and(|wire| wire.active_change.0.is_some())
        {
            diagnostics.insert(diagnostic(
                "RELEASE_UNSCOPED_ACTIVE_SPEC",
                Some(format!("specs/{spec}/spec.yaml")),
                format!("active spec {spec} is absent from Roadmap scope"),
            ));
        }
    }
}

fn add_archive_targets(
    roadmap: &RoadmapDocument,
    archive: &release::ArchiveTargets,
    targets: &mut BTreeSet<ReleaseMutationTarget>,
) {
    targets.insert(absent(&archive.roadmap));
    if !roadmap.spec_ids().is_empty() {
        targets.insert(absent(&archive.cross_spec_review));
    }
}

fn validate_targets(
    project_root: &Path,
    specbind_root: &Path,
    targets: &BTreeSet<ReleaseMutationTarget>,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    let root_relative = match specbind_root.strip_prefix(project_root) {
        Ok(path) => path,
        Err(error) => {
            diagnostics.insert(diagnostic(
                "RELEASE_PROJECT_ROOT_INVALID",
                None,
                format!("SpecBind root must be below the Git project root: {error}"),
            ));
            return;
        }
    };
    for target in targets {
        validate_target_parents(specbind_root, target, diagnostics);
        let project_relative = root_relative
            .join(&target.path)
            .to_string_lossy()
            .replace('\\', "/");
        match target.state {
            MutationTargetState::Existing => validate_existing_target(
                project_root,
                specbind_root,
                target,
                &project_relative,
                diagnostics,
            ),
            MutationTargetState::Absent => validate_absent_target(
                project_root,
                specbind_root,
                target,
                &project_relative,
                diagnostics,
            ),
        }
    }
}

fn validate_target_parents(
    specbind_root: &Path,
    target: &ReleaseMutationTarget,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    let Some(parent) = Path::new(&target.path).parent() else {
        return;
    };
    let mut current = specbind_root.to_path_buf();
    for component in parent.components() {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() => {
                diagnostics.insert(diagnostic(
                    "RELEASE_TARGET_PARENT_INVALID",
                    Some(target.path.clone()),
                    "mutation target parent must be a regular non-symlink directory",
                ));
                return;
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => {
                diagnostics.insert(diagnostic(
                    "RELEASE_TARGET_PARENT_UNAVAILABLE",
                    Some(target.path.clone()),
                    error.to_string(),
                ));
                return;
            }
        }
    }
}

fn validate_existing_target(
    project_root: &Path,
    specbind_root: &Path,
    target: &ReleaseMutationTarget,
    project_relative: &str,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    let native = specbind_root.join(&target.path);
    match fs::symlink_metadata(&native) {
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_file() => {
            diagnostics.insert(diagnostic(
                "RELEASE_TARGET_INVALID",
                Some(target.path.clone()),
                "required finalization source must be a regular non-symlink file",
            ));
            return;
        }
        Ok(_) => {}
        Err(error) => {
            diagnostics.insert(diagnostic(
                "RELEASE_TARGET_MISSING",
                Some(target.path.clone()),
                format!("required finalization source is unavailable: {error}"),
            ));
            return;
        }
    }
    match repository::predicate(
        project_root,
        &["ls-files", "--error-unmatch", "--", project_relative],
    ) {
        Ok(true) => {}
        Ok(false) => {
            diagnostics.insert(diagnostic(
                "RELEASE_TARGET_NOT_TRACKED",
                Some(target.path.clone()),
                "required finalization source must be tracked by Git",
            ));
        }
        Err(error) => {
            diagnostics.insert(diagnostic(
                "RELEASE_TARGET_GIT_FAILED",
                Some(target.path.clone()),
                error.to_string(),
            ));
        }
    }
    match repository::output_bytes(
        project_root,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--",
            project_relative,
        ],
    ) {
        Ok(status) if status.is_empty() => {}
        Ok(_) => {
            diagnostics.insert(diagnostic(
                "RELEASE_TARGET_DIRTY",
                Some(target.path.clone()),
                "finalization target has staged or unstaged changes",
            ));
        }
        Err(error) => {
            diagnostics.insert(diagnostic(
                "RELEASE_TARGET_GIT_FAILED",
                Some(target.path.clone()),
                error.to_string(),
            ));
        }
    }
}

fn validate_absent_target(
    project_root: &Path,
    specbind_root: &Path,
    target: &ReleaseMutationTarget,
    project_relative: &str,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    match fs::symlink_metadata(specbind_root.join(&target.path)) {
        Ok(_) => {
            diagnostics.insert(diagnostic(
                "RELEASE_DESTINATION_OCCUPIED",
                Some(target.path.clone()),
                "release archive destination must not already exist",
            ));
            return;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            diagnostics.insert(diagnostic(
                "RELEASE_DESTINATION_UNAVAILABLE",
                Some(target.path.clone()),
                error.to_string(),
            ));
            return;
        }
    }
    match repository::predicate(
        project_root,
        &["check-ignore", "-q", "--no-index", "--", project_relative],
    ) {
        Ok(false) => {}
        Ok(true) => {
            diagnostics.insert(diagnostic(
                "RELEASE_DESTINATION_IGNORED",
                Some(target.path.clone()),
                "release archive destination must remain visible to Git",
            ));
        }
        Err(error) => {
            diagnostics.insert(diagnostic(
                "RELEASE_TARGET_GIT_FAILED",
                Some(target.path.clone()),
                error.to_string(),
            ));
        }
    }
}

fn existing(path: impl Into<String>) -> ReleaseMutationTarget {
    ReleaseMutationTarget {
        path: path.into(),
        state: MutationTargetState::Existing,
    }
}

fn absent(path: impl Into<String>) -> ReleaseMutationTarget {
    ReleaseMutationTarget {
        path: path.into(),
        state: MutationTargetState::Absent,
    }
}

fn target_for_optional_file(
    specbind_root: &Path,
    path: impl Into<String>,
) -> ReleaseMutationTarget {
    let path = path.into();
    if fs::symlink_metadata(specbind_root.join(&path)).is_ok() {
        existing(path)
    } else {
        absent(path)
    }
}

fn validate_existing_log(
    specbind_root: &Path,
    path: &str,
    diagnostics: &mut BTreeSet<ReleaseDiagnostic>,
) {
    let native = specbind_root.join(path);
    let Ok(metadata) = fs::symlink_metadata(&native) else {
        return;
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return;
    }
    match fs::read_to_string(native) {
        Ok(input) => {
            if let Err(issues) = release_log::validate_existing(&input, path) {
                diagnostics.extend(
                    issues
                        .into_iter()
                        .map(|value| diagnostic(value.code, value.path, value.message)),
                );
            }
        }
        Err(error) => {
            diagnostics.insert(diagnostic(
                "RELEASE_LOG_READ_FAILED",
                Some(path.to_owned()),
                error.to_string(),
            ));
        }
    }
}

fn from_discovery(value: DiscoveryIssue) -> ReleaseDiagnostic {
    diagnostic(
        value.code,
        value.path.map(|path| path.to_string()),
        value.message,
    )
}

fn blocked(diagnostics: BTreeSet<ReleaseDiagnostic>) -> ReleaseReadinessFailure {
    ReleaseReadinessFailure {
        code: "RELEASE_PREFLIGHT_BLOCKED",
        diagnostics: diagnostics.into_iter().collect(),
    }
}

fn diagnostic(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> ReleaseDiagnostic {
    ReleaseDiagnostic {
        code,
        path,
        message: message.into(),
    }
}
