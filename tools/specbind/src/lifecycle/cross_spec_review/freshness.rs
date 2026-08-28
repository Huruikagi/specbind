use std::{collections::BTreeMap, path::Path};

use crate::{
    fingerprint::Fingerprint,
    roadmap::{self, RoadmapDocument},
};

use super::{
    AcceptedReviewRecord, ROADMAP_KEY, ReviewBoundary, ReviewCandidate, ReviewFreshnessReport,
    ReviewFreshnessStatus, ReviewIssue, ReviewIssues,
    accepted_state::read_accepted_review,
    guard::validate_baseline,
    one_review_issue,
    resolution::{parse_deep_selector, read_regular, resolve_candidate_inputs, valid_id},
    review_issue,
};

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
    let milestone_id = roadmap.milestone_id.clone();
    let relative = "state/contract-review.md";
    let accepted = match read_accepted_review(specbind_root, &roadmap, relative) {
        Ok(accepted) => accepted,
        Err(report) => return (*report).with_milestone(milestone_id),
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
            )
            .with_milestone(milestone_id);
        }
    };
    let current_revisions = current.input_revisions;
    let mut issues = Vec::new();
    validate_baseline(project_root, &roadmap.baseline_revision, &mut issues);
    if accepted.milestone_id != roadmap.milestone_id {
        issues.push(review_issue(
            "CONTRACT_REVIEW_MILESTONE_STALE",
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
            "CONTRACT_REVIEW_INPUTS_STALE",
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
        .with_milestone(milestone_id)
}

/// Requires the accepted contract review state for a later lifecycle boundary.
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
                "CONTRACT_REVIEW_SPEC_TARGET_INVALID",
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
                    "CONTRACT_REVIEW_SPEC_NOT_IN_MILESTONE",
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

pub(super) fn invalid_read_report(relative: &str, message: String) -> ReviewFreshnessReport {
    freshness_report(
        ReviewFreshnessStatus::Invalid,
        None,
        None,
        vec![review_issue(
            "CONTRACT_REVIEW_READ_FAILED",
            Some(relative.to_owned()),
            message,
        )],
    )
}

pub(super) fn freshness_report(
    status: ReviewFreshnessStatus,
    accepted: Option<AcceptedReviewRecord>,
    current_input_revisions: Option<BTreeMap<String, Fingerprint>>,
    issues: Vec<ReviewIssue>,
) -> ReviewFreshnessReport {
    ReviewFreshnessReport {
        status,
        milestone_id: None,
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
            "CONTRACT_REVIEW_ROADMAP_NOT_UTF8",
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
