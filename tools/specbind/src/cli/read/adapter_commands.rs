//! Project-bound adapter catalog commands.

use super::super::*;
use super::present;

/// Lists every accepted adapter and whether the project has it.
///
/// The listing enumerates the accepted selectors, never the directory. A file
/// that happens to sit below the adapters root is not an adapter and never
/// becomes one by existing.
#[must_use]
pub fn adapter_list(start: &Path) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let mut details = Vec::new();
    for entry in adapter::all() {
        match entry.state(&paths.specbind_root) {
            Ok(state) => details.push(format!(
                "selector={} type=\"{}\" path={} present={} state={}",
                escape(entry.selector),
                escape(entry.artifact_type),
                escape(&entry.path()),
                present(state != adapter::AdapterState::Absent),
                state.name()
            )),
            Err(error) => {
                return CommandOutput::failure(
                    "ADAPTER_LIST_FAILED",
                    "Cannot inspect project adapters.",
                    vec![format!("{} {}", error.code, error.message)],
                );
            }
        }
    }
    let mut output = format!(
        "OK ADAPTER_LISTED: Found {} accepted adapter(s).\n",
        details.len()
    );
    for detail in details {
        output.push_str("  ");
        output.push_str(&detail);
        output.push('\n');
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one adapter selector as raw UTF-8 Markdown.
///
/// Absence is reported, not judged. Whether a missing adapter is a fault
/// belongs to the consuming skill.
#[must_use]
pub fn adapter_read(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let Some(entry) = adapter::find(selector) else {
        return CommandOutput::failure(
            "ADAPTER_READ_INVALID",
            format!("unknown adapter selector: {selector}"),
            vec![],
        );
    };
    match entry.read(&paths.specbind_root) {
        Ok(Some(content)) => CommandOutput::success(content.into_bytes()),
        Ok(None) => CommandOutput::no_change(
            "ADAPTER_ABSENT",
            &format!("The project has no {selector} adapter."),
        ),
        Err(error) => CommandOutput::failure(error.code, error.message, vec![]),
    }
}

/// Reads active adapter guidance while projecting inactive catalog state.
///
/// Raw reads remain available to configuration workflows. Consumers use this
/// projection so they never need to parse the product-owned scaffold marker.
#[must_use]
pub fn adapter_read_for_consume(start: &Path, selector: &str) -> CommandOutput {
    let paths = match config::resolve_from(start) {
        Ok(paths) => paths,
        Err(error) => return CommandOutput::failure(error.code, error.message, vec![]),
    };
    let Some(entry) = adapter::find(selector) else {
        return CommandOutput::failure(
            "ADAPTER_READ_INVALID",
            format!("unknown adapter selector: {selector}"),
            vec![],
        );
    };
    match entry.resolve(&paths.specbind_root) {
        Ok(resolved) => match resolved.state {
            adapter::AdapterState::Absent => CommandOutput::no_change(
                "ADAPTER_ABSENT",
                &format!("The project has no {selector} adapter."),
            ),
            adapter::AdapterState::Scaffold => CommandOutput::no_change(
                "ADAPTER_SCAFFOLD",
                &format!("The project {selector} adapter is an inactive scaffold."),
            ),
            adapter::AdapterState::Active => match resolved.content {
                Some(content) => CommandOutput::success(content.into_bytes()),
                None => CommandOutput::failure(
                    "ADAPTER_READ_FAILED",
                    "Cannot read active project adapter guidance.",
                    vec![],
                ),
            },
        },
        Err(error) => CommandOutput::failure(error.code, error.message, vec![]),
    }
}
