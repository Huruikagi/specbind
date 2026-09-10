//! Installation support for the shared OKF bundle-root `index.md`.
//!
//! `SpecBind` owns the version declaration and one marked navigation block. The
//! project owns every byte outside that block, so installation preserves it.

use serde_json::Value;

use crate::config::ProjectLanguage;

const VERSION: &str = "0.2";
const OPEN: &str = "<!-- specbind:index -->";
const CLOSE: &str = "<!-- /specbind:index -->";
const EN_BODY: &str = include_str!("../../assets/bundle-index/en.md");
const JA_BODY: &str = include_str!("../../assets/bundle-index/ja.md");

/// Why a bundle index cannot be maintained safely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexError {
    pub code: &'static str,
    pub message: String,
}

/// One computed bundle index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    pub content: String,
    pub had_block: bool,
}

/// Produces an OKF v0.2 root index with the current localized product block.
///
/// Existing project content is preserved byte-for-byte. A missing version
/// declaration is inserted additively; an incompatible declaration or
/// ambiguous marker shape fails closed.
///
/// # Errors
///
/// Returns a diagnostic for invalid or incompatible Front Matter and for
/// ambiguous managed-block markers.
pub fn apply(current: Option<&str>, language: ProjectLanguage) -> Result<Application, IndexError> {
    let Some(current) = current else {
        return Ok(Application {
            content: format!("{}\n{}", preamble(), block(language)),
            had_block: false,
        });
    };

    let versioned = ensure_version(current)?;
    let opens = marker_lines(&versioned, OPEN);
    let closes = marker_lines(&versioned, CLOSE);
    match (opens.len(), closes.len()) {
        (0, 0) => Ok(Application {
            content: append(&versioned, &block(language)),
            had_block: false,
        }),
        (1, 1) => {
            let (start, _) = opens[0];
            let (close_start, close_end) = closes[0];
            if close_start < start {
                return Err(IndexError {
                    code: "BUNDLE_INDEX_MARKERS_REVERSED",
                    message: "the closing bundle-index marker precedes the opening marker"
                        .to_owned(),
                });
            }
            Ok(Application {
                content: format!(
                    "{}{}{}",
                    &versioned[..start],
                    block(language),
                    &versioned[close_end..]
                ),
                had_block: true,
            })
        }
        (opens, closes) => Err(IndexError {
            code: "BUNDLE_INDEX_MARKERS_INVALID",
            message: format!(
                "expected one opening and one closing bundle-index marker, found {opens} and {closes}"
            ),
        }),
    }
}

#[must_use]
pub fn block(language: ProjectLanguage) -> String {
    let body = match language {
        ProjectLanguage::En => EN_BODY,
        ProjectLanguage::Ja => JA_BODY,
    };
    format!("{OPEN}\n{}{CLOSE}\n", ensure_trailing_newline(body))
}

fn ensure_version(current: &str) -> Result<String, IndexError> {
    if !current.starts_with("---\n") && !current.starts_with("---\r\n") {
        return Ok(format!("{}\n{current}", preamble()));
    }
    let (frontmatter, _) =
        crate::artifacts::split_frontmatter(current).map_err(|message| IndexError {
            code: "BUNDLE_INDEX_FRONTMATTER_INVALID",
            message,
        })?;
    let value: Value = serde_saphyr::from_str(frontmatter).map_err(|error| IndexError {
        code: "BUNDLE_INDEX_FRONTMATTER_INVALID",
        message: format!("bundle index frontmatter is not valid YAML: {error}"),
    })?;
    let mapping = value.as_object().ok_or_else(|| IndexError {
        code: "BUNDLE_INDEX_FRONTMATTER_INVALID",
        message: "bundle index frontmatter must be a YAML mapping".to_owned(),
    })?;
    match mapping.get("okf_version") {
        Some(Value::String(version)) if version == VERSION => Ok(current.to_owned()),
        Some(_) => Err(IndexError {
            code: "BUNDLE_INDEX_VERSION_INCOMPATIBLE",
            message: format!("bundle index must declare okf_version: \"{VERSION}\""),
        }),
        None => {
            let first_end = current.find('\n').map_or(current.len(), |index| index + 1);
            Ok(format!(
                "{}okf_version: \"{VERSION}\"\n{}",
                &current[..first_end],
                &current[first_end..]
            ))
        }
    }
}

fn preamble() -> String {
    format!("---\nokf_version: \"{VERSION}\"\n---\n")
}

fn append(current: &str, block: &str) -> String {
    if current.trim().is_empty() {
        return block.to_owned();
    }
    let mut output = ensure_trailing_newline(current);
    if !output.ends_with("\n\n") {
        output.push('\n');
    }
    output.push_str(block);
    output
}

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
