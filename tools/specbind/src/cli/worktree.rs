use super::{CommandOutput, input};
use crate::{config, read_model::worktree};
use std::path::Path;

#[must_use]
pub fn worktree_capture(start: &Path) -> CommandOutput {
    let root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(e) => return CommandOutput::failure(e.code, e.message, vec![]),
    };
    match worktree::capture(&root) {
        Ok(snapshot) => render(&snapshot, true),
        Err(message) => CommandOutput::failure("WORKTREE_CAPTURE_FAILED", message, vec![]),
    }
}

#[must_use]
pub fn worktree_check(start: &Path, source: &str, declared: &[String]) -> CommandOutput {
    let root = match config::project_root_from(start) {
        Ok(root) => root,
        Err(e) => return CommandOutput::failure(e.code, e.message, vec![]),
    };
    let input =
        match input::read_external_input(&input::WORKTREE_SNAPSHOT_INPUT, start, &root, source) {
            Ok(input) => input,
            Err(e) => return CommandOutput::failure(e.code, e.message, vec![]),
        };
    match worktree::check(&root, &input, declared) {
        Ok(report) => render(&report, report.matched()),
        Err(message) => CommandOutput::failure("WORKTREE_CHECK_FAILED", message, vec![]),
    }
}

fn render(value: &impl serde::Serialize, success: bool) -> CommandOutput {
    match serde_json::to_vec_pretty(value) {
        Ok(mut stdout) => {
            stdout.push(b'\n');
            CommandOutput {
                stdout,
                stderr: vec![],
                success,
            }
        }
        Err(e) => CommandOutput::failure("WORKTREE_RENDER_FAILED", e.to_string(), vec![]),
    }
}
