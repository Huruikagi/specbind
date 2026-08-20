//! Lifecycle facade for guarded Spec and Direct completion handshakes.

use std::{fmt, fs, path::Path};

use crate::{artifacts, domain::spec::Spec, guarded_fs, roadmap};

mod candidate;
mod direct;
mod guard;
mod spec;

pub use direct::{direct_complete, direct_preflight};
pub use spec::{spec_accept, spec_invalidate, spec_preflight};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CompletionIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionIssues {
    pub issues: Vec<CompletionIssue>,
}

impl fmt::Display for CompletionIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "completion operation has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for CompletionIssues {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecPreflightOutcome {
    Ready { implementation_revision: String },
    AlreadyAccepted { implementation_revision: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecAcceptOutcome {
    Accepted { implementation_revision: String },
    AlreadyAccepted { implementation_revision: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecInvalidateOutcome {
    Invalidated,
    NoChange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectPreflightOutcome {
    Ready { implementation_revision: String },
    AlreadyCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectCompleteOutcome {
    Recorded,
    AlreadyCompleted,
}

fn validate_mutated_spec(
    wire: &crate::schema::spec::v1::SpecDocument,
    canonical_spec: &str,
) -> Result<(), CompletionIssues> {
    Spec::try_from(wire.clone())
        .map(|_| ())
        .map_err(|error| CompletionIssues {
            issues: error
                .issues
                .into_iter()
                .map(|value| issue(value.code, Some(spec_path(canonical_spec)), value.message))
                .collect(),
        })
}

fn render_yaml<T: serde::Serialize>(
    value: &T,
    code: &'static str,
) -> Result<String, CompletionIssues> {
    let mut rendered =
        serde_saphyr::to_string(value).map_err(|error| one_issue(code, None, error.to_string()))?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    Ok(rendered)
}

fn persist_regular(
    target: &Path,
    bytes: &[u8],
    code: &'static str,
    relative: &str,
) -> Result<(), CompletionIssues> {
    guarded_fs::replace_existing(target, bytes)
        .map_err(|error| one_issue(code, Some(relative.to_owned()), error.to_string()))
}

fn read_regular(path: &Path, relative: &str) -> Result<String, CompletionIssues> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        one_issue(
            "COMPLETION_TARGET_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(one_issue(
            "COMPLETION_TARGET_INVALID",
            Some(relative.to_owned()),
            "completion target must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(path).map_err(|error| {
        one_issue(
            "COMPLETION_TARGET_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

fn spec_path(canonical_spec: &str) -> String {
    format!("specs/{canonical_spec}/spec.yaml")
}

fn discovery_failure(issues: Vec<artifacts::DiscoveryIssue>) -> CompletionIssues {
    CompletionIssues {
        issues: issues.into_iter().map(from_discovery).collect(),
    }
}

fn from_discovery(value: artifacts::DiscoveryIssue) -> CompletionIssue {
    issue(
        value.code,
        value.path.map(|path| path.to_string()),
        value.message,
    )
}

fn roadmap_failure(error: roadmap::RoadmapIssues) -> CompletionIssues {
    CompletionIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| {
                issue(
                    value.code,
                    Some("steering/roadmap.md".to_owned()),
                    value.message,
                )
            })
            .collect(),
    }
}

fn finish_issues(mut issues: Vec<CompletionIssue>) -> Result<(), CompletionIssues> {
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(CompletionIssues { issues })
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> CompletionIssues {
    CompletionIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> CompletionIssue {
    CompletionIssue {
        code,
        path,
        message: message.into(),
    }
}
