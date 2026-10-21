//! Shared Contract baseline comparison and scoped change guards.
use super::{ReviewIssues, one_review_issue};
use crate::{
    domain::shared_contract::SharedContract, fingerprint::Fingerprint, repository,
    roadmap::RoadmapDocument,
};
use std::{collections::BTreeSet, path::Path};

pub(super) struct SharedReview {
    pub required: bool,
    pub changed: BTreeSet<String>,
    pub known_resources: BTreeSet<String>,
}

pub(super) fn assess(
    project: &Path,
    root: &Path,
    roadmap: &RoadmapDocument,
) -> Result<SharedReview, ReviewIssues> {
    let current = crate::read_model::shared_contract::read(root).map_err(failure)?;
    let root_relative = root
        .strip_prefix(project)
        .map_err(|error| failure(error.to_string()))?;
    let tree =
        repository::tree::Tree::resolve(project, &roadmap.baseline_revision).map_err(failure)?;
    let baseline = crate::read_model::shared_contract::at_baseline(
        &tree,
        &root_relative.to_string_lossy().replace('\\', "/"),
    )
    .map_err(failure)?
    .map(|(_, document)| document);
    let before = Fingerprint::shared_contract(baseline.as_ref())
        .map_err(|error| failure(error.to_string()))?;
    let after = Fingerprint::shared_contract(current.as_ref())
        .map_err(|error| failure(error.to_string()))?;
    let ids = baseline
        .iter()
        .chain(current.iter())
        .flat_map(|value| {
            value
                .as_wire()
                .resources
                .iter()
                .map(|resource| resource.id.0.clone())
        })
        .collect::<BTreeSet<_>>();
    let mut changed = BTreeSet::new();
    for id in &ids {
        if resource_fingerprint(baseline.as_ref(), id)?
            != resource_fingerprint(current.as_ref(), id)?
        {
            changed.insert(id.clone());
        }
    }
    // Creating/removing an empty manifest is still a managed boundary change.
    if before != after && changed.is_empty() {
        changed.insert("*".into());
    }
    Ok(SharedReview {
        required: !roadmap.spec_ids().is_empty() || roadmap.has_shared_changes() || before != after,
        changed,
        known_resources: ids,
    })
}

fn resource_fingerprint(
    document: Option<&SharedContract>,
    id: &str,
) -> Result<Option<Fingerprint>, ReviewIssues> {
    document
        .and_then(|value| {
            value
                .as_wire()
                .resources
                .iter()
                .find(|resource| resource.id.0 == id)
        })
        .map(|resource| {
            let wire = crate::schema::shared_contract::v1::SharedContractDocument {
                schema_version: 1,
                resources: vec![resource.clone()],
            };
            let validated =
                SharedContract::try_from(wire).map_err(|error| failure(error.to_string()))?;
            Fingerprint::shared_contract(Some(&validated))
                .map_err(|error| failure(error.to_string()))
        })
        .transpose()
}

pub(super) fn validate_scope(
    project: &Path,
    root: &Path,
    roadmap: &RoadmapDocument,
) -> Result<(), ReviewIssues> {
    let assessment = assess(project, root, roadmap)?;
    if !roadmap.reverse_specs.is_empty() {
        return Ok(());
    }
    let declared = roadmap
        .direct_changes
        .iter()
        .flat_map(|item| item.shared_contract_changes.iter().cloned())
        .collect::<BTreeSet<_>>();
    for id in &declared {
        if (id == "*" && !assessment.changed.contains("*"))
            || (id != "*" && !assessment.known_resources.contains(id))
        {
            return Err(one_review_issue(
                "SHARED_CONTRACT_PLANNED_RESOURCE_MISSING",
                Some(crate::read_model::shared_contract::SHARED_CONTRACT_RELATIVE.into()),
                format!("planned resource {id} has no baseline or proposed agreement"),
            ));
        }
    }
    let unscoped = assessment
        .changed
        .difference(&declared)
        .cloned()
        .collect::<Vec<_>>();
    if unscoped.is_empty() {
        Ok(())
    } else {
        Err(one_review_issue(
            "SHARED_CONTRACT_CHANGE_UNSCOPED",
            Some(crate::read_model::shared_contract::SHARED_CONTRACT_RELATIVE.into()),
            format!(
                "assign shared changes to a Direct item: {}",
                unscoped.join(", ")
            ),
        ))
    }
}

fn failure(message: impl Into<String>) -> ReviewIssues {
    one_review_issue(
        "SHARED_CONTRACT_REVIEW_INPUT_INVALID",
        Some(crate::read_model::shared_contract::SHARED_CONTRACT_RELATIVE.into()),
        message,
    )
}
