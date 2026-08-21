//! Artifact inventory, raw reads, and validation checks.

use super::super::*;

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
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
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
pub fn check_traceability(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let resolution = artifacts::resolve_traceability(&paths.specbind_root, canonical_spec);
    let mut details = resolution
        .inventory
        .issues
        .iter()
        .map(render_issue)
        .collect::<Vec<_>>();
    let Some(report) = resolution.report else {
        return CommandOutput::failure(
            "TRACEABILITY_FAILED",
            format!("Cannot verify traceability for spec {canonical_spec}."),
            details,
        );
    };
    details.extend(report.issues.iter().map(render_traceability_issue));
    if !details.is_empty() {
        return CommandOutput::failure(
            "TRACEABILITY_FAILED",
            format!("Traceability for spec {canonical_spec} has diagnostics."),
            details,
        );
    }
    let mut output = format!(
        "OK TRACEABILITY_VERIFIED: Verified traceability for spec {}.\n",
        escape(canonical_spec)
    );
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
            push_field(
                &mut output,
                "Design coverage",
                &format!("{design}/{}", active.len()),
            );
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
        None => push_field(&mut output, "Active requirement IDs", "none"),
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn check_contracts(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let graph = contract_graph::resolve(&paths.specbind_root);
    let mut errors = graph
        .project_issues
        .iter()
        .map(render_issue)
        .collect::<Vec<_>>();
    for (spec, inventory) in &graph.inventories {
        errors.extend(
            inventory
                .issues
                .iter()
                .map(|issue| format!("{} spec {spec}", render_issue(issue))),
        );
    }
    errors.extend(
        graph
            .report
            .issues
            .iter()
            .filter(|issue| issue.severity == GraphIssueSeverity::Error)
            .map(render_graph_issue),
    );
    if !errors.is_empty() {
        return CommandOutput::failure(
            "CONTRACTS_FAILED",
            "Contract graph has structural diagnostics.",
            errors,
        );
    }
    let warnings = graph
        .report
        .issues
        .iter()
        .filter(|issue| issue.severity == GraphIssueSeverity::Warning)
        .map(render_graph_issue)
        .collect::<Vec<_>>();
    let mut output = format!(
        "OK CONTRACTS_VERIFIED: Verified {} contract(s) and {} dependency reference(s).\n",
        graph.report.contracts.len(),
        graph.report.dependencies.len()
    );
    push_field(
        &mut output,
        "Ownership findings",
        &graph.report.ownership_findings.len().to_string(),
    );
    push_field(
        &mut output,
        "Dependency cycles",
        &graph.report.dependency_cycles.len().to_string(),
    );
    if warnings.is_empty() {
        push_field(&mut output, "Warnings", "none");
    } else {
        output.push_str("  Warnings:\n");
        for warning in warnings {
            writeln!(output, "    - {warning}").expect("writing to a String cannot fail");
        }
    }
    CommandOutput::success(output.into_bytes())
}

fn render_traceability_issue(issue: &crate::traceability::TraceabilityIssue) -> String {
    let source = issue
        .source
        .as_ref()
        .map_or_else(String::new, |source| format!(" {}:", escape(source)));
    format!("{}{source} {}", issue.code, escape(&issue.message))
}

fn render_graph_issue(issue: &contract_graph::ContractGraphIssue) -> String {
    let source = issue
        .source
        .as_ref()
        .map_or_else(String::new, |source| format!(" {}:", escape(source)));
    format!("{}{source} {}", issue.code, escape(&issue.message))
}
