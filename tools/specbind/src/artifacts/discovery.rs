//! Filesystem discovery and metadata-profile validation for live OKF artifacts.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use camino::Utf8PathBuf;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use serde_json::{Map, Value};
use walkdir::WalkDir;

use super::{
    Artifact, ArtifactInventory, ArtifactKind, DiscoveryIssue, SpecDiscovery, SpecEntryFault,
};
use crate::{contract, design, domain, instruction, requirements};

const TYPE_BRIEF: &str = "SpecBind Brief";
const TYPE_RESEARCH: &str = "SpecBind Research";
const TYPE_REQUIREMENTS: &str = "SpecBind Requirements";
const TYPE_DESIGN: &str = "SpecBind Design";
const TYPE_CONTRACT: &str = "SpecBind Contract";
const TYPE_IMPLEMENTATION_NOTES: &str = "SpecBind Implementation Notes";

pub(super) fn validate_spec_directory(
    specbind_root: &Path,
    canonical_spec: &str,
    issues: &mut Vec<DiscoveryIssue>,
) -> Option<std::path::PathBuf> {
    if !is_kebab_id(canonical_spec) {
        issues.push(issue(
            "ARTIFACT_SPEC_ID_INVALID",
            None,
            format!("canonical spec ID is invalid: {canonical_spec}"),
        ));
        return None;
    }

    let specs_root = specbind_root.join("specs");
    if fs::symlink_metadata(&specs_root).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        issues.push(issue(
            "ARTIFACT_SPECS_DIR_SYMLINK",
            relative_utf8(specbind_root, &specs_root).ok(),
            "specs directory must not be a symbolic link",
        ));
        return None;
    }
    let active_spec_dir = specs_root.join(canonical_spec);
    match fs::symlink_metadata(&active_spec_dir) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            issues.push(issue(
                "ARTIFACT_SPEC_DIR_SYMLINK",
                relative_utf8(specbind_root, &active_spec_dir).ok(),
                "spec directory must not be a symbolic link",
            ));
            None
        }
        Ok(metadata) if !metadata.is_dir() => {
            issues.push(issue(
                "ARTIFACT_SPEC_DIR_NOT_DIRECTORY",
                relative_utf8(specbind_root, &active_spec_dir).ok(),
                "spec path is not a directory",
            ));
            None
        }
        Err(error) => {
            issues.push(issue(
                "ARTIFACT_SPEC_DIR_UNREADABLE",
                relative_utf8(specbind_root, &active_spec_dir).ok(),
                format!("cannot read spec directory: {error}"),
            ));
            None
        }
        Ok(_) => Some(active_spec_dir),
    }
}

/// Reports whether a value is a canonical `SpecBind` Spec or Direct identity.
#[must_use]
pub fn canonical_id(value: &str) -> bool {
    let mut bytes = value.bytes();
    value.len() <= 64
        && bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.ends_with('-')
        && !value.contains("--")
}

/// Enumerates every persistent Spec identity below a `SpecBind` root.
///
/// # Errors
///
/// A missing `specs/` directory is the normal state before the first Spec is
/// created and returns an empty discovery. Other failures to read the directory
/// are returned to the caller. A single rejected entry is a fault, not a
/// failure, so one malformed directory never hides the Specs beside it.
pub fn discover_spec_ids(specbind_root: &Path) -> Result<SpecDiscovery, String> {
    let entries = match fs::read_dir(specbind_root.join("specs")) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(SpecDiscovery {
                specs: BTreeSet::new(),
                faults: Vec::new(),
            });
        }
        Err(error) => return Err(error.to_string()),
    };
    let mut specs = BTreeSet::new();
    let mut faults = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                faults.push((None, SpecEntryFault::Unreadable(error.to_string())));
                continue;
            }
        };
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            faults.push((None, SpecEntryFault::NonUtf8Name));
            continue;
        };
        let relative = Utf8PathBuf::from(format!("specs/{name}"));
        match entry.file_type() {
            Ok(file_type) if file_type.is_dir() && !file_type.is_symlink() => {}
            Ok(_) => {
                faults.push((Some(relative), SpecEntryFault::NotADirectory));
                continue;
            }
            Err(error) => {
                faults.push((
                    Some(relative),
                    SpecEntryFault::Unreadable(error.to_string()),
                ));
                continue;
            }
        }
        if canonical_id(name) {
            specs.insert(name.to_owned());
        } else {
            faults.push((Some(relative), SpecEntryFault::InvalidId));
        }
    }
    Ok(SpecDiscovery { specs, faults })
}

/// Discovers recognized live artifacts for one canonical spec below a `SpecBind` root.
#[must_use]
pub fn discover_spec(specbind_root: &Path, canonical_spec: &str) -> ArtifactInventory {
    let mut issues = Vec::new();
    let Some(active_spec_dir) = validate_spec_directory(specbind_root, canonical_spec, &mut issues)
    else {
        return inventory(vec![], issues);
    };

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
    profile_issues.extend(
        instruction::validate_live(body)
            .into_iter()
            .map(|fault| issue(fault.code, Some(path.clone()), fault.message)),
    );
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
    let semantic_body = instruction::mask(body);
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
        && let Err(error) =
            requirements::parse(&semantic_body, requirement_label, acceptance_criteria_label)
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
    if kind == ArtifactKind::Design
        && let Some(declared_ids) = validate_design_requirement_ids(mapping, path, &mut issues)
        && let Err(error) = design::validate(&semantic_body, &declared_ids)
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
    if kind == ArtifactKind::Contract
        && let Err(error) = contract::parse(&semantic_body)
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
    if kind == ArtifactKind::Brief && !has_substantive_content(body) {
        issues.push(issue(
            "ARTIFACT_BRIEF_BODY_EMPTY",
            Some(path.clone()),
            "brief body must contain substantive non-heading, non-comment content",
        ));
    }
    if kind == ArtifactKind::Research && !has_substantive_content(body) {
        issues.push(issue(
            "ARTIFACT_RESEARCH_BODY_EMPTY",
            Some(path.clone()),
            "research body must contain substantive non-heading, non-comment content",
        ));
    }
    if kind == ArtifactKind::ImplementationNotes && !has_substantive_content(body) {
        issues.push(issue(
            "ARTIFACT_IMPLEMENTATION_NOTES_BODY_EMPTY",
            Some(path.clone()),
            "implementation-notes body must contain substantive non-heading, non-comment content",
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
) -> Option<Vec<String>> {
    let Some(ids) = mapping.get("requirement_ids").and_then(Value::as_array) else {
        issues.push(issue(
            "ARTIFACT_DESIGN_REQUIREMENT_IDS_INVALID",
            Some(path.clone()),
            "design requirement_ids must be a non-empty array",
        ));
        return None;
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
        return None;
    }
    Some(values.into_iter().map(str::to_owned).collect())
}

pub(crate) fn split_frontmatter(content: &str) -> Result<(&str, &str), String> {
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

pub(crate) fn recognized_kind(value: &str) -> Option<ArtifactKind> {
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

pub(crate) fn collection_id(kind: ArtifactKind, mapping: &Map<String, Value>) -> Option<&str> {
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

pub(crate) fn is_kebab_id(value: &str) -> bool {
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

pub(super) fn valid_label(value: &str) -> bool {
    !value.is_empty()
        && value.trim() == value
        && !value.bytes().any(|byte| matches!(byte, b'\r' | b'\n'))
}

pub(crate) fn selector(kind: ArtifactKind, artifact_id: Option<&str>) -> String {
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

fn has_substantive_content(body: &str) -> bool {
    let body = instruction::mask(body);
    let mut in_heading = false;
    for event in Parser::new(&body) {
        match event {
            Event::Start(Tag::Heading { .. }) => in_heading = true,
            Event::End(TagEnd::Heading(_)) => in_heading = false,
            Event::Text(value) | Event::Code(value) if !in_heading && !value.trim().is_empty() => {
                return true;
            }
            Event::Html(value) | Event::InlineHtml(value)
                if !in_heading && !is_complete_comment(&value) && !value.trim().is_empty() =>
            {
                return true;
            }
            _ => {}
        }
    }
    false
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

pub(super) fn issue(
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
