//! Document semantics for the canonical `SpecBind` Contract Markdown profile.

use std::collections::BTreeSet;
use std::fmt;
use std::ops::Range;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

const SECTION_NAMES: [&str; 5] = [
    "Owns",
    "Exports",
    "Consumes",
    "Invariants",
    "File Ownership",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ContractSection {
    Owns,
    Exports,
    Consumes,
    Invariants,
    FileOwnership,
}

impl ContractSection {
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Owns => "owns",
            Self::Exports => "exports",
            Self::Consumes => "consumes",
            Self::Invariants => "invariants",
            Self::FileOwnership => "file-ownership",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescribedEntry {
    pub id: String,
    pub description: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractTarget {
    pub canonical_spec: String,
    pub section: ContractSection,
    pub entry_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsumesEntry {
    pub id: String,
    pub target: ContractTarget,
    pub description: Option<String>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileOwnershipEntry {
    pub id: String,
    pub paths: Vec<String>,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContractDocument {
    pub owns: Vec<DescribedEntry>,
    pub exports: Vec<DescribedEntry>,
    pub consumes: Vec<ConsumesEntry>,
    pub invariants: Vec<DescribedEntry>,
    pub file_ownership: Vec<FileOwnershipEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractIssue {
    pub code: &'static str,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractIssues {
    pub issues: Vec<ContractIssue>,
}

impl fmt::Display for ContractIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "contract body has {} semantic issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for ContractIssues {}

#[derive(Debug)]
enum Block {
    Heading {
        level: HeadingLevel,
        text: String,
        plain: bool,
        line: usize,
    },
    List {
        ordered: bool,
        items: Vec<ListItem>,
        line: usize,
    },
    Unsupported {
        line: usize,
    },
}

#[derive(Debug)]
struct ListItem {
    tokens: Vec<InlineToken>,
    valid: bool,
    line: usize,
}

#[derive(Debug)]
enum InlineToken {
    Code(String),
    Text(String),
}

/// Parses the fixed Decision 0056 Contract Markdown grammar.
///
/// # Errors
///
/// Returns deterministic structural and entry diagnostics when the body cannot
/// produce an unambiguous typed Contract.
pub fn parse(body: &str) -> Result<ContractDocument, ContractIssues> {
    let blocks = blocks(body);
    let mut issues = Vec::new();
    let mut document = ContractDocument::default();
    let mut cursor = 0;

    match blocks.first() {
        Some(Block::Heading {
            level: HeadingLevel::H1,
            text,
            plain: true,
            ..
        }) if text == "Contract" => cursor += 1,
        Some(block) => {
            issues.push(issue(
                "CONTRACT_ROOT_HEADING_INVALID",
                block.line(),
                "contract body must begin with the exact plain heading # Contract",
            ));
            if matches!(block, Block::Heading { .. }) {
                cursor += 1;
            }
        }
        None => issues.push(issue(
            "CONTRACT_ROOT_HEADING_INVALID",
            1,
            "contract body must begin with the exact plain heading # Contract",
        )),
    }

    parse_sections(&blocks, &mut cursor, &mut document, &mut issues, body);

    for block in &blocks[cursor..] {
        issues.push(issue(
            "CONTRACT_DOCUMENT_CONTENT_INVALID",
            block.line(),
            "contract body contains content after the File Ownership section",
        ));
    }

    validate_unique_ids(&document, &mut issues);
    if issues.is_empty() {
        Ok(document)
    } else {
        issues.sort();
        issues.dedup();
        Err(ContractIssues { issues })
    }
}

/// Walks the five canonical sections in order, advancing `cursor` past each.
fn parse_sections(
    blocks: &[Block],
    cursor: &mut usize,
    document: &mut ContractDocument,
    issues: &mut Vec<ContractIssue>,
    body: &str,
) {
    for (section_index, expected_name) in SECTION_NAMES.iter().enumerate() {
        let section = section_for_index(section_index);

        // Content sitting where a section heading belongs is reported once and
        // skipped, so the walk stays aligned with the document.
        //
        // Without this it desynchronizes: the stray block is blamed as a
        // malformed heading, the real heading is then read as the *next*
        // section's, and every following list is parsed under the wrong
        // section. One stray line reported eleven diagnostics, five of them
        // claiming a correct heading was wrong, which sends a reader to five
        // places that are all fine.
        while let Some(block) = blocks.get(*cursor) {
            if matches!(block, Block::Heading { .. }) {
                break;
            }
            issues.push(issue(
                "CONTRACT_DOCUMENT_CONTENT_INVALID",
                block.line(),
                format!("contract body contains content before section {expected_name}"),
            ));
            *cursor += 1;
        }

        match blocks.get(*cursor) {
            Some(Block::Heading {
                level: HeadingLevel::H2,
                text,
                plain: true,
                ..
            }) if text == expected_name => *cursor += 1,
            Some(block) => {
                issues.push(issue(
                    "CONTRACT_SECTION_HEADING_INVALID",
                    block.line(),
                    format!("expected exact section heading ## {expected_name}"),
                ));
                if matches!(block, Block::Heading { .. }) {
                    *cursor += 1;
                }
            }
            None => {
                issues.push(issue(
                    "CONTRACT_SECTION_HEADING_MISSING",
                    line_at_end(body),
                    format!("missing required section heading ## {expected_name}"),
                ));
                continue;
            }
        }

        if let Some(Block::List {
            ordered,
            items,
            line,
        }) = blocks.get(*cursor)
        {
            if *ordered {
                issues.push(issue(
                    "CONTRACT_SECTION_LIST_ORDERED",
                    *line,
                    format!("section {expected_name} must use an unordered list"),
                ));
            }
            parse_items(section, items, document, issues);
            *cursor += 1;
        }

        while let Some(block) = blocks.get(*cursor) {
            if matches!(block, Block::Heading { .. }) {
                break;
            }
            issues.push(issue(
                "CONTRACT_SECTION_CONTENT_INVALID",
                block.line(),
                format!("section {expected_name} may contain only one flat unordered list"),
            ));
            *cursor += 1;
        }
    }
}

impl Block {
    const fn line(&self) -> usize {
        match self {
            Self::Heading { line, .. } | Self::List { line, .. } | Self::Unsupported { line } => {
                *line
            }
        }
    }
}

fn blocks(body: &str) -> Vec<Block> {
    let events = Parser::new(body).into_offset_iter().collect::<Vec<_>>();
    let mut blocks = Vec::new();
    let mut cursor = 0;
    while cursor < events.len() {
        let (event, range) = &events[cursor];
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let (text, plain, next) = heading(&events, cursor + 1, *level);
                blocks.push(Block::Heading {
                    level: *level,
                    text,
                    plain,
                    line: line_at(body, range.start),
                });
                cursor = next;
            }
            Event::Start(Tag::List(start)) => {
                let (items, next) = list(body, &events, cursor + 1, start.is_some());
                blocks.push(Block::List {
                    ordered: start.is_some(),
                    items,
                    line: line_at(body, range.start),
                });
                cursor = next;
            }
            Event::End(_) => cursor += 1,
            _ => {
                blocks.push(Block::Unsupported {
                    line: line_at(body, range.start),
                });
                cursor = skip_block(&events, cursor);
            }
        }
    }
    blocks
}

fn heading(
    events: &[(Event<'_>, Range<usize>)],
    mut cursor: usize,
    level: HeadingLevel,
) -> (String, bool, usize) {
    let mut text = String::new();
    let mut plain = true;
    while let Some((event, _)) = events.get(cursor) {
        match event {
            Event::End(TagEnd::Heading(end_level)) if *end_level == level => {
                return (text, plain, cursor + 1);
            }
            Event::Text(value) => text.push_str(value),
            _ => plain = false,
        }
        cursor += 1;
    }
    (text, false, cursor)
}

fn list(
    body: &str,
    events: &[(Event<'_>, Range<usize>)],
    mut cursor: usize,
    ordered: bool,
) -> (Vec<ListItem>, usize) {
    let mut items = Vec::new();
    while let Some((event, range)) = events.get(cursor) {
        match event {
            Event::End(TagEnd::List(end_ordered)) if *end_ordered == ordered => {
                return (items, cursor + 1);
            }
            Event::Start(Tag::Item) => {
                let (item, next) = list_item(body, events, cursor + 1, range.start);
                items.push(item);
                cursor = next;
            }
            _ => cursor += 1,
        }
    }
    (items, cursor)
}

fn list_item(
    body: &str,
    events: &[(Event<'_>, Range<usize>)],
    mut cursor: usize,
    start: usize,
) -> (ListItem, usize) {
    let mut tokens = Vec::new();
    let mut valid = true;
    let mut paragraph_count = 0;
    let mut item_depth = 1;
    while let Some((event, _)) = events.get(cursor) {
        match event {
            Event::End(TagEnd::Item) => {
                item_depth -= 1;
                if item_depth > 0 {
                    valid = false;
                    cursor += 1;
                    continue;
                }
                return (
                    ListItem {
                        tokens,
                        valid,
                        line: line_at(body, start),
                    },
                    cursor + 1,
                );
            }
            Event::Code(value) => tokens.push(InlineToken::Code(value.to_string())),
            Event::Text(value) => tokens.push(InlineToken::Text(value.to_string())),
            Event::Start(Tag::Paragraph) => {
                paragraph_count += 1;
                valid &= paragraph_count == 1;
            }
            Event::Start(Tag::Item) => {
                item_depth += 1;
                valid = false;
            }
            Event::End(TagEnd::Paragraph | TagEnd::Emphasis | TagEnd::Strong | TagEnd::Link)
            | Event::Start(Tag::Emphasis | Tag::Strong | Tag::Link { .. }) => {}
            _ => valid = false,
        }
        cursor += 1;
    }
    (
        ListItem {
            tokens,
            valid: false,
            line: line_at(body, start),
        },
        cursor,
    )
}

fn skip_block(events: &[(Event<'_>, Range<usize>)], cursor: usize) -> usize {
    let Event::Start(tag) = &events[cursor].0 else {
        return cursor + 1;
    };
    let end = tag.to_end();
    let mut depth = 1;
    let mut cursor = cursor + 1;
    while let Some((event, _)) = events.get(cursor) {
        if matches!(event, Event::Start(tag) if tag.to_end() == end) {
            depth += 1;
        } else if matches!(event, Event::End(tag) if *tag == end) {
            depth -= 1;
            if depth == 0 {
                return cursor + 1;
            }
        }
        cursor += 1;
    }
    cursor
}

fn parse_items(
    section: ContractSection,
    items: &[ListItem],
    document: &mut ContractDocument,
    issues: &mut Vec<ContractIssue>,
) {
    for item in items {
        if !item.valid {
            issues.push(issue(
                "CONTRACT_ENTRY_STRUCTURE_INVALID",
                item.line,
                "contract entries must be single-paragraph flat list items",
            ));
            continue;
        }
        match section {
            ContractSection::Owns | ContractSection::Exports | ContractSection::Invariants => {
                match parse_described(item) {
                    Ok(entry) => match section {
                        ContractSection::Owns => document.owns.push(entry),
                        ContractSection::Exports => document.exports.push(entry),
                        ContractSection::Invariants => document.invariants.push(entry),
                        ContractSection::Consumes | ContractSection::FileOwnership => {
                            unreachable!()
                        }
                    },
                    Err(message) => issues.push(issue(
                        "CONTRACT_DESCRIBED_ENTRY_INVALID",
                        item.line,
                        message,
                    )),
                }
            }
            ContractSection::Consumes => match parse_consumes(item) {
                Ok(entry) => document.consumes.push(entry),
                Err(message) => {
                    issues.push(issue("CONTRACT_CONSUMES_ENTRY_INVALID", item.line, message));
                }
            },
            ContractSection::FileOwnership => match parse_file_ownership(item) {
                Ok(entry) => document.file_ownership.push(entry),
                Err(message) => issues.push(issue(
                    "CONTRACT_FILE_OWNERSHIP_ENTRY_INVALID",
                    item.line,
                    message,
                )),
            },
        }
    }
}

fn parse_described(item: &ListItem) -> Result<DescribedEntry, String> {
    let Some(InlineToken::Code(id)) = item.tokens.first() else {
        return Err("entry must begin with an inline-code stable ID".to_owned());
    };
    validate_id(id)?;
    let description = description_after(&item.tokens[1..], " — ")?;
    Ok(DescribedEntry {
        id: id.clone(),
        description,
        line: item.line,
    })
}

fn parse_consumes(item: &ListItem) -> Result<ConsumesEntry, String> {
    let [
        InlineToken::Code(id),
        InlineToken::Text(arrow),
        InlineToken::Code(target),
        rest @ ..,
    ] = item.tokens.as_slice()
    else {
        return Err(
            "Consumes entry must use `<local-id>` → `<spec>/<section>/<entry-id>`".to_owned(),
        );
    };
    validate_id(id)?;
    if arrow != " → " {
        return Err("Consumes entry must use the exact ` → ` separator".to_owned());
    }
    let target = parse_target(target)?;
    let description = if rest.is_empty() {
        None
    } else {
        Some(description_after(rest, " — ")?)
    };
    Ok(ConsumesEntry {
        id: id.clone(),
        target,
        description,
        line: item.line,
    })
}

fn parse_file_ownership(item: &ListItem) -> Result<FileOwnershipEntry, String> {
    let [
        InlineToken::Code(id),
        InlineToken::Text(separator),
        rest @ ..,
    ] = item.tokens.as_slice()
    else {
        return Err("File Ownership entry must begin with `<id>` — `<path>`".to_owned());
    };
    validate_id(id)?;
    if separator != " — " || rest.is_empty() {
        return Err("File Ownership entry must use the exact ` — ` separator".to_owned());
    }
    let mut paths = Vec::new();
    for (index, token) in rest.iter().enumerate() {
        match (index % 2, token) {
            (0, InlineToken::Code(path)) => {
                validate_path(path)?;
                paths.push(path.clone());
            }
            (1, InlineToken::Text(separator)) if separator == ", " => {}
            _ => {
                return Err(
                    "File Ownership paths must be inline code separated by comma and space"
                        .to_owned(),
                );
            }
        }
    }
    if rest.len().is_multiple_of(2) {
        return Err("File Ownership entry has a trailing path separator".to_owned());
    }
    let unique = paths
        .iter()
        .map(|path| path.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    if unique.len() != paths.len() {
        return Err("File Ownership paths must be unique ignoring ASCII case".to_owned());
    }
    Ok(FileOwnershipEntry {
        id: id.clone(),
        paths,
        line: item.line,
    })
}

fn description_after(tokens: &[InlineToken], separator: &str) -> Result<String, String> {
    let Some(InlineToken::Text(first)) = tokens.first() else {
        return Err(format!("entry must use the exact `{separator}` separator"));
    };
    let Some(first) = first.strip_prefix(separator) else {
        return Err(format!("entry must use the exact `{separator}` separator"));
    };
    let mut description = first.to_owned();
    for token in &tokens[1..] {
        match token {
            InlineToken::Code(value) | InlineToken::Text(value) => description.push_str(value),
        }
    }
    if description.trim().is_empty() {
        return Err("entry description must be non-empty".to_owned());
    }
    Ok(description)
}

fn parse_target(value: &str) -> Result<ContractTarget, String> {
    let parts = value.split('/').collect::<Vec<_>>();
    let [canonical_spec, section, entry_id] = parts.as_slice() else {
        return Err("Consumes target must contain exactly spec, section, and entry ID".to_owned());
    };
    validate_id(canonical_spec)?;
    validate_id(entry_id)?;
    let section = match *section {
        "owns" => ContractSection::Owns,
        "exports" => ContractSection::Exports,
        "invariants" => ContractSection::Invariants,
        "file-ownership" => ContractSection::FileOwnership,
        _ => {
            return Err(
                "Consumes target section must be owns, exports, invariants, or file-ownership"
                    .to_owned(),
            );
        }
    };
    Ok(ContractTarget {
        canonical_spec: (*canonical_spec).to_owned(),
        section,
        entry_id: (*entry_id).to_owned(),
    })
}

fn validate_id(value: &str) -> Result<(), String> {
    let mut bytes = value.bytes();
    if value.len() > 64
        || !bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        || bytes.any(|byte| !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
        || value.ends_with('-')
        || value.contains("--")
    {
        return Err(format!("invalid lowercase kebab-case Contract ID: {value}"));
    }
    Ok(())
}

fn validate_path(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.starts_with('/')
        || value.contains('\\')
        || value.contains('?')
        || value.contains('[')
        || value.contains(']')
    {
        return Err(format!("invalid project-root-relative POSIX path: {value}"));
    }
    let base = value.strip_suffix("/**").unwrap_or(value);
    if base.is_empty()
        || base.contains('*')
        || base
            .split('/')
            .any(|segment| segment.is_empty() || matches!(segment, "." | ".."))
    {
        return Err(format!(
            "invalid exact path or terminal /** subtree: {value}"
        ));
    }
    Ok(())
}

fn validate_unique_ids(document: &ContractDocument, issues: &mut Vec<ContractIssue>) {
    unique_described(ContractSection::Owns, &document.owns, issues);
    unique_described(ContractSection::Exports, &document.exports, issues);
    unique_described(ContractSection::Invariants, &document.invariants, issues);
    unique(
        ContractSection::Consumes,
        document
            .consumes
            .iter()
            .map(|entry| (&entry.id, entry.line)),
        issues,
    );
    unique(
        ContractSection::FileOwnership,
        document
            .file_ownership
            .iter()
            .map(|entry| (&entry.id, entry.line)),
        issues,
    );
}

fn unique_described(
    section: ContractSection,
    entries: &[DescribedEntry],
    issues: &mut Vec<ContractIssue>,
) {
    unique(
        section,
        entries.iter().map(|entry| (&entry.id, entry.line)),
        issues,
    );
}

fn unique<'a>(
    section: ContractSection,
    entries: impl Iterator<Item = (&'a String, usize)>,
    issues: &mut Vec<ContractIssue>,
) {
    let mut seen = BTreeSet::new();
    for (id, line) in entries {
        if !seen.insert(id) {
            issues.push(issue(
                "CONTRACT_ENTRY_ID_DUPLICATE",
                line,
                format!("{} entry ID {id} is duplicated", section.token()),
            ));
        }
    }
}

const fn section_for_index(index: usize) -> ContractSection {
    match index {
        0 => ContractSection::Owns,
        1 => ContractSection::Exports,
        2 => ContractSection::Consumes,
        3 => ContractSection::Invariants,
        4 => ContractSection::FileOwnership,
        _ => unreachable!(),
    }
}

fn line_at(body: &str, offset: usize) -> usize {
    body[..offset].bytes().filter(|byte| *byte == b'\n').count() + 1
}

fn line_at_end(body: &str) -> usize {
    body.bytes().filter(|byte| *byte == b'\n').count() + 1
}

fn issue(code: &'static str, line: usize, message: impl Into<String>) -> ContractIssue {
    ContractIssue {
        code,
        line,
        message: message.into(),
    }
}
