//! CLI execution and rendering for read surfaces and installation.

use super::*;

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
pub fn artifact_read(start: &Path, canonical_spec: &str, selector: &str) -> CommandOutput {
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
        Ok(bytes) if std::str::from_utf8(&bytes).is_ok() => {
            let mut stderr = String::new();
            for issue in &inventory.issues {
                stderr.push_str("  ");
                stderr.push_str(&render_issue(issue));
                stderr.push('\n');
            }
            CommandOutput {
                stdout: bytes,
                stderr: stderr.into_bytes(),
                success: true,
            }
        }
        Ok(_) => CommandOutput::failure(
            "ARTIFACT_READ_NOT_UTF8",
            "Resolved artifact is not valid UTF-8.",
            vec![],
        ),
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

/// Reports the installation plan without touching the filesystem.
#[must_use]
pub fn install_dry_run(start: &Path, inputs: &install::InstallInputs) -> CommandOutput {
    let project_root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match install::plan(&project_root, inputs) {
        Ok(plan) => {
            let (create, replace, keep) = plan.counts();
            let mut output = format!(
                "OK INSTALL_PLANNED: Planned {} action(s) for {} agent(s).\n",
                plan.entries.len(),
                plan.agents.len()
            );
            push_install_summary(&mut output, &plan);
            push_field(
                &mut output,
                "Summary",
                &format!("{create} create, {replace} replace, {keep} keep"),
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
            let (create, replace, keep) = outcome.plan.counts();
            let mut output = format!(
                "OK INSTALL_APPLIED: Applied {} action(s) for {} agent(s).
",
                create + replace,
                outcome.plan.agents.len()
            );
            push_install_summary(&mut output, &outcome.plan);
            push_field(
                &mut output,
                "Summary",
                &format!("{create} created, {replace} replaced, {keep} kept"),
            );
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
        match entry.present(&paths.specbind_root) {
            Ok(installed) => details.push(format!(
                "selector={} type=\"{}\" path={} present={}",
                escape(entry.selector),
                escape(entry.artifact_type),
                escape(&entry.path()),
                present(installed)
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
pub fn steering_read(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match steering::read(&paths.specbind_root, selector) {
        Ok(content) => CommandOutput::success(content.into_bytes()),
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

/// Lists every persistent Spec in the project.
///
/// A Spec whose machine state cannot be read is listed with its fault named
/// rather than failing the command, because this listing is how an agent
/// discovers that the Spec needs repair.
#[must_use]
pub fn spec_list(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let entries = match spec_list::resolve(&paths.specbind_root) {
        Ok(entries) => entries,
        Err(error) => {
            return CommandOutput::failure(
                "SPEC_LIST_FAILED",
                "Cannot enumerate persistent specs.",
                vec![error.message],
            );
        }
    };
    let mut output = format!("OK SPEC_LISTED: Found {} spec(s).\n", entries.len());
    for entry in &entries {
        output.push_str("  ");
        output.push_str(&escape(&entry.canonical_spec));
        match &entry.health {
            SpecHealth::Unreadable(reason) => {
                output.push_str(": unreadable: ");
                output.push_str(&escape(reason));
            }
            SpecHealth::Readable => {
                output.push_str(": state=");
                output.push_str(spec_status::state_name(entry.declared_state));
                output.push_str(" milestone=");
                output.push_str(&escape(entry.milestone_id.as_deref().unwrap_or("none")));
                output.push_str(" requirements=");
                output.push_str(present(entry.has_requirements));
                output.push_str(" contract=");
                output.push_str(present(entry.has_contract));
            }
        }
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

fn present(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

/// Writes the active milestone's scope as a replacement candidate document.
///
/// This is a raw-content read in the same family as `artifact read`: the
/// document goes to stdout with no result wrapper, so it can be piped straight
/// back into `milestone update-scope --scope -`.
#[must_use]
pub fn milestone_scope(start: &Path, include_body: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone_scope::resolve(&paths.specbind_root, include_body) {
        Ok(Some(document)) => CommandOutput::success(document.into_bytes()),
        Ok(None) => CommandOutput::no_change("NO_ACTIVE_MILESTONE", "No active milestone exists."),
        Err(error) => CommandOutput::failure(
            "MILESTONE_SCOPE_FAILED",
            "Cannot read the active milestone scope.",
            error
                .diagnostics
                .iter()
                .map(render_milestone_diagnostic)
                .collect(),
        ),
    }
}
