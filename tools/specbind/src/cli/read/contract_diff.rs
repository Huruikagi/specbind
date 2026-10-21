use crate::{
    cli::{CommandOutput, output::escape},
    config,
    read_model::contract_diff,
};
use std::{fmt::Write as _, path::Path};

#[must_use]
pub fn contract_diff(start: &Path, baseline: &str, json: bool) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let report = match contract_diff::resolve(&paths, baseline) {
        Ok(report) => report,
        Err(message) => {
            return CommandOutput::failure("CONTRACT_DIFF_INPUT_INVALID", message, vec![]);
        }
    };
    if json {
        return match serde_json::to_vec_pretty(&report) {
            Ok(mut bytes) => {
                bytes.push(b'\n');
                CommandOutput::success(bytes)
            }
            Err(error) => {
                CommandOutput::failure("CONTRACT_DIFF_RENDER_FAILED", error.to_string(), vec![])
            }
        };
    }
    let mut text = format!(
        "OK CONTRACT_DIFF_REPORTED: Compared {} Contract document(s); {} changed entry(s).\n  Baseline: {}\n  Current HEAD: {}\n  Current source: worktree\n  Current fingerprint: {}\n",
        report.documents.len(),
        report.entries.len(),
        report.baseline_revision,
        report.current_revision,
        report.current_fingerprint
    );
    for document in &report.documents {
        writeln!(
            text,
            "  Document: {} {}",
            escape(&document.owner),
            document.change
        )
        .expect("String write");
    }
    for entry in &report.entries {
        writeln!(
            text,
            "  Entry: {} {}\n    Baseline consumers: {}\n    Current consumers: {}",
            escape(&entry.selector),
            entry.change,
            escape(&entry.baseline_consumers.join(", ")),
            escape(&entry.current_consumers.join(", "))
        )
        .expect("String write");
    }
    for issue in &report.diagnostics {
        writeln!(
            text,
            "  Diagnostic: {} {} {} {}",
            issue.side,
            issue.severity,
            issue.code,
            escape(&issue.message)
        )
        .expect("String write");
    }
    text.push_str("  Review: inspect --json before/after values; structural diff is not a semantic verdict.\n");
    CommandOutput::success(text.into_bytes())
}
