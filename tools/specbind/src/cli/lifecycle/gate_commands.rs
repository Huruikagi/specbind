//! Gate approval command execution and rendering.

use super::super::{CommandOutput, Path, approval, config, escape, push_field};

#[must_use]
pub fn spec_gate_approve(
    start: &Path,
    canonical_spec: &str,
    gate: approval::Gate,
    approval_mode: &str,
    delegation_workflow: Option<&str>,
    requirement_ids: Option<&str>,
) -> CommandOutput {
    let failure_code = approve_failed_code(gate);
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let mode = match (approval_mode, delegation_workflow) {
        ("explicit", None) => approval::ApprovalMode::Explicit,
        ("explicit", Some(_)) => {
            return CommandOutput::failure(
                failure_code,
                approve_failure_message(gate, canonical_spec),
                vec![
                    "SPEC_GATE_DELEGATION_INVALID explicit approval does not accept a delegation workflow"
                        .to_owned(),
                ],
            );
        }
        ("delegated", Some(workflow)) => approval::ApprovalMode::Delegated {
            workflow: workflow.to_owned(),
        },
        ("delegated", None) => {
            return CommandOutput::failure(
                failure_code,
                approve_failure_message(gate, canonical_spec),
                vec![
                    "SPEC_GATE_DELEGATION_INVALID delegated approval requires --delegation-workflow"
                        .to_owned(),
                ],
            );
        }
        _ => {
            return CommandOutput::failure(
                failure_code,
                approve_failure_message(gate, canonical_spec),
                vec![
                    "SPEC_GATE_APPROVAL_MODE_INVALID approval mode must be explicit or delegated"
                        .to_owned(),
                ],
            );
        }
    };
    let request = approval::ApprovalRequest {
        gate,
        mode,
        requirement_ids: requirement_ids
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|id| !id.is_empty())
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default(),
    };
    match approval::approve(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
        &request,
    ) {
        Ok(approval::ApproveOutcome::Approved(approved)) => CommandOutput::success(
            render_gate_approval(gate, canonical_spec, &approved).into_bytes(),
        ),
        Ok(approval::ApproveOutcome::AlreadyApproved(_)) => CommandOutput::no_change(
            already_approved_code(gate),
            &format!(
                "Spec {} already has identical fresh {} approval.",
                escape(canonical_spec),
                gate.name()
            ),
        ),
        Err(error) => CommandOutput::failure(
            failure_code,
            approve_failure_message(gate, canonical_spec),
            error.issues.iter().map(render_approval_issue).collect(),
        ),
    }
}

#[must_use]
pub fn spec_gate_invalidate(
    start: &Path,
    canonical_spec: &str,
    gate: approval::Gate,
) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match approval::invalidate(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
        gate,
    ) {
        Ok(approval::InvalidateOutcome::Invalidated {
            state,
            review_removed,
        }) => {
            let mut output = format!(
                "OK SPEC_{}_INVALIDATED: Invalidated {} for spec {}.\n",
                gate.name().to_uppercase(),
                gate.name(),
                escape(canonical_spec)
            );
            push_field(&mut output, "State", approval::state_name(state));
            push_field(
                &mut output,
                "Accepted review",
                if review_removed {
                    "removed"
                } else {
                    "unchanged"
                },
            );
            CommandOutput::success(output.into_bytes())
        }
        Ok(approval::InvalidateOutcome::NoChange) => CommandOutput::no_change(
            not_approved_code(gate),
            &format!(
                "Spec {} has no {} approval to invalidate.",
                escape(canonical_spec),
                gate.name()
            ),
        ),
        Err(error) => CommandOutput::failure(
            invalidate_failed_code(gate),
            format!(
                "Cannot invalidate {} for spec {canonical_spec}.",
                gate.name()
            ),
            error.issues.iter().map(render_approval_issue).collect(),
        ),
    }
}

fn render_gate_approval(
    gate: approval::Gate,
    canonical_spec: &str,
    approved: &approval::GateApproval,
) -> String {
    let mut output = format!(
        "OK SPEC_{}_APPROVED: Approved {} for spec {}.\n",
        gate.name().to_uppercase(),
        gate.name(),
        escape(canonical_spec)
    );
    push_field(&mut output, "State", approval::state_name(approved.state));
    push_field(&mut output, "Approval mode", approved.approval_mode);
    if let Some(workflow) = &approved.delegation_workflow {
        push_field(&mut output, "Delegation workflow", workflow);
    }
    push_field(&mut output, "Passed at", &approved.passed_at);
    if let Some(count) = approved.approved_requirement_ids {
        push_field(&mut output, "Approved requirement IDs", &count.to_string());
    }
    output
}

fn approve_failed_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_APPROVE_FAILED",
        approval::Gate::Design => "SPEC_DESIGN_APPROVE_FAILED",
        approval::Gate::Tasks => "SPEC_TASKS_APPROVE_FAILED",
    }
}

fn invalidate_failed_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_INVALIDATE_FAILED",
        approval::Gate::Design => "SPEC_DESIGN_INVALIDATE_FAILED",
        approval::Gate::Tasks => "SPEC_TASKS_INVALIDATE_FAILED",
    }
}

fn already_approved_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_ALREADY_APPROVED",
        approval::Gate::Design => "SPEC_DESIGN_ALREADY_APPROVED",
        approval::Gate::Tasks => "SPEC_TASKS_ALREADY_APPROVED",
    }
}

fn not_approved_code(gate: approval::Gate) -> &'static str {
    match gate {
        approval::Gate::Requirements => "SPEC_REQUIREMENTS_NOT_APPROVED",
        approval::Gate::Design => "SPEC_DESIGN_NOT_APPROVED",
        approval::Gate::Tasks => "SPEC_TASKS_NOT_APPROVED",
    }
}

fn approve_failure_message(gate: approval::Gate, canonical_spec: &str) -> String {
    format!("Cannot approve {} for spec {canonical_spec}.", gate.name())
}

fn render_approval_issue(issue: &approval::ApprovalIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}
