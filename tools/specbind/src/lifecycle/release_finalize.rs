//! Lifecycle service for guarded whole-milestone release finalization.

use std::{fmt, path::Path};

use time::OffsetDateTime;

use crate::{config::ProjectLanguage, release_log, release_readiness};

mod persistence;
mod plan;
mod recovery;

use persistence::{apply_plan, verify_final_state};
use plan::build_plan;
use recovery::{completed_retry, resume_partial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeOutcome {
    Finalized { version: String, specs: usize },
    AlreadyFinalized { version: String, specs: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FinalizeIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizeIssues {
    pub issues: Vec<FinalizeIssue>,
}

impl fmt::Display for FinalizeIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "release finalization has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for FinalizeIssues {}

/// Rechecks current readiness and applies the ordered release-finalization transition.
///
/// # Errors
///
/// Returns input, readiness, log, serialization, path, race, or filesystem diagnostics.
pub fn finalize(
    project_root: &Path,
    specbind_root: &Path,
    language: ProjectLanguage,
    log_entries_json: Option<&str>,
) -> Result<FinalizeOutcome, FinalizeIssues> {
    let initial = match release_readiness::resolve(project_root, specbind_root) {
        Ok(readiness) => readiness,
        Err(error) if error.code == "NO_ACTIVE_MILESTONE" => {
            if let Some(outcome) =
                completed_retry(project_root, specbind_root, language, log_entries_json)?
            {
                return Ok(outcome);
            }
            return Err(readiness_failure(error));
        }
        Err(error) => {
            if let Some(outcome) =
                resume_partial(project_root, specbind_root, language, log_entries_json)?
            {
                return Ok(outcome);
            }
            return Err(readiness_failure(error));
        }
    };
    let entries =
        release_log::validate_input(log_entries_json, &initial.specs).map_err(log_failure)?;
    let date = local_date()?;
    let plan = build_plan(specbind_root, language, date, initial, &entries)?;

    let current = release_readiness::resolve(project_root, specbind_root)
        .map_err(readiness_failure)
        .and_then(|readiness| build_plan(specbind_root, language, date, readiness, &entries))?;
    if current != plan {
        return Err(one_issue(
            "RELEASE_INPUTS_CHANGED",
            None,
            "release inputs changed during guarded finalization",
        ));
    }
    apply_plan(specbind_root, &plan)?;
    verify_final_state(specbind_root, &plan)?;
    Ok(FinalizeOutcome::Finalized {
        version: plan.readiness.version.clone(),
        specs: plan.specs.len(),
    })
}

fn local_date() -> Result<time::Date, FinalizeIssues> {
    OffsetDateTime::now_local()
        .map(OffsetDateTime::date)
        .map_err(|error| {
            one_issue(
                "RELEASE_LOCAL_DATE_FAILED",
                None,
                format!("cannot resolve host local date: {error}"),
            )
        })
}

fn readiness_failure(error: release_readiness::ReleaseReadinessFailure) -> FinalizeIssues {
    FinalizeIssues {
        issues: error
            .diagnostics
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    }
}

fn log_failure(error: Vec<release_log::LogIssue>) -> FinalizeIssues {
    FinalizeIssues {
        issues: error
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> FinalizeIssues {
    FinalizeIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> FinalizeIssue {
    FinalizeIssue {
        code,
        path,
        message: message.into(),
    }
}
