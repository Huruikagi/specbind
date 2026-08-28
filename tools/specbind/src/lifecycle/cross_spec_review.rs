//! Lifecycle service for Contract-first review and authoritative input resolution.

use std::{collections::BTreeMap, fmt, path::Path};

use serde::Deserialize;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::{
    contract_graph::ContractGraphResolution, fingerprint::Fingerprint, roadmap::RoadmapDocument,
};

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

/// Later lifecycle boundaries that must recheck the accepted contract review.
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
    /// Current Roadmap milestone identity, absent only when no trustworthy
    /// active Roadmap could be parsed.
    pub milestone_id: Option<String>,
    pub accepted: Option<AcceptedReviewRecord>,
    pub current_input_revisions: Option<BTreeMap<String, Fingerprint>>,
    pub issues: Vec<ReviewIssue>,
}

impl ReviewFreshnessReport {
    fn with_milestone(mut self, milestone_id: String) -> Self {
        self.milestone_id = Some(milestone_id);
        self
    }
}

mod accepted_state;
mod freshness;
mod guard;
mod resolution;

pub use accepted_state::remove_accepted;
pub use freshness::{evaluate_freshness, require_for_boundary};
pub use resolution::resolve_inputs;

use accepted_state::{persist_review, render_review};
use guard::validate_acceptance_guards;

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
            "contract review has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for ReviewIssues {}

/// Revalidates every authoritative input and atomically writes the accepted
/// current contract review artifact.
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
            "CONTRACT_REVIEW_INPUTS_CHANGED",
            None,
            "review inputs changed during guarded acceptance",
        ));
    }
    let passed_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .map_err(|error| {
            one_review_issue("CONTRACT_REVIEW_TIMESTAMP_FAILED", None, error.to_string())
        })?;
    let content = render_review(&current, &passed_at)?;
    let relative = "state/contract-review.md";
    persist_review(specbind_root, relative, content.as_bytes())?;
    Ok(AcceptedReview {
        path: relative.to_owned(),
        milestone_id: current.roadmap.milestone_id,
        passed_at,
        input_revisions: current.input_revisions,
    })
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
            Self::TasksApproval { .. } => "CONTRACT_REVIEW_TASKS_APPROVAL_BLOCKED",
            Self::ImplementationValidation { .. } => {
                "CONTRACT_REVIEW_IMPLEMENTATION_VALIDATION_BLOCKED"
            }
            Self::ReleasePreflight => "CONTRACT_REVIEW_RELEASE_PREFLIGHT_BLOCKED",
        }
    }

    fn blocked_message(self) -> &'static str {
        match self {
            Self::TasksApproval { .. } => {
                "Tasks approval requires a fresh accepted contract review"
            }
            Self::ImplementationValidation { .. } => {
                "implementation validation requires a fresh accepted contract review"
            }
            Self::ReleasePreflight => {
                "release preflight requires a fresh review for Spec-backed work or no review for Direct-only work"
            }
        }
    }
}

pub(super) fn one_review_issue(
    code: &'static str,
    source: Option<String>,
    message: impl Into<String>,
) -> ReviewIssues {
    ReviewIssues {
        issues: vec![review_issue(code, source, message)],
    }
}

pub(super) fn review_issue(
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
