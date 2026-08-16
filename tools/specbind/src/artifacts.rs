//! Type-based discovery and metadata-profile validation for spec-local OKF artifacts.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use camino::Utf8PathBuf;
use pulldown_cmark::{Event, Parser};
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::{
    domain::{self, tasks::Tasks},
    fingerprint::Fingerprint,
    freshness::CurrentGateInputs,
    requirements,
    schema::runtime,
};

const TYPE_BRIEF: &str = "SpecBind Brief";
const TYPE_RESEARCH: &str = "SpecBind Research";
const TYPE_REQUIREMENTS: &str = "SpecBind Requirements";
const TYPE_DESIGN: &str = "SpecBind Design";
const TYPE_CONTRACT: &str = "SpecBind Contract";
const TYPE_IMPLEMENTATION_NOTES: &str = "SpecBind Implementation Notes";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtifactKind {
    Brief,
    Research,
    Requirements,
    Design,
    Contract,
    ImplementationNotes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact {
    pub selector: String,
    pub artifact_type: String,
    pub path: Utf8PathBuf,
    pub artifact_id: Option<String>,
    pub kind: ArtifactKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiscoveryIssue {
    pub code: &'static str,
    pub path: Option<Utf8PathBuf>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactInventory {
    pub artifacts: Vec<Artifact>,
    pub issues: Vec<DiscoveryIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateInputResolution {
    pub inventory: ArtifactInventory,
    pub inputs: CurrentGateInputs,
}

/// Discovers recognized live artifacts for one canonical spec below a `SpecBind` root.
#[must_use]
pub fn discover_spec(specbind_root: &Path, canonical_spec: &str) -> ArtifactInventory {
    let mut issues = Vec::new();
    if !is_kebab_id(canonical_spec) {
        issues.push(issue(
            "ARTIFACT_SPEC_ID_INVALID",
            None,
            format!("canonical spec ID is invalid: {canonical_spec}"),
        ));
        return inventory(vec![], issues);
    }

    let specs_root = specbind_root.join("specs");
    if fs::symlink_metadata(&specs_root).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        issues.push(issue(
            "ARTIFACT_SPECS_DIR_SYMLINK",
            relative_utf8(specbind_root, &specs_root).ok(),
            "specs directory must not be a symbolic link",
        ));
        return inventory(vec![], issues);
    }
    let active_spec_dir = specs_root.join(canonical_spec);
    match fs::symlink_metadata(&active_spec_dir) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            issues.push(issue(
                "ARTIFACT_SPEC_DIR_SYMLINK",
                relative_utf8(specbind_root, &active_spec_dir).ok(),
                "spec directory must not be a symbolic link",
            ));
            return inventory(vec![], issues);
        }
        Ok(metadata) if !metadata.is_dir() => {
            issues.push(issue(
                "ARTIFACT_SPEC_DIR_NOT_DIRECTORY",
                relative_utf8(specbind_root, &active_spec_dir).ok(),
                "spec path is not a directory",
            ));
            return inventory(vec![], issues);
        }
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_SPEC_DIR_UNREADABLE",
                relative_utf8(specbind_root, &active_spec_dir).ok(),
                format!("cannot read spec directory: {error}"),
            ));
            return inventory(vec![], issues);
        }
        Ok(_) => {}
    }

    let candidates = scan_concepts(specbind_root, &active_spec_dir, &mut issues);
    let mut by_selector = BTreeMap::<String, Vec<Artifact>>::new();
    for artifact in candidates {
        by_selector
            .entry(artifact.selector.clone())
            .or_default()
            .push(artifact);
    }
    let mut artifacts = Vec::new();
    for (selector, matches) in by_selector {
        if matches.len() == 1 {
            artifacts.extend(matches);
        } else {
            for artifact in matches {
                issues.push(issue(
                    "ARTIFACT_SELECTOR_DUPLICATE",
                    Some(artifact.path),
                    format!("logical selector is duplicated: {selector}"),
                ));
            }
        }
    }
    inventory(artifacts, issues)
}

/// Re-discovers one spec and resolves the current gate-owned input projections.
#[must_use]
pub fn resolve_gate_inputs(specbind_root: &Path, canonical_spec: &str) -> GateInputResolution {
    let mut inventory = discover_spec(specbind_root, canonical_spec);
    let mut inputs = CurrentGateInputs::default();
    let mut design = BTreeMap::new();

    for artifact in &inventory.artifacts {
        let fingerprint = fingerprint_artifact(specbind_root, artifact, &mut inventory.issues);
        match artifact.kind {
            ArtifactKind::Requirements => inputs.requirements = fingerprint,
            ArtifactKind::Design | ArtifactKind::Contract => {
                if let Some(fingerprint) = fingerprint {
                    design.insert(artifact.selector.clone(), fingerprint);
                }
            }
            ArtifactKind::Brief | ArtifactKind::Research | ArtifactKind::ImplementationNotes => {}
        }
    }
    inputs.design = Some(design);
    inputs.task_plan = resolve_task_plan(specbind_root, canonical_spec, &mut inventory.issues);
    inventory.issues.sort();
    inventory.issues.dedup();

    GateInputResolution { inventory, inputs }
}

fn fingerprint_artifact(
    specbind_root: &Path,
    artifact: &Artifact,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<Fingerprint> {
    let native_path = specbind_root.join(artifact.path.as_std_path());
    let metadata = match fs::symlink_metadata(&native_path) {
        Ok(metadata) => metadata,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_CHANGED_DURING_RESOLUTION",
                Some(artifact.path.clone()),
                format!("artifact changed during fingerprint resolution: {error}"),
            ));
            return None;
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        issues.push(issue(
            "ARTIFACT_CHANGED_DURING_RESOLUTION",
            Some(artifact.path.clone()),
            "artifact is no longer a regular non-symlink file",
        ));
        return None;
    }
    match fs::read(&native_path) {
        Ok(bytes) => Some(Fingerprint::markdown(&bytes)),
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_READ_FAILED",
                Some(artifact.path.clone()),
                format!("cannot read artifact for fingerprinting: {error}"),
            ));
            None
        }
    }
}

fn resolve_task_plan(
    specbind_root: &Path,
    canonical_spec: &str,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<Fingerprint> {
    let relative = Utf8PathBuf::from(format!("specs/{canonical_spec}/tasks.yaml"));
    let native_path = specbind_root.join(relative.as_std_path());
    let metadata = match fs::symlink_metadata(&native_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return None,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASKS_READ_FAILED",
                Some(relative),
                format!("cannot inspect tasks.yaml: {error}"),
            ));
            return None;
        }
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        issues.push(issue(
            "ARTIFACT_TASKS_NOT_REGULAR",
            Some(relative),
            "tasks.yaml must be a regular non-symlink file",
        ));
        return None;
    }
    let input = match fs::read_to_string(&native_path) {
        Ok(input) => input,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASKS_READ_FAILED",
                Some(relative),
                format!("cannot read tasks.yaml as UTF-8: {error}"),
            ));
            return None;
        }
    };
    let wire = match runtime::load_tasks(&input) {
        Ok(wire) => wire,
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASKS_STRUCTURAL_INVALID",
                Some(relative),
                error.to_string(),
            ));
            return None;
        }
    };
    let tasks = match Tasks::try_from(wire) {
        Ok(tasks) => tasks,
        Err(error) => {
            for semantic in error.issues {
                issues.push(issue(
                    semantic.code,
                    Some(relative.clone()),
                    semantic.message,
                ));
            }
            return None;
        }
    };
    match Fingerprint::task_plan(&tasks) {
        Ok(fingerprint) => Some(fingerprint),
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_TASK_PLAN_FINGERPRINT_FAILED",
                Some(relative),
                format!("cannot canonicalize task plan: {error}"),
            ));
            None
        }
    }
}

fn scan_concepts(
    specbind_root: &Path,
    active_spec_dir: &Path,
    issues: &mut Vec<DiscoveryIssue>,
) -> Vec<Artifact> {
    let mut candidates = Vec::new();
    for entry in WalkDir::new(active_spec_dir)
        .follow_links(false)
        .sort_by_file_name()
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                issues.push(issue(
                    "ARTIFACT_WALK_FAILED",
                    error
                        .path()
                        .and_then(|path| relative_utf8(specbind_root, path).ok()),
                    format!("cannot inspect artifact path: {error}"),
                ));
                continue;
            }
        };
        if !entry.file_type().is_file() || !is_concept_path(entry.path()) {
            continue;
        }
        let path = match relative_utf8(specbind_root, entry.path()) {
            Ok(path) => path,
            Err(message) => {
                issues.push(issue("ARTIFACT_PATH_INVALID", None, message));
                continue;
            }
        };
        match inspect_concept(entry.path(), &path) {
            Ok((artifact, mut artifact_issues)) => {
                if let Some(artifact) = artifact {
                    candidates.push(artifact);
                }
                issues.append(&mut artifact_issues);
            }
            Err(mut artifact_issues) => issues.append(&mut artifact_issues),
        }
    }

    candidates
}

fn inspect_concept(
    native_path: &Path,
    path: &Utf8PathBuf,
) -> Result<(Option<Artifact>, Vec<DiscoveryIssue>), Vec<DiscoveryIssue>> {
    let bytes = fs::read(native_path).map_err(|error| {
        vec![issue(
            "ARTIFACT_READ_FAILED",
            Some(path.clone()),
            format!("cannot read artifact: {error}"),
        )]
    })?;
    let content = std::str::from_utf8(&bytes).map_err(|error| {
        vec![issue(
            "ARTIFACT_NOT_UTF8",
            Some(path.clone()),
            format!("artifact must be UTF-8: {error}"),
        )]
    })?;
    let (frontmatter, body) = split_frontmatter(content).map_err(|message| {
        vec![issue(
            "ARTIFACT_FRONTMATTER_INVALID",
            Some(path.clone()),
            message,
        )]
    })?;
    let value: Value = serde_saphyr::from_str(frontmatter).map_err(|error| {
        vec![issue(
            "ARTIFACT_FRONTMATTER_YAML_INVALID",
            Some(path.clone()),
            error.to_string(),
        )]
    })?;
    let mapping = value.as_object().ok_or_else(|| {
        vec![issue(
            "ARTIFACT_FRONTMATTER_NOT_MAPPING",
            Some(path.clone()),
            "frontmatter root must be a mapping",
        )]
    })?;
    let artifact_type = mapping.get("type").and_then(Value::as_str).ok_or_else(|| {
        vec![issue(
            "ARTIFACT_TYPE_INVALID",
            Some(path.clone()),
            "frontmatter type must be a non-empty string",
        )]
    })?;
    if artifact_type.is_empty() {
        return Err(vec![issue(
            "ARTIFACT_TYPE_INVALID",
            Some(path.clone()),
            "frontmatter type must be a non-empty string",
        )]);
    }

    let Some(kind) = recognized_kind(artifact_type) else {
        return Ok((None, vec![]));
    };
    let body_start_line = content[..content.len() - body.len()]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1;
    let mut profile_issues = validate_profile(kind, mapping, body, body_start_line, path);
    if contains_instruction(body) {
        profile_issues.push(issue(
            "ARTIFACT_TEMPLATE_INSTRUCTION_LEAK",
            Some(path.clone()),
            "live artifact contains a specbind:instruction template directive",
        ));
    }
    let artifact_id = collection_id(kind, mapping).map(str::to_owned);
    if matches!(
        kind,
        ArtifactKind::Design | ArtifactKind::ImplementationNotes
    ) && artifact_id.is_none()
    {
        return Ok((None, profile_issues));
    }
    Ok((
        Some(Artifact {
            selector: selector(kind, artifact_id.as_deref()),
            artifact_type: artifact_type.to_owned(),
            path: path.clone(),
            artifact_id,
            kind,
        }),
        profile_issues,
    ))
}

fn validate_profile(
    kind: ArtifactKind,
    mapping: &Map<String, Value>,
    body: &str,
    body_start_line: usize,
    path: &Utf8PathBuf,
) -> Vec<DiscoveryIssue> {
    let mut issues = Vec::new();
    match kind {
        ArtifactKind::Brief
        | ArtifactKind::Research
        | ArtifactKind::Requirements
        | ArtifactKind::Contract => {
            if mapping.contains_key("artifact_id") {
                issues.push(issue(
                    "ARTIFACT_SINGLETON_ID_FORBIDDEN",
                    Some(path.clone()),
                    "singleton artifact must omit artifact_id",
                ));
            }
        }
        ArtifactKind::Design | ArtifactKind::ImplementationNotes => {
            if collection_id(kind, mapping).is_none() {
                issues.push(issue(
                    "ARTIFACT_COLLECTION_ID_INVALID",
                    Some(path.clone()),
                    "collection artifact requires a lowercase kebab-case artifact_id",
                ));
            }
        }
    }

    if kind == ArtifactKind::Requirements
        && let Some((requirement_label, acceptance_criteria_label)) =
            validate_heading_labels(mapping, path, &mut issues)
        && let Err(error) = requirements::parse(body, requirement_label, acceptance_criteria_label)
    {
        issues.extend(error.issues.into_iter().map(|body_issue| {
            issue(
                body_issue.code,
                Some(path.clone()),
                format!(
                    "line {}: {}",
                    body_start_line + body_issue.line - 1,
                    body_issue.message
                ),
            )
        }));
    }
    if kind == ArtifactKind::Design {
        validate_design_requirement_ids(mapping, path, &mut issues);
    }
    if kind == ArtifactKind::Research && body.trim().is_empty() {
        issues.push(issue(
            "ARTIFACT_RESEARCH_BODY_EMPTY",
            Some(path.clone()),
            "research body must be non-empty",
        ));
    }
    if kind == ArtifactKind::ImplementationNotes && !has_non_comment_content(body) {
        issues.push(issue(
            "ARTIFACT_IMPLEMENTATION_NOTES_BODY_EMPTY",
            Some(path.clone()),
            "implementation-notes body must contain non-comment content",
        ));
    }
    issues
}

fn validate_heading_labels<'a>(
    mapping: &'a Map<String, Value>,
    path: &Utf8PathBuf,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<(&'a str, &'a str)> {
    let Some(labels) = mapping.get("heading_labels").and_then(Value::as_object) else {
        issues.push(issue(
            "ARTIFACT_HEADING_LABELS_INVALID",
            Some(path.clone()),
            "requirements heading_labels must be a mapping",
        ));
        return None;
    };
    let expected = BTreeSet::from(["acceptance_criteria", "requirement"]);
    let exact_keys = labels.keys().map(String::as_str).collect::<BTreeSet<_>>() == expected;
    if !exact_keys {
        issues.push(issue(
            "ARTIFACT_HEADING_LABELS_INVALID",
            Some(path.clone()),
            "heading_labels must contain exactly requirement and acceptance_criteria",
        ));
    }
    let mut valid_values = true;
    for key in expected {
        if !labels
            .get(key)
            .and_then(Value::as_str)
            .is_some_and(valid_label)
        {
            valid_values = false;
            issues.push(issue(
                "ARTIFACT_HEADING_LABEL_INVALID",
                Some(path.clone()),
                format!("heading_labels.{key} must be a trimmed non-empty single-line string"),
            ));
        }
    }
    if !exact_keys || !valid_values {
        return None;
    }
    Some((
        labels.get("requirement")?.as_str()?,
        labels.get("acceptance_criteria")?.as_str()?,
    ))
}

fn validate_design_requirement_ids(
    mapping: &Map<String, Value>,
    path: &Utf8PathBuf,
    issues: &mut Vec<DiscoveryIssue>,
) {
    let Some(ids) = mapping.get("requirement_ids").and_then(Value::as_array) else {
        issues.push(issue(
            "ARTIFACT_DESIGN_REQUIREMENT_IDS_INVALID",
            Some(path.clone()),
            "design requirement_ids must be a non-empty array",
        ));
        return;
    };
    let values = ids.iter().filter_map(Value::as_str).collect::<Vec<_>>();
    let unique = values.iter().copied().collect::<BTreeSet<_>>();
    if ids.is_empty()
        || values.len() != ids.len()
        || unique.len() != ids.len()
        || values
            .iter()
            .any(|id| domain::parse_requirement_id(id).is_none())
    {
        issues.push(issue(
            "ARTIFACT_DESIGN_REQUIREMENT_IDS_INVALID",
            Some(path.clone()),
            "design requirement_ids must contain unique canonical N.M strings",
        ));
    }
}

fn split_frontmatter(content: &str) -> Result<(&str, &str), String> {
    let mut offset = 0;
    let mut lines = content.split_inclusive('\n');
    let first = lines.next().ok_or_else(|| "artifact is empty".to_owned())?;
    if line_content(first) != "---" {
        return Err("frontmatter must begin with --- on the first line".to_owned());
    }
    offset += first.len();
    let frontmatter_start = offset;
    for line in lines {
        if line_content(line) == "---" {
            let frontmatter = &content[frontmatter_start..offset];
            let body = &content[offset + line.len()..];
            return Ok((frontmatter, body));
        }
        offset += line.len();
    }
    Err("frontmatter closing --- delimiter is missing".to_owned())
}

fn line_content(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

fn is_concept_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "md")
        && !matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some("index.md" | "log.md")
        )
}

fn recognized_kind(value: &str) -> Option<ArtifactKind> {
    match value {
        TYPE_BRIEF => Some(ArtifactKind::Brief),
        TYPE_RESEARCH => Some(ArtifactKind::Research),
        TYPE_REQUIREMENTS => Some(ArtifactKind::Requirements),
        TYPE_DESIGN => Some(ArtifactKind::Design),
        TYPE_CONTRACT => Some(ArtifactKind::Contract),
        TYPE_IMPLEMENTATION_NOTES => Some(ArtifactKind::ImplementationNotes),
        _ => None,
    }
}

fn collection_id(kind: ArtifactKind, mapping: &Map<String, Value>) -> Option<&str> {
    matches!(
        kind,
        ArtifactKind::Design | ArtifactKind::ImplementationNotes
    )
    .then(|| {
        mapping
            .get("artifact_id")?
            .as_str()
            .filter(|id| is_kebab_id(id))
    })
    .flatten()
}

fn is_kebab_id(value: &str) -> bool {
    if value.is_empty() || value.len() > 64 {
        return false;
    }
    let mut segments = value.split('-');
    let Some(first) = segments.next() else {
        return false;
    };
    first
        .bytes()
        .next()
        .is_some_and(|byte| byte.is_ascii_lowercase())
        && std::iter::once(first).chain(segments).all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        })
}

fn valid_label(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.bytes().any(|byte| matches!(byte, b'\r' | b'\n'))
}

fn selector(kind: ArtifactKind, artifact_id: Option<&str>) -> String {
    match kind {
        ArtifactKind::Brief => "brief".to_owned(),
        ArtifactKind::Research => "research".to_owned(),
        ArtifactKind::Requirements => "requirements".to_owned(),
        ArtifactKind::Design => format!("design/{}", artifact_id.expect("validated collection ID")),
        ArtifactKind::Contract => "contract".to_owned(),
        ArtifactKind::ImplementationNotes => format!(
            "implementation-notes/{}",
            artifact_id.expect("validated collection ID")
        ),
    }
}

fn inventory(mut artifacts: Vec<Artifact>, mut issues: Vec<DiscoveryIssue>) -> ArtifactInventory {
    artifacts.sort_by(|left, right| artifact_order(left).cmp(&artifact_order(right)));
    issues.sort();
    issues.dedup();
    ArtifactInventory { artifacts, issues }
}

fn artifact_order(artifact: &Artifact) -> (u8, &str) {
    let rank = match artifact.kind {
        ArtifactKind::Brief => 0,
        ArtifactKind::Research => 1,
        ArtifactKind::Requirements => 2,
        ArtifactKind::Design => 3,
        ArtifactKind::Contract => 4,
        ArtifactKind::ImplementationNotes => 5,
    };
    (rank, artifact.artifact_id.as_deref().unwrap_or(""))
}

fn contains_instruction(body: &str) -> bool {
    Parser::new(body).any(|event| match event {
        Event::Html(value) | Event::InlineHtml(value) => is_instruction_comment(&value),
        _ => false,
    })
}

fn has_non_comment_content(body: &str) -> bool {
    Parser::new(body).any(|event| match event {
        Event::Text(value) | Event::Code(value) => !value.trim().is_empty(),
        Event::Html(value) | Event::InlineHtml(value) => {
            !is_complete_comment(&value) && !value.trim().is_empty()
        }
        _ => false,
    })
}

fn is_instruction_comment(value: &str) -> bool {
    comment_content(value).is_some_and(|content| {
        content
            .trim()
            .strip_prefix("specbind:instruction")
            .is_some_and(|suffix| suffix.is_empty() || suffix.starts_with(char::is_whitespace))
    })
}

fn is_complete_comment(value: &str) -> bool {
    comment_content(value).is_some()
}

fn comment_content(value: &str) -> Option<&str> {
    value.trim().strip_prefix("<!--")?.strip_suffix("-->")
}

fn relative_utf8(root: &Path, path: &Path) -> Result<Utf8PathBuf, String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| format!("artifact path escapes SpecBind root: {}", path.display()))?;
    let utf8 = Utf8PathBuf::from_path_buf(relative.to_path_buf())
        .map_err(|path| format!("artifact path is not UTF-8: {}", path.display()))?;
    Ok(Utf8PathBuf::from(utf8.as_str().replace('\\', "/")))
}

fn issue(
    code: &'static str,
    path: Option<Utf8PathBuf>,
    message: impl Into<String>,
) -> DiscoveryIssue {
    DiscoveryIssue {
        code,
        path,
        message: message.into(),
    }
}
