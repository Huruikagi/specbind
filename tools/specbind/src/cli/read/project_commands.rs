//! Project-wide Spec and milestone scope reads.

use std::fmt::Write as _;

use super::super::{
    CommandOutput, Path, SpecHealth, config, escape, milestone_scope as milestone_scope_model,
    render_issue, render_milestone_diagnostic, spec_list as spec_list_model, spec_status, steering,
};
use super::present;
use crate::{configuration, repository};

/// Validates and summarizes every supported project configuration surface.
#[must_use]
pub fn configuration_show(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let report = match configuration::resolve(&paths.project_root) {
        Ok(report) => report,
        Err(error) => {
            return CommandOutput::failure(
                "CONFIGURATION_SHOW_FAILED",
                "Cannot summarize the current SpecBind configuration.",
                error.diagnostics,
            );
        }
    };
    let language = match report.language {
        config::ProjectLanguage::En => "en",
        config::ProjectLanguage::Ja => "ja",
    };
    let mut output =
        String::from("OK CONFIGURATION_SHOWN: Current SpecBind configuration.\n  Project:\n");
    let _ = writeln!(output, "    Spec directory: {}", escape(&report.spec_dir));
    let _ = writeln!(output, "    Language: {language}");
    let _ = writeln!(
        output,
        "    Agents: {}",
        report
            .agents
            .iter()
            .map(|agent| agent.name())
            .collect::<Vec<_>>()
            .join(", ")
    );
    let _ = writeln!(
        output,
        "    Project instructions: {}",
        if report.project_instructions {
            "enabled"
        } else {
            "disabled"
        }
    );
    output.push_str("  Agent roles:\n");
    if report.roles.is_empty() {
        output.push_str("    none\n");
    }
    for role in &report.roles {
        let _ = write!(
            output,
            "    {}/{}: state={} model={}",
            role.agent.name(),
            role.selector,
            if role.overridden {
                "overridden"
            } else {
                "default"
            },
            escape(&role.model)
        );
        if let Some(effort) = role.reasoning_effort {
            let _ = write!(output, " reasoning_effort={}", effort.name());
        }
        output.push('\n');
    }
    render_configuration_items(&mut output, "Templates", &report.templates);
    render_configuration_items(&mut output, "Rules", &report.rules);
    render_configuration_items(&mut output, "Adapters", &report.adapters);
    let _ = write!(
        output,
        "  Steering:\n    Documents: {}\n",
        report.steering_documents
    );
    output.push_str("  Attention:\n");
    if report.attention.is_empty() {
        output.push_str("    none\n");
    } else {
        for attention in &report.attention {
            let _ = writeln!(output, "    - {}", escape(attention));
        }
    }
    CommandOutput::success(output.into_bytes())
}

fn render_configuration_items(
    output: &mut String,
    heading: &str,
    items: &[configuration::ConfigurationItem],
) {
    let _ = writeln!(output, "  {heading}:");
    for item in items {
        let _ = writeln!(output, "    {}: {}", escape(&item.selector), item.state);
    }
}

/// Returns the immutable Git revision against which an existing implementation
/// may be inspected for adoption into new Specs.
///
/// This command owns only deterministic prerequisites. The adoption skill owns
/// semantic Steering coverage, repository investigation, boundary proposals,
/// and user reconciliation.
#[must_use]
pub fn adoption_preflight(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let steering_count = match adoption_steering_count(&paths.specbind_root) {
        Ok(count) => count,
        Err(output) => return output,
    };
    if let Err(output) = ensure_initial_adoption_scope(&paths.specbind_root) {
        return output;
    }
    let revision = match adoption_revision(&paths.project_root) {
        Ok(revision) => revision,
        Err(output) => return output,
    };

    CommandOutput::success(
        format!(
            "OK ADOPTION_PREFLIGHT_READY: Existing-project adoption can begin.\n  source_revision: {}\n  steering_documents: {steering_count}\n",
            escape(&revision),
        )
        .into_bytes(),
    )
}

fn adoption_steering_count(specbind_root: &Path) -> Result<usize, CommandOutput> {
    let inventory = match steering::discover(specbind_root) {
        Ok(inventory) => inventory,
        Err(message) => {
            return Err(CommandOutput::failure(
                "ADOPTION_STEERING_INVALID",
                "Cannot establish the Steering baseline for adoption.",
                vec![message],
            ));
        }
    };
    if !inventory.issues.is_empty() {
        return Err(CommandOutput::failure(
            "ADOPTION_STEERING_INVALID",
            "Cannot establish the Steering baseline for adoption.",
            inventory.issues.iter().map(render_issue).collect(),
        ));
    }
    if inventory.documents.is_empty() {
        return Err(CommandOutput::failure(
            "ADOPTION_STEERING_REQUIRED",
            "Existing-project adoption requires a non-empty Steering baseline.",
            vec![
                "Run sb-steering in bootstrap mode, review it, and commit it before retrying."
                    .to_owned(),
            ],
        ));
    }
    Ok(inventory.documents.len())
}

fn ensure_initial_adoption_scope(specbind_root: &Path) -> Result<(), CommandOutput> {
    let specs = match spec_list_model::resolve(specbind_root) {
        Ok(specs) => specs,
        Err(error) => {
            return Err(CommandOutput::failure(
                "ADOPTION_SPEC_LIST_FAILED",
                "Cannot verify the initial adoption scope.",
                vec![error.message],
            ));
        }
    };
    if !specs.is_empty() {
        return Err(CommandOutput::failure(
            "ADOPTION_SPECS_PRESENT",
            "Initial existing-project adoption requires a project with no persistent Specs.",
            specs
                .iter()
                .map(|spec| format!("existing spec: {}", spec.canonical_spec))
                .collect(),
        ));
    }

    match milestone_scope_model::resolve(specbind_root, false) {
        Ok(None) => Ok(()),
        Ok(Some(_)) => Err(CommandOutput::failure(
            "ADOPTION_MILESTONE_ACTIVE",
            "Existing-project adoption cannot start while a milestone is active.",
            vec![],
        )),
        Err(error) => Err(CommandOutput::failure(
            "ADOPTION_MILESTONE_INVALID",
            "Cannot verify that no milestone is active.",
            error
                .diagnostics
                .iter()
                .map(render_milestone_diagnostic)
                .collect(),
        )),
    }
}

fn adoption_revision(project_root: &Path) -> Result<String, CommandOutput> {
    let status = match repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    ) {
        Ok(status) => status,
        Err(error) => {
            return Err(CommandOutput::failure(
                "ADOPTION_GIT_FAILED",
                "Cannot inspect the repository for adoption.",
                vec![error.to_string()],
            ));
        }
    };
    if !status.is_empty() {
        return Err(CommandOutput::failure(
            "ADOPTION_WORKTREE_DIRTY",
            "Existing-project adoption requires a clean committed repository.",
            vec![
                "Commit or otherwise reconcile the current work without having SpecBind move it."
                    .to_owned(),
            ],
        ));
    }

    match repository::output(project_root, &["rev-parse", "HEAD"]) {
        Ok(revision) if !revision.trim().is_empty() => Ok(revision.trim().to_owned()),
        Ok(_) => Err(CommandOutput::failure(
            "ADOPTION_REVISION_INVALID",
            "Git returned an empty adoption source revision.",
            vec![],
        )),
        Err(error) => Err(CommandOutput::failure(
            "ADOPTION_GIT_FAILED",
            "Cannot resolve the adoption source revision.",
            vec![error.to_string()],
        )),
    }
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
    let entries = match spec_list_model::resolve(&paths.specbind_root) {
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
    match milestone_scope_model::resolve(&paths.specbind_root, include_body) {
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
