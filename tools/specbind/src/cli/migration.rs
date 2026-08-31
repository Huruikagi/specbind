//! CLI execution and rendering for cc-sdd migration.

use super::*;

#[must_use]
pub fn migrate_cc_sdd(start: &Path, apply: bool, accept_resolution: Option<&str>) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(project_root) => project_root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    if accept_resolution.is_none() && migration::source_absent_and_target_current(&project_root) {
        return CommandOutput::no_change(
            "CC_SDD_MIGRATION_COMPLETE",
            "No cc-sdd source remains and the SpecBind target is current.",
        );
    }
    if let Some(source) = accept_resolution {
        let candidate =
            match read_external_input(&MIGRATION_RESOLUTION_INPUT, start, &project_root, source) {
                Ok(candidate) => candidate,
                Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
            };
        return match migration_resolution::accept(&project_root, &candidate) {
            Ok(accepted) => CommandOutput::success(
                format!(
                    "OK CC_SDD_MIGRATION_RESOLUTION_ACCEPTED: Accepted the current cc-sdd migration resolution.\n  Path: {}\n  Accepted at: {}\n  Resolutions: {}\n",
                    accepted.path,
                    accepted.accepted_at,
                    accepted.resolutions
                )
                .into_bytes(),
            ),
            Err(error) => CommandOutput::failure(
                "MIGRATION_RESOLUTION_ACCEPT_FAILED",
                "Cannot accept cc-sdd migration resolution.",
                error.issues.iter().map(render_migration_finding).collect(),
            ),
        };
    }
    let plan = match migration::plan(&project_root) {
        Ok(plan) => plan,
        Err(error) => {
            return CommandOutput::failure(
                "MIGRATION_PLAN_FAILED",
                "Cannot plan cc-sdd migration.",
                error.issues.iter().map(render_migration_finding).collect(),
            );
        }
    };
    if !plan.findings.is_empty() {
        let mut details = migration_summary(&plan);
        details.extend(plan.findings.iter().map(render_migration_finding));
        details.push(format!("Guide: {}", migration::GUIDE_URL));
        details.push("No files were changed.".to_owned());
        details.push(format!(
            "Original {} tree remains intact.",
            plan.legacy_root
        ));
        return CommandOutput::failure(
            "MANUAL_MIGRATION_REQUIRED",
            "cc-sdd migration requires semantic decisions.",
            details,
        );
    }
    if apply {
        return match migration::apply(&project_root) {
            Ok(outcome) => CommandOutput::success(
                format!(
                    "OK CC_SDD_MIGRATION_APPLIED: Applied the deterministic cc-sdd migration and completed cutover.\n  Installed files: {}\n  Removed legacy assets: {}\n  Removed legacy root: {}\n  Removed legacy config: {}\n  Removed resolution state: {}\n",
                    outcome.installed_files,
                    outcome.removed_legacy_assets,
                    escape(&outcome.removed_legacy_root),
                    yes_no(outcome.removed_legacy_config),
                    yes_no(outcome.removed_resolution_state)
                )
                .into_bytes(),
            ),
            Err(error) => CommandOutput::failure(
                "MIGRATION_APPLY_FAILED",
                "Cannot apply cc-sdd migration.",
                error.issues.iter().map(render_migration_finding).collect(),
            ),
        };
    }
    render_migration_plan(&plan)
}

fn render_migration_plan(plan: &migration::MigrationPlan) -> CommandOutput {
    let mut output = format!(
        "OK CC_SDD_MIGRATION_PLANNED: Planned {} read-only action(s) from {}.\n",
        plan.actions.len(),
        escape(&plan.legacy_root)
    );
    push_field(&mut output, "Target", &plan.target_root);
    push_field(
        &mut output,
        "Language",
        match plan.language {
            Some(config::ProjectLanguage::En) => "en",
            Some(config::ProjectLanguage::Ja) => "ja",
            None => "unknown",
        },
    );
    push_inline_list(&mut output, "Agents", &plan.agents);
    push_field(&mut output, "Specs", &plan.specs.len().to_string());
    output.push_str("  Actions:\n");
    for action in &plan.actions {
        let source = action
            .source
            .as_deref()
            .map_or_else(String::new, |value| format!(" source={}", escape(value)));
        let target = action
            .target
            .as_deref()
            .map_or_else(String::new, |value| format!(" target={}", escape(value)));
        writeln!(
            output,
            "    - {}{source}{target} detail={}",
            action.kind,
            escape(&action.detail)
        )
        .expect("writing to a String cannot fail");
    }
    output.push_str("  No files were changed.\n");
    CommandOutput::success(output.into_bytes())
}

fn migration_summary(plan: &migration::MigrationPlan) -> Vec<String> {
    vec![
        format!("Legacy root: {}", plan.legacy_root),
        format!("Target root: {}", plan.target_root),
        format!("Specs: {}", plan.specs.len()),
    ]
}

fn render_migration_finding(finding: &migration::MigrationFinding) -> String {
    let path = finding
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {path}:"));
    format!("{}{path} {}", finding.code, finding.message)
}
