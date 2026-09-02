//! Milestone mutation command execution and rendering.

use super::super::{
    CommandOutput, ExternalInputError, LOG_ENTRIES_INPUT, MilestoneIssue, Path, SCOPE_INPUT,
    adoption_finalize, config, escape, milestone, push_field, read_external_input,
};

#[must_use]
pub fn milestone_reverse_finalize(start: &Path, log_entries_source: Option<&str>) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let log_entries = match log_entries_source {
        Some(source) => {
            match read_external_input(&LOG_ENTRIES_INPUT, start, &paths.project_root, source) {
                Ok(input) => Some(input),
                Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
            }
        }
        None => None,
    };
    match adoption_finalize::finalize(
        &paths.project_root,
        &paths.specbind_root,
        paths.language,
        log_entries.as_deref(),
    ) {
        Ok(outcome) => CommandOutput::success(
            format!(
                "OK ADOPTION_FINALIZED: Adopted baseline {} across {} specs; no product release was created.\n",
                escape(&outcome.baseline_version),
                outcome.specs,
            )
            .into_bytes(),
        ),
        Err(error) => {
            let mut issues = error.issues.into_iter();
            let Some(first) = issues.next() else {
                return CommandOutput::failure(
                    "ADOPTION_FINALIZE_FAILED",
                    "Reverse adoption finalization failed.",
                    vec![],
                );
            };
            let render = |issue: &adoption_finalize::FinalizeIssue| {
                let path = issue
                    .path
                    .as_ref()
                    .map_or_else(String::new, |path| format!(" {}:", escape(path)));
                format!("{}{path} {}", issue.code, escape(&issue.message))
            };
            let mut details = vec![render(&first)];
            details.extend(issues.map(|issue| render(&issue)));
            CommandOutput::failure(first.code, "Reverse adoption finalization failed.", details)
        }
    }
}

#[must_use]
pub fn milestone_reverse_abandon(start: &Path, milestone_id: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    match adoption_finalize::abandon(&paths.project_root, &paths.specbind_root, milestone_id) {
        Ok(outcome) => CommandOutput::success(
            format!(
                "OK ADOPTION_ABANDONED: Abandoned reverse milestone {} and removed {} unestablished specs.\n",
                escape(&outcome.milestone_id),
                outcome.specs_removed,
            )
            .into_bytes(),
        ),
        Err(error) => {
            let mut issues = error.issues.into_iter();
            let Some(first) = issues.next() else {
                return CommandOutput::failure(
                    "ADOPTION_ABANDON_FAILED",
                    "Reverse abandonment failed.",
                    vec![],
                );
            };
            let render = |issue: &adoption_finalize::FinalizeIssue| {
                let path = issue
                    .path
                    .as_ref()
                    .map_or_else(String::new, |path| format!(" {}:", escape(path)));
                format!("{}{path} {}", issue.code, escape(&issue.message))
            };
            let mut details = vec![render(&first)];
            details.extend(issues.map(|issue| render(&issue)));
            CommandOutput::failure(first.code, "Reverse abandonment failed.", details)
        }
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
    if counts.reverse_specs > 0 {
        push_field(output, "Reverse specs", &counts.reverse_specs.to_string());
    }
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
