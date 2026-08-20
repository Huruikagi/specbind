//! Document semantics for deterministic Requirement ID extraction.

use std::fmt;
use std::ops::Range;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptanceCriterion {
    pub id: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementGroup {
    pub number: u64,
    pub title: String,
    pub line: usize,
    pub criteria: Vec<AcceptanceCriterion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementsDocument {
    pub groups: Vec<RequirementGroup>,
}

impl RequirementsDocument {
    #[must_use]
    pub fn requirement_ids(&self) -> Vec<&str> {
        self.groups
            .iter()
            .flat_map(|group| group.criteria.iter())
            .map(|criterion| criterion.id.as_str())
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequirementsIssue {
    pub code: &'static str,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementsIssues {
    pub issues: Vec<RequirementsIssue>,
}

impl fmt::Display for RequirementsIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "requirements body has {} semantic issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for RequirementsIssues {}

#[derive(Debug)]
struct Heading {
    level: HeadingLevel,
    text: String,
    range: Range<usize>,
}

#[derive(Debug)]
struct OrderedList {
    start: u64,
    item_offsets: Vec<usize>,
    range: Range<usize>,
}

/// Parses the fixed Decision 0060 Markdown grammar and derives canonical `N.M` IDs.
///
/// # Errors
///
/// Returns all detected structural issues when the body cannot produce an
/// unambiguous canonical Requirement ID set.
pub fn parse(
    body: &str,
    requirement_label: &str,
    acceptance_criteria_label: &str,
) -> Result<RequirementsDocument, RequirementsIssues> {
    let (headings, lists) = blocks(body);
    let mut issues = Vec::new();
    let mut groups = Vec::new();

    for (index, heading) in headings.iter().enumerate() {
        if heading.level != HeadingLevel::H3 {
            continue;
        }
        let (number, title) = match parse_requirement_heading(&heading.text, requirement_label) {
            HeadingMatch::NotRequirement => continue,
            HeadingMatch::Malformed(message) => {
                issues.push(issue(
                    "REQUIREMENTS_HEADING_MALFORMED",
                    line_at(body, heading.range.start),
                    message,
                ));
                continue;
            }
            HeadingMatch::Requirement { number, title } => (number, title),
        };
        groups.push(build_group(
            body,
            &headings,
            &lists,
            index,
            (number, title),
            acceptance_criteria_label,
            &mut issues,
        ));
    }

    groups.sort_by_key(|group| group.number);
    for pair in groups.windows(2) {
        if pair[0].number == pair[1].number {
            issues.push(issue(
                "REQUIREMENTS_GROUP_DUPLICATE",
                pair[1].line,
                format!("Requirement group {} is duplicated", pair[1].number),
            ));
        }
    }
    if groups.is_empty() {
        issues.push(issue(
            "REQUIREMENTS_GROUP_MISSING",
            1,
            "requirements body must contain at least one valid Requirement heading",
        ));
    }

    if issues.is_empty() {
        Ok(RequirementsDocument { groups })
    } else {
        issues.sort();
        issues.dedup();
        Err(RequirementsIssues { issues })
    }
}

fn build_group(
    body: &str,
    headings: &[Heading],
    lists: &[OrderedList],
    index: usize,
    identity: (u64, String),
    acceptance_label: &str,
    issues: &mut Vec<RequirementsIssue>,
) -> RequirementGroup {
    let (number, title) = identity;
    let heading = &headings[index];
    let group_end = headings[index + 1..]
        .iter()
        .find(|candidate| candidate.level <= HeadingLevel::H3)
        .map_or(body.len(), |candidate| candidate.range.start);
    let group_headings = headings[index + 1..]
        .iter()
        .take_while(|candidate| candidate.range.start < group_end)
        .collect::<Vec<_>>();

    let acceptance_headings = group_headings
        .iter()
        .copied()
        .filter(|candidate| {
            candidate.level == HeadingLevel::H4 && candidate.text == acceptance_label
        })
        .collect::<Vec<_>>();
    let criteria = if acceptance_headings.len() == 1 {
        criteria_for(
            body,
            lists,
            &group_headings,
            acceptance_headings[0],
            group_end,
            number,
            issues,
        )
    } else {
        let code = if acceptance_headings.is_empty() {
            "REQUIREMENTS_ACCEPTANCE_HEADING_MISSING"
        } else {
            "REQUIREMENTS_ACCEPTANCE_HEADING_DUPLICATE"
        };
        issues.push(issue(
            code,
            line_at(body, heading.range.start),
            format!(
                "Requirement {number} must contain exactly one level-four {acceptance_label:?} heading"
            ),
        ));
        Vec::new()
    };
    RequirementGroup {
        number,
        title,
        line: line_at(body, heading.range.start),
        criteria,
    }
}

fn criteria_for(
    body: &str,
    lists: &[OrderedList],
    group_headings: &[&Heading],
    acceptance_heading: &Heading,
    group_end: usize,
    number: u64,
    issues: &mut Vec<RequirementsIssue>,
) -> Vec<AcceptanceCriterion> {
    let list_end = group_headings
        .iter()
        .find(|candidate| candidate.range.start > acceptance_heading.range.start)
        .map_or(group_end, |candidate| candidate.range.start);
    let matching_lists = lists
        .iter()
        .filter(|list| {
            list.range.start >= acceptance_heading.range.end && list.range.start < list_end
        })
        .collect::<Vec<_>>();
    if matching_lists.len() != 1 {
        let code = if matching_lists.is_empty() {
            "REQUIREMENTS_ACCEPTANCE_LIST_MISSING"
        } else {
            "REQUIREMENTS_ACCEPTANCE_LIST_MULTIPLE"
        };
        issues.push(issue(
            code,
            line_at(body, acceptance_heading.range.start),
            format!(
                "Requirement {number} must contain exactly one top-level ordered list after its Acceptance Criteria heading"
            ),
        ));
        return Vec::new();
    }

    let list = matching_lists[0];
    if list.start != 1 {
        issues.push(issue(
            "REQUIREMENTS_ACCEPTANCE_LIST_START",
            line_at(body, list.range.start),
            format!("Requirement {number} Acceptance Criteria list must start at one"),
        ));
    }
    if list.item_offsets.is_empty() {
        issues.push(issue(
            "REQUIREMENTS_ACCEPTANCE_LIST_EMPTY",
            line_at(body, list.range.start),
            format!("Requirement {number} Acceptance Criteria list must be non-empty"),
        ));
    }
    list.item_offsets
        .iter()
        .enumerate()
        .map(|(index, offset)| AcceptanceCriterion {
            id: format!("{number}.{}", index + 1),
            line: line_at(body, *offset),
        })
        .collect()
}

enum HeadingMatch {
    NotRequirement,
    Malformed(String),
    Requirement { number: u64, title: String },
}

fn parse_requirement_heading(text: &str, label: &str) -> HeadingMatch {
    if text != label && !text.starts_with(&format!("{label} ")) {
        return HeadingMatch::NotRequirement;
    }
    let Some(remainder) = text.strip_prefix(&format!("{label} ")) else {
        return HeadingMatch::Malformed(format!(
            "Requirement heading must use `{label} <N>: <title>`"
        ));
    };
    let Some((number, title)) = remainder.split_once(": ") else {
        return HeadingMatch::Malformed(format!(
            "Requirement heading must use `{label} <N>: <title>`"
        ));
    };
    if number.is_empty()
        || number.starts_with('0')
        || !number.bytes().all(|byte| byte.is_ascii_digit())
    {
        return HeadingMatch::Malformed(
            "Requirement group number must be a positive ASCII integer without leading zeroes"
                .to_owned(),
        );
    }
    let Ok(number) = number.parse::<u64>() else {
        return HeadingMatch::Malformed("Requirement group number is too large".to_owned());
    };
    if title.trim().is_empty() {
        return HeadingMatch::Malformed("Requirement title must be non-empty".to_owned());
    }
    HeadingMatch::Requirement {
        number,
        title: title.trim().to_owned(),
    }
}

fn blocks(body: &str) -> (Vec<Heading>, Vec<OrderedList>) {
    let mut headings = Vec::new();
    let mut lists = Vec::new();
    let mut current_heading: Option<Heading> = None;
    let mut list_depth = 0_usize;
    let mut block_quote_depth = 0_usize;
    let mut current_list: Option<OrderedList> = None;

    for (event, range) in Parser::new(body).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current_heading = Some(Heading {
                    level,
                    text: String::new(),
                    range,
                });
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(mut heading) = current_heading.take() {
                    heading.range.end = range.end;
                    headings.push(heading);
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = &mut current_heading {
                    heading.text.push_str(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some(heading) = &mut current_heading {
                    heading.text.push(' ');
                }
            }
            Event::Start(Tag::List(start)) => {
                if list_depth == 0
                    && block_quote_depth == 0
                    && let Some(start) = start
                {
                    current_list = Some(OrderedList {
                        start,
                        item_offsets: Vec::new(),
                        range: range.clone(),
                    });
                }
                list_depth += 1;
            }
            Event::Start(Tag::Item) if list_depth == 1 => {
                if let Some(list) = &mut current_list {
                    list.item_offsets.push(range.start);
                }
            }
            Event::End(TagEnd::List(_)) => {
                list_depth = list_depth.saturating_sub(1);
                if list_depth == 0
                    && let Some(mut list) = current_list.take()
                {
                    list.range.end = range.end;
                    lists.push(list);
                }
            }
            Event::Start(Tag::BlockQuote(_)) => block_quote_depth += 1,
            Event::End(TagEnd::BlockQuote(_)) => {
                block_quote_depth = block_quote_depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    (headings, lists)
}

fn line_at(body: &str, offset: usize) -> usize {
    body[..offset].bytes().filter(|byte| *byte == b'\n').count() + 1
}

fn issue(code: &'static str, line: usize, message: impl Into<String>) -> RequirementsIssue {
    RequirementsIssue {
        code,
        line,
        message: message.into(),
    }
}
