//! Read-only cc-sdd source inventory and migration planning.

use std::{collections::BTreeSet, fs, path::Path};

use serde::Deserialize;
use serde_json::Value;

use super::{
    DEFAULT_LEGACY_ROOT, LEGACY_CONFIG, LEGACY_SKILLS, LegacySpec, MigrationAction,
    MigrationFinding, MigrationIssues, MigrationPlan, TARGET_CONFIG, TARGET_ROOT, finding,
    one_issue,
};
use crate::{
    artifacts,
    config::ProjectLanguage,
    guarded_fs,
    install::{self, Agent, InstallInputs, PlanAction},
};

#[derive(Debug, Deserialize)]
struct LegacySpecMetadata {
    language: String,
    phase: String,
    approvals: LegacyApprovals,
    #[serde(rename = "ready_for_implementation")]
    _ready_for_implementation: bool,
}

#[derive(Debug, Deserialize)]
struct LegacyApprovals {
    requirements: LegacyApproval,
    design: LegacyApproval,
    tasks: LegacyApproval,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct LegacyApproval {
    generated: bool,
    approved: bool,
}

struct LegacyConfigValues {
    root: String,
    language: Option<ProjectLanguage>,
    agents: BTreeSet<String>,
    findings: Vec<MigrationFinding>,
}

pub(super) fn plan_inner(project_root: &Path) -> Result<MigrationPlan, MigrationIssues> {
    let config = read_legacy_config(project_root)?;
    let legacy_root_path = project_root.join(&config.root);
    require_directory(
        &legacy_root_path,
        &config.root,
        "MIGRATION_LEGACY_ROOT_NOT_FOUND",
    )?;

    let mut findings = config.findings;
    if config.root == TARGET_ROOT {
        findings.push(finding(
            "MIGRATE_TARGET_ALREADY_EXISTS",
            Some(TARGET_ROOT.to_owned()),
            "the legacy root is also the default SpecBind target root",
        ));
    }

    let mut actions = vec![MigrationAction {
        kind: "retire-source-after-cutover",
        source: Some(config.root.clone()),
        target: None,
        detail: "remove the Git-tracked cc-sdd source only during final apply".to_owned(),
    }];
    let mut languages = BTreeSet::new();
    if let Some(language) = config.language {
        languages.insert(language_name(language).to_owned());
    }
    let mut specs = inspect_specs(
        &legacy_root_path,
        &config.root,
        &mut languages,
        &mut actions,
        &mut findings,
    )?;
    specs.sort_by(|left, right| left.id.cmp(&right.id));
    inspect_legacy_content(&legacy_root_path, &config.root, &mut actions, &mut findings)?;
    inspect_project_instructions(project_root, &mut actions, &mut findings)?;

    inspect_scope_findings(&specs, &config.root, &languages, &mut findings);

    let mut agents = config.agents;
    inspect_agent_assets(project_root, &mut agents, &mut actions, &mut findings)?;
    let agents = agents.into_iter().collect::<Vec<_>>();
    if agents.is_empty() {
        findings.push(finding(
            "MIGRATE_AGENT_SELECTION_REQUIRED",
            None,
            "no supported legacy agent selection can be established",
        ));
    }

    let target_converged = if findings.iter().any(|finding| {
        matches!(
            finding.code,
            "MIGRATE_AGENT_SELECTION_REQUIRED"
                | "MIGRATE_AGENT_UNSUPPORTED"
                | "MIGRATE_LANGUAGE_MIXED"
                | "MIGRATE_LANGUAGE_SELECTION_REQUIRED"
                | "MIGRATE_LANGUAGE_UNSUPPORTED"
        )
    }) {
        false
    } else {
        inspect_target_state(
            project_root,
            one_language(&languages),
            &agents,
            &mut findings,
        )?
    };
    insert_config_actions(project_root, target_converged, &mut actions);

    findings.sort();
    findings.dedup();
    Ok(MigrationPlan {
        legacy_root: config.root,
        target_root: TARGET_ROOT.to_owned(),
        language: one_language(&languages),
        agents,
        specs,
        actions,
        findings,
        target_converged,
    })
}

fn insert_config_actions(
    project_root: &Path,
    target_converged: bool,
    actions: &mut Vec<MigrationAction>,
) {
    let legacy_config = project_root.join(LEGACY_CONFIG).exists();
    actions.insert(
        1,
        MigrationAction {
            kind: if target_converged {
                "keep"
            } else if legacy_config {
                "convert"
            } else {
                "create"
            },
            source: legacy_config.then(|| LEGACY_CONFIG.to_owned()),
            target: Some(TARGET_CONFIG.to_owned()),
            detail: "establish the strict SpecBind project configuration before retiring the legacy config"
                .to_owned(),
        },
    );
    if legacy_config {
        actions.insert(
            2,
            MigrationAction {
                kind: "retire-config-after-cutover",
                source: Some(LEGACY_CONFIG.to_owned()),
                target: None,
                detail: "remove the Git-tracked cc-sdd config during final apply".to_owned(),
            },
        );
    }
}

fn inspect_scope_findings(
    specs: &[LegacySpec],
    legacy_root: &str,
    languages: &BTreeSet<String>,
    findings: &mut Vec<MigrationFinding>,
) {
    let specs_path = Some(format!("{legacy_root}/specs"));
    if specs.len() > 1 {
        findings.push(finding(
            "MIGRATE_ACTIVE_SCOPE_AMBIGUOUS",
            specs_path.clone(),
            "multiple legacy Specs require a user-confirmed active milestone or baseline disposition",
        ));
    }
    if !specs.is_empty() {
        findings.push(finding(
            "MIGRATE_SPEC_CONVERSION_REQUIRED",
            specs_path.clone(),
            "legacy Specs require guided lifecycle conversion before automatic apply",
        ));
    }
    if languages.len() > 1 {
        findings.push(finding(
            "MIGRATE_LANGUAGE_MIXED",
            specs_path,
            "legacy configuration and Spec metadata do not establish one project-global artifact language",
        ));
    } else if languages.is_empty() {
        findings.push(finding(
            "MIGRATE_LANGUAGE_SELECTION_REQUIRED",
            None,
            "no supported project-global artifact language can be established",
        ));
    }
}

fn inspect_target_state(
    project_root: &Path,
    language: Option<ProjectLanguage>,
    agents: &[String],
    findings: &mut Vec<MigrationFinding>,
) -> Result<bool, MigrationIssues> {
    let config_exists = path_exists(&project_root.join(TARGET_CONFIG))?;
    let root_exists = path_exists(&project_root.join(TARGET_ROOT))?;
    if !config_exists && !root_exists {
        return Ok(false);
    }
    let Some(inputs) = install_inputs(language, agents) else {
        return Ok(false);
    };
    let converged = config_exists
        && root_exists
        && install::plan(project_root, &inputs).is_ok_and(|plan| {
            plan.entries
                .iter()
                .all(|entry| entry.action == PlanAction::Keep)
        });
    if !converged {
        findings.push(finding(
            "MIGRATE_TARGET_ALREADY_EXISTS",
            Some(TARGET_CONFIG.to_owned()),
            "the existing SpecBind target is not the exact converged migration target",
        ));
    }
    Ok(converged)
}

pub(super) fn install_inputs(
    language: Option<ProjectLanguage>,
    agents: &[String],
) -> Option<InstallInputs> {
    let language = language?;
    let agents = agents
        .iter()
        .map(|agent| Agent::parse(agent))
        .collect::<Option<Vec<_>>>()?;
    Some(InstallInputs {
        agents,
        language: Some(language),
        spec_dir: Some(TARGET_ROOT.to_owned()),
        project_instructions: Some(false),
    })
}

fn read_legacy_config(project_root: &Path) -> Result<LegacyConfigValues, MigrationIssues> {
    let path = project_root.join(LEGACY_CONFIG);
    let Some(input) = read_optional_regular(&path, LEGACY_CONFIG)? else {
        return Ok(LegacyConfigValues {
            root: DEFAULT_LEGACY_ROOT.to_owned(),
            language: None,
            agents: BTreeSet::new(),
            findings: vec![],
        });
    };
    let value = serde_json::from_str::<Value>(&input).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_CONFIG_INVALID",
            Some(LEGACY_CONFIG.to_owned()),
            format!("legacy configuration is invalid JSON: {error}"),
        )
    })?;
    let object = value.as_object().ok_or_else(|| {
        one_issue(
            "MIGRATION_LEGACY_CONFIG_INVALID",
            Some(LEGACY_CONFIG.to_owned()),
            "legacy configuration must be a JSON object",
        )
    })?;
    let root = normalize_legacy_root(
        optional_string(object.get("kiroDir"), "kiroDir")?.unwrap_or(DEFAULT_LEGACY_ROOT),
    )?;
    let mut findings = Vec::new();
    let language = optional_string(object.get("lang"), "lang")?.and_then(|value| {
        match parse_language(value) {
            Ok(language) => Some(language),
            Err(error) => {
                findings.extend(error.issues.into_iter().map(|mut issue| {
                    issue.path = Some(LEGACY_CONFIG.to_owned());
                    issue
                }));
                None
            }
        }
    });
    let mut agents = BTreeSet::new();
    if let Some(agent) = optional_string(object.get("agent"), "agent")? {
        match agent {
            "codex-skills" => {
                agents.insert("codex".to_owned());
            }
            "claude-code-skills" => {
                agents.insert("claude-code".to_owned());
            }
            value => {
                findings.push(finding(
                    "MIGRATE_AGENT_UNSUPPORTED",
                    Some(LEGACY_CONFIG.to_owned()),
                    format!("legacy agent is not supported by SpecBind v1: {value}"),
                ));
            }
        }
    }
    Ok(LegacyConfigValues {
        root,
        language,
        agents,
        findings,
    })
}

fn inspect_specs(
    legacy_root: &Path,
    legacy_root_relative: &str,
    languages: &mut BTreeSet<String>,
    actions: &mut Vec<MigrationAction>,
    findings: &mut Vec<MigrationFinding>,
) -> Result<Vec<LegacySpec>, MigrationIssues> {
    let specs_root = legacy_root.join("specs");
    let specs_relative = format!("{legacy_root_relative}/specs");
    let metadata = match fs::symlink_metadata(&specs_root) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(error) => {
            return Err(one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(specs_relative),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "MIGRATION_LEGACY_SOURCE_INVALID",
            Some(specs_relative),
            "legacy specs path must be a regular non-symlink directory",
        ));
    }

    let mut directories = Vec::new();
    for entry in fs::read_dir(&specs_root).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_SOURCE_UNREADABLE",
            Some(format!("{legacy_root_relative}/specs")),
            error.to_string(),
        )
    })? {
        let entry = entry.map_err(|error| {
            one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(format!("{legacy_root_relative}/specs")),
                error.to_string(),
            )
        })?;
        let name = entry.file_name().into_string().map_err(|_| {
            one_issue(
                "MIGRATION_LEGACY_PATH_NOT_UTF8",
                Some(format!("{legacy_root_relative}/specs")),
                "legacy Spec directory name is not UTF-8",
            )
        })?;
        let metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
            one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(format!("{legacy_root_relative}/specs/{name}")),
                error.to_string(),
            )
        })?;
        if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
            findings.push(finding(
                "MIGRATE_SPEC_DIRECTORY_INVALID",
                Some(format!("{legacy_root_relative}/specs/{name}")),
                "legacy Spec entry is not a regular non-symlink directory",
            ));
            continue;
        }
        directories.push((name, entry.path()));
    }
    directories.sort_by(|left, right| left.0.cmp(&right.0));

    let mut specs = Vec::new();
    for (id, directory) in directories {
        let relative = format!("{legacy_root_relative}/specs/{id}");
        if !artifacts::canonical_id(&id) {
            findings.push(finding(
                "MIGRATE_SPEC_ID_INVALID",
                Some(relative),
                "legacy Spec directory is not a canonical kebab-case Spec ID",
            ));
            continue;
        }
        specs.push(inspect_spec(
            &id,
            &directory,
            legacy_root_relative,
            languages,
            actions,
            findings,
        )?);
    }
    Ok(specs)
}

fn inspect_spec(
    id: &str,
    directory: &Path,
    legacy_root: &str,
    languages: &mut BTreeSet<String>,
    actions: &mut Vec<MigrationAction>,
    findings: &mut Vec<MigrationFinding>,
) -> Result<LegacySpec, MigrationIssues> {
    let source_root = format!("{legacy_root}/specs/{id}");
    let metadata_path = directory.join("spec.json");
    let metadata_relative = format!("{source_root}/spec.json");
    let metadata = read_legacy_spec_metadata(&metadata_path, &metadata_relative, findings)?;
    let mut language = None;
    let mut phase = None;
    if let Some(metadata) = metadata.as_ref() {
        match parse_language(&metadata.language) {
            Ok(value) => {
                languages.insert(language_name(value).to_owned());
                language = Some(value);
            }
            Err(error) => findings.extend(error.issues.into_iter().map(|mut issue| {
                issue.path = Some(metadata_relative.clone());
                issue
            })),
        }
        if let Err(message) = validate_legacy_state(metadata) {
            findings.push(finding(
                "MIGRATE_SPEC_STATE_INVALID",
                Some(metadata_relative.clone()),
                message,
            ));
        }
        phase = Some(metadata.phase.clone());
    }

    plan_spec_artifacts(
        id,
        directory,
        &source_root,
        metadata_relative,
        metadata.as_ref(),
        actions,
        findings,
    )?;
    Ok(LegacySpec {
        id: id.to_owned(),
        language,
        phase,
    })
}

fn read_legacy_spec_metadata(
    path: &Path,
    relative: &str,
    findings: &mut Vec<MigrationFinding>,
) -> Result<Option<LegacySpecMetadata>, MigrationIssues> {
    let Some(input) = read_optional_regular(path, relative)? else {
        findings.push(finding(
            "MIGRATE_SPEC_METADATA_MISSING",
            Some(relative.to_owned()),
            "legacy Spec has no spec.json metadata",
        ));
        return Ok(None);
    };
    match serde_json::from_str::<LegacySpecMetadata>(&input) {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) => {
            findings.push(finding(
                "MIGRATE_SPEC_STATE_INVALID",
                Some(relative.to_owned()),
                format!("legacy Spec metadata is incomplete or invalid: {error}"),
            ));
            Ok(None)
        }
    }
}

fn plan_spec_artifacts(
    id: &str,
    directory: &Path,
    source_root: &str,
    metadata_relative: String,
    metadata: Option<&LegacySpecMetadata>,
    actions: &mut Vec<MigrationAction>,
    findings: &mut Vec<MigrationFinding>,
) -> Result<(), MigrationIssues> {
    actions.push(MigrationAction {
        kind: "convert",
        source: Some(metadata_relative),
        target: Some(format!("{TARGET_ROOT}/specs/{id}/spec.yaml")),
        detail: "convert legacy lifecycle metadata without inventing gate evidence".to_owned(),
    });
    for (source_name, target_name, detail) in [
        (
            "requirements.md",
            "requirements.md",
            "add and validate the SpecBind Requirements profile",
        ),
        (
            "design.md",
            "design.md",
            "add and validate the SpecBind Design profile",
        ),
        (
            "tasks.md",
            "tasks.yaml",
            "convert supported task grammar and provable execution state",
        ),
        (
            "brief.md",
            "brief.md",
            "preserve the active brief with its SpecBind profile",
        ),
        (
            "research.md",
            "research.md",
            "preserve active research with its SpecBind profile",
        ),
    ] {
        let source = directory.join(source_name);
        let exists =
            read_optional_regular(&source, &format!("{source_root}/{source_name}"))?.is_some();
        if let Some(generated) = generated_flag(metadata, source_name)
            && generated != exists
        {
            findings.push(finding(
                "MIGRATE_SPEC_STATE_INVALID",
                Some(format!("{source_root}/{source_name}")),
                "legacy generated metadata does not match artifact presence",
            ));
        }
        if exists {
            actions.push(MigrationAction {
                kind: "convert",
                source: Some(format!("{source_root}/{source_name}")),
                target: Some(format!("{TARGET_ROOT}/specs/{id}/{target_name}")),
                detail: detail.to_owned(),
            });
            if source_name == "design.md" {
                findings.push(finding(
                    "MIGRATE_DESIGN_TRACEABILITY_REQUIRED",
                    Some(format!("{source_root}/design.md")),
                    "legacy Design requires semantic Requirement traceability and Contract authoring",
                ));
            }
        }
    }
    Ok(())
}

fn generated_flag(metadata: Option<&LegacySpecMetadata>, source_name: &str) -> Option<bool> {
    let approvals = &metadata?.approvals;
    match source_name {
        "requirements.md" => Some(approvals.requirements.generated),
        "design.md" => Some(approvals.design.generated),
        "tasks.md" => Some(approvals.tasks.generated),
        _ => None,
    }
}

fn inspect_agent_assets(
    project_root: &Path,
    agents: &mut BTreeSet<String>,
    actions: &mut Vec<MigrationAction>,
    findings: &mut Vec<MigrationFinding>,
) -> Result<(), MigrationIssues> {
    for (agent, root) in [
        ("codex", ".agents/skills"),
        ("claude-code", ".claude/skills"),
    ] {
        let path = project_root.join(root);
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(one_issue(
                    "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                    Some(root.to_owned()),
                    error.to_string(),
                ));
            }
        };
        if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
            findings.push(finding(
                "MIGRATE_LEGACY_AGENT_ASSET_INVALID",
                Some(root.to_owned()),
                "legacy agent asset root is not a regular non-symlink directory",
            ));
            continue;
        }
        let mut assets = Vec::new();
        for entry in fs::read_dir(&path).map_err(|error| {
            one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(root.to_owned()),
                error.to_string(),
            )
        })? {
            let entry = entry.map_err(|error| {
                one_issue(
                    "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                    Some(root.to_owned()),
                    error.to_string(),
                )
            })?;
            let name = entry.file_name().into_string().map_err(|_| {
                one_issue(
                    "MIGRATION_LEGACY_PATH_NOT_UTF8",
                    Some(root.to_owned()),
                    "legacy agent asset name is not UTF-8",
                )
            })?;
            if LEGACY_SKILLS.contains(&name.as_str()) {
                let metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
                    one_issue(
                        "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                        Some(format!("{root}/{name}")),
                        error.to_string(),
                    )
                })?;
                if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
                    findings.push(finding(
                        "MIGRATE_LEGACY_AGENT_ASSET_INVALID",
                        Some(format!("{root}/{name}")),
                        "known legacy skill path is not a regular non-symlink directory",
                    ));
                } else {
                    assets.push(name);
                }
            } else if name.starts_with("kiro-") {
                findings.push(finding(
                    "MIGRATE_LEGACY_AGENT_ASSET_UNKNOWN",
                    Some(format!("{root}/{name}")),
                    "legacy kiro-prefixed agent asset is not an exact known cc-sdd skill",
                ));
            }
        }
        assets.sort();
        if !assets.is_empty() {
            agents.insert(agent.to_owned());
        }
        for asset in assets {
            actions.push(MigrationAction {
                kind: "remove-after-cutover",
                source: Some(format!("{root}/{asset}")),
                target: None,
                detail: "remove only after target validation and user-confirmed cutover".to_owned(),
            });
        }
    }
    Ok(())
}

fn inspect_legacy_content(
    legacy_root: &Path,
    legacy_root_relative: &str,
    actions: &mut Vec<MigrationAction>,
    findings: &mut Vec<MigrationFinding>,
) -> Result<(), MigrationIssues> {
    for (relative, code, detail) in [
        (
            "settings/rules",
            "MIGRATE_RULE_REVIEW_REQUIRED",
            "legacy rules require comparison with SpecBind defaults and preservation of project-owned intent",
        ),
        (
            "settings/templates",
            "MIGRATE_TEMPLATE_REVIEW_REQUIRED",
            "legacy templates require semantic conversion to supported project-owned overrides",
        ),
        (
            "steering",
            "MIGRATE_STEERING_REVIEW_REQUIRED",
            "legacy steering documents require explicit SpecBind artifact identities",
        ),
    ] {
        let path = legacy_root.join(relative);
        let display = format!("{legacy_root_relative}/{relative}");
        if directory_has_entries(&path, &display)? {
            actions.push(MigrationAction {
                kind: "review",
                source: Some(display.clone()),
                target: Some(format!("{TARGET_ROOT}/{relative}")),
                detail: detail.to_owned(),
            });
            findings.push(finding(code, Some(display), detail));
        }
    }

    for entry in fs::read_dir(legacy_root).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_SOURCE_UNREADABLE",
            Some(legacy_root_relative.to_owned()),
            error.to_string(),
        )
    })? {
        let entry = entry.map_err(|error| {
            one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(legacy_root_relative.to_owned()),
                error.to_string(),
            )
        })?;
        let name = entry.file_name().into_string().map_err(|_| {
            one_issue(
                "MIGRATION_LEGACY_PATH_NOT_UTF8",
                Some(legacy_root_relative.to_owned()),
                "legacy root entry name is not UTF-8",
            )
        })?;
        if matches!(name.as_str(), "specs" | "settings" | "steering") {
            continue;
        }
        findings.push(finding(
            "MIGRATE_LEGACY_CONTENT_UNSUPPORTED",
            Some(format!("{legacy_root_relative}/{name}")),
            "legacy root content has no deterministic SpecBind conversion",
        ));
    }
    Ok(())
}

fn inspect_project_instructions(
    project_root: &Path,
    actions: &mut Vec<MigrationAction>,
    findings: &mut Vec<MigrationFinding>,
) -> Result<(), MigrationIssues> {
    for relative in ["AGENTS.md", "CLAUDE.md"] {
        let Some(input) = read_optional_regular(&project_root.join(relative), relative)? else {
            continue;
        };
        if input.contains("kiro-") || input.contains("cc-sdd") {
            actions.push(MigrationAction {
                kind: "review",
                source: Some(relative.to_owned()),
                target: Some(relative.to_owned()),
                detail:
                    "replace only an exact known legacy instruction block after target validation"
                        .to_owned(),
            });
            findings.push(finding(
                "MIGRATE_LEGACY_INSTRUCTIONS_AMBIGUOUS",
                Some(relative.to_owned()),
                "legacy workflow instructions require an exact-block or semantic review",
            ));
        }
    }
    Ok(())
}

fn directory_has_entries(path: &Path, relative: &str) -> Result<bool, MigrationIssues> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(relative.to_owned()),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "MIGRATION_LEGACY_SOURCE_INVALID",
            Some(relative.to_owned()),
            "legacy content root must be a regular non-symlink directory",
        ));
    }
    fs::read_dir(path)
        .map_err(|error| {
            one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })?
        .next()
        .transpose()
        .map(|entry| entry.is_some())
        .map_err(|error| {
            one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })
}

fn validate_legacy_state(metadata: &LegacySpecMetadata) -> Result<(), String> {
    let requirements = metadata.approvals.requirements;
    let design = metadata.approvals.design;
    let tasks = metadata.approvals.tasks;
    if requirements.approved && !requirements.generated
        || design.approved && !design.generated
        || tasks.approved && !tasks.generated
    {
        return Err("an approval cannot be true when its artifact is not generated".to_owned());
    }
    if design.generated && !requirements.generated || tasks.generated && !design.generated {
        return Err("generated phases must form a requirements-design-tasks prefix".to_owned());
    }
    if design.generated && !requirements.approved
        || tasks.generated && (!requirements.approved || !design.approved)
    {
        return Err("generated phases require the preceding legacy approvals".to_owned());
    }
    let expected = match metadata.phase.as_str() {
        "initialized" => !requirements.generated && !design.generated && !tasks.generated,
        "requirements-generated" => requirements.generated && !design.generated && !tasks.generated,
        "design-generated" => requirements.generated && design.generated && !tasks.generated,
        "tasks-generated" => requirements.generated && design.generated && tasks.generated,
        _ => return Err(format!("legacy phase is unsupported: {}", metadata.phase)),
    };
    if !expected {
        return Err("phase does not match the complete generated-artifact state".to_owned());
    }
    Ok(())
}

fn optional_string<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<Option<&'a str>, MigrationIssues> {
    value
        .map(|value| {
            value.as_str().ok_or_else(|| {
                one_issue(
                    "MIGRATION_LEGACY_CONFIG_INVALID",
                    Some(LEGACY_CONFIG.to_owned()),
                    format!("legacy {field} must be a string"),
                )
            })
        })
        .transpose()
}

fn parse_language(value: &str) -> Result<ProjectLanguage, MigrationIssues> {
    match value {
        "en" => Ok(ProjectLanguage::En),
        "ja" => Ok(ProjectLanguage::Ja),
        _ => Err(one_issue(
            "MIGRATE_LANGUAGE_UNSUPPORTED",
            None,
            format!("legacy artifact language is unsupported: {value}"),
        )),
    }
}

fn language_name(language: ProjectLanguage) -> &'static str {
    match language {
        ProjectLanguage::En => "en",
        ProjectLanguage::Ja => "ja",
    }
}

fn one_language(languages: &BTreeSet<String>) -> Option<ProjectLanguage> {
    if languages.len() != 1 {
        return None;
    }
    match languages.first().map(String::as_str) {
        Some("en") => Some(ProjectLanguage::En),
        Some("ja") => Some(ProjectLanguage::Ja),
        _ => None,
    }
}

fn normalize_legacy_root(value: &str) -> Result<String, MigrationIssues> {
    let allowed = value
        .chars()
        .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-' | '/'));
    let trimmed = value.trim_end_matches('/');
    let segments = trimmed.split('/').filter(|segment| !segment.is_empty());
    let invalid = trimmed.is_empty()
        || !allowed
        || Path::new(value).is_absolute()
        || segments
            .clone()
            .any(|segment| segment == "." || segment == "..")
        || segments
            .clone()
            .next()
            .is_some_and(|segment| segment.eq_ignore_ascii_case(".git"));
    if invalid {
        Err(one_issue(
            "MIGRATION_LEGACY_CONFIG_INVALID",
            Some(LEGACY_CONFIG.to_owned()),
            "legacy kiroDir does not match the original cc-sdd relative-path grammar",
        ))
    } else {
        let mut normalized = String::with_capacity(trimmed.len());
        for segment in trimmed.split('/').filter(|segment| !segment.is_empty()) {
            if !normalized.is_empty() {
                normalized.push('/');
            }
            normalized.push_str(segment);
        }
        Ok(normalized)
    }
}

fn require_directory(
    path: &Path,
    relative: &str,
    missing_code: &'static str,
) -> Result<(), MigrationIssues> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        one_issue(
            if error.kind() == std::io::ErrorKind::NotFound {
                missing_code
            } else {
                "MIGRATION_LEGACY_SOURCE_UNREADABLE"
            },
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        Err(one_issue(
            "MIGRATION_LEGACY_SOURCE_INVALID",
            Some(relative.to_owned()),
            "legacy root must be a regular non-symlink directory",
        ))
    } else {
        Ok(())
    }
}

fn read_optional_regular(path: &Path, relative: &str) -> Result<Option<String>, MigrationIssues> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(one_issue(
                "MIGRATION_LEGACY_SOURCE_UNREADABLE",
                Some(relative.to_owned()),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "MIGRATION_LEGACY_SOURCE_INVALID",
            Some(relative.to_owned()),
            "legacy source must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(path).map(Some).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_SOURCE_UNREADABLE",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

pub(super) fn path_exists(path: &Path) -> Result<bool, MigrationIssues> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(one_issue(
            "MIGRATION_TARGET_UNREADABLE",
            path.file_name()
                .and_then(|value| value.to_str())
                .map(ToOwned::to_owned),
            error.to_string(),
        )),
    }
}
