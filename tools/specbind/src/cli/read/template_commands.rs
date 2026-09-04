//! Project-bound Spec, Milestone, and Steering template catalog commands.

use super::super::*;
use super::project_relative_spec_root;

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
pub fn template_list_milestone(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let inventory = template::discover_milestone_templates(&paths.specbind_root, paths.language);
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .templates
            .iter()
            .map(render_milestone_template)
            .collect::<Vec<_>>();
        details.extend(inventory.issues.iter().map(render_issue));
        return CommandOutput::failure(
            "TEMPLATE_LIST_FAILED",
            "Milestone template inventory has diagnostics.",
            details,
        );
    }
    let mut output = format!(
        "OK TEMPLATE_LISTED: Found {} recognized milestone template(s).\n",
        inventory.templates.len()
    );
    for template in &inventory.templates {
        output.push_str("  ");
        output.push_str(&render_milestone_template(template));
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn template_read_milestone(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match template::read_milestone_template(&paths.specbind_root, paths.language, selector) {
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
                format!("Selector {selector} does not resolve to a readable milestone template."),
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
    let spec_root = match project_relative_spec_root(&paths) {
        Ok(root) => root,
        Err(output) => return output,
    };
    let project_path = format!("{spec_root}/{target_path}");
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
    push_field(&mut output, "Project path", &project_path);
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn template_list_steering(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let spec_root = match project_relative_spec_root(&paths) {
        Ok(root) => root,
        Err(output) => return output,
    };
    let inventory = template::discover_steering_templates(&paths.specbind_root, paths.language);
    if !inventory.issues.is_empty() {
        let mut details = inventory
            .templates
            .iter()
            .map(|template| render_steering_template(template, &spec_root))
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
        output.push_str(&render_steering_template(template, &spec_root));
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
fn render_steering_template(template: &template::SteeringTemplate, spec_root: &str) -> String {
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
        Some(output_path) => write!(
            output,
            " output_path={} project_path={}",
            escape(output_path.as_str()),
            escape(&format!("{spec_root}/{output_path}"))
        ),
        None => write!(
            output,
            " output_path=<authored> project_path={}",
            escape(&format!("{spec_root}/steering/<artifact_id>.md"))
        ),
    }
    .expect("writing to a String cannot fail");
    output
}

fn render_milestone_template(template: &template::MilestoneTemplate) -> String {
    format!(
        "selector={} source={} type=\"{}\" template_path={} body_target=steering/roadmap.md",
        escape(&template.selector),
        template.source.name(),
        escape(&template.artifact_type),
        escape(template.template_path.as_str())
    )
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
