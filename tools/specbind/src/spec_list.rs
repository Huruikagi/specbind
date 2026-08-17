//! Project-wide listing of persistent Specs.
//!
//! This is the read model Decision 0097 requires for routing: which Specs exist,
//! what state each declares, and whether each one is usable or needs repair. It
//! deliberately reports a Spec whose machine state cannot be read rather than
//! failing, because this is the command an agent uses to discover that fault.

use std::path::Path;

use crate::{
    artifacts::{self, ArtifactKind},
    schema::spec::v1::WorkflowState,
};

/// Whether one listed Spec is usable as it stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecHealth {
    /// A structurally valid `spec.yaml` was read.
    Readable,
    /// No valid `spec.yaml` could be read; the message names the fault.
    Unreadable(String),
}

/// One persistent Spec as the project-wide listing sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecListEntry {
    pub canonical_spec: String,
    pub health: SpecHealth,
    /// The declared active workflow state, absent when the Spec is idle.
    pub declared_state: Option<WorkflowState>,
    /// The owning milestone of the active change, absent when the Spec is idle.
    pub milestone_id: Option<String>,
    /// Whether a canonical Contract artifact is present.
    pub has_contract: bool,
    /// Whether a canonical Requirements artifact is present.
    pub has_requirements: bool,
}

impl SpecListEntry {
    /// Reports whether this Spec currently holds an active change.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.milestone_id.is_some()
    }
}

/// Why the listing itself could not be produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecListFailure {
    pub message: String,
}

/// Lists every persistent Spec below a `SpecBind` root, ordered by identity.
///
/// # Errors
///
/// Returns a failure only when `specs/` cannot be enumerated at all. An empty
/// project lists no Specs and is not a failure.
pub fn resolve(specbind_root: &Path) -> Result<Vec<SpecListEntry>, SpecListFailure> {
    let discovery = artifacts::discover_spec_ids(specbind_root)
        .map_err(|message| SpecListFailure { message })?;
    // `discover_spec_ids` collects into a `BTreeSet`, so identities already
    // arrive in Unicode code point order and the listing is stable everywhere.
    Ok(discovery
        .specs
        .into_iter()
        .map(|canonical_spec| entry(specbind_root, canonical_spec))
        .collect())
}

fn entry(specbind_root: &Path, canonical_spec: String) -> SpecListEntry {
    let resolution = artifacts::resolve_spec(specbind_root, &canonical_spec);
    let (health, declared_state, milestone_id) = match resolution.wire.as_ref() {
        Some(wire) => {
            let active = wire.active_change.0.as_ref();
            (
                SpecHealth::Readable,
                active.map(|active| active.state),
                active.map(|active| active.milestone_id.0.clone()),
            )
        }
        None => (
            SpecHealth::Unreadable(unreadable_reason(&resolution.issues)),
            None,
            None,
        ),
    };
    let inventory = artifacts::discover_spec(specbind_root, &canonical_spec);
    let has = |kind: ArtifactKind| {
        inventory
            .artifacts
            .iter()
            .any(|artifact| artifact.kind == kind)
    };
    SpecListEntry {
        canonical_spec,
        health,
        declared_state,
        milestone_id,
        has_contract: has(ArtifactKind::Contract),
        has_requirements: has(ArtifactKind::Requirements),
    }
}

/// Names the fault behind an unreadable Spec, preferring the first reported
/// diagnostic so the message points at the actual cause.
fn unreadable_reason(issues: &[artifacts::DiscoveryIssue]) -> String {
    issues.first().map_or_else(
        || "spec.yaml is missing or unreadable".to_owned(),
        |issue| format!("{}: {}", issue.code, issue.message),
    )
}
