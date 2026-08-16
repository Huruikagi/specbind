//! Guarded Spec and Direct completion handshakes.

use std::{
    fmt, fs,
    io::Write as _,
    path::{Component, Path},
    process::Command,
};

use serde::Deserialize;
use tempfile::NamedTempFile;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    artifacts,
    cross_spec_review::{self, ReviewBoundary},
    domain::spec::Spec,
    freshness::{self, FreshnessStatus},
    milestone_status::{self, MilestoneHealth},
    roadmap::{self, Dependency, DirectCompletionEdit, DirectStatus, RoadmapDocument},
    schema::spec::v1::{
        CompletionGateEvidence, ImplementationRevision, MechanicalCheck, MechanicalCheckKind,
        NonEmptyString, PassedAt, SuccessfulExitCode, WorkflowState,
    },
    spec_status::{self, ConsistencyHealth},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompletionIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionIssues {
    pub issues: Vec<CompletionIssue>,
}

impl fmt::Display for CompletionIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "completion operation has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for CompletionIssues {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecPreflightOutcome {
    Ready { implementation_revision: String },
    AlreadyAccepted { implementation_revision: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecAcceptOutcome {
    Accepted { implementation_revision: String },
    AlreadyAccepted { implementation_revision: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecInvalidateOutcome {
    Invalidated,
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectPreflightOutcome {
    Ready { implementation_revision: String },
    AlreadyCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectCompleteOutcome {
    Recorded,
    AlreadyCompleted,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CompletionCandidate {
    schema_version: u64,
    implementation_revision: String,
    mechanical_checks: Vec<CandidateMechanicalCheck>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CandidateMechanicalCheck {
    kind: MechanicalCheckKind,
    command: String,
    exit_code: u8,
    #[serde(default)]
    working_directory: Option<String>,
}

struct ValidatedCandidate {
    implementation_revision: String,
    mechanical_checks: Vec<MechanicalCheck>,
}

struct SpecGuard {
    source: String,
    wire: crate::schema::spec::v1::SpecDocument,
}

struct RoadmapGuard {
    source: String,
    roadmap: RoadmapDocument,
}

/// Begins validation for one explicit Spec at the converged milestone revision.
///
/// # Errors
///
/// Returns deterministic lifecycle, milestone, review, task, or Git diagnostics.
pub fn spec_preflight(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
) -> Result<SpecPreflightOutcome, CompletionIssues> {
    if let Some(revision) = accepted_revision(project_root, specbind_root, canonical_spec, None)? {
        return Ok(SpecPreflightOutcome::AlreadyAccepted {
            implementation_revision: revision,
        });
    }
    let revision = clean_head(project_root)?;
    spec_guard(
        project_root,
        specbind_root,
        canonical_spec,
        &revision,
        false,
    )?;
    Ok(SpecPreflightOutcome::Ready {
        implementation_revision: revision,
    })
}

/// Accepts one strict completion candidate and performs `IMPLEMENTATION_VALIDATED`.
///
/// # Errors
///
/// Returns candidate, lifecycle, freshness, race, Git, serialization, or write
/// diagnostics without partially changing `spec.yaml`.
pub fn spec_accept(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    candidate_json: &str,
) -> Result<SpecAcceptOutcome, CompletionIssues> {
    let candidate = validate_candidate(candidate_json)?;
    if let Some(revision) = accepted_revision(
        project_root,
        specbind_root,
        canonical_spec,
        Some(&candidate),
    )? {
        return Ok(SpecAcceptOutcome::AlreadyAccepted {
            implementation_revision: revision,
        });
    }
    let initial = spec_guard(
        project_root,
        specbind_root,
        canonical_spec,
        &candidate.implementation_revision,
        true,
    )?;
    let current = spec_guard(
        project_root,
        specbind_root,
        canonical_spec,
        &candidate.implementation_revision,
        true,
    )?;
    if initial.source != current.source || initial.wire != current.wire {
        return Err(one_issue(
            "COMPLETION_INPUTS_CHANGED",
            Some(spec_path(canonical_spec)),
            "spec completion inputs changed during guarded acceptance",
        ));
    }

    let mut wire = current.wire;
    let active = wire.active_change.0.as_mut().ok_or_else(|| {
        one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion acceptance requires an active implementation change",
        )
    })?;
    let passed_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| {
            one_issue(
                "COMPLETION_TIMESTAMP_FAILED",
                Some(spec_path(canonical_spec)),
                error.to_string(),
            )
        })?;
    active.state = WorkflowState::ReleaseReady;
    let evidence = active.gate_evidence.as_mut().ok_or_else(|| {
        one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "implementation state requires cumulative gate evidence",
        )
    })?;
    evidence.completion = Some(CompletionGateEvidence {
        passed_at: PassedAt(passed_at),
        implementation_revision: ImplementationRevision(candidate.implementation_revision.clone()),
        mechanical_checks: candidate.mechanical_checks,
    });
    validate_mutated_spec(&wire, canonical_spec)?;
    let rendered = render_yaml(&wire, "COMPLETION_SPEC_SERIALIZE_FAILED")?;
    persist_regular(
        &specbind_root.join(spec_path(canonical_spec)),
        rendered.as_bytes(),
        "COMPLETION_SPEC_WRITE_FAILED",
        &spec_path(canonical_spec),
    )?;
    Ok(SpecAcceptOutcome::Accepted {
        implementation_revision: candidate.implementation_revision,
    })
}

/// Explicitly clears stale completion evidence and rewinds to implementation.
///
/// # Errors
///
/// Returns prior-gate, target-path, serialization, or filesystem diagnostics.
pub fn spec_invalidate(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
) -> Result<SpecInvalidateOutcome, CompletionIssues> {
    if !valid_id(canonical_spec) {
        return Err(one_issue(
            "SPEC_COMPLETION_TARGET_INVALID",
            Some(format!("specs/{canonical_spec}")),
            "completion invalidation requires a canonical Spec ID",
        ));
    }
    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(mut wire) = resolution.wire else {
        return Err(discovery_failure(resolution.issues));
    };
    let Some(active) = wire.active_change.0.as_ref() else {
        return Err(one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion invalidation requires an active change",
        ));
    };
    let completion_present = active
        .gate_evidence
        .as_ref()
        .and_then(|evidence| evidence.completion.as_ref())
        .is_some();
    if active.state == WorkflowState::Implementation && !completion_present {
        return Ok(SpecInvalidateOutcome::NoChange);
    }
    if active.state != WorkflowState::ReleaseReady {
        return Err(one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion invalidation requires release_ready or an already-clean implementation state",
        ));
    }

    let gate_inputs = artifacts::resolve_gate_inputs(specbind_root, canonical_spec);
    let freshness = freshness::evaluate_wire(&wire, &gate_inputs.inputs);
    let mut issues = gate_inputs
        .inventory
        .issues
        .iter()
        .cloned()
        .map(from_discovery)
        .collect::<Vec<_>>();
    for (name, gate) in [
        ("Requirements", &freshness.requirements),
        ("Design", &freshness.design),
        ("Tasks", &freshness.tasks),
    ] {
        if gate.status != FreshnessStatus::Fresh {
            issues.push(issue(
                "SPEC_COMPLETION_EARLIER_GATE_STALE",
                Some(spec_path(canonical_spec)),
                format!("{name} must remain fresh before completion-only invalidation"),
            ));
        }
    }
    ensure_target_clean(
        project_root,
        specbind_root,
        &spec_path(canonical_spec),
        &mut issues,
    );
    finish_issues(issues)?;

    let Some(active) = wire.active_change.0.as_mut() else {
        return Err(one_issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion invalidation lost its active change",
        ));
    };
    active.state = WorkflowState::Implementation;
    if let Some(evidence) = active.gate_evidence.as_mut() {
        evidence.completion = None;
    }
    validate_mutated_spec(&wire, canonical_spec)?;
    let rendered = render_yaml(&wire, "COMPLETION_SPEC_SERIALIZE_FAILED")?;
    persist_regular(
        &specbind_root.join(spec_path(canonical_spec)),
        rendered.as_bytes(),
        "COMPLETION_SPEC_WRITE_FAILED",
        &spec_path(canonical_spec),
    )?;
    Ok(SpecInvalidateOutcome::Invalidated)
}

/// Begins the clean-revision handshake for one pending Direct Roadmap item.
///
/// # Errors
///
/// Returns Roadmap identity, dependency, or Git diagnostics.
pub fn direct_preflight(
    project_root: &Path,
    specbind_root: &Path,
    canonical_direct: &str,
) -> Result<DirectPreflightOutcome, CompletionIssues> {
    if direct_completed(specbind_root, canonical_direct)? {
        return Ok(DirectPreflightOutcome::AlreadyCompleted);
    }
    let revision = clean_head(project_root)?;
    direct_guard(project_root, specbind_root, canonical_direct, &revision)?;
    Ok(DirectPreflightOutcome::Ready {
        implementation_revision: revision,
    })
}

/// Records one Direct item complete after independently rechecking its revision.
///
/// # Errors
///
/// Returns Roadmap, dependency, revision, race, serialization, or write diagnostics.
pub fn direct_complete(
    project_root: &Path,
    specbind_root: &Path,
    canonical_direct: &str,
    implementation_revision: &str,
) -> Result<DirectCompleteOutcome, CompletionIssues> {
    if direct_completed(specbind_root, canonical_direct)? {
        return Ok(DirectCompleteOutcome::AlreadyCompleted);
    }
    validate_revision(implementation_revision)?;
    let initial = direct_guard(
        project_root,
        specbind_root,
        canonical_direct,
        implementation_revision,
    )?;
    let current = direct_guard(
        project_root,
        specbind_root,
        canonical_direct,
        implementation_revision,
    )?;
    if initial.source != current.source || initial.roadmap != current.roadmap {
        return Err(one_issue(
            "DIRECT_COMPLETION_INPUTS_CHANGED",
            Some("steering/roadmap.md".to_owned()),
            "Roadmap changed during guarded Direct completion",
        ));
    }
    match roadmap::complete_direct(&current.source, canonical_direct).map_err(roadmap_failure)? {
        DirectCompletionEdit::NoChange => Ok(DirectCompleteOutcome::AlreadyCompleted),
        DirectCompletionEdit::Updated(rendered) => {
            persist_regular(
                &specbind_root.join("steering/roadmap.md"),
                rendered.as_bytes(),
                "DIRECT_COMPLETION_WRITE_FAILED",
                "steering/roadmap.md",
            )?;
            Ok(DirectCompleteOutcome::Recorded)
        }
    }
}

fn direct_completed(
    specbind_root: &Path,
    canonical_direct: &str,
) -> Result<bool, CompletionIssues> {
    if !valid_id(canonical_direct) {
        return Err(one_issue(
            "DIRECT_COMPLETION_TARGET_INVALID",
            Some(format!("direct:{canonical_direct}")),
            "Direct completion requires a canonical Direct ID",
        ));
    }
    let guard = read_roadmap(specbind_root)?;
    guard
        .roadmap
        .direct_changes
        .iter()
        .find(|item| item.id == canonical_direct)
        .map(|item| item.status == Some(DirectStatus::Completed))
        .ok_or_else(|| {
            one_issue(
                "DIRECT_COMPLETION_NOT_FOUND",
                Some("steering/roadmap.md".to_owned()),
                format!("Direct item {canonical_direct} is not in the active Roadmap"),
            )
        })
}

fn validate_candidate(candidate_json: &str) -> Result<ValidatedCandidate, CompletionIssues> {
    let candidate =
        serde_json::from_str::<CompletionCandidate>(candidate_json).map_err(|error| {
            one_issue(
                "COMPLETION_EVIDENCE_INVALID",
                None,
                format!("completion evidence is not strict version-1 JSON: {error}"),
            )
        })?;
    let mut issues = Vec::new();
    if candidate.schema_version != 1 {
        issues.push(issue(
            "COMPLETION_EVIDENCE_INVALID",
            Some("/schemaVersion".to_owned()),
            "schemaVersion must be 1",
        ));
    }
    if let Err(error) = validate_revision(&candidate.implementation_revision) {
        issues.extend(error.issues);
    }
    if candidate.mechanical_checks.is_empty() {
        issues.push(issue(
            "COMPLETION_EVIDENCE_INVALID",
            Some("/mechanicalChecks".to_owned()),
            "mechanicalChecks must contain at least one successful command",
        ));
    }
    let mechanical_checks = candidate
        .mechanical_checks
        .into_iter()
        .enumerate()
        .filter_map(|(index, check)| {
            let path = format!("/mechanicalChecks/{index}");
            let command_valid =
                !check.command.trim().is_empty() && !check.command.chars().any(char::is_control);
            if !command_valid {
                issues.push(issue(
                    "COMPLETION_EVIDENCE_INVALID",
                    Some(format!("{path}/command")),
                    "command must be a non-empty display-safe single line",
                ));
            }
            if check.exit_code != 0 {
                issues.push(issue(
                    "COMPLETION_EVIDENCE_INVALID",
                    Some(format!("{path}/exitCode")),
                    "exitCode must be 0",
                ));
            }
            if let Some(directory) = check.working_directory.as_deref()
                && !valid_portable_relative(directory)
            {
                issues.push(issue(
                    "COMPLETION_EVIDENCE_INVALID",
                    Some(format!("{path}/workingDirectory")),
                    "workingDirectory must be a portable project-root-relative path",
                ));
            }
            (command_valid
                && check.exit_code == 0
                && check
                    .working_directory
                    .as_deref()
                    .is_none_or(valid_portable_relative))
            .then(|| MechanicalCheck {
                kind: check.kind,
                command: NonEmptyString(check.command),
                exit_code: SuccessfulExitCode(0),
                working_directory: check.working_directory.map(NonEmptyString),
            })
        })
        .collect::<Vec<_>>();
    finish_issues(issues)?;
    Ok(ValidatedCandidate {
        implementation_revision: candidate.implementation_revision,
        mechanical_checks,
    })
}

fn accepted_revision(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    candidate: Option<&ValidatedCandidate>,
) -> Result<Option<String>, CompletionIssues> {
    if !valid_id(canonical_spec) {
        return Err(one_issue(
            "SPEC_COMPLETION_TARGET_INVALID",
            Some(format!("specs/{canonical_spec}")),
            "completion requires a canonical Spec ID",
        ));
    }
    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(wire) = resolution.wire else {
        return Err(discovery_failure(resolution.issues));
    };
    let Some(active) = wire.active_change.0.as_ref() else {
        return Ok(None);
    };
    if active.state != WorkflowState::ReleaseReady {
        return Ok(None);
    }
    let Some(completion) = active
        .gate_evidence
        .as_ref()
        .and_then(|evidence| evidence.completion.as_ref())
    else {
        return Ok(None);
    };
    let model = spec_status::resolve(project_root, specbind_root, canonical_spec)
        .map_err(|error| discovery_failure(error.issues))?;
    if model.freshness.completion.status != FreshnessStatus::Fresh {
        return Ok(None);
    }
    if let Some(candidate) = candidate
        && (completion.implementation_revision.0 != candidate.implementation_revision
            || completion.mechanical_checks != candidate.mechanical_checks)
    {
        return Ok(None);
    }
    Ok(Some(completion.implementation_revision.0.clone()))
}

fn spec_guard(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    implementation_revision: &str,
    allow_pending_metadata: bool,
) -> Result<SpecGuard, CompletionIssues> {
    validate_revision(implementation_revision)?;
    let mut issues = Vec::new();
    validate_completion_checkout(
        project_root,
        specbind_root,
        implementation_revision,
        allow_pending_metadata,
        &mut issues,
    )?;
    let roadmap_guard = read_roadmap(specbind_root)?;
    validate_completion_milestone(
        project_root,
        specbind_root,
        canonical_spec,
        &roadmap_guard.roadmap,
        &mut issues,
    )?;
    validate_participating_specs(
        project_root,
        specbind_root,
        &roadmap_guard.roadmap,
        &mut issues,
    );
    validate_completion_review(project_root, specbind_root, canonical_spec, &mut issues);

    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(wire) = resolution.wire else {
        issues.extend(resolution.issues.into_iter().map(from_discovery));
        return finish_issues(issues).map(|()| unreachable!());
    };
    if wire
        .active_change
        .0
        .as_ref()
        .is_none_or(|active| active.state != WorkflowState::Implementation)
    {
        issues.push(issue(
            "SPEC_COMPLETION_STATE_INVALID",
            Some(spec_path(canonical_spec)),
            "completion acceptance requires the target Spec in implementation",
        ));
    }
    let source = read_regular(
        &specbind_root.join(spec_path(canonical_spec)),
        &spec_path(canonical_spec),
    )?;
    finish_issues(issues)?;
    Ok(SpecGuard { source, wire })
}

fn validate_completion_checkout(
    project_root: &Path,
    specbind_root: &Path,
    implementation_revision: &str,
    allow_pending_metadata: bool,
    issues: &mut Vec<CompletionIssue>,
) -> Result<(), CompletionIssues> {
    if head_revision(project_root)? != implementation_revision {
        issues.push(issue(
            "COMPLETION_REVISION_CHANGED",
            None,
            "current HEAD does not match the completion implementation revision",
        ));
    }
    if allow_pending_metadata {
        issues.extend(
            freshness::assess_pending_completion_mutations(
                project_root,
                specbind_root,
                implementation_revision,
            )
            .issues
            .into_iter()
            .map(|value| issue(value.code, Some(value.path), value.message)),
        );
    } else if !worktree_status(project_root)?.is_empty() {
        issues.push(issue(
            "COMPLETION_WORKTREE_DIRTY",
            None,
            "completion preflight requires a clean worktree",
        ));
    }
    Ok(())
}

fn validate_completion_milestone(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    roadmap: &RoadmapDocument,
    issues: &mut Vec<CompletionIssue>,
) -> Result<(), CompletionIssues> {
    if !roadmap.spec_ids().iter().any(|spec| spec == canonical_spec) {
        issues.push(issue(
            "SPEC_COMPLETION_NOT_IN_MILESTONE",
            Some(spec_path(canonical_spec)),
            "completion target must participate in the active Roadmap",
        ));
    }
    let milestone = milestone_status::resolve(project_root, specbind_root).map_err(|error| {
        CompletionIssues {
            issues: error
                .diagnostics
                .into_iter()
                .map(|value| issue(value.code, value.path, value.message))
                .collect(),
        }
    })?;
    match milestone {
        Some(model) if model.milestone_id == roadmap.milestone_id => {
            if model.health != MilestoneHealth::Consistent {
                issues.extend(
                    model
                        .diagnostics
                        .into_iter()
                        .map(|value| issue(value.code, value.path, value.message)),
                );
            }
        }
        Some(_) => issues.push(issue(
            "COMPLETION_MILESTONE_CHANGED",
            Some("steering/roadmap.md".to_owned()),
            "resolved milestone identity changed during completion",
        )),
        None => issues.push(issue(
            "COMPLETION_MILESTONE_MISSING",
            Some("steering/roadmap.md".to_owned()),
            "completion requires an active Roadmap",
        )),
    }
    if roadmap
        .direct_changes
        .iter()
        .any(|item| item.status != Some(DirectStatus::Completed))
    {
        issues.push(issue(
            "COMPLETION_MILESTONE_NOT_CONVERGED",
            Some("steering/roadmap.md".to_owned()),
            "every Direct item must be completed before Spec validation",
        ));
    }
    Ok(())
}

fn validate_participating_specs(
    project_root: &Path,
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    issues: &mut Vec<CompletionIssue>,
) {
    for spec in roadmap.spec_ids() {
        match spec_status::resolve(project_root, specbind_root, &spec) {
            Ok(model) => {
                if model.milestone_id.as_deref() != Some(&roadmap.milestone_id) {
                    issues.push(issue(
                        "COMPLETION_SPEC_MILESTONE_MISMATCH",
                        Some(spec_path(&spec)),
                        "participating Spec milestone does not match the Roadmap",
                    ));
                }
                if model.health != ConsistencyHealth::Consistent {
                    issues.extend(
                        model
                            .diagnostics
                            .into_iter()
                            .map(|value| issue(value.code, value.path, value.message)),
                    );
                }
                if !matches!(
                    model.declared_state,
                    Some(WorkflowState::Implementation | WorkflowState::ReleaseReady)
                ) {
                    issues.push(issue(
                        "COMPLETION_SPEC_STATE_INVALID",
                        Some(spec_path(&spec)),
                        "every participating Spec must be in implementation or release_ready",
                    ));
                }
                if [
                    &model.freshness.requirements,
                    &model.freshness.design,
                    &model.freshness.tasks,
                ]
                .iter()
                .any(|gate| gate.status != FreshnessStatus::Fresh)
                {
                    issues.push(issue(
                        "COMPLETION_SPEC_GATE_STALE",
                        Some(spec_path(&spec)),
                        "every participating Spec requires fresh prior gates",
                    ));
                }
                if !model.task_model.as_ref().is_some_and(|tasks| {
                    tasks.completed == tasks.total() && tasks.pending == 0 && tasks.blocked == 0
                }) {
                    issues.push(issue(
                        "COMPLETION_MILESTONE_NOT_CONVERGED",
                        Some(format!("specs/{spec}/tasks.yaml")),
                        "every participating Spec task plan must be complete and unblocked",
                    ));
                }
            }
            Err(error) => issues.extend(error.issues.into_iter().map(from_discovery)),
        }
    }
}

fn validate_completion_review(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    issues: &mut Vec<CompletionIssue>,
) {
    if let Err(error) = cross_spec_review::require_for_boundary(
        project_root,
        specbind_root,
        ReviewBoundary::ImplementationValidation { canonical_spec },
    ) {
        issues.extend(
            error
                .issues
                .into_iter()
                .map(|value| issue(value.code, value.source, value.message)),
        );
    }
}

fn direct_guard(
    project_root: &Path,
    specbind_root: &Path,
    canonical_direct: &str,
    implementation_revision: &str,
) -> Result<RoadmapGuard, CompletionIssues> {
    if !valid_id(canonical_direct) {
        return Err(one_issue(
            "DIRECT_COMPLETION_TARGET_INVALID",
            Some(format!("direct:{canonical_direct}")),
            "Direct completion requires a canonical Direct ID",
        ));
    }
    validate_revision(implementation_revision)?;
    let current_revision = clean_head(project_root)?;
    let guard = read_roadmap(specbind_root)?;
    let Some(item) = guard
        .roadmap
        .direct_changes
        .iter()
        .find(|item| item.id == canonical_direct)
    else {
        return Err(one_issue(
            "DIRECT_COMPLETION_NOT_FOUND",
            Some("steering/roadmap.md".to_owned()),
            format!("Direct item {canonical_direct} is not in the active Roadmap"),
        ));
    };
    if item.status == Some(DirectStatus::Completed) {
        return Ok(guard);
    }
    let mut issues = Vec::new();
    if current_revision != implementation_revision {
        issues.push(issue(
            "DIRECT_COMPLETION_REVISION_CHANGED",
            None,
            "current HEAD does not match the Direct implementation revision",
        ));
    }
    for dependency in &item.depends_on {
        match dependency {
            Dependency::Direct(value) => {
                let complete = guard
                    .roadmap
                    .direct_changes
                    .iter()
                    .find(|item| item.id == value.direct)
                    .is_some_and(|item| item.status == Some(DirectStatus::Completed));
                if !complete {
                    issues.push(issue(
                        "DIRECT_COMPLETION_DEPENDENCY_PENDING",
                        Some("steering/roadmap.md".to_owned()),
                        format!("dependency direct:{} is not completed", value.direct),
                    ));
                }
            }
            Dependency::Spec(value) => {
                let complete = spec_status::resolve(project_root, specbind_root, &value.spec)
                    .is_ok_and(|model| {
                        model.health == ConsistencyHealth::Consistent
                            && model.milestone_id.as_deref() == Some(&guard.roadmap.milestone_id)
                            && model.task_model.as_ref().is_some_and(|tasks| {
                                tasks.completed == tasks.total()
                                    && tasks.pending == 0
                                    && tasks.blocked == 0
                            })
                    });
                if !complete {
                    issues.push(issue(
                        "DIRECT_COMPLETION_DEPENDENCY_PENDING",
                        Some("steering/roadmap.md".to_owned()),
                        format!(
                            "dependency spec:{} is not implementation-complete",
                            value.spec
                        ),
                    ));
                }
            }
        }
    }
    finish_issues(issues)?;
    Ok(guard)
}

fn read_roadmap(specbind_root: &Path) -> Result<RoadmapGuard, CompletionIssues> {
    let relative = "steering/roadmap.md";
    let source = read_regular(&specbind_root.join(relative), relative)?;
    let roadmap = roadmap::parse(&source).map_err(roadmap_failure)?;
    Ok(RoadmapGuard { source, roadmap })
}

fn clean_head(project_root: &Path) -> Result<String, CompletionIssues> {
    let revision = head_revision(project_root)?;
    if !worktree_status(project_root)?.is_empty() {
        return Err(one_issue(
            "COMPLETION_WORKTREE_DIRTY",
            None,
            "completion preflight requires a clean worktree",
        ));
    }
    Ok(revision)
}

fn head_revision(project_root: &Path) -> Result<String, CompletionIssues> {
    let revision = git_output(project_root, &["rev-parse", "HEAD"])?;
    let revision = revision.trim().to_owned();
    validate_revision(&revision)?;
    Ok(revision)
}

fn worktree_status(project_root: &Path) -> Result<Vec<u8>, CompletionIssues> {
    git_output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
}

fn ensure_target_clean(
    project_root: &Path,
    specbind_root: &Path,
    spec_relative: &str,
    issues: &mut Vec<CompletionIssue>,
) {
    let Ok(root_relative) = specbind_root.strip_prefix(project_root) else {
        issues.push(issue(
            "COMPLETION_PROJECT_ROOT_INVALID",
            Some(spec_relative.to_owned()),
            "SpecBind root must be below the Git project root",
        ));
        return;
    };
    let path = root_relative
        .join(spec_relative)
        .to_string_lossy()
        .replace('\\', "/");
    match git_output_bytes(
        project_root,
        &[
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--",
            &path,
        ],
    ) {
        Ok(output) if output.is_empty() => {}
        Ok(_) => issues.push(issue(
            "SPEC_COMPLETION_TARGET_DIRTY",
            Some(spec_relative.to_owned()),
            "completion invalidation refuses to overwrite a dirty or staged spec.yaml",
        )),
        Err(error) => issues.extend(error.issues),
    }
}

fn validate_mutated_spec(
    wire: &crate::schema::spec::v1::SpecDocument,
    canonical_spec: &str,
) -> Result<(), CompletionIssues> {
    Spec::try_from(wire.clone())
        .map(|_| ())
        .map_err(|error| CompletionIssues {
            issues: error
                .issues
                .into_iter()
                .map(|value| issue(value.code, Some(spec_path(canonical_spec)), value.message))
                .collect(),
        })
}

fn render_yaml<T: serde::Serialize>(
    value: &T,
    code: &'static str,
) -> Result<String, CompletionIssues> {
    let mut rendered =
        serde_saphyr::to_string(value).map_err(|error| one_issue(code, None, error.to_string()))?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}

fn persist_regular(
    target: &Path,
    bytes: &[u8],
    code: &'static str,
    relative: &str,
) -> Result<(), CompletionIssues> {
    let metadata = fs::symlink_metadata(target).map_err(|error| {
        one_issue(
            code,
            Some(relative.to_owned()),
            format!("cannot inspect target: {error}"),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(one_issue(
            code,
            Some(relative.to_owned()),
            "mutation target must be a regular non-symlink file",
        ));
    }
    let parent = target.parent().ok_or_else(|| {
        one_issue(
            code,
            Some(relative.to_owned()),
            "mutation target has no parent directory",
        )
    })?;
    let mut temporary = NamedTempFile::new_in(parent)
        .map_err(|error| one_issue(code, Some(relative.to_owned()), error.to_string()))?;
    temporary
        .write_all(bytes)
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(|error| one_issue(code, Some(relative.to_owned()), error.to_string()))?;
    temporary
        .persist(target)
        .map_err(|error| one_issue(code, Some(relative.to_owned()), error.error.to_string()))?;
    Ok(())
}

fn read_regular(path: &Path, relative: &str) -> Result<String, CompletionIssues> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        one_issue(
            "COMPLETION_TARGET_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(one_issue(
            "COMPLETION_TARGET_INVALID",
            Some(relative.to_owned()),
            "completion target must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(path).map_err(|error| {
        one_issue(
            "COMPLETION_TARGET_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

fn validate_revision(value: &str) -> Result<(), CompletionIssues> {
    let valid = matches!(value.len(), 40 | 64)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if valid {
        Ok(())
    } else {
        Err(one_issue(
            "COMPLETION_EVIDENCE_INVALID",
            Some("/implementationRevision".to_owned()),
            "implementationRevision must be a full lowercase Git object ID",
        ))
    }
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn valid_portable_relative(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !value.contains('\\')
        && !path.is_absolute()
        && value
            .split('/')
            .all(|segment| !matches!(segment, "" | "." | "..") && valid_portable_segment(segment))
        && path.components().all(|component| match component {
            Component::Normal(segment) => segment.to_str().is_some_and(valid_portable_segment),
            _ => false,
        })
}

fn valid_portable_segment(value: &str) -> bool {
    let invalid_shape = value.is_empty()
        || value.ends_with([' ', '.'])
        || value
            .chars()
            .any(|character| character.is_control() || r#"<>:"\|?*"#.contains(character));
    if invalid_shape {
        return false;
    }
    let stem = value
        .split('.')
        .next()
        .unwrap_or(value)
        .to_ascii_uppercase();
    !matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        && !stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"))
            .is_some_and(|suffix| {
                matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
            })
}

fn git_output(project_root: &Path, arguments: &[&str]) -> Result<String, CompletionIssues> {
    let output = git_output_bytes(project_root, arguments)?;
    String::from_utf8(output).map_err(|error| {
        one_issue(
            "COMPLETION_GIT_FAILED",
            None,
            format!("Git output is not UTF-8: {error}"),
        )
    })
}

fn git_output_bytes(project_root: &Path, arguments: &[&str]) -> Result<Vec<u8>, CompletionIssues> {
    let output = Command::new("git")
        .arg("-C")
        .arg(project_root)
        .args(arguments)
        .output()
        .map_err(|error| {
            one_issue(
                "COMPLETION_GIT_FAILED",
                None,
                format!("cannot start Git: {error}"),
            )
        })?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(one_issue(
            "COMPLETION_GIT_FAILED",
            None,
            String::from_utf8_lossy(&output.stderr).trim(),
        ))
    }
}

fn spec_path(canonical_spec: &str) -> String {
    format!("specs/{canonical_spec}/spec.yaml")
}

fn discovery_failure(issues: Vec<artifacts::DiscoveryIssue>) -> CompletionIssues {
    CompletionIssues {
        issues: issues.into_iter().map(from_discovery).collect(),
    }
}

fn from_discovery(value: artifacts::DiscoveryIssue) -> CompletionIssue {
    issue(
        value.code,
        value.path.map(|path| path.to_string()),
        value.message,
    )
}

fn roadmap_failure(error: roadmap::RoadmapIssues) -> CompletionIssues {
    CompletionIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| {
                issue(
                    value.code,
                    Some("steering/roadmap.md".to_owned()),
                    value.message,
                )
            })
            .collect(),
    }
}

fn finish_issues(mut issues: Vec<CompletionIssue>) -> Result<(), CompletionIssues> {
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(CompletionIssues { issues })
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> CompletionIssues {
    CompletionIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> CompletionIssue {
    CompletionIssue {
        code,
        path,
        message: message.into(),
    }
}
