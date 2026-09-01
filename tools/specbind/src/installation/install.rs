//! Installation planning and guarded application.
//!
//! Planning resolves the effective project configuration and reports exactly
//! which files an install would create, replace, or leave untouched. It never
//! writes, and it refuses to plan a replacement the Decision 0077 repository
//! guards would not permit.

mod apply;
mod assets;
mod guard;
mod input;

use std::{fmt, path::Path};

use serde::Deserialize;

use crate::{agent_role::AgentRoleOverrides, config::ProjectLanguage};

pub use apply::apply;
pub use input::read_installed_config;

use assets::{
    adapter_entries, agent_role_entries, config_entry, project_instruction_entries,
    retired_skill_entries, rule_entries, skill_entries, template_entries,
};
use guard::require_replaceable_repository;
use input::{read_existing_config, resolve_inputs};

pub(super) const CONFIG_RELATIVE: &str = ".specbind.json";
pub(super) const DEFAULT_SPEC_DIR: &str = ".specbind";

/// One supported coding agent. Selection is additive under Decision 0077.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Agent {
    ClaudeCode,
    Codex,
    Generic,
}

impl Agent {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
            Self::Generic => "generic",
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "claude-code" => Some(Self::ClaudeCode),
            "codex" => Some(Self::Codex),
            "generic" => Some(Self::Generic),
            _ => None,
        }
    }
}

/// Caller-supplied installation inputs. Absent values fall back to the existing
/// configuration, then to the accepted defaults.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InstallInputs {
    pub agents: Vec<Agent>,
    pub language: Option<ProjectLanguage>,
    pub spec_dir: Option<String>,
    pub project_instructions: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanAction {
    /// The target is absent and would be created.
    Create,
    /// The target exists and would be replaced with the current product asset.
    Replace,
    /// The target exists and is left untouched because the project owns it.
    Keep,
    /// A retired exact product-managed file exists and would be removed.
    Remove,
}

impl PlanAction {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Replace => "replace",
            Self::Keep => "keep",
            Self::Remove => "remove",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanEntry {
    pub action: PlanAction,
    /// Project-root-relative POSIX path.
    pub path: String,
    pub category: &'static str,
    pub detail: Option<String>,
    /// Exact bytes an apply would write. Absent for a kept target.
    content: Option<String>,
    /// Exact prior content the plan was computed from.
    ///
    /// Presence alone cannot detect a race for a target the installer edits in
    /// place rather than creating whole, so those categories carry what they
    /// read and the apply compares against it.
    expected_current: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallOutcome {
    pub plan: InstallPlan,
    /// True when the plan contained only kept targets.
    pub unchanged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    /// True when no readable `.specbind.json` exists yet.
    pub initial: bool,
    pub spec_dir: String,
    pub language: ProjectLanguage,
    pub agents: Vec<Agent>,
    pub project_instructions: bool,
    pub entries: Vec<PlanEntry>,
}

impl InstallPlan {
    #[must_use]
    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let count = |action: PlanAction| {
            self.entries
                .iter()
                .filter(|entry| entry.action == action)
                .count()
        };
        (
            count(PlanAction::Create),
            count(PlanAction::Replace),
            count(PlanAction::Keep),
            count(PlanAction::Remove),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstallIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallIssues {
    pub issues: Vec<InstallIssue>,
}

impl fmt::Display for InstallIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "installation planning has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for InstallIssues {}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledConfig {
    pub schema_version: u64,
    pub spec_dir: String,
    pub language: ProjectLanguage,
    pub agents: Vec<String>,
    #[serde(default)]
    pub project_instructions: bool,
    #[serde(default)]
    pub agent_roles: AgentRoleOverrides,
}

/// Computes the installation plan without touching the filesystem.
///
/// # Errors
///
/// Returns configuration, input, repository-guard, or asset diagnostics when no
/// trustworthy plan can be produced.
pub fn plan(project_root: &Path, inputs: &InstallInputs) -> Result<InstallPlan, InstallIssues> {
    let existing = read_existing_config(project_root)?;
    let resolved = resolve_inputs(existing.as_ref(), inputs)?;
    let mut entries = vec![config_entry(existing.as_ref(), &resolved)];
    entries.extend(template_entries(project_root, &resolved)?);
    entries.extend(rule_entries(project_root, &resolved)?);
    entries.extend(adapter_entries(project_root, &resolved)?);
    entries.extend(skill_entries(project_root, &resolved)?);
    entries.extend(retired_skill_entries(project_root, &resolved)?);
    entries.extend(agent_role_entries(project_root, &resolved)?);
    entries.extend(project_instruction_entries(project_root, &resolved)?);
    if entries
        .iter()
        .any(|entry| matches!(entry.action, PlanAction::Replace | PlanAction::Remove))
    {
        require_replaceable_repository(project_root)?;
    }
    Ok(InstallPlan {
        initial: existing.is_none(),
        spec_dir: resolved.spec_dir,
        language: resolved.language,
        agents: resolved.agents,
        project_instructions: resolved.project_instructions,
        entries,
    })
}

pub(super) fn finish(mut issues: Vec<InstallIssue>) -> Result<(), InstallIssues> {
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(InstallIssues { issues })
    }
}

pub(super) fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> InstallIssues {
    InstallIssues {
        issues: vec![issue(code, path, message)],
    }
}

pub(super) fn issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> InstallIssue {
    InstallIssue {
        code,
        path,
        message: message.into(),
    }
}
