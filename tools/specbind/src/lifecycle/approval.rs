//! Lifecycle service for guarded Spec gate approval and invalidation transitions.

mod evidence;
mod persistence;

use evidence::{
    build_design, build_requirements, build_tasks, clears_anything, identical_fresh_approval,
};
use persistence::{ensure_target_clean, persist, read_regular};

use std::{fmt, fs, path::Path};

use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    artifacts::{self, DiscoveryIssue},
    cross_spec_review,
    domain::parse_requirement_id,
    freshness::{self, CurrentGateInputs, FreshnessStatus},
    roadmap::{self, RoadmapDocument},
    schema::spec::v1::{
        DesignGateEvidence, GateEvidence, RequirementIdList, RequirementsGateEvidence,
        SpecDocument, TasksGateEvidence, WorkflowState,
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
    Invalidated {
        state: WorkflowState,
        /// True when this rewind also removed the accepted contract review.
        review_removed: bool,
    },
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
    /// Whether rewinding this gate invalidates the milestone-owned cross-spec
    /// review under Decision 0078.
    ///
    /// The review is accepted between Design approval and Tasks authoring, so a
    /// Tasks rewind leaves it intact while an earlier rewind removes it.
    fn rewind_removes_review(self) -> bool {
        match self {
            Self::Requirements | Self::Design => true,
            Self::Tasks => false,
        }
    }

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
    active.state = approved_state(&current.roadmap, canonical_spec, gate);
    let mut container = active.gate_evidence.take().unwrap_or(GateEvidence {
        requirements: None,
        design: None,
        tasks: None,
        completion: None,
    });
    let mut approval = match evidence {
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
    approval.state = active.state;
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

    // Remove the milestone review before rewinding the Spec. An interruption
    // then leaves a milestone that visibly needs a new review, never a rewound
    // Spec behind a review that still claims to cover it.
    let review_removed = if gate.rewind_removes_review() {
        cross_spec_review::remove_accepted(specbind_root).map_err(|error| ApprovalIssues {
            issues: error
                .issues
                .into_iter()
                .map(|value| issue(value.code, value.source, value.message))
                .collect(),
        })?
    } else {
        false
    };

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
        review_removed,
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
    if !crate::guarded_fs::is_regular_file(&metadata) {
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
    let gate_inputs = if gate == Gate::Design {
        artifacts::resolve_design_gate_inputs(specbind_root, canonical_spec)
    } else {
        artifacts::resolve_gate_inputs(specbind_root, canonical_spec)
    };
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
        Gate::Requirements => build_requirements(
            project_root,
            specbind_root,
            canonical_spec,
            context,
            request,
            passed_at,
        ),
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

/// Renders one declared lifecycle state as its stable public name.
#[must_use]
pub fn state_name(state: WorkflowState) -> &'static str {
    match state {
        WorkflowState::Requirements => "requirements",
        WorkflowState::Design => "design",
        WorkflowState::Tasks => "tasks",
        WorkflowState::AdoptionReady => "adoption_ready",
        WorkflowState::Implementation => "implementation",
        WorkflowState::ReleaseReady => "release_ready",
    }
}

fn approved_state(
    roadmap: &crate::roadmap::RoadmapDocument,
    canonical_spec: &str,
    gate: Gate,
) -> WorkflowState {
    if gate == Gate::Design
        && roadmap
            .reverse_specs
            .iter()
            .any(|item| canonical_spec.is_empty() || item.spec == canonical_spec)
    {
        WorkflowState::AdoptionReady
    } else {
        gate.approved_state()
    }
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
