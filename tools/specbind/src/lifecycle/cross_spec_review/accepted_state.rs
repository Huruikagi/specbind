use std::{collections::BTreeMap, fs, path::Path};

use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::{
    guarded_fs::{self, GuardedWriteError},
    roadmap::RoadmapDocument,
};

use super::{
    AcceptedReviewRecord, Fingerprint, ROADMAP_KEY, ReviewFreshnessReport, ReviewFreshnessStatus,
    ReviewInputResolution, ReviewIssues,
    freshness::{freshness_report, invalid_read_report},
    one_review_issue,
    resolution::{parse_deep_selector, valid_id},
    review_issue,
};

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

/// Removes the accepted contract review artifact.
///
/// Explicit Design and scope rewinds own this removal under Decision 0078; the
/// review is milestone-level state, so no per-Spec condition applies.
///
/// # Errors
///
/// Returns target-type or filesystem diagnostics. An absent artifact is not an
/// error and reports that nothing was removed.
pub fn remove_accepted(specbind_root: &Path) -> Result<bool, ReviewIssues> {
    let relative = "state/contract-review.md";
    let path = specbind_root.join(relative);
    let metadata = match fs::symlink_metadata(&path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(one_review_issue(
                "CONTRACT_REVIEW_TARGET_INVALID",
                Some(relative.to_owned()),
                error.to_string(),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_review_issue(
            "CONTRACT_REVIEW_TARGET_INVALID",
            Some(relative.to_owned()),
            "accepted review must be a regular non-symlink file",
        ));
    }
    fs::remove_file(&path).map_err(|error| {
        one_review_issue(
            "CONTRACT_REVIEW_REMOVE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    Ok(true)
}

pub(super) fn read_accepted_review(
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
                        "CONTRACT_REVIEW_MISSING",
                        Some(relative.to_owned()),
                        "Spec-backed milestone requires an accepted contract review",
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
                "CONTRACT_REVIEW_UNEXPECTED_FOR_DIRECT_ONLY",
                Some(relative.to_owned()),
                "Direct-only milestone must not retain an accepted contract review",
            )],
        )));
    }
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(Box::new(freshness_report(
            ReviewFreshnessStatus::Invalid,
            None,
            None,
            vec![review_issue(
                "CONTRACT_REVIEW_TARGET_INVALID",
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

fn parse_accepted_review(
    content: &str,
    source: &str,
) -> Result<AcceptedReviewRecord, ReviewIssues> {
    let (frontmatter, body) = split_review_frontmatter(content).map_err(|message| {
        one_review_issue(
            "CONTRACT_REVIEW_FRONTMATTER_INVALID",
            Some(source.to_owned()),
            message,
        )
    })?;
    let raw = serde_saphyr::from_str::<StoredReviewFrontmatter>(frontmatter).map_err(|error| {
        one_review_issue(
            "CONTRACT_REVIEW_FRONTMATTER_INVALID",
            Some(source.to_owned()),
            error.to_string(),
        )
    })?;
    let mut issues = Vec::new();
    if raw.artifact_type != "SpecBind Contract Review" {
        issues.push(review_issue(
            "CONTRACT_REVIEW_TYPE_INVALID",
            Some(source.to_owned()),
            "type must be SpecBind Contract Review",
        ));
    }
    if Uuid::parse_str(&raw.milestone_id).map_or(true, |id| {
        id.get_version_num() != 7 || id.hyphenated().to_string() != raw.milestone_id
    }) {
        issues.push(review_issue(
            "CONTRACT_REVIEW_MILESTONE_ID_INVALID",
            Some(source.to_owned()),
            "milestone_id must be a canonical UUID v7",
        ));
    }
    if OffsetDateTime::parse(&raw.passed_at, &Rfc3339).is_err() {
        issues.push(review_issue(
            "CONTRACT_REVIEW_PASSED_AT_INVALID",
            Some(source.to_owned()),
            "passed_at must be a timezone-qualified RFC 3339 timestamp",
        ));
    }
    if body.trim().is_empty() {
        issues.push(review_issue(
            "CONTRACT_REVIEW_ASSESSMENT_EMPTY",
            Some(source.to_owned()),
            "accepted review must contain a non-empty Markdown body",
        ));
    }
    if !raw.input_revisions.contains_key(ROADMAP_KEY) {
        issues.push(review_issue(
            "CONTRACT_REVIEW_ROADMAP_INPUT_MISSING",
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
                "CONTRACT_REVIEW_INPUT_KEY_INVALID",
                Some(key.clone()),
                "input revision key is not a canonical contract review selector",
            ));
        }
        if !valid_fingerprint(value) {
            issues.push(review_issue(
                "CONTRACT_REVIEW_FINGERPRINT_INVALID",
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

pub(super) fn render_review(
    resolution: &ReviewInputResolution,
    passed_at: &str,
) -> Result<String, ReviewIssues> {
    let frontmatter = ReviewFrontmatter {
        artifact_type: "SpecBind Contract Review",
        milestone_id: &resolution.roadmap.milestone_id,
        passed_at,
        input_revisions: OrderedRevisions(&resolution.input_revisions),
    };
    let yaml = serde_saphyr::to_string(&frontmatter).map_err(|error| {
        one_review_issue("CONTRACT_REVIEW_SERIALIZE_FAILED", None, error.to_string())
    })?;
    let mut content = format!("---\n{yaml}---\n{}", resolution.candidate.assessment);
    if !content.ends_with('\n') {
        content.push('\n');
    }
    Ok(content)
}

pub(super) fn persist_review(
    specbind_root: &Path,
    relative: &str,
    bytes: &[u8],
) -> Result<(), ReviewIssues> {
    let state = specbind_root.join("state");
    match fs::symlink_metadata(&state) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(one_review_issue(
                "CONTRACT_REVIEW_STATE_DIR_INVALID",
                Some("state".to_owned()),
                "state must be a regular non-symlink directory",
            ));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(&state).map_err(|error| {
                one_review_issue(
                    "CONTRACT_REVIEW_STATE_DIR_CREATE_FAILED",
                    Some("state".to_owned()),
                    error.to_string(),
                )
            })?;
        }
        Err(error) => {
            return Err(one_review_issue(
                "CONTRACT_REVIEW_STATE_DIR_INVALID",
                Some("state".to_owned()),
                error.to_string(),
            ));
        }
    }
    let target = specbind_root.join(relative);
    guarded_fs::replace_optional(&target, bytes).map_err(|error| match error {
        GuardedWriteError::InvalidTarget(_) => one_review_issue(
            "CONTRACT_REVIEW_TARGET_INVALID",
            Some(relative.to_owned()),
            "accepted review target must be absent or a regular non-symlink file",
        ),
        _ => one_review_issue(
            "CONTRACT_REVIEW_WRITE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        ),
    })?;
    Ok(())
}
