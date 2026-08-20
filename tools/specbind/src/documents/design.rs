//! Document semantics for Design Markdown Requirement traceability.

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Range;

use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::domain;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignMarker {
    pub requirement_ids: Vec<String>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignTraceability {
    pub markers: Vec<DesignMarker>,
    pub requirement_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DesignIssue {
    pub code: &'static str,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignIssues {
    pub issues: Vec<DesignIssue>,
}

impl fmt::Display for DesignIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "design body has {} traceability issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for DesignIssues {}

#[derive(Debug)]
struct PendingEmphasis {
    text: String,
    valid: bool,
    range: Range<usize>,
}

/// Validates Decision 0061 marker equality against the declared Front Matter set.
///
/// # Errors
///
/// Returns deterministic issues when no exact marker exists or either set contains
/// an ID absent from the other.
pub fn validate(body: &str, declared_ids: &[String]) -> Result<DesignTraceability, DesignIssues> {
    let markers = extract_markers(body);
    let declared = declared_ids.iter().cloned().collect::<BTreeSet<_>>();
    let marked = markers
        .iter()
        .flat_map(|marker| marker.requirement_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let mut issues = Vec::new();

    if markers.is_empty() {
        issues.push(issue(
            "DESIGN_REQUIREMENT_MARKER_MISSING",
            1,
            "design body must contain at least one exact italic Requirements marker",
        ));
    }
    for id in declared.difference(&marked) {
        issues.push(issue(
            "DESIGN_BODY_REQUIREMENT_ID_MISSING",
            1,
            format!("Front Matter Requirement ID {id} is absent from body markers"),
        ));
    }
    for id in marked.difference(&declared) {
        let line = markers
            .iter()
            .find(|marker| marker.requirement_ids.contains(id))
            .map_or(1, |marker| marker.line);
        issues.push(issue(
            "DESIGN_FRONTMATTER_REQUIREMENT_ID_MISSING",
            line,
            format!("body marker Requirement ID {id} is absent from Front Matter"),
        ));
    }

    if issues.is_empty() {
        Ok(DesignTraceability {
            markers,
            requirement_ids: sorted_ids(marked),
        })
    } else {
        issues.sort();
        issues.dedup();
        Err(DesignIssues { issues })
    }
}

fn extract_markers(body: &str) -> Vec<DesignMarker> {
    let mut markers = Vec::new();
    let mut emphasis_depth = 0_usize;
    let mut pending: Option<PendingEmphasis> = None;

    for (event, range) in Parser::new(body).into_offset_iter() {
        match event {
            Event::Start(Tag::Emphasis) => {
                if emphasis_depth == 0 {
                    pending = Some(PendingEmphasis {
                        text: String::new(),
                        valid: true,
                        range: range.clone(),
                    });
                } else if let Some(value) = &mut pending {
                    value.valid = false;
                }
                emphasis_depth += 1;
            }
            Event::End(TagEnd::Emphasis) => {
                emphasis_depth = emphasis_depth.saturating_sub(1);
                if emphasis_depth == 0
                    && let Some(mut value) = pending.take()
                {
                    value.range.end = range.end;
                    if value.valid
                        && let Some(requirement_ids) = parse_marker_text(&value.text)
                    {
                        markers.push(DesignMarker {
                            requirement_ids,
                            line: line_at(body, value.range.start),
                        });
                    }
                }
            }
            Event::Text(text) if emphasis_depth > 0 => {
                if let Some(value) = &mut pending {
                    value.text.push_str(&text);
                }
            }
            Event::Start(_)
            | Event::Code(_)
            | Event::InlineMath(_)
            | Event::DisplayMath(_)
            | Event::SoftBreak
            | Event::HardBreak
            | Event::Html(_)
            | Event::InlineHtml(_)
                if emphasis_depth > 0 =>
            {
                if let Some(value) = &mut pending {
                    value.valid = false;
                }
            }
            _ => {}
        }
    }
    markers
}

fn parse_marker_text(text: &str) -> Option<Vec<String>> {
    let values = text.strip_prefix("Requirements: ")?.split(", ");
    let ids = values.map(str::to_owned).collect::<Vec<_>>();
    if ids.is_empty()
        || ids
            .iter()
            .any(|id| domain::parse_requirement_id(id).is_none())
        || format!("Requirements: {}", ids.join(", ")) != text
    {
        return None;
    }
    Some(ids)
}

fn sorted_ids(ids: BTreeSet<String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort_by_key(|id| domain::parse_requirement_id(id));
    ids
}

fn line_at(body: &str, offset: usize) -> usize {
    body[..offset].bytes().filter(|byte| *byte == b'\n').count() + 1
}

fn issue(code: &'static str, line: usize, message: impl Into<String>) -> DesignIssue {
    DesignIssue {
        code,
        line,
        message: message.into(),
    }
}
