//! Read model for one Spec in project context.

use std::{collections::BTreeSet, path::Path};

use crate::{
    artifacts::{self, ArtifactKind, DiscoveryIssue},
    cross_spec_review::{self, ReviewFreshnessStatus},
    freshness::{self, ArtifactFreshnessReport, FreshnessStatus},
    schema::spec::{self, v1::WorkflowState},
    task_read_model::TaskReadModel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsistencyHealth {
    Consistent,
    Inconsistent,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatusDiagnostic {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementCoverage {
    pub active: usize,
    pub design: usize,
    pub tasks: usize,
    pub tasks_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskBlocker {
    pub task_id: String,
    pub reason: String,
}

/// One gate crossed under Decision 0012 delegated authority, with the workflow
/// that carried it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegatedGate {
    pub gate: &'static str,
    pub workflow: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowAction {
    None,
    Requirements,
    Design,
    ContractReview,
    Tasks,
    Implementation,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpectedDesignWork {
    pub missing_coverage: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecStatusModel {
    pub declared_state: Option<WorkflowState>,
    pub milestone_id: Option<String>,
    pub health: ConsistencyHealth,
    pub freshness: ArtifactFreshnessReport,
    /// Milestone-owned contract review, reported only where it is a prerequisite
    /// of something this Spec still needs. It never affects `health`, because
    /// Decision 0078 keeps the review out of the per-Spec invariant.
    pub contract_review: Option<ReviewFreshnessStatus>,
    pub next_action: WorkflowAction,
    pub expected_design_work: Option<ExpectedDesignWork>,
    /// Present once any gate is approved. Empty means every approval was
    /// explicit, which is why absence of the field and an empty list mean
    /// different things.
    pub delegated_gates: Option<Vec<DelegatedGate>>,
    pub task_model: Option<TaskReadModel>,
    pub blockers: Vec<TaskBlocker>,
    pub coverage: Option<RequirementCoverage>,
    pub diagnostics: Vec<StatusDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusFailure {
    pub issues: Vec<DiscoveryIssue>,
}

/// Resolves lifecycle, freshness, traceability, and task progress for one Spec.
///
/// # Errors
///
/// Returns a fatal failure only when no structurally valid `spec.yaml` wire model
/// can be read. Semantic contradictions remain reportable in the returned model.
pub fn resolve(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
) -> Result<SpecStatusModel, StatusFailure> {
    let spec_resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(wire) = spec_resolution.wire else {
        return Err(StatusFailure {
            issues: spec_resolution.issues,
        });
    };
    let active = wire.active_change.0.as_ref();
    let declared_state = active.map(|active| active.state);
    let milestone_id = active.map(|active| active.milestone_id.0.clone());
    let mut gate_resolution = artifacts::resolve_gate_inputs(specbind_root, canonical_spec);
    if active
        .and_then(|active| active.gate_evidence.as_ref())
        .and_then(|evidence| evidence.completion.as_ref())
        .is_some()
        && let Some(spec) = spec_resolution.spec.as_ref()
    {
        gate_resolution.inputs.completion_revision = Some(freshness::assess_completion_revision(
            project_root,
            specbind_root,
            canonical_spec,
            spec,
        ));
    }
    let freshness = freshness::evaluate_wire(&wire, &gate_resolution.inputs);
    let traceability = active
        .is_some()
        .then(|| artifacts::resolve_traceability(specbind_root, canonical_spec));
    let task_model = gate_resolution
        .inputs
        .tasks
        .as_ref()
        .map(TaskReadModel::derive);
    let blockers = task_model.as_ref().map_or_else(Vec::new, task_blockers);
    let coverage = traceability
        .as_ref()
        .and_then(|resolution| resolution.report.as_ref())
        .and_then(coverage);

    let expected_design_work =
        expected_design_work(declared_state, &freshness, traceability.as_ref());
    let diagnostics = collect_diagnostics(
        &spec_resolution.issues,
        &gate_resolution,
        traceability.as_ref(),
        &freshness,
        active.is_none(),
        expected_design_work.is_some(),
        canonical_spec,
    );
    let health = if diagnostics.is_empty() {
        ConsistencyHealth::Consistent
    } else {
        ConsistencyHealth::Inconsistent
    };
    let contract_review = contract_review(project_root, specbind_root, declared_state);
    let next_action = next_action(declared_state, &freshness, contract_review);
    let delegated_gates = active
        .and_then(|active| active.gate_evidence.as_ref())
        .map(delegated_gates);

    Ok(SpecStatusModel {
        declared_state,
        milestone_id,
        health,
        freshness,
        contract_review,
        next_action,
        expected_design_work,
        delegated_gates,
        task_model,
        blockers,
        coverage,
        diagnostics,
    })
}

fn expected_design_work(
    declared_state: Option<WorkflowState>,
    freshness: &ArtifactFreshnessReport,
    traceability: Option<&artifacts::TraceabilityResolution>,
) -> Option<ExpectedDesignWork> {
    if declared_state != Some(WorkflowState::Design)
        || freshness.requirements.status != FreshnessStatus::Fresh
        || freshness.design.status != FreshnessStatus::NotReached
    {
        return None;
    }
    let missing_coverage = traceability?
        .inventory
        .issues
        .iter()
        .filter(|issue| issue.code == "TRACEABILITY_DESIGN_COVERAGE_MISSING")
        .count();
    (missing_coverage > 0).then_some(ExpectedDesignWork { missing_coverage })
}

fn next_action(
    declared_state: Option<WorkflowState>,
    freshness: &ArtifactFreshnessReport,
    contract_review: Option<ReviewFreshnessStatus>,
) -> WorkflowAction {
    match declared_state {
        None => WorkflowAction::None,
        Some(WorkflowState::Requirements) => WorkflowAction::Requirements,
        Some(WorkflowState::Design) => {
            if freshness.requirements.status == FreshnessStatus::Fresh {
                WorkflowAction::Design
            } else {
                WorkflowAction::Requirements
            }
        }
        Some(WorkflowState::Tasks) => {
            if freshness.design.status != FreshnessStatus::Fresh {
                WorkflowAction::Design
            } else if contract_review.is_some_and(|status| status != ReviewFreshnessStatus::Fresh) {
                WorkflowAction::ContractReview
            } else {
                WorkflowAction::Tasks
            }
        }
        Some(WorkflowState::Implementation) => {
            if freshness.tasks.status != FreshnessStatus::Fresh {
                WorkflowAction::Tasks
            } else if contract_review.is_some_and(|status| status != ReviewFreshnessStatus::Fresh) {
                WorkflowAction::ContractReview
            } else {
                WorkflowAction::Implementation
            }
        }
        Some(WorkflowState::ReleaseReady) => WorkflowAction::Release,
    }
}

/// Collects every gate whose accepted evidence records delegated authority.
///
/// Decision 0012 delegation skips the confirmation a user would otherwise give,
/// and Decision 0100 calls that skip auditable. The audit trail is this evidence,
/// so it needs a reader that is not the approving command's own transient output.
fn delegated_gates(evidence: &spec::v1::GateEvidence) -> Vec<DelegatedGate> {
    use spec::v1::{DesignGateEvidence, RequirementsGateEvidence, TasksGateEvidence};

    let mut delegated = Vec::new();
    if let Some(RequirementsGateEvidence::Delegated(gate)) = evidence.requirements.as_ref() {
        delegated.push(DelegatedGate {
            gate: "requirements",
            workflow: gate.delegation_workflow.0.clone(),
        });
    }
    if let Some(DesignGateEvidence::Delegated(gate)) = evidence.design.as_ref() {
        delegated.push(DelegatedGate {
            gate: "design",
            workflow: gate.delegation_workflow.0.clone(),
        });
    }
    if let Some(TasksGateEvidence::Delegated(gate)) = evidence.tasks.as_ref() {
        delegated.push(DelegatedGate {
            gate: "tasks",
            workflow: gate.delegation_workflow.0.clone(),
        });
    }
    delegated
}

/// Resolves the milestone-owned contract review where it gates this Spec.
///
/// Before the `tasks` state the review is not yet runnable, because acceptance
/// requires every participating Spec to hold current Design approval. Reporting
/// its absence there would describe an expected condition as a finding, so the
/// evaluation is skipped entirely and its Git work is not paid for.
fn contract_review(
    project_root: &Path,
    specbind_root: &Path,
    declared_state: Option<WorkflowState>,
) -> Option<ReviewFreshnessStatus> {
    if !matches!(
        declared_state,
        Some(WorkflowState::Tasks | WorkflowState::Implementation | WorkflowState::ReleaseReady)
    ) {
        return None;
    }
    let report = cross_spec_review::evaluate_freshness(project_root, specbind_root);
    // An absent `milestone_id` means no trustworthy active Roadmap was parsed, so
    // the review could not be evaluated at all. Reporting `invalid` here would
    // send a reader looking for a broken review file when the fault is the
    // Roadmap, which the milestone read model already reports with the
    // diagnostics that name it.
    report.milestone_id.as_ref()?;
    // A Direct-only Roadmap cannot participate this Spec, so `NotRequired` is a
    // contradiction the milestone read model owns rather than a barrier here.
    (report.status != ReviewFreshnessStatus::NotRequired).then_some(report.status)
}

fn task_blockers(model: &TaskReadModel) -> Vec<TaskBlocker> {
    model
        .items
        .iter()
        .flat_map(|item| match item {
            crate::task_read_model::TaskPlanItemView::Group(group) => {
                group.tasks.iter().collect::<Vec<_>>()
            }
            crate::task_read_model::TaskPlanItemView::Task(task) => vec![task.as_ref()],
        })
        .filter_map(|task| {
            task.blocked_reason.as_ref().map(|reason| TaskBlocker {
                task_id: task.id.clone(),
                reason: reason.clone(),
            })
        })
        .collect()
}

fn collect_diagnostics(
    spec_issues: &[DiscoveryIssue],
    gate_resolution: &artifacts::GateInputResolution,
    traceability: Option<&artifacts::TraceabilityResolution>,
    freshness: &ArtifactFreshnessReport,
    idle: bool,
    design_coverage_is_expected: bool,
    canonical_spec: &str,
) -> Vec<StatusDiagnostic> {
    let mut diagnostics = BTreeSet::new();
    diagnostics.extend(spec_issues.iter().map(from_discovery));
    diagnostics.extend(gate_resolution.inventory.issues.iter().map(from_discovery));
    if let Some(traceability) = traceability {
        diagnostics.extend(traceability.inventory.issues.iter().map(from_discovery));
    }
    for gate in [
        &freshness.requirements,
        &freshness.design,
        &freshness.tasks,
        &freshness.completion,
    ] {
        diagnostics.extend(gate.issues.iter().map(|issue| StatusDiagnostic {
            code: issue.code,
            path: Some(issue.path.clone()),
            message: issue.message.clone(),
        }));
    }
    if idle {
        add_idle_diagnostics(&mut diagnostics, gate_resolution, canonical_spec);
    }
    if design_coverage_is_expected {
        diagnostics.retain(|diagnostic| diagnostic.code != "TRACEABILITY_DESIGN_COVERAGE_MISSING");
    }
    diagnostics.into_iter().collect()
}

fn add_idle_diagnostics(
    diagnostics: &mut BTreeSet<StatusDiagnostic>,
    gate_resolution: &artifacts::GateInputResolution,
    canonical_spec: &str,
) {
    for artifact in &gate_resolution.inventory.artifacts {
        if matches!(artifact.kind, ArtifactKind::Brief | ArtifactKind::Research) {
            diagnostics.insert(StatusDiagnostic {
                code: "SPEC_IDLE_ARTIFACT_PRESENT",
                path: Some(artifact.path.as_str().to_owned()),
                message: format!(
                    "idle spec retains milestone-local {}",
                    artifact.artifact_type
                ),
            });
        }
    }
    if gate_resolution.inputs.tasks.is_some() {
        diagnostics.insert(StatusDiagnostic {
            code: "SPEC_IDLE_ARTIFACT_PRESENT",
            path: Some(format!("specs/{canonical_spec}/tasks.yaml")),
            message: "idle spec retains milestone-local tasks.yaml".to_owned(),
        });
    }
}

fn coverage(report: &crate::traceability::TraceabilityReport) -> Option<RequirementCoverage> {
    let active = report.active_requirement_ids.as_ref()?;
    let active_set = active.iter().collect::<BTreeSet<_>>();
    let design = report
        .design_requirement_ids
        .iter()
        .filter(|id| active_set.contains(id))
        .count();
    let tasks = report
        .task_requirement_ids
        .iter()
        .filter(|id| active_set.contains(id))
        .count();
    Some(RequirementCoverage {
        active: active.len(),
        design,
        tasks,
        tasks_required: report.tasks_required,
    })
}

fn from_discovery(issue: &DiscoveryIssue) -> StatusDiagnostic {
    StatusDiagnostic {
        code: issue.code,
        path: issue.path.as_ref().map(|path| path.as_str().to_owned()),
        message: issue.message.clone(),
    }
}

#[must_use]
pub fn state_name(state: Option<WorkflowState>) -> &'static str {
    match state {
        None => "idle",
        Some(WorkflowState::Requirements) => "requirements",
        Some(WorkflowState::Design) => "design",
        Some(WorkflowState::Tasks) => "tasks",
        Some(WorkflowState::Implementation) => "implementation",
        Some(WorkflowState::ReleaseReady) => "release_ready",
    }
}

#[must_use]
pub fn freshness_name(status: FreshnessStatus) -> &'static str {
    match status {
        FreshnessStatus::NotReached => "not_reached",
        FreshnessStatus::Fresh => "fresh",
        FreshnessStatus::Stale => "stale",
    }
}

#[must_use]
pub fn action_name(action: WorkflowAction) -> &'static str {
    match action {
        WorkflowAction::None => "none",
        WorkflowAction::Requirements => "requirements",
        WorkflowAction::Design => "design",
        WorkflowAction::ContractReview => "contract_review",
        WorkflowAction::Tasks => "tasks",
        WorkflowAction::Implementation => "implementation",
        WorkflowAction::Release => "release",
    }
}
