//! Read-only prerequisites for initial and resumed reverse establishment.

use std::{fs, path::Path};

use serde::Deserialize;

use crate::{
    adoption_finalize,
    artifacts::DiscoveryIssue,
    guarded_fs, milestone_scope as milestone_scope_model,
    milestone_status::{self, MilestoneDiagnostic, MilestoneStatusModel},
    repository, spec_list as spec_list_model, steering,
};

#[derive(Debug)]
pub(crate) struct Preflight {
    pub steering_count: usize,
    pub readiness: Readiness,
}

#[derive(Debug)]
pub(crate) enum Readiness {
    Initial { revision: String },
    Resume(Box<MilestoneStatusModel>),
}

#[derive(Debug)]
pub(crate) struct PreflightFailure {
    pub code: &'static str,
    pub message: &'static str,
    pub details: FailureDetails,
}

#[derive(Debug)]
pub(crate) enum FailureDetails {
    Messages(Vec<String>),
    Artifacts(Vec<DiscoveryIssue>),
    Milestone {
        diagnostics: Vec<MilestoneDiagnostic>,
        blockers: Vec<String>,
    },
    Source(Vec<adoption_finalize::FinalizeIssue>),
}

impl PreflightFailure {
    fn messages(code: &'static str, message: &'static str, details: Vec<String>) -> Self {
        Self::with_details(code, message, FailureDetails::Messages(details))
    }

    fn with_details(code: &'static str, message: &'static str, details: FailureDetails) -> Self {
        Self {
            code,
            message,
            details,
        }
    }
}

#[derive(Deserialize)]
struct AdoptionResumeRecord {
    source_revision: String,
}

/// Observes prerequisites in the established order without mutating the project.
/// The returned readiness is not persisted authority for any later transition.
pub(crate) fn resolve(
    project_root: &Path,
    specbind_root: &Path,
) -> Result<Preflight, PreflightFailure> {
    let steering_count = adoption_steering_count(specbind_root)?;
    if let Some(status) = reverse_resume_preflight(project_root, specbind_root)? {
        return Ok(Preflight {
            steering_count,
            readiness: Readiness::Resume(Box::new(status)),
        });
    }
    ensure_initial_adoption_scope(specbind_root)?;
    let revision = adoption_revision(project_root)?;
    Ok(Preflight {
        steering_count,
        readiness: Readiness::Initial { revision },
    })
}

fn reverse_resume_preflight(
    project_root: &Path,
    specbind_root: &Path,
) -> Result<Option<milestone_status::MilestoneStatusModel>, PreflightFailure> {
    let roadmap = milestone_status::read_roadmap(specbind_root).map_err(|error| {
        PreflightFailure::with_details(
            "ADOPTION_MILESTONE_INVALID",
            "Cannot inspect the active milestone for reverse establishment.",
            FailureDetails::Milestone {
                diagnostics: error.diagnostics,
                blockers: vec![],
            },
        )
    })?;
    let Some(roadmap) = roadmap else {
        return Ok(None);
    };
    if roadmap.reverse_specs.is_empty()
        || roadmap.baseline_version.is_none()
        || roadmap.target_release.is_some()
    {
        return Err(PreflightFailure::messages(
            "ADOPTION_MILESTONE_ACTIVE",
            "Existing-project adoption cannot start while a delivery milestone is active.",
            vec![],
        ));
    }

    let record = read_adoption_resume_record(specbind_root)?;
    if record.source_revision != roadmap.baseline_revision {
        return Err(PreflightFailure::messages(
            "ADOPTION_RESUME_RECORD_MISMATCH",
            "Temporary adoption evidence does not match the active reverse milestone.",
            vec!["adoption/reverse-discovery.yaml: source_revision does not match roadmap baseline_revision".to_owned()],
        ));
    }
    let status = resolve_reverse_resume_status(project_root, specbind_root)?;
    Ok(Some(status))
}

fn read_adoption_resume_record(
    specbind_root: &Path,
) -> Result<AdoptionResumeRecord, PreflightFailure> {
    let record_path = "adoption/reverse-discovery.yaml";
    let path = specbind_root.join(record_path);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        PreflightFailure::messages(
            "ADOPTION_RESUME_RECORD_REQUIRED",
            "Active reverse establishment requires its temporary adoption record.",
            vec![format!("{record_path}: {error}")],
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(PreflightFailure::messages(
            "ADOPTION_RESUME_RECORD_INVALID",
            "Cannot trust the temporary adoption record for reverse resumption.",
            vec![format!(
                "{record_path}: expected a regular non-symlink file"
            )],
        ));
    }
    let source = fs::read_to_string(&path).map_err(|error| {
        PreflightFailure::messages(
            "ADOPTION_RESUME_RECORD_INVALID",
            "Cannot read the temporary adoption record for reverse resumption.",
            vec![format!("{record_path}: {error}")],
        )
    })?;
    let record: AdoptionResumeRecord = serde_saphyr::from_str(&source).map_err(|error| {
        PreflightFailure::messages(
            "ADOPTION_RESUME_RECORD_INVALID",
            "Cannot parse the temporary adoption record for reverse resumption.",
            vec![format!("{record_path}: {error}")],
        )
    })?;
    Ok(record)
}

fn resolve_reverse_resume_status(
    project_root: &Path,
    specbind_root: &Path,
) -> Result<milestone_status::MilestoneStatusModel, PreflightFailure> {
    let _current_revision = adoption_revision(project_root)?;
    let status = milestone_status::resolve(project_root, specbind_root)
        .map_err(|error| {
            PreflightFailure::with_details(
                "ADOPTION_RESUME_STATE_INVALID",
                "Cannot derive a safe reverse-establishment resume point.",
                FailureDetails::Milestone {
                    diagnostics: error.diagnostics,
                    blockers: vec![],
                },
            )
        })?
        .ok_or_else(|| {
            PreflightFailure::messages(
                "ADOPTION_RESUME_STATE_INVALID",
                "The active reverse milestone disappeared during preflight.",
                vec![],
            )
        })?;
    if status.health != milestone_status::MilestoneHealth::Consistent
        || !status.current_blockers.is_empty()
    {
        let details = FailureDetails::Milestone {
            diagnostics: status.diagnostics,
            blockers: status.current_blockers,
        };
        return Err(PreflightFailure::with_details(
            "ADOPTION_RESUME_STATE_INVALID",
            "Active reverse establishment is not safe to resume.",
            details,
        ));
    }
    adoption_finalize::ensure_source_unchanged(
        project_root,
        specbind_root,
        &status.baseline_revision,
        &status
            .items
            .iter()
            .map(|item| item.id.clone())
            .collect::<Vec<_>>(),
    )
    .map_err(|error| {
        PreflightFailure::with_details(
            "ADOPTION_RESUME_SOURCE_STALE",
            "Implementation evidence changed after reverse establishment began.",
            FailureDetails::Source(error.issues),
        )
    })?;

    Ok(status)
}

fn adoption_steering_count(specbind_root: &Path) -> Result<usize, PreflightFailure> {
    let inventory = match steering::discover(specbind_root) {
        Ok(inventory) => inventory,
        Err(message) => {
            return Err(PreflightFailure::messages(
                "ADOPTION_STEERING_INVALID",
                "Cannot establish the Steering baseline for adoption.",
                vec![message],
            ));
        }
    };
    if !inventory.issues.is_empty() {
        return Err(PreflightFailure::with_details(
            "ADOPTION_STEERING_INVALID",
            "Cannot establish the Steering baseline for adoption.",
            FailureDetails::Artifacts(inventory.issues),
        ));
    }
    if inventory.documents.is_empty() {
        return Err(PreflightFailure::messages(
            "ADOPTION_STEERING_REQUIRED",
            "Existing-project adoption requires a non-empty Steering baseline.",
            vec![
                "Run sb-steering in bootstrap mode, review it, and commit it before retrying."
                    .to_owned(),
            ],
        ));
    }
    Ok(inventory.documents.len())
}

fn ensure_initial_adoption_scope(specbind_root: &Path) -> Result<(), PreflightFailure> {
    let record = "adoption/reverse-discovery.yaml";
    match fs::symlink_metadata(specbind_root.join(record)) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Ok(_) => {
            return Err(PreflightFailure::messages(
                "ADOPTION_RECORD_PRESENT",
                "Existing-project adoption cannot start from a pre-existing temporary record.",
                vec![format!(
                    "inspect and explicitly reconcile the unsupported record before retrying: {record}"
                )],
            ));
        }
        Err(error) => {
            return Err(PreflightFailure::messages(
                "ADOPTION_RECORD_UNREADABLE",
                "Cannot verify that no temporary adoption record exists.",
                vec![format!("{record}: {error}")],
            ));
        }
    }

    let specs = match spec_list_model::resolve(specbind_root) {
        Ok(specs) => specs,
        Err(error) => {
            return Err(PreflightFailure::messages(
                "ADOPTION_SPEC_LIST_FAILED",
                "Cannot verify the initial adoption scope.",
                vec![error.message],
            ));
        }
    };
    if !specs.is_empty() {
        return Err(PreflightFailure::messages(
            "ADOPTION_SPECS_PRESENT",
            "Initial existing-project adoption requires a project with no persistent Specs.",
            specs
                .iter()
                .map(|spec| format!("existing spec: {}", spec.canonical_spec))
                .collect(),
        ));
    }

    match milestone_scope_model::resolve(specbind_root, false) {
        Ok(None) => Ok(()),
        Ok(Some(_)) => Err(PreflightFailure::messages(
            "ADOPTION_MILESTONE_ACTIVE",
            "Existing-project adoption cannot start while a milestone is active.",
            vec![],
        )),
        Err(error) => Err(PreflightFailure::with_details(
            "ADOPTION_MILESTONE_INVALID",
            "Cannot verify that no milestone is active.",
            FailureDetails::Milestone {
                diagnostics: error.diagnostics,
                blockers: vec![],
            },
        )),
    }
}

fn adoption_revision(project_root: &Path) -> Result<String, PreflightFailure> {
    let status = match repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    ) {
        Ok(status) => status,
        Err(error) => {
            return Err(PreflightFailure::messages(
                "ADOPTION_GIT_FAILED",
                "Cannot inspect the repository for adoption.",
                vec![error.to_string()],
            ));
        }
    };
    if !status.is_empty() {
        return Err(PreflightFailure::messages(
            "ADOPTION_WORKTREE_DIRTY",
            "Existing-project adoption requires a clean committed repository.",
            vec![
                "Commit or otherwise reconcile the current work without having SpecBind move it."
                    .to_owned(),
            ],
        ));
    }

    match repository::output(project_root, &["rev-parse", "HEAD"]) {
        Ok(revision) if !revision.trim().is_empty() => Ok(revision.trim().to_owned()),
        Ok(_) => Err(PreflightFailure::messages(
            "ADOPTION_REVISION_INVALID",
            "Git returned an empty adoption source revision.",
            vec![],
        )),
        Err(error) => Err(PreflightFailure::messages(
            "ADOPTION_GIT_FAILED",
            "Cannot resolve the adoption source revision.",
            vec![error.to_string()],
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_steering_precedes_orphan_record_and_git_checks_without_mutation() {
        let project = tempfile::tempdir().unwrap();
        let root = project.path().join(".specbind");
        fs::create_dir_all(root.join("adoption")).unwrap();
        let record = root.join("adoption/reverse-discovery.yaml");
        fs::write(&record, "orphan record").unwrap();
        let error = resolve(project.path(), &root).unwrap_err();
        assert_eq!(error.code, "ADOPTION_STEERING_REQUIRED");
        assert!(matches!(error.details, FailureDetails::Messages(_)));
        assert_eq!(fs::read_to_string(record).unwrap(), "orphan record");
        assert!(!root.join("specs").exists());
        assert!(!project.path().join(".git").exists());
    }

    #[test]
    fn invalid_steering_returns_structured_artifact_diagnostics() {
        let project = tempfile::tempdir().unwrap();
        let root = project.path().join(".specbind");
        fs::create_dir_all(root.join("steering")).unwrap();
        let path = root.join("steering/project.md");
        let source = "---\ntype: SpecBind Steering\n---\n# Missing identity\n";
        fs::write(&path, source).unwrap();
        let expected = steering::discover(&root).unwrap().issues;
        assert!(
            !expected.is_empty(),
            "fixture must produce a discovery issue"
        );
        let error = resolve(project.path(), &root).unwrap_err();
        assert_eq!(error.code, "ADOPTION_STEERING_INVALID");
        let FailureDetails::Artifacts(issues) = error.details else {
            panic!("artifact diagnostics must remain structured");
        };
        assert_eq!(issues, expected);
        assert_eq!(fs::read_to_string(path).unwrap(), source);
    }
}
