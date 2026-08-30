//! Read model for gate-local freshness with prerequisite cascading.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::{
    domain::{SemanticIssue, spec::Spec, tasks::Tasks},
    fingerprint::Fingerprint,
    repository,
    roadmap::{self, ReleaseBindingEdit},
    schema::{
        runtime,
        spec::v1 as wire,
        tasks::v1::{ExecutableTask, PlanItem, TaskExecutionState},
    },
};

/// Current gate inputs resolved from authoritative project artifact discovery.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CurrentGateInputs {
    pub requirements: Option<Fingerprint>,
    pub design: Option<BTreeMap<String, Fingerprint>>,
    pub task_plan: Option<Fingerprint>,
    pub tasks: Option<Tasks>,
    pub completion_revision: Option<CompletionRevisionAssessment>,
}

/// Result of comparing accepted completion evidence with the current Git checkout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRevisionAssessment {
    pub issues: Vec<SemanticIssue>,
}

#[derive(Debug, Clone, Copy)]
struct CompletionPathPolicy {
    allow_release_binding: bool,
    code: &'static str,
    message: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessStatus {
    NotReached,
    Fresh,
    Stale,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateFreshness {
    pub status: FreshnessStatus,
    pub issues: Vec<SemanticIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactFreshnessReport {
    pub requirements: GateFreshness,
    pub design: GateFreshness,
    pub tasks: GateFreshness,
    pub completion: GateFreshness,
}

/// Compares current gate-owned input projections with persisted approval evidence.
#[must_use]
pub fn evaluate(spec: &Spec, current: &CurrentGateInputs) -> ArtifactFreshnessReport {
    evaluate_wire(spec.as_wire(), current)
}

/// Compares current inputs with the evidence in a structurally valid wire document.
///
/// This preserves gate diagnostics for `spec status` even when artifact-local semantic
/// contradictions prevent conversion to the validated domain wrapper.
#[must_use]
pub fn evaluate_wire(
    spec: &wire::SpecDocument,
    current: &CurrentGateInputs,
) -> ArtifactFreshnessReport {
    let active = spec.active_change.0.as_ref();
    let evidence = active.and_then(|active| active.gate_evidence.as_ref());

    let requirements = evaluate_requirements(evidence, current.requirements);
    let design = evaluate_design(evidence, current.design.as_ref(), &requirements);
    let tasks = evaluate_tasks(evidence, current.task_plan, &design);
    let completion = evaluate_completion(
        evidence,
        current.tasks.as_ref(),
        current.completion_revision.as_ref(),
        &tasks,
    );

    ArtifactFreshnessReport {
        requirements,
        design,
        tasks,
        completion,
    }
}

/// Validates the accepted implementation revision against the current checkout.
///
/// A successor checkout is accepted only when its complete tracked difference from the
/// implementation revision consists of recognized evidence-preserving metadata transitions.
#[must_use]
pub fn assess_completion_revision(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    spec: &Spec,
) -> CompletionRevisionAssessment {
    let mut issues = Vec::new();
    let Some(completion) = spec
        .as_wire()
        .active_change
        .0
        .as_ref()
        .and_then(|active| active.gate_evidence.as_ref())
        .and_then(|evidence| evidence.completion.as_ref())
    else {
        return CompletionRevisionAssessment { issues };
    };
    let revision = completion.implementation_revision.0.as_str();
    let relative_spec = match completion_spec_path(project_root, specbind_root, canonical_spec) {
        Ok(path) => path,
        Err(issue) => {
            issues.push(issue);
            return CompletionRevisionAssessment { issues };
        }
    };
    validate_current_spec(specbind_root, canonical_spec, spec, &mut issues);
    if !issues.is_empty() {
        return completion_assessment(issues);
    }

    match git_output(
        project_root,
        &["rev-parse", "--verify", &format!("{revision}^{{commit}}")],
    ) {
        Ok(value) if value.trim() == revision => {}
        Ok(_) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_REVISION_NOT_EXACT",
            "/completion/implementation_revision",
            "implementation_revision must resolve to the same full commit object ID",
        )),
        Err(message) => {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_REVISION_MISSING",
                "/completion/implementation_revision",
                message,
            ));
            return completion_assessment(issues);
        }
    }
    match git_status(
        project_root,
        &["merge-base", "--is-ancestor", revision, "HEAD"],
    ) {
        Ok(true) => {}
        Ok(false) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_REVISION_NOT_ANCESTOR",
            "/completion/implementation_revision",
            "implementation_revision is not an ancestor of current HEAD",
        )),
        Err(message) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_GIT_FAILED",
            "/completion/implementation_revision",
            message,
        )),
    }
    if !issues.is_empty() {
        return completion_assessment(issues);
    }

    validate_commit_history(project_root, specbind_root, revision, &mut issues);
    let changed =
        match git_output_bytes(project_root, &["diff", "--name-only", "-z", revision, "--"]) {
            Ok(output) => nul_paths(&output),
            Err(message) => {
                issues.push(freshness_issue(
                    "FRESHNESS_COMPLETION_GIT_FAILED",
                    "/completion/implementation_revision",
                    message,
                ));
                return completion_assessment(issues);
            }
        };
    match changed {
        Ok(paths) => validate_completion_paths(
            project_root,
            specbind_root,
            revision,
            &paths,
            Some(&relative_spec),
            CompletionPathPolicy {
                allow_release_binding: true,
                code: "FRESHNESS_COMPLETION_PROJECT_CHANGED",
                message: "project content since implementation_revision is not limited to recognized evidence-preserving metadata transitions",
            },
            &mut issues,
        ),
        Err(message) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_GIT_OUTPUT_INVALID",
            "/completion/implementation_revision",
            message,
        )),
    }
    validate_worktree_status(project_root, specbind_root, revision, true, &mut issues);
    completion_assessment(issues)
}

/// Validates that every current worktree change is a SpecBind-owned completion
/// metadata transition bound to the supplied implementation revision.
///
/// This is the guarded-acceptance exception that permits several participating
/// Specs validated at one clean revision to be accepted before their metadata
/// mutations are committed together.
#[must_use]
pub fn assess_pending_completion_mutations(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
) -> CompletionRevisionAssessment {
    let mut issues = Vec::new();
    validate_worktree_status(project_root, specbind_root, revision, false, &mut issues);
    completion_assessment(issues)
}

fn evaluate_requirements(
    evidence: Option<&wire::GateEvidence>,
    current: Option<Fingerprint>,
) -> GateFreshness {
    let Some(expected) = evidence
        .and_then(|value| value.requirements.as_ref())
        .map(requirements_fingerprint)
    else {
        return not_reached();
    };

    let issues = match current {
        None => vec![freshness_issue(
            "FRESHNESS_REQUIREMENTS_MISSING",
            "/requirements",
            "current requirements artifact is missing",
        )],
        Some(current) if !current.matches_wire(expected) => vec![freshness_issue(
            "FRESHNESS_REQUIREMENTS_CHANGED",
            "/requirements",
            "current requirements fingerprint differs from approved evidence",
        )],
        Some(_) => vec![],
    };
    reached(issues)
}

fn evaluate_design(
    evidence: Option<&wire::GateEvidence>,
    current: Option<&BTreeMap<String, Fingerprint>>,
    prerequisite: &GateFreshness,
) -> GateFreshness {
    let Some(expected) = evidence.and_then(|value| value.design.as_ref()) else {
        return not_reached();
    };
    let expected = design_fingerprints(expected);
    let mut issues = prerequisite_issue("design", prerequisite);

    match current {
        None => issues.push(freshness_issue(
            "FRESHNESS_DESIGN_INPUTS_MISSING",
            "/design",
            "current contract and design artifact set is missing",
        )),
        Some(current) => {
            for key in expected.keys().filter(|key| !current.contains_key(*key)) {
                issues.push(freshness_issue(
                    "FRESHNESS_DESIGN_INPUT_MISSING",
                    format!("/design/{key}"),
                    format!("current design input {key} is missing"),
                ));
            }
            for key in current.keys().filter(|key| !expected.contains_key(*key)) {
                issues.push(freshness_issue(
                    "FRESHNESS_DESIGN_INPUT_ADDED",
                    format!("/design/{key}"),
                    format!("current design input {key} was not approved"),
                ));
            }
            for (key, fingerprint) in current {
                if let Some(expected) = expected.get(key)
                    && !fingerprint.matches_wire(expected)
                {
                    issues.push(freshness_issue(
                        "FRESHNESS_DESIGN_INPUT_CHANGED",
                        format!("/design/{key}"),
                        format!("current design input {key} differs from approved evidence"),
                    ));
                }
            }
        }
    }
    reached(issues)
}

fn evaluate_tasks(
    evidence: Option<&wire::GateEvidence>,
    current: Option<Fingerprint>,
    prerequisite: &GateFreshness,
) -> GateFreshness {
    let Some(expected) = evidence
        .and_then(|value| value.tasks.as_ref())
        .map(tasks_fingerprint)
    else {
        return not_reached();
    };
    let mut issues = prerequisite_issue("tasks", prerequisite);
    match current {
        None => issues.push(freshness_issue(
            "FRESHNESS_TASK_PLAN_MISSING",
            "/tasks.yaml#plan",
            "current task plan is missing",
        )),
        Some(current) if !current.matches_wire(expected) => issues.push(freshness_issue(
            "FRESHNESS_TASK_PLAN_CHANGED",
            "/tasks.yaml#plan",
            "current task plan differs from approved evidence",
        )),
        Some(_) => {}
    }
    reached(issues)
}

fn evaluate_completion(
    evidence: Option<&wire::GateEvidence>,
    tasks: Option<&Tasks>,
    revision: Option<&CompletionRevisionAssessment>,
    prerequisite: &GateFreshness,
) -> GateFreshness {
    if evidence
        .and_then(|value| value.completion.as_ref())
        .is_none()
    {
        return not_reached();
    }
    let mut issues = prerequisite_issue("completion", prerequisite);
    match tasks {
        None => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_TASKS_MISSING",
            "/tasks.yaml#execution",
            "completion freshness requires the current validated tasks artifact",
        )),
        Some(tasks) => validate_all_tasks_completed(tasks, &mut issues),
    }
    match revision {
        Some(revision) => issues.extend(revision.issues.iter().cloned()),
        None => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_REVISION_UNRESOLVED",
            "/completion/implementation_revision",
            "completion freshness requires the current Git revision assessment",
        )),
    }
    reached(issues)
}

fn validate_all_tasks_completed(tasks: &Tasks, issues: &mut Vec<SemanticIssue>) {
    let execution = tasks.as_wire().execution.as_ref();
    for task in tasks.as_wire().plan.items.iter().flat_map(plan_tasks) {
        let id = task_id(task);
        let state = execution.and_then(|execution| {
            execution
                .tasks
                .0
                .iter()
                .find(|(reference, _)| reference.0 == id)
                .map(|(_, state)| state)
        });
        match state {
            Some(TaskExecutionState::Completed(_)) => {}
            Some(TaskExecutionState::Blocked(_)) => issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_TASK_BLOCKED",
                format!("/tasks.yaml#execution/{id}"),
                format!("task {id} is blocked"),
            )),
            None => issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_TASK_INCOMPLETE",
                format!("/tasks.yaml#execution/{id}"),
                format!("task {id} is not completed"),
            )),
        }
    }
}

fn plan_tasks(item: &PlanItem) -> Vec<&ExecutableTask> {
    match item {
        PlanItem::Task(task) => vec![task],
        PlanItem::Group(group) => group.tasks.iter().collect(),
    }
}

fn task_id(task: &ExecutableTask) -> &str {
    &task.id.0
}

fn validate_worktree_status(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    allow_release_binding: bool,
    issues: &mut Vec<SemanticIssue>,
) {
    let output = match git_output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    ) {
        Ok(output) => output,
        Err(message) => {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_GIT_FAILED",
                "/completion/implementation_revision",
                message,
            ));
            return;
        }
    };
    let mut paths = Vec::new();
    for record in output
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        let accepted = record.len() > 3
            && record[0..2]
                .iter()
                .all(|byte| *byte == b' ' || *byte == b'M')
            && std::str::from_utf8(&record[3..])
                .map(|path| paths.push(path.to_owned()))
                .is_ok();
        if !accepted {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_WORKTREE_DIRTY",
                "/completion/implementation_revision",
                "working tree contains changes other than the expected completion metadata mutation",
            ));
            break;
        }
    }
    if issues.is_empty() {
        validate_completion_paths(
            project_root,
            specbind_root,
            revision,
            &paths,
            None,
            CompletionPathPolicy {
                allow_release_binding,
                code: "FRESHNESS_COMPLETION_WORKTREE_DIRTY",
                message: "working tree contains changes other than recognized evidence-preserving metadata transitions",
            },
            issues,
        );
    }
}

fn validate_commit_history(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    issues: &mut Vec<SemanticIssue>,
) {
    let range = format!("{revision}..HEAD");
    let output = match git_output_bytes(
        project_root,
        &["log", "--format=", "--name-only", "-z", &range, "--"],
    ) {
        Ok(output) => output,
        Err(message) => {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_GIT_FAILED",
                "/completion/implementation_revision",
                message,
            ));
            return;
        }
    };
    match nul_paths(&output) {
        Ok(paths) => validate_completion_paths(
            project_root,
            specbind_root,
            revision,
            &paths,
            None,
            CompletionPathPolicy {
                allow_release_binding: true,
                code: "FRESHNESS_COMPLETION_PROJECT_CHANGED",
                message: "commit history since implementation_revision contains a change other than recognized evidence-preserving metadata transitions",
            },
            issues,
        ),
        Err(message) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_GIT_OUTPUT_INVALID",
            "/completion/implementation_revision",
            message,
        )),
    }
}

fn validate_current_spec(
    specbind_root: &Path,
    canonical_spec: &str,
    expected: &Spec,
    issues: &mut Vec<SemanticIssue>,
) {
    let path = specbind_root
        .join("specs")
        .join(canonical_spec)
        .join("spec.yaml");
    if !fs::symlink_metadata(&path)
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
    {
        issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_CURRENT_SPEC_INVALID",
            "/completion/implementation_revision",
            "current spec.yaml must be a regular non-symlink file",
        ));
        return;
    }
    let current = fs::read_to_string(path)
        .map_err(|error| error.to_string())
        .and_then(|input| runtime::load_spec(&input).map_err(|error| error.to_string()))
        .and_then(|wire| Spec::try_from(wire).map_err(|error| error.to_string()));
    if !current.is_ok_and(|current| &current == expected) {
        issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_CURRENT_SPEC_CHANGED",
            "/completion/implementation_revision",
            "current spec.yaml differs from the completion freshness input",
        ));
    }
}

fn validate_completion_paths(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    paths: &[String],
    required_path: Option<&str>,
    policy: CompletionPathPolicy,
    issues: &mut Vec<SemanticIssue>,
) {
    let paths = paths.iter().collect::<BTreeSet<_>>();
    if required_path.is_some_and(|required| !paths.iter().any(|path| path.as_str() == required)) {
        issues.push(freshness_issue(
            policy.code,
            "/completion/implementation_revision",
            policy.message,
        ));
        return;
    }
    for relative_spec in paths {
        let completion_transition = validate_completion_transition_path(
            project_root,
            specbind_root,
            revision,
            relative_spec,
        );
        let release_binding = policy.allow_release_binding
            && validate_release_binding_transition_path(
                project_root,
                specbind_root,
                revision,
                relative_spec,
            );
        if !completion_transition && !release_binding {
            issues.push(freshness_issue(
                policy.code,
                "/completion/implementation_revision",
                policy.message,
            ));
            return;
        }
    }
}

fn validate_completion_transition_path(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    relative_spec: &str,
) -> bool {
    let Some(_canonical_spec) = completion_path_spec_id(project_root, specbind_root, relative_spec)
    else {
        return false;
    };
    let current = fs::read_to_string(project_root.join(relative_spec))
        .map_err(|error| error.to_string())
        .and_then(|input| runtime::load_spec(&input).map_err(|error| error.to_string()))
        .and_then(|wire| Spec::try_from(wire).map_err(|error| error.to_string()));
    let Ok(current) = current else {
        return false;
    };
    let current_revision = current
        .as_wire()
        .active_change
        .0
        .as_ref()
        .and_then(|active| active.gate_evidence.as_ref())
        .and_then(|evidence| evidence.completion.as_ref())
        .map(|completion| completion.implementation_revision.0.as_str());
    if current_revision != Some(revision) {
        return false;
    }
    let Ok(baseline) = git_output_bytes(
        project_root,
        &["show", &format!("{revision}:{relative_spec}")],
    ) else {
        return false;
    };
    let baseline = std::str::from_utf8(&baseline)
        .map_err(|error| error.to_string())
        .and_then(|input| runtime::load_spec(input).map_err(|error| error.to_string()))
        .and_then(|wire| Spec::try_from(wire).map_err(|error| error.to_string()));
    let Ok(baseline) = baseline else {
        return false;
    };
    let mut expected = current.as_wire().clone();
    let Some(active) = expected.active_change.0.as_mut() else {
        return false;
    };
    active.state = wire::WorkflowState::Implementation;
    if let Some(evidence) = active.gate_evidence.as_mut() {
        evidence.completion = None;
    }
    baseline.as_wire() == &expected
}

fn validate_release_binding_transition_path(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    relative_path: &str,
) -> bool {
    if release_binding_path(project_root, specbind_root).as_deref() != Some(relative_path) {
        return false;
    }
    let current_path = project_root.join(relative_path);
    if !fs::symlink_metadata(&current_path)
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
    {
        return false;
    }
    let Ok(current) = fs::read_to_string(&current_path) else {
        return false;
    };
    let Ok(current_roadmap) = roadmap::parse(&current) else {
        return false;
    };
    let Some(current_release) = current_roadmap.target_release.as_deref() else {
        return false;
    };
    let Ok(baseline) = git_output_bytes(
        project_root,
        &["show", &format!("{revision}:{relative_path}")],
    ) else {
        return false;
    };
    let Ok(baseline) = std::str::from_utf8(&baseline) else {
        return false;
    };
    let Ok(baseline_roadmap) = roadmap::parse(baseline) else {
        return false;
    };
    if baseline_roadmap.target_release.as_deref() == Some(current_release) {
        return false;
    }
    matches!(
        roadmap::bind_release(
            baseline,
            current_release,
            baseline_roadmap.target_release.is_some(),
        ),
        Ok(ReleaseBindingEdit::Updated(expected)) if expected == current
    )
}

fn release_binding_path(project_root: &Path, specbind_root: &Path) -> Option<String> {
    let relative = specbind_root.strip_prefix(project_root).ok()?;
    Some(
        relative
            .join("steering")
            .join("roadmap.md")
            .to_string_lossy()
            .replace('\\', "/"),
    )
}

fn completion_path_spec_id<'a>(
    project_root: &Path,
    specbind_root: &Path,
    relative_spec: &'a str,
) -> Option<&'a str> {
    let root = specbind_root
        .strip_prefix(project_root)
        .ok()?
        .to_string_lossy()
        .replace('\\', "/");
    let prefix = if root.is_empty() {
        "specs/".to_owned()
    } else {
        format!("{root}/specs/")
    };
    let canonical_spec = relative_spec
        .strip_prefix(&prefix)?
        .strip_suffix("/spec.yaml")?;
    (!canonical_spec.contains('/') && valid_id(canonical_spec)).then_some(canonical_spec)
}

fn nul_paths(output: &[u8]) -> Result<Vec<String>, String> {
    output
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| {
            std::str::from_utf8(path)
                .map(str::to_owned)
                .map_err(|error| error.to_string())
        })
        .collect()
}

fn git_output(project_root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = git_output_bytes(project_root, arguments)?;
    String::from_utf8(output).map_err(|error| format!("Git output is not UTF-8: {error}"))
}

fn git_output_bytes(project_root: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    repository::output_bytes(project_root, arguments).map_err(|error| error.to_string())
}

fn git_status(project_root: &Path, arguments: &[&str]) -> Result<bool, String> {
    repository::predicate(project_root, arguments).map_err(|error| error.to_string())
}

fn completion_assessment(mut issues: Vec<SemanticIssue>) -> CompletionRevisionAssessment {
    issues.sort();
    issues.dedup();
    CompletionRevisionAssessment { issues }
}

fn completion_spec_path(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
) -> Result<String, SemanticIssue> {
    if !valid_id(canonical_spec) {
        return Err(freshness_issue(
            "FRESHNESS_COMPLETION_SPEC_ID_INVALID",
            "/completion/implementation_revision",
            "canonical spec ID is invalid",
        ));
    }
    let relative = specbind_root.strip_prefix(project_root).map_err(|error| {
        freshness_issue(
            "FRESHNESS_COMPLETION_PROJECT_ROOT_INVALID",
            "/completion/implementation_revision",
            format!("SpecBind root is not below the project root: {error}"),
        )
    })?;
    Ok(relative
        .join("specs")
        .join(canonical_spec)
        .join("spec.yaml")
        .to_string_lossy()
        .replace('\\', "/"))
}

fn valid_id(value: &str) -> bool {
    crate::artifacts::canonical_id(value)
}

fn requirements_fingerprint(evidence: &wire::RequirementsGateEvidence) -> &wire::Fingerprint {
    match evidence {
        wire::RequirementsGateEvidence::Explicit(value) => &value.input_revisions.requirements,
        wire::RequirementsGateEvidence::Delegated(value) => &value.input_revisions.requirements,
    }
}

fn design_fingerprints(
    evidence: &wire::DesignGateEvidence,
) -> &BTreeMap<String, wire::Fingerprint> {
    match evidence {
        wire::DesignGateEvidence::Explicit(value) => &value.input_revisions.0,
        wire::DesignGateEvidence::Delegated(value) => &value.input_revisions.0,
    }
}

fn tasks_fingerprint(evidence: &wire::TasksGateEvidence) -> &wire::Fingerprint {
    match evidence {
        wire::TasksGateEvidence::Explicit(value) => &value.input_revisions.plan,
        wire::TasksGateEvidence::Delegated(value) => &value.input_revisions.plan,
    }
}

fn prerequisite_issue(gate: &str, prerequisite: &GateFreshness) -> Vec<SemanticIssue> {
    if prerequisite.status == FreshnessStatus::Fresh {
        vec![]
    } else {
        vec![freshness_issue(
            "FRESHNESS_PREREQUISITE_STALE",
            format!("/{gate}"),
            format!("{gate} freshness requires its prerequisite gate to be fresh"),
        )]
    }
}

fn not_reached() -> GateFreshness {
    GateFreshness {
        status: FreshnessStatus::NotReached,
        issues: vec![],
    }
}

fn reached(mut issues: Vec<SemanticIssue>) -> GateFreshness {
    issues.sort();
    issues.dedup();
    GateFreshness {
        status: if issues.is_empty() {
            FreshnessStatus::Fresh
        } else {
            FreshnessStatus::Stale
        },
        issues,
    }
}

fn freshness_issue(
    code: &'static str,
    path: impl Into<String>,
    message: impl Into<String>,
) -> SemanticIssue {
    SemanticIssue {
        code,
        path: path.into(),
        message: message.into(),
    }
}
