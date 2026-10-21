//! Transport for guarded existing Task plan revision.

use super::{CommandOutput, Path, config, input};
use crate::task_revision;

#[must_use]
pub fn task_revision_preflight(start: &Path, spec: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    render(task_revision::preflight(
        &paths.project_root,
        &paths.specbind_root,
        spec,
    ))
}

#[must_use]
pub fn task_revision_apply(start: &Path, spec: &str, source: &str, dry_run: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let candidate = match input::read_external_input(
        &input::TASK_REVISION_INPUT,
        start,
        &paths.project_root,
        source,
    ) {
        Ok(candidate) => candidate,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    render(task_revision::apply(
        &paths.project_root,
        &paths.specbind_root,
        spec,
        &candidate,
        dry_run,
    ))
}

fn render(result: Result<String, task_revision::RevisionError>) -> CommandOutput {
    match result {
        Ok(json) => CommandOutput::success(json.into_bytes()),
        Err(error) => CommandOutput::failure(error.code, error.message, error.details),
    }
}
