//! Scoped agent instructions embedded in managed Markdown comments.

use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};

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
    output: Option<String>,
    range: Range<usize>,
}

/// Validates every instruction in a managed template body.
#[must_use]
pub fn validate_template(body: &str) -> Vec<InstructionIssue> {
    let (instructions, mut issues) = inspect(body);
    issues.extend(validate_outputs(body, &instructions));
    issues
}

/// Validates scoped instructions and named creation outputs in a Spec artifact
/// template.
#[must_use]
pub fn validate_spec_template(body: &str, _artifact_id: Option<&str>) -> Vec<InstructionIssue> {
    validate_template(body)
}

/// Rejects named output references from template Front Matter.
///
/// Machine identity remains literal under Decision 0059. Named outputs are
/// deliberately limited to the Markdown body.
#[must_use]
pub fn validate_template_frontmatter(frontmatter: &str) -> Vec<InstructionIssue> {
    output_references(frontmatter)
        .into_iter()
        .map(|name| InstructionIssue {
            code: "TEMPLATE_OUTPUT_FRONTMATTER_FORBIDDEN",
            message: format!("template output reference {name} is forbidden in Front Matter"),
        })
        .collect()
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
    if !output_references(&mask_instruction_ranges(body, &instructions)).is_empty() {
        issues.push(InstructionIssue {
            code: "ARTIFACT_TEMPLATE_OUTPUT_LEAK",
            message: "live artifact contains an unresolved template output reference".to_owned(),
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
        let output = remainder
            .split_whitespace()
            .nth(1)
            .and_then(|token| token.strip_prefix("output="))
            .map(ToOwned::to_owned);
        if output.as_deref().is_some_and(str::is_empty) {
            issues.push(InstructionIssue {
                code: "INSTRUCTION_OUTPUT_INVALID",
                message: "specbind:instruction output must name a creation output".to_owned(),
            });
        }
        if output.is_some() && scope != InstructionScope::Create {
            issues.push(InstructionIssue {
                code: "INSTRUCTION_OUTPUT_SCOPE_INVALID",
                message: "only a create instruction may produce a named template output".to_owned(),
            });
        }
        instructions.push(Instruction {
            scope,
            output,
            range,
        });
    }
    (instructions, issues)
}

fn validate_outputs(body: &str, instructions: &[Instruction]) -> Vec<InstructionIssue> {
    let mut issues = Vec::new();
    let references = output_references(&mask_instruction_ranges(body, instructions));
    let mut outputs: BTreeMap<&str, usize> = BTreeMap::new();
    for instruction in instructions {
        let Some(output) = instruction.output.as_deref() else {
            continue;
        };
        if !valid_output_name(output) {
            issues.push(InstructionIssue {
                code: "TEMPLATE_OUTPUT_NAME_INVALID",
                message: format!(
                    "template output name {output:?} must be non-empty and contain no whitespace or braces"
                ),
            });
        }
        *outputs.entry(output).or_default() += 1;
    }
    for (output, count) in &outputs {
        if *count > 1 {
            issues.push(InstructionIssue {
                code: "TEMPLATE_OUTPUT_DUPLICATE",
                message: format!(
                    "template output {output} is produced by more than one create instruction"
                ),
            });
        }
        if !references.contains(*output) {
            issues.push(InstructionIssue {
                code: "TEMPLATE_OUTPUT_UNUSED",
                message: format!("create instruction produces unused template output {output}"),
            });
        }
    }
    for reference in references {
        if !valid_output_name(&reference) {
            issues.push(InstructionIssue {
                code: "TEMPLATE_OUTPUT_NAME_INVALID",
                message: format!(
                    "template output name {reference:?} must be non-empty and contain no whitespace or braces"
                ),
            });
        }
        if !outputs.contains_key(reference.as_str()) {
            issues.push(InstructionIssue {
                code: "TEMPLATE_OUTPUT_MISSING",
                message: format!(
                    "template output reference {reference} requires exactly one create instruction with output={reference}"
                ),
            });
        }
    }
    issues.sort_by(|left, right| {
        left.code
            .cmp(right.code)
            .then(left.message.cmp(&right.message))
    });
    issues.dedup();
    issues
}

fn output_references(content: &str) -> BTreeSet<String> {
    let mut references = BTreeSet::new();
    let mut rest = content;
    while let Some(start) = rest.find("{{") {
        let after = &rest[start + 2..];
        let Some(end) = after.find("}}") else {
            break;
        };
        let name = &after[..end];
        references.insert(name.to_owned());
        rest = &after[end + 2..];
    }
    references
}

fn valid_output_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| !character.is_whitespace() && !matches!(character, '{' | '}'))
}

fn mask_instruction_ranges(content: &str, instructions: &[Instruction]) -> String {
    let mut bytes = content.as_bytes().to_vec();
    for instruction in instructions {
        for byte in &mut bytes[instruction.range.clone()] {
            if !matches!(*byte, b'\r' | b'\n') {
                *byte = b' ';
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
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
    fn requires_one_create_instruction_for_each_named_output() {
        let valid = concat!(
            "<!-- specbind:instruction create output=今日の天気\n",
            "Fetch Tokyo's current weather.\n",
            "-->\n",
            "{{今日の天気}}の日に作成。{{今日の天気}}に合わせる。\n",
        );
        assert!(validate_spec_template(valid, None).is_empty());

        let missing = validate_spec_template("# `{{title}}` Requirements\n", None);
        assert_eq!(missing[0].code, "TEMPLATE_OUTPUT_MISSING");

        let unused = validate_spec_template(
            "<!-- specbind:instruction create output=spec Render the identity. -->\n# Requirements\n",
            None,
        );
        assert_eq!(unused[0].code, "TEMPLATE_OUTPUT_UNUSED");

        let duplicate = validate_spec_template(
            concat!(
                "<!-- specbind:instruction create output=spec First. -->\n",
                "<!-- specbind:instruction create output=spec Second. -->\n",
                "# `{{spec}}` Requirements\n",
            ),
            None,
        );
        assert!(
            duplicate
                .iter()
                .any(|issue| issue.code == "TEMPLATE_OUTPUT_DUPLICATE")
        );

        let durable = validate_spec_template(
            concat!(
                "<!-- specbind:instruction maintain output=spec Keep it. -->\n",
                "# `{{spec}}` Requirements\n",
            ),
            None,
        );
        assert!(
            durable
                .iter()
                .any(|issue| issue.code == "INSTRUCTION_OUTPUT_SCOPE_INVALID")
        );

        let legacy_bind = validate_spec_template(
            concat!(
                "<!-- specbind:instruction create bind=spec Legacy syntax. -->\n",
                "# `{{spec}}` Requirements\n",
            ),
            None,
        );
        assert!(
            legacy_bind
                .iter()
                .any(|issue| issue.code == "TEMPLATE_OUTPUT_MISSING")
        );
    }

    #[test]
    fn accepts_agent_produced_outputs_in_every_managed_template() {
        let source = concat!(
            "<!-- specbind:instruction create output=audience Ask who will read this. -->\n",
            "# Guidance for {{audience}}\n",
        );
        assert!(validate_template(source).is_empty());
    }

    #[test]
    fn treats_previous_built_ins_as_ordinary_agent_produced_outputs() {
        let source = concat!(
            "<!-- specbind:instruction create output=spec Use the Spec identity. -->\n",
            "<!-- specbind:instruction create output=artifact_id Use the artifact identity. -->\n",
            "# `{{spec}}` Design — `{{artifact_id}}`\n",
        );
        assert!(validate_spec_template(source, None).is_empty());
    }

    #[test]
    fn rejects_invalid_output_names() {
        let issues = validate_template(concat!(
            "<!-- specbind:instruction create output=valid Produce it. -->\n",
            "{{bad name}} {{}} {{valid}}\n",
        ));
        assert!(
            issues
                .iter()
                .any(|issue| issue.code == "TEMPLATE_OUTPUT_NAME_INVALID")
        );
    }

    #[test]
    fn live_artifacts_reject_unresolved_output_references() {
        let issues = validate_live("{{今日の天気}}の日に作成。\n");
        assert_eq!(issues[0].code, "ARTIFACT_TEMPLATE_OUTPUT_LEAK");
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
}
