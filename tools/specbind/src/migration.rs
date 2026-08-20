//! cc-sdd migration inventory, planning, and deterministic application.

use std::{collections::BTreeSet, fmt, fs, path::Path};

use serde::Deserialize;
use serde_json::Value;

use crate::{
    artifacts,
    config::ProjectLanguage,
    install::{self, Agent, InstallInputs, PlanAction},
    migration_resolution, repository,
};

pub const GUIDE_NEUTRAL: &str = "https://huruikagi.github.io/specbind/guide/migration/cc-sdd/";
pub const GUIDE_EN: &str = "https://huruikagi.github.io/specbind/guide/en/migrate-from-cc-sdd/";
pub const GUIDE_JA: &str = "https://huruikagi.github.io/specbind/guide/ja/migrate-from-cc-sdd/";

const LEGACY_CONFIG: &str = ".cc-sdd.json";
const TARGET_CONFIG: &str = ".specbind.json";
const DEFAULT_LEGACY_ROOT: &str = ".kiro";
const TARGET_ROOT: &str = ".specbind";
const LEGACY_SKILLS: &[&str] = &[
    "kiro-debug",
    "kiro-discovery",
    "kiro-impl",
    "kiro-review",
    "kiro-spec-batch",
    "kiro-spec-design",
    "kiro-spec-init",
    "kiro-spec-quick",
    "kiro-spec-requirements",
    "kiro-spec-status",
    "kiro-spec-tasks",
    "kiro-steering",
    "kiro-steering-custom",
    "kiro-validate-design",
    "kiro-validate-gap",
    "kiro-validate-impl",
    "kiro-verify-completion",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationPlan {
    pub legacy_root: String,
    pub target_root: String,
    pub language: Option<ProjectLanguage>,
    pub agents: Vec<String>,
    pub specs: Vec<LegacySpec>,
    pub actions: Vec<MigrationAction>,
    pub findings: Vec<MigrationFinding>,
    pub target_converged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationOutcome {
    pub installed_files: usize,
    pub removed_legacy_assets: usize,
    pub removed_legacy_root: String,
    pub removed_legacy_config: bool,
    pub removed_resolution_state: bool,
}

/// Returns the unresolved source plan before any accepted semantic resolution
/// record is considered. Resolution acceptance uses this to avoid allowing an
/// older record to authorize its own replacement.
pub(crate) fn unresolved_plan(project_root: &Path) -> Result<MigrationPlan, MigrationIssues> {
    plan_inner(project_root)
}

/// Reports a completed cutover when no default cc-sdd source remains and the
/// installed `SpecBind` target is current.
#[must_use]
pub fn source_absent_and_target_current(project_root: &Path) -> bool {
    if path_entry_present(&project_root.join(LEGACY_CONFIG))
        || path_entry_present(&project_root.join(DEFAULT_LEGACY_ROOT))
    {
        return false;
    }
    install::plan(project_root, &InstallInputs::default()).is_ok_and(|plan| {
        !plan.initial
            && plan
                .entries
                .iter()
                .all(|entry| entry.action == PlanAction::Keep)
    })
}

fn path_entry_present(path: &Path) -> bool {
    match fs::symlink_metadata(path) {
        Ok(_) => true,
        Err(error) => error.kind() != std::io::ErrorKind::NotFound,
    }
}

impl MigrationPlan {
    #[must_use]
    pub fn guide_url(&self) -> &'static str {
        match self.language {
            Some(ProjectLanguage::En) => GUIDE_EN,
            Some(ProjectLanguage::Ja) => GUIDE_JA,
            None => GUIDE_NEUTRAL,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacySpec {
    pub id: String,
    pub language: Option<ProjectLanguage>,
    pub phase: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationAction {
    pub kind: &'static str,
    pub source: Option<String>,
    pub target: Option<String>,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MigrationFinding {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationIssues {
    pub issues: Vec<MigrationFinding>,
}

impl fmt::Display for MigrationIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "cc-sdd migration planning failed")
    }
}

impl std::error::Error for MigrationIssues {}

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

/// Inventories one cc-sdd project without modifying it.
///
/// # Errors
///
/// Returns stable diagnostics for unreadable or structurally unsafe source
/// paths. Semantic ambiguity is returned in `MigrationPlan::findings` so the
/// CLI can route it to the agent-assisted guide.
pub fn plan(project_root: &Path) -> Result<MigrationPlan, MigrationIssues> {
    let mut plan = plan_inner(project_root)?;
    migration_resolution::reconcile(project_root, &mut plan);
    Ok(plan)
}

fn plan_inner(project_root: &Path) -> Result<MigrationPlan, MigrationIssues> {
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

fn install_inputs(language: Option<ProjectLanguage>, agents: &[String]) -> Option<InstallInputs> {
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

/// Recomputes and applies only a finding-free deterministic migration plan.
///
/// # Errors
///
/// Returns stable diagnostics when the plan changed, Git is not a safe
/// recovery boundary, installation fails, or a known legacy asset cannot be
/// removed safely.
pub fn apply(project_root: &Path) -> Result<MigrationOutcome, MigrationIssues> {
    let plan = plan(project_root)?;
    if !plan.findings.is_empty() {
        return Err(one_issue(
            "MIGRATION_PLAN_CHANGED",
            None,
            "the freshly recomputed migration plan contains findings",
        ));
    }
    let inputs = install_inputs(plan.language, &plan.agents).ok_or_else(|| {
        one_issue(
            "MIGRATION_PLAN_CHANGED",
            None,
            "the freshly recomputed plan has no complete installation inputs",
        )
    })?;
    let install_plan = install::plan(project_root, &inputs).map_err(render_install_issues)?;
    let legacy_assets = plan
        .actions
        .iter()
        .filter(|action| action.kind == "remove-after-cutover")
        .filter_map(|action| action.source.clone())
        .collect::<Vec<_>>();
    let legacy_config = path_exists(&project_root.join(LEGACY_CONFIG))?;
    let resolution_state = path_exists(&project_root.join(migration_resolution::STATE_RELATIVE))?;
    let mut cleanup_targets = legacy_assets.clone();
    cleanup_targets.push(plan.legacy_root.clone());
    if legacy_config {
        cleanup_targets.push(LEGACY_CONFIG.to_owned());
    }
    if resolution_state {
        cleanup_targets.push(migration_resolution::STATE_RELATIVE.to_owned());
    }
    require_apply_repository(project_root, &cleanup_targets)?;
    let installed_files = install_plan
        .entries
        .iter()
        .filter(|entry| entry.action != PlanAction::Keep)
        .count();
    install::apply(project_root, &inputs).map_err(render_install_issues)?;
    for relative in &legacy_assets {
        remove_legacy_asset(project_root, relative)?;
    }
    remove_cleanup_target(project_root, &plan.legacy_root)?;
    if legacy_config {
        remove_cleanup_target(project_root, LEGACY_CONFIG)?;
    }
    if resolution_state {
        remove_cleanup_target(project_root, migration_resolution::STATE_RELATIVE)?;
    }
    Ok(MigrationOutcome {
        installed_files,
        removed_legacy_assets: legacy_assets.len(),
        removed_legacy_root: plan.legacy_root,
        removed_legacy_config: legacy_config,
        removed_resolution_state: resolution_state,
    })
}

fn render_install_issues(error: install::InstallIssues) -> MigrationIssues {
    MigrationIssues {
        issues: error
            .issues
            .into_iter()
            .map(|issue| finding(issue.code, issue.path, issue.message))
            .collect(),
    }
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
    if is_link_like(&metadata) || !metadata.is_dir() {
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
        if is_link_like(&metadata) || !metadata.is_dir() {
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
        if is_link_like(&metadata) || !metadata.is_dir() {
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
                if is_link_like(&metadata) || !metadata.is_dir() {
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

fn require_apply_repository(
    project_root: &Path,
    cleanup_targets: &[String],
) -> Result<(), MigrationIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| one_issue("MIGRATION_GIT_FAILED", None, error.to_string()))?;
    if !committed {
        return Err(one_issue(
            "MIGRATION_COMMIT_REQUIRED",
            None,
            "cc-sdd migration apply requires at least one commit",
        ));
    }
    for relative in cleanup_targets {
        require_cleanup_target_tracked(project_root, relative)?;
    }
    let status = repository::output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )
    .map_err(|error| one_issue("MIGRATION_GIT_FAILED", None, error.to_string()))?;
    if !status.is_empty() {
        return Err(one_issue(
            "MIGRATION_REPOSITORY_DIRTY",
            None,
            "cc-sdd migration apply requires a clean repository",
        ));
    }
    Ok(())
}

fn require_cleanup_target_tracked(
    project_root: &Path,
    relative: &str,
) -> Result<(), MigrationIssues> {
    let path = project_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if is_link_like(&metadata) {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup targets must not contain links or reparse points",
        ));
    }
    if metadata.is_file() {
        return require_file_tracked(project_root, relative);
    }
    if !metadata.is_dir() {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must be a regular file or directory",
        ));
    }
    for entry in fs::read_dir(&path).map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })? {
        let entry = entry.map_err(|error| {
            one_issue(
                "MIGRATION_CLEANUP_TARGET_CHANGED",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })?;
        let child = entry
            .path()
            .strip_prefix(project_root)
            .expect("cleanup target stays below project root")
            .to_string_lossy()
            .replace('\\', "/");
        require_cleanup_target_tracked(project_root, &child)?;
    }
    Ok(())
}

fn require_file_tracked(project_root: &Path, relative: &str) -> Result<(), MigrationIssues> {
    let tracked = repository::output_bytes(project_root, &["ls-files", "-z", "--", relative])
        .map_err(|error| {
            one_issue(
                "MIGRATION_GIT_FAILED",
                Some(relative.to_owned()),
                error.to_string(),
            )
        })?;
    if tracked.is_empty() {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNTRACKED",
            Some(relative.to_owned()),
            "final cutover deletes only Git-tracked files",
        ));
    }
    Ok(())
}

fn remove_legacy_asset(project_root: &Path, relative: &str) -> Result<(), MigrationIssues> {
    let known = [".agents/skills", ".claude/skills"].iter().any(|root| {
        LEGACY_SKILLS
            .iter()
            .any(|name| relative == format!("{root}/{name}"))
    });
    if !known {
        return Err(one_issue(
            "MIGRATION_LEGACY_ASSET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target is not an exact known skill path",
        ));
    }
    let path = project_root.join(relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_ASSET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "MIGRATION_LEGACY_ASSET_CHANGED",
            Some(relative.to_owned()),
            "legacy cleanup target is no longer a regular non-symlink directory",
        ));
    }
    let status = repository::path_status(project_root, relative).map_err(|error| {
        one_issue(
            "MIGRATION_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if !status.is_empty() {
        return Err(one_issue(
            "MIGRATION_LEGACY_ASSET_DIRTY",
            Some(relative.to_owned()),
            "legacy cleanup target changed after the recovery boundary was established",
        ));
    }
    fs::remove_dir_all(&path).map_err(|error| {
        one_issue(
            "MIGRATION_LEGACY_ASSET_REMOVE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

fn remove_cleanup_target(project_root: &Path, relative: &str) -> Result<(), MigrationIssues> {
    require_cleanup_target_tracked(project_root, relative)?;
    let status = repository::path_status(project_root, relative).map_err(|error| {
        one_issue(
            "MIGRATION_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if !status.is_empty() {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            "legacy cleanup target changed after the recovery boundary was established",
        ));
    }
    let canonical_project = project_root
        .canonicalize()
        .map_err(|error| one_issue("MIGRATION_GIT_FAILED", None, error.to_string()))?;
    let path = project_root.join(relative);
    let canonical = path.canonicalize().map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if canonical == canonical_project || !canonical.starts_with(&canonical_project) {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must stay below the project root",
        ));
    }
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_TARGET_CHANGED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if is_link_like(&metadata) {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must not be a link or reparse point",
        ));
    }
    if metadata.is_dir() {
        fs::remove_dir_all(path)
    } else if metadata.is_file() {
        fs::remove_file(path)
    } else {
        return Err(one_issue(
            "MIGRATION_CLEANUP_TARGET_UNSAFE",
            Some(relative.to_owned()),
            "legacy cleanup target must be a regular file or directory",
        ));
    }
    .map_err(|error| {
        one_issue(
            "MIGRATION_CLEANUP_REMOVE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
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
    if is_link_like(&metadata) || !metadata.is_dir() {
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
    if is_link_like(&metadata) || !metadata.is_dir() {
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
    if is_link_like(&metadata) || !metadata.is_file() {
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

fn path_exists(path: &Path) -> Result<bool, MigrationIssues> {
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

#[cfg(windows)]
fn is_link_like(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt as _;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    metadata.file_type().is_symlink()
        || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn is_link_like(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_symlink()
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> MigrationIssues {
    MigrationIssues {
        issues: vec![finding(code, path, message)],
    }
}

fn finding(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> MigrationFinding {
    MigrationFinding {
        code,
        path,
        message: message.into(),
    }
}
