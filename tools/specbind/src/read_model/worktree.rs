//! Compare observations without inferring authorship, intent or commit authority.
use crate::{
    repository::worktree,
    schema::{
        self,
        worktree_snapshot::v1::{FileState, PathSnapshot, WorktreeSnapshot},
    },
};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::LazyLock,
};

static VALIDATOR: LazyLock<jsonschema::Validator> = LazyLock::new(|| {
    let schema =
        serde_json::from_str(schema::WORKTREE_SNAPSHOT_V1_SCHEMA_JSON).expect("embedded schema");
    jsonschema::draft202012::new(&schema).expect("snapshot schema")
});

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorktreeCheck {
    pub result: &'static str,
    pub baseline_revision: String,
    pub current_revision: String,
    pub head_changed: bool,
    pub changes: Vec<PathChange>,
    pub unexpected_paths: Vec<String>,
    pub preexisting_touched_paths: Vec<String>,
    pub declared_unchanged_paths: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathChange {
    pub path: String,
    pub index_changed: bool,
    pub worktree_changed: bool,
    pub declared: bool,
    pub before: Option<PathSnapshot>,
    pub after: Option<PathSnapshot>,
}

impl WorktreeCheck {
    #[must_use]
    pub fn matched(&self) -> bool {
        !self.head_changed && self.unexpected_paths.is_empty()
    }
}

/// Captures tracked and nonignored untracked paths at a committed HEAD.
/// # Errors
/// Rejects unsupported or unstable repository/file state.
pub fn capture(root: &Path) -> Result<WorktreeSnapshot, String> {
    worktree::capture(root, &BTreeSet::new())
}

fn parse(input: &str) -> Result<WorktreeSnapshot, String> {
    // Deserialize first so duplicate fields are not silently collapsed into Value.
    let snapshot: WorktreeSnapshot = serde_json::from_str(input).map_err(|e| e.to_string())?;
    let value = serde_json::from_str(input).map_err(|e| e.to_string())?;
    VALIDATOR.validate(&value).map_err(|e| e.to_string())?;
    let mut seen = BTreeSet::new();
    for entry in &snapshot.paths {
        if !worktree::valid_path(&entry.path) || !seen.insert(&entry.path) {
            return Err(format!(
                "invalid or duplicate snapshot path: {:?}",
                entry.path
            ));
        }
    }
    Ok(snapshot)
}

fn same_index(before: Option<&PathSnapshot>, after: Option<&PathSnapshot>) -> bool {
    before.and_then(|v| v.index.as_ref()) == after.and_then(|v| v.index.as_ref())
}

fn same_worktree(before: Option<&PathSnapshot>, after: Option<&PathSnapshot>) -> bool {
    before.map_or(&FileState::Missing, |v| &v.worktree)
        == after.map_or(&FileState::Missing, |v| &v.worktree)
}

/// Checks exact declared paths against two observations; never mutates the tree.
/// # Errors
/// Rejects invalid snapshots, mismatched repositories and incomplete reads.
pub fn check(root: &Path, input: &str, declared: &[String]) -> Result<WorktreeCheck, String> {
    let before = parse(input)?;
    if root.canonicalize().map_err(|e| e.to_string())?.to_str()
        != Some(before.repository_root.as_str())
    {
        return Err("snapshot belongs to another worktree".into());
    }
    let mut allowed = BTreeSet::new();
    for name in declared {
        if !worktree::valid_path(name) || !allowed.insert(name.clone()) {
            return Err(format!("invalid or duplicate declared path: {name:?}"));
        }
    }
    let retained = before.paths.iter().map(|p| p.path.clone()).collect();
    let after = worktree::capture(root, &retained)?;
    if before.git_directory != after.git_directory {
        return Err("snapshot belongs to another Git directory".into());
    }
    let baseline_revision = before.head_revision;
    let current_revision = after.head_revision;
    let old = before
        .paths
        .into_iter()
        .map(|p| (p.path.clone(), p))
        .collect::<BTreeMap<_, _>>();
    let new = after
        .paths
        .into_iter()
        .map(|p| (p.path.clone(), p))
        .collect::<BTreeMap<_, _>>();
    let names = old.keys().chain(new.keys()).collect::<BTreeSet<_>>();
    let changes = names
        .into_iter()
        .filter_map(|name| {
            let before = old.get(name);
            let after = new.get(name);
            let index_changed = !same_index(before, after);
            let worktree_changed = !same_worktree(before, after);
            (index_changed || worktree_changed).then(|| PathChange {
                path: name.clone(),
                index_changed,
                worktree_changed,
                declared: allowed.contains(name),
                before: before.cloned(),
                after: after.cloned(),
            })
        })
        .collect::<Vec<_>>();
    let unexpected_paths = changes
        .iter()
        .filter(|p| !p.declared)
        .map(|p| p.path.clone())
        .collect::<Vec<_>>();
    let preexisting_touched_paths = changes
        .iter()
        .filter(|p| p.before.as_ref().is_some_and(|before| before.dirty))
        .map(|p| p.path.clone())
        .collect();
    let changed_names = changes.iter().map(|p| &p.path).collect::<BTreeSet<_>>();
    let declared_unchanged_paths = allowed
        .iter()
        .filter(|p| !changed_names.contains(p))
        .cloned()
        .collect();
    let head_changed = baseline_revision != current_revision;
    let result = if head_changed || !unexpected_paths.is_empty() {
        "WORKTREE_CHECK_MISMATCH"
    } else {
        "WORKTREE_CHECK_MATCHED"
    };
    Ok(WorktreeCheck {
        result,
        baseline_revision,
        current_revision,
        head_changed,
        changes,
        unexpected_paths,
        preexisting_touched_paths,
        declared_unchanged_paths,
    })
}
