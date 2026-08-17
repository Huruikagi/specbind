//! The marked `SpecBind` block in a project's root agent instruction file.
//!
//! Decision 0099 makes the installer maintain exactly one marked region and
//! leave every other byte of the file alone. The block is product-managed, so a
//! divergent one is replaced; local guidance belongs outside the markers, where
//! nothing here can reach it.

use crate::install::Agent;

/// The embedded block body, authored as prose rather than as a string literal.
pub const BODY: &str = include_str!("../assets/project-instructions/block.md");

const OPEN: &str = "<!-- specbind:block -->";
const CLOSE: &str = "<!-- /specbind:block -->";

/// Why one instruction file cannot be maintained.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkerError {
    pub code: &'static str,
    pub message: String,
}

/// Returns the root-relative instruction file one agent reads.
#[must_use]
pub fn target(agent: Agent) -> &'static str {
    match agent {
        Agent::ClaudeCode => "CLAUDE.md",
        Agent::Codex => "AGENTS.md",
    }
}

/// Renders the complete marked block, markers included.
#[must_use]
pub fn block() -> String {
    format!("{OPEN}\n{}{CLOSE}\n", ensure_trailing_newline(BODY))
}

/// One computed instruction file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    /// The complete file content to write.
    pub content: String,
    /// Whether a marked block was already present.
    ///
    /// This separates replacing a block from adding one. Appending removes no
    /// text, so it is not a replacement under Decision 0077 and does not require
    /// a committed clean repository.
    pub had_block: bool,
}

/// Produces the file content that carries the current block.
///
/// `current` is `None` when the file does not exist, in which case the result is
/// the block alone. An existing file keeps every byte outside the markers.
///
/// # Errors
///
/// Returns a diagnostic when the markers are unpaired, reversed, or repeated.
/// The installer never guesses which of two blocks is authoritative and never
/// repairs a malformed one, because both would edit text the project owns.
pub fn apply(current: Option<&str>) -> Result<Application, MarkerError> {
    let Some(current) = current else {
        return Ok(Application {
            content: block(),
            had_block: false,
        });
    };
    let opens = marker_lines(current, OPEN);
    let closes = marker_lines(current, CLOSE);
    match (opens.len(), closes.len()) {
        (0, 0) => Ok(Application {
            content: append(current),
            had_block: false,
        }),
        (1, 1) => {
            let (start, _) = opens[0];
            let (close_start, close_end) = closes[0];
            if close_start < start {
                return Err(MarkerError {
                    code: "PROJECT_INSTRUCTIONS_MARKERS_REVERSED",
                    message: "the closing SpecBind marker precedes the opening marker".to_owned(),
                });
            }
            Ok(Application {
                content: format!("{}{}{}", &current[..start], block(), &current[close_end..]),
                had_block: true,
            })
        }
        (opens, closes) => Err(MarkerError {
            code: "PROJECT_INSTRUCTIONS_MARKERS_INVALID",
            message: format!(
                "expected one opening and one closing SpecBind marker, found {opens} and {closes}"
            ),
        }),
    }
}

/// Appends the block, separated from existing content by exactly one blank line.
fn append(current: &str) -> String {
    if current.trim().is_empty() {
        return block();
    }
    let mut output = ensure_trailing_newline(current);
    if !output.ends_with("\n\n") {
        output.push('\n');
    }
    output.push_str(&block());
    output
}

/// Locates each marker occupying a whole line, returning its byte span.
///
/// A marker inside a fenced code block still counts. Resolving that would need a
/// Markdown parse whose outcome a reader cannot easily predict, and stopping is
/// recoverable.
fn marker_lines(content: &str, marker: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut offset = 0;
    for line in content.split_inclusive('\n') {
        if line.trim_end_matches(['\n', '\r']).trim() == marker {
            found.push((offset, offset + line.len()));
        }
        offset += line.len();
    }
    found
}

fn ensure_trailing_newline(value: &str) -> String {
    if value.ends_with('\n') {
        value.to_owned()
    } else {
        format!("{value}\n")
    }
}
