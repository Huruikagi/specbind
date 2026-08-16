//! Cross-artifact Requirement traceability over one spec's current artifacts.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignRequirementSet {
    pub selector: String,
    pub requirement_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TraceabilityIssue {
    pub code: &'static str,
    pub selector: Option<String>,
    pub requirement_id: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityReport {
    pub requirement_ids: Vec<String>,
    pub active_requirement_ids: Option<Vec<String>>,
    pub design_requirement_ids: Vec<String>,
    pub designs: BTreeMap<String, Vec<String>>,
    pub issues: Vec<TraceabilityIssue>,
}

/// Compares the complete Requirements catalog, per-Design mappings, and active scope.
#[must_use]
pub fn evaluate(
    requirement_ids: &[String],
    designs: Vec<DesignRequirementSet>,
    active_requirement_ids: Option<Vec<String>>,
) -> TraceabilityReport {
    let requirements = requirement_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut design_map = BTreeMap::new();
    let mut design_union = BTreeSet::new();
    let mut issues = Vec::new();

    for design in designs {
        let ids = design.requirement_ids.into_iter().collect::<BTreeSet<_>>();
        for id in ids.difference(&requirements) {
            issues.push(TraceabilityIssue {
                code: "TRACEABILITY_DESIGN_REQUIREMENT_UNKNOWN",
                selector: Some(design.selector.clone()),
                requirement_id: id.clone(),
                message: format!(
                    "{} references Requirement ID {id}, which does not exist in Requirements",
                    design.selector
                ),
            });
        }
        design_union.extend(ids.iter().cloned());
        design_map.insert(design.selector, numeric_ids(ids));
    }

    if let Some(active) = &active_requirement_ids {
        for id in active {
            if !requirements.contains(id) {
                issues.push(TraceabilityIssue {
                    code: "TRACEABILITY_ACTIVE_REQUIREMENT_UNKNOWN",
                    selector: None,
                    requirement_id: id.clone(),
                    message: format!("active Requirement ID {id} does not exist in Requirements"),
                });
            } else if !design_union.contains(id) {
                issues.push(TraceabilityIssue {
                    code: "TRACEABILITY_DESIGN_COVERAGE_MISSING",
                    selector: None,
                    requirement_id: id.clone(),
                    message: format!(
                        "active Requirement ID {id} is not covered by any Design artifact"
                    ),
                });
            }
        }
    }

    issues.sort();
    issues.dedup();
    TraceabilityReport {
        requirement_ids: numeric_ids(requirements),
        active_requirement_ids,
        design_requirement_ids: numeric_ids(design_union),
        designs: design_map,
        issues,
    }
}

fn numeric_ids(ids: BTreeSet<String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort_by_key(|id| parse_id(id));
    ids
}

fn parse_id(value: &str) -> Option<(u64, u64)> {
    let (group, criterion) = value.split_once('.')?;
    Some((group.parse().ok()?, criterion.parse().ok()?))
}
