//! Installation input and installed-configuration resolution.

use std::{fs, path::Path};

use crate::agent_role::{self, AgentRoleOverrides};
use crate::config::ProjectLanguage;

use super::{
    Agent, CONFIG_RELATIVE, DEFAULT_SPEC_DIR, InstallInputs, InstallIssue, InstallIssues,
    InstalledConfig, finish, issue, one_issue,
};

/// Reads and validates the complete installed configuration without planning
/// any filesystem change.
///
/// # Errors
///
/// Returns the same configuration and capability diagnostics installation
/// planning would report.
pub fn read_installed_config(project_root: &Path) -> Result<InstalledConfig, InstallIssues> {
    let Some(config) = read_existing_config(project_root)? else {
        return Err(one_issue(
            "INSTALL_CONFIG_REQUIRED",
            Some(CONFIG_RELATIVE.to_owned()),
            "SpecBind is not installed in this project",
        ));
    };
    let resolved = resolve_inputs(Some(&config), &InstallInputs::default())?;
    Ok(InstalledConfig {
        schema_version: config.schema_version,
        spec_dir: resolved.spec_dir,
        language: resolved.language,
        agents: resolved
            .agents
            .iter()
            .map(|agent| agent.name().to_owned())
            .collect(),
        project_instructions: resolved.project_instructions,
        agent_roles: resolved.agent_roles,
    })
}

pub(super) struct ResolvedInputs {
    pub(super) spec_dir: String,
    pub(super) language: ProjectLanguage,
    pub(super) agents: Vec<Agent>,
    pub(super) project_instructions: bool,
    pub(super) agent_roles: AgentRoleOverrides,
}

pub(super) fn resolve_inputs(
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

    let agent_roles = existing
        .map(|config| config.agent_roles.clone())
        .unwrap_or_default();
    issues.extend(agent_role_issues(&agent_roles, &agents));

    finish(issues)?;
    Ok(ResolvedInputs {
        spec_dir,
        language: language.unwrap_or(ProjectLanguage::En),
        agents,
        project_instructions,
        agent_roles,
    })
}

/// Reports role overrides that name an unselected agent or an unusable model.
///
/// An override is capability policy, so it is only meaningful for an agent the
/// installation actually renders.
fn agent_role_issues(overrides: &AgentRoleOverrides, agents: &[Agent]) -> Vec<InstallIssue> {
    let mut issues = Vec::new();
    if overrides.codex.is_some() && !agents.contains(&Agent::Codex) {
        issues.push(issue(
            "INSTALL_AGENT_ROLE_UNUSED",
            Some(CONFIG_RELATIVE.to_owned()),
            "agentRoles.codex requires codex in the selected agents",
        ));
    }
    if overrides.claude_code.is_some() && !agents.contains(&Agent::ClaudeCode) {
        issues.push(issue(
            "INSTALL_AGENT_ROLE_UNUSED",
            Some(CONFIG_RELATIVE.to_owned()),
            "agentRoles.claudeCode requires claude-code in the selected agents",
        ));
    }
    for role in agent_role::all() {
        let models = [
            (
                "codex",
                overrides
                    .codex
                    .as_ref()
                    .and_then(|codex| role.override_from(Some(codex)))
                    .and_then(|value| value.model.as_deref()),
            ),
            (
                "claudeCode",
                overrides
                    .claude_code
                    .as_ref()
                    .and_then(|claude| role.claude_override_from(Some(claude)))
                    .and_then(|value| value.model.as_deref()),
            ),
        ];
        for (agent, model) in models {
            if let Some(model) = model
                && !agent_role::valid_model(model)
            {
                issues.push(issue(
                    "INSTALL_AGENT_ROLE_MODEL_INVALID",
                    Some(CONFIG_RELATIVE.to_owned()),
                    format!(
                        "agentRoles.{agent}.{}.model must use only ASCII letters, digits, '.', '_', or '-'",
                        role.selector
                    ),
                ));
            }
        }
    }
    issues
}

pub(super) fn read_existing_config(
    project_root: &Path,
) -> Result<Option<InstalledConfig>, InstallIssues> {
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
