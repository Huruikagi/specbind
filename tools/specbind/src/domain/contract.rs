use std::collections::BTreeSet;

use crate::schema::contract::v1::{self as wire, TargetSection};

use super::diagnostics::{SemanticIssues, issue};

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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileOwnershipEntry {
    pub id: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    wire: wire::ContractDocument,
    pub owns: Vec<DescribedEntry>,
    pub exports: Vec<DescribedEntry>,
    pub consumes: Vec<ConsumesEntry>,
    pub invariants: Vec<DescribedEntry>,
    pub file_ownership: Vec<FileOwnershipEntry>,
}

pub type ContractDocument = Contract;

impl Contract {
    #[must_use]
    pub fn as_wire(&self) -> &wire::ContractDocument {
        &self.wire
    }
}

impl TryFrom<wire::ContractDocument> for Contract {
    type Error = SemanticIssues;

    fn try_from(wire: wire::ContractDocument) -> Result<Self, Self::Error> {
        let issues = validate(&wire);
        if !issues.is_empty() {
            return Err(SemanticIssues::from_unsorted(issues));
        }
        Ok(Self {
            owns: described(&wire.owns),
            exports: described(&wire.exports),
            consumes: wire
                .consumes
                .iter()
                .map(|entry| ConsumesEntry {
                    id: entry.id.0.clone(),
                    target: ContractTarget {
                        canonical_spec: entry.target.spec.clone(),
                        section: section(entry.target.section),
                        entry_id: entry.target.id.0.clone(),
                    },
                    description: entry.description.clone(),
                })
                .collect(),
            invariants: described(&wire.invariants),
            file_ownership: wire
                .file_ownership
                .iter()
                .map(|entry| FileOwnershipEntry {
                    id: entry.id.0.clone(),
                    paths: entry.paths.clone(),
                })
                .collect(),
            wire,
        })
    }
}

fn described(entries: &[wire::DescribedEntry]) -> Vec<DescribedEntry> {
    entries
        .iter()
        .map(|entry| DescribedEntry {
            id: entry.id.0.clone(),
            description: entry.description.clone(),
        })
        .collect()
}

fn section(section: TargetSection) -> ContractSection {
    match section {
        TargetSection::Owns => ContractSection::Owns,
        TargetSection::Exports => ContractSection::Exports,
        TargetSection::Invariants => ContractSection::Invariants,
        TargetSection::FileOwnership => ContractSection::FileOwnership,
    }
}

fn validate(document: &wire::ContractDocument) -> Vec<super::SemanticIssue> {
    let mut issues = Vec::new();
    validate_descriptions("owns", &document.owns, &mut issues);
    validate_descriptions("exports", &document.exports, &mut issues);
    validate_descriptions("invariants", &document.invariants, &mut issues);
    validate_ids(
        "owns",
        document.owns.iter().map(|entry| &entry.id.0),
        &mut issues,
    );
    validate_ids(
        "exports",
        document.exports.iter().map(|entry| &entry.id.0),
        &mut issues,
    );
    validate_ids(
        "consumes",
        document.consumes.iter().map(|entry| &entry.id.0),
        &mut issues,
    );
    validate_ids(
        "invariants",
        document.invariants.iter().map(|entry| &entry.id.0),
        &mut issues,
    );
    validate_ids(
        "file_ownership",
        document.file_ownership.iter().map(|entry| &entry.id.0),
        &mut issues,
    );

    for (index, entry) in document.consumes.iter().enumerate() {
        if entry
            .description
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            issues.push(issue(
                "CONTRACT_DESCRIPTION_EMPTY",
                format!("/consumes/{index}/description"),
                "Consumes description must be omitted or contain non-whitespace text",
            ));
        }
    }
    validate_paths(document, &mut issues);
    issues
}

fn validate_descriptions(
    section: &str,
    entries: &[wire::DescribedEntry],
    issues: &mut Vec<super::SemanticIssue>,
) {
    for (index, entry) in entries.iter().enumerate() {
        if entry.description.trim().is_empty() {
            issues.push(issue(
                "CONTRACT_DESCRIPTION_EMPTY",
                format!("/{section}/{index}/description"),
                "Contract description must contain non-whitespace text",
            ));
        }
    }
}

fn validate_ids<'a>(
    section: &str,
    ids: impl Iterator<Item = &'a String>,
    issues: &mut Vec<super::SemanticIssue>,
) {
    let mut seen = BTreeSet::new();
    for (index, id) in ids.enumerate() {
        if !seen.insert(id) {
            issues.push(issue(
                "CONTRACT_ENTRY_ID_DUPLICATE",
                format!("/{section}/{index}/id"),
                format!("entry ID {id} is duplicated within {section}"),
            ));
        }
    }
}

fn validate_paths(document: &wire::ContractDocument, issues: &mut Vec<super::SemanticIssue>) {
    let mut seen = BTreeSet::new();
    for (entry_index, entry) in document.file_ownership.iter().enumerate() {
        for (path_index, path) in entry.paths.iter().enumerate() {
            let pointer = format!("/file_ownership/{entry_index}/paths/{path_index}");
            if !valid_path(path) {
                issues.push(issue(
                    "CONTRACT_FILE_OWNERSHIP_PATH_INVALID",
                    pointer,
                    format!("File Ownership path is invalid: {path}"),
                ));
            } else if !seen.insert(path.to_ascii_lowercase()) {
                issues.push(issue(
                    "CONTRACT_FILE_OWNERSHIP_PATH_DUPLICATE",
                    pointer,
                    format!("File Ownership path is duplicated case-insensitively: {path}"),
                ));
            }
        }
    }
}

fn valid_path(path: &str) -> bool {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains('\\')
        || path.contains(':')
        || path.ends_with('/')
    {
        return false;
    }
    let subtree = path.ends_with("/**");
    let base = path.strip_suffix("/**").unwrap_or(path);
    if base.is_empty() || base.contains('*') || base.contains('?') {
        return false;
    }
    base.split('/')
        .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
        && (!subtree || !base.ends_with('/'))
}
