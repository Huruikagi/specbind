//! Agent-removal and project-uninstall command rendering.

use super::super::*;
use crate::removal::{self, KnowledgePolicy, RemovalAction, RemovalIssue, RemovalPlan};

#[must_use]
pub fn remove_agent_plan(start: &Path, agent: install::Agent) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match removal::plan_agent(&project_root, agent) {
        Ok(plan) if plan.unchanged => CommandOutput::no_change(
            "AGENT_ALREADY_REMOVED",
            &format!("The {} integration is already removed.", agent.name()),
        ),
        Ok(plan) => CommandOutput::success(render_plan(
            "AGENT_REMOVAL_PLANNED",
            &format!("Planned removal of the {} integration.", agent.name()),
            &plan,
        )),
        Err(error) => removal_failure(
            "AGENT_REMOVAL_PLAN_FAILED",
            "Cannot plan the agent removal.",
            &error,
        ),
    }
}

#[must_use]
pub fn remove_agent_apply(start: &Path, agent: install::Agent) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match removal::apply_agent(&project_root, agent) {
        Ok(plan) if plan.unchanged => CommandOutput::no_change(
            "AGENT_ALREADY_REMOVED",
            &format!("The {} integration is already removed.", agent.name()),
        ),
        Ok(plan) => CommandOutput::success(render_plan(
            "AGENT_REMOVAL_APPLIED",
            &format!("Removed the {} integration.", agent.name()),
            &plan,
        )),
        Err(error) => removal_failure(
            "AGENT_REMOVAL_FAILED",
            "Cannot apply the agent removal.",
            &error,
        ),
    }
}

#[must_use]
pub fn uninstall_plan(start: &Path, knowledge: KnowledgePolicy) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match removal::plan_uninstall(&project_root, knowledge) {
        Ok(plan) if plan.unchanged => CommandOutput::no_change(
            "PROJECT_ALREADY_UNINSTALLED",
            "The SpecBind project integration is already uninstalled.",
        ),
        Ok(plan) => CommandOutput::success(render_plan(
            "PROJECT_UNINSTALL_PLANNED",
            "Planned removal of the SpecBind project integration.",
            &plan,
        )),
        Err(error) => removal_failure(
            "PROJECT_UNINSTALL_PLAN_FAILED",
            "Cannot plan the project uninstall.",
            &error,
        ),
    }
}

#[must_use]
pub fn uninstall_apply(start: &Path, knowledge: KnowledgePolicy) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match removal::apply_uninstall(&project_root, knowledge) {
        Ok(plan) if plan.unchanged => CommandOutput::no_change(
            "PROJECT_ALREADY_UNINSTALLED",
            "The SpecBind project integration is already uninstalled.",
        ),
        Ok(plan) => CommandOutput::success(render_plan(
            "PROJECT_UNINSTALL_APPLIED",
            "Removed the SpecBind project integration.",
            &plan,
        )),
        Err(error) => removal_failure(
            "PROJECT_UNINSTALL_FAILED",
            "Cannot apply the project uninstall.",
            &error,
        ),
    }
}

fn render_plan(code: &str, message: &str, plan: &RemovalPlan) -> Vec<u8> {
    let mut output = format!("OK {code}: {message}\n");
    if let Some(agent) = plan.agent {
        push_field(&mut output, "Agent", agent.name());
    }
    if let Some(knowledge) = plan.knowledge {
        push_field(&mut output, "Knowledge", knowledge.name());
    }
    let removed = plan
        .entries
        .iter()
        .filter(|entry| entry.action == RemovalAction::Remove)
        .count();
    let updated = plan
        .entries
        .iter()
        .filter(|entry| entry.action == RemovalAction::Update)
        .count();
    let retained = plan
        .entries
        .iter()
        .filter(|entry| entry.action == RemovalAction::Retain)
        .count();
    let absent = plan
        .entries
        .iter()
        .filter(|entry| entry.action == RemovalAction::Absent)
        .count();
    push_field(
        &mut output,
        "Summary",
        &format!("{removed} remove, {updated} update, {retained} retain, {absent} absent"),
    );
    push_field(
        &mut output,
        "Recovery",
        "removed tracked content is recoverable from the pre-apply Git revision",
    );
    output.push_str("  Targets:\n");
    for entry in &plan.entries {
        writeln!(
            output,
            "    - {} {} [{}] ({})",
            entry.action.name(),
            escape(&entry.path),
            entry.category,
            escape(&entry.detail)
        )
        .expect("writing to a String cannot fail");
    }
    output.into_bytes()
}

fn removal_failure(code: &str, message: &str, error: &removal::RemovalIssues) -> CommandOutput {
    CommandOutput::failure(
        code,
        message,
        error.issues.iter().map(render_issue).collect(),
    )
}

fn render_issue(issue: &RemovalIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}
