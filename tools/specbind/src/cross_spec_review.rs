//! Contract-first cross-spec review candidate and authoritative input resolution.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::artifacts::{Artifact, ArtifactInventory, ArtifactKind};
use crate::contract_graph::{self, ContractGraphResolution, GraphIssueSeverity};
use crate::fingerprint::Fingerprint;
use crate::roadmap::{self, RoadmapDocument};

const ROADMAP_KEY: &str = "steering/roadmap.md#cross-spec-scope";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReviewCandidate {
    pub schema_version: u64,
    pub assessment: String,
    pub deep_inputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewInputResolution {
    pub roadmap: RoadmapDocument,
    pub graph: ContractGraphResolution,
    pub candidate: ReviewCandidate,
    pub input_revisions: BTreeMap<String, Fingerprint>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReviewIssue {
    pub code: &'static str,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewIssues {
    pub issues: Vec<ReviewIssue>,
}

impl fmt::Display for ReviewIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cross-spec review has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for ReviewIssues {}

/// Resolves the current Roadmap, every persistent Contract, and declared deep
/// inputs without accepting caller-supplied paths or fingerprints.
///
/// # Errors
///
/// Returns all deterministic candidate, discovery, graph, and input diagnostics.
pub fn resolve_inputs(
    specbind_root: &Path,
    candidate_json: &str,
) -> Result<ReviewInputResolution, ReviewIssues> {
    let candidate = parse_candidate(candidate_json)?;
    let mut issues = Vec::new();
    let roadmap_path = specbind_root.join("steering/roadmap.md");
    let roadmap_bytes = read_regular(&roadmap_path, ROADMAP_KEY, &mut issues);
    let roadmap = roadmap_bytes
        .as_deref()
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .and_then(|content| match roadmap::parse(content) {
            Ok(roadmap) => Some(roadmap),
            Err(error) => {
                issues.extend(error.issues.into_iter().map(|value| {
                    review_issue(value.code, Some(ROADMAP_KEY.to_owned()), value.message)
                }));
                None
            }
        });
    if roadmap_bytes
        .as_deref()
        .is_some_and(|bytes| std::str::from_utf8(bytes).is_err())
    {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_ROADMAP_NOT_UTF8",
            Some(ROADMAP_KEY.to_owned()),
            "Roadmap must be UTF-8",
        ));
    }

    let graph = contract_graph::resolve(specbind_root);
    collect_graph_issues(&graph, &mut issues);
    let mut input_revisions = BTreeMap::new();
    if let (Some(roadmap), Some(_bytes)) = (&roadmap, roadmap_bytes) {
        if roadmap.spec_ids().is_empty() {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_DIRECT_ONLY",
                Some(ROADMAP_KEY.to_owned()),
                "Direct-only milestones do not accept a cross-spec review",
            ));
        }
        for spec in roadmap.spec_ids() {
            if !graph.inventories.contains_key(&spec) {
                issues.push(review_issue(
                    "CROSS_SPEC_REVIEW_ROADMAP_SPEC_MISSING",
                    Some(ROADMAP_KEY.to_owned()),
                    format!("Roadmap spec {spec} does not exist in the persistent Spec set"),
                ));
            }
        }
        match Fingerprint::roadmap_cross_spec_scope(roadmap) {
            Ok(fingerprint) => {
                input_revisions.insert(ROADMAP_KEY.to_owned(), fingerprint);
            }
            Err(error) => issues.push(review_issue(
                "CROSS_SPEC_REVIEW_ROADMAP_FINGERPRINT_FAILED",
                Some(ROADMAP_KEY.to_owned()),
                error.to_string(),
            )),
        }
    }

    collect_contract_revisions(specbind_root, &graph, &mut input_revisions, &mut issues);

    let mut deep_seen = BTreeSet::new();
    for selector in &candidate.deep_inputs {
        if !deep_seen.insert(selector) {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_DEEP_INPUT_DUPLICATE",
                Some(selector.clone()),
                "deepInputs must not contain duplicates",
            ));
            continue;
        }
        resolve_deep_input(
            specbind_root,
            selector,
            &graph.inventories,
            &mut input_revisions,
            &mut issues,
        );
    }

    if issues.is_empty()
        && let Some(roadmap) = roadmap
    {
        return Ok(ReviewInputResolution {
            roadmap,
            graph,
            candidate,
            input_revisions,
        });
    }
    if issues.is_empty() {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_ROADMAP_UNAVAILABLE",
            Some(ROADMAP_KEY.to_owned()),
            "current Roadmap could not be resolved",
        ));
    }
    issues.sort();
    issues.dedup();
    Err(ReviewIssues { issues })
}

fn collect_contract_revisions(
    specbind_root: &Path,
    graph: &ContractGraphResolution,
    revisions: &mut BTreeMap<String, Fingerprint>,
    issues: &mut Vec<ReviewIssue>,
) {
    for (spec, inventory) in &graph.inventories {
        let key = format!("specs/{spec}#contract");
        if graph.report.contracts.contains_key(spec)
            && let Some(artifact) = inventory
                .artifacts
                .iter()
                .find(|artifact| artifact.kind == ArtifactKind::Contract)
            && let Some(bytes) = read_artifact(specbind_root, artifact, &key, issues)
        {
            revisions.insert(key, Fingerprint::markdown(&bytes));
        }
    }
}

fn parse_candidate(input: &str) -> Result<ReviewCandidate, ReviewIssues> {
    let candidate =
        serde_json::from_str::<ReviewCandidate>(input).map_err(|error| ReviewIssues {
            issues: vec![review_issue(
                "CROSS_SPEC_REVIEW_CANDIDATE_INVALID",
                None,
                error.to_string(),
            )],
        })?;
    let mut issues = Vec::new();
    if candidate.schema_version != 1 {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_CANDIDATE_VERSION_UNSUPPORTED",
            None,
            "schemaVersion must be 1",
        ));
    }
    if candidate.assessment.trim().is_empty() {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_ASSESSMENT_EMPTY",
            None,
            "assessment must contain non-empty Markdown",
        ));
    }
    if issues.is_empty() {
        Ok(candidate)
    } else {
        Err(ReviewIssues { issues })
    }
}

fn collect_graph_issues(graph: &ContractGraphResolution, issues: &mut Vec<ReviewIssue>) {
    issues.extend(graph.project_issues.iter().map(|value| {
        review_issue(
            value.code,
            value.path.as_ref().map(ToString::to_string),
            value.message.clone(),
        )
    }));
    for (spec, inventory) in &graph.inventories {
        issues.extend(inventory.issues.iter().map(|value| {
            review_issue(
                value.code,
                value.path.as_ref().map(ToString::to_string),
                format!("spec {spec}: {}", value.message),
            )
        }));
    }
    issues.extend(
        graph
            .report
            .issues
            .iter()
            .filter(|value| value.severity == GraphIssueSeverity::Error)
            .map(|value| review_issue(value.code, value.source.clone(), value.message.clone())),
    );
}

fn resolve_deep_input(
    specbind_root: &Path,
    selector: &str,
    inventories: &BTreeMap<String, ArtifactInventory>,
    revisions: &mut BTreeMap<String, Fingerprint>,
    issues: &mut Vec<ReviewIssue>,
) {
    let Some((spec, logical_selector)) = parse_deep_selector(selector) else {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_DEEP_INPUT_INVALID",
            Some(selector.to_owned()),
            "deep input must be specs/<spec>#requirements or specs/<spec>#design/<artifact-id>",
        ));
        return;
    };
    let Some(inventory) = inventories.get(spec) else {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_DEEP_INPUT_SPEC_MISSING",
            Some(selector.to_owned()),
            format!("deep input spec {spec} does not exist"),
        ));
        return;
    };
    let Some(artifact) = inventory
        .artifacts
        .iter()
        .find(|artifact| artifact.selector == logical_selector)
    else {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_DEEP_INPUT_MISSING",
            Some(selector.to_owned()),
            "deep input selector does not resolve uniquely",
        ));
        return;
    };
    if inventory
        .issues
        .iter()
        .any(|issue| issue.path.as_ref() == Some(&artifact.path))
    {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_DEEP_INPUT_INVALID_ARTIFACT",
            Some(selector.to_owned()),
            "deep input artifact has profile or content diagnostics",
        ));
        return;
    }
    if let Some(bytes) = read_artifact(specbind_root, artifact, selector, issues) {
        revisions.insert(selector.to_owned(), Fingerprint::markdown(&bytes));
    }
}

fn parse_deep_selector(selector: &str) -> Option<(&str, String)> {
    let rest = selector.strip_prefix("specs/")?;
    let (spec, artifact) = rest.split_once('#')?;
    if !valid_id(spec) {
        return None;
    }
    if artifact == "requirements" {
        Some((spec, "requirements".to_owned()))
    } else {
        let id = artifact.strip_prefix("design/")?;
        valid_id(id).then(|| (spec, format!("design/{id}")))
    }
}

fn read_artifact(
    specbind_root: &Path,
    artifact: &Artifact,
    source: &str,
    issues: &mut Vec<ReviewIssue>,
) -> Option<Vec<u8>> {
    read_regular(
        &specbind_root.join(artifact.path.as_std_path()),
        source,
        issues,
    )
}

fn read_regular(path: &Path, source: &str, issues: &mut Vec<ReviewIssue>) -> Option<Vec<u8>> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_INPUT_NOT_REGULAR",
                Some(source.to_owned()),
                "review input must be a regular non-symlink file",
            ));
            None
        }
        Ok(_) => match fs::read(path) {
            Ok(bytes) => Some(bytes),
            Err(error) => {
                issues.push(review_issue(
                    "CROSS_SPEC_REVIEW_INPUT_READ_FAILED",
                    Some(source.to_owned()),
                    error.to_string(),
                ));
                None
            }
        },
        Err(error) => {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_INPUT_READ_FAILED",
                Some(source.to_owned()),
                error.to_string(),
            ));
            None
        }
    }
}

fn valid_id(value: &str) -> bool {
    let mut bytes = value.bytes();
    value.len() <= 64
        && bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.ends_with('-')
        && !value.contains("--")
}

fn review_issue(
    code: &'static str,
    source: Option<String>,
    message: impl Into<String>,
) -> ReviewIssue {
    ReviewIssue {
        code,
        source,
        message: message.into(),
    }
}
