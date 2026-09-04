//! Project-bound Steering document commands.

use super::super::{
    CommandOutput, Path, config, escape, instruction, render_issue, steering, template,
};
use super::project_relative_spec_root;

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
