//! Lifecycle values for portable release identities and archive destinations.

use std::{fmt, fs, path::Path};

use crate::guarded_fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchiveTargets {
    pub roadmap: String,
    pub cross_spec_review: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReleaseIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseIssues {
    pub issues: Vec<ReleaseIssue>,
}

impl fmt::Display for ReleaseIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "release resolution has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for ReleaseIssues {}

#[must_use]
pub fn valid_version(value: &str) -> bool {
    let bytes = value.as_bytes();
    (1..=64).contains(&bytes.len())
        && bytes[0].is_ascii_alphanumeric()
        && bytes[1..]
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
}

/// Resolves portable archive paths and rejects existing case-insensitive collisions.
///
/// # Errors
///
/// Returns version, archive-root, directory-read, filename, or collision diagnostics.
pub fn resolve_available_archive_targets(
    specbind_root: &Path,
    version: &str,
) -> Result<ArchiveTargets, ReleaseIssues> {
    let targets = archive_targets(version)?;
    let releases = specbind_root.join("releases");
    let metadata = match fs::symlink_metadata(&releases) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(targets),
        Err(error) => {
            return Err(one_issue(
                "RELEASE_ARCHIVE_ROOT_UNAVAILABLE",
                Some("releases".to_owned()),
                format!("cannot inspect release archive root: {error}"),
            ));
        }
    };
    if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
        return Err(one_issue(
            "RELEASE_ARCHIVE_ROOT_INVALID",
            Some("releases".to_owned()),
            "release archive root must be a regular non-symlink directory",
        ));
    }

    let target_names = [
        targets.roadmap.trim_start_matches("releases/"),
        targets.cross_spec_review.trim_start_matches("releases/"),
    ];
    let entries = fs::read_dir(&releases).map_err(|error| {
        one_issue(
            "RELEASE_ARCHIVE_ROOT_UNAVAILABLE",
            Some("releases".to_owned()),
            format!("cannot read release archive root: {error}"),
        )
    })?;
    let mut issues = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| {
            one_issue(
                "RELEASE_ARCHIVE_ROOT_UNAVAILABLE",
                Some("releases".to_owned()),
                format!("cannot read release archive entry: {error}"),
            )
        })?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            issues.push(issue(
                "RELEASE_ARCHIVE_NAME_INVALID",
                Some("releases".to_owned()),
                "release archive entry name must be UTF-8",
            ));
            continue;
        };
        for target in target_names {
            if name.eq_ignore_ascii_case(target) {
                issues.push(issue(
                    "RELEASE_ARCHIVE_COLLISION",
                    Some(format!("releases/{name}")),
                    format!("archive destination conflicts with releases/{target}"),
                ));
            }
        }
    }
    finish_issues(issues)?;
    Ok(targets)
}

/// Derives the two portable release archive paths without inspecting the filesystem.
///
/// # Errors
///
/// Returns `INVALID_RELEASE_VERSION` when the opaque label is not portable.
pub fn archive_targets(version: &str) -> Result<ArchiveTargets, ReleaseIssues> {
    if !valid_version(version) {
        return Err(one_issue(
            "INVALID_RELEASE_VERSION",
            None,
            "release version must match ^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$",
        ));
    }
    Ok(ArchiveTargets {
        roadmap: format!("releases/{version}-roadmap.md"),
        cross_spec_review: format!("releases/{version}-contract-review.md"),
    })
}

fn finish_issues(mut issues: Vec<ReleaseIssue>) -> Result<(), ReleaseIssues> {
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(ReleaseIssues { issues })
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> ReleaseIssues {
    ReleaseIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> ReleaseIssue {
    ReleaseIssue {
        code,
        path,
        message: message.into(),
    }
}
