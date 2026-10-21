//! Explicit-set completion operations; semantic verdicts and Git commits stay external.

use std::{collections::BTreeSet, path::Path};

use serde::Serialize;

use crate::{repository, schema::completion_batch::v1::CompletionBatch};

use super::{
    CompletionIssue, CompletionIssues,
    candidate::{ValidatedCandidate, validate},
    guard::{clean_head, spec_guard, valid_id},
    one_issue, persist_regular, read_regular,
    spec::render_acceptance,
    spec_path,
};

mod verify;
pub use verify::verify;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchReport {
    pub schema_version: u64,
    pub result: &'static str,
    pub implementation_revision: String,
    pub targets: Vec<String>,
    pub accepted: Vec<String>,
    pub pending: Vec<String>,
    pub paths: Vec<String>,
    pub issues: Vec<CompletionIssue>,
}

impl BatchReport {
    #[must_use]
    pub fn succeeded(&self) -> bool {
        self.issues.is_empty()
    }
}

struct Input {
    revision: String,
    ids: Vec<String>,
    candidates: Vec<ValidatedCandidate>,
}

struct Prepared {
    id: String,
    source: String,
    rendered: String,
}

fn parse(source: &str) -> Result<Input, CompletionIssues> {
    let wire: CompletionBatch = serde_json::from_str(source)
        .map_err(|e| one_issue("COMPLETION_BATCH_INPUT_INVALID", None, e.to_string()))?;
    let ids: Vec<_> = wire.targets.iter().map(|t| t.spec.clone()).collect();
    validate_ids(&ids)?;
    let mut candidates = Vec::new();
    for target in wire.targets {
        let candidate = serde_json::json!({
            "schemaVersion": wire.schema_version,
            "implementationRevision": wire.implementation_revision,
            "mechanicalChecks": target.mechanical_checks,
        });
        candidates.push(validate(&candidate.to_string())?);
    }
    Ok(Input {
        revision: wire.implementation_revision,
        ids,
        candidates,
    })
}

fn validate_ids(ids: &[String]) -> Result<(), CompletionIssues> {
    let unique: BTreeSet<_> = ids.iter().collect();
    if ids.is_empty() || unique.len() != ids.len() || ids.iter().any(|id| !valid_id(id)) {
        return Err(one_issue(
            "COMPLETION_BATCH_TARGET_INVALID",
            None,
            "supply a nonempty set of unique canonical Spec IDs",
        ));
    }
    Ok(())
}

fn paths(project: &Path, root: &Path, ids: &[String]) -> Result<Vec<String>, CompletionIssues> {
    let relative = root
        .strip_prefix(project)
        .map_err(|e| one_issue("COMPLETION_PROJECT_ROOT_INVALID", None, e.to_string()))?;
    Ok(ids
        .iter()
        .map(|id| {
            relative
                .join(spec_path(id))
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect())
}

fn report(
    project: &Path,
    root: &Path,
    revision: String,
    ids: &[String],
) -> Result<BatchReport, CompletionIssues> {
    Ok(BatchReport {
        schema_version: 1,
        result: "COMPLETION_BATCH_PREFLIGHT_READY",
        implementation_revision: revision,
        targets: ids.to_vec(),
        accepted: vec![],
        pending: ids.to_vec(),
        paths: paths(project, root, ids)?,
        issues: vec![],
    })
}

/// Preflights every explicit target at one clean revision without mutation.
/// # Errors
/// Rejects invalid targets, nonconvergence, stale gates and dirty or moving HEAD.
pub fn preflight(
    project: &Path,
    root: &Path,
    ids: &[String],
) -> Result<BatchReport, CompletionIssues> {
    validate_ids(ids)?;
    let revision = clean_head(project)?;
    for id in ids {
        spec_guard(project, root, id, &revision, false)?;
    }
    same_clean_head(project, &revision)?;
    report(project, root, revision, ids)
}

fn same_clean_head(project: &Path, revision: &str) -> Result<(), CompletionIssues> {
    if clean_head(project)? != revision {
        return Err(one_issue(
            "COMPLETION_REVISION_CHANGED",
            None,
            "HEAD differs from the fixed implementation revision",
        ));
    }
    Ok(())
}

/// Accepts an explicit set after all candidate and lifecycle guards pass.
/// # Errors
/// Invalid input returns diagnostics. Once parsed, failures return a report with
/// confirmed writes and remaining targets; callers must inspect `succeeded()`.
pub fn accept(project: &Path, root: &Path, source: &str) -> Result<BatchReport, CompletionIssues> {
    let input = parse(source)?;
    let mut result = report(project, root, input.revision.clone(), &input.ids)?;
    let prepared = match prepare(project, root, &input) {
        Ok(prepared) => prepared,
        Err(error) => {
            result.result = "COMPLETION_BATCH_REJECTED";
            result.issues = error.issues;
            return Ok(result);
        }
    };
    let target_paths = result.paths.clone();
    apply_prepared(&mut result, &prepared, |index| {
        verify_pending(
            project,
            root,
            &input.revision,
            &prepared,
            &target_paths,
            index,
        )?;
        let target = &prepared[index];
        let current = spec_guard(project, root, &target.id, &input.revision, true)?;
        if current.source != target.source {
            return Err(one_issue(
                "COMPLETION_INPUTS_CHANGED",
                Some(spec_path(&target.id)),
                "target changed after full-set validation",
            ));
        }
        persist_regular(
            &root.join(spec_path(&target.id)),
            target.rendered.as_bytes(),
            "COMPLETION_SPEC_WRITE_FAILED",
            &spec_path(&target.id),
        )
    });
    if result.succeeded()
        && let Err(error) = verify_pending(
            project,
            root,
            &input.revision,
            &prepared,
            &result.paths,
            prepared.len(),
        )
    {
        result.result = "COMPLETION_BATCH_INTERRUPTED";
        result.issues = error.issues;
    }
    Ok(result)
}

fn prepare(project: &Path, root: &Path, input: &Input) -> Result<Vec<Prepared>, CompletionIssues> {
    same_clean_head(project, &input.revision)?;
    let mut prepared = Vec::new();
    for (id, candidate) in input.ids.iter().zip(&input.candidates) {
        let guard = spec_guard(project, root, id, &input.revision, false)?;
        prepared.push(Prepared {
            id: id.clone(),
            source: guard.source,
            rendered: render_acceptance(guard.wire, id, candidate)?,
        });
    }
    same_clean_head(project, &input.revision)?;
    Ok(prepared)
}

fn apply_prepared(
    result: &mut BatchReport,
    prepared: &[Prepared],
    mut apply: impl FnMut(usize) -> Result<(), CompletionIssues>,
) {
    for (index, target) in prepared.iter().enumerate() {
        if let Err(error) = apply(index) {
            result.result = "COMPLETION_BATCH_INTERRUPTED";
            result.issues = error.issues;
            return;
        }
        result.accepted.push(target.id.clone());
        result.pending.remove(0);
    }
    result.result = "COMPLETION_BATCH_ACCEPTED";
}

fn git(project: &Path, args: &[&str]) -> Result<String, CompletionIssues> {
    repository::output(project, args)
        .map_err(|e| one_issue("COMPLETION_GIT_FAILED", None, e.to_string()))
}

fn verify_pending(
    project: &Path,
    root: &Path,
    revision: &str,
    prepared: &[Prepared],
    paths: &[String],
    written: usize,
) -> Result<(), CompletionIssues> {
    if git(project, &["rev-parse", "HEAD"])?.trim() != revision {
        return Err(one_issue(
            "COMPLETION_REVISION_CHANGED",
            None,
            "HEAD moved during batch acceptance",
        ));
    }
    let status = git(
        project,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    )?;
    let actual: BTreeSet<_> = status
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    let expected: BTreeSet<_> = paths[..written].iter().map(|p| format!(" M {p}")).collect();
    if actual != expected {
        return Err(one_issue(
            "COMPLETION_BATCH_WORKTREE_CHANGED",
            None,
            "only this batch's unstaged completion writes are permitted",
        ));
    }
    for (index, target) in prepared.iter().enumerate() {
        let expected = if index < written {
            &target.rendered
        } else {
            &target.source
        };
        if read_regular(&root.join(spec_path(&target.id)), &spec_path(&target.id))? != *expected {
            return Err(one_issue(
                "COMPLETION_INPUTS_CHANGED",
                Some(spec_path(&target.id)),
                "target bytes changed during batch acceptance",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intervening_changes_stop_the_suffix_without_overwriting_external_work() {
        for interference in ["head", "unrelated", "target", "index"] {
            let directory = tempfile::tempdir().unwrap();
            let project = directory.path();
            git(project, &["init", "-q"]).unwrap();
            let prepared: Vec<_> = ["a", "b"]
                .iter()
                .map(|id| Prepared {
                    id: (*id).into(),
                    source: "before".into(),
                    rendered: "after".into(),
                })
                .collect();
            for target in &prepared {
                let path = project.join(spec_path(&target.id));
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(path, &target.source).unwrap();
            }
            git(project, &["add", "."]).unwrap();
            let commit = [
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "commit",
                "-qm",
                "Fixture",
            ];
            git(project, &commit).unwrap();
            let revision = git(project, &["rev-parse", "HEAD"]).unwrap();
            let ids = vec!["a".to_owned(), "b".to_owned()];
            let mut result = report(project, project, revision.trim().into(), &ids).unwrap();
            let target_paths = result.paths.clone();
            apply_prepared(&mut result, &prepared, |index| {
                verify_pending(
                    project,
                    project,
                    revision.trim(),
                    &prepared,
                    &target_paths,
                    index,
                )?;
                let path = spec_path(&prepared[index].id);
                persist_regular(
                    &project.join(&path),
                    b"after",
                    "COMPLETION_SPEC_WRITE_FAILED",
                    &path,
                )?;
                match interference {
                    "head" => {
                        git(project, &["add", "."])?;
                        git(project, &commit)?;
                    }
                    "unrelated" => {
                        std::fs::write(project.join("external.txt"), "external").unwrap();
                    }
                    "target" => std::fs::write(project.join(spec_path("b")), "external").unwrap(),
                    "index" => {
                        git(project, &["add", "."])?;
                    }
                    _ => unreachable!(),
                }
                Ok(())
            });
            assert_eq!(result.result, "COMPLETION_BATCH_INTERRUPTED");
            assert_eq!(result.accepted, ["a"]);
            assert_eq!(result.pending, ["b"]);
            assert_eq!(
                std::fs::read_to_string(project.join(spec_path("b"))).unwrap(),
                if interference == "target" {
                    "external"
                } else {
                    "before"
                }
            );
        }
    }

    #[test]
    fn interrupted_write_keeps_the_confirmed_prefix_and_never_writes_the_suffix() {
        let directory = tempfile::tempdir().unwrap();
        let prepared: Vec<_> = ["a", "b", "c"]
            .iter()
            .map(|id| Prepared {
                id: (*id).into(),
                source: "before".into(),
                rendered: "after".into(),
            })
            .collect();
        for target in &prepared {
            std::fs::write(directory.path().join(&target.id), &target.source).unwrap();
        }
        let ids = prepared.iter().map(|p| p.id.clone()).collect::<Vec<_>>();
        let mut result = report(directory.path(), directory.path(), "a".repeat(40), &ids).unwrap();
        apply_prepared(&mut result, &prepared, |index| {
            if index == 1 {
                return Err(one_issue(
                    "COMPLETION_SPEC_WRITE_FAILED",
                    Some("b".into()),
                    "injected I/O failure",
                ));
            }
            persist_regular(
                &directory.path().join(&prepared[index].id),
                b"after",
                "COMPLETION_SPEC_WRITE_FAILED",
                &prepared[index].id,
            )
        });
        assert!(!result.succeeded());
        assert_eq!(result.accepted, ["a"]);
        assert_eq!(result.pending, ["b", "c"]);
        assert_eq!(
            std::fs::read_to_string(directory.path().join("a")).unwrap(),
            "after"
        );
        for id in ["b", "c"] {
            assert_eq!(
                std::fs::read_to_string(directory.path().join(id)).unwrap(),
                "before"
            );
        }
    }
}
