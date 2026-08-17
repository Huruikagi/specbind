//! Guarded active-milestone metadata mutations.

use std::{fmt, fs, path::Path};

use crate::{
    guarded_fs, release, release::ArchiveTargets, repository, roadmap, roadmap::ReleaseBindingEdit,
};

mod candidate;
mod scope;

pub(crate) use scope::existing_body;

pub use scope::{
    CreateOutcome, RebaselineOutcome, ScopeCounts, ScopeUpdateOutcome, create, rebaseline,
    update_scope,
};

pub(crate) const ROADMAP_RELATIVE: &str = "steering/roadmap.md";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindReleaseOutcome {
    Bound {
        milestone_id: String,
        version: String,
        targets: ArchiveTargets,
    },
    Rebound {
        milestone_id: String,
        previous: String,
        version: String,
        targets: ArchiveTargets,
    },
    AlreadyBound {
        milestone_id: String,
        version: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MilestoneIssue {
    pub code: &'static str,
    pub path: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneIssues {
    pub issues: Vec<MilestoneIssue>,
}

impl fmt::Display for MilestoneIssues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "milestone operation has {} issue(s)",
            self.issues.len()
        )
    }
}

impl std::error::Error for MilestoneIssues {}

/// Binds or explicitly rebinds the active milestone's portable release label.
///
/// # Errors
///
/// Returns version, active-Roadmap, rebind-authorization, archive-collision,
/// target-dirty, race, serialization, or guarded-write diagnostics.
pub fn bind_release(
    project_root: &Path,
    specbind_root: &Path,
    requested: &str,
    allow_rebind: bool,
) -> Result<BindReleaseOutcome, MilestoneIssues> {
    if !release::valid_version(requested) {
        return Err(one_issue(
            "INVALID_RELEASE_VERSION",
            None,
            "release version must match ^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$",
        ));
    }
    let relative = ROADMAP_RELATIVE;
    let path = specbind_root.join(relative);
    let initial_source = read_roadmap(specbind_root, "release binding")?;
    let initial = roadmap::parse(&initial_source).map_err(roadmap_failure)?;
    if initial.target_release.as_deref() == Some(requested) {
        return Ok(BindReleaseOutcome::AlreadyBound {
            milestone_id: initial.milestone_id,
            version: requested.to_owned(),
        });
    }
    if let Some(current) = initial.target_release.as_deref()
        && !allow_rebind
    {
        return Err(one_issue(
            "RELEASE_REBIND_REQUIRED",
            Some(relative.to_owned()),
            format!(
                "milestone is bound to {current}; replacing it with {requested} requires --rebind"
            ),
        ));
    }
    let targets = release::resolve_available_archive_targets(specbind_root, requested)
        .map_err(release_failure)?;
    ensure_target_clean(
        project_root,
        specbind_root,
        relative,
        "MILESTONE_ROADMAP_DIRTY",
        "release binding",
    )?;

    let current_source = read_roadmap(specbind_root, "release binding")?;
    if current_source != initial_source {
        return Err(one_issue(
            "MILESTONE_INPUTS_CHANGED",
            Some(relative.to_owned()),
            "active Roadmap changed during release binding",
        ));
    }
    release::resolve_available_archive_targets(specbind_root, requested)
        .map_err(release_failure)?;
    let previous = initial.target_release.clone();
    let edit =
        roadmap::bind_release(&current_source, requested, allow_rebind).map_err(roadmap_failure)?;
    let ReleaseBindingEdit::Updated(rendered) = edit else {
        return Err(one_issue(
            "MILESTONE_INPUTS_CHANGED",
            Some(relative.to_owned()),
            "release binding outcome changed during guarded mutation",
        ));
    };
    guarded_fs::replace_existing(&path, rendered.as_bytes()).map_err(|error| {
        one_issue(
            "MILESTONE_ROADMAP_WRITE_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;

    match previous {
        Some(previous) => Ok(BindReleaseOutcome::Rebound {
            milestone_id: initial.milestone_id,
            previous,
            version: requested.to_owned(),
            targets,
        }),
        None => Ok(BindReleaseOutcome::Bound {
            milestone_id: initial.milestone_id,
            version: requested.to_owned(),
            targets,
        }),
    }
}

fn read_roadmap(specbind_root: &Path, operation: &str) -> Result<String, MilestoneIssues> {
    let relative = ROADMAP_RELATIVE;
    let path = &specbind_root.join(relative);
    let parent = path.parent().ok_or_else(|| {
        one_issue(
            "MILESTONE_ROADMAP_TARGET_INVALID",
            Some(relative.to_owned()),
            "active Roadmap has no parent directory",
        )
    })?;
    let parent_metadata = fs::symlink_metadata(parent).map_err(|error| {
        one_issue(
            "MILESTONE_ROADMAP_READ_FAILED",
            Some(relative.to_owned()),
            format!("cannot inspect Roadmap parent: {error}"),
        )
    })?;
    if guarded_fs::is_link_like(&parent_metadata) || !parent_metadata.is_dir() {
        return Err(one_issue(
            "MILESTONE_ROADMAP_TARGET_INVALID",
            Some(relative.to_owned()),
            "active Roadmap parent must be a regular non-symlink directory",
        ));
    }
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        let (code, message) = if error.kind() == std::io::ErrorKind::NotFound {
            (
                "NO_ACTIVE_MILESTONE",
                format!("{operation} requires an active Roadmap"),
            )
        } else {
            (
                "MILESTONE_ROADMAP_READ_FAILED",
                format!("cannot inspect active Roadmap: {error}"),
            )
        };
        one_issue(code, Some(relative.to_owned()), message)
    })?;
    if guarded_fs::is_link_like(&metadata) || !metadata.is_file() {
        return Err(one_issue(
            "MILESTONE_ROADMAP_TARGET_INVALID",
            Some(relative.to_owned()),
            "active Roadmap must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(path).map_err(|error| {
        one_issue(
            "MILESTONE_ROADMAP_READ_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })
}

fn ensure_target_clean(
    project_root: &Path,
    specbind_root: &Path,
    relative: &str,
    code: &'static str,
    operation: &str,
) -> Result<(), MilestoneIssues> {
    let root_relative = specbind_root.strip_prefix(project_root).map_err(|error| {
        one_issue(
            "MILESTONE_PROJECT_ROOT_INVALID",
            Some(relative.to_owned()),
            format!("SpecBind root must be below the Git project root: {error}"),
        )
    })?;
    let git_path = root_relative
        .join(relative)
        .to_string_lossy()
        .replace('\\', "/");
    let status = repository::path_status(project_root, &git_path).map_err(|error| {
        one_issue(
            "MILESTONE_GIT_FAILED",
            Some(relative.to_owned()),
            error.to_string(),
        )
    })?;
    if status.is_empty() {
        Ok(())
    } else {
        Err(one_issue(
            code,
            Some(relative.to_owned()),
            format!("{operation} refuses to overwrite a dirty, staged, or untracked target"),
        ))
    }
}

fn finish_issues(mut issues: Vec<MilestoneIssue>) -> Result<(), MilestoneIssues> {
    issues.sort();
    issues.dedup();
    if issues.is_empty() {
        Ok(())
    } else {
        Err(MilestoneIssues { issues })
    }
}

fn roadmap_failure(error: roadmap::RoadmapIssues) -> MilestoneIssues {
    MilestoneIssues {
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

fn release_failure(error: release::ReleaseIssues) -> MilestoneIssues {
    MilestoneIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, value.path, value.message))
            .collect(),
    }
}

fn one_issue(
    code: &'static str,
    path: Option<String>,
    message: impl Into<String>,
) -> MilestoneIssues {
    MilestoneIssues {
        issues: vec![issue(code, path, message)],
    }
}

fn issue(code: &'static str, path: Option<String>, message: impl Into<String>) -> MilestoneIssue {
    MilestoneIssue {
        code,
        path,
        message: message.into(),
    }
}
