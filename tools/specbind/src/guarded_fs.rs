//! Guarded atomic replacement for SpecBind-owned state files.

use std::{fmt, fs, io::Write as _, path::Path};

use tempfile::NamedTempFile;

#[derive(Debug)]
pub enum GuardedWriteError {
    Inspect(std::io::Error),
    InvalidTarget(&'static str),
    Write(std::io::Error),
}

impl fmt::Display for GuardedWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inspect(error) => write!(formatter, "cannot inspect target: {error}"),
            Self::InvalidTarget(message) => formatter.write_str(message),
            Self::Write(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for GuardedWriteError {}

/// Atomically replaces an existing regular non-symlink file.
///
/// # Errors
///
/// Returns an inspection, target-shape, temporary-write, or persist failure.
pub fn replace_existing(target: &Path, bytes: &[u8]) -> Result<(), GuardedWriteError> {
    validate_target(target, false)?;
    replace(target, bytes)
}

/// Atomically creates or replaces a regular non-symlink file.
///
/// # Errors
///
/// Returns an inspection, target-shape, temporary-write, or persist failure.
pub fn replace_optional(target: &Path, bytes: &[u8]) -> Result<(), GuardedWriteError> {
    validate_target(target, true)?;
    replace(target, bytes)
}

fn validate_target(target: &Path, allow_absent: bool) -> Result<(), GuardedWriteError> {
    match fs::symlink_metadata(target) {
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => Err(
            GuardedWriteError::InvalidTarget("mutation target must be a regular non-symlink file"),
        ),
        Ok(_) => Ok(()),
        Err(error) if allow_absent && error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(GuardedWriteError::Inspect(error)),
    }
}

fn replace(target: &Path, bytes: &[u8]) -> Result<(), GuardedWriteError> {
    let parent = target.parent().ok_or(GuardedWriteError::InvalidTarget(
        "mutation target has no parent directory",
    ))?;
    let mut temporary = NamedTempFile::new_in(parent).map_err(GuardedWriteError::Write)?;
    temporary
        .write_all(bytes)
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(GuardedWriteError::Write)?;
    temporary
        .persist(target)
        .map_err(|error| GuardedWriteError::Write(error.error))?;
    Ok(())
}
