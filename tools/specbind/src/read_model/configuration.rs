//! Read-only projection over every supported project configuration surface.

use std::{fmt, fs, path::Path};

use crate::{
    adapter,
    agent_role::{self, ReasoningEffort},
    config::ProjectLanguage,
    install::{self, Agent},
    instruction, rule, steering, template,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationReport {
    pub spec_dir: String,
    pub language: ProjectLanguage,
    pub agents: Vec<Agent>,
    pub project_instructions: bool,
    pub roles: Vec<RoleConfiguration>,
    pub templates: Vec<ConfigurationItem>,
    pub rules: Vec<ConfigurationItem>,
    pub adapters: Vec<ConfigurationItem>,
    pub steering_documents: usize,
    pub attention: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleConfiguration {
    pub agent: Agent,
    pub selector: &'static str,
    pub model: String,
    pub reasoning_effort: Option<ReasoningEffort>,
    pub overridden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationItem {
    pub selector: String,
    pub state: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationReportError {
    pub diagnostics: Vec<String>,
}

impl fmt::Display for ConfigurationReportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "configuration has {} diagnostic(s)",
            self.diagnostics.len()
        )
    }
}

impl std::error::Error for ConfigurationReportError {}

/// Resolves the complete current configuration without authorizing a change.
///
/// # Errors
///
/// Returns every deterministic diagnostic found while resolving the supported
/// configuration catalogs.
pub fn resolve(project_root: &Path) -> Result<ConfigurationReport, ConfigurationReportError> {
    let config =
        install::read_installed_config(project_root).map_err(|error| ConfigurationReportError {
            diagnostics: error
                .issues
                .iter()
                .map(|issue| format!("{} {}", issue.code, issue.message))
                .collect(),
        })?;
    let specbind_root = project_root.join(&config.spec_dir);
    let agents = config
        .agents
        .iter()
        .filter_map(|agent| Agent::parse(agent))
        .collect::<Vec<_>>();
    let mut diagnostics = Vec::new();
    let role_configurations = resolve_roles(&agents, &config.agent_roles);
    let templates = resolve_templates(&specbind_root, config.language, &mut diagnostics);
    let rule_items = resolve_rules(&specbind_root, config.language, &mut diagnostics);
    let (adapters, mut attention) =
        resolve_adapters(&specbind_root, config.language, &mut diagnostics);
    let steering_documents = match steering::discover(&specbind_root) {
        Ok(inventory) => {
            diagnostics.extend(
                inventory
                    .issues
                    .iter()
                    .map(|issue| format!("{} {}", issue.code, issue.message)),
            );
            inventory.documents.len()
        }
        Err(error) => {
            diagnostics.push(format!("STEERING_SCAN_FAILED {error}"));
            0
        }
    };
    if steering_documents == 0 {
        attention.push("no Steering documents are present".to_owned());
    }
    if !diagnostics.is_empty() {
        return Err(ConfigurationReportError { diagnostics });
    }
    Ok(ConfigurationReport {
        spec_dir: config.spec_dir,
        language: config.language,
        agents,
        project_instructions: config.project_instructions,
        roles: role_configurations,
        templates,
        rules: rule_items,
        adapters,
        steering_documents,
        attention,
    })
}

fn resolve_roles(
    agents: &[Agent],
    overrides: &agent_role::AgentRoleOverrides,
) -> Vec<RoleConfiguration> {
    let mut roles = Vec::new();
    for agent in agents {
        for role in agent_role::all() {
            match agent {
                Agent::Codex => {
                    let role_override = role.override_from(overrides.codex.as_ref());
                    roles.push(RoleConfiguration {
                        agent: *agent,
                        selector: role.selector,
                        model: role_override
                            .and_then(|value| value.model.clone())
                            .unwrap_or_else(|| role.default_model.to_owned()),
                        reasoning_effort: Some(
                            role_override
                                .and_then(|value| value.reasoning_effort)
                                .unwrap_or(role.default_reasoning_effort),
                        ),
                        overridden: role_override.is_some_and(|value| {
                            value.model.is_some() || value.reasoning_effort.is_some()
                        }),
                    });
                }
                Agent::ClaudeCode => {
                    let role_override = role.claude_override_from(overrides.claude_code.as_ref());
                    roles.push(RoleConfiguration {
                        agent: *agent,
                        selector: role.selector,
                        model: role_override
                            .and_then(|value| value.model.clone())
                            .unwrap_or_else(|| role.default_claude_model.to_owned()),
                        reasoning_effort: None,
                        overridden: role_override.is_some_and(|value| value.model.is_some()),
                    });
                }
                Agent::Generic => {}
            }
        }
    }
    roles
}

fn resolve_templates(
    specbind_root: &Path,
    language: ProjectLanguage,
    diagnostics: &mut Vec<String>,
) -> Vec<ConfigurationItem> {
    let mut items = Vec::new();
    let spec = template::discover_spec_templates(specbind_root, language);
    diagnostics.extend(
        spec.issues
            .iter()
            .map(|issue| format!("{} {}", issue.code, issue.message)),
    );
    for entry in &spec.templates {
        let state = template_state(
            specbind_root,
            entry.source,
            entry.template_path.as_std_path(),
            template::read_embedded(language, &entry.selector).as_deref(),
            diagnostics,
        );
        items.push(ConfigurationItem {
            selector: format!("spec/{}", entry.selector),
            state,
        });
    }
    let steering = template::discover_steering_templates(specbind_root, language);
    diagnostics.extend(
        steering
            .issues
            .iter()
            .map(|issue| format!("{} {}", issue.code, issue.message)),
    );
    for entry in &steering.templates {
        let state = template_state(
            specbind_root,
            entry.source,
            entry.template_path.as_std_path(),
            template::read_embedded_steering(language, &entry.selector).as_deref(),
            diagnostics,
        );
        items.push(ConfigurationItem {
            selector: format!("steering/{}", entry.selector),
            state,
        });
    }
    let milestone = template::discover_milestone_templates(specbind_root, language);
    diagnostics.extend(
        milestone
            .issues
            .iter()
            .map(|issue| format!("{} {}", issue.code, issue.message)),
    );
    for entry in &milestone.templates {
        let state = template_state(
            specbind_root,
            entry.source,
            entry.template_path.as_std_path(),
            template::read_embedded_milestone(language).as_deref(),
            diagnostics,
        );
        items.push(ConfigurationItem {
            selector: format!("milestone/{}", entry.selector),
            state,
        });
    }
    items.sort_by(|left, right| left.selector.cmp(&right.selector));
    items
}

fn template_state(
    specbind_root: &Path,
    source: template::TemplateSource,
    path: &Path,
    embedded: Option<&str>,
    diagnostics: &mut Vec<String>,
) -> &'static str {
    if source == template::TemplateSource::Embedded {
        return "embedded-fallback";
    }
    match fs::read_to_string(specbind_root.join(path)) {
        Ok(content) if embedded == Some(content.as_str()) => "current-default",
        Ok(_) => "project-content",
        Err(error) => {
            diagnostics.push(format!("TEMPLATE_READ_FAILED {}: {error}", path.display()));
            "invalid"
        }
    }
}

fn resolve_rules(
    specbind_root: &Path,
    language: ProjectLanguage,
    diagnostics: &mut Vec<String>,
) -> Vec<ConfigurationItem> {
    let spec_templates = template::discover_spec_templates(specbind_root, language);
    let design_selectors = spec_templates
        .templates
        .iter()
        .filter(|entry| entry.artifact_type == "SpecBind Design")
        .map(|entry| entry.selector.clone())
        .collect::<Vec<_>>();
    let mut items = Vec::new();
    for entry in rule::defaults() {
        match entry.read(specbind_root) {
            Ok(Some(content)) => {
                diagnostics.extend(
                    instruction::validate_live(&content)
                        .iter()
                        .map(|issue| format!("{} {}: {}", issue.code, entry.path(), issue.message)),
                );
                if entry.selector == "design-template-selection" {
                    diagnostics.extend(
                        rule::validate_design_template_selection(&content, &design_selectors)
                            .iter()
                            .map(|issue| format!("{} {}", issue.code, issue.message)),
                    );
                }
                items.push(ConfigurationItem {
                    selector: entry.selector.to_owned(),
                    state: if content == entry.content() {
                        "current-default"
                    } else {
                        "project-content"
                    },
                });
            }
            Ok(None) => {
                if entry.selector == "design-template-selection" {
                    diagnostics.push(format!("RULE_REQUIRED missing {}", entry.path()));
                }
                items.push(ConfigurationItem {
                    selector: entry.selector.to_owned(),
                    state: "absent",
                });
            }
            Err(error) => diagnostics.push(format!("{} {}", error.code, error.message)),
        }
    }
    items
}

fn resolve_adapters(
    specbind_root: &Path,
    language: ProjectLanguage,
    diagnostics: &mut Vec<String>,
) -> (Vec<ConfigurationItem>, Vec<String>) {
    let mut items = Vec::new();
    let mut attention = Vec::new();
    for entry in adapter::all() {
        match (entry.state(specbind_root), entry.read(specbind_root)) {
            (Ok(state), Ok(content)) => {
                let item_state = match (state, content.as_deref()) {
                    (adapter::AdapterState::Absent, _) => "absent",
                    (adapter::AdapterState::Scaffold, _) => "scaffold",
                    (adapter::AdapterState::Active, Some(content))
                        if content == entry.scaffold(language) =>
                    {
                        "current-default"
                    }
                    (adapter::AdapterState::Active, _) => "project-content",
                };
                if entry.selector == "release"
                    && matches!(
                        state,
                        adapter::AdapterState::Absent | adapter::AdapterState::Scaffold
                    )
                {
                    attention.push("Release adapter is not configured".to_owned());
                }
                items.push(ConfigurationItem {
                    selector: entry.selector.to_owned(),
                    state: item_state,
                });
            }
            (Err(error), _) | (_, Err(error)) => {
                diagnostics.push(format!("{} {}", error.code, error.message));
            }
        }
    }
    (items, attention)
}
