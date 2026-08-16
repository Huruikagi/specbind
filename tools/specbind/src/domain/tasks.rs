use std::collections::{BTreeMap, BTreeSet};

use petgraph::{algo::is_cyclic_directed, graphmap::DiGraphMap};

use crate::schema::tasks::v1::{self as wire, ExecutableTask, PlanItem};

use super::diagnostics::{SemanticIssues, issue};

/// A `tasks.yaml` v1 document whose artifact-local plan invariants hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tasks {
    wire: wire::TasksDocument,
}

impl Tasks {
    #[must_use]
    pub fn as_wire(&self) -> &wire::TasksDocument {
        &self.wire
    }

    #[must_use]
    pub fn into_wire(self) -> wire::TasksDocument {
        self.wire
    }
}

impl TryFrom<wire::TasksDocument> for Tasks {
    type Error = SemanticIssues;

    fn try_from(wire: wire::TasksDocument) -> Result<Self, Self::Error> {
        let issues = validate(&wire);
        if issues.is_empty() {
            Ok(Self { wire })
        } else {
            Err(SemanticIssues::from_unsorted(issues))
        }
    }
}

fn validate(document: &wire::TasksDocument) -> Vec<super::SemanticIssue> {
    let mut issues = Vec::new();
    let mut tasks = BTreeMap::<String, &ExecutableTask>::new();

    for (top_index, item) in document.plan.items.iter().enumerate() {
        let expected_top = (top_index + 1).to_string();
        match item {
            PlanItem::Task(task) => {
                check_id(task_id(task), &expected_top, &mut issues);
                validate_requirement_ids(task, &mut issues);
                tasks.insert(task_id(task).to_owned(), task);
            }
            PlanItem::Group(group) => {
                check_id(&group.id.0, &expected_top, &mut issues);
                for (child_index, task) in group.tasks.iter().enumerate() {
                    let expected = format!("{}.{}", top_index + 1, child_index + 1);
                    check_id(task_id(task), &expected, &mut issues);
                    validate_requirement_ids(task, &mut issues);
                    tasks.insert(task_id(task).to_owned(), task);
                }
            }
        }
    }

    validate_references(document, &tasks, &mut issues);
    validate_execution(document, &tasks, &mut issues);
    issues
}

fn validate_requirement_ids(task: &ExecutableTask, issues: &mut Vec<super::SemanticIssue>) {
    let ids = match task {
        ExecutableTask::Parallel(value) => &value.requirement_ids.0,
        ExecutableTask::Sequential(value) => &value.requirement_ids.0,
    };
    if ids
        .iter()
        .any(|id| super::parse_requirement_id(id).is_none())
    {
        let id = task_id(task);
        issues.push(issue(
            "TASK_REQUIREMENT_ID_FORMAT",
            format!("/plan/tasks/{id}/requirement_ids"),
            format!("task {id} Requirement IDs must use positive numeric N.M form"),
        ));
    }
}

fn check_id(actual: &str, expected: &str, issues: &mut Vec<super::SemanticIssue>) {
    if actual != expected {
        issues.push(issue(
            "TASK_POSITIONAL_ID",
            format!("/plan/items/{actual}"),
            format!("positional Task ID must be {expected}, found {actual}"),
        ));
    }
}

fn validate_references(
    document: &wire::TasksDocument,
    tasks: &BTreeMap<String, &ExecutableTask>,
    issues: &mut Vec<super::SemanticIssue>,
) {
    let mut graph = DiGraphMap::<&str, ()>::new();
    for id in tasks.keys() {
        graph.add_node(id);
    }

    let mut preceding_top = Vec::<&str>::new();
    for item in &document.plan.items {
        match item {
            PlanItem::Task(task) => {
                add_implicit_edges(&mut graph, &preceding_top, task);
                add_explicit_edges(&mut graph, tasks, task, issues);
                preceding_top = vec![task_id(task)];
            }
            PlanItem::Group(group) => {
                let mut preceding_sibling = None;
                for task in &group.tasks {
                    for prerequisite in &preceding_top {
                        graph.add_edge(*prerequisite, task_id(task), ());
                    }
                    if !is_parallel(task)
                        && let Some(prerequisite) = preceding_sibling
                    {
                        graph.add_edge(prerequisite, task_id(task), ());
                    }
                    add_explicit_edges(&mut graph, tasks, task, issues);
                    preceding_sibling = Some(task_id(task));
                }
                preceding_top = group.tasks.iter().map(task_id).collect();
            }
        }
    }

    if is_cyclic_directed(&graph) {
        issues.push(issue(
            "TASK_DEPENDENCY_CYCLE",
            "/plan/items",
            "effective task dependencies contain a cycle",
        ));
    }
}

fn add_implicit_edges<'a>(
    graph: &mut DiGraphMap<&'a str, ()>,
    preceding: &[&'a str],
    task: &'a ExecutableTask,
) {
    if !is_parallel(task) {
        for prerequisite in preceding {
            graph.add_edge(*prerequisite, task_id(task), ());
        }
    }
}

fn add_explicit_edges<'a>(
    graph: &mut DiGraphMap<&'a str, ()>,
    tasks: &BTreeMap<String, &'a ExecutableTask>,
    task: &'a ExecutableTask,
    issues: &mut Vec<super::SemanticIssue>,
) {
    let id = task_id(task);
    for dependency in depends_on(task) {
        if dependency == id {
            issues.push(issue(
                "TASK_DEPENDENCY_SELF",
                format!("/plan/tasks/{id}/depends_on"),
                format!("task {id} cannot depend on itself"),
            ));
        } else if tasks.contains_key(dependency) {
            graph.add_edge(dependency, id, ());
        } else {
            issues.push(issue(
                "TASK_DEPENDENCY_MISSING",
                format!("/plan/tasks/{id}/depends_on"),
                format!("dependency {dependency} does not resolve to an executable task"),
            ));
        }
    }
}

fn validate_execution(
    document: &wire::TasksDocument,
    tasks: &BTreeMap<String, &ExecutableTask>,
    issues: &mut Vec<super::SemanticIssue>,
) {
    let Some(execution) = &document.execution else {
        return;
    };
    for id in execution.tasks.0.keys() {
        if !tasks.contains_key(&id.0) {
            issues.push(issue(
                "TASK_EXECUTION_UNKNOWN",
                format!("/execution/tasks/{}", id.0),
                format!(
                    "execution state references unknown executable task {}",
                    id.0
                ),
            ));
        }
    }
}

fn task_id(task: &ExecutableTask) -> &str {
    match task {
        ExecutableTask::Parallel(value) => &value.id.0,
        ExecutableTask::Sequential(value) => &value.id.0,
    }
}

fn is_parallel(task: &ExecutableTask) -> bool {
    matches!(task, ExecutableTask::Parallel(_))
}

fn depends_on(task: &ExecutableTask) -> BTreeSet<&str> {
    let references = match task {
        ExecutableTask::Parallel(value) => value.depends_on.as_ref(),
        ExecutableTask::Sequential(value) => value.depends_on.as_ref(),
    };
    references
        .into_iter()
        .flat_map(|values| &values.0)
        .map(|reference| reference.0.as_str())
        .collect()
}
