//! Shared guarded external-input loading for CLI commands.

use super::*;

/// One transient external command input and its stable diagnostic vocabulary.
///
/// Every caller shares the same safety boundary: `-` reads standard input, a
/// path must be an ordinary non-symlink file, and the content must be UTF-8.
/// Inputs that carry authority over persisted state additionally require a
/// repository-external path so the worktree cannot supply its own evidence.
pub(super) struct ExternalInputSpec {
    read_failed: &'static str,
    target_invalid: &'static str,
    /// Subject phrase used for the standard-input diagnostic.
    stdin_subject: &'static str,
    /// Subject phrase used inside a sentence.
    subject: &'static str,
    /// Subject phrase used to start a sentence.
    capitalized: &'static str,
    require_external: bool,
}

pub(super) struct ExternalInputError {
    pub(super) code: &'static str,
    pub(super) message: String,
}

pub(super) const COMPLETION_EVIDENCE_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "COMPLETION_EVIDENCE_READ_FAILED",
    target_invalid: "COMPLETION_EVIDENCE_TARGET_INVALID",
    stdin_subject: "completion evidence",
    subject: "completion evidence",
    capitalized: "Completion evidence",
    require_external: true,
};

pub(super) const LOG_ENTRIES_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "LOG_INPUT_READ_FAILED",
    target_invalid: "LOG_INPUT_TARGET_INVALID",
    stdin_subject: "log entries",
    subject: "log-entry input",
    capitalized: "Log-entry input",
    require_external: false,
};

pub(super) const SCOPE_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "MILESTONE_SCOPE_READ_FAILED",
    target_invalid: "MILESTONE_SCOPE_TARGET_INVALID",
    stdin_subject: "milestone scope",
    subject: "milestone scope",
    capitalized: "Milestone scope",
    require_external: true,
};

pub(super) const REVIEW_CANDIDATE_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "MILESTONE_REVIEW_CANDIDATE_READ_FAILED",
    target_invalid: "MILESTONE_REVIEW_CANDIDATE_TARGET_INVALID",
    stdin_subject: "review candidate",
    subject: "review candidate",
    capitalized: "Review candidate",
    require_external: true,
};

pub(super) const MIGRATION_RESOLUTION_INPUT: ExternalInputSpec = ExternalInputSpec {
    read_failed: "MIGRATION_RESOLUTION_READ_FAILED",
    target_invalid: "MIGRATION_RESOLUTION_TARGET_INVALID",
    stdin_subject: "migration resolution candidate",
    subject: "migration resolution candidate",
    capitalized: "Migration resolution candidate",
    require_external: true,
};

pub(super) fn read_external_input(
    spec: &ExternalInputSpec,
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, ExternalInputError> {
    let read_failed = |message: String| ExternalInputError {
        code: spec.read_failed,
        message,
    };
    let target_invalid = |message: String| ExternalInputError {
        code: spec.target_invalid,
        message,
    };
    if source == "-" {
        let mut input = String::new();
        return io::stdin()
            .read_to_string(&mut input)
            .map(|_| input)
            .map_err(|error| {
                read_failed(format!(
                    "Cannot read {} from stdin: {error}",
                    spec.stdin_subject
                ))
            });
    }
    let requested = start.join(source);
    let metadata = fs::symlink_metadata(&requested)
        .map_err(|error| read_failed(format!("Cannot inspect {}: {error}", spec.subject)))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(target_invalid(format!(
            "{} must be a regular non-symlink file.",
            spec.capitalized
        )));
    }
    let source_path = if spec.require_external {
        let canonical = requested
            .canonicalize()
            .map_err(|error| read_failed(format!("Cannot resolve {}: {error}", spec.subject)))?;
        let canonical_project = project_root
            .canonicalize()
            .map_err(|error| read_failed(format!("Cannot resolve project root: {error}")))?;
        if canonical.starts_with(canonical_project) {
            return Err(target_invalid(format!(
                "{} file must be outside the project worktree.",
                spec.capitalized
            )));
        }
        canonical
    } else {
        requested
    };
    fs::read_to_string(source_path)
        .map_err(|error| read_failed(format!("Cannot read {} as UTF-8: {error}", spec.subject)))
}

pub(super) fn read_external_json(
    start: &Path,
    project_root: &Path,
    source: &str,
) -> Result<String, CommandOutput> {
    read_external_input(&COMPLETION_EVIDENCE_INPUT, start, project_root, source)
        .map_err(|error| CommandOutput::failure(error.code, error.message, vec![]))
}
