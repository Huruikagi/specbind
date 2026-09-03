//! Project-wide Spec and milestone scope reads.

use std::{fmt::Write as _, fs};

use serde::Deserialize;

use super::super::{
    CommandOutput, Path, SpecHealth, config, escape, milestone_scope as milestone_scope_model,
    render_issue, render_milestone_diagnostic, spec_list as spec_list_model, spec_status, steering,
};
use super::present;
use crate::{adoption_finalize, configuration, guarded_fs, milestone_status, repository};

#[derive(Deserialize)]
struct AdoptionResumeRecord {
    source_revision: String,
}

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
    match reverse_resume_preflight(&paths.project_root, &paths.specbind_root, steering_count) {
        Ok(None) => {}
        Ok(Some(output)) | Err(output) => return output,
    }
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

fn reverse_resume_preflight(
    project_root: &Path,
    specbind_root: &Path,
    steering_count: usize,
) -> Result<Option<CommandOutput>, CommandOutput> {
    let roadmap = milestone_status::read_roadmap(specbind_root).map_err(|error| {
        CommandOutput::failure(
            "ADOPTION_MILESTONE_INVALID",
            "Cannot inspect the active milestone for reverse establishment.",
            error
                .diagnostics
                .iter()
                .map(render_milestone_diagnostic)
                .collect(),
        )
    })?;
    let Some(roadmap) = roadmap else {
        return Ok(None);
    };
    if roadmap.reverse_specs.is_empty()
        || roadmap.baseline_version.is_none()
        || roadmap.target_release.is_some()
    {
        return Err(CommandOutput::failure(
            "ADOPTION_MILESTONE_ACTIVE",
            "Existing-project adoption cannot start while a delivery milestone is active.",
            vec![],
        ));
    }

    let record = read_adoption_resume_record(specbind_root)?;
    if record.source_revision != roadmap.baseline_revision {
        return Err(CommandOutput::failure(
            "ADOPTION_RESUME_RECORD_MISMATCH",
            "Temporary adoption evidence does not match the active reverse milestone.",
            vec!["adoption/reverse-discovery.yaml: source_revision does not match roadmap baseline_revision".to_owned()],
        ));
    }
    let status = resolve_reverse_resume_status(project_root, specbind_root)?;
    Ok(Some(render_reverse_resume(&status, steering_count)))
}

fn read_adoption_resume_record(
    specbind_root: &Path,
) -> Result<AdoptionResumeRecord, CommandOutput> {
    let record_path = "adoption/reverse-discovery.yaml";
    let path = specbind_root.join(record_path);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        CommandOutput::failure(
            "ADOPTION_RESUME_RECORD_REQUIRED",
            "Active reverse establishment requires its temporary adoption record.",
            vec![format!("{record_path}: {error}")],
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(CommandOutput::failure(
            "ADOPTION_RESUME_RECORD_INVALID",
            "Cannot trust the temporary adoption record for reverse resumption.",
            vec![format!(
                "{record_path}: expected a regular non-symlink file"
            )],
        ));
    }
    let source = fs::read_to_string(&path).map_err(|error| {
        CommandOutput::failure(
            "ADOPTION_RESUME_RECORD_INVALID",
            "Cannot read the temporary adoption record for reverse resumption.",
            vec![format!("{record_path}: {error}")],
        )
    })?;
    let record: AdoptionResumeRecord = serde_saphyr::from_str(&source).map_err(|error| {
        CommandOutput::failure(
            "ADOPTION_RESUME_RECORD_INVALID",
            "Cannot parse the temporary adoption record for reverse resumption.",
            vec![format!("{record_path}: {error}")],
        )
    })?;
    Ok(record)
}

fn resolve_reverse_resume_status(
    project_root: &Path,
    specbind_root: &Path,
) -> Result<milestone_status::MilestoneStatusModel, CommandOutput> {
    let _current_revision = adoption_revision(project_root)?;
    let status = milestone_status::resolve(project_root, specbind_root)
        .map_err(|error| {
            CommandOutput::failure(
                "ADOPTION_RESUME_STATE_INVALID",
                "Cannot derive a safe reverse-establishment resume point.",
                error
                    .diagnostics
                    .iter()
                    .map(render_milestone_diagnostic)
                    .collect(),
            )
        })?
        .ok_or_else(|| {
            CommandOutput::failure(
                "ADOPTION_RESUME_STATE_INVALID",
                "The active reverse milestone disappeared during preflight.",
                vec![],
            )
        })?;
    if status.health != milestone_status::MilestoneHealth::Consistent
        || !status.current_blockers.is_empty()
    {
        let mut details = status
            .diagnostics
            .iter()
            .map(render_milestone_diagnostic)
            .collect::<Vec<_>>();
        details.extend(status.current_blockers.iter().cloned());
        return Err(CommandOutput::failure(
            "ADOPTION_RESUME_STATE_INVALID",
            "Active reverse establishment is not safe to resume.",
            details,
        ));
    }
    adoption_finalize::ensure_source_unchanged(
        project_root,
        specbind_root,
        &status.baseline_revision,
        &status
            .items
            .iter()
            .map(|item| item.id.clone())
            .collect::<Vec<_>>(),
    )
    .map_err(|error| {
        CommandOutput::failure(
            "ADOPTION_RESUME_SOURCE_STALE",
            "Implementation evidence changed after reverse establishment began.",
            error
                .issues
                .iter()
                .map(|issue| {
                    let path = issue
                        .path
                        .as_ref()
                        .map_or_else(String::new, |path| format!(" {path}:"));
                    format!("{}{path} {}", issue.code, issue.message)
                })
                .collect(),
        )
    })?;

    Ok(status)
}

fn render_reverse_resume(
    status: &milestone_status::MilestoneStatusModel,
    steering_count: usize,
) -> CommandOutput {
    let actionable = if status.actionable.is_empty() {
        "none".to_owned()
    } else {
        status
            .actionable
            .iter()
            .map(|action| format!("{}:{}", action.action.name(), action.item))
            .collect::<Vec<_>>()
            .join(", ")
    };
    CommandOutput::success(
        format!(
            "OK ADOPTION_RESUME_READY: Active reverse establishment can resume.\n  milestone_id: {}\n  source_revision: {}\n  baseline_version: {}\n  stage: {}\n  actionable: {}\n  steering_documents: {steering_count}\n",
            escape(&status.milestone_id),
            escape(&status.baseline_revision),
            escape(status.baseline_version.as_deref().expect("reverse version checked")),
            milestone_status::stage_name(status.stage),
            escape(&actionable),
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
    let record = "adoption/reverse-discovery.yaml";
    match fs::symlink_metadata(specbind_root.join(record)) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Ok(_) => {
            return Err(CommandOutput::failure(
                "ADOPTION_RECORD_PRESENT",
                "Existing-project adoption cannot start from a pre-existing temporary record.",
                vec![format!(
                    "inspect and explicitly reconcile the unsupported record before retrying: {record}"
                )],
            ));
        }
        Err(error) => {
            return Err(CommandOutput::failure(
                "ADOPTION_RECORD_UNREADABLE",
                "Cannot verify that no temporary adoption record exists.",
                vec![format!("{record}: {error}")],
            ));
        }
    }

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
