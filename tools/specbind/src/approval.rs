//! Guarded Spec gate approval and invalidation transitions.

use std::{fmt, fs, path::Path};

use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    artifacts::{self, DiscoveryIssue},
    cross_spec_review::{self, ReviewBoundary},
    domain::{parse_requirement_id, spec::Spec},
    freshness::{self, CurrentGateInputs, FreshnessStatus},
    guarded_fs, repository,
    roadmap::{self, RoadmapDocument},
    schema::spec::v1::{
        DelegatedApprovalMode, DelegatedDesignGateEvidence, DelegatedRequirementsGateEvidence,
        DelegatedTasksGateEvidence, DesignGateEvidence, DesignInputRevisions, ExplicitApprovalMode,
        ExplicitDesignGateEvidence, ExplicitRequirementsGateEvidence, ExplicitTasksGateEvidence,
        Fingerprint as WireFingerprint, GateEvidence, NonEmptyString, PassedAt, RequirementIdList,
        RequirementsGateEvidence, RequirementsInputRevisions, SpecDocument, TasksGateEvidence,
        TasksInputRevisions, WorkflowState,
    },
};

/// The three Spec gates whose approval evidence this module owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gate {
    Requirements,
    Design,
    Tasks,
}

/// Decision 0012 approval authority for one gate crossing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalMode {
    Explicit,
    Delegated { workflow: String },
}

/// One transient approval request. Every durable input is derived by the CLI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalRequest {
    pub gate: Gate,
    pub mode: ApprovalMode,
    /// Submitted active Requirement ID selection; empty except for the
    /// requirements gate, whose selection cannot be derived.
    pub requirement_ids: Vec<String>,
}

/// The persisted result of one gate crossing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateApproval {
    pub state: WorkflowState,
    pub passed_at: String,
    pub approval_mode: &'static str,
    pub delegation_workflow: Option<String>,
    pub approved_requirement_ids: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApproveOutcome {
    Approved(GateApproval),
    AlreadyApproved(GateApproval),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidateOutcome {
    Invalidated { state: WorkflowState },
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApprovalIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalIssues {
    pub issues: Vec<ApprovalIssue>,
}

impl fmt::Display for ApprovalIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "gate approval operation has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for ApprovalIssues {}

impl Gate {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Requirements => "requirements",
            Self::Design => "design",
            Self::Tasks => "tasks",
        }
    }

    /// The only declared state from which this gate may be approved.
    fn required_state(self) -> WorkflowState {
        match self {
            Self::Requirements => WorkflowState::Requirements,
            Self::Design => WorkflowState::Design,
            Self::Tasks => WorkflowState::Tasks,
        }
    }

    /// The declared state recorded by a successful approval.
    fn approved_state(self) -> WorkflowState {
        match self {
            Self::Requirements => WorkflowState::Design,
            Self::Design => WorkflowState::Tasks,
            Self::Tasks => WorkflowState::Implementation,
        }
    }

    fn approve_failed(self) -> &'static str {
        match self {
            Self::Requirements => "SPEC_REQUIREMENTS_STATE_INVALID",
            Self::Design => "SPEC_DESIGN_STATE_INVALID",
            Self::Tasks => "SPEC_TASKS_STATE_INVALID",
        }
    }

    fn target_invalid(self) -> &'static str {
        match self {
            Self::Requirements => "SPEC_REQUIREMENTS_TARGET_INVALID",
            Self::Design => "SPEC_DESIGN_TARGET_INVALID",
            Self::Tasks => "SPEC_TASKS_TARGET_INVALID",
        }
    }

    fn target_dirty(self) -> &'static str {
        match self {
            Self::Requirements => "SPEC_REQUIREMENTS_TARGET_DIRTY",
            Self::Design => "SPEC_DESIGN_TARGET_DIRTY",
            Self::Tasks => "SPEC_TASKS_TARGET_DIRTY",
        }
    }
}

/// Approves one Spec gate and records its evidence.
///
/// # Errors
///
/// Returns deterministic request, lifecycle, milestone, freshness, coverage,
/// review, race, serialization, or filesystem diagnostics without partially
/// changing `spec.yaml`.
pub fn approve(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    request: &ApprovalRequest,
) -> Result<ApproveOutcome, ApprovalIssues> {
    let gate = request.gate;
    validate_request(request)?;
    let initial = resolve_context(specbind_root, canonical_spec, gate)?;
    if let Some(existing) = identical_fresh_approval(&initial, request) {
        return Ok(ApproveOutcome::AlreadyApproved(existing));
    }
    let evidence = guard_and_build(
        project_root,
        specbind_root,
        canonical_spec,
        &initial,
        request,
    )?;

    let current = resolve_context(specbind_root, canonical_spec, gate)?;
    if current.source != initial.source || current.inputs != initial.inputs {
        return Err(one_issue(
            "SPEC_GATE_INPUTS_CHANGED",
            Some(spec_path(canonical_spec)),
            "gate approval inputs changed during guarded acceptance",
        ));
    }

    let mut wire = current.wire;
    let active = wire.active_change.0.as_mut().ok_or_else(|| {
        one_issue(
            gate.approve_failed(),
            Some(spec_path(canonical_spec)),
            "gate approval lost its active change",
        )
    })?;
    active.state = gate.approved_state();
    let mut container = active.gate_evidence.take().unwrap_or(GateEvidence {
        requirements: None,
        design: None,
        tasks: None,
        completion: None,
    });
    let approval = match evidence {
        BuiltEvidence::Requirements {
            evidence,
            approved,
            requirement_ids,
        } => {
            active.requirement_ids.0 = Some(RequirementIdList(requirement_ids));
            container.requirements = Some(evidence);
            container.design = None;
            container.tasks = None;
            container.completion = None;
            approved
        }
        BuiltEvidence::Design { evidence, approved } => {
            container.design = Some(evidence);
            container.tasks = None;
            container.completion = None;
            approved
        }
        BuiltEvidence::Tasks { evidence, approved } => {
            container.tasks = Some(evidence);
            container.completion = None;
            approved
        }
    };
    active.gate_evidence = Some(container);
    persist(specbind_root, canonical_spec, &wire, gate)?;
    Ok(ApproveOutcome::Approved(approval))
}

/// Rewinds one Spec gate and clears its cumulative downstream evidence.
///
/// # Errors
///
/// Returns lifecycle, target-path, serialization, or filesystem diagnostics.
pub fn invalidate(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    gate: Gate,
) -> Result<InvalidateOutcome, ApprovalIssues> {
    if !artifacts::canonical_id(canonical_spec) {
        return Err(one_issue(
            gate.target_invalid(),
            Some(format!("specs/{canonical_spec}")),
            "gate invalidation requires a canonical Spec ID",
        ));
    }
    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(mut wire) = resolution.wire else {
        return Err(discovery_failure(resolution.issues));
    };
    let Some(active) = wire.active_change.0.as_ref() else {
        return Err(one_issue(
            gate.approve_failed(),
            Some(spec_path(canonical_spec)),
            "gate invalidation requires an active change",
        ));
    };
    if !clears_anything(active, gate) {
        return Ok(InvalidateOutcome::NoChange);
    }

    let mut issues = Vec::new();
    ensure_target_clean(
        project_root,
        specbind_root,
        canonical_spec,
        gate,
        &mut issues,
    );
    finish_issues(issues)?;

    let Some(active) = wire.active_change.0.as_mut() else {
        return Err(one_issue(
            gate.approve_failed(),
            Some(spec_path(canonical_spec)),
            "gate invalidation lost its active change",
        ));
    };
    active.state = gate.required_state();
    if let Some(container) = active.gate_evidence.as_mut() {
        match gate {
            Gate::Requirements => {
                active.requirement_ids.0 = None;
                container.requirements = None;
                container.design = None;
                container.tasks = None;
                container.completion = None;
            }
            Gate::Design => {
                container.design = None;
                container.tasks = None;
                container.completion = None;
            }
            Gate::Tasks => {
                container.tasks = None;
                container.completion = None;
            }
        }
        if container.requirements.is_none()
            && container.design.is_none()
            && container.tasks.is_none()
            && container.completion.is_none()
        {
            active.gate_evidence = None;
        }
    } else if gate == Gate::Requirements {
        active.requirement_ids.0 = None;
    }
    persist(specbind_root, canonical_spec, &wire, gate)?;
    Ok(InvalidateOutcome::Invalidated {
        state: gate.required_state(),
    })
}

struct Context {
    source: String,
    wire: SpecDocument,
    inputs: CurrentGateInputs,
    inventory_issues: Vec<DiscoveryIssue>,
    roadmap: RoadmapDocument,
}

enum BuiltEvidence {
    Requirements {
        evidence: RequirementsGateEvidence,
        approved: GateApproval,
        requirement_ids: Vec<String>,
    },
    Design {
        evidence: DesignGateEvidence,
        approved: GateApproval,
    },
    Tasks {
        evidence: TasksGateEvidence,
        approved: GateApproval,
    },
}

fn validate_request(request: &ApprovalRequest) -> Result<(), ApprovalIssues> {
    let mut issues = Vec::new();
    if let ApprovalMode::Delegated { workflow } = &request.mode
        && workflow.trim().is_empty()
    {
        issues.push(issue(
            "SPEC_GATE_DELEGATION_INVALID",
            None,
            "delegated approval requires a non-empty delegation workflow",
        ));
    }
    if request.gate == Gate::Requirements {
        if request.requirement_ids.is_empty() {
            issues.push(issue(
                "SPEC_REQUIREMENTS_SELECTION_EMPTY",
                None,
                "requirements approval requires a non-empty active Requirement ID selection",
            ));
        }
        if request
            .requirement_ids
            .iter()
            .any(|id| parse_requirement_id(id).is_none())
        {
            issues.push(issue(
                "SPEC_REQUIREMENTS_SELECTION_INVALID",
                None,
                "Requirement IDs must use positive numeric N.M form without leading zeroes",
            ));
        }
    } else if !request.requirement_ids.is_empty() {
        issues.push(issue(
            "SPEC_GATE_SELECTION_UNSUPPORTED",
            None,
            "only requirements approval accepts an active Requirement ID selection",
        ));
    }
    finish_issues(issues)
}

fn resolve_context(
    specbind_root: &Path,
    canonical_spec: &str,
    gate: Gate,
) -> Result<Context, ApprovalIssues> {
    if !artifacts::canonical_id(canonical_spec) {
        return Err(one_issue(
            gate.target_invalid(),
            Some(format!("specs/{canonical_spec}")),
            "gate approval requires a canonical Spec ID",
        ));
    }
    let relative = "steering/roadmap.md";
    let path = specbind_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "SPEC_GATE_ROADMAP_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(one_issue(
            "SPEC_GATE_ROADMAP_INVALID",
            Some(relative.to_owned()),
            "steering/roadmap.md must be a regular non-symlink file",
        ));
    }
    let content = fs::read_to_string(&path).map_err(|error| {
        one_issue(
            "SPEC_GATE_ROADMAP_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    let roadmap = roadmap::parse(&content).map_err(|error| ApprovalIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, Some(relative.to_owned()), value.message))
            .collect(),
    })?;

    let source = read_regular(specbind_root, canonical_spec, gate)?;
    let gate_inputs = artifacts::resolve_gate_inputs(specbind_root, canonical_spec);
    let resolution = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(wire) = resolution.wire else {
        return Err(discovery_failure(resolution.issues));
    };
    Ok(Context {
        source,
        wire,
        inputs: gate_inputs.inputs,
        inventory_issues: gate_inputs.inventory.issues,
        roadmap,
    })
}

fn guard_and_build(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    context: &Context,
    request: &ApprovalRequest,
) -> Result<BuiltEvidence, ApprovalIssues> {
    let gate = request.gate;
    let mut issues = context
        .inventory_issues
        .iter()
        .cloned()
        .map(from_discovery)
        .collect::<Vec<_>>();
    validate_participation(context, canonical_spec, gate, &mut issues);
    validate_declared_state(context, canonical_spec, gate, &mut issues);
    validate_prior_gates(context, canonical_spec, gate, &mut issues);
    finish_issues(issues)?;

    let passed_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| {
            one_issue(
                "SPEC_GATE_TIMESTAMP_FAILED",
                Some(spec_path(canonical_spec)),
                error.to_string(),
            )
        })?;
    match gate {
        Gate::Requirements => {
            build_requirements(specbind_root, canonical_spec, context, request, passed_at)
        }
        Gate::Design => build_design(specbind_root, canonical_spec, context, request, passed_at),
        Gate::Tasks => build_tasks(
            project_root,
            specbind_root,
            canonical_spec,
            context,
            request,
            passed_at,
        ),
    }
}

fn validate_participation(
    context: &Context,
    canonical_spec: &str,
    gate: Gate,
    issues: &mut Vec<ApprovalIssue>,
) {
    if !context
        .roadmap
        .spec_ids()
        .iter()
        .any(|spec| spec == canonical_spec)
    {
        issues.push(issue(
            "SPEC_GATE_NOT_IN_MILESTONE",
            Some(spec_path(canonical_spec)),
            "gate approval target must participate in the active Roadmap",
        ));
        return;
    }
    let matching = context
        .wire
        .active_change
        .0
        .as_ref()
        .is_some_and(|active| active.milestone_id.0 == context.roadmap.milestone_id);
    if !matching {
        issues.push(issue(
            gate.approve_failed(),
            Some(spec_path(canonical_spec)),
            "active change milestone does not match the active Roadmap",
        ));
    }
}

fn validate_declared_state(
    context: &Context,
    canonical_spec: &str,
    gate: Gate,
    issues: &mut Vec<ApprovalIssue>,
) {
    let declared = context
        .wire
        .active_change
        .0
        .as_ref()
        .map(|active| active.state);
    if declared != Some(gate.required_state()) {
        issues.push(issue(
            gate.approve_failed(),
            Some(spec_path(canonical_spec)),
            format!(
                "{} approval requires the Spec in {} state",
                gate.name(),
                state_name(gate.required_state())
            ),
        ));
    }
}

fn validate_prior_gates(
    context: &Context,
    canonical_spec: &str,
    gate: Gate,
    issues: &mut Vec<ApprovalIssue>,
) {
    let required: &[(&str, Gate)] = match gate {
        Gate::Requirements => &[],
        Gate::Design => &[("Requirements", Gate::Requirements)],
        Gate::Tasks => &[
            ("Requirements", Gate::Requirements),
            ("Design", Gate::Design),
        ],
    };
    if required.is_empty() {
        return;
    }
    let report = freshness::evaluate_wire(&context.wire, &context.inputs);
    for (name, prior) in required {
        let status = match prior {
            Gate::Requirements => report.requirements.status,
            Gate::Design => report.design.status,
            Gate::Tasks => report.tasks.status,
        };
        if status != FreshnessStatus::Fresh {
            issues.push(issue(
                "SPEC_GATE_PRIOR_STALE",
                Some(spec_path(canonical_spec)),
                format!(
                    "{name} gate must remain fresh before {} approval",
                    gate.name()
                ),
            ));
        }
    }
}

fn build_requirements(
    specbind_root: &Path,
    canonical_spec: &str,
    context: &Context,
    request: &ApprovalRequest,
    passed_at: String,
) -> Result<BuiltEvidence, ApprovalIssues> {
    let Some(fingerprint) = context.inputs.requirements.as_ref() else {
        return Err(one_issue(
            "SPEC_REQUIREMENTS_ARTIFACT_MISSING",
            Some(format!("specs/{canonical_spec}/requirements.md")),
            "requirements approval requires a valid Requirements artifact",
        ));
    };
    let resolution = artifacts::resolve_traceability(specbind_root, canonical_spec);
    let Some(report) = resolution.report else {
        return Err(discovery_failure(resolution.inventory.issues));
    };
    let mut issues = Vec::new();
    for id in &request.requirement_ids {
        if !report.requirement_ids.contains(id) {
            issues.push(issue(
                "SPEC_REQUIREMENTS_SELECTION_UNKNOWN",
                Some(format!("specs/{canonical_spec}/requirements.md")),
                format!("Requirement ID {id} does not exist in the Requirements artifact"),
            ));
        }
    }
    finish_issues(issues)?;

    let requirement_ids = canonical_order(&request.requirement_ids);
    if requirement_ids.len() != request.requirement_ids.len() {
        return Err(one_issue(
            "SPEC_REQUIREMENTS_SELECTION_DUPLICATE",
            None,
            "the active Requirement ID selection must be unique",
        ));
    }
    let input_revisions = RequirementsInputRevisions {
        requirements: WireFingerprint(fingerprint.to_string()),
    };
    let approved = GateApproval {
        state: Gate::Requirements.approved_state(),
        passed_at: passed_at.clone(),
        approval_mode: mode_name(&request.mode),
        delegation_workflow: delegation_workflow(&request.mode),
        approved_requirement_ids: Some(requirement_ids.len()),
    };
    let evidence = match &request.mode {
        ApprovalMode::Explicit => {
            RequirementsGateEvidence::Explicit(ExplicitRequirementsGateEvidence {
                passed_at: PassedAt(passed_at),
                approval_mode: ExplicitApprovalMode::Explicit,
                approved_requirement_ids: RequirementIdList(requirement_ids.clone()),
                input_revisions,
            })
        }
        ApprovalMode::Delegated { workflow } => {
            RequirementsGateEvidence::Delegated(DelegatedRequirementsGateEvidence {
                passed_at: PassedAt(passed_at),
                approval_mode: DelegatedApprovalMode::Delegated,
                delegation_workflow: NonEmptyString(workflow.clone()),
                approved_requirement_ids: RequirementIdList(requirement_ids.clone()),
                input_revisions,
            })
        }
    };
    Ok(BuiltEvidence::Requirements {
        evidence,
        approved,
        requirement_ids,
    })
}

fn build_design(
    specbind_root: &Path,
    canonical_spec: &str,
    context: &Context,
    request: &ApprovalRequest,
    passed_at: String,
) -> Result<BuiltEvidence, ApprovalIssues> {
    let Some(design) = context.inputs.design.as_ref() else {
        return Err(one_issue(
            "SPEC_DESIGN_ARTIFACT_MISSING",
            Some(format!("specs/{canonical_spec}")),
            "design approval requires the Contract and at least one Design artifact",
        ));
    };
    let mut issues = Vec::new();
    if !design.keys().any(|key| key == "contract") {
        issues.push(issue(
            "SPEC_DESIGN_CONTRACT_MISSING",
            Some(format!("specs/{canonical_spec}/contract.md")),
            "design approval requires the singleton Contract artifact",
        ));
    }
    if !design.keys().any(|key| key.starts_with("design/")) {
        issues.push(issue(
            "SPEC_DESIGN_ARTIFACT_MISSING",
            Some(format!("specs/{canonical_spec}")),
            "design approval requires at least one Design artifact",
        ));
    }
    validate_traceability(specbind_root, canonical_spec, Gate::Design, &mut issues)?;
    finish_issues(issues)?;

    let input_revisions = DesignInputRevisions(
        design
            .iter()
            .map(|(key, value)| (key.clone(), WireFingerprint(value.to_string())))
            .collect(),
    );
    let approved = GateApproval {
        state: Gate::Design.approved_state(),
        passed_at: passed_at.clone(),
        approval_mode: mode_name(&request.mode),
        delegation_workflow: delegation_workflow(&request.mode),
        approved_requirement_ids: None,
    };
    let evidence = match &request.mode {
        ApprovalMode::Explicit => DesignGateEvidence::Explicit(ExplicitDesignGateEvidence {
            passed_at: PassedAt(passed_at),
            approval_mode: ExplicitApprovalMode::Explicit,
            input_revisions,
        }),
        ApprovalMode::Delegated { workflow } => {
            DesignGateEvidence::Delegated(DelegatedDesignGateEvidence {
                passed_at: PassedAt(passed_at),
                approval_mode: DelegatedApprovalMode::Delegated,
                delegation_workflow: NonEmptyString(workflow.clone()),
                input_revisions,
            })
        }
    };
    Ok(BuiltEvidence::Design { evidence, approved })
}

fn build_tasks(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    context: &Context,
    request: &ApprovalRequest,
    passed_at: String,
) -> Result<BuiltEvidence, ApprovalIssues> {
    let Some(fingerprint) = context.inputs.task_plan.as_ref() else {
        return Err(one_issue(
            "SPEC_TASKS_ARTIFACT_MISSING",
            Some(format!("specs/{canonical_spec}/tasks.yaml")),
            "tasks approval requires a valid task plan",
        ));
    };
    let mut issues = Vec::new();
    validate_traceability(specbind_root, canonical_spec, Gate::Tasks, &mut issues)?;
    if let Err(error) = cross_spec_review::require_for_boundary(
        project_root,
        specbind_root,
        ReviewBoundary::TasksApproval { canonical_spec },
    ) {
        issues.extend(
            error
                .issues
                .into_iter()
                .map(|value| issue(value.code, value.source, value.message)),
        );
    }
    finish_issues(issues)?;

    let input_revisions = TasksInputRevisions {
        plan: WireFingerprint(fingerprint.to_string()),
    };
    let approved = GateApproval {
        state: Gate::Tasks.approved_state(),
        passed_at: passed_at.clone(),
        approval_mode: mode_name(&request.mode),
        delegation_workflow: delegation_workflow(&request.mode),
        approved_requirement_ids: None,
    };
    let evidence = match &request.mode {
        ApprovalMode::Explicit => TasksGateEvidence::Explicit(ExplicitTasksGateEvidence {
            passed_at: PassedAt(passed_at),
            approval_mode: ExplicitApprovalMode::Explicit,
            input_revisions,
        }),
        ApprovalMode::Delegated { workflow } => {
            TasksGateEvidence::Delegated(DelegatedTasksGateEvidence {
                passed_at: PassedAt(passed_at),
                approval_mode: DelegatedApprovalMode::Delegated,
                delegation_workflow: NonEmptyString(workflow.clone()),
                input_revisions,
            })
        }
    };
    Ok(BuiltEvidence::Tasks { evidence, approved })
}

fn validate_traceability(
    specbind_root: &Path,
    canonical_spec: &str,
    gate: Gate,
    issues: &mut Vec<ApprovalIssue>,
) -> Result<(), ApprovalIssues> {
    let resolution = artifacts::resolve_traceability(specbind_root, canonical_spec);
    let Some(report) = resolution.report else {
        return Err(discovery_failure(resolution.inventory.issues));
    };
    for value in &report.issues {
        let task_scoped = value.code.starts_with("TRACEABILITY_TASK");
        if gate == Gate::Design && task_scoped {
            continue;
        }
        issues.push(issue(
            value.code,
            value.source.clone(),
            value.message.clone(),
        ));
    }
    Ok(())
}

fn identical_fresh_approval(context: &Context, request: &ApprovalRequest) -> Option<GateApproval> {
    let active = context.wire.active_change.0.as_ref()?;
    if active.state != request.gate.approved_state() {
        return None;
    }
    let container = active.gate_evidence.as_ref()?;
    let report = freshness::evaluate_wire(&context.wire, &context.inputs);
    let (status, approval) = match request.gate {
        Gate::Requirements => (
            report.requirements.status,
            container.requirements.as_ref().map(requirements_view),
        ),
        Gate::Design => (
            report.design.status,
            container.design.as_ref().map(design_view),
        ),
        Gate::Tasks => (
            report.tasks.status,
            container.tasks.as_ref().map(tasks_view),
        ),
    };
    if status != FreshnessStatus::Fresh {
        return None;
    }
    let (mode, workflow, approved_ids, passed_at) = approval?;
    if mode != mode_name(&request.mode) || workflow != delegation_workflow(&request.mode) {
        return None;
    }
    if request.gate == Gate::Requirements
        && approved_ids.as_deref() != Some(canonical_order(&request.requirement_ids).as_slice())
    {
        return None;
    }
    Some(GateApproval {
        state: request.gate.approved_state(),
        passed_at,
        approval_mode: mode,
        delegation_workflow: workflow,
        approved_requirement_ids: approved_ids.map(|ids| ids.len()),
    })
}

type ApprovalView = (&'static str, Option<String>, Option<Vec<String>>, String);

fn requirements_view(evidence: &RequirementsGateEvidence) -> ApprovalView {
    match evidence {
        RequirementsGateEvidence::Explicit(value) => (
            "explicit",
            None,
            Some(value.approved_requirement_ids.0.clone()),
            value.passed_at.0.clone(),
        ),
        RequirementsGateEvidence::Delegated(value) => (
            "delegated",
            Some(value.delegation_workflow.0.clone()),
            Some(value.approved_requirement_ids.0.clone()),
            value.passed_at.0.clone(),
        ),
    }
}

fn design_view(evidence: &DesignGateEvidence) -> ApprovalView {
    match evidence {
        DesignGateEvidence::Explicit(value) => ("explicit", None, None, value.passed_at.0.clone()),
        DesignGateEvidence::Delegated(value) => (
            "delegated",
            Some(value.delegation_workflow.0.clone()),
            None,
            value.passed_at.0.clone(),
        ),
    }
}

fn tasks_view(evidence: &TasksGateEvidence) -> ApprovalView {
    match evidence {
        TasksGateEvidence::Explicit(value) => ("explicit", None, None, value.passed_at.0.clone()),
        TasksGateEvidence::Delegated(value) => (
            "delegated",
            Some(value.delegation_workflow.0.clone()),
            None,
            value.passed_at.0.clone(),
        ),
    }
}

fn clears_anything(active: &crate::schema::spec::v1::ActiveChange, gate: Gate) -> bool {
    if active.state != gate.required_state() {
        return true;
    }
    let Some(container) = active.gate_evidence.as_ref() else {
        return gate == Gate::Requirements && active.requirement_ids.0.is_some();
    };
    match gate {
        Gate::Requirements => {
            active.requirement_ids.0.is_some()
                || container.requirements.is_some()
                || container.design.is_some()
                || container.tasks.is_some()
                || container.completion.is_some()
        }
        Gate::Design => {
            container.design.is_some()
                || container.tasks.is_some()
                || container.completion.is_some()
        }
        Gate::Tasks => container.tasks.is_some() || container.completion.is_some(),
    }
}

fn canonical_order(ids: &[String]) -> Vec<String> {
    let mut ordered = ids.to_vec();
    ordered.sort_by_key(|id| parse_requirement_id(id));
    ordered.dedup();
    ordered
}

fn mode_name(mode: &ApprovalMode) -> &'static str {
    match mode {
        ApprovalMode::Explicit => "explicit",
        ApprovalMode::Delegated { .. } => "delegated",
    }
}

fn delegation_workflow(mode: &ApprovalMode) -> Option<String> {
    match mode {
        ApprovalMode::Explicit => None,
        ApprovalMode::Delegated { workflow } => Some(workflow.clone()),
    }
}

/// Renders one declared lifecycle state as its stable public name.
#[must_use]
pub fn state_name(state: WorkflowState) -> &'static str {
    match state {
        WorkflowState::Requirements => "requirements",
        WorkflowState::Design => "design",
        WorkflowState::Tasks => "tasks",
        WorkflowState::Implementation => "implementation",
        WorkflowState::ReleaseReady => "release_ready",
    }
}

fn persist(
    specbind_root: &Path,
    canonical_spec: &str,
    wire: &SpecDocument,
    gate: Gate,
) -> Result<(), ApprovalIssues> {
    Spec::try_from(wire.clone()).map_err(|error| ApprovalIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, Some(spec_path(canonical_spec)), value.message))
            .collect(),
    })?;
    let mut rendered = serde_saphyr::to_string(wire).map_err(|error| {
        one_issue(
            "SPEC_GATE_SERIALIZE_FAILED",
            Some(spec_path(canonical_spec)),
            error.to_string(),
        )
    })?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    guarded_fs::replace_existing(
        &specbind_root.join(spec_path(canonical_spec)),
        rendered.as_bytes(),
    )
    .map_err(|error| {
        one_issue(
            gate.target_invalid(),
            Some(spec_path(canonical_spec)),
            error.to_string(),
        )
    })
}

fn ensure_target_clean(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    gate: Gate,
    issues: &mut Vec<ApprovalIssue>,
) {
    let relative = spec_path(canonical_spec);
    let Ok(root_relative) = specbind_root.strip_prefix(project_root) else {
        issues.push(issue(
            "SPEC_GATE_PROJECT_ROOT_INVALID",
            Some(relative),
            "SpecBind root must be below the Git project root",
        ));
        return;
    };
    let path = root_relative
        .join(&relative)
        .to_string_lossy()
        .replace('\\', "/");
    match repository::path_status(project_root, &path) {
        Ok(output) if output.is_empty() => {}
        Ok(_) => issues.push(issue(
            gate.target_dirty(),
            Some(relative),
            "gate invalidation refuses to overwrite a dirty or staged spec.yaml",
        )),
        Err(error) => issues.push(issue(
            "SPEC_GATE_GIT_FAILED",
            Some(relative),
            error.to_string(),
        )),
    }
}

fn read_regular(
    specbind_root: &Path,
    canonical_spec: &str,
    gate: Gate,
) -> Result<String, ApprovalIssues> {
    let relative = spec_path(canonical_spec);
    let path = specbind_root.join(&relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            gate.target_invalid(),
            Some(relative.clone()),
            error.to_string(),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(one_issue(
            gate.target_invalid(),
            Some(relative),
            "spec.yaml must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(&path)
        .map_err(|error| one_issue(gate.target_invalid(), Some(relative), error.to_string()))
}

fn spec_path(canonical_spec: &str) -> String {
    format!("specs/{canonical_spec}/spec.yaml")
}

fn discovery_failure(issues: Vec<DiscoveryIssue>) -> ApprovalIssues {
    ApprovalIssues {
        issues: issues.into_iter().map(from_discovery).collect(),
    }
}

fn from_discovery(value: DiscoveryIssue) -> ApprovalIssue {
    issue(
        value.code,
        value.path.map(|path| path.to_string()),
        value.message,
    )
}

fn finish_issues(mut issues: Vec<ApprovalIssue>) -> Result<(), ApprovalIssues> {
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(ApprovalIssues { issues })
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> ApprovalIssues {
    ApprovalIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> ApprovalIssue {
    ApprovalIssue {
        code,
        path,
        message: message.into(),
    }
}
