use super::{
    Agent, BTreeSet, CONFIG_RELATIVE, InstalledConfig, Mutation, Path, RemovalAction, RemovalEntry,
    RemovalIssues, Value, agent_role, completed_instruction_removal, fs, guarded_fs, one_issue,
    project_instructions, skill, validate_path_chain, validate_tracked_not_ignored, validate_tree,
};

pub(super) fn agent_entries(
    project_root: &Path,
    removed: &[Agent],
    remaining: &[Agent],
    project_instructions_enabled: bool,
) -> Result<Vec<RemovalEntry>, RemovalIssues> {
    let mut entries = Vec::new();
    let mut planned = BTreeSet::new();
    let remaining_skill_targets = remaining
        .iter()
        .flat_map(|agent| {
            skill::all()
                .iter()
                .flat_map(move |embedded| embedded.targets(*agent))
        })
        .collect::<BTreeSet<_>>();
    for agent in removed {
        for embedded in skill::all() {
            for path in embedded.targets(*agent) {
                if !planned.insert(path.clone()) {
                    continue;
                }
                if remaining_skill_targets.contains(&path) {
                    entries.push(retained_entry(
                        project_root,
                        &path,
                        "skill",
                        "remaining selected agent requires this shared skill target",
                    )?);
                } else {
                    entries.push(file_entry(
                        project_root,
                        &path,
                        "skill",
                        "exact product-managed skill target",
                    )?);
                }
            }
        }
        for role in agent_role::all() {
            let Some(path) = role_target(*agent, role) else {
                continue;
            };
            if !planned.insert(path.clone()) {
                continue;
            }
            entries.push(file_entry(
                project_root,
                &path,
                "agent-role",
                "exact product-managed agent-role target",
            )?);
        }
        for root in [Some(skill_root(*agent)), role_root(*agent)]
            .into_iter()
            .flatten()
        {
            if !planned.insert(root.to_owned()) {
                continue;
            }
            if remaining
                .iter()
                .any(|selected| skill_root(*selected) == root || role_root(*selected) == Some(root))
            {
                entries.push(retained_entry(
                    project_root,
                    root,
                    "container",
                    "remaining selected agent requires this shared container",
                )?);
            } else {
                entries.push(container_entry(project_root, root)?);
            }
        }
        let instruction_path = project_instructions::target(*agent);
        if planned.insert(instruction_path.to_owned()) {
            if project_instructions_enabled
                && remaining
                    .iter()
                    .any(|selected| project_instructions::target(*selected) == instruction_path)
            {
                entries.push(retained_entry(
                    project_root,
                    instruction_path,
                    "project-instructions",
                    "remaining selected agent requires this shared instruction block",
                )?);
            } else {
                entries.push(instruction_entry(
                    project_root,
                    instruction_path,
                    project_instructions_enabled,
                )?);
            }
        }
    }
    Ok(entries)
}

fn role_target(agent: Agent, role: &agent_role::AgentRole) -> Option<String> {
    match agent {
        Agent::ClaudeCode => Some(role.claude_target()),
        Agent::Codex => Some(role.target()),
        Agent::Generic => None,
    }
}

fn skill_root(agent: Agent) -> &'static str {
    match agent {
        Agent::ClaudeCode => ".claude/skills",
        Agent::Codex | Agent::Generic => ".agents/skills",
    }
}

fn role_root(agent: Agent) -> Option<&'static str> {
    match agent {
        Agent::ClaudeCode => Some(".claude/agents"),
        Agent::Codex => Some(".codex/agents"),
        Agent::Generic => None,
    }
}

fn retained_entry(
    project_root: &Path,
    relative: &str,
    category: &'static str,
    detail: &str,
) -> Result<RemovalEntry, RemovalIssues> {
    match fs::symlink_metadata(project_root.join(relative)) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(RemovalEntry {
            action: RemovalAction::Absent,
            path: relative.to_owned(),
            category,
            detail: "shared target is already absent".to_owned(),
            mutation: Mutation::None,
            expected_current: None,
        }),
        Err(error) => Err(one_issue(
            "REMOVAL_TARGET_UNREADABLE",
            Some(relative.to_owned()),
            error.to_string(),
        )),
        Ok(_) => Ok(RemovalEntry {
            action: RemovalAction::Retain,
            path: relative.to_owned(),
            category,
            detail: detail.to_owned(),
            mutation: Mutation::None,
            expected_current: None,
        }),
    }
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

pub(super) fn file_entry(
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

pub(super) fn tree_entry(
    project_root: &Path,
    relative: &str,
) -> Result<RemovalEntry, RemovalIssues> {
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

pub(super) fn config_update_entry(
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

pub(super) fn update_config_value(
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
    let role_key = match removed {
        Agent::Codex => Some("codex"),
        Agent::ClaudeCode => Some("claudeCode"),
        Agent::Generic => None,
    };
    if let Some(role_key) = role_key
        && let Some(roles) = object.get_mut("agentRoles").and_then(Value::as_object_mut)
    {
        roles.remove(role_key);
        if roles.is_empty() {
            object.remove("agentRoles");
        }
    }
    Ok(())
}
