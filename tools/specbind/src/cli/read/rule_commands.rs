//! Project-bound shared-rule catalog commands.

use super::super::*;
use super::present;

/// Lists the fixed shared-rule set and whether each project copy is present.
///
/// Unknown files below `settings/rules/` are not extensions and are never
/// listed. Present rules must carry valid durable managed instructions.
#[must_use]
pub fn rule_list(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let mut details = Vec::new();
    for entry in rule::defaults() {
        match entry.read(&paths.specbind_root) {
            Ok(content) => {
                if content.is_none() && entry.selector == "design-template-selection" {
                    return CommandOutput::failure(
                        "RULE_LIST_FAILED",
                        "Cannot inspect project rules.",
                        vec![format!("RULE_REQUIRED missing {}", entry.path())],
                    );
                }
                if let Some(content) = &content {
                    let issues = instruction::validate_live(content);
                    if !issues.is_empty() {
                        return CommandOutput::failure(
                            "RULE_LIST_FAILED",
                            "Cannot inspect project rules.",
                            issues
                                .iter()
                                .map(|issue| {
                                    format!("{} {}: {}", issue.code, entry.path(), issue.message)
                                })
                                .collect(),
                        );
                    }
                    if entry.selector == "design-template-selection"
                        && let Err(details) = validate_design_template_selection_rule(
                            &paths.specbind_root,
                            paths.language,
                            content,
                        )
                    {
                        return CommandOutput::failure(
                            "RULE_LIST_FAILED",
                            "Cannot inspect project rules.",
                            details,
                        );
                    }
                }
                details.push(format!(
                    "selector={} type=\"SpecBind Rule\" path={} present={}",
                    escape(entry.selector),
                    escape(&entry.path()),
                    present(content.is_some())
                ));
            }
            Err(error) => {
                return CommandOutput::failure(
                    "RULE_LIST_FAILED",
                    "Cannot inspect project rules.",
                    vec![format!("{} {}", error.code, error.message)],
                );
            }
        }
    }
    let mut output = format!(
        "OK RULE_LISTED: Found {} accepted rule(s).\n",
        details.len()
    );
    for detail in details {
        output.push_str("  ");
        output.push_str(&detail);
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one project-owned rule as raw or purpose-projected UTF-8 Markdown.
///
/// Absence is a successful no-change result for ordinary preference Rules
/// because product protocols remain authoritative. Decision 0152's Design
/// template selection Rule is required and fails when absent.
#[must_use]
pub fn rule_read(start: &Path, selector: &str, purpose: Option<&str>) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let Some(entry) = rule::find(selector) else {
        return CommandOutput::failure(
            "RULE_READ_INVALID",
            format!("unknown rule selector: {selector}"),
            vec![],
        );
    };
    match entry.read(&paths.specbind_root) {
        Ok(Some(content)) => {
            let issues = instruction::validate_live(&content);
            if !issues.is_empty() {
                return CommandOutput::failure(
                    "RULE_READ_FAILED",
                    format!("Rule {selector} has invalid managed instructions."),
                    issues
                        .iter()
                        .map(|issue| format!("{} {}", issue.code, issue.message))
                        .collect(),
                );
            }
            if selector == "design-template-selection"
                && let Err(details) = validate_design_template_selection_rule(
                    &paths.specbind_root,
                    paths.language,
                    &content,
                )
            {
                return CommandOutput::failure(
                    "RULE_READ_FAILED",
                    "Design template selection rule is inconsistent with the template set.",
                    details,
                );
            }
            let projected = match purpose {
                Some("maintain") => {
                    instruction::project(&content, instruction::InstructionScope::Maintain)
                }
                Some("consume") => {
                    instruction::project(&content, instruction::InstructionScope::Consume)
                }
                _ => content,
            };
            CommandOutput::success(projected.into_bytes())
        }
        Ok(None) if selector == "design-template-selection" => CommandOutput::failure(
            "RULE_REQUIRED",
            "The project has no Design template selection rule.",
            vec![format!("missing {}", entry.path())],
        ),
        Ok(None) => CommandOutput::no_change(
            "RULE_ABSENT",
            &format!("The project has no {selector} rule."),
        ),
        Err(error) => CommandOutput::failure(error.code, error.message, vec![]),
    }
}

fn validate_design_template_selection_rule(
    specbind_root: &Path,
    language: config::ProjectLanguage,
    content: &str,
) -> Result<(), Vec<String>> {
    let inventory = template::discover_spec_templates(specbind_root, language);
    if !inventory.issues.is_empty() {
        return Err(inventory.issues.iter().map(render_issue).collect());
    }
    let selectors = inventory
        .templates
        .iter()
        .filter(|template| template.artifact_type == "SpecBind Design")
        .map(|template| template.selector.clone())
        .collect::<Vec<_>>();
    let issues = rule::validate_design_template_selection(content, &selectors);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(issues
            .into_iter()
            .map(|issue| format!("{} {}", issue.code, issue.message))
            .collect())
    }
}
