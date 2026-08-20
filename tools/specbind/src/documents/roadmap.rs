//! Document semantics and review projection for `SpecBind Roadmap` artifacts.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use petgraph::{algo::is_cyclic_directed, graphmap::DiGraphMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use schemars::JsonSchema;

use crate::release;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapDocument {
    pub milestone_id: String,
    pub baseline_revision: String,
    pub target_release: Option<String>,
    pub new_specs: Vec<SpecItem>,
    pub spec_updates: Vec<SpecItem>,
    pub direct_changes: Vec<DirectItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecItem {
    pub spec: String,
    pub summary: String,
    #[serde(default)]
    pub depends_on: Vec<Dependency>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DirectItem {
    pub id: String,
    pub summary: String,
    #[serde(default)]
    pub depends_on: Vec<Dependency>,
    #[serde(default)]
    pub status: Option<DirectStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DirectStatus {
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectCompletionEdit {
    Updated(String),
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleaseBindingEdit {
    Updated(String),
    NoChange,
    RebindRequired { current: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Dependency {
    Spec(SpecDependency),
    Direct(DirectDependency),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SpecDependency {
    pub spec: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DirectDependency {
    pub direct: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CrossSpecScope {
    pub milestone_id: String,
    pub baseline_revision: String,
    pub work_items: CrossSpecWorkItems,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CrossSpecWorkItems {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub new_specs: Vec<CrossSpecItem>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub spec_updates: Vec<CrossSpecItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CrossSpecItem {
    pub spec: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<SpecDependency>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoadmapIssue {
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapIssues {
    pub issues: Vec<RoadmapIssue>,
}

impl fmt::Display for RoadmapIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "roadmap has {} issue(s)", self.issues.len())
    }
}

impl std::error::Error for RoadmapIssues {}

#[derive(Debug, Deserialize)]
struct RawRoadmap {
    #[serde(rename = "type")]
    artifact_type: String,
    milestone_id: String,
    baseline_revision: String,
    target_release: Value,
    work_items: RawWorkItems,
    #[serde(flatten)]
    _extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWorkItems {
    new_specs: Option<Vec<SpecItem>>,
    spec_updates: Option<Vec<SpecItem>>,
    direct_changes: Option<Vec<DirectItem>>,
}

/// Parses and semantically validates an active Roadmap Markdown file.
///
/// # Errors
///
/// Returns structural or cross-item diagnostics when no authoritative scope can
/// be derived.
pub fn parse(content: &str) -> Result<RoadmapDocument, RoadmapIssues> {
    let frontmatter = split_frontmatter(content).map_err(single_issue)?;
    let raw = serde_saphyr::from_str::<RawRoadmap>(frontmatter)
        .map_err(|error| single_issue(format!("ROADMAP_FRONTMATTER_INVALID: {error}")))?;
    let mut issues = Vec::new();
    if raw.artifact_type != "SpecBind Roadmap" {
        issues.push(issue(
            "ROADMAP_TYPE_INVALID",
            "/type",
            "type must be SpecBind Roadmap",
        ));
    }
    if Uuid::parse_str(&raw.milestone_id).map_or(true, |id| {
        id.get_version_num() != 7 || id.hyphenated().to_string() != raw.milestone_id
    }) {
        issues.push(issue(
            "ROADMAP_MILESTONE_ID_INVALID",
            "/milestone_id",
            "milestone_id must be a canonical UUID v7",
        ));
    }
    if !valid_revision(&raw.baseline_revision) {
        issues.push(issue(
            "ROADMAP_BASELINE_REVISION_INVALID",
            "/baseline_revision",
            "baseline_revision must be a full lowercase 40- or 64-character Git object ID",
        ));
    }
    if !raw.target_release.is_null()
        && !raw
            .target_release
            .as_str()
            .is_some_and(release::valid_version)
    {
        issues.push(issue(
            "ROADMAP_TARGET_RELEASE_INVALID",
            "/target_release",
            "target_release must be null or match ^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$",
        ));
    }
    for (category, empty) in [
        (
            "new_specs",
            raw.work_items.new_specs.as_ref().is_some_and(Vec::is_empty),
        ),
        (
            "spec_updates",
            raw.work_items
                .spec_updates
                .as_ref()
                .is_some_and(Vec::is_empty),
        ),
        (
            "direct_changes",
            raw.work_items
                .direct_changes
                .as_ref()
                .is_some_and(Vec::is_empty),
        ),
    ] {
        if empty {
            issues.push(issue(
                "ROADMAP_WORK_ITEM_CATEGORY_EMPTY",
                format!("/work_items/{category}"),
                "present work-item categories must be non-empty",
            ));
        }
    }
    let new_specs = raw.work_items.new_specs.unwrap_or_default();
    let spec_updates = raw.work_items.spec_updates.unwrap_or_default();
    let direct_changes = raw.work_items.direct_changes.unwrap_or_default();
    if new_specs.is_empty() && spec_updates.is_empty() && direct_changes.is_empty() {
        issues.push(issue(
            "ROADMAP_WORK_ITEMS_EMPTY",
            "/work_items",
            "work_items must contain at least one non-empty category",
        ));
    }
    validate_items(&new_specs, &spec_updates, &direct_changes, &mut issues);
    if issues.is_empty() {
        Ok(RoadmapDocument {
            milestone_id: raw.milestone_id,
            baseline_revision: raw.baseline_revision,
            target_release: raw.target_release.as_str().map(str::to_owned),
            new_specs,
            spec_updates,
            direct_changes,
        })
    } else {
        issues.sort();
        issues.dedup();
        Err(RoadmapIssues { issues })
    }
}

impl RoadmapDocument {
    #[must_use]
    pub fn cross_spec_scope(&self) -> CrossSpecScope {
        CrossSpecScope {
            milestone_id: self.milestone_id.clone(),
            baseline_revision: self.baseline_revision.clone(),
            work_items: CrossSpecWorkItems {
                new_specs: normalized_items(&self.new_specs),
                spec_updates: normalized_items(&self.spec_updates),
            },
        }
    }

    #[must_use]
    pub fn spec_ids(&self) -> Vec<String> {
        let mut ids = self
            .new_specs
            .iter()
            .chain(&self.spec_updates)
            .map(|item| item.spec.clone())
            .collect::<Vec<_>>();
        ids.sort();
        ids
    }
}

/// Marks one pending Direct item completed while preserving the Roadmap body.
///
/// # Errors
///
/// Returns Roadmap diagnostics when the current document is invalid, the target
/// does not exist, or the mutated frontmatter cannot be rendered and revalidated.
pub fn complete_direct(
    content: &str,
    canonical_direct: &str,
) -> Result<DirectCompletionEdit, RoadmapIssues> {
    let roadmap = parse(content)?;
    let Some(item) = roadmap
        .direct_changes
        .iter()
        .find(|item| item.id == canonical_direct)
    else {
        return Err(RoadmapIssues {
            issues: vec![issue(
                "ROADMAP_DIRECT_NOT_FOUND",
                "/work_items/direct_changes",
                format!("direct item {canonical_direct} does not exist"),
            )],
        });
    };
    if item.status == Some(DirectStatus::Completed) {
        return Ok(DirectCompletionEdit::NoChange);
    }

    let (frontmatter, body) = split_frontmatter_parts(content).map_err(single_issue)?;
    let mut value = serde_saphyr::from_str::<Value>(frontmatter)
        .map_err(|error| single_issue(format!("ROADMAP_FRONTMATTER_INVALID: {error}")))?;
    let direct_changes = value
        .get_mut("work_items")
        .and_then(|value| value.get_mut("direct_changes"))
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            single_issue(
                "ROADMAP_DIRECT_MUTATION_FAILED: direct_changes is not an array".to_owned(),
            )
        })?;
    let target = direct_changes
        .iter_mut()
        .find(|value| value.get("id").and_then(Value::as_str) == Some(canonical_direct))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            single_issue("ROADMAP_DIRECT_MUTATION_FAILED: target item disappeared".to_owned())
        })?;
    target.insert("status".to_owned(), Value::String("completed".to_owned()));
    let yaml = serde_saphyr::to_string(&value)
        .map_err(|error| single_issue(format!("ROADMAP_DIRECT_SERIALIZE_FAILED: {error}")))?;
    let rendered = format!("---\n{yaml}---\n{body}");
    let validated = parse(&rendered)?;
    if !validated
        .direct_changes
        .iter()
        .any(|item| item.id == canonical_direct && item.status == Some(DirectStatus::Completed))
    {
        return Err(single_issue(
            "ROADMAP_DIRECT_MUTATION_FAILED: completed status was not preserved".to_owned(),
        ));
    }
    Ok(DirectCompletionEdit::Updated(rendered))
}

/// Binds or explicitly rebinds the release label while preserving the Roadmap body.
///
/// # Errors
///
/// Returns Roadmap diagnostics when the document or requested release is invalid,
/// or when the mutated frontmatter cannot be rendered and revalidated.
pub fn bind_release(
    content: &str,
    requested: &str,
    allow_rebind: bool,
) -> Result<ReleaseBindingEdit, RoadmapIssues> {
    if !release::valid_version(requested) {
        return Err(RoadmapIssues {
            issues: vec![issue(
                "ROADMAP_TARGET_RELEASE_INVALID",
                "/target_release",
                "requested release must match ^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$",
            )],
        });
    }
    let roadmap = parse(content)?;
    if roadmap.target_release.as_deref() == Some(requested) {
        return Ok(ReleaseBindingEdit::NoChange);
    }
    if let Some(current) = roadmap.target_release
        && !allow_rebind
    {
        return Ok(ReleaseBindingEdit::RebindRequired { current });
    }

    let (frontmatter, body) = split_frontmatter_parts(content).map_err(single_issue)?;
    let mut value = serde_saphyr::from_str::<Value>(frontmatter)
        .map_err(|error| single_issue(format!("ROADMAP_FRONTMATTER_INVALID: {error}")))?;
    let root = value.as_object_mut().ok_or_else(|| {
        single_issue("ROADMAP_RELEASE_MUTATION_FAILED: root is not a mapping".to_owned())
    })?;
    root.insert(
        "target_release".to_owned(),
        Value::String(requested.to_owned()),
    );
    let yaml = serde_saphyr::to_string(&value)
        .map_err(|error| single_issue(format!("ROADMAP_RELEASE_SERIALIZE_FAILED: {error}")))?;
    let rendered = format!("---\n{yaml}---\n{body}");
    let validated = parse(&rendered)?;
    if validated.target_release.as_deref() != Some(requested) {
        return Err(single_issue(
            "ROADMAP_RELEASE_MUTATION_FAILED: target release was not preserved".to_owned(),
        ));
    }
    Ok(ReleaseBindingEdit::Updated(rendered))
}

fn validate_items(
    new_specs: &[SpecItem],
    spec_updates: &[SpecItem],
    direct_changes: &[DirectItem],
    issues: &mut Vec<RoadmapIssue>,
) {
    let mut keys = BTreeSet::new();
    for item in new_specs.iter().chain(spec_updates) {
        validate_identity(&item.spec, "spec", issues);
        validate_summary(&item.summary, &item.spec, issues);
        if !keys.insert(format!("spec:{}", item.spec)) {
            issues.push(issue(
                "ROADMAP_ITEM_DUPLICATE",
                "/work_items",
                format!("spec item {} is duplicated", item.spec),
            ));
        }
    }
    for item in direct_changes {
        validate_identity(&item.id, "direct", issues);
        validate_summary(&item.summary, &item.id, issues);
        if !keys.insert(format!("direct:{}", item.id)) {
            issues.push(issue(
                "ROADMAP_ITEM_DUPLICATE",
                "/work_items/direct_changes",
                format!("direct item {} is duplicated", item.id),
            ));
        }
    }

    let indices = keys
        .iter()
        .enumerate()
        .map(|(index, key)| (key.clone(), index))
        .collect::<BTreeMap<_, _>>();
    let mut graph = DiGraphMap::<usize, ()>::new();
    for index in indices.values() {
        graph.add_node(*index);
    }
    for (key, dependencies) in new_specs
        .iter()
        .chain(spec_updates)
        .map(|item| (format!("spec:{}", item.spec), &item.depends_on))
        .chain(
            direct_changes
                .iter()
                .map(|item| (format!("direct:{}", item.id), &item.depends_on)),
        )
    {
        let mut seen = BTreeSet::new();
        for dependency in dependencies {
            let target = dependency_key(dependency);
            if !seen.insert(target.clone()) {
                issues.push(issue(
                    "ROADMAP_DEPENDENCY_DUPLICATE",
                    "/work_items/depends_on",
                    format!("item {key} repeats dependency {target}"),
                ));
            } else if target == key {
                issues.push(issue(
                    "ROADMAP_DEPENDENCY_SELF",
                    "/work_items/depends_on",
                    format!("item {key} cannot depend on itself"),
                ));
            } else if let Some(target_index) = indices.get(&target) {
                graph.add_edge(*target_index, indices[&key], ());
            } else {
                issues.push(issue(
                    "ROADMAP_DEPENDENCY_MISSING",
                    "/work_items/depends_on",
                    format!("dependency {target} does not resolve"),
                ));
            }
        }
    }
    if is_cyclic_directed(&graph) {
        issues.push(issue(
            "ROADMAP_DEPENDENCY_CYCLE",
            "/work_items",
            "roadmap dependencies contain a cycle",
        ));
    }
}

fn normalized_items(items: &[SpecItem]) -> Vec<CrossSpecItem> {
    let mut items = items
        .iter()
        .map(|item| {
            let mut depends_on = item
                .depends_on
                .iter()
                .filter_map(|dependency| match dependency {
                    Dependency::Spec(value) => Some(value.clone()),
                    Dependency::Direct(_) => None,
                })
                .collect::<Vec<_>>();
            depends_on
                .sort_by(|left, right| left.spec.encode_utf16().cmp(right.spec.encode_utf16()));
            CrossSpecItem {
                spec: item.spec.clone(),
                summary: item.summary.clone(),
                depends_on,
            }
        })
        .collect::<Vec<_>>();
    items.sort_by(|left, right| left.spec.encode_utf16().cmp(right.spec.encode_utf16()));
    items
}

fn dependency_key(dependency: &Dependency) -> String {
    match dependency {
        Dependency::Spec(value) => format!("spec:{}", value.spec),
        Dependency::Direct(value) => format!("direct:{}", value.direct),
    }
}

fn validate_identity(value: &str, kind: &str, issues: &mut Vec<RoadmapIssue>) {
    let mut bytes = value.bytes();
    if value.len() > 64
        || !bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        || bytes.any(|byte| !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
        || value.ends_with('-')
        || value.contains("--")
    {
        issues.push(issue(
            "ROADMAP_ITEM_ID_INVALID",
            "/work_items",
            format!("{kind} identity must be lowercase kebab-case"),
        ));
    }
}

fn validate_summary(value: &str, identity: &str, issues: &mut Vec<RoadmapIssue>) {
    if value.trim().is_empty() || value.contains(['\r', '\n']) {
        issues.push(issue(
            "ROADMAP_ITEM_SUMMARY_INVALID",
            "/work_items",
            format!("item {identity} summary must be a non-empty single line"),
        ));
    }
}

fn valid_revision(value: &str) -> bool {
    matches!(value.len(), 40 | 64)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn split_frontmatter(content: &str) -> Result<&str, String> {
    split_frontmatter_parts(content).map(|(frontmatter, _)| frontmatter)
}

fn split_frontmatter_parts(content: &str) -> Result<(&str, &str), String> {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);
    let rest = normalized
        .strip_prefix("---\n")
        .or_else(|| normalized.strip_prefix("---\r\n"))
        .ok_or_else(|| "ROADMAP_FRONTMATTER_MISSING: expected opening ---".to_owned())?;
    let (end, marker_len) = rest
        .find("\n---\n")
        .map(|end| (end, "\n---\n".len()))
        .or_else(|| {
            rest.find("\r\n---\r\n")
                .map(|end| (end, "\r\n---\r\n".len()))
        })
        .ok_or_else(|| "ROADMAP_FRONTMATTER_MISSING: expected closing ---".to_owned())?;
    Ok((&rest[..end], &rest[end + marker_len..]))
}

fn single_issue(message: String) -> RoadmapIssues {
    RoadmapIssues {
        issues: vec![issue("ROADMAP_PARSE_FAILED", "/", message)],
    }
}

fn issue(code: &'static str, path: impl Into<String>, message: impl Into<String>) -> RoadmapIssue {
    RoadmapIssue {
        code,
        path: path.into(),
        message: message.into(),
    }
}
