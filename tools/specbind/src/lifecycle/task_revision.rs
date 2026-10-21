//! Guarded replacement of an existing Task plan and its sparse execution map.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::LazyLock,
};

use sha2::{Digest as _, Sha256};

use crate::{
    artifacts,
    cross_spec_review::{self, ReviewBoundary},
    domain::tasks::Tasks,
    freshness::{self, FreshnessStatus},
    guarded_fs, roadmap, schema,
    schema::{
        spec::v1::WorkflowState,
        task_revision::v1::{Disposition, TaskRevision},
        tasks::v1::{
            PlanItem, SchemaVersion, TaskExecution, TaskExecutionStates, TaskReference,
            TasksDocument,
        },
    },
    traceability::{self, DesignRequirementSet, TaskRequirementSet},
};

#[derive(Debug, thiserror::Error)]
#[error("{code}: {message}")]
pub struct RevisionError {
    pub code: &'static str,
    pub message: String,
    pub details: Vec<String>,
}

fn error(code: &'static str, message: impl Into<String>) -> RevisionError {
    RevisionError {
        code,
        message: message.into(),
        details: Vec::new(),
    }
}

fn semantic_error(code: &'static str, issues: crate::domain::SemanticIssues) -> RevisionError {
    RevisionError {
        code,
        message: issues.to_string(),
        details: issues
            .issues
            .into_iter()
            .map(|issue| format!("{} {}: {}", issue.code, issue.path, issue.message))
            .collect(),
    }
}

fn source_error(value: schema::runtime::LoadError) -> RevisionError {
    let mut failure = error("TASK_REVISION_SOURCE_INVALID", value.to_string());
    if let schema::runtime::LoadError::Schema { issues } = value {
        failure.details = issues
            .into_iter()
            .map(|issue| format!("{}: {}", issue.instance_path, issue.message))
            .collect();
    }
    failure
}

struct Context {
    fingerprint: String,
    tasks: Tasks,
    traceability: traceability::TraceabilityReport,
}

/// Reads one existing plan and the revision token binding its governing state.
/// # Errors
/// Returns invalid source, lifecycle, freshness or repository diagnostics.
pub fn preflight(project: &Path, root: &Path, spec: &str) -> Result<String, RevisionError> {
    let context = resolve(project, root, spec)?;
    json(&serde_json::json!({
        "schemaVersion": 1,
        "result": "TASK_REVISION_READY",
        "spec": spec,
        "sourceFingerprint": context.fingerprint,
        "current": context.tasks.as_wire(),
    }))
}

/// Validates and optionally applies one explicit existing-plan replacement.
/// # Errors
/// Rejects stale source, invalid candidate/mapping, unsafe state or failed writes.
pub fn apply(
    project: &Path,
    root: &Path,
    spec: &str,
    input: &str,
    dry_run: bool,
) -> Result<String, RevisionError> {
    let candidate = parse_candidate(input)?;
    let context = resolve(project, root, spec)?;
    ensure_source(&context, &candidate)?;
    let revised = reconcile(&context, &candidate)?;
    let rendered = serde_saphyr::to_string(revised.as_wire())
        .map_err(|value| error("TASK_REVISION_SERIALIZE_FAILED", value.to_string()))?;
    // Recheck all governing inputs after preparing the complete replacement.
    let current = resolve(project, root, spec)?;
    ensure_source(&current, &candidate)?;
    validate_traceability(&current.traceability, &revised)?;
    let output = json(&serde_json::json!({
        "schemaVersion": 1,
        "result": if dry_run { "TASK_REVISION_CHECKED" } else { "TASK_REVISION_APPLIED" },
        "spec": spec,
        "sourceFingerprint": candidate.source_fingerprint,
        "progress": candidate.progress,
        "tasks": revised.as_wire(),
    }))?;
    if !dry_run {
        guarded_fs::replace_existing(
            &root.join(format!("specs/{spec}/tasks.yaml")),
            rendered.as_bytes(),
        )
        .map_err(|value| error("TASK_REVISION_WRITE_FAILED", value.to_string()))?;
    }
    Ok(output)
}

fn ensure_source(context: &Context, candidate: &TaskRevision) -> Result<(), RevisionError> {
    if context.fingerprint != candidate.source_fingerprint {
        return Err(error(
            "TASK_REVISION_SOURCE_CHANGED",
            "preflight source changed; reread it and reassess the complete mapping",
        ));
    }
    Ok(())
}

static VALIDATOR: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    let schema: serde_json::Value = serde_json::from_str(schema::TASK_REVISION_V1_SCHEMA_JSON)
        .expect("embedded Task revision schema is JSON");
    jsonschema::draft202012::options()
        .build(&schema)
        .expect("Task revision schema compiles")
});

fn parse_candidate(input: &str) -> Result<TaskRevision, RevisionError> {
    // Deserialize the original JSON first: duplicate known fields must not be
    // silently collapsed by the intermediate JSON value used for schema checks.
    let candidate: TaskRevision = serde_json::from_str(input)
        .map_err(|value| error("TASK_REVISION_INPUT_INVALID", value.to_string()))?;
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|value| error("TASK_REVISION_INPUT_INVALID", value.to_string()))?;
    VALIDATOR
        .validate(&value)
        .map_err(|value| error("TASK_REVISION_INPUT_INVALID", value.to_string()))?;
    Ok(candidate)
}

fn resolve(project: &Path, root: &Path, spec: &str) -> Result<Context, RevisionError> {
    if !artifacts::canonical_id(spec) {
        return Err(error(
            "TASK_REVISION_TARGET_INVALID",
            "a canonical Spec ID is required",
        ));
    }
    let source = read_regular(root, &format!("specs/{spec}/tasks.yaml"))?;
    let spec_source = read_regular(root, &format!("specs/{spec}/spec.yaml"))?;
    let roadmap_source = read_regular(root, "steering/roadmap.md")?;
    let roadmap = roadmap::parse(&roadmap_source)
        .map_err(|value| error("TASK_REVISION_ROADMAP_INVALID", value.to_string()))?;
    let resolution = artifacts::resolve_spec(root, spec);
    if !resolution.issues.is_empty() {
        return Err(error(
            "TASK_REVISION_SPEC_INVALID",
            format!("{:?}", resolution.issues),
        ));
    }
    let wire = resolution
        .wire
        .ok_or_else(|| error("TASK_REVISION_SPEC_INVALID", "Spec unavailable"))?;
    let active = wire.active_change.0.as_ref().ok_or_else(|| {
        error(
            "TASK_REVISION_STATE_INVALID",
            "an active change is required",
        )
    })?;
    if active.state != WorkflowState::Tasks
        || active
            .gate_evidence
            .as_ref()
            .is_some_and(|gates| gates.tasks.is_some() || gates.completion.is_some())
    {
        return Err(error(
            "TASK_REVISION_STATE_INVALID",
            "revision requires tasks state with Tasks/completion evidence cleared; use the authorized rewind first",
        ));
    }
    if !roadmap.reverse_specs.is_empty()
        || !roadmap.spec_ids().iter().any(|id| id == spec)
        || active.milestone_id.0 != roadmap.milestone_id
    {
        return Err(error(
            "TASK_REVISION_SCOPE_INVALID",
            "target must belong to the active delivery milestone",
        ));
    }
    let inputs = artifacts::resolve_design_gate_inputs(root, spec);
    if !inputs.inventory.issues.is_empty() {
        return Err(error(
            "TASK_REVISION_INPUTS_INVALID",
            format!("{:?}", inputs.inventory.issues),
        ));
    }
    let freshness = freshness::evaluate_wire(&wire, &inputs.inputs);
    if freshness.requirements.status != FreshnessStatus::Fresh
        || freshness.design.status != FreshnessStatus::Fresh
    {
        return Err(error(
            "TASK_REVISION_PRIOR_STALE",
            "Requirements and Design approval must be fresh",
        ));
    }
    cross_spec_review::require_for_boundary(
        project,
        root,
        ReviewBoundary::TasksApproval {
            canonical_spec: spec,
        },
    )
    .map_err(|value| {
        error(
            "TASK_REVISION_REVIEW_REQUIRED",
            format!("{:?}", value.issues),
        )
    })?;
    let old = schema::runtime::load_tasks(&source).map_err(source_error)?;
    let tasks = Tasks::try_from(old)
        .map_err(|value| semantic_error("TASK_REVISION_SOURCE_INVALID", value))?;
    let traceability = current_traceability(root, spec)?;
    let binding = serde_json::to_vec(&(spec, &source, &spec_source, &roadmap_source))
        .map_err(|value| error("TASK_REVISION_SERIALIZE_FAILED", value.to_string()))?;
    let fingerprint = format!("sha256:{}", hex::encode(Sha256::digest(binding)));
    Ok(Context {
        fingerprint,
        tasks,
        traceability,
    })
}

fn current_traceability(
    root: &Path,
    spec: &str,
) -> Result<traceability::TraceabilityReport, RevisionError> {
    let trace = artifacts::resolve_design_traceability(root, spec);
    if !trace.inventory.issues.is_empty() {
        return Err(error(
            "TASK_REVISION_TRACEABILITY_INVALID",
            format!("{:?}", trace.inventory.issues),
        ));
    }
    let traceability = trace.report.ok_or_else(|| {
        error(
            "TASK_REVISION_TRACEABILITY_INVALID",
            "Design traceability unavailable",
        )
    })?;
    if !traceability.issues.is_empty() {
        return Err(error(
            "TASK_REVISION_TRACEABILITY_INVALID",
            format!("{:?}", traceability.issues),
        ));
    }
    Ok(traceability)
}

fn read_regular(root: &Path, relative: &str) -> Result<String, RevisionError> {
    let mut current = root.to_path_buf();
    let parts: Vec<_> = Path::new(relative).components().collect();
    for (index, part) in parts.iter().enumerate() {
        current.push(part.as_os_str());
        let metadata = fs::symlink_metadata(&current)
            .map_err(|value| error("TASK_REVISION_READ_FAILED", format!("{relative}: {value}")))?;
        let valid = if index + 1 == parts.len() {
            guarded_fs::is_regular_file(&metadata)
        } else {
            guarded_fs::is_regular_dir(&metadata)
        };
        if !valid {
            return Err(error(
                "TASK_REVISION_TARGET_INVALID",
                format!("{relative}: regular non-link paths required"),
            ));
        }
    }
    fs::read_to_string(current)
        .map_err(|value| error("TASK_REVISION_READ_FAILED", value.to_string()))
}

fn reconcile(context: &Context, candidate: &TaskRevision) -> Result<Tasks, RevisionError> {
    let mut wire = TasksDocument {
        schema_version: SchemaVersion(1),
        plan: candidate.plan.clone(),
        execution: None,
    };
    let plan = Tasks::try_from(wire.clone())
        .map_err(|value| semantic_error("TASK_REVISION_PLAN_INVALID", value))?;
    let targets: BTreeSet<_> = task_requirements(&plan)
        .into_iter()
        .map(|task| task.task_id)
        .collect();
    let previous = context
        .tasks
        .as_wire()
        .execution
        .as_ref()
        .map(|execution| &execution.tasks.0);
    let mut seen = BTreeSet::new();
    let mut destinations = BTreeSet::new();
    let mut states = BTreeMap::new();
    for disposition in &candidate.progress {
        let from = disposition.source();
        let Some(state) = previous.and_then(|entries| entries.get(from)) else {
            return Err(error(
                "TASK_REVISION_MAPPING_INVALID",
                format!("{} has no recorded execution", from.0),
            ));
        };
        if !seen.insert(from.clone()) {
            return Err(error(
                "TASK_REVISION_MAPPING_INVALID",
                format!("duplicate source {}", from.0),
            ));
        }
        if let Some(to) = disposition.target()
            && (!targets.contains(&to.0) || !destinations.insert(to.clone()))
        {
            return Err(error(
                "TASK_REVISION_MAPPING_INVALID",
                format!("target {} is missing, a group, or duplicated", to.0),
            ));
        }
        if let Disposition::Preserve { to, .. } = disposition {
            states.insert(to.clone(), state.clone());
        }
    }
    let expected: BTreeSet<TaskReference> = previous
        .into_iter()
        .flat_map(|entries| entries.keys().cloned())
        .collect();
    if seen != expected {
        return Err(error(
            "TASK_REVISION_MAPPING_INCOMPLETE",
            "every recorded execution entry requires exactly one explicit disposition",
        ));
    }
    wire.execution = (!states.is_empty()).then_some(TaskExecution {
        tasks: TaskExecutionStates(states),
    });
    let revised = Tasks::try_from(wire)
        .map_err(|value| semantic_error("TASK_REVISION_PLAN_INVALID", value))?;
    validate_traceability(&context.traceability, &revised)?;
    Ok(revised)
}

fn task_requirements(tasks: &Tasks) -> Vec<TaskRequirementSet> {
    tasks
        .as_wire()
        .plan
        .items
        .iter()
        .flat_map(|item| match item {
            PlanItem::Task(task) => vec![task],
            PlanItem::Group(group) => group.tasks.iter().collect(),
        })
        .map(|task| TaskRequirementSet {
            task_id: task.id.0.clone(),
            requirement_ids: task.requirement_ids.0.clone(),
        })
        .collect()
}

fn validate_traceability(
    current: &traceability::TraceabilityReport,
    tasks: &Tasks,
) -> Result<(), RevisionError> {
    let designs = current
        .designs
        .iter()
        .map(|(selector, ids)| DesignRequirementSet {
            selector: selector.clone(),
            requirement_ids: ids.clone(),
        })
        .collect();
    let report = traceability::evaluate(
        &current.requirement_ids,
        designs,
        current.active_requirement_ids.clone(),
        Some(task_requirements(tasks)),
        true,
    );
    if !report.issues.is_empty() {
        return Err(error(
            "TASK_REVISION_TRACEABILITY_INVALID",
            format!("{:?}", report.issues),
        ));
    }
    Ok(())
}

fn json(value: &serde_json::Value) -> Result<String, RevisionError> {
    serde_json::to_string_pretty(value)
        .map(|value| format!("{value}\n"))
        .map_err(|value| error("TASK_REVISION_SERIALIZE_FAILED", value.to_string()))
}
