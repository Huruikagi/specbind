//! Guarded acceptance and freshness checks for agent-assisted cc-sdd migration.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    config::ProjectLanguage,
    guarded_fs::{self, GuardedWriteError},
    install::{self, Agent, InstallInputs, PlanAction},
    migration::{self, MigrationFinding, MigrationIssues, MigrationPlan},
    repository, yaml,
};

const STATE_RELATIVE: &str = ".specbind/state/cc-sdd-migration.yaml";
const STATE_PARENT: &str = ".specbind/state";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ResolutionCandidate {
    schema_version: u64,
    assessment: String,
    target: TargetSelection,
    resolutions: Vec<CandidateResolution>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct TargetSelection {
    language: ProjectLanguage,
    agents: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CandidateResolution {
    code: String,
    #[serde(default)]
    path: Option<String>,
    disposition: Disposition,
    #[serde(default)]
    targets: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Disposition {
    Converted,
    NotMigrated,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredResolution {
    code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    disposition: Disposition,
    source_fingerprint: String,
    target_fingerprints: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct StoredRecord {
    schema_version: u64,
    accepted_at: String,
    legacy_root: String,
    target: TargetSelection,
    assessment: String,
    resolutions: Vec<StoredResolution>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedMigrationResolution {
    pub path: String,
    pub accepted_at: String,
    pub resolutions: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedCandidate {
    legacy_root: String,
    target: TargetSelection,
    assessment: String,
    resolutions: Vec<StoredResolution>,
}

/// Accepts a strict transient candidate after re-resolving every current
/// finding and fingerprint, then atomically persists CLI-owned state.
///
/// # Errors
///
/// Returns deterministic candidate, target, Git, race, or write diagnostics.
pub fn accept(
    project_root: &Path,
    candidate_json: &str,
) -> Result<AcceptedMigrationResolution, MigrationIssues> {
    let candidate = parse_candidate(candidate_json)?;
    let initial = resolve_candidate(project_root, &candidate)?;
    ensure_clean_repository(project_root)?;
    let current = resolve_candidate(project_root, &candidate)?;
    ensure_clean_repository(project_root)?;
    if initial != current {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_INPUTS_CHANGED",
            None,
            "migration resolution inputs changed during guarded acceptance",
        ));
    }
    let accepted_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| {
            one_issue(
                "MIGRATION_RESOLUTION_TIMESTAMP_FAILED",
                None,
                error.to_string(),
            )
        })?;
    let record = StoredRecord {
        schema_version: 1,
        accepted_at: accepted_at.clone(),
        legacy_root: current.legacy_root,
        target: current.target,
        assessment: current.assessment,
        resolutions: current.resolutions,
    };
    persist(project_root, &record)?;
    Ok(AcceptedMigrationResolution {
        path: STATE_RELATIVE.to_owned(),
        accepted_at,
        resolutions: record.resolutions.len(),
    })
}

/// Applies a fresh accepted record to a newly recomputed unresolved plan.
/// Invalid or stale state never suppresses findings and is reported alongside
/// the original semantic work.
pub fn reconcile(project_root: &Path, plan: &mut MigrationPlan) {
    let record = match read_record(project_root) {
        Ok(Some(record)) => record,
        Ok(None) => return,
        Err(issue) => {
            plan.findings.push(issue);
            sort_findings(plan);
            return;
        }
    };
    let current = match validate_stored(project_root, plan, &record) {
        Ok(current) => current,
        Err(issue) => {
            plan.findings.push(issue);
            sort_findings(plan);
            return;
        }
    };
    let resolved = current
        .resolutions
        .iter()
        .map(|resolution| (resolution.code.as_str(), resolution.path.as_deref()))
        .collect::<BTreeSet<_>>();
    plan.findings
        .retain(|finding| !resolved.contains(&(finding.code, finding.path.as_deref())));
    plan.language = Some(current.target.language);
    plan.agents = current.target.agents;
    plan.target_converged = true;
    if let Some(action) = plan
        .actions
        .iter_mut()
        .find(|action| action.target.as_deref() == Some(".specbind.json"))
    {
        action.kind = "keep";
    }
    sort_findings(plan);
}

fn parse_candidate(input: &str) -> Result<ResolutionCandidate, MigrationIssues> {
    let candidate = serde_json::from_str::<ResolutionCandidate>(input).map_err(|error| {
        one_issue(
            "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
            None,
            format!("migration resolution candidate must be strict JSON: {error}"),
        )
    })?;
    if candidate.schema_version != 1 {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_VERSION_UNSUPPORTED",
            None,
            "schemaVersion must be 1",
        ));
    }
    if candidate.assessment.trim().is_empty() {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
            None,
            "assessment must contain the agent's migration rationale",
        ));
    }
    validate_agents(&candidate.target.agents)?;
    if candidate.resolutions.is_empty() {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
            None,
            "resolutions must contain at least one current finding",
        ));
    }
    Ok(candidate)
}

fn resolve_candidate(
    project_root: &Path,
    candidate: &ResolutionCandidate,
) -> Result<ResolvedCandidate, MigrationIssues> {
    let plan = migration::unresolved_plan(project_root)?;
    validate_target_selection_against_plan(&plan, &candidate.target)?;
    ensure_target_converged(project_root, &candidate.target)?;

    let current = plan
        .findings
        .iter()
        .map(|finding| (finding.code.to_owned(), finding.path.clone()))
        .collect::<BTreeSet<_>>();
    if let Some(finding) = plan
        .findings
        .iter()
        .find(|finding| !resolvable(finding.code))
    {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_BLOCKED",
            finding.path.clone(),
            format!(
                "{} is a mechanical safety finding and cannot be resolved by agent assessment",
                finding.code
            ),
        ));
    }

    let declared = candidate
        .resolutions
        .iter()
        .map(|resolution| (resolution.code.clone(), resolution.path.clone()))
        .collect::<BTreeSet<_>>();
    if declared.len() != candidate.resolutions.len() {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
            None,
            "resolutions must not contain duplicate code and path pairs",
        ));
    }
    if current != declared {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_FINDINGS_CHANGED",
            None,
            "candidate resolutions must exactly match all current migration findings",
        ));
    }

    let mut resolutions = Vec::new();
    for resolution in &candidate.resolutions {
        validate_disposition(resolution)?;
        let source = resolution.path.as_deref().unwrap_or(&plan.legacy_root);
        validate_project_relative(source, false)?;
        let source_fingerprint =
            fingerprint_source(project_root, &plan.legacy_root, resolution.path.as_deref())?;
        let mut target_fingerprints = BTreeMap::new();
        for target in &resolution.targets {
            validate_project_relative(target, true)?;
            if target == ".specbind" || target.starts_with(".specbind/state") {
                return Err(one_issue(
                    "MIGRATION_RESOLUTION_TARGET_INVALID",
                    Some(target.clone()),
                    "resolution targets must not include the SpecBind root or CLI-owned state",
                ));
            }
            let fingerprint = fingerprint_path(project_root, target)?;
            if target_fingerprints
                .insert(target.clone(), fingerprint)
                .is_some()
            {
                return Err(one_issue(
                    "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
                    Some(target.clone()),
                    "resolution targets must be unique",
                ));
            }
        }
        resolutions.push(StoredResolution {
            code: resolution.code.clone(),
            path: resolution.path.clone(),
            disposition: resolution.disposition,
            source_fingerprint,
            target_fingerprints,
        });
    }
    resolutions.sort_by(|left, right| (&left.code, &left.path).cmp(&(&right.code, &right.path)));
    let mut target = candidate.target.clone();
    target.agents.sort();
    Ok(ResolvedCandidate {
        legacy_root: plan.legacy_root,
        target,
        assessment: candidate.assessment.trim().to_owned(),
        resolutions,
    })
}

fn validate_stored(
    project_root: &Path,
    plan: &MigrationPlan,
    record: &StoredRecord,
) -> Result<ResolvedCandidate, MigrationFinding> {
    validate_record_shape(plan, record)?;
    validate_agents(&record.target.agents).map_err(|_| {
        finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            "accepted migration resolution has an invalid target selection",
        )
    })?;
    validate_target_selection_against_plan(plan, &record.target).map_err(|_| {
        finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            "accepted migration resolution conflicts with unambiguous legacy selections",
        )
    })?;
    ensure_target_converged(project_root, &record.target).map_err(|_| {
        finding(
            "MIGRATE_RESOLUTION_STALE",
            Some(STATE_RELATIVE.to_owned()),
            "accepted migration target is no longer the selected converged installation",
        )
    })?;
    let current_findings = plan
        .findings
        .iter()
        .map(|finding| (finding.code, finding.path.as_deref()))
        .collect::<BTreeSet<_>>();
    let stored_findings = record
        .resolutions
        .iter()
        .map(|resolution| (resolution.code.as_str(), resolution.path.as_deref()))
        .collect::<BTreeSet<_>>();
    if stored_findings.len() != record.resolutions.len()
        || stored_findings != current_findings
        || plan
            .findings
            .iter()
            .any(|finding| !resolvable(finding.code))
    {
        return Err(finding(
            "MIGRATE_RESOLUTION_STALE",
            Some(STATE_RELATIVE.to_owned()),
            "accepted migration findings no longer exactly match the current source inventory",
        ));
    }
    for resolution in &record.resolutions {
        if !resolvable(&resolution.code)
            || !current_findings.contains(&(resolution.code.as_str(), resolution.path.as_deref()))
        {
            return Err(finding(
                "MIGRATE_RESOLUTION_STALE",
                Some(STATE_RELATIVE.to_owned()),
                "accepted migration findings no longer match the current source inventory",
            ));
        }
        validate_resolution_fingerprints(project_root, plan, resolution)?;
    }
    Ok(ResolvedCandidate {
        legacy_root: record.legacy_root.clone(),
        target: record.target.clone(),
        assessment: record.assessment.clone(),
        resolutions: record.resolutions.clone(),
    })
}

fn validate_record_shape(
    plan: &MigrationPlan,
    record: &StoredRecord,
) -> Result<(), MigrationFinding> {
    if record.schema_version != 1 || record.legacy_root != plan.legacy_root {
        return Err(finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            "accepted migration resolution has an unsupported version or legacy root",
        ));
    }
    if record.assessment.trim().is_empty()
        || record.resolutions.is_empty()
        || OffsetDateTime::parse(&record.accepted_at, &Rfc3339).is_err()
    {
        return Err(finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            "accepted migration resolution has invalid required fields",
        ));
    }
    for resolution in &record.resolutions {
        let disposition_invalid = match resolution.disposition {
            Disposition::Converted => resolution.target_fingerprints.is_empty(),
            Disposition::NotMigrated => !resolution.target_fingerprints.is_empty(),
        };
        let source_invalid = resolution
            .path
            .as_deref()
            .is_some_and(|path| validate_project_relative(path, false).is_err());
        let target_invalid = resolution.target_fingerprints.keys().any(|path| {
            validate_project_relative(path, true).is_err()
                || path == ".specbind"
                || path.starts_with(".specbind/state")
        });
        let fingerprint_invalid = !valid_fingerprint(&resolution.source_fingerprint)
            || resolution
                .target_fingerprints
                .values()
                .any(|fingerprint| !valid_fingerprint(fingerprint));
        if disposition_invalid || source_invalid || target_invalid || fingerprint_invalid {
            return Err(finding(
                "MIGRATE_RESOLUTION_STATE_INVALID",
                Some(STATE_RELATIVE.to_owned()),
                "accepted migration resolution contains an invalid resolution entry",
            ));
        }
    }
    Ok(())
}

fn valid_fingerprint(value: &str) -> bool {
    value.len() == 71
        && value.strip_prefix("sha256:").is_some_and(|hex| {
            hex.bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
}

fn validate_resolution_fingerprints(
    project_root: &Path,
    plan: &MigrationPlan,
    resolution: &StoredResolution,
) -> Result<(), MigrationFinding> {
    let source = resolution.path.as_deref().unwrap_or(&plan.legacy_root);
    let current_source =
        fingerprint_source(project_root, &plan.legacy_root, resolution.path.as_deref()).map_err(
            |_| {
                finding(
                    "MIGRATE_RESOLUTION_STALE",
                    Some(source.to_owned()),
                    "an accepted migration source is missing, unsafe, or changed",
                )
            },
        )?;
    if current_source != resolution.source_fingerprint {
        return Err(finding(
            "MIGRATE_RESOLUTION_STALE",
            Some(source.to_owned()),
            "an accepted migration source changed after resolution",
        ));
    }
    for (target, expected) in &resolution.target_fingerprints {
        let current_target = fingerprint_path(project_root, target).map_err(|_| {
            finding(
                "MIGRATE_RESOLUTION_STALE",
                Some(target.clone()),
                "an accepted migration target is missing, unsafe, or changed",
            )
        })?;
        if &current_target != expected {
            return Err(finding(
                "MIGRATE_RESOLUTION_STALE",
                Some(target.clone()),
                "an accepted migration target changed after resolution",
            ));
        }
    }
    Ok(())
}

fn fingerprint_source(
    project_root: &Path,
    legacy_root: &str,
    relative: Option<&str>,
) -> Result<String, MigrationIssues> {
    if let Some(relative) = relative {
        return fingerprint_path(project_root, relative);
    }
    let mut hasher = Sha256::new();
    hash_field(&mut hasher, legacy_root.as_bytes());
    hash_field(
        &mut hasher,
        fingerprint_path(project_root, legacy_root)?.as_bytes(),
    );
    let config = project_root.join(".cc-sdd.json");
    match fs::symlink_metadata(&config) {
        Ok(_) => {
            hash_field(&mut hasher, b".cc-sdd.json");
            hash_field(
                &mut hasher,
                fingerprint_path(project_root, ".cc-sdd.json")?.as_bytes(),
            );
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            hash_field(&mut hasher, b"no-.cc-sdd.json");
        }
        Err(error) => {
            return Err(one_issue(
                "MIGRATION_RESOLUTION_INPUT_INVALID",
                Some(".cc-sdd.json".to_owned()),
                error.to_string(),
            ));
        }
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

fn validate_target_selection_against_plan(
    plan: &MigrationPlan,
    target: &TargetSelection,
) -> Result<(), MigrationIssues> {
    let language_is_semantic = plan.findings.iter().any(|finding| {
        matches!(
            finding.code,
            "MIGRATE_LANGUAGE_MIXED"
                | "MIGRATE_LANGUAGE_SELECTION_REQUIRED"
                | "MIGRATE_LANGUAGE_UNSUPPORTED"
        )
    });
    if !language_is_semantic && plan.language != Some(target.language) {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_TARGET_MISMATCH",
            Some(".specbind.json".to_owned()),
            "candidate language must match the unambiguous legacy language",
        ));
    }
    let agents_are_semantic = plan.findings.iter().any(|finding| {
        matches!(
            finding.code,
            "MIGRATE_AGENT_SELECTION_REQUIRED" | "MIGRATE_AGENT_UNSUPPORTED"
        )
    });
    let selected = target.agents.iter().cloned().collect::<BTreeSet<_>>();
    let legacy = plan.agents.iter().cloned().collect::<BTreeSet<_>>();
    if !agents_are_semantic && selected != legacy {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_TARGET_MISMATCH",
            Some(".specbind.json".to_owned()),
            "candidate agents must match the unambiguous legacy agent selection",
        ));
    }
    Ok(())
}

fn validate_disposition(resolution: &CandidateResolution) -> Result<(), MigrationIssues> {
    match resolution.disposition {
        Disposition::Converted if resolution.targets.is_empty() => Err(one_issue(
            "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
            resolution.path.clone(),
            "converted resolutions must identify at least one target",
        )),
        Disposition::NotMigrated if !resolution.targets.is_empty() => Err(one_issue(
            "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
            resolution.path.clone(),
            "not_migrated resolutions must not identify targets",
        )),
        _ => Ok(()),
    }
}

fn validate_agents(agents: &[String]) -> Result<(), MigrationIssues> {
    let unique = agents.iter().collect::<BTreeSet<_>>();
    if agents.is_empty()
        || unique.len() != agents.len()
        || agents.iter().any(|agent| Agent::parse(agent).is_none())
    {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_CANDIDATE_INVALID",
            None,
            "target agents must be a unique non-empty list of claude-code and/or codex",
        ));
    }
    Ok(())
}

fn ensure_target_converged(
    project_root: &Path,
    target: &TargetSelection,
) -> Result<(), MigrationIssues> {
    let agents = target
        .agents
        .iter()
        .map(|agent| Agent::parse(agent).expect("validated agent"))
        .collect();
    let inputs = InstallInputs {
        agents,
        language: Some(target.language),
        spec_dir: Some(".specbind".to_owned()),
        project_instructions: Some(false),
    };
    let plan = install::plan(project_root, &inputs).map_err(|error| MigrationIssues {
        issues: error
            .issues
            .into_iter()
            .map(|issue| finding(issue.code, issue.path, issue.message))
            .collect(),
    })?;
    if plan
        .entries
        .iter()
        .any(|entry| entry.action != PlanAction::Keep)
    {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_TARGET_NOT_CONVERGED",
            Some(".specbind.json".to_owned()),
            "install the selected SpecBind language and agents before accepting migration resolution",
        ));
    }
    Ok(())
}

fn read_record(project_root: &Path) -> Result<Option<StoredRecord>, MigrationFinding> {
    let path = project_root.join(STATE_RELATIVE);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(finding(
                "MIGRATE_RESOLUTION_STATE_INVALID",
                Some(STATE_RELATIVE.to_owned()),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            "accepted migration resolution must be a regular non-symlink file",
        ));
    }
    let input = fs::read_to_string(path).map_err(|error| {
        finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            error.to_string(),
        )
    })?;
    let value = yaml::parse(&input).map_err(|error| {
        finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            error.to_string(),
        )
    })?;
    serde_json::from_value(value).map(Some).map_err(|error| {
        finding(
            "MIGRATE_RESOLUTION_STATE_INVALID",
            Some(STATE_RELATIVE.to_owned()),
            error.to_string(),
        )
    })
}

fn persist(project_root: &Path, record: &StoredRecord) -> Result<(), MigrationIssues> {
    let parent = project_root.join(STATE_PARENT);
    match fs::symlink_metadata(&parent) {
        Ok(metadata) if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() => {
            return Err(one_issue(
                "MIGRATION_RESOLUTION_STATE_DIR_INVALID",
                Some(STATE_PARENT.to_owned()),
                "migration state parent must be a regular non-symlink directory",
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(&parent).map_err(|error| {
                one_issue(
                    "MIGRATION_RESOLUTION_STATE_DIR_CREATE_FAILED",
                    Some(STATE_PARENT.to_owned()),
                    error.to_string(),
                )
            })?;
        }
        Err(error) => {
            return Err(one_issue(
                "MIGRATION_RESOLUTION_STATE_DIR_INVALID",
                Some(STATE_PARENT.to_owned()),
                error.to_string(),
            ));
        }
    }
    let yaml = serde_saphyr::to_string(record).map_err(|error| {
        one_issue(
            "MIGRATION_RESOLUTION_SERIALIZE_FAILED",
            Some(STATE_RELATIVE.to_owned()),
            error.to_string(),
        )
    })?;
    guarded_fs::replace_optional(&project_root.join(STATE_RELATIVE), yaml.as_bytes()).map_err(
        |error| match error {
            GuardedWriteError::InvalidTarget(_) => one_issue(
                "MIGRATION_RESOLUTION_TARGET_INVALID",
                Some(STATE_RELATIVE.to_owned()),
                "accepted migration resolution must be absent or a regular non-symlink file",
            ),
            _ => one_issue(
                "MIGRATION_RESOLUTION_WRITE_FAILED",
                Some(STATE_RELATIVE.to_owned()),
                error.to_string(),
            ),
        },
    )
}

fn ensure_clean_repository(project_root: &Path) -> Result<(), MigrationIssues> {
    let committed = repository::predicate(project_root, &["rev-parse", "--verify", "-q", "HEAD"])
        .map_err(|error| {
        one_issue("MIGRATION_RESOLUTION_GIT_FAILED", None, error.to_string())
    })?;
    if !committed {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_COMMIT_REQUIRED",
            None,
            "migration resolution acceptance requires at least one commit",
        ));
    }
    let clean = repository::predicate(project_root, &["diff", "--quiet", "HEAD", "--"])
        .map_err(|error| one_issue("MIGRATION_RESOLUTION_GIT_FAILED", None, error.to_string()))?;
    let untracked = repository::output_bytes(
        project_root,
        &["ls-files", "--others", "--exclude-standard", "-z"],
    )
    .map_err(|error| one_issue("MIGRATION_RESOLUTION_GIT_FAILED", None, error.to_string()))?;
    if !clean || !untracked.is_empty() {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_REPOSITORY_DIRTY",
            None,
            "migration resolution acceptance requires a clean repository",
        ));
    }
    Ok(())
}

fn fingerprint_path(project_root: &Path, relative: &str) -> Result<String, MigrationIssues> {
    let root = project_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let mut entries = Vec::new();
    collect_fingerprint_entries(&root, &root, &mut entries, relative)?;
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    let mut hasher = Sha256::new();
    for (path, bytes) in entries {
        hash_field(&mut hasher, path.as_bytes());
        hash_field(&mut hasher, &bytes);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}

fn collect_fingerprint_entries(
    root: &Path,
    path: &Path,
    entries: &mut Vec<(String, Vec<u8>)>,
    display: &str,
) -> Result<(), MigrationIssues> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        one_issue(
            "MIGRATION_RESOLUTION_INPUT_INVALID",
            Some(display.to_owned()),
            error.to_string(),
        )
    })?;
    if guarded_fs::is_link_like(&metadata) {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_INPUT_INVALID",
            Some(display.to_owned()),
            "migration resolution inputs must not contain links or reparse points",
        ));
    }
    if metadata.is_file() {
        let relative = path
            .strip_prefix(root)
            .expect("fingerprint root contains path")
            .to_string_lossy()
            .replace('\\', "/");
        entries.push((
            relative,
            fs::read(path).map_err(|error| {
                one_issue(
                    "MIGRATION_RESOLUTION_INPUT_INVALID",
                    Some(display.to_owned()),
                    error.to_string(),
                )
            })?,
        ));
        return Ok(());
    }
    if !metadata.is_dir() {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_INPUT_INVALID",
            Some(display.to_owned()),
            "migration resolution input must be a regular file or directory",
        ));
    }
    let mut children = fs::read_dir(path)
        .map_err(|error| {
            one_issue(
                "MIGRATION_RESOLUTION_INPUT_INVALID",
                Some(display.to_owned()),
                error.to_string(),
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            one_issue(
                "MIGRATION_RESOLUTION_INPUT_INVALID",
                Some(display.to_owned()),
                error.to_string(),
            )
        })?;
    children.sort_by_key(std::fs::DirEntry::file_name);
    if children.is_empty() {
        let relative = path
            .strip_prefix(root)
            .expect("fingerprint root contains path")
            .to_string_lossy()
            .replace('\\', "/");
        entries.push((format!("{relative}/"), Vec::new()));
    }
    for child in children {
        collect_fingerprint_entries(root, &child.path(), entries, display)?;
    }
    Ok(())
}

fn hash_field(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update(bytes.len().to_be_bytes());
    hasher.update(bytes);
}

fn validate_project_relative(value: &str, target: bool) -> Result<(), MigrationIssues> {
    let path = Path::new(value);
    let invalid = value.is_empty()
        || value.contains('\\')
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::CurDir
                    | Component::ParentDir
                    | Component::RootDir
                    | Component::Prefix(_)
            )
        })
        || value.split('/').any(invalid_portable_segment);
    let target_outside = target
        && !(value == ".specbind.json"
            || value == "AGENTS.md"
            || value == "CLAUDE.md"
            || value.starts_with(".specbind/"));
    if invalid || target_outside {
        return Err(one_issue(
            "MIGRATION_RESOLUTION_TARGET_INVALID",
            Some(value.to_owned()),
            "migration resolution paths must be portable project-relative paths in the allowed target surface",
        ));
    }
    Ok(())
}

fn invalid_portable_segment(segment: &str) -> bool {
    if segment.is_empty()
        || segment.ends_with([' ', '.'])
        || segment
            .chars()
            .any(|value| value.is_control() || r#"<>:"\|?*"#.contains(value))
    {
        return true;
    }
    let stem = segment
        .split('.')
        .next()
        .unwrap_or(segment)
        .to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"))
            .is_some_and(|suffix| {
                matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
            })
}

fn resolvable(code: &str) -> bool {
    matches!(
        code,
        "MIGRATE_ACTIVE_SCOPE_AMBIGUOUS"
            | "MIGRATE_AGENT_SELECTION_REQUIRED"
            | "MIGRATE_AGENT_UNSUPPORTED"
            | "MIGRATE_DESIGN_TRACEABILITY_REQUIRED"
            | "MIGRATE_LANGUAGE_MIXED"
            | "MIGRATE_LANGUAGE_SELECTION_REQUIRED"
            | "MIGRATE_LANGUAGE_UNSUPPORTED"
            | "MIGRATE_LEGACY_CONTENT_UNSUPPORTED"
            | "MIGRATE_LEGACY_INSTRUCTIONS_AMBIGUOUS"
            | "MIGRATE_RULE_REVIEW_REQUIRED"
            | "MIGRATE_SPEC_CONVERSION_REQUIRED"
            | "MIGRATE_SPEC_ID_INVALID"
            | "MIGRATE_SPEC_METADATA_MISSING"
            | "MIGRATE_SPEC_STATE_INVALID"
            | "MIGRATE_STEERING_REVIEW_REQUIRED"
            | "MIGRATE_TEMPLATE_REVIEW_REQUIRED"
    )
}

fn sort_findings(plan: &mut MigrationPlan) {
    plan.findings.sort();
    plan.findings.dedup();
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

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> MigrationIssues {
    MigrationIssues {
        issues: vec![finding(code, path, message)],
    }
}
