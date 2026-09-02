//! Concise text CLI execution and stream routing.

mod input;
mod lifecycle;
#[path = "cli/migration.rs"]
mod migration_cli;
mod output;
mod read;
mod tasks;

use input::{
    ExternalInputError, LOG_ENTRIES_INPUT, MIGRATION_RESOLUTION_INPUT, REVIEW_CANDIDATE_INPUT,
    SCOPE_INPUT, read_external_input, read_external_json,
};
pub use lifecycle::*;
pub use migration_cli::*;
pub use output::CommandOutput;
pub use read::*;
pub use tasks::*;

use output::{escape, push_field, push_inline_list, push_list, yes_no};

use std::{
    fmt::Write as _,
    fs,
    io::{self, Read as _},
    path::Path,
};

use crate::{
    adapter, adoption_finalize, approval,
    artifacts::{self, Artifact, DiscoveryIssue},
    completion::{self, CompletionIssue},
    config,
    contract_graph::{self, GraphIssueSeverity},
    cross_spec_review::{self, ReviewFreshnessStatus, ReviewIssue},
    install, instruction, migration, migration_resolution,
    milestone::{self, MilestoneIssue},
    milestone_scope,
    milestone_status::{self, MilestoneHealth, MilestoneStatusModel},
    protocol,
    release_finalize::{self, FinalizeIssue},
    release_readiness::{self, MutationTargetState, ReleaseDiagnostic},
    rule, schema,
    spec_list::{self, SpecHealth},
    spec_status::{self, ConsistencyHealth, SpecStatusModel},
    steering,
    task_progress::{self, ProgressReport},
    task_read_model::{GroupView, TaskPlanItemView, TaskReadModel, TaskStatus, TaskView},
    template,
};

/// Reports the public feedback routes without opening a browser or transmitting data.
#[must_use]
pub fn feedback() -> CommandOutput {
    CommandOutput::success(
        concat!(
            "OK FEEDBACK_REPORTED: SpecBind feedback routes.\n",
            "  Bug report: https://github.com/Huruikagi/specbind/issues/new?template=bug-report.yml\n",
            "  Improvement proposal: https://github.com/Huruikagi/specbind/issues/new?template=improvement.yml\n",
            "  Include: specbind --version, the affected command or Skill, and reproduction steps\n",
            "  Evidence: Relevant sanitized output or artifacts\n",
            "  Privacy: Remove secrets and private project content before submitting\n",
            "  No information has been transmitted.\n",
        )
        .as_bytes()
        .to_vec(),
    )
}

fn render_milestone_diagnostic(diagnostic: &milestone_status::MilestoneDiagnostic) -> String {
    let path = diagnostic
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", diagnostic.code, escape(&diagnostic.message))
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
