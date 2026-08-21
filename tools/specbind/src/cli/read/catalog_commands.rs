//! Protocol, template, schema, adapter, and steering catalog commands.

use super::super::*;
use super::present;

/// Lists the embedded product protocols.
///
/// Protocols are compiled into this binary, so this command deliberately takes
/// no project path and works without `.specbind.json` or an installation.
#[must_use]
pub fn protocol_list() -> CommandOutput {
    let protocols = protocol::list();
    let mut output = format!(
        "OK PROTOCOL_LISTED: Found {} product protocol(s).
",
        protocols.len()
    );
    for entry in protocols {
        writeln!(
            output,
            "  selector={} purpose=\"{}\"",
            escape(entry.selector),
            escape(entry.purpose)
        )
        .expect("writing to a String cannot fail");
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one embedded product protocol as raw Markdown.
#[must_use]
pub fn protocol_read(selector: &str) -> CommandOutput {
    match protocol::read(selector) {
        Some(entry) => CommandOutput::success(entry.content().as_bytes().to_vec()),
        None => CommandOutput::failure(
            "PROTOCOL_SELECTOR_NOT_FOUND",
            format!("Selector {selector} does not resolve to an embedded product protocol."),
            protocol::list()
                .iter()
                .map(|entry| format!("available selector {}", escape(entry.selector)))
                .collect(),
        ),
    }
}

#[must_use]
pub fn template_list_spec(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = template::discover_spec_templates(&paths.specbind_root, paths.language);
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .templates
            .iter()
            .map(render_template)
            .collect::<Vec<_>>();
        details.extend(inventory.issues.iter().map(render_issue));
        return CommandOutput::failure(
            "TEMPLATE_LIST_FAILED",
            "Spec template inventory has diagnostics.",
            details,
        );
    }
    let mut output = format!(
        "OK TEMPLATE_LISTED: Found {} recognized spec template(s).\n",
        inventory.templates.len()
    );
    for template in &inventory.templates {
        output.push_str("  ");
        output.push_str(&render_template(template));
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn template_read_spec(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match template::read_spec_template(&paths.specbind_root, paths.language, selector) {
        Ok((content, inventory)) => {
            let mut stderr = String::new();
            for issue in &inventory.issues {
                stderr.push_str("  ");
                stderr.push_str(&render_issue(issue));
                stderr.push('\n');
            }
            CommandOutput {
                stdout: content.into_bytes(),
                stderr: stderr.into_bytes(),
                success: true,
            }
        }
        Err(inventory) => {
            let resolved = inventory
                .templates
                .iter()
                .any(|template| template.selector == selector);
            let code = if resolved {
                "TEMPLATE_READ_FAILED"
            } else {
                "TEMPLATE_SELECTOR_NOT_FOUND"
            };
            CommandOutput::failure(
                code,
                format!("Selector {selector} does not resolve to a readable spec template."),
                inventory.issues.iter().map(render_issue).collect(),
            )
        }
    }
}

#[must_use]
pub fn template_resolve_spec(start: &Path, canonical_spec: &str, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let spec = artifacts::resolve_spec(&paths.specbind_root, canonical_spec);
    if spec.wire.is_none() {
        return CommandOutput::failure(
            "TEMPLATE_TARGET_SPEC_INVALID",
            format!("Cannot resolve a template target for spec {canonical_spec}."),
            spec.issues.iter().map(render_issue).collect(),
        );
    }
    let inventory = template::discover_spec_templates(&paths.specbind_root, paths.language);
    if !inventory.issues.is_empty() {
        return CommandOutput::failure(
            "TEMPLATE_RESOLVE_FAILED",
            "Spec template inventory has diagnostics.",
            inventory.issues.iter().map(render_issue).collect(),
        );
    }
    let Some(resolved) = inventory
        .templates
        .iter()
        .find(|template| template.selector == selector)
    else {
        return CommandOutput::failure(
            "TEMPLATE_SELECTOR_NOT_FOUND",
            format!("Selector {selector} does not resolve to a spec template."),
            inventory
                .templates
                .iter()
                .map(|template| format!("available selector {}", escape(&template.selector)))
                .collect(),
        );
    };
    let target_path = format!("specs/{canonical_spec}/{}", resolved.output_path);
    let mut output = format!(
        "OK TEMPLATE_RESOLVED: Resolved template {} for spec {}.\n  Selector: {}\n  Source: {}\n  Type: {}\n",
        escape(selector),
        escape(canonical_spec),
        escape(&resolved.selector),
        resolved.source.name(),
        escape(&resolved.artifact_type),
    );
    if let Some(artifact_id) = &resolved.artifact_id {
        push_field(&mut output, "Artifact ID", artifact_id);
    }
    push_field(
        &mut output,
        "Template path",
        resolved.template_path.as_str(),
    );
    push_field(&mut output, "Output path", resolved.output_path.as_str());
    push_field(&mut output, "Target path", &target_path);
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn template_list_steering(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = template::discover_steering_templates(&paths.specbind_root, paths.language);
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .templates
            .iter()
            .map(render_steering_template)
            .collect::<Vec<_>>();
        details.extend(inventory.issues.iter().map(render_issue));
        return CommandOutput::failure(
            "TEMPLATE_LIST_FAILED",
            "Steering template inventory has diagnostics.",
            details,
        );
    }
    let mut output = format!(
        "OK TEMPLATE_LISTED: Found {} recognized steering template(s).\n",
        inventory.templates.len()
    );
    for template in &inventory.templates {
        output.push_str("  ");
        output.push_str(&render_steering_template(template));
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn template_read_steering(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match template::read_steering_template(&paths.specbind_root, paths.language, selector) {
        Ok((content, inventory)) => {
            let mut stderr = String::new();
            for issue in &inventory.issues {
                stderr.push_str("  ");
                stderr.push_str(&render_issue(issue));
                stderr.push('\n');
            }
            CommandOutput {
                stdout: content.into_bytes(),
                stderr: stderr.into_bytes(),
                success: true,
            }
        }
        Err(inventory) => {
            let resolved = inventory
                .templates
                .iter()
                .any(|template| template.selector == selector);
            let code = if resolved {
                "TEMPLATE_READ_FAILED"
            } else {
                "TEMPLATE_SELECTOR_NOT_FOUND"
            };
            CommandOutput::failure(
                code,
                format!("Selector {selector} does not resolve to a readable steering template."),
                inventory.issues.iter().map(render_issue).collect(),
            )
        }
    }
}

/// Renders one steering template, whose output path is absent exactly when the
/// authoring skill supplies the identity under Decision 0117.
fn render_steering_template(template: &template::SteeringTemplate) -> String {
    let mut output = format!(
        "selector={} source={} type=\"{}\"",
        escape(&template.selector),
        template.source.name(),
        escape(&template.artifact_type)
    );
    if let Some(artifact_id) = &template.artifact_id {
        output.push_str(" artifact_id=");
        output.push_str(&escape(artifact_id));
    }
    write!(
        output,
        " template_path={}",
        escape(template.template_path.as_str())
    )
    .expect("writing to a String cannot fail");
    match &template.output_path {
        Some(output_path) => write!(output, " output_path={}", escape(output_path.as_str())),
        None => write!(output, " output_path=<authored>"),
    }
    .expect("writing to a String cannot fail");
    output
}

fn render_template(template: &template::Template) -> String {
    let mut output = format!(
        "selector={} source={} type=\"{}\"",
        escape(&template.selector),
        template.source.name(),
        escape(&template.artifact_type)
    );
    if let Some(artifact_id) = &template.artifact_id {
        output.push_str(" artifact_id=");
        output.push_str(&escape(artifact_id));
    }
    write!(
        output,
        " template_path={} output_path={}",
        escape(template.template_path.as_str()),
        escape(template.output_path.as_str())
    )
    .expect("writing to a String cannot fail");
    output
}

/// Lists every embedded structured-artifact schema.
///
/// Like the protocols, these are properties of the binary. Taking no project
/// path is the structural guarantee of that rather than a convenience.
#[must_use]
pub fn schema_list() -> CommandOutput {
    let schemas = schema::schemas();
    let mut output = format!(
        "OK SCHEMA_LISTED: Found {} embedded schema(s).\n",
        schemas.len()
    );
    for entry in schemas {
        let _ = writeln!(
            output,
            "  selector={} artifact={} written_by=\"{}\"",
            escape(entry.selector),
            escape(entry.artifact),
            escape(entry.written_by)
        );
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one versioned schema selector as raw JSON.
#[must_use]
pub fn schema_read(selector: &str) -> CommandOutput {
    schema::find_schema(selector).map_or_else(
        || {
            CommandOutput::failure(
                "SCHEMA_READ_INVALID",
                format!("unknown schema selector: {selector}"),
                vec![],
            )
        },
        |entry| CommandOutput::success(entry.content().as_bytes().to_vec()),
    )
}

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
            .map(render_steering)
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
        output.push_str(&render_steering(document));
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
        Err(failure) => CommandOutput::failure(
            failure.code,
            failure.message,
            failure.issues.iter().map(render_issue).collect(),
        ),
    }
}

fn render_steering(document: &steering::SteeringDocument) -> String {
    format!(
        "selector={} type=\"{}\" path={}",
        escape(&document.selector),
        escape(&document.artifact_type),
        escape(document.path.as_str())
    )
}
