//! Typed artifact resolution for gates, status reads, and traceability.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use camino::Utf8PathBuf;
use serde_json::{Map, Value};

use super::discovery::{
    collection_id, discover_spec, issue, split_frontmatter, valid_label, validate_spec_directory,
};
use super::{
    Artifact, ArtifactKind, DiscoveryIssue, GateInputResolution, SpecResolution, TasksResolution,
    TraceabilityResolution,
};
use crate::{
    contract::{self, ContractDocument},
    domain::{self, spec::Spec, tasks::Tasks},
    fingerprint::Fingerprint,
    freshness::CurrentGateInputs,
    instruction, requirements,
    schema::{
        runtime,
        spec::v1::WorkflowState,
        tasks::v1::{ExecutableTask, PlanItem},
    },
    traceability::{self, DesignRequirementSet, TaskRequirementSet},
};

struct ActiveTraceabilityScope {
    requirement_ids: Option<Vec<String>>,
    tasks_required: bool,
}

/// Loads the current validated `tasks.yaml` without scanning unrelated Markdown artifacts.
#[must_use]
pub fn resolve_tasks(specbind_root: &Path, canonical_spec: &str) -> TasksResolution {
    let mut issues = Vec::new();
    if validate_spec_directory(specbind_root, canonical_spec, &mut issues).is_none() {
        return TasksResolution {
            tasks: None,
            issues,
        };
    }
    let tasks = match load_tasks_artifact(specbind_root, canonical_spec, &mut issues) {
        Ok(Some(tasks)) => Some(tasks),
        Ok(None) => {
            issues.push(issue(
                "ARTIFACT_TASKS_MISSING",
                Some(Utf8PathBuf::from(format!(
                    "specs/{canonical_spec}/tasks.yaml"
                ))),
                "tasks.yaml does not exist",
            ));
            None
        }
        Err(()) => None,
    };
    issues.sort();
    issues.dedup();
    TasksResolution { tasks, issues }
}

/// Loads `spec.yaml` while retaining a structurally valid but semantically inconsistent wire model.
#[must_use]
pub fn resolve_spec(specbind_root: &Path, canonical_spec: &str) -> SpecResolution {
    let mut issues = Vec::new();
    if validate_spec_directory(specbind_root, canonical_spec, &mut issues).is_none() {
        return SpecResolution {
            wire: None,
            spec: None,
            issues,
        };
    }
    let relative = Utf8PathBuf::from(format!("specs/{canonical_spec}/spec.yaml"));
    let native_path = specbind_root.join(relative.as_std_path());
    let metadata = match fs::symlink_metadata(&native_path) {
        Ok(metadata) => metadata,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_SPEC_READ_FAILED",
                Some(relative),
                format!("cannot inspect spec.yaml: {error}"),
            ));
            return SpecResolution {
                wire: None,
                spec: None,
                issues,
            };
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        issues.push(issue(
            "ARTIFACT_SPEC_NOT_REGULAR",
            Some(relative),
            "spec.yaml must be a regular non-symlink file",
        ));
        return SpecResolution {
            wire: None,
            spec: None,
            issues,
        };
    }
    let input = match fs::read_to_string(&native_path) {
        Ok(input) => input,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_SPEC_READ_FAILED",
                Some(relative),
                format!("cannot read spec.yaml as UTF-8: {error}"),
            ));
            return SpecResolution {
                wire: None,
                spec: None,
                issues,
            };
        }
    };
    let wire = match runtime::load_spec(&input) {
        Ok(wire) => wire,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_SPEC_STRUCTURAL_INVALID",
                Some(relative),
                error.to_string(),
            ));
            return SpecResolution {
                wire: None,
                spec: None,
                issues,
            };
        }
    };
    let spec = match Spec::try_from(wire.clone()) {
        Ok(spec) => Some(spec),
        Err(error) => {
            for semantic in error.issues {
                issues.push(issue(
                    semantic.code,
                    Some(relative.clone()),
                    semantic.message,
                ));
            }
            None
        }
    };
    issues.sort();
    issues.dedup();
    SpecResolution {
        wire: Some(wire),
        spec,
        issues,
    }
}

/// Re-discovers one spec and resolves the current gate-owned input projections.
#[must_use]
pub fn resolve_gate_inputs(specbind_root: &Path, canonical_spec: &str) -> GateInputResolution {
    let mut inventory = discover_spec(specbind_root, canonical_spec);
    let mut inputs = CurrentGateInputs::default();
    let mut design = BTreeMap::new();

    for artifact in &inventory.artifacts {
        let fingerprint = fingerprint_artifact(specbind_root, artifact, &mut inventory.issues);
        match artifact.kind {
            ArtifactKind::Requirements => inputs.requirements = fingerprint,
            ArtifactKind::Design | ArtifactKind::Contract => {
                if let Some(fingerprint) = fingerprint {
                    design.insert(artifact.selector.clone(), fingerprint);
                }
            }
            ArtifactKind::Brief | ArtifactKind::Research | ArtifactKind::ImplementationNotes => {}
        }
    }
    inputs.design = Some(design);
    inputs.tasks = load_tasks_artifact(specbind_root, canonical_spec, &mut inventory.issues)
        .ok()
        .flatten();
    inputs.task_plan = inputs
        .tasks
        .as_ref()
        .and_then(|tasks| resolve_task_plan(tasks, canonical_spec, &mut inventory.issues));
    inventory.issues.sort();
    inventory.issues.dedup();

    GateInputResolution { inventory, inputs }
}

/// Resolves and checks Requirements, Design mappings, and the current active scope.
#[must_use]
pub fn resolve_traceability(specbind_root: &Path, canonical_spec: &str) -> TraceabilityResolution {
    let mut inventory = discover_spec(specbind_root, canonical_spec);
    let artifacts = inventory.artifacts.clone();
    let mut requirement_ids = None;
    let mut designs = Vec::new();
    let mut design_paths = BTreeMap::new();

    for artifact in &artifacts {
        match artifact.kind {
            ArtifactKind::Requirements => {
                requirement_ids =
                    resolve_requirements_projection(specbind_root, artifact, &mut inventory.issues);
            }
            ArtifactKind::Design => {
                if let Some(ids) =
                    resolve_design_projection(specbind_root, artifact, &mut inventory.issues)
                {
                    design_paths.insert(artifact.selector.clone(), artifact.path.clone());
                    designs.push(DesignRequirementSet {
                        selector: artifact.selector.clone(),
                        requirement_ids: ids,
                    });
                }
            }
            ArtifactKind::Brief
            | ArtifactKind::Research
            | ArtifactKind::Contract
            | ArtifactKind::ImplementationNotes => {}
        }
    }

    let active =
        resolve_active_traceability_scope(specbind_root, canonical_spec, &mut inventory.issues);
    let tasks = load_tasks_artifact(specbind_root, canonical_spec, &mut inventory.issues)
        .ok()
        .flatten()
        .as_ref()
        .map(task_requirement_sets);
    let requirements_unavailable = requirement_ids.is_none();
    let report = requirement_ids
        .zip(active.ok())
        .map(|(requirements, active)| {
            let report = traceability::evaluate(
                &requirements,
                designs,
                active.requirement_ids,
                tasks,
                active.tasks_required,
            );
            let spec_path = Utf8PathBuf::from(format!("specs/{canonical_spec}/spec.yaml"));
            let tasks_path = Utf8PathBuf::from(format!("specs/{canonical_spec}/tasks.yaml"));
            for traceability_issue in &report.issues {
                let path = traceability_issue
                    .source
                    .as_ref()
                    .and_then(|source| design_paths.get(source))
                    .cloned()
                    .or_else(|| {
                        (traceability_issue.code.starts_with("TRACEABILITY_TASK")
                            || traceability_issue
                                .source
                                .as_deref()
                                .is_some_and(|source| source.starts_with("tasks/")))
                        .then(|| tasks_path.clone())
                    })
                    .or_else(|| Some(spec_path.clone()));
                inventory.issues.push(issue(
                    traceability_issue.code,
                    path,
                    traceability_issue.message.clone(),
                ));
            }
            report
        });
    if requirements_unavailable {
        inventory.issues.push(issue(
            "TRACEABILITY_REQUIREMENTS_UNAVAILABLE",
            None,
            "traceability requires one valid discovered Requirements artifact",
        ));
    }
    inventory.issues.sort();
    inventory.issues.dedup();
    TraceabilityResolution { inventory, report }
}

fn resolve_requirements_projection(
    specbind_root: &Path,
    artifact: &Artifact,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<Vec<String>> {
    let (mapping, body) = read_traceability_concept(specbind_root, artifact, issues)?;
    let labels = mapping.get("heading_labels")?.as_object()?;
    let expected = BTreeSet::from(["acceptance_criteria", "requirement"]);
    if mapping.contains_key("artifact_id")
        || labels.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected
    {
        return None;
    }
    let requirement_label = labels.get("requirement")?.as_str()?;
    let acceptance_label = labels.get("acceptance_criteria")?.as_str()?;
    if !valid_label(requirement_label) || !valid_label(acceptance_label) {
        return None;
    }
    match requirements::parse(
        &instruction::mask(&body),
        requirement_label,
        acceptance_label,
    ) {
        Ok(document) => Some(
            document
                .requirement_ids()
                .into_iter()
                .map(str::to_owned)
                .collect(),
        ),
        Err(_) => None,
    }
}

fn resolve_design_projection(
    specbind_root: &Path,
    artifact: &Artifact,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<Vec<String>> {
    let (mapping, _) = read_traceability_concept(specbind_root, artifact, issues)?;
    let ids = mapping
        .get("requirement_ids")?
        .as_array()?
        .iter()
        .map(|id| id.as_str().map(str::to_owned))
        .collect::<Option<Vec<_>>>()?;
    let unique = ids.iter().collect::<BTreeSet<_>>();
    if ids.is_empty()
        || unique.len() != ids.len()
        || ids
            .iter()
            .any(|id| domain::parse_requirement_id(id).is_none())
    {
        return None;
    }
    Some(ids)
}

pub(crate) fn resolve_contract_projection(
    specbind_root: &Path,
    artifact: &Artifact,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<ContractDocument> {
    let (_, body) = read_traceability_concept(specbind_root, artifact, issues)?;
    contract::parse(&instruction::mask(&body)).ok()
}

fn read_traceability_concept(
    specbind_root: &Path,
    artifact: &Artifact,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<(Map<String, Value>, String)> {
    let native_path = specbind_root.join(artifact.path.as_std_path());
    let metadata = match fs::symlink_metadata(&native_path) {
        Ok(metadata) => metadata,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_CHANGED_DURING_TRACEABILITY",
                Some(artifact.path.clone()),
                format!("cannot reinspect artifact for traceability: {error}"),
            ));
            return None;
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        issues.push(issue(
            "ARTIFACT_CHANGED_DURING_TRACEABILITY",
            Some(artifact.path.clone()),
            "artifact is no longer a regular non-symlink file",
        ));
        return None;
    }
    let content = match fs::read_to_string(&native_path) {
        Ok(content) => content,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_CHANGED_DURING_TRACEABILITY",
                Some(artifact.path.clone()),
                format!("cannot reread artifact for traceability: {error}"),
            ));
            return None;
        }
    };
    let (frontmatter, body) = match split_frontmatter(&content) {
        Ok(parts) => parts,
        Err(message) => {
            issues.push(issue(
                "ARTIFACT_CHANGED_DURING_TRACEABILITY",
                Some(artifact.path.clone()),
                message,
            ));
            return None;
        }
    };
    let value = match serde_saphyr::from_str::<Value>(frontmatter) {
        Ok(value) => value,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_CHANGED_DURING_TRACEABILITY",
                Some(artifact.path.clone()),
                format!("artifact Front Matter changed during traceability: {error}"),
            ));
            return None;
        }
    };
    let Some(mapping) = value.as_object() else {
        issues.push(issue(
            "ARTIFACT_CHANGED_DURING_TRACEABILITY",
            Some(artifact.path.clone()),
            "artifact Front Matter is no longer a mapping",
        ));
        return None;
    };
    let current_type = mapping.get("type").and_then(Value::as_str);
    let current_id = collection_id(artifact.kind, mapping);
    if current_type != Some(artifact.artifact_type.as_str())
        || current_id != artifact.artifact_id.as_deref()
    {
        issues.push(issue(
            "ARTIFACT_CHANGED_DURING_TRACEABILITY",
            Some(artifact.path.clone()),
            "artifact logical identity changed during traceability resolution",
        ));
        return None;
    }
    Some((mapping.clone(), body.to_owned()))
}

fn resolve_active_traceability_scope(
    specbind_root: &Path,
    canonical_spec: &str,
    issues: &mut Vec<DiscoveryIssue>,
) -> Result<ActiveTraceabilityScope, ()> {
    let relative = Utf8PathBuf::from(format!("specs/{canonical_spec}/spec.yaml"));
    let native_path = specbind_root.join(relative.as_std_path());
    let metadata = match fs::symlink_metadata(&native_path) {
        Ok(metadata) => metadata,
        Err(error) => {
            issues.push(issue(
                "TRACEABILITY_SPEC_UNAVAILABLE",
                Some(relative),
                format!("cannot inspect spec.yaml: {error}"),
            ));
            return Err(());
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        issues.push(issue(
            "TRACEABILITY_SPEC_UNAVAILABLE",
            Some(relative),
            "spec.yaml must be a regular non-symlink file",
        ));
        return Err(());
    }
    let input = match fs::read_to_string(&native_path) {
        Ok(input) => input,
        Err(error) => {
            issues.push(issue(
                "TRACEABILITY_SPEC_UNAVAILABLE",
                Some(relative),
                format!("cannot read spec.yaml as UTF-8: {error}"),
            ));
            return Err(());
        }
    };
    let wire = match runtime::load_spec(&input) {
        Ok(wire) => wire,
        Err(error) => {
            issues.push(issue(
                "TRACEABILITY_SPEC_STRUCTURAL_INVALID",
                Some(relative),
                error.to_string(),
            ));
            return Err(());
        }
    };
    let spec = match Spec::try_from(wire) {
        Ok(spec) => spec,
        Err(error) => {
            for semantic in error.issues {
                issues.push(issue(
                    semantic.code,
                    Some(relative.clone()),
                    semantic.message,
                ));
            }
            return Err(());
        }
    };
    let active = spec.as_wire().active_change.0.as_ref();
    let requirement_ids = active
        .and_then(|active| active.requirement_ids.0.clone())
        .map(|ids| ids.0);
    let tasks_required = active.is_some_and(|active| {
        matches!(
            active.state,
            WorkflowState::Implementation | WorkflowState::ReleaseReady
        )
    });
    Ok(ActiveTraceabilityScope {
        requirement_ids,
        tasks_required,
    })
}

fn fingerprint_artifact(
    specbind_root: &Path,
    artifact: &Artifact,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<Fingerprint> {
    let native_path = specbind_root.join(artifact.path.as_std_path());
    let metadata = match fs::symlink_metadata(&native_path) {
        Ok(metadata) => metadata,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_CHANGED_DURING_RESOLUTION",
                Some(artifact.path.clone()),
                format!("artifact changed during fingerprint resolution: {error}"),
            ));
            return None;
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        issues.push(issue(
            "ARTIFACT_CHANGED_DURING_RESOLUTION",
            Some(artifact.path.clone()),
            "artifact is no longer a regular non-symlink file",
        ));
        return None;
    }
    match fs::read(&native_path) {
        Ok(bytes) => Some(Fingerprint::markdown(&bytes)),
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_READ_FAILED",
                Some(artifact.path.clone()),
                format!("cannot read artifact for fingerprinting: {error}"),
            ));
            None
        }
    }
}

fn resolve_task_plan(
    tasks: &Tasks,
    canonical_spec: &str,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<Fingerprint> {
    let relative = Utf8PathBuf::from(format!("specs/{canonical_spec}/tasks.yaml"));
    match Fingerprint::task_plan(tasks) {
        Ok(fingerprint) => Some(fingerprint),
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASK_PLAN_FINGERPRINT_FAILED",
                Some(relative),
                format!("cannot canonicalize task plan: {error}"),
            ));
            None
        }
    }
}

fn load_tasks_artifact(
    specbind_root: &Path,
    canonical_spec: &str,
    issues: &mut Vec<DiscoveryIssue>,
) -> Result<Option<Tasks>, ()> {
    let relative = Utf8PathBuf::from(format!("specs/{canonical_spec}/tasks.yaml"));
    let native_path = specbind_root.join(relative.as_std_path());
    let metadata = match fs::symlink_metadata(&native_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASKS_READ_FAILED",
                Some(relative),
                format!("cannot inspect tasks.yaml: {error}"),
            ));
            return Err(());
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        issues.push(issue(
            "ARTIFACT_TASKS_NOT_REGULAR",
            Some(relative),
            "tasks.yaml must be a regular non-symlink file",
        ));
        return Err(());
    }
    let input = match fs::read_to_string(&native_path) {
        Ok(input) => input,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASKS_READ_FAILED",
                Some(relative),
                format!("cannot read tasks.yaml as UTF-8: {error}"),
            ));
            return Err(());
        }
    };
    let wire = match runtime::load_tasks(&input) {
        Ok(wire) => wire,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASKS_STRUCTURAL_INVALID",
                Some(relative),
                error.to_string(),
            ));
            return Err(());
        }
    };
    let tasks = match Tasks::try_from(wire) {
        Ok(tasks) => tasks,
        Err(error) => {
            for semantic in error.issues {
                issues.push(issue(
                    semantic.code,
                    Some(relative.clone()),
                    semantic.message,
                ));
            }
            return Err(());
        }
    };
    Ok(Some(tasks))
}

fn task_requirement_sets(tasks: &Tasks) -> Vec<TaskRequirementSet> {
    tasks
        .as_wire()
        .plan
        .items
        .iter()
        .flat_map(|item| match item {
            PlanItem::Task(task) => vec![task_requirement_set(task)],
            PlanItem::Group(group) => group.tasks.iter().map(task_requirement_set).collect(),
        })
        .collect()
}

fn task_requirement_set(task: &ExecutableTask) -> TaskRequirementSet {
    match task {
        ExecutableTask::Parallel(task) => TaskRequirementSet {
            task_id: task.id.0.clone(),
            requirement_ids: task.requirement_ids.0.clone(),
        },
        ExecutableTask::Sequential(task) => TaskRequirementSet {
            task_id: task.id.0.clone(),
            requirement_ids: task.requirement_ids.0.clone(),
        },
    }
}
