//! Completion evidence assessment against Git history and current project files.

use std::{collections::BTreeSet, fs, path::Path};

use super::{CompletionRevisionAssessment, freshness_issue};
use crate::{
    domain::{SemanticIssue, spec::Spec},
    repository,
    roadmap::{self, ReleaseBindingEdit},
    schema::{runtime, spec::v1 as wire},
};

#[derive(Debug, Clone, Copy)]
struct CompletionPathPolicy {
    allow_release_binding: bool,
    code: &'static str,
    message: &'static str,
}

/// Validates the accepted implementation revision against the current checkout.
///
/// A successor checkout is accepted only when its complete tracked difference from the
/// implementation revision consists of recognized evidence-preserving metadata transitions.
#[must_use]
pub fn assess_completion_revision(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    spec: &Spec,
) -> CompletionRevisionAssessment {
    let mut issues = Vec::new();
    let Some(completion) = spec
        .as_wire()
        .active_change
        .0
        .as_ref()
        .and_then(|active| active.gate_evidence.as_ref())
        .and_then(|evidence| evidence.completion.as_ref())
    else {
        return CompletionRevisionAssessment { issues };
    };
    let revision = completion.implementation_revision.0.as_str();
    let relative_spec = match completion_spec_path(project_root, specbind_root, canonical_spec) {
        Ok(path) => path,
        Err(issue) => {
            issues.push(issue);
            return CompletionRevisionAssessment { issues };
        }
    };
    validate_current_spec(specbind_root, canonical_spec, spec, &mut issues);
    if !issues.is_empty() {
        return completion_assessment(issues);
    }

    match git_output(
        project_root,
        &["rev-parse", "--verify", &format!("{revision}^{{commit}}")],
    ) {
        Ok(value) if value.trim() == revision => {}
        Ok(_) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_REVISION_NOT_EXACT",
            "/completion/implementation_revision",
            "implementation_revision must resolve to the same full commit object ID",
        )),
        Err(message) => {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_REVISION_MISSING",
                "/completion/implementation_revision",
                message,
            ));
            return completion_assessment(issues);
        }
    }
    match git_status(
        project_root,
        &["merge-base", "--is-ancestor", revision, "HEAD"],
    ) {
        Ok(true) => {}
        Ok(false) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_REVISION_NOT_ANCESTOR",
            "/completion/implementation_revision",
            "implementation_revision is not an ancestor of current HEAD",
        )),
        Err(message) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_GIT_FAILED",
            "/completion/implementation_revision",
            message,
        )),
    }
    if !issues.is_empty() {
        return completion_assessment(issues);
    }

    validate_commit_history(project_root, specbind_root, revision, &mut issues);
    let changed =
        match git_output_bytes(project_root, &["diff", "--name-only", "-z", revision, "--"]) {
            Ok(output) => nul_paths(&output),
            Err(message) => {
                issues.push(freshness_issue(
                    "FRESHNESS_COMPLETION_GIT_FAILED",
                    "/completion/implementation_revision",
                    message,
                ));
                return completion_assessment(issues);
            }
        };
    match changed {
        Ok(paths) => validate_completion_paths(
            project_root,
            specbind_root,
            revision,
            &paths,
            Some(&relative_spec),
            CompletionPathPolicy {
                allow_release_binding: true,
                code: "FRESHNESS_COMPLETION_PROJECT_CHANGED",
                message: "project content since implementation_revision is not limited to recognized evidence-preserving metadata transitions",
            },
            &mut issues,
        ),
        Err(message) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_GIT_OUTPUT_INVALID",
            "/completion/implementation_revision",
            message,
        )),
    }
    validate_worktree_status(project_root, specbind_root, revision, true, &mut issues);
    completion_assessment(issues)
}

/// Validates that every current worktree change is a SpecBind-owned completion
/// metadata transition bound to the supplied implementation revision.
///
/// This is the guarded-acceptance exception that permits several participating
/// Specs validated at one clean revision to be accepted before their metadata
/// mutations are committed together.
#[must_use]
pub fn assess_pending_completion_mutations(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
) -> CompletionRevisionAssessment {
    let mut issues = Vec::new();
    validate_worktree_status(project_root, specbind_root, revision, false, &mut issues);
    completion_assessment(issues)
}

fn validate_worktree_status(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    allow_release_binding: bool,
    issues: &mut Vec<SemanticIssue>,
) {
    let output = match git_output_bytes(
        project_root,
        &["status", "--porcelain=v1", "-z", "--untracked-files=all"],
    ) {
        Ok(output) => output,
        Err(message) => {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_GIT_FAILED",
                "/completion/implementation_revision",
                message,
            ));
            return;
        }
    };
    let mut paths = Vec::new();
    for record in output
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        let accepted = record.len() > 3
            && record[0..2]
                .iter()
                .all(|byte| *byte == b' ' || *byte == b'M')
            && std::str::from_utf8(&record[3..])
                .map(|path| paths.push(path.to_owned()))
                .is_ok();
        if !accepted {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_WORKTREE_DIRTY",
                "/completion/implementation_revision",
                "working tree contains changes other than the expected completion metadata mutation",
            ));
            break;
        }
    }
    if issues.is_empty() {
        validate_completion_paths(
            project_root,
            specbind_root,
            revision,
            &paths,
            None,
            CompletionPathPolicy {
                allow_release_binding,
                code: "FRESHNESS_COMPLETION_WORKTREE_DIRTY",
                message: "working tree contains changes other than recognized evidence-preserving metadata transitions",
            },
            issues,
        );
    }
}

fn validate_commit_history(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    issues: &mut Vec<SemanticIssue>,
) {
    let range = format!("{revision}..HEAD");
    let output = match git_output_bytes(
        project_root,
        &["log", "--format=", "--name-only", "-z", &range, "--"],
    ) {
        Ok(output) => output,
        Err(message) => {
            issues.push(freshness_issue(
                "FRESHNESS_COMPLETION_GIT_FAILED",
                "/completion/implementation_revision",
                message,
            ));
            return;
        }
    };
    match nul_paths(&output) {
        Ok(paths) => validate_completion_paths(
            project_root,
            specbind_root,
            revision,
            &paths,
            None,
            CompletionPathPolicy {
                allow_release_binding: true,
                code: "FRESHNESS_COMPLETION_PROJECT_CHANGED",
                message: "commit history since implementation_revision contains a change other than recognized evidence-preserving metadata transitions",
            },
            issues,
        ),
        Err(message) => issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_GIT_OUTPUT_INVALID",
            "/completion/implementation_revision",
            message,
        )),
    }
}

fn validate_current_spec(
    specbind_root: &Path,
    canonical_spec: &str,
    expected: &Spec,
    issues: &mut Vec<SemanticIssue>,
) {
    let path = specbind_root
        .join("specs")
        .join(canonical_spec)
        .join("spec.yaml");
    if !fs::symlink_metadata(&path)
        .is_ok_and(|metadata| crate::guarded_fs::is_regular_file(&metadata))
    {
        issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_CURRENT_SPEC_INVALID",
            "/completion/implementation_revision",
            "current spec.yaml must be a regular non-symlink file",
        ));
        return;
    }
    let current = fs::read_to_string(path)
        .map_err(|error| error.to_string())
        .and_then(|input| runtime::load_spec(&input).map_err(|error| error.to_string()))
        .and_then(|wire| Spec::try_from(wire).map_err(|error| error.to_string()));
    if !current.is_ok_and(|current| &current == expected) {
        issues.push(freshness_issue(
            "FRESHNESS_COMPLETION_CURRENT_SPEC_CHANGED",
            "/completion/implementation_revision",
            "current spec.yaml differs from the completion freshness input",
        ));
    }
}

fn validate_completion_paths(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    paths: &[String],
    required_path: Option<&str>,
    policy: CompletionPathPolicy,
    issues: &mut Vec<SemanticIssue>,
) {
    let paths = paths.iter().collect::<BTreeSet<_>>();
    if required_path.is_some_and(|required| !paths.iter().any(|path| path.as_str() == required)) {
        issues.push(freshness_issue(
            policy.code,
            "/completion/implementation_revision",
            policy.message,
        ));
        return;
    }
    for relative_spec in paths {
        let completion_transition = validate_completion_transition_path(
            project_root,
            specbind_root,
            revision,
            relative_spec,
        );
        let release_binding = policy.allow_release_binding
            && validate_release_binding_transition_path(
                project_root,
                specbind_root,
                revision,
                relative_spec,
            );
        if !completion_transition && !release_binding {
            issues.push(freshness_issue(
                policy.code,
                "/completion/implementation_revision",
                policy.message,
            ));
            return;
        }
    }
}

fn validate_completion_transition_path(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    relative_spec: &str,
) -> bool {
    let Some(_canonical_spec) = completion_path_spec_id(project_root, specbind_root, relative_spec)
    else {
        return false;
    };
    let current = fs::read_to_string(project_root.join(relative_spec))
        .map_err(|error| error.to_string())
        .and_then(|input| runtime::load_spec(&input).map_err(|error| error.to_string()))
        .and_then(|wire| Spec::try_from(wire).map_err(|error| error.to_string()));
    let Ok(current) = current else {
        return false;
    };
    let current_revision = current
        .as_wire()
        .active_change
        .0
        .as_ref()
        .and_then(|active| active.gate_evidence.as_ref())
        .and_then(|evidence| evidence.completion.as_ref())
        .map(|completion| completion.implementation_revision.0.as_str());
    if current_revision != Some(revision) {
        return false;
    }
    let Ok(baseline) = git_output_bytes(
        project_root,
        &["show", &format!("{revision}:{relative_spec}")],
    ) else {
        return false;
    };
    let baseline = std::str::from_utf8(&baseline)
        .map_err(|error| error.to_string())
        .and_then(|input| runtime::load_spec(input).map_err(|error| error.to_string()))
        .and_then(|wire| Spec::try_from(wire).map_err(|error| error.to_string()));
    let Ok(baseline) = baseline else {
        return false;
    };
    let mut expected = current.as_wire().clone();
    let Some(active) = expected.active_change.0.as_mut() else {
        return false;
    };
    active.state = wire::WorkflowState::Implementation;
    if let Some(evidence) = active.gate_evidence.as_mut() {
        evidence.completion = None;
    }
    baseline.as_wire() == &expected
}

fn validate_release_binding_transition_path(
    project_root: &Path,
    specbind_root: &Path,
    revision: &str,
    relative_path: &str,
) -> bool {
    if release_binding_path(project_root, specbind_root).as_deref() != Some(relative_path) {
        return false;
    }
    let current_path = project_root.join(relative_path);
    if !fs::symlink_metadata(&current_path)
        .is_ok_and(|metadata| crate::guarded_fs::is_regular_file(&metadata))
    {
        return false;
    }
    let Ok(current) = fs::read_to_string(&current_path) else {
        return false;
    };
    let Ok(current_roadmap) = roadmap::parse(&current) else {
        return false;
    };
    let Some(current_release) = current_roadmap.target_release.as_deref() else {
        return false;
    };
    let Ok(baseline) = git_output_bytes(
        project_root,
        &["show", &format!("{revision}:{relative_path}")],
    ) else {
        return false;
    };
    let Ok(baseline) = std::str::from_utf8(&baseline) else {
        return false;
    };
    let Ok(baseline_roadmap) = roadmap::parse(baseline) else {
        return false;
    };
    if baseline_roadmap.target_release.as_deref() == Some(current_release) {
        return false;
    }
    matches!(
        roadmap::bind_release(
            baseline,
            current_release,
            baseline_roadmap.target_release.is_some(),
        ),
        Ok(ReleaseBindingEdit::Updated(expected)) if expected == current
    )
}

fn release_binding_path(project_root: &Path, specbind_root: &Path) -> Option<String> {
    let relative = specbind_root.strip_prefix(project_root).ok()?;
    Some(
        relative
            .join("steering")
            .join("roadmap.md")
            .to_string_lossy()
            .replace('\\', "/"),
    )
}

fn completion_path_spec_id<'a>(
    project_root: &Path,
    specbind_root: &Path,
    relative_spec: &'a str,
) -> Option<&'a str> {
    let root = specbind_root
        .strip_prefix(project_root)
        .ok()?
        .to_string_lossy()
        .replace('\\', "/");
    let prefix = if root.is_empty() {
        "specs/".to_owned()
    } else {
        format!("{root}/specs/")
    };
    let canonical_spec = relative_spec
        .strip_prefix(&prefix)?
        .strip_suffix("/spec.yaml")?;
    (!canonical_spec.contains('/') && valid_id(canonical_spec)).then_some(canonical_spec)
}

fn nul_paths(output: &[u8]) -> Result<Vec<String>, String> {
    output
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| {
            std::str::from_utf8(path)
                .map(str::to_owned)
                .map_err(|error| error.to_string())
        })
        .collect()
}

fn git_output(project_root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = git_output_bytes(project_root, arguments)?;
    String::from_utf8(output).map_err(|error| format!("Git output is not UTF-8: {error}"))
}

fn git_output_bytes(project_root: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    repository::output_bytes(project_root, arguments).map_err(|error| error.to_string())
}

fn git_status(project_root: &Path, arguments: &[&str]) -> Result<bool, String> {
    repository::predicate(project_root, arguments).map_err(|error| error.to_string())
}

fn completion_assessment(mut issues: Vec<SemanticIssue>) -> CompletionRevisionAssessment {
    issues.sort();
    issues.dedup();
    CompletionRevisionAssessment { issues }
}

fn completion_spec_path(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
) -> Result<String, SemanticIssue> {
    if !valid_id(canonical_spec) {
        return Err(freshness_issue(
            "FRESHNESS_COMPLETION_SPEC_ID_INVALID",
            "/completion/implementation_revision",
            "canonical spec ID is invalid",
        ));
    }
    let relative = specbind_root.strip_prefix(project_root).map_err(|error| {
        freshness_issue(
            "FRESHNESS_COMPLETION_PROJECT_ROOT_INVALID",
            "/completion/implementation_revision",
            format!("SpecBind root is not below the project root: {error}"),
        )
    })?;
    Ok(relative
        .join("specs")
        .join(canonical_spec)
        .join("spec.yaml")
        .to_string_lossy()
        .replace('\\', "/"))
}

fn valid_id(value: &str) -> bool {
    crate::artifacts::canonical_id(value)
}
