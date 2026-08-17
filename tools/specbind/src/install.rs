//! Read-only installation planning.
//!
//! Planning resolves the effective project configuration and reports exactly
//! which files an install would create, replace, or leave untouched. It never
//! writes, and it refuses to plan a replacement the Decision 0077 repository
//! guards would not permit.

use std::{fmt, fs, path::Path};

use serde::Deserialize;

use crate::{config::ProjectLanguage, repository, rule, template};

const CONFIG_RELATIVE: &str = ".specbind.json";
const DEFAULT_SPEC_DIR: &str = ".specbind";

/// One supported coding agent. Selection is additive under Decision 0077.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Agent {
    ClaudeCode,
    Codex,
}

impl Agent {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::ClaudeCode => "claude-code",
            Self::Codex => "codex",
        }
    }

    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "claude-code" => Some(Self::ClaudeCode),
            "codex" => Some(Self::Codex),
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
}

impl PlanAction {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Replace => "replace",
            Self::Keep => "keep",
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
    pub fn counts(&self) -> (usize, usize, usize) {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstalledConfig {
    schema_version: u64,
    spec_dir: String,
    language: ProjectLanguage,
    agents: Vec<String>,
    #[serde(default)]
    project_instructions: bool,
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
    if entries
        .iter()
        .any(|entry| entry.action == PlanAction::Replace)
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

struct ResolvedInputs {
    spec_dir: String,
    language: ProjectLanguage,
    agents: Vec<Agent>,
    project_instructions: bool,
}

fn resolve_inputs(
    existing: Option<&InstalledConfig>,
    inputs: &InstallInputs,
) -> Result<ResolvedInputs, InstallIssues> {
    let mut issues = Vec::new();
    let spec_dir = inputs
        .spec_dir
        .clone()
        .or_else(|| existing.map(|config| config.spec_dir.clone()))
        .unwrap_or_else(|| DEFAULT_SPEC_DIR.to_owned());
    if let (Some(requested), Some(config)) = (inputs.spec_dir.as_deref(), existing)
        && requested != config.spec_dir
    {
        issues.push(issue(
            "INSTALL_SPEC_DIR_CHANGE_UNSUPPORTED",
            Some(CONFIG_RELATIVE.to_owned()),
            "changing specDir on an installed project is not supported in v1",
        ));
    }

    let language = inputs
        .language
        .or_else(|| existing.map(|config| config.language));
    if language.is_none() {
        issues.push(issue(
            "INSTALL_LANGUAGE_REQUIRED",
            None,
            "initial installation requires an explicit artifact language",
        ));
    }

    let mut agents = existing
        .map(|config| {
            config
                .agents
                .iter()
                .filter_map(|value| Agent::parse(value))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if let Some(config) = existing
        && config
            .agents
            .iter()
            .any(|value| Agent::parse(value).is_none())
    {
        issues.push(issue(
            "INSTALL_AGENT_UNSUPPORTED",
            Some(CONFIG_RELATIVE.to_owned()),
            "installed configuration names an unsupported agent",
        ));
    }
    agents.extend(inputs.agents.iter().copied());
    agents.sort_unstable();
    agents.dedup();
    if agents.is_empty() {
        issues.push(issue(
            "INSTALL_AGENT_REQUIRED",
            None,
            "installation requires at least one supported agent",
        ));
    }

    let project_instructions = inputs
        .project_instructions
        .or_else(|| existing.map(|config| config.project_instructions))
        .unwrap_or(false);

    finish(issues)?;
    Ok(ResolvedInputs {
        spec_dir,
        language: language.unwrap_or(ProjectLanguage::En),
        agents,
        project_instructions,
    })
}

fn read_existing_config(project_root: &Path) -> Result<Option<InstalledConfig>, InstallIssues> {
    let path = project_root.join(CONFIG_RELATIVE);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(one_issue(
                "INSTALL_CONFIG_READ_FAILED",
                Some(CONFIG_RELATIVE.to_owned()),
                error.to_string(),
            ));
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(one_issue(
            "INSTALL_CONFIG_TARGET_INVALID",
            Some(CONFIG_RELATIVE.to_owned()),
            ".specbind.json must be a regular non-symlink file",
        ));
    }
    let input = fs::read_to_string(&path).map_err(|error| {
        one_issue(
            "INSTALL_CONFIG_READ_FAILED",
            Some(CONFIG_RELATIVE.to_owned()),
            error.to_string(),
        )
    })?;
    let config = serde_json::from_str::<InstalledConfig>(&input).map_err(|error| {
        one_issue(
            "INSTALL_CONFIG_INVALID",
            Some(CONFIG_RELATIVE.to_owned()),
            format!(".specbind.json is invalid: {error}"),
        )
    })?;
    if config.schema_version != 1 {
        return Err(one_issue(
            "INSTALL_CONFIG_VERSION_UNSUPPORTED",
            Some(CONFIG_RELATIVE.to_owned()),
            "schemaVersion must be 1",
        ));
    }
    Ok(Some(config))
}

fn config_entry(existing: Option<&InstalledConfig>, resolved: &ResolvedInputs) -> PlanEntry {
    let Some(config) = existing else {
        return PlanEntry {
            action: PlanAction::Create,
            path: CONFIG_RELATIVE.to_owned(),
            category: "config",
            detail: None,
        };
    };
    let installed_agents = config
        .agents
        .iter()
        .filter_map(|value| Agent::parse(value))
        .collect::<Vec<_>>();
    let unchanged = config.language == resolved.language
        && config.project_instructions == resolved.project_instructions
        && installed_agents == resolved.agents;
    PlanEntry {
        action: if unchanged {
            PlanAction::Keep
        } else {
            PlanAction::Replace
        },
        path: CONFIG_RELATIVE.to_owned(),
        category: "config",
        detail: unchanged.then(|| "already matches the requested inputs".to_owned()),
    }
}

/// Plans the Decision 0091 customization surface.
///
/// Existing project copies are user-owned under Decision 0008 and are never
/// replaced; only a missing default is created.
fn template_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let templates = template::installed_default_templates(resolved.language);
    if templates.len() != template::INSTALLED_SELECTORS.len() {
        return Err(one_issue(
            "INSTALL_ASSET_UNAVAILABLE",
            None,
            "embedded template assets do not cover the installed selector set",
        ));
    }
    let mut entries = Vec::new();
    for embedded in templates {
        let relative = format!(
            "{}/{}/{}",
            resolved.spec_dir,
            template::SPEC_TEMPLATE_ROOT,
            embedded.output_path
        );
        let target = project_root.join(&relative);
        let action = match fs::symlink_metadata(&target) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => PlanAction::Create,
            Ok(_) => PlanAction::Keep,
            Err(error) => {
                return Err(one_issue(
                    "INSTALL_TARGET_UNREADABLE",
                    Some(relative),
                    error.to_string(),
                ));
            }
        };
        entries.push(PlanEntry {
            action,
            path: relative,
            category: "template",
            detail: (action == PlanAction::Keep)
                .then(|| "project-owned settings are never overwritten".to_owned()),
        });
    }
    Ok(entries)
}

/// Plans the Decision 0093 installed shared-rule set.
///
/// Installed rules are user-owned policy under Decision 0008, so an existing
/// file is kept and only a missing default is created.
fn rule_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let mut entries = Vec::new();
    for default in rule::defaults() {
        let relative = format!(
            "{}/{}/{}",
            resolved.spec_dir,
            rule::RULES_ROOT,
            default.file_name
        );
        let target = project_root.join(&relative);
        let action = match fs::symlink_metadata(&target) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => PlanAction::Create,
            Ok(_) => PlanAction::Keep,
            Err(error) => {
                return Err(one_issue(
                    "INSTALL_TARGET_UNREADABLE",
                    Some(relative),
                    error.to_string(),
                ));
            }
        };
        entries.push(PlanEntry {
            action,
            path: relative,
            category: "rule",
            detail: (action == PlanAction::Keep)
                .then(|| "project-owned settings are never overwritten".to_owned()),
        });
    }
    Ok(entries)
}

/// Decision 0077 permits creating new files in a repository without a commit,
/// but any replacement of an existing file requires a committed clean state.
fn require_replaceable_repository(project_root: &Path) -> Result<(), InstallIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))?;
    if !committed {
        return Err(one_issue(
            "INSTALL_COMMIT_REQUIRED",
            None,
            "replacing an existing file requires at least one commit",
        ));
    }
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("INSTALL_GIT_FAILED", None, error.to_string()))?;
    if status.is_empty() {
        Ok(())
    } else {
        Err(one_issue(
            "INSTALL_REPOSITORY_DIRTY",
            None,
            "replacing an existing file requires a clean repository",
        ))
    }
}

fn finish(mut issues: Vec<InstallIssue>) -> Result<(), InstallIssues> {
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(InstallIssues { issues })
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> InstallIssues {
    InstallIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> InstallIssue {
    InstallIssue {
        code,
        path,
        message: message.into(),
    }
}
