//! Catalog discovery and raw reads for OKF artifact templates.
//!
//! A project-owned copy below `settings/templates/` overrides the official
//! default embedded in this binary, one selector at a time.

use std::{fs, path::Path};

use camino::Utf8PathBuf;
use include_dir::{Dir, include_dir};
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::artifacts::{
    ArtifactKind, DiscoveryIssue, collection_id, is_kebab_id, recognized_kind, selector,
    split_frontmatter,
};
use crate::config::ProjectLanguage;
use crate::instruction;

/// The project tree that scaffolds one Spec's artifacts.
pub const SPEC_TEMPLATE_ROOT: &str = "settings/templates/specs";

static EMBEDDED_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/templates");

/// Where one resolved template came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateSource {
    /// A user-owned copy below the project `SpecBind` root.
    Project,
    /// The official default embedded in this binary.
    Embedded,
}

impl TemplateSource {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Embedded => "embedded",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub source: TemplateSource,
    pub selector: String,
    pub artifact_type: String,
    pub artifact_id: Option<String>,
    /// Location below the `SpecBind` root, or below the embedded asset tree.
    pub template_path: Utf8PathBuf,
    /// Materialization target relative to the destination Spec directory.
    pub output_path: Utf8PathBuf,
    pub kind: ArtifactKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateInventory {
    pub templates: Vec<Template>,
    pub issues: Vec<DiscoveryIssue>,
}

mod common;
mod milestone;
mod spec;
mod steering;

use common::{issue, relative, validate_template_root};
pub use milestone::{
    discover_milestone_templates, read_embedded_milestone, read_milestone_template,
};
pub use spec::{
    INSTALLED_SELECTORS, discover_spec_templates, embedded_spec_templates,
    installed_default_templates, read_embedded, read_spec_template,
};
pub use steering::{
    discover_steering_templates, embedded_steering_templates, read_embedded_steering,
    read_steering_template,
};

/// The project tree that scaffolds steering documents.
pub const STEERING_TEMPLATE_ROOT: &str = "settings/templates/steering";

/// One resolved steering document template.
///
/// Decision 0117 gives this scope the identity exception no other scope needs.
/// A template that declares `artifact_id` materializes at `steering/<id>.md`;
/// one that omits it is a scaffold whose identity the authoring skill supplies,
/// so it has no fixed output path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringTemplate {
    pub source: TemplateSource,
    /// The template file stem, which is this scope's selector.
    pub selector: String,
    pub artifact_type: String,
    pub artifact_id: Option<String>,
    /// Location below the `SpecBind` root, or below the embedded asset tree.
    pub template_path: Utf8PathBuf,
    /// Materialization target below the `SpecBind` root, when identity is fixed.
    pub output_path: Option<Utf8PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringTemplateInventory {
    pub templates: Vec<SteeringTemplate>,
    pub issues: Vec<DiscoveryIssue>,
}

/// The project-owned singleton that scaffolds the active Roadmap body.
pub const MILESTONE_ROADMAP_TEMPLATE_PATH: &str = "settings/templates/roadmap.md";

const MILESTONE_ROADMAP_SELECTOR: &str = "roadmap";

/// One resolved milestone-wide template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneTemplate {
    pub source: TemplateSource,
    pub selector: String,
    pub artifact_type: String,
    pub template_path: Utf8PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneTemplateInventory {
    pub templates: Vec<MilestoneTemplate>,
    pub issues: Vec<DiscoveryIssue>,
}
