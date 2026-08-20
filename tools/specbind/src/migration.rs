//! cc-sdd migration inventory, planning, and deterministic application.

mod apply;
mod inventory;
pub mod resolution;

use std::{fmt, fs, path::Path};

use crate::{
    config::ProjectLanguage,
    install::{self, InstallInputs, PlanAction},
};

pub const GUIDE_NEUTRAL: &str = "https://huruikagi.github.io/specbind/guide/migration/cc-sdd/";
pub const GUIDE_EN: &str = "https://huruikagi.github.io/specbind/guide/en/migrate-from-cc-sdd/";
pub const GUIDE_JA: &str = "https://huruikagi.github.io/specbind/guide/ja/migrate-from-cc-sdd/";

pub(super) const LEGACY_CONFIG: &str = ".cc-sdd.json";
pub(super) const TARGET_CONFIG: &str = ".specbind.json";
pub(super) const DEFAULT_LEGACY_ROOT: &str = ".kiro";
pub(super) const TARGET_ROOT: &str = ".specbind";
pub(super) const LEGACY_SKILLS: &[&str] = &[
    "kiro-debug",
    "kiro-discovery",
    "kiro-impl",
    "kiro-review",
    "kiro-spec-batch",
    "kiro-spec-design",
    "kiro-spec-init",
    "kiro-spec-quick",
    "kiro-spec-requirements",
    "kiro-spec-status",
    "kiro-spec-tasks",
    "kiro-steering",
    "kiro-steering-custom",
    "kiro-validate-design",
    "kiro-validate-gap",
    "kiro-validate-impl",
    "kiro-verify-completion",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationPlan {
    pub legacy_root: String,
    pub target_root: String,
    pub language: Option<ProjectLanguage>,
    pub agents: Vec<String>,
    pub specs: Vec<LegacySpec>,
    pub actions: Vec<MigrationAction>,
    pub findings: Vec<MigrationFinding>,
    pub target_converged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationOutcome {
    pub installed_files: usize,
    pub removed_legacy_assets: usize,
    pub removed_legacy_root: String,
    pub removed_legacy_config: bool,
    pub removed_resolution_state: bool,
}

/// Returns the unresolved source plan before any accepted semantic resolution
/// record is considered. Resolution acceptance uses this to avoid allowing an
/// older record to authorize its own replacement.
pub(crate) fn unresolved_plan(project_root: &Path) -> Result<MigrationPlan, MigrationIssues> {
    inventory::plan_inner(project_root)
}

/// Reports a completed cutover when no default cc-sdd source remains and the
/// installed `SpecBind` target is current.
#[must_use]
pub fn source_absent_and_target_current(project_root: &Path) -> bool {
    if path_entry_present(&project_root.join(LEGACY_CONFIG))
        || path_entry_present(&project_root.join(DEFAULT_LEGACY_ROOT))
    {
        return false;
    }
    install::plan(project_root, &InstallInputs::default()).is_ok_and(|plan| {
        !plan.initial
            && plan
                .entries
                .iter()
                .all(|entry| entry.action == PlanAction::Keep)
    })
}

fn path_entry_present(path: &Path) -> bool {
    match fs::symlink_metadata(path) {
        Ok(_) => true,
        Err(error) => error.kind() != std::io::ErrorKind::NotFound,
    }
}

impl MigrationPlan {
    #[must_use]
    pub fn guide_url(&self) -> &'static str {
        match self.language {
            Some(ProjectLanguage::En) => GUIDE_EN,
            Some(ProjectLanguage::Ja) => GUIDE_JA,
            None => GUIDE_NEUTRAL,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacySpec {
    pub id: String,
    pub language: Option<ProjectLanguage>,
    pub phase: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationAction {
    pub kind: &'static str,
    pub source: Option<String>,
    pub target: Option<String>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationFinding {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationIssues {
    pub issues: Vec<MigrationFinding>,
}

impl fmt::Display for MigrationIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cc-sdd migration planning failed")
    }
}

impl std::error::Error for MigrationIssues {}

/// Inventories one cc-sdd project without modifying it.
///
/// # Errors
///
/// Returns stable diagnostics for unreadable or structurally unsafe source
/// paths. Semantic ambiguity is returned in `MigrationPlan::findings` so the
/// CLI can route it to the agent-assisted guide.
pub fn plan(project_root: &Path) -> Result<MigrationPlan, MigrationIssues> {
    let mut plan = inventory::plan_inner(project_root)?;
    resolution::reconcile(project_root, &mut plan);
    Ok(plan)
}

/// Recomputes and applies only a finding-free deterministic migration plan.
///
/// # Errors
///
/// Returns stable diagnostics when the plan changed, Git is not a safe
/// recovery boundary, installation fails, or a known legacy asset cannot be
/// removed safely.
pub fn apply(project_root: &Path) -> Result<MigrationOutcome, MigrationIssues> {
    apply::apply(project_root)
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> MigrationIssues {
    MigrationIssues {
        issues: vec![finding(code, path, message)],
    }
}

fn finding(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> MigrationFinding {
    MigrationFinding {
        code,
        path,
        message: message.into(),
    }
}
