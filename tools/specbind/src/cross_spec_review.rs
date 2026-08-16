//! Contract-first cross-spec review candidate and authoritative input resolution.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::artifacts::{Artifact, ArtifactInventory, ArtifactKind, resolve_gate_inputs};
use crate::contract_graph::{self, ContractGraphResolution, GraphIssueSeverity};
use crate::domain::spec::Spec;
use crate::fingerprint::Fingerprint;
use crate::freshness::{self, FreshnessStatus};
use crate::guarded_fs::{self, GuardedWriteError};
use crate::repository;
use crate::roadmap::{self, RoadmapDocument};
use crate::schema::{runtime, spec::v1::WorkflowState};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedReview {
    pub path: String,
    pub milestone_id: String,
    pub passed_at: String,
    pub input_revisions: BTreeMap<String, Fingerprint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewFreshnessStatus {
    NotRequired,
    Missing,
    Fresh,
    Stale,
    Invalid,
}

/// Later lifecycle boundaries that must recheck the accepted cross-spec review.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReviewBoundary<'a> {
    TasksApproval { canonical_spec: &'a str },
    ImplementationValidation { canonical_spec: &'a str },
    ReleasePreflight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedReviewRecord {
    pub milestone_id: String,
    pub passed_at: String,
    pub input_revisions: BTreeMap<String, String>,
    pub assessment: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewFreshnessReport {
    pub status: ReviewFreshnessStatus,
    pub accepted: Option<AcceptedReviewRecord>,
    pub current_input_revisions: Option<BTreeMap<String, Fingerprint>>,
    pub issues: Vec<ReviewIssue>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredReviewFrontmatter {
    #[serde(rename = "type")]
    artifact_type: String,
    milestone_id: String,
    passed_at: String,
    input_revisions: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct ReviewFrontmatter<'a> {
    #[serde(rename = "type")]
    artifact_type: &'static str,
    milestone_id: &'a str,
    passed_at: &'a str,
    input_revisions: OrderedRevisions<'a>,
}

struct OrderedRevisions<'a>(&'a BTreeMap<String, Fingerprint>);

impl Serialize for OrderedRevisions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut mapping = serializer.serialize_map(Some(self.0.len()))?;
        if let Some(fingerprint) = self.0.get(ROADMAP_KEY) {
            mapping.serialize_entry(ROADMAP_KEY, &fingerprint.to_string())?;
        }
        for (key, fingerprint) in self.0.iter().filter(|(key, _)| key.as_str() != ROADMAP_KEY) {
            mapping.serialize_entry(key, &fingerprint.to_string())?;
        }
        mapping.end()
    }
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
    resolve_candidate_inputs(specbind_root, candidate)
}

fn resolve_candidate_inputs(
    specbind_root: &Path,
    candidate: ReviewCandidate,
) -> Result<ReviewInputResolution, ReviewIssues> {
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

/// Revalidates every authoritative input and atomically writes the accepted
/// current cross-spec review artifact.
///
/// # Errors
///
/// Returns deterministic Git, lifecycle, freshness, race, serialization, or
/// filesystem diagnostics without changing accepted state on failure.
pub fn accept(
    project_root: &Path,
    specbind_root: &Path,
    candidate_json: &str,
) -> Result<AcceptedReview, ReviewIssues> {
    let initial = resolve_inputs(specbind_root, candidate_json)?;
    validate_acceptance_guards(project_root, specbind_root, &initial)?;
    let current = resolve_inputs(specbind_root, candidate_json)?;
    validate_acceptance_guards(project_root, specbind_root, &current)?;
    if initial.roadmap.milestone_id != current.roadmap.milestone_id
        || initial.input_revisions != current.input_revisions
    {
        return Err(one_review_issue(
            "CROSS_SPEC_REVIEW_INPUTS_CHANGED",
            None,
            "review inputs changed during guarded acceptance",
        ));
    }
    let passed_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| {
            one_review_issue(
                "CROSS_SPEC_REVIEW_TIMESTAMP_FAILED",
                None,
                error.to_string(),
            )
        })?;
    let content = render_review(&current, &passed_at)?;
    let relative = "state/cross-spec-review.md";
    persist_review(specbind_root, relative, content.as_bytes())?;
    Ok(AcceptedReview {
        path: relative.to_owned(),
        milestone_id: current.roadmap.milestone_id,
        passed_at,
        input_revisions: current.input_revisions,
    })
}

/// Reads the accepted review, reconstructs its declared deep inputs, and
/// compares the persisted revisions with the current authoritative inputs.
#[must_use]
pub fn evaluate_freshness(project_root: &Path, specbind_root: &Path) -> ReviewFreshnessReport {
    let roadmap = match read_current_roadmap(specbind_root) {
        Ok(roadmap) => roadmap,
        Err(error) => {
            return freshness_report(ReviewFreshnessStatus::Invalid, None, None, error.issues);
        }
    };
    let relative = "state/cross-spec-review.md";
    let accepted = match read_accepted_review(specbind_root, &roadmap, relative) {
        Ok(accepted) => accepted,
        Err(report) => return *report,
    };

    let deep_inputs = accepted
        .input_revisions
        .keys()
        .filter(|key| parse_deep_selector(key).is_some())
        .cloned()
        .collect();
    let candidate = ReviewCandidate {
        schema_version: 1,
        assessment: accepted.assessment.clone(),
        deep_inputs,
    };
    let current = match resolve_candidate_inputs(specbind_root, candidate) {
        Ok(current) => current,
        Err(error) => {
            return freshness_report(
                ReviewFreshnessStatus::Stale,
                Some(accepted),
                None,
                error.issues,
            );
        }
    };
    let current_revisions = current.input_revisions;
    let mut issues = Vec::new();
    validate_baseline(project_root, &roadmap.baseline_revision, &mut issues);
    if accepted.milestone_id != roadmap.milestone_id {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_MILESTONE_STALE",
            Some(relative.to_owned()),
            "accepted review milestone_id does not match the current Roadmap",
        ));
    }
    if accepted.input_revisions.len() != current_revisions.len()
        || accepted.input_revisions.iter().any(|(key, persisted)| {
            current_revisions
                .get(key)
                .is_none_or(|current| persisted != &current.to_string())
        })
    {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_INPUTS_STALE",
            Some(relative.to_owned()),
            "accepted input_revisions do not match the current authoritative review inputs",
        ));
    }
    issues.sort();
    issues.dedup();
    let status = if issues.is_empty() {
        ReviewFreshnessStatus::Fresh
    } else {
        ReviewFreshnessStatus::Stale
    };
    freshness_report(status, Some(accepted), Some(current_revisions), issues)
}

/// Requires the accepted cross-spec review state for a later lifecycle boundary.
///
/// Tasks approval and implementation validation require a canonical participating
/// Spec ID and a fresh review. Release preflight accepts `NotRequired` only for a
/// Direct-only Roadmap.
///
/// # Errors
///
/// Returns the authoritative freshness diagnostics plus a stable boundary code
/// when the requested lifecycle boundary is blocked.
pub fn require_for_boundary(
    project_root: &Path,
    specbind_root: &Path,
    boundary: ReviewBoundary<'_>,
) -> Result<ReviewFreshnessReport, ReviewIssues> {
    let canonical_spec = boundary.canonical_spec();
    let mut issues = canonical_spec
        .filter(|spec| !valid_id(spec))
        .map(|spec| {
            review_issue(
                "CROSS_SPEC_REVIEW_SPEC_TARGET_INVALID",
                Some(format!("specs/{spec}")),
                "later lifecycle review guard requires a canonical Spec ID",
            )
        })
        .into_iter()
        .collect::<Vec<_>>();
    let report = evaluate_freshness(project_root, specbind_root);
    issues.extend(report.issues.iter().cloned());

    if issues.is_empty()
        && let Some(canonical_spec) = canonical_spec
    {
        match read_current_roadmap(specbind_root) {
            Ok(roadmap) if !roadmap.spec_ids().iter().any(|spec| spec == canonical_spec) => {
                issues.push(review_issue(
                    "CROSS_SPEC_REVIEW_SPEC_NOT_IN_MILESTONE",
                    Some(format!("specs/{canonical_spec}")),
                    "later lifecycle review guard requires a current Spec-backed Roadmap participant",
                ));
            }
            Ok(_) => {}
            Err(error) => issues.extend(error.issues),
        }
    }

    let accepted = match boundary {
        ReviewBoundary::TasksApproval { .. } | ReviewBoundary::ImplementationValidation { .. } => {
            report.status == ReviewFreshnessStatus::Fresh
        }
        ReviewBoundary::ReleasePreflight => matches!(
            report.status,
            ReviewFreshnessStatus::Fresh | ReviewFreshnessStatus::NotRequired
        ),
    };
    if !accepted {
        issues.push(review_issue(
            boundary.blocked_code(),
            canonical_spec.map(|spec| format!("specs/{spec}")),
            boundary.blocked_message(),
        ));
    }
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(report)
    } else {
        Err(ReviewIssues { issues })
    }
}

impl<'a> ReviewBoundary<'a> {
    fn canonical_spec(self) -> Option<&'a str> {
        match self {
            Self::TasksApproval { canonical_spec }
            | Self::ImplementationValidation { canonical_spec } => Some(canonical_spec),
            Self::ReleasePreflight => None,
        }
    }

    fn blocked_code(self) -> &'static str {
        match self {
            Self::TasksApproval { .. } => "CROSS_SPEC_REVIEW_TASKS_APPROVAL_BLOCKED",
            Self::ImplementationValidation { .. } => {
                "CROSS_SPEC_REVIEW_IMPLEMENTATION_VALIDATION_BLOCKED"
            }
            Self::ReleasePreflight => "CROSS_SPEC_REVIEW_RELEASE_PREFLIGHT_BLOCKED",
        }
    }

    fn blocked_message(self) -> &'static str {
        match self {
            Self::TasksApproval { .. } => {
                "Tasks approval requires a fresh accepted cross-spec review"
            }
            Self::ImplementationValidation { .. } => {
                "implementation validation requires a fresh accepted cross-spec review"
            }
            Self::ReleasePreflight => {
                "release preflight requires a fresh review for Spec-backed work or no review for Direct-only work"
            }
        }
    }
}

fn read_accepted_review(
    specbind_root: &Path,
    roadmap: &RoadmapDocument,
    relative: &str,
) -> Result<AcceptedReviewRecord, Box<ReviewFreshnessReport>> {
    let path = specbind_root.join(relative);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let status = if roadmap.spec_ids().is_empty() {
                ReviewFreshnessStatus::NotRequired
            } else {
                ReviewFreshnessStatus::Missing
            };
            let issues = (status == ReviewFreshnessStatus::Missing)
                .then(|| {
                    review_issue(
                        "CROSS_SPEC_REVIEW_MISSING",
                        Some(relative.to_owned()),
                        "Spec-backed milestone requires an accepted cross-spec review",
                    )
                })
                .into_iter()
                .collect();
            return Err(Box::new(freshness_report(status, None, None, issues)));
        }
        Err(error) => {
            return Err(Box::new(invalid_read_report(relative, error.to_string())));
        }
    };
    if roadmap.spec_ids().is_empty() {
        return Err(Box::new(freshness_report(
            ReviewFreshnessStatus::Invalid,
            None,
            None,
            vec![review_issue(
                "CROSS_SPEC_REVIEW_UNEXPECTED_FOR_DIRECT_ONLY",
                Some(relative.to_owned()),
                "Direct-only milestone must not retain an accepted cross-spec review",
            )],
        )));
    }
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(Box::new(freshness_report(
            ReviewFreshnessStatus::Invalid,
            None,
            None,
            vec![review_issue(
                "CROSS_SPEC_REVIEW_TARGET_INVALID",
                Some(relative.to_owned()),
                "accepted review must be a regular non-symlink file",
            )],
        )));
    }
    let content = fs::read_to_string(path)
        .map_err(|error| Box::new(invalid_read_report(relative, error.to_string())))?;
    parse_accepted_review(&content, relative).map_err(|error| {
        Box::new(freshness_report(
            ReviewFreshnessStatus::Invalid,
            None,
            None,
            error.issues,
        ))
    })
}

fn invalid_read_report(relative: &str, message: String) -> ReviewFreshnessReport {
    freshness_report(
        ReviewFreshnessStatus::Invalid,
        None,
        None,
        vec![review_issue(
            "CROSS_SPEC_REVIEW_READ_FAILED",
            Some(relative.to_owned()),
            message,
        )],
    )
}

fn freshness_report(
    status: ReviewFreshnessStatus,
    accepted: Option<AcceptedReviewRecord>,
    current_input_revisions: Option<BTreeMap<String, Fingerprint>>,
    issues: Vec<ReviewIssue>,
) -> ReviewFreshnessReport {
    ReviewFreshnessReport {
        status,
        accepted,
        current_input_revisions,
        issues,
    }
}

fn read_current_roadmap(specbind_root: &Path) -> Result<RoadmapDocument, ReviewIssues> {
    let mut issues = Vec::new();
    let bytes = read_regular(
        &specbind_root.join("steering/roadmap.md"),
        ROADMAP_KEY,
        &mut issues,
    );
    let Some(bytes) = bytes else {
        return Err(ReviewIssues { issues });
    };
    let content = std::str::from_utf8(&bytes).map_err(|error| {
        one_review_issue(
            "CROSS_SPEC_REVIEW_ROADMAP_NOT_UTF8",
            Some(ROADMAP_KEY.to_owned()),
            error.to_string(),
        )
    })?;
    roadmap::parse(content).map_err(|error| ReviewIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| review_issue(value.code, Some(ROADMAP_KEY.to_owned()), value.message))
            .collect(),
    })
}

fn parse_accepted_review(
    content: &str,
    source: &str,
) -> Result<AcceptedReviewRecord, ReviewIssues> {
    let (frontmatter, body) = split_review_frontmatter(content).map_err(|message| {
        one_review_issue(
            "CROSS_SPEC_REVIEW_FRONTMATTER_INVALID",
            Some(source.to_owned()),
            message,
        )
    })?;
    let raw = serde_saphyr::from_str::<StoredReviewFrontmatter>(frontmatter).map_err(|error| {
        one_review_issue(
            "CROSS_SPEC_REVIEW_FRONTMATTER_INVALID",
            Some(source.to_owned()),
            error.to_string(),
        )
    })?;
    let mut issues = Vec::new();
    if raw.artifact_type != "SpecBind Cross-Spec Review" {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_TYPE_INVALID",
            Some(source.to_owned()),
            "type must be SpecBind Cross-Spec Review",
        ));
    }
    if Uuid::parse_str(&raw.milestone_id).map_or(true, |id| {
        id.get_version_num() != 7 || id.hyphenated().to_string() != raw.milestone_id
    }) {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_MILESTONE_ID_INVALID",
            Some(source.to_owned()),
            "milestone_id must be a canonical UUID v7",
        ));
    }
    if OffsetDateTime::parse(&raw.passed_at, &Rfc3339).is_err() {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_PASSED_AT_INVALID",
            Some(source.to_owned()),
            "passed_at must be a timezone-qualified RFC 3339 timestamp",
        ));
    }
    if body.trim().is_empty() {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_ASSESSMENT_EMPTY",
            Some(source.to_owned()),
            "accepted review must contain a non-empty Markdown body",
        ));
    }
    if !raw.input_revisions.contains_key(ROADMAP_KEY) {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_ROADMAP_INPUT_MISSING",
            Some(source.to_owned()),
            "input_revisions must include the Roadmap cross-spec scope",
        ));
    }
    for (key, value) in &raw.input_revisions {
        if key != ROADMAP_KEY
            && parse_contract_selector(key).is_none()
            && parse_deep_selector(key).is_none()
        {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_INPUT_KEY_INVALID",
                Some(key.clone()),
                "input revision key is not a canonical cross-spec review selector",
            ));
        }
        if !valid_fingerprint(value) {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_FINGERPRINT_INVALID",
                Some(key.clone()),
                "input revision must use sha256: followed by 64 lowercase hexadecimal characters",
            ));
        }
    }
    if issues.is_empty() {
        Ok(AcceptedReviewRecord {
            milestone_id: raw.milestone_id,
            passed_at: raw.passed_at,
            input_revisions: raw.input_revisions,
            assessment: body.to_owned(),
        })
    } else {
        issues.sort();
        issues.dedup();
        Err(ReviewIssues { issues })
    }
}

fn split_review_frontmatter(content: &str) -> Result<(&str, &str), String> {
    let mut offset = 0;
    let mut lines = content.split_inclusive('\n');
    let first = lines
        .next()
        .ok_or_else(|| "accepted review is empty".to_owned())?;
    if line_content(first) != "---" {
        return Err("frontmatter must begin with --- on the first line".to_owned());
    }
    offset += first.len();
    let frontmatter_start = offset;
    for line in lines {
        if line_content(line) == "---" {
            return Ok((
                &content[frontmatter_start..offset],
                &content[offset + line.len()..],
            ));
        }
        offset += line.len();
    }
    Err("frontmatter closing --- delimiter is missing".to_owned())
}

fn line_content(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

fn parse_contract_selector(selector: &str) -> Option<&str> {
    let rest = selector.strip_prefix("specs/")?;
    let spec = rest.strip_suffix("#contract")?;
    valid_id(spec).then_some(spec)
}

fn valid_fingerprint(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn validate_acceptance_guards(
    project_root: &Path,
    specbind_root: &Path,
    resolution: &ReviewInputResolution,
) -> Result<(), ReviewIssues> {
    let mut issues = Vec::new();
    validate_baseline(
        project_root,
        &resolution.roadmap.baseline_revision,
        &mut issues,
    );
    for spec in resolution.roadmap.spec_ids() {
        validate_participating_spec(
            specbind_root,
            &spec,
            &resolution.roadmap.milestone_id,
            &mut issues,
        );
    }
    if issues.is_empty() {
        Ok(())
    } else {
        issues.sort();
        issues.dedup();
        Err(ReviewIssues { issues })
    }
}

fn validate_baseline(project_root: &Path, baseline: &str, issues: &mut Vec<ReviewIssue>) {
    let resolved = git_output(
        project_root,
        &["rev-parse", "--verify", &format!("{baseline}^{{commit}}")],
    );
    match resolved {
        Ok(value) if value.trim() == baseline => {}
        Ok(_) => issues.push(review_issue(
            "CROSS_SPEC_REVIEW_BASELINE_NOT_EXACT",
            Some(ROADMAP_KEY.to_owned()),
            "baseline_revision must resolve to the same full commit object ID",
        )),
        Err(message) => {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_BASELINE_MISSING",
                Some(ROADMAP_KEY.to_owned()),
                message,
            ));
            return;
        }
    }
    match git_status(
        project_root,
        &["merge-base", "--is-ancestor", baseline, "HEAD"],
    ) {
        Ok(true) => {}
        Ok(false) => issues.push(review_issue(
            "CROSS_SPEC_REVIEW_BASELINE_NOT_ANCESTOR",
            Some(ROADMAP_KEY.to_owned()),
            "baseline_revision is not an ancestor of current HEAD",
        )),
        Err(message) => issues.push(review_issue(
            "CROSS_SPEC_REVIEW_GIT_FAILED",
            Some(ROADMAP_KEY.to_owned()),
            message,
        )),
    }
}

fn validate_participating_spec(
    specbind_root: &Path,
    canonical_spec: &str,
    milestone_id: &str,
    issues: &mut Vec<ReviewIssue>,
) {
    let source = format!("specs/{canonical_spec}/spec.yaml");
    let Some(bytes) = read_regular(&specbind_root.join(&source), &source, issues) else {
        return;
    };
    let input = match std::str::from_utf8(&bytes) {
        Ok(input) => input,
        Err(error) => {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_SPEC_NOT_UTF8",
                Some(source),
                error.to_string(),
            ));
            return;
        }
    };
    let wire = match runtime::load_spec(input) {
        Ok(wire) => wire,
        Err(error) => {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_SPEC_INVALID",
                Some(source),
                error.to_string(),
            ));
            return;
        }
    };
    let spec = match Spec::try_from(wire) {
        Ok(spec) => spec,
        Err(error) => {
            issues.push(review_issue(
                "CROSS_SPEC_REVIEW_SPEC_INVALID",
                Some(source),
                error.to_string(),
            ));
            return;
        }
    };
    let Some(active) = spec.as_wire().active_change.0.as_ref() else {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_SPEC_STATE_INVALID",
            Some(source),
            "participating spec must have an active change in tasks state",
        ));
        return;
    };
    if active.milestone_id.0 != milestone_id || active.state != WorkflowState::Tasks {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_SPEC_STATE_INVALID",
            Some(source.clone()),
            "participating spec must match the Roadmap milestone and be in tasks state",
        ));
    }
    let gate_inputs = resolve_gate_inputs(specbind_root, canonical_spec);
    for value in gate_inputs.inventory.issues {
        issues.push(review_issue(
            value.code,
            value.path.map(|path| path.to_string()),
            value.message,
        ));
    }
    let freshness = freshness::evaluate(&spec, &gate_inputs.inputs);
    if freshness.design.status != FreshnessStatus::Fresh {
        issues.push(review_issue(
            "CROSS_SPEC_REVIEW_DESIGN_NOT_FRESH",
            Some(source),
            "participating spec requires a fresh Design gate",
        ));
    }
    let tasks = specbind_root.join(format!("specs/{canonical_spec}/tasks.yaml"));
    match fs::symlink_metadata(tasks) {
        Ok(_) => issues.push(review_issue(
            "CROSS_SPEC_REVIEW_TASKS_ALREADY_EXIST",
            Some(format!("specs/{canonical_spec}/tasks.yaml")),
            "tasks.yaml must not exist before cross-spec review acceptance",
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => issues.push(review_issue(
            "CROSS_SPEC_REVIEW_TASKS_INSPECT_FAILED",
            Some(format!("specs/{canonical_spec}/tasks.yaml")),
            error.to_string(),
        )),
    }
}

fn render_review(
    resolution: &ReviewInputResolution,
    passed_at: &str,
) -> Result<String, ReviewIssues> {
    let frontmatter = ReviewFrontmatter {
        artifact_type: "SpecBind Cross-Spec Review",
        milestone_id: &resolution.roadmap.milestone_id,
        passed_at,
        input_revisions: OrderedRevisions(&resolution.input_revisions),
    };
    let yaml = serde_saphyr::to_string(&frontmatter).map_err(|error| {
        one_review_issue(
            "CROSS_SPEC_REVIEW_SERIALIZE_FAILED",
            None,
            error.to_string(),
        )
    })?;
    let mut content = format!("---\n{yaml}---\n{}", resolution.candidate.assessment);
    if !content.ends_with('\n') {
        content.push('\n');
    }
    Ok(content)
}

fn persist_review(specbind_root: &Path, relative: &str, bytes: &[u8]) -> Result<(), ReviewIssues> {
    let state = specbind_root.join("state");
    match fs::symlink_metadata(&state) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(one_review_issue(
                "CROSS_SPEC_REVIEW_STATE_DIR_INVALID",
                Some("state".to_owned()),
                "state must be a regular non-symlink directory",
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(&state).map_err(|error| {
                one_review_issue(
                    "CROSS_SPEC_REVIEW_STATE_DIR_CREATE_FAILED",
                    Some("state".to_owned()),
                    error.to_string(),
                )
            })?;
        }
        Err(error) => {
            return Err(one_review_issue(
                "CROSS_SPEC_REVIEW_STATE_DIR_INVALID",
                Some("state".to_owned()),
                error.to_string(),
            ));
        }
    }
    let target = specbind_root.join(relative);
    guarded_fs::replace_optional(&target, bytes).map_err(|error| match error {
        GuardedWriteError::InvalidTarget(_) => one_review_issue(
            "CROSS_SPEC_REVIEW_TARGET_INVALID",
            Some(relative.to_owned()),
            "accepted review target must be absent or a regular non-symlink file",
        ),
        _ => one_review_issue(
            "CROSS_SPEC_REVIEW_WRITE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        ),
    })?;
    Ok(())
}

fn git_output(project_root: &Path, arguments: &[&str]) -> Result<String, String> {
    repository::output(project_root, arguments).map_err(|error| error.to_string())
}

fn git_status(project_root: &Path, arguments: &[&str]) -> Result<bool, String> {
    repository::predicate(project_root, arguments).map_err(|error| error.to_string())
}

fn one_review_issue(
    code: &'static str,
    source: Option<String>,
    message: impl Into<String>,
) -> ReviewIssues {
    ReviewIssues {
        issues: vec![review_issue(code, source, message)],
    }
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
