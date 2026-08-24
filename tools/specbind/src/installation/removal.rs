//! Guarded removal of one agent integration or the complete project integration.

use std::{fmt, fs, path::Path};

use serde_json::Value;

use crate::{
    agent_role, config, guarded_fs, install::Agent, project_instructions, repository, skill,
};

const CONFIG_RELATIVE: &str = ".specbind.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgePolicy {
    Retain,
    Remove,
}

impl KnowledgePolicy {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Retain => "retain",
            Self::Remove => "remove",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalAction {
    Remove,
    Update,
    Retain,
    Absent,
}

impl RemovalAction {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Remove => "remove",
            Self::Update => "update",
            Self::Retain => "retain",
            Self::Absent => "absent",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mutation {
    None,
    RemoveFile,
    RemoveTree,
    Replace(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovalEntry {
    pub action: RemovalAction,
    pub path: String,
    pub category: &'static str,
    pub detail: String,
    mutation: Mutation,
    expected_current: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovalPlan {
    pub agent: Option<Agent>,
    pub knowledge: Option<KnowledgePolicy>,
    pub entries: Vec<RemovalEntry>,
    pub unchanged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RemovalIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovalIssues {
    pub issues: Vec<RemovalIssue>,
}

impl fmt::Display for RemovalIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "removal planning has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for RemovalIssues {}

struct InstalledConfig {
    bytes: Vec<u8>,
    value: Value,
    spec_dir: String,
    agents: Vec<Agent>,
    project_instructions: bool,
}

/// Plans removal of exactly one selected agent integration.
///
/// # Errors
///
/// Returns configuration, ownership, repository, or filesystem diagnostics
/// when an exact safe plan cannot be established.
pub fn plan_agent(project_root: &Path, agent: Agent) -> Result<RemovalPlan, RemovalIssues> {
    let Some(mut installed) = read_config(project_root)? else {
        return Err(one_issue(
            "REMOVE_AGENT_NOT_INSTALLED",
            Some(CONFIG_RELATIVE.to_owned()),
            "cannot remove an agent after the project integration is uninstalled",
        ));
    };
    if !installed.agents.contains(&agent) {
        return Ok(RemovalPlan {
            agent: Some(agent),
            knowledge: None,
            entries: vec![],
            unchanged: true,
        });
    }
    if installed.agents.len() == 1 {
        return Err(one_issue(
            "REMOVE_AGENT_LAST_AGENT",
            Some(CONFIG_RELATIVE.to_owned()),
            "the last selected agent is removed only by project uninstall",
        ));
    }
    require_commit(project_root)?;
    let mut entries = agent_entries(project_root, agent, installed.project_instructions)?;
    installed.agents.retain(|selected| *selected != agent);
    update_config_value(&mut installed.value, agent, &installed.agents)?;
    entries.push(config_update_entry(project_root, &installed)?);
    validate_repository_state(project_root, &entries, None)?;
    Ok(RemovalPlan {
        agent: Some(agent),
        knowledge: None,
        entries,
        unchanged: false,
    })
}

/// Plans project uninstall under an explicit durable-knowledge policy.
///
/// # Errors
///
/// Returns configuration, ownership, repository, or filesystem diagnostics
/// when an exact safe plan cannot be established.
pub fn plan_uninstall(
    project_root: &Path,
    knowledge: KnowledgePolicy,
) -> Result<RemovalPlan, RemovalIssues> {
    let Some(installed) = read_config(project_root)? else {
        return Ok(RemovalPlan {
            agent: None,
            knowledge: Some(knowledge),
            entries: vec![],
            unchanged: true,
        });
    };
    require_commit(project_root)?;
    let mut entries = Vec::new();
    for agent in &installed.agents {
        entries.extend(agent_entries(
            project_root,
            *agent,
            installed.project_instructions,
        )?);
    }
    match knowledge {
        KnowledgePolicy::Retain => entries.push(RemovalEntry {
            action: RemovalAction::Retain,
            path: installed.spec_dir.clone(),
            category: "knowledge",
            detail: "explicit retain policy preserves the complete durable knowledge bundle"
                .to_owned(),
            mutation: Mutation::None,
            expected_current: None,
        }),
        KnowledgePolicy::Remove => entries.push(tree_entry(project_root, &installed.spec_dir)?),
    }
    entries.push(file_entry(
        project_root,
        CONFIG_RELATIVE,
        "config",
        "configuration is the uninstall completion marker",
    )?);
    validate_repository_state(
        project_root,
        &entries,
        (knowledge == KnowledgePolicy::Remove).then_some(installed.spec_dir.as_str()),
    )?;
    Ok(RemovalPlan {
        agent: None,
        knowledge: Some(knowledge),
        entries,
        unchanged: false,
    })
}

/// Applies a freshly recomputed agent-removal plan.
///
/// # Errors
///
/// Returns planning, race, or guarded-write diagnostics without advancing the
/// configuration completion marker.
pub fn apply_agent(project_root: &Path, agent: Agent) -> Result<RemovalPlan, RemovalIssues> {
    let plan = plan_agent(project_root, agent)?;
    apply_plan(project_root, &plan)?;
    Ok(plan)
}

/// Applies a freshly recomputed project-uninstall plan.
///
/// # Errors
///
/// Returns planning, race, or guarded-write diagnostics without deleting the
/// configuration completion marker.
pub fn apply_uninstall(
    project_root: &Path,
    knowledge: KnowledgePolicy,
) -> Result<RemovalPlan, RemovalIssues> {
    let plan = plan_uninstall(project_root, knowledge)?;
    apply_plan(project_root, &plan)?;
    Ok(plan)
}

fn apply_plan(project_root: &Path, plan: &RemovalPlan) -> Result<(), RemovalIssues> {
    if plan.unchanged {
        return Ok(());
    }
    let removable_tree = plan
        .entries
        .iter()
        .find(|entry| matches!(entry.mutation, Mutation::RemoveTree))
        .map(|entry| entry.path.as_str());
    validate_repository_state(project_root, &plan.entries, removable_tree)?;
    for entry in &plan.entries {
        match &entry.mutation {
            Mutation::None => {}
            Mutation::RemoveFile => {
                revalidate_file(project_root, entry)?;
                fs::remove_file(project_root.join(&entry.path)).map_err(|error| {
                    one_issue(
                        "REMOVAL_WRITE_FAILED",
                        Some(entry.path.clone()),
                        error.to_string(),
                    )
                })?;
            }
            Mutation::RemoveTree => {
                validate_tree(project_root, &entry.path)?;
                fs::remove_dir_all(project_root.join(&entry.path)).map_err(|error| {
                    one_issue(
                        "REMOVAL_WRITE_FAILED",
                        Some(entry.path.clone()),
                        error.to_string(),
                    )
                })?;
            }
            Mutation::Replace(content) => {
                revalidate_file(project_root, entry)?;
                guarded_fs::replace_existing(&project_root.join(&entry.path), content).map_err(
                    |error| {
                        one_issue(
                            "REMOVAL_WRITE_FAILED",
                            Some(entry.path.clone()),
                            error.to_string(),
                        )
                    },
                )?;
            }
        }
    }
    Ok(())
}

fn agent_entries(
    project_root: &Path,
    agent: Agent,
    project_instructions_enabled: bool,
) -> Result<Vec<RemovalEntry>, RemovalIssues> {
    let mut entries = Vec::new();
    for embedded in skill::all() {
        entries.push(file_entry(
            project_root,
            &embedded.target(agent),
            "skill",
            "exact product-managed skill target",
        )?);
    }
    for role in agent_role::all() {
        let path = match agent {
            Agent::Codex => role.target(),
            Agent::ClaudeCode => role.claude_target(),
        };
        entries.push(file_entry(
            project_root,
            &path,
            "agent-role",
            "exact product-managed agent-role target",
        )?);
    }
    for root in match agent {
        Agent::Codex => [".agents/skills", ".codex/agents"],
        Agent::ClaudeCode => [".claude/skills", ".claude/agents"],
    } {
        entries.push(container_entry(project_root, root)?);
    }
    let instruction_path = project_instructions::target(agent);
    entries.push(instruction_entry(
        project_root,
        instruction_path,
        project_instructions_enabled,
    )?);
    Ok(entries)
}

fn container_entry(project_root: &Path, relative: &str) -> Result<RemovalEntry, RemovalIssues> {
    match fs::symlink_metadata(project_root.join(relative)) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(RemovalEntry {
            action: RemovalAction::Absent,
            path: relative.to_owned(),
            category: "container",
            detail: "agent container is already absent".to_owned(),
            mutation: Mutation::None,
            expected_current: None,
        }),
        Err(error) => Err(one_issue(
            "REMOVAL_TARGET_UNREADABLE",
            Some(relative.to_owned()),
            error.to_string(),
        )),
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() => {
            Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative.to_owned()),
                "agent containers must be regular non-link directories",
            ))
        }
        Ok(_) => {
            validate_path_chain(project_root, relative)?;
            Ok(RemovalEntry {
                action: RemovalAction::Retain,
                path: relative.to_owned(),
                category: "container",
                detail: "the directory and any content outside exact catalog targets are retained"
                    .to_owned(),
                mutation: Mutation::None,
                expected_current: None,
            })
        }
    }
}

fn file_entry(
    project_root: &Path,
    relative: &str,
    category: &'static str,
    detail: &str,
) -> Result<RemovalEntry, RemovalIssues> {
    let path = project_root.join(relative);
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(RemovalEntry {
            action: RemovalAction::Absent,
            path: relative.to_owned(),
            category,
            detail: "already absent; retry-safe".to_owned(),
            mutation: Mutation::None,
            expected_current: None,
        }),
        Err(error) => Err(one_issue(
            "REMOVAL_TARGET_UNREADABLE",
            Some(relative.to_owned()),
            error.to_string(),
        )),
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_file() => {
            Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative.to_owned()),
                "removal targets must be regular non-link files",
            ))
        }
        Ok(_) => {
            validate_path_chain(project_root, relative)?;
            validate_tracked_not_ignored(project_root, relative)?;
            let current = fs::read(&path).map_err(|error| {
                one_issue(
                    "REMOVAL_TARGET_UNREADABLE",
                    Some(relative.to_owned()),
                    error.to_string(),
                )
            })?;
            Ok(RemovalEntry {
                action: RemovalAction::Remove,
                path: relative.to_owned(),
                category,
                detail: detail.to_owned(),
                mutation: Mutation::RemoveFile,
                expected_current: Some(current),
            })
        }
    }
}

fn instruction_entry(
    project_root: &Path,
    relative: &str,
    was_enabled: bool,
) -> Result<RemovalEntry, RemovalIssues> {
    let path = project_root.join(relative);
    let current = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(RemovalEntry {
                action: RemovalAction::Absent,
                path: relative.to_owned(),
                category: "project-instructions",
                detail: "instruction file is absent".to_owned(),
                mutation: Mutation::None,
                expected_current: None,
            });
        }
        Err(error) => {
            return Err(one_issue(
                "REMOVAL_TARGET_UNREADABLE",
                Some(relative.to_owned()),
                error.to_string(),
            ));
        }
    };
    let text = String::from_utf8(current.clone()).map_err(|_| {
        one_issue(
            "REMOVAL_TARGET_NOT_UTF8",
            Some(relative.to_owned()),
            "agent instruction file must be UTF-8",
        )
    })?;
    let removed = project_instructions::remove(&text)
        .map_err(|error| one_issue(error.code, Some(relative.to_owned()), error.message))?;
    let Some(content) = removed else {
        let completed = completed_instruction_removal(project_root, relative, &current)?;
        return Ok(RemovalEntry {
            action: if completed {
                RemovalAction::Absent
            } else {
                RemovalAction::Retain
            },
            path: relative.to_owned(),
            category: "project-instructions",
            detail: if completed {
                "managed block is already removed; retry-safe"
            } else if was_enabled {
                "no managed block remains; unrelated project content is retained"
            } else {
                "project instructions were disabled; project content is retained"
            }
            .to_owned(),
            mutation: Mutation::None,
            expected_current: None,
        });
    };
    validate_path_chain(project_root, relative)?;
    validate_tracked_not_ignored(project_root, relative)?;
    if content.is_empty() {
        Ok(RemovalEntry {
            action: RemovalAction::Remove,
            path: relative.to_owned(),
            category: "project-instructions",
            detail: "file contains only the exact managed block".to_owned(),
            mutation: Mutation::RemoveFile,
            expected_current: Some(current),
        })
    } else {
        Ok(RemovalEntry {
            action: RemovalAction::Update,
            path: relative.to_owned(),
            category: "project-instructions",
            detail: "remove only the marked block and preserve surrounding project text".to_owned(),
            mutation: Mutation::Replace(content.into_bytes()),
            expected_current: Some(current),
        })
    }
}

fn tree_entry(project_root: &Path, relative: &str) -> Result<RemovalEntry, RemovalIssues> {
    let path = project_root.join(relative);
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(RemovalEntry {
            action: RemovalAction::Absent,
            path: relative.to_owned(),
            category: "knowledge",
            detail: "durable knowledge bundle is already absent; retry-safe".to_owned(),
            mutation: Mutation::None,
            expected_current: None,
        }),
        Err(error) => Err(one_issue(
            "REMOVAL_TARGET_UNREADABLE",
            Some(relative.to_owned()),
            error.to_string(),
        )),
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() => {
            Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative.to_owned()),
                "configured specDir must be a regular non-link directory",
            ))
        }
        Ok(_) => {
            validate_tree(project_root, relative)?;
            Ok(RemovalEntry {
                action: RemovalAction::Remove,
                path: relative.to_owned(),
                category: "knowledge",
                detail: "explicit remove policy deletes the complete Git-recoverable bundle"
                    .to_owned(),
                mutation: Mutation::RemoveTree,
                expected_current: None,
            })
        }
    }
}

fn config_update_entry(
    project_root: &Path,
    installed: &InstalledConfig,
) -> Result<RemovalEntry, RemovalIssues> {
    validate_path_chain(project_root, CONFIG_RELATIVE)?;
    validate_tracked_not_ignored(project_root, CONFIG_RELATIVE)?;
    let content = serde_json::to_string_pretty(&installed.value)
        .map_err(|error| one_issue("REMOVE_AGENT_CONFIG_INVALID", None, error.to_string()))?
        + "\n";
    Ok(RemovalEntry {
        action: RemovalAction::Update,
        path: CONFIG_RELATIVE.to_owned(),
        category: "config",
        detail: "remove the selected agent and its role overrides last".to_owned(),
        mutation: Mutation::Replace(content.into_bytes()),
        expected_current: Some(installed.bytes.clone()),
    })
}

fn update_config_value(
    value: &mut Value,
    removed: Agent,
    remaining: &[Agent],
) -> Result<(), RemovalIssues> {
    let object = value.as_object_mut().ok_or_else(|| {
        one_issue(
            "REMOVE_AGENT_CONFIG_INVALID",
            Some(CONFIG_RELATIVE.to_owned()),
            ".specbind.json must contain an object",
        )
    })?;
    object.insert(
        "agents".to_owned(),
        Value::Array(
            remaining
                .iter()
                .map(|agent| Value::String(agent.name().to_owned()))
                .collect(),
        ),
    );
    if let Some(roles) = object.get_mut("agentRoles").and_then(Value::as_object_mut) {
        roles.remove(match removed {
            Agent::Codex => "codex",
            Agent::ClaudeCode => "claudeCode",
        });
        if roles.is_empty() {
            object.remove("agentRoles");
        }
    }
    Ok(())
}

fn read_config(project_root: &Path) -> Result<Option<InstalledConfig>, RemovalIssues> {
    let path = project_root.join(CONFIG_RELATIVE);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(one_issue(
                "REMOVAL_CONFIG_READ_FAILED",
                Some(CONFIG_RELATIVE.to_owned()),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "REMOVAL_CONFIG_UNSAFE",
            Some(CONFIG_RELATIVE.to_owned()),
            ".specbind.json must be a regular non-link file",
        ));
    }
    let bytes = fs::read(&path).map_err(|error| {
        one_issue(
            "REMOVAL_CONFIG_READ_FAILED",
            Some(CONFIG_RELATIVE.to_owned()),
            error.to_string(),
        )
    })?;
    parse_config(bytes).map(Some)
}

fn parse_config(bytes: Vec<u8>) -> Result<InstalledConfig, RemovalIssues> {
    let value = serde_json::from_slice::<Value>(&bytes).map_err(|error| {
        one_issue(
            "REMOVAL_CONFIG_INVALID",
            Some(CONFIG_RELATIVE.to_owned()),
            error.to_string(),
        )
    })?;
    let object = value.as_object().ok_or_else(|| {
        one_issue(
            "REMOVAL_CONFIG_INVALID",
            Some(CONFIG_RELATIVE.to_owned()),
            ".specbind.json must contain an object",
        )
    })?;
    if object.get("schemaVersion").and_then(Value::as_u64) != Some(1) {
        return Err(one_issue(
            "REMOVAL_CONFIG_VERSION_UNSUPPORTED",
            Some(CONFIG_RELATIVE.to_owned()),
            "schemaVersion must be 1",
        ));
    }
    let spec_dir = object
        .get("specDir")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            one_issue(
                "REMOVAL_CONFIG_INVALID",
                Some(CONFIG_RELATIVE.to_owned()),
                "specDir must be a string",
            )
        })?
        .to_owned();
    config::validate_spec_dir(&spec_dir)
        .map_err(|error| one_issue(error.code, Some(CONFIG_RELATIVE.to_owned()), error.message))?;
    let agent_values = object
        .get("agents")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            one_issue(
                "REMOVAL_CONFIG_INVALID",
                Some(CONFIG_RELATIVE.to_owned()),
                "agents must be an array",
            )
        })?;
    let mut agents = Vec::new();
    for selected in agent_values {
        let agent = selected.as_str().and_then(Agent::parse).ok_or_else(|| {
            one_issue(
                "REMOVAL_AGENT_UNSUPPORTED",
                Some(CONFIG_RELATIVE.to_owned()),
                "installed configuration names an unsupported agent",
            )
        })?;
        if !agents.contains(&agent) {
            agents.push(agent);
        }
    }
    if agents.is_empty() {
        return Err(one_issue(
            "REMOVAL_CONFIG_INVALID",
            Some(CONFIG_RELATIVE.to_owned()),
            "installed configuration must select at least one agent",
        ));
    }
    let project_instructions = object
        .get("projectInstructions")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    Ok(InstalledConfig {
        bytes,
        value,
        spec_dir,
        agents,
        project_instructions,
    })
}

fn require_commit(project_root: &Path) -> Result<(), RemovalIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("REMOVAL_GIT_FAILED", None, error.to_string()))?;
    if committed {
        Ok(())
    } else {
        Err(one_issue(
            "REMOVAL_COMMIT_REQUIRED",
            None,
            "removal requires at least one Git commit as its recovery boundary",
        ))
    }
}

fn validate_repository_state(
    project_root: &Path,
    entries: &[RemovalEntry],
    removable_tree: Option<&str>,
) -> Result<(), RemovalIssues> {
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("REMOVAL_GIT_FAILED", None, error.to_string()))?;
    let allowed_files = entries
        .iter()
        .filter(|entry| matches!(entry.action, RemovalAction::Remove | RemovalAction::Absent))
        .map(|entry| entry.path.as_str())
        .collect::<Vec<_>>();
    for record in status
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        if record.len() < 4 {
            return Err(one_issue(
                "REMOVAL_REPOSITORY_DIRTY",
                None,
                "Git returned an unreadable status record",
            ));
        }
        let code = &record[..2];
        let path = std::str::from_utf8(&record[3..]).map_err(|_| {
            one_issue(
                "REMOVAL_REPOSITORY_DIRTY",
                None,
                "dirty repository path is not UTF-8",
            )
        })?;
        let within_tree = removable_tree.is_some_and(|root| {
            path == root
                || path
                    .strip_prefix(root)
                    .is_some_and(|rest| rest.starts_with('/'))
        });
        if code == b" D" && (allowed_files.contains(&path) || within_tree) {
            continue;
        }
        let completed_instruction = entries.iter().any(|entry| {
            entry.path == path
                && entry.category == "project-instructions"
                && entry.action == RemovalAction::Absent
        });
        if code == b" M" && completed_instruction {
            continue;
        }
        return Err(one_issue(
            "REMOVAL_REPOSITORY_DIRTY",
            Some(path.to_owned()),
            "repository changes outside an already-removed exact target must be resolved first",
        ));
    }
    Ok(())
}

fn validate_tree(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    validate_path_chain(project_root, relative)?;
    let root = project_root.join(relative);
    let metadata = fs::symlink_metadata(&root).map_err(|error| {
        one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "REMOVAL_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "configured specDir must remain a regular non-link directory",
        ));
    }
    validate_directory(project_root, &root)?;
    Ok(())
}

fn completed_instruction_removal(
    project_root: &Path,
    relative: &str,
    current: &[u8],
) -> Result<bool, RemovalIssues> {
    let revision_path = format!("HEAD:{relative}");
    let Ok(head) = repository::output_bytes(project_root, &["show", &revision_path]) else {
        return Ok(false);
    };
    let head = String::from_utf8(head).map_err(|_| {
        one_issue(
            "REMOVAL_TARGET_NOT_UTF8",
            Some(relative.to_owned()),
            "committed agent instruction file must be UTF-8",
        )
    })?;
    let Some(expected) = project_instructions::remove(&head)
        .map_err(|error| one_issue(error.code, Some(relative.to_owned()), error.message))?
    else {
        return Ok(false);
    };
    Ok(expected.as_bytes() == current)
}

fn validate_directory(project_root: &Path, directory: &Path) -> Result<(), RemovalIssues> {
    for child in fs::read_dir(directory)
        .map_err(|error| one_issue("REMOVAL_TARGET_UNREADABLE", None, error.to_string()))?
    {
        let child = child
            .map_err(|error| one_issue("REMOVAL_TARGET_UNREADABLE", None, error.to_string()))?;
        let path = child.path();
        let relative = path
            .strip_prefix(project_root)
            .expect("walked removal target stays below project root")
            .to_string_lossy()
            .replace('\\', "/");
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            one_issue(
                "REMOVAL_TARGET_UNREADABLE",
                Some(relative.clone()),
                error.to_string(),
            )
        })?;
        if guarded_fs::is_link_like(&metadata) {
            return Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative),
                "knowledge removal rejects links, junctions, and reparse points",
            ));
        }
        validate_not_ignored(project_root, &relative)?;
        if metadata.is_dir() {
            validate_directory(project_root, &path)?;
        } else if metadata.is_file() {
            validate_tracked(project_root, &relative)?;
        } else {
            return Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative),
                "knowledge removal accepts only regular files and directories",
            ));
        }
    }
    Ok(())
}

fn validate_path_chain(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    let mut current = project_root.to_path_buf();
    for component in relative.split('/') {
        current.push(component);
        let metadata = fs::symlink_metadata(&current).map_err(|error| {
            one_issue(
                "REMOVAL_TARGET_UNREADABLE",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })?;
        if guarded_fs::is_link_like(&metadata) {
            return Err(one_issue(
                "REMOVAL_TARGET_UNSAFE",
                Some(relative.to_owned()),
                "removal target traversal rejects links, junctions, and reparse points",
            ));
        }
    }
    Ok(())
}

fn validate_tracked_not_ignored(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    validate_tracked(project_root, relative)?;
    validate_not_ignored(project_root, relative)
}

fn validate_tracked(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    let tracked = repository::predicate(
        project_root,
        &["ls-files", "--error-unmatch", "--", relative],
    )
    .map_err(|error| {
        one_issue(
            "REMOVAL_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if tracked {
        Ok(())
    } else {
        Err(one_issue(
            "REMOVAL_TARGET_UNTRACKED",
            Some(relative.to_owned()),
            "removal deletes only Git-tracked targets",
        ))
    }
}

fn validate_not_ignored(project_root: &Path, relative: &str) -> Result<(), RemovalIssues> {
    let ignored = repository::predicate(
        project_root,
        &["check-ignore", "--no-index", "-q", "--", relative],
    )
    .map_err(|error| {
        one_issue(
            "REMOVAL_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if ignored {
        Err(one_issue(
            "REMOVAL_TARGET_IGNORED",
            Some(relative.to_owned()),
            "ignored targets are not safe removal inputs",
        ))
    } else {
        Ok(())
    }
}

fn revalidate_file(project_root: &Path, entry: &RemovalEntry) -> Result<(), RemovalIssues> {
    validate_path_chain(project_root, &entry.path)?;
    let path = project_root.join(&entry.path);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            "target is no longer a regular non-link file",
        ));
    }
    let current = fs::read(path).map_err(|error| {
        one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            error.to_string(),
        )
    })?;
    if entry.expected_current.as_deref() != Some(current.as_slice()) {
        return Err(one_issue(
            "REMOVAL_TARGET_CHANGED",
            Some(entry.path.clone()),
            "target content changed after planning",
        ));
    }
    Ok(())
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> RemovalIssue {
    RemovalIssue {
        code,
        path,
        message: message.into(),
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> RemovalIssues {
    RemovalIssues {
        issues: vec![issue(code, path, message)],
    }
}
