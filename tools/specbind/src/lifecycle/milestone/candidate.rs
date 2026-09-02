//! Milestone scope candidate parsing.

use crate::{
    roadmap::{DirectItem, SpecItem},
    schema::scope::v1::{ScopeDocument, SpecItemDocument},
};

use super::{MilestoneIssues, one_issue};

/// One transient milestone scope document. Identity, baseline, release binding,
/// and Direct completion status are never accepted from the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ValidatedScope {
    pub(super) new_specs: Vec<SpecItem>,
    pub(super) spec_updates: Vec<SpecItem>,
    pub(super) reverse_specs: Vec<SpecItem>,
    pub(super) direct_changes: Vec<DirectItem>,
    pub(super) baseline_version: Option<String>,
    pub(super) body: Option<String>,
}

/// Decodes one strict version-1 scope document.
///
/// Every accepted work-item rule is enforced later by the authoritative Roadmap
/// parser, so this boundary owns only transport shape and version.
pub(super) fn parse(json: &str, code: &'static str) -> Result<ValidatedScope, MilestoneIssues> {
    let document = serde_json::from_str::<ScopeDocument>(json)
        .map_err(|error| one_issue(code, None, format!("scope document is invalid: {error}")))?;
    if document.schema_version != 1 {
        return Err(one_issue(
            code,
            Some("/schemaVersion".to_owned()),
            "scope document schemaVersion must be 1",
        ));
    }
    Ok(ValidatedScope {
        new_specs: spec_items(document.work_items.new_specs),
        spec_updates: spec_items(document.work_items.spec_updates),
        reverse_specs: spec_items(document.work_items.reverse_specs),
        direct_changes: document
            .work_items
            .direct_changes
            .unwrap_or_default()
            .into_iter()
            .map(|item| DirectItem {
                id: item.id,
                summary: item.summary,
                depends_on: item.depends_on,
                status: None,
            })
            .collect(),
        baseline_version: document.baseline_version,
        body: document.body,
    })
}

fn spec_items(items: Option<Vec<SpecItemDocument>>) -> Vec<SpecItem> {
    items
        .unwrap_or_default()
        .into_iter()
        .map(|item| SpecItem {
            spec: item.spec,
            summary: item.summary,
            depends_on: item.depends_on,
        })
        .collect()
}
