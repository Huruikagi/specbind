//! Scoped agent instructions embedded in managed Markdown comments.

use std::ops::Range;

use pulldown_cmark::{Event, Parser};

const PREFIX: &str = "specbind:instruction";

/// The lifecycle audience named by one managed instruction comment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionScope {
    /// Template-only guidance used while the artifact is first materialized.
    Create,
    /// Durable guidance used whenever the materialized artifact is revised.
    Maintain,
    /// Durable guidance used when the materialized artifact is consumed.
    Consume,
}

impl InstructionScope {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "create" => Some(Self::Create),
            "maintain" => Some(Self::Maintain),
            "consume" => Some(Self::Consume),
            _ => None,
        }
    }
}

/// One deterministic instruction syntax or lifecycle fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionIssue {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    scope: InstructionScope,
    range: Range<usize>,
}

/// Validates every instruction in a managed template body.
#[must_use]
pub fn validate_template(body: &str) -> Vec<InstructionIssue> {
    inspect(body).1
}

/// Validates every instruction in a live artifact body.
///
/// `create` is deliberately template-only. The two durable scopes are valid
/// live-artifact content and remain available to purpose-filtered reads.
#[must_use]
pub fn validate_live(body: &str) -> Vec<InstructionIssue> {
    let (instructions, mut issues) = inspect(body);
    if instructions
        .iter()
        .any(|instruction| instruction.scope == InstructionScope::Create)
    {
        issues.push(InstructionIssue {
            code: "ARTIFACT_CREATE_INSTRUCTION_LEAK",
            message: "live artifact contains a template-only create instruction".to_owned(),
        });
    }
    issues
}

/// Removes scoped instruction comments not addressed to `scope`.
///
/// The remaining Markdown bytes are preserved exactly, including Front Matter,
/// ordinary comments, and whitespace around removed instruction nodes.
#[must_use]
pub fn project(content: &str, scope: InstructionScope) -> String {
    let (instructions, _) = inspect(content);
    let mut output = String::with_capacity(content.len());
    let mut cursor = 0;
    for instruction in instructions {
        if instruction.scope == scope {
            continue;
        }
        output.push_str(&content[cursor..instruction.range.start]);
        cursor = instruction.range.end;
    }
    output.push_str(&content[cursor..]);
    output
}

/// Masks every valid scoped instruction while retaining exact line boundaries.
///
/// Artifact body parsers use this projection so durable agent guidance does not
/// become semantic prose and diagnostic line numbers remain stable.
#[must_use]
pub fn mask(content: &str) -> String {
    let mut bytes = content.as_bytes().to_vec();
    for range in comment_ranges(content) {
        if instruction_suffix(&content[range.clone()]).is_none() {
            continue;
        }
        for byte in &mut bytes[range] {
            if !matches!(*byte, b'\r' | b'\n') {
                *byte = b' ';
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn inspect(content: &str) -> (Vec<Instruction>, Vec<InstructionIssue>) {
    let mut instructions = Vec::new();
    let mut issues = Vec::new();
    for range in comment_ranges(content) {
        let Some(suffix) = instruction_suffix(&content[range.clone()]) else {
            continue;
        };
        let remainder = suffix.trim_start();
        let Some(token) = remainder.split_whitespace().next() else {
            issues.push(InstructionIssue {
                code: "INSTRUCTION_SCOPE_MISSING",
                message: "specbind:instruction must name create, maintain, or consume".to_owned(),
            });
            continue;
        };
        let Some(scope) = InstructionScope::parse(token) else {
            issues.push(InstructionIssue {
                code: "INSTRUCTION_SCOPE_INVALID",
                message: format!(
                    "unknown specbind:instruction scope {token}; expected create, maintain, or consume"
                ),
            });
            continue;
        };
        instructions.push(Instruction { scope, range });
    }
    (instructions, issues)
}

fn comment_ranges(content: &str) -> Vec<Range<usize>> {
    let mut comments = Vec::new();
    let mut open: Option<Range<usize>> = None;
    for (event, range) in Parser::new(content).into_offset_iter() {
        let (Event::Html(value) | Event::InlineHtml(value)) = event else {
            open = None;
            continue;
        };
        match &mut open {
            Some(comment) if comment.end == range.start => comment.end = range.end,
            Some(_) => {
                open = None;
                if value.trim_start().starts_with("<!--") {
                    open = Some(range.clone());
                }
            }
            None if value.trim_start().starts_with("<!--") => open = Some(range.clone()),
            None => {}
        }
        if let Some(comment) = &open
            && content[comment.clone()].trim_end().ends_with("-->")
        {
            comments.push(comment.clone());
            open = None;
        }
    }
    comments
}

fn comment_content(value: &str) -> Option<&str> {
    value.trim().strip_prefix("<!--")?.strip_suffix("-->")
}

fn instruction_suffix(value: &str) -> Option<&str> {
    let comment = comment_content(value)?;
    let suffix = comment.trim().strip_prefix(PREFIX)?;
    (suffix.is_empty() || suffix.starts_with(char::is_whitespace)).then_some(suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projects_only_the_requested_complete_comment_nodes() {
        let source = "before\n<!-- specbind:instruction maintain Keep this. -->\n<!-- specbind:instruction consume Read this. -->\n`<!-- specbind:instruction consume code -->`\nafter\n";
        let projected = project(source, InstructionScope::Consume);
        assert!(!projected.contains("Keep this."));
        assert!(projected.contains("Read this."));
        assert!(projected.contains("code"));
        assert!(projected.starts_with("before\n"));
        assert!(projected.ends_with("after\n"));
    }

    #[test]
    fn rejects_missing_and_unknown_scopes() {
        let issues = validate_template(
            "<!-- specbind:instruction -->\n<!-- specbind:instruction revise Later. -->\n",
        );
        assert_eq!(
            issues.iter().map(|issue| issue.code).collect::<Vec<_>>(),
            ["INSTRUCTION_SCOPE_MISSING", "INSTRUCTION_SCOPE_INVALID"]
        );
    }

    #[test]
    fn live_artifacts_accept_durable_scopes_but_reject_create() {
        assert!(validate_live("<!-- specbind:instruction maintain Keep. -->").is_empty());
        assert!(validate_live("<!-- specbind:instruction consume Read. -->").is_empty());
        assert_eq!(
            validate_live("<!-- specbind:instruction create Start. -->")[0].code,
            "ARTIFACT_CREATE_INSTRUCTION_LEAK"
        );
    }

    #[test]
    fn masks_instructions_without_changing_line_count() {
        let source = "before\n<!-- specbind:instruction maintain\nKeep.\n-->\nafter\n";
        let masked = mask(source);
        assert!(!masked.contains("Keep."));
        assert_eq!(masked.lines().count(), source.lines().count());
        assert!(masked.starts_with("before\n"));
        assert!(masked.ends_with("after\n"));
    }

    #[test]
    fn masks_durable_guidance_out_of_the_contract_grammar() {
        let template = include_str!("../../assets/templates/en/specs/contract.md");
        let body = template.split_once("\n---\n").expect("frontmatter").1;
        let masked = mask(body);
        assert!(
            crate::contract::parse(&masked).is_ok(),
            "masked body:\n{masked}"
        );
    }
}
