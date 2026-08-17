//! Read-only installation planning.
//!
//! Planning resolves the effective project configuration and reports exactly
//! which files an install would create, replace, or leave untouched. It never
//! writes, and it refuses to plan a replacement the Decision 0077 repository
//! guards would not permit.

use std::{fmt, fs, path::Path};

use serde::Deserialize;

use crate::{
    config::ProjectLanguage, guarded_fs, project_instructions, repository, rule, skill, template,
};

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
    entries.extend(skill_entries(project_root, &resolved)?);
    entries.extend(project_instruction_entries(project_root, &resolved)?);
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
    let rendered = render_config(resolved);
    let Some(config) = existing else {
        return PlanEntry {
            action: PlanAction::Create,
            path: CONFIG_RELATIVE.to_owned(),
            category: "config",
            detail: None,
            content: Some(rendered),
            expected_current: None,
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
        content: (!unchanged).then_some(rendered),
        expected_current: None,
    }
}

/// Renders the version-controlled project configuration deterministically.
fn render_config(resolved: &ResolvedInputs) -> String {
    let agents = resolved
        .agents
        .iter()
        .map(|agent| format!("\"{}\"", agent.name()))
        .collect::<Vec<_>>()
        .join(", ");
    let language = match resolved.language {
        ProjectLanguage::En => "en",
        ProjectLanguage::Ja => "ja",
    };
    let instructions = if resolved.project_instructions {
        ",\n  \"projectInstructions\": true"
    } else {
        ""
    };
    let output = format!(
        "{{\n  \"schemaVersion\": 1,\n  \"specDir\": \"{}\",\n  \"language\": \"{language}\",\n  \"agents\": [{agents}]{instructions}\n}}\n",
        resolved.spec_dir
    );
    output
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
        let content = (action == PlanAction::Create)
            .then(|| {
                template::read_embedded(resolved.language, &embedded.selector).ok_or_else(|| {
                    one_issue(
                        "INSTALL_ASSET_UNAVAILABLE",
                        Some(relative.clone()),
                        "embedded template content is unavailable",
                    )
                })
            })
            .transpose()?;
        entries.push(PlanEntry {
            action,
            path: relative,
            category: "template",
            detail: (action == PlanAction::Keep)
                .then(|| "project-owned settings are never overwritten".to_owned()),
            content,
            expected_current: None,
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
            content: (action == PlanAction::Create).then(|| default.content().to_owned()),
            expected_current: None,
        });
    }
    Ok(entries)
}

/// Plans the product-managed skill assets for every selected agent.
///
/// Skills are replaced rather than kept: a local edit is not a supported
/// customization path, and the repository guard below refuses the replacement
/// while that edit is uncommitted.
/// Plans the Decision 0099 marked block in each selected agent's root
/// instruction file.
///
/// Nothing is planned when project instructions are disabled. Disabling does not
/// remove an existing block: that would delete text from a project-owned file,
/// which Decision 0077 defers along with uninstall.
fn project_instruction_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    if !resolved.project_instructions {
        return Ok(vec![]);
    }
    let mut entries = Vec::new();
    for agent in &resolved.agents {
        let relative = project_instructions::target(*agent);
        let target = project_root.join(relative);
        let current = match fs::read(&target) {
            Ok(bytes) => Some(String::from_utf8(bytes).map_err(|_| {
                one_issue(
                    "INSTALL_TARGET_NOT_UTF8",
                    Some(relative.to_owned()),
                    "agent instruction file must be UTF-8",
                )
            })?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                return Err(one_issue(
                    "INSTALL_TARGET_UNREADABLE",
                    Some(relative.to_owned()),
                    error.to_string(),
                ));
            }
        };
        let applied = project_instructions::apply(current.as_deref())
            .map_err(|error| one_issue(error.code, Some(relative.to_owned()), error.message))?;
        // The entry describes the block, not the file. Adding a block to an
        // existing file removes no text, so it is a creation rather than a
        // Decision 0077 replacement and needs no committed clean repository.
        let action = if applied.had_block {
            if current.as_deref() == Some(applied.content.as_str()) {
                PlanAction::Keep
            } else {
                PlanAction::Replace
            }
        } else {
            PlanAction::Create
        };
        let detail = match action {
            PlanAction::Keep => Some("already matches the current product asset".to_owned()),
            PlanAction::Create if current.is_some() => {
                Some("appended to the existing instruction file".to_owned())
            }
            _ => None,
        };
        entries.push(PlanEntry {
            action,
            path: relative.to_owned(),
            category: "project-instructions",
            detail,
            content: (action != PlanAction::Keep).then_some(applied.content),
            expected_current: current,
        });
    }
    Ok(entries)
}

fn skill_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let mut entries = Vec::new();
    for agent in &resolved.agents {
        for skill in skill::all() {
            let relative = skill.target(*agent);
            let rendered = skill.render(*agent).map_err(|error| {
                one_issue(
                    "INSTALL_ASSET_UNAVAILABLE",
                    Some(relative.clone()),
                    error.message,
                )
            })?;
            let target = project_root.join(&relative);
            let action = match fs::read(&target) {
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => PlanAction::Create,
                Ok(current) if current == rendered.as_bytes() => PlanAction::Keep,
                Ok(_) => PlanAction::Replace,
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
                category: "skill",
                detail: (action == PlanAction::Keep)
                    .then(|| "already matches the current product asset".to_owned()),
                content: (action != PlanAction::Keep).then_some(rendered),
                expected_current: None,
            });
        }
    }
    Ok(entries)
}

/// Decision 0077 permits creating new files in a repository without a commit,
/// but any replacement of an existing file requires a committed clean state.
/// Applies a freshly computed plan, writing the Roadmap-style config last.
///
/// # Errors
///
/// Returns planning, race, or guarded-write diagnostics. A failure may leave
/// earlier assets written; a later run converges because missing defaults are
/// created and existing project files are kept.
pub fn apply(project_root: &Path, inputs: &InstallInputs) -> Result<InstallOutcome, InstallIssues> {
    let plan = plan(project_root, inputs)?;
    let unchanged = plan
        .entries
        .iter()
        .all(|entry| entry.action == PlanAction::Keep);
    if unchanged {
        return Ok(InstallOutcome {
            plan,
            unchanged: true,
        });
    }
    // Assets first, configuration last: a project only claims to be installed
    // once the assets its skills read actually exist.
    let ordered = plan
        .entries
        .iter()
        .filter(|entry| entry.category != "config")
        .chain(
            plan.entries
                .iter()
                .filter(|entry| entry.category == "config"),
        );
    for entry in ordered {
        let Some(content) = entry.content.as_deref() else {
            continue;
        };
        let target = project_root.join(&entry.path);
        verify_expected_state(&target, entry)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                one_issue(
                    "INSTALL_WRITE_FAILED",
                    Some(entry.path.clone()),
                    error.to_string(),
                )
            })?;
        }
        guarded_fs::replace_optional(&target, content.as_bytes()).map_err(|error| {
            one_issue(
                "INSTALL_WRITE_FAILED",
                Some(entry.path.clone()),
                error.to_string(),
            )
        })?;
    }
    Ok(InstallOutcome {
        plan,
        unchanged: false,
    })
}

/// Fails closed when the filesystem no longer matches the planned action.
fn verify_expected_state(target: &Path, entry: &PlanEntry) -> Result<(), InstallIssues> {
    let present = match fs::symlink_metadata(target) {
        Ok(_) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(one_issue(
                "INSTALL_TARGET_UNREADABLE",
                Some(entry.path.clone()),
                error.to_string(),
            ));
        }
    };
    if entry.action == PlanAction::Keep {
        return Ok(());
    }
    if let Some(expected) = &entry.expected_current {
        // An in-place edit leaves the file present either way, so presence
        // proves nothing. Compare the bytes the plan actually read.
        return match fs::read(target) {
            Ok(current) if current == expected.as_bytes() => Ok(()),
            Ok(_) => Err(one_issue(
                "INSTALL_TARGET_CHANGED",
                Some(entry.path.clone()),
                "installation target changed after the plan was computed",
            )),
            Err(error) => Err(one_issue(
                "INSTALL_TARGET_UNREADABLE",
                Some(entry.path.clone()),
                error.to_string(),
            )),
        };
    }
    let expected = match entry.action {
        PlanAction::Create => false,
        PlanAction::Replace => true,
        PlanAction::Keep => return Ok(()),
    };
    if present == expected {
        Ok(())
    } else {
        Err(one_issue(
            "INSTALL_TARGET_CHANGED",
            Some(entry.path.clone()),
            "installation target changed after the plan was computed",
        ))
    }
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
