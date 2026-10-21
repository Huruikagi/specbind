//! Transport for read-only replan recovery evidence.

use super::{CommandOutput, Path, config};

fn run(start: &Path, revisions: Option<(&str, &str)>) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let result = if let Some((baseline, recovery)) = revisions {
        crate::replan_completion::check(
            &paths.project_root,
            &paths.specbind_root,
            baseline,
            recovery,
        )
    } else {
        crate::replan_completion::capture(&paths.project_root, &paths.specbind_root)
    };
    match result {
        Ok(json) => CommandOutput::success(json.into_bytes()),
        Err(error) => CommandOutput::failure(error.code, error.message, vec![]),
    }
}

#[must_use]
pub fn replan_capture(start: &Path) -> CommandOutput {
    run(start, None)
}

#[must_use]
pub fn replan_check(start: &Path, baseline: &str, recovery: &str) -> CommandOutput {
    run(start, Some((baseline, recovery)))
}
