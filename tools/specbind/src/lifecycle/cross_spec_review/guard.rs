use std::{fs, path::Path};

use crate::{
    artifacts::resolve_gate_inputs,
    domain::spec::Spec,
    freshness::{self, FreshnessStatus},
    repository,
    schema::{runtime, spec::v1::WorkflowState},
};

use super::{
    ROADMAP_KEY, ReviewInputResolution, ReviewIssue, ReviewIssues, resolution::read_regular,
    review_issue,
};

pub(super) fn validate_acceptance_guards(
    project_root: &Path,
    specbind_root: &Path,
    resolution: &ReviewInputResolution,
) -> Result<(), ReviewIssues> {
    let mut issues = Vec::new();
    validate_baseline(
        project_root,
        &resolution.roadmap.baseline_revision,
        &mut issues,
    );
    for spec in resolution.roadmap.spec_ids() {
        validate_participating_spec(
            specbind_root,
            &spec,
            &resolution.roadmap.milestone_id,
            &mut issues,
        );
    }
    if issues.is_empty() {
        Ok(())
    } else {
        issues.sort();
        issues.dedup();
        Err(ReviewIssues { issues })
    }
}

pub(super) fn validate_baseline(
    project_root: &Path,
    baseline: &str,
    issues: &mut Vec<ReviewIssue>,
) {
    let resolved = git_output(
        project_root,
        &["rev-parse", "--verify", &format!("{baseline}^{{commit}}")],
    );
    match resolved {
        Ok(value) if value.trim() == baseline => {}
        Ok(_) => issues.push(review_issue(
            "CONTRACT_REVIEW_BASELINE_NOT_EXACT",
            Some(ROADMAP_KEY.to_owned()),
            "baseline_revision must resolve to the same full commit object ID",
        )),
        Err(message) => {
            issues.push(review_issue(
                "CONTRACT_REVIEW_BASELINE_MISSING",
                Some(ROADMAP_KEY.to_owned()),
                message,
            ));
            return;
        }
    }
    match git_status(
        project_root,
        &["merge-base", "--is-ancestor", baseline, "HEAD"],
    ) {
        Ok(true) => {}
        Ok(false) => issues.push(review_issue(
            "CONTRACT_REVIEW_BASELINE_NOT_ANCESTOR",
            Some(ROADMAP_KEY.to_owned()),
            "baseline_revision is not an ancestor of current HEAD",
        )),
        Err(message) => issues.push(review_issue(
            "CONTRACT_REVIEW_GIT_FAILED",
            Some(ROADMAP_KEY.to_owned()),
            message,
        )),
    }
}

fn validate_participating_spec(
    specbind_root: &Path,
    canonical_spec: &str,
    milestone_id: &str,
    issues: &mut Vec<ReviewIssue>,
) {
    let source = format!("specs/{canonical_spec}/spec.yaml");
    let Some(bytes) = read_regular(&specbind_root.join(&source), &source, issues) else {
        return;
    };
    let input = match std::str::from_utf8(&bytes) {
        Ok(input) => input,
        Err(error) => {
            issues.push(review_issue(
                "CONTRACT_REVIEW_SPEC_NOT_UTF8",
                Some(source),
                error.to_string(),
            ));
            return;
        }
    };
    let wire = match runtime::load_spec(input) {
        Ok(wire) => wire,
        Err(error) => {
            issues.push(review_issue(
                "CONTRACT_REVIEW_SPEC_INVALID",
                Some(source),
                error.to_string(),
            ));
            return;
        }
    };
    let spec = match Spec::try_from(wire) {
        Ok(spec) => spec,
        Err(error) => {
            issues.push(review_issue(
                "CONTRACT_REVIEW_SPEC_INVALID",
                Some(source),
                error.to_string(),
            ));
            return;
        }
    };
    let Some(active) = spec.as_wire().active_change.0.as_ref() else {
        issues.push(review_issue(
            "CONTRACT_REVIEW_SPEC_STATE_INVALID",
            Some(source),
            "participating spec must have an active change ready for contract review",
        ));
        return;
    };
    if active.milestone_id.0 != milestone_id
        || !matches!(
            active.state,
            WorkflowState::Tasks | WorkflowState::AdoptionReady
        )
    {
        issues.push(review_issue(
            "CONTRACT_REVIEW_SPEC_STATE_INVALID",
            Some(source.clone()),
            "participating spec must match the Roadmap milestone and be ready for contract review",
        ));
    }
    let gate_inputs = resolve_gate_inputs(specbind_root, canonical_spec);
    for value in gate_inputs.inventory.issues {
        issues.push(review_issue(
            value.code,
            value.path.map(|path| path.to_string()),
            value.message,
        ));
    }
    let freshness = freshness::evaluate(&spec, &gate_inputs.inputs);
    if freshness.design.status != FreshnessStatus::Fresh {
        issues.push(review_issue(
            "CONTRACT_REVIEW_DESIGN_NOT_FRESH",
            Some(source),
            "participating spec requires a fresh Design gate",
        ));
    }
    let tasks = specbind_root.join(format!("specs/{canonical_spec}/tasks.yaml"));
    match fs::symlink_metadata(tasks) {
        Ok(_) => issues.push(review_issue(
            "CONTRACT_REVIEW_TASKS_ALREADY_EXIST",
            Some(format!("specs/{canonical_spec}/tasks.yaml")),
            "tasks.yaml must not exist before contract review acceptance",
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => issues.push(review_issue(
            "CONTRACT_REVIEW_TASKS_INSPECT_FAILED",
            Some(format!("specs/{canonical_spec}/tasks.yaml")),
            error.to_string(),
        )),
    }
}

fn git_output(project_root: &Path, arguments: &[&str]) -> Result<String, String> {
    repository::output(project_root, arguments).map_err(|error| error.to_string())
}

fn git_status(project_root: &Path, arguments: &[&str]) -> Result<bool, String> {
    repository::predicate(project_root, arguments).map_err(|error| error.to_string())
}
