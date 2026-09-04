//! Project-bound adapter, rule, and steering catalog commands.

use super::super::*;
use super::{present, project_relative_spec_root};

/// Lists every accepted adapter and whether the project has it.
///
/// The listing enumerates the accepted selectors, never the directory. A file
/// that happens to sit below the adapters root is not an adapter and never
/// becomes one by existing.
#[must_use]
pub fn adapter_list(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let mut details = Vec::new();
    for entry in adapter::all() {
        match entry.state(&paths.specbind_root) {
            Ok(state) => details.push(format!(
                "selector={} type=\"{}\" path={} present={} state={}",
                escape(entry.selector),
                escape(entry.artifact_type),
                escape(&entry.path()),
                present(state != adapter::AdapterState::Absent),
                state.name()
            )),
            Err(error) => {
                return CommandOutput::failure(
                    "ADAPTER_LIST_FAILED",
                    "Cannot inspect project adapters.",
                    vec![format!("{} {}", error.code, error.message)],
                );
            }
        }
    }
    let mut output = format!(
        "OK ADAPTER_LISTED: Found {} accepted adapter(s).\n",
        details.len()
    );
    for detail in details {
        output.push_str("  ");
        output.push_str(&detail);
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one adapter selector as raw UTF-8 Markdown.
///
/// Absence is reported, not judged. Whether a missing adapter is a fault
/// belongs to the consuming skill.
#[must_use]
pub fn adapter_read(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let Some(entry) = adapter::find(selector) else {
        return CommandOutput::failure(
            "ADAPTER_READ_INVALID",
            format!("unknown adapter selector: {selector}"),
            vec![],
        );
    };
    match entry.read(&paths.specbind_root) {
        Ok(Some(content)) => CommandOutput::success(content.into_bytes()),
        Ok(None) => CommandOutput::no_change(
            "ADAPTER_ABSENT",
            &format!("The project has no {selector} adapter."),
        ),
        Err(error) => CommandOutput::failure(error.code, error.message, vec![]),
    }
}

/// Reads active adapter guidance while projecting inactive catalog state.
///
/// Raw reads remain available to configuration workflows. Consumers use this
/// projection so they never need to parse the product-owned scaffold marker.
#[must_use]
pub fn adapter_read_for_consume(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let Some(entry) = adapter::find(selector) else {
        return CommandOutput::failure(
            "ADAPTER_READ_INVALID",
            format!("unknown adapter selector: {selector}"),
            vec![],
        );
    };
    match entry.resolve(&paths.specbind_root) {
        Ok(resolved) => match resolved.state {
            adapter::AdapterState::Absent => CommandOutput::no_change(
                "ADAPTER_ABSENT",
                &format!("The project has no {selector} adapter."),
            ),
            adapter::AdapterState::Scaffold => CommandOutput::no_change(
                "ADAPTER_SCAFFOLD",
                &format!("The project {selector} adapter is an inactive scaffold."),
            ),
            adapter::AdapterState::Active => match resolved.content {
                Some(content) => CommandOutput::success(content.into_bytes()),
                None => CommandOutput::failure(
                    "ADAPTER_READ_FAILED",
                    "Cannot read active project adapter guidance.",
                    vec![],
                ),
            },
        },
        Err(error) => CommandOutput::failure(error.code, error.message, vec![]),
    }
}

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

/// Lists every recognized steering document.
///
/// Any per-document fault returns the unambiguously discovered documents as
/// diagnostic detail and exits nonzero, so a caller never mistakes a partial
/// view of the project's guidance for the whole of it.
#[must_use]
pub fn steering_list(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let spec_root = match project_relative_spec_root(&paths) {
        Ok(root) => root,
        Err(output) => return output,
    };
    let inventory = match steering::discover(&paths.specbind_root) {
        Ok(inventory) => inventory,
        Err(message) => {
            return CommandOutput::failure(
                "STEERING_LIST_FAILED",
                "Cannot enumerate steering documents.",
                vec![message],
            );
        }
    };
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .documents
            .iter()
            .map(|document| render_steering(document, &spec_root))
            .collect::<Vec<_>>();
        details.extend(inventory.issues.iter().map(render_issue));
        return CommandOutput::failure(
            "STEERING_LIST_FAILED",
            "Steering inventory has diagnostics.",
            details,
        );
    }
    let mut output = format!(
        "OK STEERING_LISTED: Found {} steering document(s).\n",
        inventory.documents.len()
    );
    for document in &inventory.documents {
        output.push_str("  ");
        output.push_str(&render_steering(document, &spec_root));
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one steering selector as raw UTF-8 Markdown.
#[must_use]
pub fn steering_read(start: &Path, selector: &str, purpose: Option<&str>) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let spec_root = match project_relative_spec_root(&paths) {
        Ok(root) => root,
        Err(output) => return output,
    };
    match steering::read(&paths.specbind_root, selector) {
        Ok(content) => {
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
        Err(failure) => {
            let message = if failure.code == "STEERING_READ_INVALID" {
                format!(
                    "{}; searched_project_path={}/steering",
                    failure.message, spec_root
                )
            } else {
                failure.message
            };
            CommandOutput::failure(
                failure.code,
                message,
                failure.issues.iter().map(render_issue).collect(),
            )
        }
    }
}

/// Verifies one materialized Steering document against the selected scaffold.
#[must_use]
pub fn steering_check(start: &Path, selector: &str, template_selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = match steering::discover(&paths.specbind_root) {
        Ok(inventory) => inventory,
        Err(message) => return CommandOutput::failure("STEERING_CHECK_FAILED", message, vec![]),
    };
    let Some(document) = inventory
        .documents
        .iter()
        .find(|document| document.selector == selector)
    else {
        return CommandOutput::failure(
            "STEERING_CHECK_INVALID",
            format!("unknown or ambiguous steering selector: {selector}"),
            inventory.issues.iter().map(render_issue).collect(),
        );
    };
    let (scaffold, template_artifact_id) =
        match check_scaffold(&paths.specbind_root, paths.language, template_selector) {
            Ok(value) => value,
            Err(output) => return output,
        };
    if template_artifact_id
        .as_deref()
        .is_some_and(|id| id != selector)
    {
        return CommandOutput::failure(
            "STEERING_CHECK_IDENTITY_MISMATCH",
            format!(
                "template {template_selector} materializes artifact_id {} rather than {selector}",
                template_artifact_id.as_deref().unwrap_or_default()
            ),
            vec![],
        );
    }
    let path = paths.specbind_root.join(document.path.as_str());
    let live = match std::fs::read(&path).map(String::from_utf8) {
        Ok(Ok(content)) => content,
        Ok(Err(_)) => {
            return CommandOutput::failure(
                "STEERING_CHECK_FAILED",
                format!("{} must be UTF-8", document.path),
                vec![],
            );
        }
        Err(error) => {
            return CommandOutput::failure(
                "STEERING_CHECK_FAILED",
                format!("{}: {error}", document.path),
                vec![],
            );
        }
    };
    let mut details = inventory
        .issues
        .iter()
        .map(render_issue)
        .collect::<Vec<_>>();
    details.extend(
        instruction::verify_materialization(&scaffold, &live)
            .into_iter()
            .map(|issue| format!("{}: {}", issue.code, issue.message)),
    );
    details.sort();
    details.dedup();
    if details.is_empty() {
        CommandOutput::success(
            format!("OK STEERING_CHECKED: {selector} conforms to scaffold {template_selector}.\n")
                .into_bytes(),
        )
    } else {
        CommandOutput::failure(
            "STEERING_CHECK_FAILED",
            "Steering document does not conform to the selected scaffold.",
            details,
        )
    }
}

fn check_scaffold(
    specbind_root: &Path,
    language: config::ProjectLanguage,
    template_selector: &str,
) -> Result<(String, Option<String>), CommandOutput> {
    match template::read_steering_template(specbind_root, language, template_selector) {
        Ok((content, inventory)) => {
            let artifact_id = inventory
                .templates
                .iter()
                .find(|candidate| candidate.selector == template_selector)
                .map(|template| template.artifact_id.clone())
                .ok_or_else(|| {
                    CommandOutput::failure(
                        "TEMPLATE_READ_FAILED",
                        format!("Resolved steering template disappeared: {template_selector}"),
                        vec![],
                    )
                })?;
            Ok((content, artifact_id))
        }
        Err(inventory) => {
            let resolved = inventory
                .templates
                .iter()
                .any(|template| template.selector == template_selector);
            Err(CommandOutput::failure(
                if resolved {
                    "TEMPLATE_READ_FAILED"
                } else {
                    "TEMPLATE_SELECTOR_NOT_FOUND"
                },
                format!(
                    "Selector {template_selector} does not resolve to a readable steering template."
                ),
                inventory.issues.iter().map(render_issue).collect(),
            ))
        }
    }
}

fn render_steering(document: &steering::SteeringDocument, spec_root: &str) -> String {
    format!(
        "selector={} type=\"{}\" path={} project_path={}",
        escape(&document.selector),
        escape(&document.artifact_type),
        escape(document.path.as_str()),
        escape(&format!("{spec_root}/{}", document.path))
    )
}
