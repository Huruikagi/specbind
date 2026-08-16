//! Semantically validated domain models for structured `SpecBind` artifacts.

mod diagnostics;
pub mod spec;
pub mod tasks;

pub use diagnostics::{SemanticIssue, SemanticIssues};

pub(crate) fn parse_requirement_id(value: &str) -> Option<(u64, u64)> {
    let (group, criterion) = value.split_once('.')?;
    if group.starts_with('0') || criterion.starts_with('0') || criterion.contains('.') {
        return None;
    }
    let group = group.parse().ok()?;
    let criterion = criterion.parse().ok()?;
    (group > 0 && criterion > 0).then_some((group, criterion))
}
