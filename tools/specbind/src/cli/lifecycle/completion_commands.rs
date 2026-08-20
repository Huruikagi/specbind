//! Completion command execution and rendering.

use super::super::{
    CommandOutput, CompletionIssue, Path, completion, config, escape, read_external_json,
};

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
