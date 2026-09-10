use super::{ApprovalIssue, ApprovalIssues, Gate, issue, one_issue, spec_path};
use crate::{
    domain::spec::Spec, guarded_fs, infrastructure::repository, schema::spec::v1::SpecDocument,
};
use std::{fs, path::Path};

pub(super) fn persist(
    specbind_root: &Path,
    canonical_spec: &str,
    wire: &SpecDocument,
    gate: Gate,
) -> Result<(), ApprovalIssues> {
    Spec::try_from(wire.clone()).map_err(|error| ApprovalIssues {
        issues: error
            .issues
            .into_iter()
            .map(|value| issue(value.code, Some(spec_path(canonical_spec)), value.message))
            .collect(),
    })?;
    let mut rendered = serde_saphyr::to_string(wire).map_err(|error| {
        one_issue(
            "SPEC_GATE_SERIALIZE_FAILED",
            Some(spec_path(canonical_spec)),
            error.to_string(),
        )
    })?;
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    guarded_fs::replace_existing(
        &specbind_root.join(spec_path(canonical_spec)),
        rendered.as_bytes(),
    )
    .map_err(|error| {
        one_issue(
            gate.target_invalid(),
            Some(spec_path(canonical_spec)),
            error.to_string(),
        )
    })
}

pub(super) fn ensure_target_clean(
    project_root: &Path,
    specbind_root: &Path,
    canonical_spec: &str,
    gate: Gate,
    issues: &mut Vec<ApprovalIssue>,
) {
    let relative = spec_path(canonical_spec);
    let Ok(root_relative) = specbind_root.strip_prefix(project_root) else {
        issues.push(issue(
            "SPEC_GATE_PROJECT_ROOT_INVALID",
            Some(relative),
            "SpecBind root must be below the Git project root",
        ));
        return;
    };
    let path = root_relative
        .join(&relative)
        .to_string_lossy()
        .replace('\\', "/");
    match repository::path_status(project_root, &path) {
        Ok(output) if output.is_empty() => {}
        Ok(_) => issues.push(issue(
            gate.target_dirty(),
            Some(relative),
            "gate invalidation refuses to overwrite a dirty or staged spec.yaml",
        )),
        Err(error) => issues.push(issue(
            "SPEC_GATE_GIT_FAILED",
            Some(relative),
            error.to_string(),
        )),
    }
}

pub(super) fn read_regular(
    specbind_root: &Path,
    canonical_spec: &str,
    gate: Gate,
) -> Result<String, ApprovalIssues> {
    let relative = spec_path(canonical_spec);
    let path = specbind_root.join(&relative);
    let metadata = fs::symlink_metadata(&path).map_err(|error| {
        one_issue(
            gate.target_invalid(),
            Some(relative.clone()),
            error.to_string(),
        )
    })?;
    if !crate::guarded_fs::is_regular_file(&metadata) {
        return Err(one_issue(
            gate.target_invalid(),
            Some(relative),
            "spec.yaml must be a regular non-symlink file",
        ));
    }
    fs::read_to_string(&path)
        .map_err(|error| one_issue(gate.target_invalid(), Some(relative), error.to_string()))
}
