//! Installation planning and apply command rendering.

use super::super::*;

/// Reports the installation plan without touching the filesystem.
#[must_use]
pub fn install_dry_run(start: &Path, inputs: &install::InstallInputs) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match install::plan(&project_root, inputs) {
        Ok(plan) => {
            let (create, replace, keep, remove) = plan.counts();
            let mut output = format!(
                "OK INSTALL_PLANNED: Planned {} action(s) for {} agent(s).\n",
                plan.entries.len(),
                plan.agents.len()
            );
            push_install_summary(&mut output, &plan);
            push_field(
                &mut output,
                "Summary",
                &format!("{create} create, {replace} replace, {keep} keep, {remove} remove"),
            );
            CommandOutput::success(output.into_bytes())
        }
        Err(error) => CommandOutput::failure(
            "INSTALL_PLAN_FAILED",
            "Cannot plan the SpecBind installation.",
            error.issues.iter().map(render_install_issue).collect(),
        ),
    }
}

/// Applies the installation plan.
#[must_use]
pub fn install_apply(start: &Path, inputs: &install::InstallInputs) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match install::apply(&project_root, inputs) {
        Ok(outcome) if outcome.unchanged => CommandOutput::no_change(
            "INSTALL_UP_TO_DATE",
            &format!(
                "SpecBind product assets are already installed for {} agent(s).",
                outcome.plan.agents.len()
            ),
        ),
        Ok(outcome) => {
            let (create, replace, keep, remove) = outcome.plan.counts();
            let mut output = format!(
                "OK INSTALL_APPLIED: Applied {} action(s) for {} agent(s).
",
                create + replace + remove,
                outcome.plan.agents.len()
            );
            push_install_summary(&mut output, &outcome.plan);
            push_field(
                &mut output,
                "Summary",
                &format!("{create} created, {replace} replaced, {keep} kept, {remove} removed"),
            );
            if outcome.plan.initial {
                push_field(
                    &mut output,
                    "Next",
                    "Ask your coding agent to use specbind-configure to review and configure SpecBind for this project.",
                );
            }
            CommandOutput::success(output.into_bytes())
        }
        Err(error) => CommandOutput::failure(
            "INSTALL_FAILED",
            "Cannot apply the SpecBind installation.",
            error.issues.iter().map(render_install_issue).collect(),
        ),
    }
}

fn push_install_summary(output: &mut String, plan: &install::InstallPlan) {
    push_field(
        output,
        "Mode",
        if plan.initial { "initial" } else { "refresh" },
    );
    push_field(output, "Spec directory", &plan.spec_dir);
    push_field(
        output,
        "Language",
        match plan.language {
            crate::config::ProjectLanguage::En => "en",
            crate::config::ProjectLanguage::Ja => "ja",
        },
    );
    push_field(
        output,
        "Agents",
        &plan
            .agents
            .iter()
            .map(|agent| agent.name())
            .collect::<Vec<_>>()
            .join(", "),
    );
    push_field(
        output,
        "Project instructions",
        if plan.project_instructions {
            "enabled"
        } else {
            "disabled"
        },
    );
    output.push_str("  Actions:\n");
    for entry in &plan.entries {
        let detail = entry
            .detail
            .as_ref()
            .map_or_else(String::new, |detail| format!(" ({})", escape(detail)));
        writeln!(
            output,
            "    - {} {} [{}]{detail}",
            entry.action.name(),
            escape(&entry.path),
            entry.category
        )
        .expect("writing to a String cannot fail");
    }
}

fn render_install_issue(issue: &install::InstallIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}
