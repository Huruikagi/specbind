//! Read model for active milestone scope replacement candidates.
//!
//! Decision 0097 accepts this read so an agent can compose a complete
//! `milestone update-scope` candidate from the current value. The document is
//! written in exactly the Decision 0089 version-1 transport shape.

use std::path::Path;

use crate::{
    milestone,
    milestone_status::{self, MilestoneStatusFailure},
    roadmap::{Dependency, DirectItem, RoadmapDocument, SpecItem},
};

/// Renders the active milestone's scope as a version-1 candidate document.
///
/// With `include_body`, the complete current Markdown body is emitted after the
/// work items, so a caller that deliberately intends to change the prose edits a
/// complete value rather than composing one. There is no body-only read whose
/// result could be mistaken for a complete replacement candidate.
///
/// Returns `Ok(None)` when no milestone is active.
///
/// # Errors
///
/// Returns read or parser diagnostics when an active Roadmap exists but cannot
/// be read as a valid document. No partial scope is produced, because a
/// replacement composed from a scope the parser never accepted would be
/// rejected or, worse, silently lose work items.
pub fn resolve(
    specbind_root: &Path,
    include_body: bool,
) -> Result<Option<String>, MilestoneStatusFailure> {
    Ok(
        milestone_status::read_roadmap_source(specbind_root)?.map(|(source, document)| {
            let body = include_body.then(|| milestone::existing_body(&source));
            render(&document, body)
        }),
    )
}

/// Serializes the candidate by hand.
///
/// Field order is part of the accepted contract, and `serde_json` orders object
/// keys alphabetically without the `preserve_order` feature, which would emit
/// `directChanges` before `newSpecs` and invert the declared order. Writing the
/// document directly also fixes the indentation and trailing newline.
fn render(roadmap: &RoadmapDocument, body: Option<&str>) -> String {
    let mut categories = Vec::new();
    if !roadmap.new_specs.is_empty() {
        categories.push(spec_category("newSpecs", &roadmap.new_specs));
    }
    if !roadmap.spec_updates.is_empty() {
        categories.push(spec_category("specUpdates", &roadmap.spec_updates));
    }
    if !roadmap.reverse_specs.is_empty() {
        categories.push(spec_category("reverseSpecs", &roadmap.reverse_specs));
    }
    if !roadmap.direct_changes.is_empty() {
        categories.push(direct_category(&roadmap.direct_changes));
    }
    let body = body.map_or_else(String::new, |body| {
        format!(",\n  \"body\": {}", quote(body))
    });
    let baseline_version = roadmap
        .baseline_version
        .as_ref()
        .map_or_else(String::new, |value| {
            format!(",\n  \"baselineVersion\": {}", quote(value))
        });
    format!(
        "{{\n  \"schemaVersion\": 1,\n  \"workItems\": {{\n{}\n  }}{baseline_version}{body}\n}}\n",
        categories.join(",\n")
    )
}

fn spec_category(name: &str, items: &[SpecItem]) -> String {
    let rendered = items
        .iter()
        .map(|item| item_object(&[("spec", &item.spec)], &item.summary, &item.depends_on))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("    \"{name}\": [\n{rendered}\n    ]")
}

/// Per-item `status` is never emitted. Decision 0089 forbids it in a candidate
/// and preserves completed Direct status by identity across an update, so
/// echoing it would produce a document `update-scope` rejects.
fn direct_category(items: &[DirectItem]) -> String {
    let rendered = items
        .iter()
        .map(|item| item_object(&[("id", &item.id)], &item.summary, &item.depends_on))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("    \"directChanges\": [\n{rendered}\n    ]")
}

fn item_object(identity: &[(&str, &String)], summary: &str, depends_on: &[Dependency]) -> String {
    let mut fields = identity
        .iter()
        .map(|(key, value)| format!("        \"{key}\": {}", quote(value)))
        .collect::<Vec<_>>();
    fields.push(format!("        \"summary\": {}", quote(summary)));
    if !depends_on.is_empty() {
        let rendered = depends_on
            .iter()
            .map(|dependency| {
                let (key, value) = match dependency {
                    Dependency::Spec(target) => ("spec", &target.spec),
                    Dependency::Direct(target) => ("direct", &target.direct),
                };
                format!("          {{ \"{key}\": {} }}", quote(value))
            })
            .collect::<Vec<_>>()
            .join(",\n");
        fields.push(format!("        \"dependsOn\": [\n{rendered}\n        ]"));
    }
    format!("      {{\n{}\n      }}", fields.join(",\n"))
}

/// Escapes one string exactly as JSON requires.
fn quote(value: &str) -> String {
    serde_json::Value::String(value.to_owned()).to_string()
}
