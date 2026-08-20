//! Cross-document Requirement traceability over one Spec's current artifacts.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignRequirementSet {
    pub selector: String,
    pub requirement_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskRequirementSet {
    pub task_id: String,
    pub requirement_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraceabilityIssue {
    pub code: &'static str,
    pub source: Option<String>,
    pub requirement_id: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityReport {
    pub requirement_ids: Vec<String>,
    pub active_requirement_ids: Option<Vec<String>>,
    pub design_requirement_ids: Vec<String>,
    pub designs: BTreeMap<String, Vec<String>>,
    pub task_requirement_ids: Vec<String>,
    pub tasks: Option<BTreeMap<String, Vec<String>>>,
    pub tasks_required: bool,
    pub issues: Vec<TraceabilityIssue>,
}

/// Compares the Requirements catalog, Design mappings, Tasks mappings, and active scope.
#[must_use]
pub fn evaluate(
    requirement_ids: &[String],
    designs: Vec<DesignRequirementSet>,
    active_requirement_ids: Option<Vec<String>>,
    tasks: Option<Vec<TaskRequirementSet>>,
    tasks_required: bool,
) -> TraceabilityReport {
    let requirements = requirement_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut design_map = BTreeMap::new();
    let mut design_union = BTreeSet::new();
    let mut issues = Vec::new();

    for design in designs {
        let ids = design.requirement_ids.into_iter().collect::<BTreeSet<_>>();
        for id in ids.difference(&requirements) {
            issues.push(TraceabilityIssue {
                code: "TRACEABILITY_DESIGN_REQUIREMENT_UNKNOWN",
                source: Some(design.selector.clone()),
                requirement_id: Some(id.clone()),
                message: format!(
                    "{} references Requirement ID {id}, which does not exist in Requirements",
                    design.selector
                ),
            });
        }
        design_union.extend(ids.iter().cloned());
        design_map.insert(design.selector, numeric_ids(ids));
    }

    if let Some(active) = &active_requirement_ids {
        for id in active {
            if !requirements.contains(id) {
                issues.push(TraceabilityIssue {
                    code: "TRACEABILITY_ACTIVE_REQUIREMENT_UNKNOWN",
                    source: None,
                    requirement_id: Some(id.clone()),
                    message: format!("active Requirement ID {id} does not exist in Requirements"),
                });
            } else if !design_union.contains(id) {
                issues.push(TraceabilityIssue {
                    code: "TRACEABILITY_DESIGN_COVERAGE_MISSING",
                    source: None,
                    requirement_id: Some(id.clone()),
                    message: format!(
                        "active Requirement ID {id} is not covered by any Design artifact"
                    ),
                });
            }
        }
    }

    let (task_map, task_union) = evaluate_tasks(
        tasks,
        &requirements,
        active_requirement_ids.as_deref(),
        &mut issues,
    );
    if tasks_required {
        if task_map.is_none() {
            issues.push(TraceabilityIssue {
                code: "TRACEABILITY_TASKS_UNAVAILABLE",
                source: None,
                requirement_id: None,
                message: "tasks.yaml is required for traceability in the current workflow state"
                    .to_owned(),
            });
        } else if let Some(active) = &active_requirement_ids {
            for id in active {
                if requirements.contains(id) && !task_union.contains(id) {
                    issues.push(TraceabilityIssue {
                        code: "TRACEABILITY_TASK_COVERAGE_MISSING",
                        source: None,
                        requirement_id: Some(id.clone()),
                        message: format!(
                            "active Requirement ID {id} is not covered by any executable Task"
                        ),
                    });
                }
            }
        }
    }

    issues.sort();
    issues.dedup();
    TraceabilityReport {
        requirement_ids: numeric_ids(requirements),
        active_requirement_ids,
        design_requirement_ids: numeric_ids(design_union),
        designs: design_map,
        task_requirement_ids: numeric_ids(task_union),
        tasks: task_map,
        tasks_required,
        issues,
    }
}

fn evaluate_tasks(
    tasks: Option<Vec<TaskRequirementSet>>,
    requirements: &BTreeSet<String>,
    active_requirement_ids: Option<&[String]>,
    issues: &mut Vec<TraceabilityIssue>,
) -> (Option<BTreeMap<String, Vec<String>>>, BTreeSet<String>) {
    let Some(tasks) = tasks else {
        return (None, BTreeSet::new());
    };
    let active = active_requirement_ids.map(|ids| ids.iter().cloned().collect::<BTreeSet<_>>());
    let mut task_map = BTreeMap::new();
    let mut task_union = BTreeSet::new();
    for task in tasks {
        let ids = task.requirement_ids.into_iter().collect::<BTreeSet<_>>();
        // The reverse direction of coverage. A task plan is milestone-local, so
        // a task accountable to no active Requirement is work this change was
        // not asked for, or an active set that is missing one.
        if let Some(active) = &active
            && ids.is_disjoint(active)
        {
            issues.push(TraceabilityIssue {
                code: "TRACEABILITY_TASK_SCOPE_INACTIVE",
                source: Some(format!("tasks/{}", task.task_id)),
                requirement_id: None,
                message: format!(
                    "Task {} references no active Requirement ID, so the plan carries work the active scope does not account for",
                    task.task_id
                ),
            });
        }
        for id in ids.difference(requirements) {
            issues.push(TraceabilityIssue {
                code: "TRACEABILITY_TASK_REQUIREMENT_UNKNOWN",
                source: Some(format!("tasks/{}", task.task_id)),
                requirement_id: Some(id.clone()),
                message: format!(
                    "Task {} references Requirement ID {id}, which does not exist in Requirements",
                    task.task_id
                ),
            });
        }
        task_union.extend(ids.iter().cloned());
        task_map.insert(task.task_id, numeric_ids(ids));
    }
    (Some(task_map), task_union)
}

fn numeric_ids(ids: BTreeSet<String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort_by_key(|id| parse_id(id));
    ids
}

fn parse_id(value: &str) -> Option<(u64, u64)> {
    let (group, criterion) = value.split_once('.')?;
    Some((group.parse().ok()?, criterion.parse().ok()?))
}
