//! Contract-review command execution and rendering.

use super::super::*;

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
