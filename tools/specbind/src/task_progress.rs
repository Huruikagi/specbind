//! Guarded task execution progress records.
//!
//! These operations mutate only `execution.tasks`. They never change the plan,
//! the declared lifecycle state, or any gate evidence, and Decision 0028
//! excludes execution state from the task-plan fingerprint, so recording
//! progress cannot stale the Tasks gate it was approved under.
//!
//! No Git guard applies. Implementation is the one phase whose worktree is
//! expected to be dirty, and a task is normally completed while its own code
//! changes are uncommitted; the Decision 0086 handshake owns the revision-bound
//! guarantees.

use std::{fmt, path::Path};

use crate::{
    artifacts,
    domain::tasks::Tasks,
    schema::{
        spec::v1::WorkflowState,
        tasks::v1::{
            BlockedStatus, BlockedTaskState, CompletedStatus, CompletedTaskState, NonEmptyString,
            TaskExecution, TaskExecutionState, TaskExecutionStates, TaskReference, TasksDocument,
        },
    },
    task_read_model::{TaskPlanItemView, TaskReadModel, TaskStatus},
};

/// Derived plan progress reported after one recorded change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressReport {
    pub task_id: String,
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub blocked: usize,
    pub actionable_ids: Vec<String>,
    /// Present only for a blocked record.
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompleteOutcome {
    Completed(Box<ProgressReport>),
    AlreadyCompleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockOutcome {
    Blocked(Box<ProgressReport>),
    AlreadyBlocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReopenOutcome {
    Reopened(Box<ProgressReport>),
    NotRecorded,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProgressIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgressIssues {
    pub issues: Vec<ProgressIssue>,
}

impl fmt::Display for ProgressIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "task progress operation has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for ProgressIssues {}

/// Records one task as completed.
///
/// # Errors
///
/// Returns identity, lifecycle, plan, prerequisite, serialization, or
/// filesystem diagnostics without changing `tasks.yaml`.
pub fn complete(
    specbind_root: &Path,
    canonical_spec: &str,
    task_id: &str,
) -> Result<CompleteOutcome, ProgressIssues> {
    let context = resolve(
        specbind_root,
        canonical_spec,
        task_id,
        "TASK_COMPLETE_FAILED",
    )?;
    if context.status == TaskStatus::Completed {
        return Ok(CompleteOutcome::AlreadyCompleted);
    }
    let incomplete = context.unmet_prerequisites();
    if !incomplete.is_empty() {
        return Err(one_issue(
            "TASK_PREREQUISITES_INCOMPLETE",
            Some(tasks_path(canonical_spec)),
            format!(
                "task {task_id} cannot complete before its prerequisites: {}",
                incomplete.join(", ")
            ),
        ));
    }
    let wire = context.with_state(
        task_id,
        Some(TaskExecutionState::Completed(CompletedTaskState {
            status: CompletedStatus::Completed,
        })),
    );
    let report = persist(specbind_root, canonical_spec, wire, task_id, None)?;
    Ok(CompleteOutcome::Completed(Box::new(report)))
}

/// Records one task as blocked with an explicit reason.
///
/// # Errors
///
/// Returns identity, lifecycle, plan, reason, serialization, or filesystem
/// diagnostics without changing `tasks.yaml`.
pub fn block(
    specbind_root: &Path,
    canonical_spec: &str,
    task_id: &str,
    reason: &str,
) -> Result<BlockOutcome, ProgressIssues> {
    let reason = reason.trim();
    if reason.is_empty() || reason.contains(['\n', '\r']) {
        return Err(one_issue(
            "TASK_BLOCKED_REASON_INVALID",
            None,
            "a blocked task requires a non-empty single-line reason",
        ));
    }
    let context = resolve(specbind_root, canonical_spec, task_id, "TASK_BLOCK_FAILED")?;
    if context.status == TaskStatus::Completed {
        return Err(one_issue(
            "TASK_ALREADY_COMPLETED",
            Some(tasks_path(canonical_spec)),
            format!("task {task_id} is completed; reopen it before recording a blocker"),
        ));
    }
    if context.blocked_reason.as_deref() == Some(reason) {
        return Ok(BlockOutcome::AlreadyBlocked);
    }
    let wire = context.with_state(
        task_id,
        Some(TaskExecutionState::Blocked(BlockedTaskState {
            status: BlockedStatus::Blocked,
            blocked_reason: NonEmptyString(reason.to_owned()),
        })),
    );
    let report = persist(
        specbind_root,
        canonical_spec,
        wire,
        task_id,
        Some(reason.to_owned()),
    )?;
    Ok(BlockOutcome::Blocked(Box::new(report)))
}

/// Returns one task to pending by removing its persisted entry.
///
/// # Errors
///
/// Returns identity, lifecycle, plan, serialization, or filesystem diagnostics
/// without changing `tasks.yaml`.
pub fn reopen(
    specbind_root: &Path,
    canonical_spec: &str,
    task_id: &str,
) -> Result<ReopenOutcome, ProgressIssues> {
    let context = resolve(specbind_root, canonical_spec, task_id, "TASK_REOPEN_FAILED")?;
    if context.status == TaskStatus::Pending {
        return Ok(ReopenOutcome::NotRecorded);
    }
    let wire = context.with_state(task_id, None);
    let report = persist(specbind_root, canonical_spec, wire, task_id, None)?;
    Ok(ReopenOutcome::Reopened(Box::new(report)))
}

struct Context {
    wire: TasksDocument,
    status: TaskStatus,
    blocked_reason: Option<String>,
    effective_dependencies: Vec<String>,
    completed_ids: Vec<String>,
}

impl Context {
    /// Prerequisites that are not yet complete, in plan order.
    fn unmet_prerequisites(&self) -> Vec<String> {
        self.effective_dependencies
            .iter()
            .filter(|id| !self.completed_ids.contains(id))
            .cloned()
            .collect()
    }

    /// Returns the document with one execution entry set or removed.
    fn with_state(mut self, task_id: &str, state: Option<TaskExecutionState>) -> TasksDocument {
        let mut states = self
            .wire
            .execution
            .take()
            .map(|execution| execution.tasks.0)
            .unwrap_or_default();
        match state {
            Some(state) => {
                states.insert(TaskReference(task_id.to_owned()), state);
            }
            None => {
                states.remove(&TaskReference(task_id.to_owned()));
            }
        }
        self.wire.execution = (!states.is_empty()).then_some(TaskExecution {
            tasks: TaskExecutionStates(states),
        });
        self.wire
    }
}

fn resolve(
    specbind_root: &Path,
    canonical_spec: &str,
    task_id: &str,
    code: &'static str,
) -> Result<Context, ProgressIssues> {
    if !artifacts::canonical_id(canonical_spec) {
        return Err(one_issue(
            code,
            Some(format!("specs/{canonical_spec}")),
            "task progress requires a canonical Spec ID",
        ));
    }
    let spec = artifacts::resolve_spec(specbind_root, canonical_spec);
    let Some(wire) = spec.wire else {
        return Err(discovery_failure(spec.issues));
    };
    let implementing = wire
        .active_change
        .0
        .as_ref()
        .is_some_and(|active| active.state == WorkflowState::Implementation);
    if !implementing {
        return Err(one_issue(
            "TASK_SPEC_STATE_INVALID",
            Some(format!("specs/{canonical_spec}/spec.yaml")),
            "task progress requires the Spec in implementation state",
        ));
    }

    let resolution = artifacts::resolve_tasks(specbind_root, canonical_spec);
    let Some(tasks) = resolution.tasks.filter(|_| resolution.issues.is_empty()) else {
        return Err(discovery_failure(resolution.issues));
    };
    let model = TaskReadModel::derive(&tasks);
    let Some(view) = model.task(task_id) else {
        return Err(one_issue(
            "TASK_NOT_FOUND",
            Some(tasks_path(canonical_spec)),
            format!("task {task_id} does not resolve to an executable task in the current plan"),
        ));
    };
    Ok(Context {
        status: view.status,
        blocked_reason: view.blocked_reason.clone(),
        effective_dependencies: view.effective_dependencies.clone(),
        completed_ids: completed_ids(&model),
        wire: as_owned_wire(&tasks),
    })
}

fn completed_ids(model: &TaskReadModel) -> Vec<String> {
    model
        .items
        .iter()
        .flat_map(|item| match item {
            TaskPlanItemView::Task(task) => vec![task.as_ref()],
            TaskPlanItemView::Group(group) => group.tasks.iter().collect(),
        })
        .filter(|task| task.status == TaskStatus::Completed)
        .map(|task| task.id.clone())
        .collect()
}

fn as_owned_wire(tasks: &Tasks) -> TasksDocument {
    tasks.as_wire().clone()
}

fn persist(
    specbind_root: &Path,
    canonical_spec: &str,
    wire: TasksDocument,
    task_id: &str,
    blocked_reason: Option<String>,
) -> Result<ProgressReport, ProgressIssues> {
    let relative = tasks_path(canonical_spec);
    let tasks = Tasks::try_from(wire).map_err(|error| ProgressIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, Some(relative.clone()), value.message))
            .collect(),
    })?;
    let mut rendered = serde_saphyr::to_string(tasks.as_wire()).map_err(|error| {
        one_issue(
            "TASK_SERIALIZE_FAILED",
            Some(relative.clone()),
            error.to_string(),
        )
    })?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    crate::guarded_fs::replace_existing(&specbind_root.join(&relative), rendered.as_bytes())
        .map_err(|error| {
            one_issue(
                "TASK_WRITE_FAILED",
                Some(relative.clone()),
                error.to_string(),
            )
        })?;

    let model = TaskReadModel::derive(&tasks);
    Ok(ProgressReport {
        task_id: task_id.to_owned(),
        total: model.total(),
        completed: model.completed,
        pending: model.pending,
        blocked: model.blocked,
        actionable_ids: model.actionable_ids.clone(),
        blocked_reason,
    })
}

fn tasks_path(canonical_spec: &str) -> String {
    format!("specs/{canonical_spec}/tasks.yaml")
}

fn discovery_failure(issues: Vec<artifacts::DiscoveryIssue>) -> ProgressIssues {
    ProgressIssues {
        issues: issues
            .into_iter()
            .map(|value| {
                issue(
                    value.code,
                    value.path.map(|path| path.to_string()),
                    value.message,
                )
            })
            .collect(),
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> ProgressIssues {
    ProgressIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> ProgressIssue {
    ProgressIssue {
        code,
        path,
        message: message.into(),
    }
}
