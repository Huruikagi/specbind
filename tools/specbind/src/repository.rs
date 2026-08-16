//! Narrow adapter around the installed Git executable.

use std::{fmt, path::Path, process::Command};

#[derive(Debug)]
pub enum RepositoryError {
    Start(std::io::Error),
    Command(String),
    NonUtf8(std::string::FromUtf8Error),
    UnexpectedStatus(std::process::ExitStatus),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start(error) => write!(formatter, "cannot start Git: {error}"),
            Self::Command(message) => formatter.write_str(message),
            Self::NonUtf8(error) => write!(formatter, "Git output is not UTF-8: {error}"),
            Self::UnexpectedStatus(status) => write!(formatter, "Git exited with status {status}"),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Runs a Git command and returns its stdout bytes when it succeeds.
///
/// # Errors
///
/// Returns a start or command failure when Git cannot produce successful output.
pub fn output_bytes(project_root: &Path, arguments: &[&str]) -> Result<Vec<u8>, RepositoryError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(project_root)
        .args(arguments)
        .output()
        .map_err(RepositoryError::Start)?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(RepositoryError::Command(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}

/// Runs a Git command and returns UTF-8 stdout when it succeeds.
///
/// # Errors
///
/// Returns a process failure or a non-UTF-8 output error.
pub fn output(project_root: &Path, arguments: &[&str]) -> Result<String, RepositoryError> {
    String::from_utf8(output_bytes(project_root, arguments)?).map_err(RepositoryError::NonUtf8)
}

/// Runs a Git predicate whose exit codes zero and one mean true and false.
///
/// # Errors
///
/// Returns a start failure or an unexpected Git exit status.
pub fn predicate(project_root: &Path, arguments: &[&str]) -> Result<bool, RepositoryError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(project_root)
        .args(arguments)
        .output()
        .map_err(RepositoryError::Start)?;
    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(RepositoryError::UnexpectedStatus(output.status)),
    }
}
