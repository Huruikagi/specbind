use std::path::{Component, Path};

use serde::Deserialize;

use crate::schema::spec::v1::{
    MechanicalCheck, MechanicalCheckKind, NonEmptyString, SuccessfulExitCode,
};

use super::{CompletionIssues, finish_issues, guard::validate_revision, issue, one_issue};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CompletionCandidate {
    schema_version: u64,
    implementation_revision: String,
    mechanical_checks: Vec<CandidateMechanicalCheck>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct CandidateMechanicalCheck {
    kind: MechanicalCheckKind,
    command: String,
    exit_code: u8,
    #[serde(default)]
    working_directory: Option<String>,
}

pub(super) struct ValidatedCandidate {
    pub(super) implementation_revision: String,
    pub(super) mechanical_checks: Vec<MechanicalCheck>,
}

pub(super) fn validate(candidate_json: &str) -> Result<ValidatedCandidate, CompletionIssues> {
    let candidate =
        serde_json::from_str::<CompletionCandidate>(candidate_json).map_err(|error| {
            one_issue(
                "COMPLETION_EVIDENCE_INVALID",
                None,
                format!("completion evidence is not strict version-1 JSON: {error}"),
            )
        })?;
    let mut issues = Vec::new();
    if candidate.schema_version != 1 {
        issues.push(issue(
            "COMPLETION_EVIDENCE_INVALID",
            Some("/schemaVersion".to_owned()),
            "schemaVersion must be 1",
        ));
    }
    if let Err(error) = validate_revision(&candidate.implementation_revision) {
        issues.extend(error.issues);
    }
    if candidate.mechanical_checks.is_empty() {
        issues.push(issue(
            "COMPLETION_EVIDENCE_INVALID",
            Some("/mechanicalChecks".to_owned()),
            "mechanicalChecks must contain at least one successful command",
        ));
    }
    let mechanical_checks = candidate
        .mechanical_checks
        .into_iter()
        .enumerate()
        .filter_map(|(index, check)| {
            let path = format!("/mechanicalChecks/{index}");
            let command_valid =
                !check.command.trim().is_empty() && !check.command.chars().any(char::is_control);
            if !command_valid {
                issues.push(issue(
                    "COMPLETION_EVIDENCE_INVALID",
                    Some(format!("{path}/command")),
                    "command must be a non-empty display-safe single line",
                ));
            }
            if check.exit_code != 0 {
                issues.push(issue(
                    "COMPLETION_EVIDENCE_INVALID",
                    Some(format!("{path}/exitCode")),
                    "exitCode must be 0",
                ));
            }
            if let Some(directory) = check.working_directory.as_deref()
                && !valid_portable_relative(directory)
            {
                issues.push(issue(
                    "COMPLETION_EVIDENCE_INVALID",
                    Some(format!("{path}/workingDirectory")),
                    "workingDirectory must be a portable project-root-relative path",
                ));
            }
            (command_valid
                && check.exit_code == 0
                && check
                    .working_directory
                    .as_deref()
                    .is_none_or(valid_portable_relative))
            .then(|| MechanicalCheck {
                kind: check.kind,
                command: NonEmptyString(check.command),
                exit_code: SuccessfulExitCode(0),
                working_directory: check.working_directory.map(NonEmptyString),
            })
        })
        .collect::<Vec<_>>();
    finish_issues(issues)?;
    Ok(ValidatedCandidate {
        implementation_revision: candidate.implementation_revision,
        mechanical_checks,
    })
}

fn valid_portable_relative(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !value.contains('\\')
        && !path.is_absolute()
        && value
            .split('/')
            .all(|segment| !matches!(segment, "" | "." | "..") && valid_portable_segment(segment))
        && path.components().all(|component| match component {
            Component::Normal(segment) => segment.to_str().is_some_and(valid_portable_segment),
            _ => false,
        })
}

fn valid_portable_segment(value: &str) -> bool {
    let invalid_shape = value.is_empty()
        || value.ends_with([' ', '.'])
        || value
            .chars()
            .any(|character| character.is_control() || r#"<>:"\|?*"#.contains(character));
    if invalid_shape {
        return false;
    }
    let stem = value
        .split('.')
        .next()
        .unwrap_or(value)
        .to_ascii_uppercase();
    !matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        && !stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"))
            .is_some_and(|suffix| {
                matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
            })
}
