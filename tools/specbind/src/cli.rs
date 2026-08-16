//! Concise text CLI execution and stream routing.

use std::{fmt::Write as _, fs, path::Path};

use crate::{
    artifacts::{self, Artifact, DiscoveryIssue},
    config,
    task_read_model::{GroupView, TaskPlanItemView, TaskReadModel, TaskStatus, TaskView},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub success: bool,
}

impl CommandOutput {
    fn success(stdout: Vec<u8>) -> Self {
        Self {
            stdout,
            stderr: vec![],
            success: true,
        }
    }

    fn failure(code: &str, message: impl AsRef<str>, details: Vec<String>) -> Self {
        let mut stderr = format!("ERROR {code}: {}\n", escape(message.as_ref()));
        for detail in details {
            stderr.push_str("  ");
            stderr.push_str(&escape(&detail));
            stderr.push('\n');
        }
        Self {
            stdout: vec![],
            stderr: stderr.into_bytes(),
            success: false,
        }
    }
}

#[must_use]
pub fn artifact_list(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = artifacts::discover_spec(&paths.specbind_root, canonical_spec);
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .artifacts
            .iter()
            .map(render_artifact)
            .collect::<Vec<_>>();
        details.extend(inventory.issues.iter().map(render_issue));
        return CommandOutput::failure(
            "ARTIFACT_LIST_FAILED",
            format!("Artifact inventory for spec {canonical_spec} has diagnostics."),
            details,
        );
    }
    let mut output = format!(
        "OK ARTIFACT_LISTED: Found {} recognized artifact(s) for spec {}.\n",
        inventory.artifacts.len(),
        escape(canonical_spec)
    );
    for artifact in &inventory.artifacts {
        output.push_str("  ");
        output.push_str(&render_artifact(artifact));
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn artifact_read(start: &Path, canonical_spec: &str, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = artifacts::discover_spec(&paths.specbind_root, canonical_spec);
    let Some(artifact) = inventory
        .artifacts
        .iter()
        .find(|artifact| artifact.selector == selector)
    else {
        return CommandOutput::failure(
            "ARTIFACT_SELECTOR_NOT_FOUND",
            format!("Selector {selector} does not resolve for spec {canonical_spec}."),
            inventory.issues.iter().map(render_issue).collect(),
        );
    };
    let selected_issues = inventory
        .issues
        .iter()
        .filter(|issue| issue.path.as_ref() == Some(&artifact.path))
        .map(render_issue)
        .collect::<Vec<_>>();
    if !selected_issues.is_empty() {
        return CommandOutput::failure(
            "ARTIFACT_READ_INVALID",
            format!("Selector {selector} has profile or content diagnostics."),
            inventory.issues.iter().map(render_issue).collect(),
        );
    }
    let path = paths.specbind_root.join(artifact.path.as_std_path());
    if !fs::symlink_metadata(&path)
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
    {
        return CommandOutput::failure(
            "ARTIFACT_READ_TARGET_INVALID",
            "Resolved artifact is no longer a regular non-symlink file.",
            vec![],
        );
    }
    match fs::read(path) {
        Ok(bytes) if std::str::from_utf8(&bytes).is_ok() => {
            let mut stderr = String::new();
            for issue in &inventory.issues {
                stderr.push_str("  ");
                stderr.push_str(&render_issue(issue));
                stderr.push('\n');
            }
            CommandOutput {
                stdout: bytes,
                stderr: stderr.into_bytes(),
                success: true,
            }
        }
        Ok(_) => CommandOutput::failure(
            "ARTIFACT_READ_NOT_UTF8",
            "Resolved artifact is not valid UTF-8.",
            vec![],
        ),
        Err(error) => CommandOutput::failure("ARTIFACT_READ_FAILED", error.to_string(), vec![]),
    }
}

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

fn load_task_model(start: &Path, canonical_spec: &str) -> Result<TaskReadModel, CommandOutput> {
    let paths = config::resolve_from(start)
        .map_err(|error| CommandOutput::failure(error.code, error.message, vec![]))?;
    let resolution = artifacts::resolve_tasks(&paths.specbind_root, canonical_spec);
    match resolution.tasks {
        Some(tasks) if resolution.issues.is_empty() => Ok(TaskReadModel::derive(&tasks)),
        _ => Err(CommandOutput::failure(
            "TASKS_READ_FAILED",
            format!("Cannot derive tasks for spec {canonical_spec}."),
            resolution.issues.iter().map(render_issue).collect(),
        )),
    }
}

fn render_group(group: &GroupView, output: &mut String) {
    let total = group.tasks.len();
    let status = if group.completed == total {
        "completed"
    } else if group.completed == 0 {
        "pending"
    } else {
        "partial"
    };
    writeln!(
        output,
        "  [{status} {}/{}; {} blocked] {} {}",
        group.completed,
        total,
        group.blocked,
        escape(&group.id),
        escape(&group.title)
    )
    .expect("writing to a String cannot fail");
    for task in &group.tasks {
        output.push_str("    ");
        output.push_str(&render_task_summary(task));
        output.push('\n');
    }
}

fn render_task_summary(task: &TaskView) -> String {
    format!(
        "[{}] {} {}",
        render_status(task),
        escape(&task.id),
        escape(&task.title)
    )
}

fn render_status(task: &TaskView) -> String {
    match task.status {
        TaskStatus::Completed => "completed".to_owned(),
        TaskStatus::Blocked => "blocked".to_owned(),
        TaskStatus::Pending if task.actionable => "pending actionable".to_owned(),
        TaskStatus::Pending => "pending waiting".to_owned(),
    }
}

fn push_field(output: &mut String, label: &str, value: &str) {
    output.push_str("  ");
    output.push_str(label);
    output.push_str(": ");
    output.push_str(&escape(value));
    output.push('\n');
}

fn push_inline_list(output: &mut String, label: &str, values: &[String]) {
    let value = if values.is_empty() {
        "none".to_owned()
    } else {
        values
            .iter()
            .map(|value| escape(value))
            .collect::<Vec<_>>()
            .join(", ")
    };
    push_field(output, label, &value);
}

fn push_list(output: &mut String, label: &str, values: &[String]) {
    if values.is_empty() {
        push_field(output, label, "none");
        return;
    }
    output.push_str("  ");
    output.push_str(label);
    output.push_str(":\n");
    for value in values {
        output.push_str("    - ");
        output.push_str(&escape(value));
        output.push('\n');
    }
}

fn render_artifact(artifact: &Artifact) -> String {
    let mut output = format!(
        "selector={} type=\"{}\" path={}",
        escape(&artifact.selector),
        escape(&artifact.artifact_type),
        escape(artifact.path.as_str())
    );
    if let Some(artifact_id) = &artifact.artifact_id {
        output.push_str(" artifact_id=");
        output.push_str(&escape(artifact_id));
    }
    output
}

fn render_issue(issue: &DiscoveryIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path.as_str())));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

fn escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|value| match value {
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            value if value.is_control() || value == '\u{1b}' => {
                format!("\\u{{{:x}}}", u32::from(value)).chars().collect()
            }
            value => vec![value],
        })
        .collect()
}
