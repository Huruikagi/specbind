//! Artifact inventory, raw reads, and validation checks.

use std::{fs, path::Path};

use super::super::{
    CommandOutput,
    output::{escape, push_field},
    render_artifact, render_issue,
};
use crate::{artifacts, config, instruction, template};

/// Checks creation obligations against an explicitly selected scaffold.
#[must_use]
pub fn artifact_check(
    start: &Path,
    canonical_spec: &str,
    selector: &str,
    template_selector: &str,
) -> CommandOutput {
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
            "Artifact selector does not resolve.",
            inventory.issues.iter().map(render_issue).collect(),
        );
    };
    let mut details: Vec<_> = inventory
        .issues
        .iter()
        .filter(|issue| issue.path.as_ref() == Some(&artifact.path))
        .map(render_issue)
        .collect();
    let (scaffold, templates) =
        match template::read_spec_template(&paths.specbind_root, paths.language, template_selector)
        {
            Ok(result) => result,
            Err(inventory) => {
                return CommandOutput::failure(
                    "ARTIFACT_CHECK_FAILED",
                    "Cannot resolve selected template.",
                    inventory.issues.iter().map(render_issue).collect(),
                );
            }
        };
    let selected = templates
        .templates
        .iter()
        .find(|template| template.selector == template_selector);
    if !selected.is_some_and(|template| {
        template.kind == artifact.kind
            && artifact.kind != artifacts::ArtifactKind::Contract
            && (template.artifact_id == artifact.artifact_id
                || (artifact.kind == artifacts::ArtifactKind::Design
                    && template.artifact_id.as_deref() == Some("main")))
    }) {
        return CommandOutput::failure(
            "ARTIFACT_CHECK_FAILED",
            "Template and live artifact must have the same Markdown profile.",
            vec![],
        );
    }
    details.extend(templates.issues.iter().map(render_issue));
    let native_path = paths.specbind_root.join(artifact.path.as_std_path());
    if !fs::symlink_metadata(&native_path)
        .is_ok_and(|metadata| crate::guarded_fs::is_regular_file(&metadata))
    {
        return CommandOutput::failure(
            "ARTIFACT_CHECK_FAILED",
            "Artifact must remain a regular non-link file.",
            details,
        );
    }
    let live = match fs::read_to_string(native_path) {
        Ok(live) => live,
        Err(error) => {
            return CommandOutput::failure("ARTIFACT_CHECK_FAILED", error.to_string(), details);
        }
    };
    details.extend(
        instruction::verify_materialization(&scaffold, &live)
            .iter()
            .map(|issue| format!("{}: {}", issue.code, issue.message)),
    );
    if !details.is_empty() {
        return CommandOutput::failure(
            "ARTIFACT_CHECK_FAILED",
            "Artifact does not conform to the selected scaffold.",
            details,
        );
    }
    CommandOutput::success(
        format!(
            "OK ARTIFACT_CHECKED: {}/{} conforms to scaffold {}.\n",
            escape(canonical_spec),
            escape(selector),
            escape(template_selector)
        )
        .into_bytes(),
    )
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
pub fn artifact_read(
    start: &Path,
    canonical_spec: &str,
    selector: &str,
    purpose: Option<&str>,
) -> CommandOutput {
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
        .is_ok_and(|metadata| crate::guarded_fs::is_regular_file(&metadata))
    {
        return CommandOutput::failure(
            "ARTIFACT_READ_TARGET_INVALID",
            "Resolved artifact is no longer a regular non-symlink file.",
            vec![],
        );
    }
    match fs::read(path) {
        Ok(bytes) => {
            let Ok(content) = String::from_utf8(bytes) else {
                return CommandOutput::failure(
                    "ARTIFACT_READ_NOT_UTF8",
                    "Resolved artifact is not valid UTF-8.",
                    vec![],
                );
            };
            let mut stderr = String::new();
            for issue in &inventory.issues {
                stderr.push_str("  ");
                stderr.push_str(&render_issue(issue));
                stderr.push('\n');
            }
            let stdout = match purpose {
                Some("maintain") => {
                    instruction::project(&content, instruction::InstructionScope::Maintain)
                        .into_bytes()
                }
                Some("consume") => {
                    instruction::project(&content, instruction::InstructionScope::Consume)
                        .into_bytes()
                }
                _ => content.into_bytes(),
            };
            CommandOutput {
                stdout,
                stderr: stderr.into_bytes(),
                success: true,
            }
        }
        Err(error) => CommandOutput::failure("ARTIFACT_READ_FAILED", error.to_string(), vec![]),
    }
}

#[must_use]
pub fn check_traceability(start: &Path, canonical_spec: &str, for_design: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let resolution = if for_design {
        artifacts::resolve_design_traceability(&paths.specbind_root, canonical_spec)
    } else {
        artifacts::resolve_traceability(&paths.specbind_root, canonical_spec)
    };
    let failure_code = if for_design {
        "TRACEABILITY_DESIGN_FAILED"
    } else {
        "TRACEABILITY_FAILED"
    };
    let subject = if for_design {
        "Design traceability"
    } else {
        "Traceability"
    };
    let mut details = resolution
        .inventory
        .issues
        .iter()
        .map(render_issue)
        .collect::<Vec<_>>();
    let Some(report) = resolution.report else {
        return CommandOutput::failure(
            failure_code,
            format!("Cannot verify {subject} for spec {canonical_spec}."),
            details,
        );
    };
    details.extend(report.issues.iter().map(render_traceability_issue));
    if !details.is_empty() {
        return CommandOutput::failure(
            failure_code,
            format!("{subject} for spec {canonical_spec} has diagnostics."),
            details,
        );
    }
    CommandOutput::success(
        render_traceability_success(canonical_spec, &report, for_design).into_bytes(),
    )
}

fn render_traceability_success(
    canonical_spec: &str,
    report: &crate::traceability::TraceabilityReport,
    for_design: bool,
) -> String {
    let mut output = if for_design {
        format!(
            "OK TRACEABILITY_DESIGN_VERIFIED: Verified Design traceability for spec {}.\n",
            escape(canonical_spec)
        )
    } else {
        format!(
            "OK TRACEABILITY_VERIFIED: Verified traceability for spec {}.\n",
            escape(canonical_spec)
        )
    };
    if for_design {
        push_field(&mut output, "Scope", "Requirements and Design");
    }
    push_field(
        &mut output,
        "Requirements",
        &report.requirement_ids.len().to_string(),
    );
    match &report.active_requirement_ids {
        Some(active) => {
            let active_set = active.iter().collect::<std::collections::BTreeSet<_>>();
            let design = report
                .design_requirement_ids
                .iter()
                .filter(|id| active_set.contains(id))
                .count();
            let tasks = report
                .task_requirement_ids
                .iter()
                .filter(|id| active_set.contains(id))
                .count();
            push_field(
                &mut output,
                "Active requirement IDs",
                &active.len().to_string(),
            );
            push_field(&mut output, "Active requirement set", &active.join(", "));
            push_field(
                &mut output,
                "Design coverage",
                &format!("{design}/{}", active.len()),
            );
            if for_design {
                push_field(&mut output, "Task coverage", "not evaluated (Design scope)");
            } else {
                push_field(
                    &mut output,
                    "Task coverage",
                    &format!(
                        "{tasks}/{} ({})",
                        active.len(),
                        if report.tasks_required {
                            "required"
                        } else {
                            "not required"
                        }
                    ),
                );
            }
        }
        None => push_field(&mut output, "Active requirement IDs", "none"),
    }
    if !report.retired_requirement_ids.is_empty() {
        push_field(
            &mut output,
            "Retired requirement IDs",
            &report.retired_requirement_ids.join(", "),
        );
    }
    output
}

fn render_traceability_issue(issue: &crate::traceability::TraceabilityIssue) -> String {
    let source = issue
        .source
        .as_ref()
        .map_or_else(String::new, |source| format!(" {}:", escape(source)));
    format!("{}{source} {}", issue.code, escape(&issue.message))
}
