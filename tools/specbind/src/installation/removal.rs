//! Guarded removal of one agent integration or the complete project integration.

mod apply;
mod config_read;
mod guard;
mod plan;

use apply::apply_plan;
use config_read::read_config;
use guard::{
    completed_instruction_removal, require_commit, revalidate_file, validate_path_chain,
    validate_repository_state, validate_tracked_not_ignored, validate_tree,
};
use plan::{agent_entries, config_update_entry, file_entry, tree_entry, update_config_value};

use std::{collections::BTreeSet, fmt, fs, path::Path};

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
    installed.agents.retain(|selected| *selected != agent);
    let mut entries = agent_entries(
        project_root,
        &[agent],
        &installed.agents,
        installed.project_instructions,
    )?;
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
    let mut entries = agent_entries(
        project_root,
        &installed.agents,
        &[],
        installed.project_instructions,
    )?;
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
