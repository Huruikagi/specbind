//! Release command execution and rendering.

use super::super::*;

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
