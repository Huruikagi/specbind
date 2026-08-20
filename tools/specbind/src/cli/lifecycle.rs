//! CLI execution and rendering for lifecycle commands.

use super::*;

fn render_completion_failure(message: &str, error: completion::CompletionIssues) -> CommandOutput {
    let mut issues = error.issues.into_iter();
    let Some(first) = issues.next() else {
        return CommandOutput::failure("COMPLETION_FAILED", message, vec![]);
    };
    let mut details = vec![render_completion_issue(&first)];
    details.extend(issues.map(|issue| render_completion_issue(&issue)));
    CommandOutput::failure(first.code, message, details)
}

fn render_completion_issue(issue: &CompletionIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn spec_status(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let model =
        match spec_status::resolve(&paths.project_root, &paths.specbind_root, canonical_spec) {
            Ok(model) => model,
            Err(error) => {
                return CommandOutput::failure(
                    "SPEC_STATUS_FAILED",
                    format!("Cannot report status for spec {canonical_spec}."),
                    error.issues.iter().map(render_issue).collect(),
                );
            }
        };
    render_spec_status(canonical_spec, &model)
}

#[must_use]
pub fn spec_completion_preflight(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::spec_preflight(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
    ) {
        Ok(completion::SpecPreflightOutcome::Ready {
            implementation_revision,
        }) => CommandOutput::success(
            format!(
                "OK SPEC_COMPLETION_PREFLIGHT_READY: Spec {} is ready for completion validation.\n  Implementation revision: {}\n",
                escape(canonical_spec),
                implementation_revision
            )
            .into_bytes(),
        ),
        Ok(completion::SpecPreflightOutcome::AlreadyAccepted { .. }) => CommandOutput::no_change(
            "SPEC_COMPLETION_ALREADY_ACCEPTED",
            &format!("Spec {} already has fresh completion evidence.", escape(canonical_spec)),
        ),
        Err(error) => render_completion_failure("Cannot begin Spec completion validation.", error),
    }
}

#[must_use]
pub fn spec_completion_accept(
    start: &Path,
    canonical_spec: &str,
    evidence_source: &str,
) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let evidence = match read_external_json(start, &paths.project_root, evidence_source) {
        Ok(evidence) => evidence,
        Err(output) => return output,
    };
    match completion::spec_accept(
        &paths.project_root,
        &paths.specbind_root,
        canonical_spec,
        &evidence,
    ) {
        Ok(completion::SpecAcceptOutcome::Accepted {
            implementation_revision,
        }) => CommandOutput::success(
            format!(
                "OK SPEC_COMPLETION_ACCEPTED: Accepted completion for spec {}.\n  Implementation revision: {}\n",
                escape(canonical_spec),
                implementation_revision
            )
            .into_bytes(),
        ),
        Ok(completion::SpecAcceptOutcome::AlreadyAccepted { .. }) => CommandOutput::no_change(
            "SPEC_COMPLETION_ALREADY_ACCEPTED",
            &format!("Spec {} already has identical fresh completion evidence.", escape(canonical_spec)),
        ),
        Err(error) => render_completion_failure("Cannot accept Spec completion evidence.", error),
    }
}

#[must_use]
pub fn spec_completion_invalidate(start: &Path, canonical_spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::spec_invalidate(&paths.project_root, &paths.specbind_root, canonical_spec) {
        Ok(completion::SpecInvalidateOutcome::Invalidated) => CommandOutput::success(
            format!(
                "OK SPEC_COMPLETION_INVALIDATED: Invalidated completion for spec {}.\n",
                escape(canonical_spec)
            )
            .into_bytes(),
        ),
        Ok(completion::SpecInvalidateOutcome::NoChange) => CommandOutput::no_change(
            "SPEC_COMPLETION_NOT_ACCEPTED",
            &format!(
                "Spec {} has no accepted completion to invalidate.",
                escape(canonical_spec)
            ),
        ),
        Err(error) => render_completion_failure("Cannot invalidate Spec completion.", error),
    }
}

#[must_use]
pub fn direct_completion_preflight(start: &Path, canonical_direct: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::direct_preflight(
        &paths.project_root,
        &paths.specbind_root,
        canonical_direct,
    ) {
        Ok(completion::DirectPreflightOutcome::Ready {
            implementation_revision,
        }) => CommandOutput::success(
            format!(
                "OK DIRECT_COMPLETION_PREFLIGHT_READY: Direct item {} is ready for completion validation.\n  Implementation revision: {}\n",
                escape(canonical_direct),
                implementation_revision
            )
            .into_bytes(),
        ),
        Ok(completion::DirectPreflightOutcome::AlreadyCompleted) => CommandOutput::no_change(
            "DIRECT_COMPLETION_ALREADY_RECORDED",
            &format!("Direct item {} is already completed.", escape(canonical_direct)),
        ),
        Err(error) => render_completion_failure("Cannot begin Direct completion validation.", error),
    }
}

#[must_use]
pub fn direct_completion_complete(
    start: &Path,
    canonical_direct: &str,
    implementation_revision: &str,
) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match completion::direct_complete(
        &paths.project_root,
        &paths.specbind_root,
        canonical_direct,
        implementation_revision,
    ) {
        Ok(completion::DirectCompleteOutcome::Recorded) => CommandOutput::success(
            format!(
                "OK DIRECT_COMPLETION_RECORDED: Recorded completion for Direct item {}.\n",
                escape(canonical_direct)
            )
            .into_bytes(),
        ),
        Ok(completion::DirectCompleteOutcome::AlreadyCompleted) => CommandOutput::no_change(
            "DIRECT_COMPLETION_ALREADY_RECORDED",
            &format!(
                "Direct item {} is already completed.",
                escape(canonical_direct)
            ),
        ),
        Err(error) => render_completion_failure("Cannot record Direct completion.", error),
    }
}

#[must_use]
pub fn milestone_bind_release(start: &Path, version: &str, rebind: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone::bind_release(
        &paths.project_root,
        &paths.specbind_root,
        version,
        rebind,
    ) {
        Ok(milestone::BindReleaseOutcome::Bound {
            milestone_id,
            version,
            targets,
        }) => CommandOutput::success(
            format!(
                "OK RELEASE_BOUND: Bound milestone {} to release {}.\n  Roadmap archive: {}\n  Contract review archive: {}\n",
                escape(&milestone_id),
                escape(&version),
                escape(&targets.roadmap),
                escape(&targets.cross_spec_review)
            )
            .into_bytes(),
        ),
        Ok(milestone::BindReleaseOutcome::Rebound {
            milestone_id,
            previous,
            version,
            targets,
        }) => CommandOutput::success(
            format!(
                "OK RELEASE_REBOUND: Rebound milestone {} from release {} to {}.\n  Roadmap archive: {}\n  Contract review archive: {}\n",
                escape(&milestone_id),
                escape(&previous),
                escape(&version),
                escape(&targets.roadmap),
                escape(&targets.cross_spec_review)
            )
            .into_bytes(),
        ),
        Ok(milestone::BindReleaseOutcome::AlreadyBound {
            milestone_id,
            version,
        }) => CommandOutput::no_change(
            "RELEASE_ALREADY_BOUND",
            &format!(
                "Milestone {} is already bound to release {}.",
                escape(&milestone_id),
                escape(&version)
            ),
        ),
        Err(error) => render_milestone_mutation_failure("Cannot bind milestone release.", error),
    }
}

#[must_use]
pub fn milestone_create(start: &Path, scope_source: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let scope = match read_scope_document(start, &paths.project_root, scope_source) {
        Ok(scope) => scope,
        Err(error) => {
            return CommandOutput::failure(
                "MILESTONE_CREATE_FAILED",
                "Cannot create the active milestone.",
                vec![format!("{} {}", error.code, escape(&error.message))],
            );
        }
    };
    match milestone::create(&paths.project_root, &paths.specbind_root, &scope) {
        Ok(milestone::CreateOutcome::Created {
            milestone_id,
            baseline_revision,
            counts,
        }) => {
            let mut output = format!(
                "OK MILESTONE_CREATED: Created milestone {}.\n",
                escape(&milestone_id)
            );
            push_field(&mut output, "Baseline revision", &baseline_revision);
            push_scope_counts(&mut output, &counts);
            CommandOutput::success(output.into_bytes())
        }
        Err(error) => render_milestone_failure(
            "MILESTONE_CREATE_FAILED",
            "Cannot create the active milestone.",
            &error,
        ),
    }
}

#[must_use]
pub fn milestone_update_scope(start: &Path, scope_source: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let scope = match read_scope_document(start, &paths.project_root, scope_source) {
        Ok(scope) => scope,
        Err(error) => {
            return CommandOutput::failure(
                "MILESTONE_SCOPE_UPDATE_FAILED",
                "Cannot update the milestone scope.",
                vec![format!("{} {}", error.code, escape(&error.message))],
            );
        }
    };
    match milestone::update_scope(&paths.project_root, &paths.specbind_root, &scope) {
        Ok(milestone::ScopeUpdateOutcome::Updated {
            milestone_id,
            counts,
            review_removed,
        }) => {
            let mut output = format!(
                "OK MILESTONE_SCOPE_UPDATED: Updated scope for milestone {}.\n",
                escape(&milestone_id)
            );
            push_scope_counts(&mut output, &counts);
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
        Ok(milestone::ScopeUpdateOutcome::NoChange { milestone_id }) => CommandOutput::no_change(
            "MILESTONE_SCOPE_UNCHANGED",
            &format!(
                "Milestone {} already has the submitted scope.",
                escape(&milestone_id)
            ),
        ),
        Err(error) => render_milestone_failure(
            "MILESTONE_SCOPE_UPDATE_FAILED",
            "Cannot update the milestone scope.",
            &error,
        ),
    }
}

#[must_use]
pub fn milestone_rebaseline(start: &Path, revision: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone::rebaseline(&paths.project_root, &paths.specbind_root, revision) {
        Ok(milestone::RebaselineOutcome::Rebaselined {
            milestone_id,
            previous,
            baseline_revision,
            review_removed,
        }) => {
            let mut output = format!(
                "OK MILESTONE_REBASELINED: Rebaselined milestone {}.\n",
                escape(&milestone_id)
            );
            push_field(&mut output, "Previous baseline", &previous);
            push_field(&mut output, "Baseline revision", &baseline_revision);
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
        Ok(milestone::RebaselineOutcome::NoChange {
            milestone_id,
            baseline_revision,
        }) => CommandOutput::no_change(
            "MILESTONE_BASELINE_UNCHANGED",
            &format!(
                "Milestone {} is already based on {}.",
                escape(&milestone_id),
                escape(&baseline_revision)
            ),
        ),
        Err(error) => render_milestone_failure(
            "MILESTONE_REBASELINE_FAILED",
            "Cannot rebaseline the active milestone.",
            &error,
        ),
    }
}

fn push_scope_counts(output: &mut String, counts: &milestone::ScopeCounts) {
    push_field(output, "New specs", &counts.new_specs.to_string());
    push_field(output, "Spec updates", &counts.spec_updates.to_string());
    push_field(output, "Direct changes", &counts.direct_changes.to_string());
}

fn read_scope_document(
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, ExternalInputError> {
    read_external_input(&SCOPE_INPUT, start, project_root, source)
}

fn render_milestone_failure(
    code: &'static str,
    message: &str,
    error: &milestone::MilestoneIssues,
) -> CommandOutput {
    CommandOutput::failure(
        code,
        message,
        error.issues.iter().map(render_milestone_issue).collect(),
    )
}

fn render_milestone_mutation_failure(
    message: &str,
    error: milestone::MilestoneIssues,
) -> CommandOutput {
    let mut issues = error.issues.into_iter();
    let Some(first) = issues.next() else {
        return CommandOutput::failure("MILESTONE_MUTATION_FAILED", message, vec![]);
    };
    let mut details = vec![render_milestone_issue(&first)];
    details.extend(issues.map(|issue| render_milestone_issue(&issue)));
    CommandOutput::failure(first.code, message, details)
}

fn render_milestone_issue(issue: &MilestoneIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn release_preflight(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match release_readiness::resolve(&paths.project_root, &paths.specbind_root) {
        Ok(readiness) => {
            let mut output = format!(
                "OK RELEASE_READY: Release {} is ready for project release work across {} specs.\n  Milestone ID: {}\n  Specs: {}\n  Direct changes: {}\n  Mutation targets:\n",
                escape(&readiness.version),
                readiness.specs.len(),
                escape(&readiness.milestone_id),
                if readiness.specs.is_empty() {
                    "none".to_owned()
                } else {
                    readiness
                        .specs
                        .iter()
                        .map(|spec| escape(spec))
                        .collect::<Vec<_>>()
                        .join(", ")
                },
                readiness.direct_changes,
            );
            for target in readiness.mutation_targets {
                let state = match target.state {
                    MutationTargetState::Existing => "existing",
                    MutationTargetState::Absent => "absent",
                };
                writeln!(output, "    - {state} {}", escape(&target.path))
                    .expect("write to String");
            }
            CommandOutput::success(output.into_bytes())
        }
        Err(error) => CommandOutput::failure(
            error.code,
            "Release preflight failed.",
            error
                .diagnostics
                .iter()
                .map(render_release_diagnostic)
                .collect(),
        ),
    }
}

#[must_use]
pub fn release_finalize(start: &Path, log_entries_source: Option<&str>) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let log_entries = match log_entries_source {
        Some(source) => match read_release_log_entries(start, &paths.project_root, source) {
            Ok(input) => Some(input),
            Err(output) => return output,
        },
        None => None,
    };
    match release_finalize::finalize(
        &paths.project_root,
        &paths.specbind_root,
        paths.language,
        log_entries.as_deref(),
    ) {
        Ok(release_finalize::FinalizeOutcome::Finalized { version, specs }) => {
            CommandOutput::success(
                format!(
                    "OK RELEASE_FINALIZED: Finalized {} for {} specs.\n",
                    escape(&version),
                    specs
                )
                .into_bytes(),
            )
        }
        Ok(release_finalize::FinalizeOutcome::AlreadyFinalized { version, .. }) => {
            CommandOutput::no_change(
                "RELEASE_ALREADY_FINALIZED",
                &format!("Release {} is already finalized.", escape(&version)),
            )
        }
        Err(error) => {
            let mut issues = error.issues.into_iter();
            let Some(first) = issues.next() else {
                return CommandOutput::failure(
                    "RELEASE_FINALIZE_FAILED",
                    "Release finalization failed.",
                    vec![],
                );
            };
            let mut details = vec![render_finalize_issue(&first)];
            details.extend(issues.map(|issue| render_finalize_issue(&issue)));
            CommandOutput::failure(first.code, "Release finalization failed.", details)
        }
    }
}

fn read_release_log_entries(
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, CommandOutput> {
    read_external_input(&LOG_ENTRIES_INPUT, start, project_root, source)
        .map_err(|error| CommandOutput::failure(error.code, error.message, vec![]))
}

fn render_finalize_issue(issue: &FinalizeIssue) -> String {
    let path = issue
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

fn render_release_diagnostic(diagnostic: &ReleaseDiagnostic) -> String {
    let path = diagnostic
        .path
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", diagnostic.code, escape(&diagnostic.message))
}

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

#[must_use]
pub fn milestone_review_status(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let report = cross_spec_review::evaluate_freshness(&paths.project_root, &paths.specbind_root);
    let details = report.issues.iter().map(render_review_issue).collect();
    let reportable = matches!(
        report.status,
        ReviewFreshnessStatus::NotRequired
            | ReviewFreshnessStatus::Missing
            | ReviewFreshnessStatus::Fresh
            | ReviewFreshnessStatus::Stale
    );
    let Some(milestone_id) = report.milestone_id.as_deref().filter(|_| reportable) else {
        return CommandOutput::failure(
            "MILESTONE_REVIEW_STATUS_FAILED",
            "Cannot report the contract review status.",
            details,
        );
    };
    let mut output = format!(
        "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported contract review status for milestone {}.\n",
        escape(milestone_id)
    );
    push_field(
        &mut output,
        "Status",
        milestone_status::review_name(report.status),
    );
    if let Some(accepted) = &report.accepted {
        push_field(&mut output, "Passed at", &accepted.passed_at);
        push_field(
            &mut output,
            "Inputs",
            &accepted.input_revisions.len().to_string(),
        );
    }
    if !details.is_empty() {
        output.push_str("  Diagnostics:\n");
        for detail in details {
            writeln!(output, "    - {detail}").expect("writing to a String cannot fail");
        }
    }
    CommandOutput::success(output.into_bytes())
}

#[must_use]
pub fn milestone_review_accept(start: &Path, candidate_source: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let candidate = match read_external_input(
        &REVIEW_CANDIDATE_INPUT,
        start,
        &paths.project_root,
        candidate_source,
    ) {
        Ok(candidate) => candidate,
        Err(error) => {
            return CommandOutput::failure(
                "MILESTONE_REVIEW_ACCEPT_FAILED",
                "Cannot accept the contract review.",
                vec![format!("{} {}", error.code, escape(&error.message))],
            );
        }
    };
    match cross_spec_review::accept(&paths.project_root, &paths.specbind_root, &candidate) {
        Ok(accepted) => CommandOutput::success(
            format!(
                "OK MILESTONE_REVIEW_ACCEPTED: Accepted contract review for milestone {}.\n  Passed at: {}\n  Inputs: {}\n",
                escape(&accepted.milestone_id),
                escape(&accepted.passed_at),
                accepted.input_revisions.len()
            )
            .into_bytes(),
        ),
        Err(error) => CommandOutput::failure(
            "MILESTONE_REVIEW_ACCEPT_FAILED",
            "Cannot accept the contract review.",
            error.issues.iter().map(render_review_issue).collect(),
        ),
    }
}

fn render_review_issue(issue: &ReviewIssue) -> String {
    let path = issue
        .source
        .as_ref()
        .map_or_else(String::new, |path| format!(" {}:", escape(path)));
    format!("{}{path} {}", issue.code, escape(&issue.message))
}

#[must_use]
pub fn milestone_status(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match milestone_status::resolve(&paths.project_root, &paths.specbind_root) {
        Ok(Some(model)) => render_milestone_status(&model),
        Ok(None) => CommandOutput::no_change("NO_ACTIVE_MILESTONE", "No active milestone exists."),
        Err(error) => CommandOutput::failure(
            "MILESTONE_STATUS_FAILED",
            "Cannot report the active milestone.",
            error
                .diagnostics
                .iter()
                .map(render_milestone_diagnostic)
                .collect(),
        ),
    }
}

fn render_milestone_status(model: &MilestoneStatusModel) -> CommandOutput {
    let mut output = format!(
        "OK MILESTONE_STATUS_REPORTED: Reported the active milestone.\n  Milestone: {}\n",
        escape(&model.milestone_id)
    );
    push_field(
        &mut output,
        "Target release",
        model.target_release.as_deref().unwrap_or("none"),
    );
    push_field(
        &mut output,
        "Stage",
        milestone_status::stage_name(model.stage),
    );
    push_field(
        &mut output,
        "Health",
        match model.health {
            MilestoneHealth::Consistent => "consistent",
            MilestoneHealth::Inconsistent => "inconsistent",
        },
    );
    push_field(
        &mut output,
        "Contract review",
        milestone_status::review_name(model.review_status),
    );
    let spec_counts = if model.spec_state_counts.is_empty() {
        "none".to_owned()
    } else {
        model
            .spec_state_counts
            .iter()
            .map(|(state, count)| format!("{state}={count}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    push_field(&mut output, "Spec states", &spec_counts);
    push_field(
        &mut output,
        "Direct progress",
        &format!(
            "{}/{} completed",
            model.direct_completed, model.direct_total
        ),
    );
    push_field(
        &mut output,
        "Revision",
        model.current_revision.as_deref().unwrap_or("unavailable"),
    );
    push_field(&mut output, "Baseline", &model.baseline_revision);
    render_milestone_items(model, &mut output);
    render_milestone_actions(model, &mut output);
    push_inline_list(&mut output, "Release blockers", &model.release_blockers);
    render_milestone_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_milestone_items(model: &MilestoneStatusModel, output: &mut String) {
    output.push_str("  Items:\n");
    for item in &model.items {
        let waiting = if item.waiting_for.is_empty() {
            String::new()
        } else {
            format!(" waiting_for={}", item.waiting_for.join(","))
        };
        writeln!(
            output,
            "    - {} status={}{} summary={}",
            escape(&item.id),
            escape(&item.status),
            escape(&waiting),
            escape(&item.summary)
        )
        .expect("writing to a String cannot fail");
    }
}

fn render_milestone_actions(model: &MilestoneStatusModel, output: &mut String) {
    if model.actionable.is_empty() {
        push_field(output, "Actionable", "none");
        return;
    }
    output.push_str("  Actionable:\n");
    for action in &model.actionable {
        writeln!(
            output,
            "    - {} action={}",
            escape(&action.item),
            action.action
        )
        .expect("writing to a String cannot fail");
    }
}

fn render_milestone_diagnostics(model: &MilestoneStatusModel, output: &mut String) {
    if model.diagnostics.is_empty() {
        push_field(output, "Diagnostics", "none");
        return;
    }
    output.push_str("  Diagnostics:\n");
    for diagnostic in &model.diagnostics {
        writeln!(output, "    - {}", render_milestone_diagnostic(diagnostic))
            .expect("writing to a String cannot fail");
    }
}

fn render_spec_status(canonical_spec: &str, model: &SpecStatusModel) -> CommandOutput {
    let mut output = format!(
        "OK SPEC_STATUS_REPORTED: Reported status for spec {}.\n",
        escape(canonical_spec)
    );
    push_field(
        &mut output,
        "State",
        spec_status::state_name(model.declared_state),
    );
    push_field(
        &mut output,
        "Milestone",
        model.milestone_id.as_deref().unwrap_or("none"),
    );
    push_field(
        &mut output,
        "Health",
        match model.health {
            ConsistencyHealth::Consistent => "consistent",
            ConsistencyHealth::Inconsistent => "inconsistent",
        },
    );
    push_field(
        &mut output,
        "Gates",
        &format!(
            "requirements={}, design={}, tasks={}, completion={}",
            spec_status::freshness_name(model.freshness.requirements.status),
            spec_status::freshness_name(model.freshness.design.status),
            spec_status::freshness_name(model.freshness.tasks.status),
            spec_status::freshness_name(model.freshness.completion.status),
        ),
    );
    if let Some(review) = model.contract_review {
        push_field(
            &mut output,
            "Contract review",
            milestone_status::review_name(review),
        );
    }
    if let Some(delegated) = &model.delegated_gates {
        push_field(
            &mut output,
            "Delegated gates",
            &if delegated.is_empty() {
                "none".to_owned()
            } else {
                delegated
                    .iter()
                    .map(|gate| format!("{} ({})", gate.gate, escape(&gate.workflow)))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        );
    }
    render_status_tasks(model, &mut output);
    render_status_coverage(model, &mut output);
    render_status_diagnostics(model, &mut output);
    CommandOutput::success(output.into_bytes())
}

fn render_status_tasks(model: &SpecStatusModel, output: &mut String) {
    if let Some(tasks) = &model.task_model {
        push_field(
            output,
            "Task progress",
            &format!(
                "{} total, {} completed, {} pending, {} blocked",
                tasks.total(),
                tasks.completed,
                tasks.pending,
                tasks.blocked
            ),
        );
        push_inline_list(output, "Next actionable", &tasks.actionable_ids);
    } else {
        push_field(output, "Task progress", "unavailable");
        push_field(output, "Next actionable", "none");
    }
    if model.blockers.is_empty() {
        push_field(output, "Blockers", "none");
    } else {
        output.push_str("  Blockers:\n");
        for blocker in &model.blockers {
            writeln!(
                output,
                "    - {}: {}",
                escape(&blocker.task_id),
                escape(&blocker.reason)
            )
            .expect("writing to a String cannot fail");
        }
    }
}

fn render_status_coverage(model: &SpecStatusModel, output: &mut String) {
    if let Some(coverage) = &model.coverage {
        push_field(
            output,
            "Requirement coverage",
            &format!(
                "design {}/{}, tasks {}/{}{}",
                coverage.design,
                coverage.active,
                coverage.tasks,
                coverage.active,
                if coverage.tasks_required {
                    " (required)"
                } else {
                    " (not required)"
                }
            ),
        );
    } else {
        push_field(output, "Requirement coverage", "inactive");
    }
}

fn render_status_diagnostics(model: &SpecStatusModel, output: &mut String) {
    if model.diagnostics.is_empty() {
        push_field(output, "Diagnostics", "none");
    } else {
        output.push_str("  Diagnostics:\n");
        for diagnostic in &model.diagnostics {
            let path = diagnostic
                .path
                .as_ref()
                .map_or_else(String::new, |path| format!(" {}:", escape(path)));
            writeln!(
                output,
                "    - {}{path} {}",
                diagnostic.code,
                escape(&diagnostic.message)
            )
            .expect("writing to a String cannot fail");
        }
    }
}
