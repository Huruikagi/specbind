//! Project-wide Spec and milestone scope reads.

use std::fmt::Write as _;

use super::super::{
    CommandOutput, Path, SpecHealth, config, escape, milestone_scope as milestone_scope_model,
    render_issue, render_milestone_diagnostic, spec_list as spec_list_model, spec_status,
};
use super::present;
use crate::{configuration, lifecycle::adoption, milestone_status};

fn render_drive_defaults(output: &mut String, drive: crate::schema::drive::v1::DriveDefaults) {
    let _ = writeln!(
        output,
        "  Drive defaults:\n    Replan: {} ({})\n    Recovery: {} ({})",
        drive.replan_enabled(),
        if drive.replan.is_some() {
            "configured"
        } else {
            "legacy default"
        },
        drive.recovery_mode().name(),
        if drive.recovery.is_some() {
            "configured"
        } else {
            "legacy default"
        },
    );
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
    let _ = writeln!(
        output,
        "    Adoption Skill: {}",
        if report.adoption {
            "enabled"
        } else {
            "disabled"
        }
    );
    render_drive_defaults(&mut output, report.drive);
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
    let preflight = match adoption::resolve(&paths.project_root, &paths.specbind_root) {
        Ok(preflight) => preflight,
        Err(error) => return render_adoption_failure(error),
    };
    match preflight.readiness {
        adoption::Readiness::Initial { revision } => CommandOutput::success(
            format!(
                "OK ADOPTION_PREFLIGHT_READY: Existing-project adoption can begin.\n  source_revision: {}\n  steering_documents: {}\n",
                escape(&revision), preflight.steering_count,
            ).into_bytes(),
        ),
        adoption::Readiness::Resume(status) => render_reverse_resume(&status, preflight.steering_count),
    }
}

fn render_adoption_failure(error: adoption::PreflightFailure) -> CommandOutput {
    let details = match error.details {
        adoption::FailureDetails::Messages(messages) => messages,
        adoption::FailureDetails::Artifacts(issues) => issues.iter().map(render_issue).collect(),
        adoption::FailureDetails::Milestone {
            diagnostics,
            blockers,
        } => {
            let mut details = diagnostics
                .iter()
                .map(render_milestone_diagnostic)
                .collect::<Vec<_>>();
            details.extend(blockers);
            details
        }
        adoption::FailureDetails::Source(issues) => issues
            .iter()
            .map(|issue| {
                let path = issue
                    .path
                    .as_ref()
                    .map_or_else(String::new, |path| format!(" {path}:"));
                format!("{}{path} {}", issue.code, issue.message)
            })
            .collect(),
    };
    CommandOutput::failure(error.code, error.message, details)
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
        output.push_str(&super::super::render_description(&entry.description));
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

#[cfg(test)]
mod tests {
    use super::*;
    use adoption::{FailureDetails, PreflightFailure};

    #[test]
    fn adoption_failures_preserve_detail_order_escaping_and_stderr_routing() {
        let cases = [
            (
                FailureDetails::Messages(vec!["first\nsecond".to_owned()]),
                "  first\\nsecond\n",
            ),
            (
                FailureDetails::Artifacts(vec![crate::artifacts::DiscoveryIssue {
                    code: "ARTIFACT",
                    path: Some("steering/project.md".into()),
                    message: "bad\tvalue".to_owned(),
                }]),
                "  ARTIFACT steering/project.md: bad\\tvalue\n",
            ),
            (
                FailureDetails::Milestone {
                    diagnostics: vec![milestone_status::MilestoneDiagnostic {
                        code: "STATE",
                        path: None,
                        message: "bad\nstate".to_owned(),
                    }],
                    blockers: vec!["blocker\tvalue".to_owned()],
                },
                "  STATE bad\\nstate\n  blocker\\tvalue\n",
            ),
            (
                FailureDetails::Source(vec![crate::adoption_finalize::FinalizeIssue {
                    code: "SOURCE",
                    path: Some("source\tfile".to_owned()),
                    message: "changed\nvalue".to_owned(),
                }]),
                "  SOURCE source\\tfile: changed\\nvalue\n",
            ),
        ];
        for (details, expected) in cases {
            let output = render_adoption_failure(PreflightFailure {
                code: "ADOPTION_FAILED",
                message: "Cannot continue.",
                details,
            });
            assert!(!output.success);
            assert_eq!(output.stdout, b"");
            assert_eq!(
                String::from_utf8(output.stderr).unwrap(),
                format!("ERROR ADOPTION_FAILED: Cannot continue.\n{expected}"),
            );
        }
    }
}
