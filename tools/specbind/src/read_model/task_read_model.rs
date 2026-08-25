//! Read model for task plan and execution projections.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    domain::tasks::Tasks,
    schema::tasks::v1::{
        ExecutableTask, NonEmptyStringList, PlanItem, TaskExecutionState, TaskReferenceList,
        UniqueNonEmptyStringList,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Completed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskView {
    pub id: String,
    pub title: String,
    pub group: Option<(String, String)>,
    pub details: Vec<String>,
    pub completion_criteria: Vec<String>,
    pub requirement_ids: Vec<String>,
    pub boundaries: Vec<String>,
    pub contracts: Vec<String>,
    pub explicit_dependencies: Vec<String>,
    pub effective_dependencies: Vec<String>,
    pub status: TaskStatus,
    pub blocked_reason: Option<String>,
    pub actionable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupView {
    pub id: String,
    pub title: String,
    pub completed: usize,
    pub blocked: usize,
    pub tasks: Vec<TaskView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskPlanItemView {
    Group(GroupView),
    Task(Box<TaskView>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskReadModel {
    pub items: Vec<TaskPlanItemView>,
    pub completed: usize,
    pub pending: usize,
    pub blocked: usize,
    pub actionable_ids: Vec<String>,
}

impl TaskReadModel {
    #[must_use]
    pub fn derive(tasks: &Tasks) -> Self {
        let document = tasks.as_wire();
        let execution = document
            .execution
            .as_ref()
            .map(|execution| &execution.tasks.0);
        let plan_order = document
            .plan
            .items
            .iter()
            .flat_map(|item| match item {
                PlanItem::Task(task) => vec![task_id(task).to_owned()],
                PlanItem::Group(group) => group
                    .tasks
                    .iter()
                    .map(|task| task_id(task).to_owned())
                    .collect(),
            })
            .collect::<Vec<_>>();
        let completed_ids = execution
            .into_iter()
            .flat_map(|states| states.iter())
            .filter(|(_, state)| matches!(state, TaskExecutionState::Completed(_)))
            .map(|(id, _)| id.0.clone())
            .collect::<BTreeSet<_>>();
        let dependencies = effective_dependencies(document);
        let mut completed = 0;
        let mut pending = 0;
        let mut blocked = 0;
        let mut actionable_ids = Vec::new();
        let mut items = Vec::new();

        for item in &document.plan.items {
            match item {
                PlanItem::Task(task) => {
                    let view = task_view(
                        task,
                        None,
                        execution,
                        &completed_ids,
                        &dependencies,
                        &plan_order,
                    );
                    count(&view, &mut completed, &mut pending, &mut blocked);
                    if view.actionable {
                        actionable_ids.push(view.id.clone());
                    }
                    items.push(TaskPlanItemView::Task(Box::new(view)));
                }
                PlanItem::Group(group) => {
                    let group_identity = (group.id.0.clone(), group.title.0.clone());
                    let mut group_completed = 0;
                    let mut group_blocked = 0;
                    let views = group
                        .tasks
                        .iter()
                        .map(|task| {
                            let view = task_view(
                                task,
                                Some(group_identity.clone()),
                                execution,
                                &completed_ids,
                                &dependencies,
                                &plan_order,
                            );
                            count(&view, &mut completed, &mut pending, &mut blocked);
                            if view.status == TaskStatus::Completed {
                                group_completed += 1;
                            }
                            if view.status == TaskStatus::Blocked {
                                group_blocked += 1;
                            }
                            if view.actionable {
                                actionable_ids.push(view.id.clone());
                            }
                            view
                        })
                        .collect();
                    items.push(TaskPlanItemView::Group(GroupView {
                        id: group.id.0.clone(),
                        title: group.title.0.clone(),
                        completed: group_completed,
                        blocked: group_blocked,
                        tasks: views,
                    }));
                }
            }
        }

        Self {
            items,
            completed,
            pending,
            blocked,
            actionable_ids,
        }
    }

    #[must_use]
    pub fn task(&self, id: &str) -> Option<&TaskView> {
        self.items.iter().find_map(|item| match item {
            TaskPlanItemView::Task(task) if task.id == id => Some(task.as_ref()),
            TaskPlanItemView::Group(group) => group.tasks.iter().find(|task| task.id == id),
            TaskPlanItemView::Task(_) => None,
        })
    }

    #[must_use]
    pub fn total(&self) -> usize {
        self.completed + self.pending + self.blocked
    }
}

fn count(view: &TaskView, completed: &mut usize, pending: &mut usize, blocked: &mut usize) {
    match view.status {
        TaskStatus::Pending => *pending += 1,
        TaskStatus::Completed => *completed += 1,
        TaskStatus::Blocked => *blocked += 1,
    }
}

fn task_view(
    task: &ExecutableTask,
    group: Option<(String, String)>,
    execution: Option<&BTreeMap<crate::schema::tasks::v1::TaskReference, TaskExecutionState>>,
    completed_ids: &BTreeSet<String>,
    dependencies: &BTreeMap<String, BTreeSet<String>>,
    plan_order: &[String],
) -> TaskView {
    let fields = fields(task);
    let state = execution.and_then(|states| {
        states.get(&crate::schema::tasks::v1::TaskReference(
            fields.id.to_owned(),
        ))
    });
    let (status, blocked_reason) = match state {
        Some(TaskExecutionState::Completed(_)) => (TaskStatus::Completed, None),
        Some(TaskExecutionState::Blocked(blocked)) => {
            (TaskStatus::Blocked, Some(blocked.blocked_reason.0.clone()))
        }
        None => (TaskStatus::Pending, None),
    };
    let effective = dependencies.get(fields.id).cloned().unwrap_or_default();
    let effective_dependencies = plan_order
        .iter()
        .filter(|id| effective.contains(*id))
        .cloned()
        .collect::<Vec<_>>();
    let actionable = status == TaskStatus::Pending
        && effective_dependencies
            .iter()
            .all(|dependency| completed_ids.contains(dependency));

    TaskView {
        id: fields.id.to_owned(),
        title: fields.title.to_owned(),
        group,
        details: strings(fields.details),
        completion_criteria: strings(fields.completion_criteria),
        requirement_ids: fields.requirement_ids.0.clone(),
        boundaries: unique_strings(fields.boundaries),
        contracts: unique_strings(fields.contracts),
        explicit_dependencies: references(fields.depends_on),
        effective_dependencies,
        status,
        blocked_reason,
        actionable,
    }
}

struct TaskFields<'a> {
    id: &'a str,
    title: &'a str,
    details: Option<&'a NonEmptyStringList>,
    completion_criteria: Option<&'a NonEmptyStringList>,
    requirement_ids: &'a UniqueNonEmptyStringList,
    boundaries: Option<&'a UniqueNonEmptyStringList>,
    contracts: Option<&'a UniqueNonEmptyStringList>,
    depends_on: Option<&'a TaskReferenceList>,
}

fn fields(task: &ExecutableTask) -> TaskFields<'_> {
    TaskFields {
        id: &task.id.0,
        title: &task.title.0,
        details: task.details.as_ref(),
        completion_criteria: task.completion_criteria.as_ref(),
        requirement_ids: &task.requirement_ids,
        boundaries: task.boundaries.as_ref(),
        contracts: task.contracts.as_ref(),
        depends_on: task.depends_on.as_ref(),
    }
}

fn strings(values: Option<&NonEmptyStringList>) -> Vec<String> {
    values.map_or_else(Vec::new, |values| values.0.clone())
}

fn unique_strings(values: Option<&UniqueNonEmptyStringList>) -> Vec<String> {
    values.map_or_else(Vec::new, |values| values.0.clone())
}

fn references(values: Option<&TaskReferenceList>) -> Vec<String> {
    values.map_or_else(Vec::new, |values| {
        values.0.iter().map(|value| value.0.clone()).collect()
    })
}

fn effective_dependencies(
    document: &crate::schema::tasks::v1::TasksDocument,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut dependencies = BTreeMap::<String, BTreeSet<String>>::new();
    let mut preceding_top = Vec::<String>::new();
    for item in &document.plan.items {
        match item {
            PlanItem::Task(task) => {
                let entry = dependencies.entry(task_id(task).to_owned()).or_default();
                entry.extend(preceding_top.iter().cloned());
                entry.extend(references(fields(task).depends_on));
                preceding_top = vec![task_id(task).to_owned()];
            }
            PlanItem::Group(group) => {
                let mut preceding_sibling = None::<String>;
                for task in &group.tasks {
                    let entry = dependencies.entry(task_id(task).to_owned()).or_default();
                    entry.extend(preceding_top.iter().cloned());
                    if let Some(prerequisite) = &preceding_sibling {
                        entry.insert(prerequisite.clone());
                    }
                    entry.extend(references(fields(task).depends_on));
                    preceding_sibling = Some(task_id(task).to_owned());
                }
                preceding_top = group
                    .tasks
                    .iter()
                    .map(|task| task_id(task).to_owned())
                    .collect();
            }
        }
    }
    dependencies
}

fn task_id(task: &ExecutableTask) -> &str {
    fields(task).id
}
