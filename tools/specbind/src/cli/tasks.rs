//! CLI execution and rendering for task progress.

use super::{
    CommandOutput, Path, ProgressReport, TaskPlanItemView, config, escape, load_task_model,
    push_field, push_inline_list, push_list, render_group, render_status, render_task_summary,
    task_progress,
};

#[must_use]
pub fn tasks_list(start: &Path, canonical_spec: &str) -> CommandOutput {
    let model = match load_task_model(start, canonical_spec) {
        Ok(model) => model,
        Err(output) => return output,
    };
    let mut output = format!(
        "OK TASKS_LISTED: Listed {} task(s) for spec {} ({} completed, {} pending, {} blocked).\n",
        model.total(),
        escape(canonical_spec),
        model.completed,
        model.pending,
        model.blocked
    );
    for item in &model.items {
        match item {
            TaskPlanItemView::Task(task) => {
                output.push_str("  ");
                output.push_str(&render_task_summary(task));
                output.push('\n');
            }
            TaskPlanItemView::Group(group) => render_group(group, &mut output),
        }
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn tasks_show(start: &Path, canonical_spec: &str, task_id: &str) -> CommandOutput {
    let model = match load_task_model(start, canonical_spec) {
        Ok(model) => model,
        Err(output) => return output,
    };
    let Some(task) = model.task(task_id) else {
        return CommandOutput::failure(
            "TASK_NOT_FOUND",
            format!("Task {task_id} does not exist in spec {canonical_spec}."),
            vec![],
        );
    };
    let mut output = format!(
        "OK TASK_SHOWN: Found task {} in spec {}.\n",
        escape(&task.id),
        escape(canonical_spec)
    );
    push_field(&mut output, "Status", &render_status(task));
    push_field(&mut output, "Title", &task.title);
    push_field(
        &mut output,
        "Group",
        &task.group.as_ref().map_or_else(
            || "none".to_owned(),
            |(id, title)| format!("{} {}", escape(id), escape(title)),
        ),
    );
    push_list(&mut output, "Details", &task.details);
    push_inline_list(&mut output, "Requirement IDs", &task.requirement_ids);
    push_inline_list(&mut output, "Boundaries", &task.boundaries);
    push_inline_list(&mut output, "Contracts", &task.contracts);
    push_inline_list(
        &mut output,
        "Explicit prerequisites",
        &task.explicit_dependencies,
    );
    push_inline_list(
        &mut output,
        "Effective prerequisites",
        &task.effective_dependencies,
    );
    push_field(
        &mut output,
        "Blocker",
        task.blocked_reason.as_deref().unwrap_or("none"),
    );
    push_list(
        &mut output,
        "Completion criteria",
        &task.completion_criteria,
    );
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn tasks_complete(start: &Path, canonical_spec: &str, task_id: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match task_progress::complete(&paths.specbind_root, canonical_spec, task_id) {
        Ok(task_progress::CompleteOutcome::Completed(report)) => CommandOutput::success(
            render_progress("TASK_COMPLETED", "Completed", canonical_spec, &report).into_bytes(),
        ),
        Ok(task_progress::CompleteOutcome::AlreadyCompleted) => CommandOutput::no_change(
            "TASK_ALREADY_COMPLETED",
            &format!(
                "Task {} in spec {} is already completed.",
                escape(task_id),
                escape(canonical_spec)
            ),
        ),
        Err(error) => render_progress_failure(
            "TASK_COMPLETE_FAILED",
            &format!("Cannot complete task {task_id} in spec {canonical_spec}."),
            &error,
        ),
    }
}

#[must_use]
pub fn tasks_block(
    start: &Path,
    canonical_spec: &str,
    task_id: &str,
    reason: &str,
) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match task_progress::block(&paths.specbind_root, canonical_spec, task_id, reason) {
        Ok(task_progress::BlockOutcome::Blocked(report)) => CommandOutput::success(
            render_progress("TASK_BLOCKED", "Blocked", canonical_spec, &report).into_bytes(),
        ),
        Ok(task_progress::BlockOutcome::AlreadyBlocked) => CommandOutput::no_change(
            "TASK_ALREADY_BLOCKED",
            &format!(
                "Task {} in spec {} already records that blocker.",
                escape(task_id),
                escape(canonical_spec)
            ),
        ),
        Err(error) => render_progress_failure(
            "TASK_BLOCK_FAILED",
            &format!("Cannot block task {task_id} in spec {canonical_spec}."),
            &error,
        ),
    }
}

#[must_use]
pub fn tasks_reopen(start: &Path, canonical_spec: &str, task_id: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match task_progress::reopen(&paths.specbind_root, canonical_spec, task_id) {
        Ok(task_progress::ReopenOutcome::Reopened(report)) => CommandOutput::success(
            render_progress("TASK_REOPENED", "Reopened", canonical_spec, &report).into_bytes(),
        ),
        Ok(task_progress::ReopenOutcome::NotRecorded) => CommandOutput::no_change(
            "TASK_NOT_RECORDED",
            &format!(
                "Task {} in spec {} has no recorded execution state.",
                escape(task_id),
                escape(canonical_spec)
            ),
        ),
        Err(error) => render_progress_failure(
            "TASK_REOPEN_FAILED",
            &format!("Cannot reopen task {task_id} in spec {canonical_spec}."),
            &error,
        ),
    }
}

fn render_progress(
    code: &str,
    verb: &str,
    canonical_spec: &str,
    report: &ProgressReport,
) -> String {
    let mut output = format!(
        "OK {code}: {verb} task {} in spec {}.
",
        escape(&report.task_id),
        escape(canonical_spec)
    );
    if let Some(reason) = &report.blocked_reason {
        push_field(&mut output, "Blocker", reason);
    }
    push_field(
        &mut output,
        "Progress",
        &format!(
            "{}/{} completed, {} pending, {} blocked",
            report.completed, report.total, report.pending, report.blocked
        ),
    );
    push_inline_list(&mut output, "Next actionable", &report.actionable_ids);
    output
}

fn render_progress_failure(
    code: &'static str,
    message: &str,
    error: &task_progress::ProgressIssues,
) -> CommandOutput {
    CommandOutput::failure(
        code,
        message,
        error
            .issues
            .iter()
            .map(|issue| {
                let path = issue
                    .path
                    .as_ref()
                    .map_or_else(String::new, |path| format!(" {}:", escape(path)));
                format!("{}{path} {}", issue.code, escape(&issue.message))
            })
            .collect(),
    )
}
