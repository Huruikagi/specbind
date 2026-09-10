use super::{
    Agent, CONFIG_RELATIVE, InstalledConfig, Path, RemovalIssues, Value, config, fs, guarded_fs,
    one_issue,
};

pub(super) fn read_config(project_root: &Path) -> Result<Option<InstalledConfig>, RemovalIssues> {
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
