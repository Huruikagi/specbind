//! Stable artifact read-model facade.
//!
//! Discovery owns filesystem enumeration and metadata profiles. Resolution owns
//! typed projections used by lifecycle services. Callers depend on this facade
//! rather than either implementation module.

mod discovery;
mod resolution;

use std::collections::BTreeSet;

use camino::Utf8PathBuf;

use crate::{
    domain::{spec::Spec, tasks::Tasks},
    freshness::CurrentGateInputs,
    schema::spec::v1::SpecDocument,
    traceability::TraceabilityReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactKind {
    Brief,
    Research,
    Requirements,
    Design,
    Contract,
    ImplementationNotes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub selector: String,
    pub artifact_type: String,
    pub path: Utf8PathBuf,
    pub artifact_id: Option<String>,
    pub kind: ArtifactKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiscoveryIssue {
    pub code: &'static str,
    pub path: Option<Utf8PathBuf>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactInventory {
    pub artifacts: Vec<Artifact>,
    pub issues: Vec<DiscoveryIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateInputResolution {
    pub inventory: ArtifactInventory,
    pub inputs: CurrentGateInputs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityResolution {
    pub inventory: ArtifactInventory,
    pub report: Option<TraceabilityReport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TasksResolution {
    pub tasks: Option<Tasks>,
    pub issues: Vec<DiscoveryIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecResolution {
    pub wire: Option<SpecDocument>,
    pub spec: Option<Spec>,
    pub issues: Vec<DiscoveryIssue>,
}

/// Why one entry below `specs/` is not a persistent Spec.
///
/// The reason is reported structurally so each caller can name it in its own
/// diagnostic vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecEntryFault {
    /// The directory entry itself could not be inspected.
    Unreadable(String),
    /// The entry name is not UTF-8, so it cannot be a canonical identity.
    NonUtf8Name,
    /// The entry is a symlink or not a directory.
    NotADirectory,
    /// The name is UTF-8 but not a canonical lowercase kebab-case identity.
    InvalidId,
}

/// One enumeration of `specs/`, separating identities from rejected entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecDiscovery {
    pub specs: BTreeSet<String>,
    /// Rejected entries, each with its project-relative path when one is known.
    pub faults: Vec<(Option<Utf8PathBuf>, SpecEntryFault)>,
}

pub use discovery::{canonical_id, discover_spec, discover_spec_ids};
pub use resolution::{resolve_gate_inputs, resolve_spec, resolve_tasks, resolve_traceability};

pub(crate) use discovery::{
    collection_id, is_kebab_id, recognized_kind, selector, split_frontmatter,
};
pub(crate) use resolution::resolve_contract_projection;
