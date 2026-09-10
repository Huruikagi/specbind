use super::{
    ApprovalIssue, ApprovalIssues, ApprovalMode, ApprovalRequest, BuiltEvidence, Context, Gate,
    GateApproval, artifacts, cross_spec_review, discovery_failure, finish_issues, freshness, issue,
    one_issue,
};
use crate::{
    cross_spec_review::ReviewBoundary,
    domain::parse_requirement_id,
    freshness::FreshnessStatus,
    repository,
    schema::spec::v1::{
        DelegatedApprovalMode, DelegatedDesignGateEvidence, DelegatedRequirementsGateEvidence,
        DelegatedTasksGateEvidence, DesignGateEvidence, DesignInputRevisions, ExplicitApprovalMode,
        ExplicitDesignGateEvidence, ExplicitRequirementsGateEvidence, ExplicitTasksGateEvidence,
        Fingerprint as WireFingerprint, NonEmptyString, PassedAt, RequirementIdList,
        RequirementsGateEvidence, RequirementsInputRevisions, TasksGateEvidence,
        TasksInputRevisions, WorkflowState,
    },
};
use std::path::Path;

pub(super) fn build_requirements(
    project_root: &Path,
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
    let Some(document) = artifacts::resolve_requirements(specbind_root, canonical_spec) else {
        return Err(one_issue(
            "SPEC_REQUIREMENTS_ARTIFACT_MISSING",
            None,
            "cannot resolve Requirements",
        ));
    };
    let current_ids = document.requirement_ids();
    let mut issues = Vec::new();
    for id in &request.requirement_ids {
        if !current_ids.contains(&id.as_str()) && !document.is_retired(id) {
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
    let mut preservation_issues = Vec::new();
    validate_requirements_preservation(
        project_root,
        specbind_root,
        canonical_spec,
        context,
        &document,
        &requirement_ids,
        &mut preservation_issues,
    );
    finish_issues(preservation_issues)?;
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

fn validate_requirements_preservation(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    context: &Context,
    current: &crate::requirements::RequirementsDocument,
    selected: &[String],
    issues: &mut Vec<ApprovalIssue>,
) {
    if !context
        .roadmap
        .spec_updates
        .iter()
        .any(|item| item.spec == canonical_spec)
    {
        if current
            .groups
            .iter()
            .any(|group| group.retired || group.criteria.iter().any(|criterion| criterion.retired))
        {
            issues.push(issue(
                "SPEC_REQUIREMENTS_RETIREMENT_WITHOUT_BASELINE",
                None,
                "new Specs cannot declare retired identities",
            ));
        }
        return;
    }
    let Ok(specbind_relative) = specbind_root.strip_prefix(project_root) else {
        issues.push(issue(
            "SPEC_REQUIREMENTS_BASELINE_READ_FAILED",
            Some(format!("specs/{canonical_spec}/requirements.md")),
            "cannot resolve the configured SpecBind root relative to the Git project",
        ));
        return;
    };
    let relative = specbind_relative
        .join("specs")
        .join(canonical_spec)
        .join("requirements.md")
        .to_string_lossy()
        .replace('\\', "/");
    let baseline_ref = format!("{}:{relative}", context.roadmap.baseline_revision);
    let baseline = match repository::output(project_root, &["show", &baseline_ref]) {
        Ok(source) => source,
        Err(error) => {
            issues.push(issue(
                "SPEC_REQUIREMENTS_BASELINE_READ_FAILED",
                Some(format!("specs/{canonical_spec}/requirements.md")),
                format!("cannot read established Requirements at the milestone baseline: {error}"),
            ));
            return;
        }
    };
    let Some(baseline) = artifacts::requirements_from_content(&baseline) else {
        issues.push(issue(
            "SPEC_REQUIREMENTS_BASELINE_READ_FAILED",
            Some(format!("specs/{canonical_spec}/requirements.md")),
            "established Requirements at the milestone baseline are not a valid Requirements artifact",
        ));
        return;
    };
    validate_retirement_transition(canonical_spec, &baseline, current, selected, issues);
}

fn validate_retirement_transition(
    canonical_spec: &str,
    baseline: &crate::requirements::RequirementsDocument,
    current: &crate::requirements::RequirementsDocument,
    selected: &[String],
    issues: &mut Vec<ApprovalIssue>,
) {
    let current_ids = current.requirement_ids();
    let baseline_ids = baseline.requirement_ids();
    let missing = baseline_ids
        .iter()
        .filter(|id| !current_ids.contains(id) && !current.is_retired(id))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        issues.push(issue(
            "SPEC_REQUIREMENTS_RETIREMENT_UNSUPPORTED",
            Some(format!("specs/{canonical_spec}/requirements.md")),
            format!(
                "requirements approval cannot remove established Requirement IDs: {}",
                missing.join(", ")
            ),
        ));
    }
    if current.live_requirement_ids().is_empty() {
        issues.push(issue(
            "SPEC_REQUIREMENTS_SPEC_RETIREMENT_UNSUPPORTED",
            None,
            "retiring all Spec obligations requires Spec retirement",
        ));
    }
    for group in &baseline.groups {
        if group.retired
            && !current
                .groups
                .iter()
                .any(|candidate| candidate.number == group.number && candidate.retired)
        {
            issues.push(issue(
                "SPEC_REQUIREMENTS_RETIRED_ID_REUSED",
                None,
                format!("retired group {} must remain retired", group.number),
            ));
        }
    }
    for id in &baseline_ids {
        if baseline.is_retired(id) && !current.is_retired(id) {
            issues.push(issue(
                "SPEC_REQUIREMENTS_RETIRED_ID_REUSED",
                None,
                format!("retired ID {id} must remain retired"),
            ));
        }
        if !baseline.is_retired(id)
            && current.is_retired(id)
            && !selected.iter().any(|selected| selected == id)
        {
            issues.push(issue(
                "SPEC_REQUIREMENTS_RETIREMENT_SELECTION_MISSING",
                None,
                format!("newly retired ID {id} must be selected for delivery and verification"),
            ));
        }
    }
    for id in selected {
        if current.is_retired(id)
            && (baseline.is_retired(id) || !baseline_ids.contains(&id.as_str()))
        {
            issues.push(issue(
                "SPEC_REQUIREMENTS_RETIREMENT_SELECTION_INVALID",
                None,
                format!("retirement selection {id} must identify a live baseline obligation"),
            ));
        }
    }
    for id in &current_ids {
        if current.is_retired(id) && !baseline.is_retired(id) && !baseline_ids.contains(id) {
            issues.push(issue(
                "SPEC_REQUIREMENTS_RETIREMENT_WITHOUT_BASELINE",
                None,
                format!("retired ID {id} has no baseline obligation"),
            ));
        }
    }
    for group in &current.groups {
        if group.retired && !baseline.groups.iter().any(|old| old.number == group.number) {
            issues.push(issue(
                "SPEC_REQUIREMENTS_RETIREMENT_WITHOUT_BASELINE",
                None,
                format!("retired group {} has no baseline", group.number),
            ));
        }
    }
}

pub(super) fn build_design(
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
            Some(format!("specs/{canonical_spec}/contract.yaml")),
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

pub(super) fn build_tasks(
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
    let resolution = if gate == Gate::Design {
        artifacts::resolve_design_traceability(specbind_root, canonical_spec)
    } else {
        artifacts::resolve_traceability(specbind_root, canonical_spec)
    };
    let Some(report) = resolution.report else {
        return Err(discovery_failure(resolution.inventory.issues));
    };
    for value in &report.issues {
        issues.push(issue(
            value.code,
            value.source.clone(),
            value.message.clone(),
        ));
    }
    Ok(())
}

pub(super) fn identical_fresh_approval(
    context: &Context,
    request: &ApprovalRequest,
) -> Option<GateApproval> {
    let active = context.wire.active_change.0.as_ref()?;
    let expected_state =
        if request.gate == Gate::Design && !context.roadmap.reverse_specs.is_empty() {
            WorkflowState::AdoptionReady
        } else {
            request.gate.approved_state()
        };
    if active.state != expected_state {
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
        state: active.state,
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

pub(super) fn clears_anything(active: &crate::schema::spec::v1::ActiveChange, gate: Gate) -> bool {
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
