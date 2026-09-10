use super::{
    Mutation, Path, RemovalIssues, RemovalPlan, fs, guarded_fs, one_issue, revalidate_file,
    validate_repository_state, validate_tree,
};

pub(super) fn apply_plan(project_root: &Path, plan: &RemovalPlan) -> Result<(), RemovalIssues> {
    if plan.unchanged {
        return Ok(());
    }
    let removable_tree = plan
        .entries
        .iter()
        .find(|entry| matches!(entry.mutation, Mutation::RemoveTree))
        .map(|entry| entry.path.as_str());
    validate_repository_state(project_root, &plan.entries, removable_tree)?;
    for entry in &plan.entries {
        match &entry.mutation {
            Mutation::None => {}
            Mutation::RemoveFile => {
                revalidate_file(project_root, entry)?;
                fs::remove_file(project_root.join(&entry.path)).map_err(|error| {
                    one_issue(
                        "REMOVAL_WRITE_FAILED",
                        Some(entry.path.clone()),
                        error.to_string(),
                    )
                })?;
            }
            Mutation::RemoveTree => {
                validate_tree(project_root, &entry.path)?;
                fs::remove_dir_all(project_root.join(&entry.path)).map_err(|error| {
                    one_issue(
                        "REMOVAL_WRITE_FAILED",
                        Some(entry.path.clone()),
                        error.to_string(),
                    )
                })?;
            }
            Mutation::Replace(content) => {
                revalidate_file(project_root, entry)?;
                guarded_fs::replace_existing(&project_root.join(&entry.path), content).map_err(
                    |error| {
                        one_issue(
                            "REMOVAL_WRITE_FAILED",
                            Some(entry.path.clone()),
                            error.to_string(),
                        )
                    },
                )?;
            }
        }
    }
    Ok(())
}
