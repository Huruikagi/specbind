//! Disposable committed checkout for historical read-model evaluation.

use std::{path::Path, process::Command};

use super::RepositoryError;

/// Reconstructs committed files without modifying the source repository.
pub(crate) fn at(project: &Path, revision: &str) -> Result<tempfile::TempDir, RepositoryError> {
    let directory = tempfile::tempdir().map_err(RepositoryError::Start)?;
    run(
        project,
        &[
            "clone",
            "--quiet",
            "--shared",
            "--no-checkout",
            "--no-hardlinks",
            "--template=",
            "--",
            &project.to_string_lossy(),
            &directory.path().to_string_lossy(),
        ],
    )?;
    // Keep all subsequent read-model Git calls consistent with this materialization.
    for (key, value) in [
        ("core.autocrlf", "false"),
        ("core.symlinks", "false"),
        ("core.hooksPath", ".git/no-hooks"),
        ("submodule.recurse", "false"),
    ] {
        run(directory.path(), &["config", "--local", key, value])?;
    }
    run(
        directory.path(),
        &["checkout", "--quiet", "--detach", revision, "--"],
    )?;
    Ok(directory)
}

fn run(project: &Path, args: &[&str]) -> Result<(), RepositoryError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(project)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env(
            "GIT_CONFIG_GLOBAL",
            if cfg!(windows) { "NUL" } else { "/dev/null" },
        )
        .env("GIT_CONFIG_COUNT", "0")
        .env("GIT_NO_LAZY_FETCH", "1")
        .env_remove("GIT_CONFIG_PARAMETERS")
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .args(args)
        .output()
        .map_err(RepositoryError::Start)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(RepositoryError::Command(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}
