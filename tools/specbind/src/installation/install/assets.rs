//! Installation asset plan construction.

use std::{fs, path::Path};

use crate::{
    adapter,
    agent_role::{self, AgentRoleOverrides},
    config::ProjectLanguage,
    project_instructions, rule, skill, template,
};

use super::input::ResolvedInputs;
use super::{
    Agent, CONFIG_RELATIVE, InstallIssues, InstalledConfig, PlanAction, PlanEntry, one_issue,
};

pub(super) fn config_entry(
    existing: Option<&InstalledConfig>,
    resolved: &ResolvedInputs,
) -> PlanEntry {
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
        && config.agent_roles == resolved.agent_roles
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
    let agent_roles = render_agent_role_overrides(&resolved.agent_roles);
    let output = format!(
        "{{\n  \"schemaVersion\": 1,\n  \"specDir\": \"{}\",\n  \"language\": \"{language}\",\n  \"agents\": [{agents}]{instructions}{agent_roles}\n}}\n",
        resolved.spec_dir
    );
    output
}

fn render_agent_role_overrides(overrides: &AgentRoleOverrides) -> String {
    let codex = overrides.codex.as_ref().map(|codex| {
        role_block("codex", |role| {
            let role_override = role.override_from(Some(codex))?;
            Some(
                [
                    role_override
                        .model
                        .as_ref()
                        .map(|model| format!("\"model\": \"{model}\"")),
                    role_override
                        .reasoning_effort
                        .map(|effort| format!("\"reasoningEffort\": \"{}\"", effort.name())),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            )
        })
    });
    let claude_code = overrides.claude_code.as_ref().map(|claude| {
        role_block("claudeCode", |role| {
            let role_override = role.claude_override_from(Some(claude))?;
            Some(
                role_override
                    .model
                    .as_ref()
                    .map(|model| format!("\"model\": \"{model}\""))
                    .into_iter()
                    .collect::<Vec<_>>(),
            )
        })
    });
    let blocks = [codex, claude_code]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    if blocks.is_empty() {
        return String::new();
    }
    format!(",\n  \"agentRoles\": {{\n{}\n  }}", blocks.join(",\n"))
}

/// Renders one agent's role overrides as a nested `agentRoles` object.
fn role_block(
    agent: &str,
    fields: impl Fn(&agent_role::AgentRole) -> Option<Vec<String>>,
) -> String {
    let roles = agent_role::all()
        .iter()
        .filter_map(|role| {
            let fields = fields(role)?.join(", ");
            Some(format!("      \"{}\": {{ {fields} }}", role.selector))
        })
        .collect::<Vec<_>>()
        .join(",\n");
    format!("    \"{agent}\": {{\n{roles}\n    }}")
}

/// Plans the Decision 0091 customization surface.
///
/// Existing project copies are user-owned under Decision 0008 and are never
/// replaced; only a missing default is created.
pub(super) fn template_entries(
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
    let roadmap_relative = format!(
        "{}/{}",
        resolved.spec_dir,
        template::MILESTONE_ROADMAP_TEMPLATE_PATH
    );
    let roadmap_target = project_root.join(&roadmap_relative);
    let roadmap_action = match fs::symlink_metadata(&roadmap_target) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => PlanAction::Create,
        Ok(_) => PlanAction::Keep,
        Err(error) => {
            return Err(one_issue(
                "INSTALL_TARGET_UNREADABLE",
                Some(roadmap_relative),
                error.to_string(),
            ));
        }
    };
    let roadmap_content = (roadmap_action == PlanAction::Create)
        .then(|| {
            template::read_embedded_milestone(resolved.language).ok_or_else(|| {
                one_issue(
                    "INSTALL_ASSET_UNAVAILABLE",
                    Some(roadmap_relative.clone()),
                    "embedded milestone template content is unavailable",
                )
            })
        })
        .transpose()?;
    entries.push(PlanEntry {
        action: roadmap_action,
        path: roadmap_relative,
        category: "template",
        detail: (roadmap_action == PlanAction::Keep)
            .then(|| "project-owned settings are never overwritten".to_owned()),
        content: roadmap_content,
        expected_current: None,
    });
    Ok(entries)
}

/// Plans the Decision 0101 operational adapter scaffolds.
///
/// Scaffolds are localized, because a project fills them with its own
/// operational procedure rather than reading the product's judgment from them.
/// Like every other project-owned setting, an existing copy is kept.
pub(super) fn adapter_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let mut entries = Vec::new();
    for entry in adapter::all() {
        let relative = format!("{}/{}", resolved.spec_dir, entry.path());
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
            category: "adapter",
            detail: (action == PlanAction::Keep)
                .then(|| "project-owned settings are never overwritten".to_owned()),
            content: (action == PlanAction::Create)
                .then(|| entry.scaffold(resolved.language).to_owned()),
            expected_current: None,
        });
    }
    Ok(entries)
}

/// Plans the Decision 0093 installed shared-rule set.
///
/// Installed rules are user-owned policy under Decision 0008, so an existing
/// file is kept and only a missing default is created.
pub(super) fn rule_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let mut entries = Vec::new();
    for default in rule::installed_defaults(resolved.language) {
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

/// Plans the Decision 0099 marked block in each selected agent's root
/// instruction file.
///
/// Nothing is planned when project instructions are disabled. Disabling does not
/// remove an existing block: that would delete text from a project-owned file,
/// which Decision 0077 defers along with uninstall.
pub(super) fn project_instruction_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    if !resolved.project_instructions {
        return Ok(vec![]);
    }
    let mut entries = Vec::new();
    let mut planned = std::collections::BTreeSet::new();
    for agent in &resolved.agents {
        let relative = project_instructions::target(*agent);
        if !planned.insert(relative) {
            continue;
        }
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

/// Plans the product-managed skill assets for every selected agent.
///
/// Skills are replaced rather than kept: a local edit is not a supported
/// customization path, and the repository guard below refuses the replacement
/// while that edit is uncommitted.
pub(super) fn skill_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let mut entries = Vec::new();
    let mut planned = std::collections::BTreeSet::new();
    for agent in &resolved.agents {
        for skill in skill::all() {
            let rendered_files = skill.render_files(*agent).map_err(|error| {
                one_issue(
                    "INSTALL_ASSET_UNAVAILABLE",
                    Some(skill.target(*agent)),
                    error.message,
                )
            })?;
            for rendered in rendered_files {
                let relative = rendered.target;
                if !planned.insert(relative.clone()) {
                    continue;
                }
                let target = project_root.join(&relative);
                let action = match fs::read(&target) {
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                        PlanAction::Create
                    }
                    Ok(current) if current == rendered.content.as_bytes() => PlanAction::Keep,
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
                    content: (action != PlanAction::Keep).then_some(rendered.content),
                    expected_current: None,
                });
            }
        }
    }
    Ok(entries)
}

/// Plans removal of exact files from superseded product-managed Skill packages
/// and retired resources inside active packages.
pub(super) fn retired_skill_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let mut entries = Vec::new();
    let mut planned = std::collections::BTreeSet::new();
    for agent in &resolved.agents {
        let root = match agent {
            Agent::ClaudeCode => ".claude/skills",
            Agent::Codex | Agent::Generic => ".agents/skills",
        };
        for name in skill::retired_names() {
            for file in skill::retired_files(name) {
                let relative = format!("{root}/{name}/{file}");
                if !planned.insert(relative.clone()) {
                    continue;
                }
                if let Some(entry) = retired_skill_file_entry(
                    project_root,
                    relative,
                    "retired exact product-managed Skill package file",
                )? {
                    entries.push(entry);
                }
            }
        }
        for embedded in skill::all() {
            for file in embedded.retired_resources() {
                let relative = format!("{root}/{}/{file}", embedded.name);
                if !planned.insert(relative.clone()) {
                    continue;
                }
                if let Some(entry) = retired_skill_file_entry(
                    project_root,
                    relative,
                    "retired exact product-managed Skill package resource",
                )? {
                    entries.push(entry);
                }
            }
        }
    }
    Ok(entries)
}

fn retired_skill_file_entry(
    project_root: &Path,
    relative: String,
    detail: &str,
) -> Result<Option<PlanEntry>, InstallIssues> {
    let target = project_root.join(&relative);
    match fs::symlink_metadata(&target) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Ok(metadata) if metadata.is_file() && !crate::guarded_fs::is_link_like(&metadata) => {
            Ok(Some(PlanEntry {
                action: PlanAction::Remove,
                path: relative,
                category: "skill",
                detail: Some(detail.to_owned()),
                content: None,
                expected_current: None,
            }))
        }
        Ok(_) => Err(one_issue(
            "INSTALL_TARGET_UNREADABLE",
            Some(relative),
            "retired Skill target must be a regular non-symlink file",
        )),
        Err(error) => Err(one_issue(
            "INSTALL_TARGET_UNREADABLE",
            Some(relative),
            error.to_string(),
        )),
    }
}

/// Plans each selected host's execution adapter for the stable roles named by
/// skills.
///
/// Role semantics remain product-managed. A project may override only model
/// capability through `.specbind.json`; the resulting files are derived assets
/// and are replaced under the same repository guard as skills.
pub(super) fn agent_role_entries(
    project_root: &Path,
    resolved: &ResolvedInputs,
) -> Result<Vec<PlanEntry>, InstallIssues> {
    let mut rendered_roles = Vec::new();
    if resolved.agents.contains(&Agent::Codex) {
        let overrides = resolved.agent_roles.codex.as_ref();
        rendered_roles.extend(
            agent_role::all()
                .iter()
                .map(|role| (role.target(), role.render(overrides))),
        );
    }
    if resolved.agents.contains(&Agent::ClaudeCode) {
        let overrides = resolved.agent_roles.claude_code.as_ref();
        rendered_roles.extend(
            agent_role::all()
                .iter()
                .map(|role| (role.claude_target(), role.render_claude(overrides))),
        );
    }
    let mut entries = Vec::new();
    for (relative, rendered) in rendered_roles {
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
            category: "agent-role",
            detail: (action == PlanAction::Keep)
                .then(|| "already matches the configured capability".to_owned()),
            content: (action != PlanAction::Keep).then_some(rendered),
            expected_current: None,
        });
    }
    Ok(entries)
}
