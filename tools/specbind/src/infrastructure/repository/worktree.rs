//! Complete observed path state; no index refresh, checkout or temporary writes.
use crate::{
    guarded_fs,
    schema::worktree_snapshot::v1::{FileState, IndexEntry, PathSnapshot, WorktreeSnapshot},
};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read as _,
    path::Path,
};

fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    let mut command = vec!["--no-optional-locks", "-c", "core.fsmonitor=false"];
    command.extend(args);
    super::output(root, &command).map_err(|e| e.to_string())
}

pub(crate) fn valid_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':', '\0'])
        && path.split('/').all(|part| {
            !part.is_empty() && !part.ends_with([' ', '.']) && !part.eq_ignore_ascii_case(".git")
        })
}

fn path(value: &str) -> Result<String, String> {
    if !valid_path(value) {
        return Err(format!("unsupported project-relative path: {value:?}"));
    }
    Ok(value.to_owned())
}

fn identity(root: &Path) -> Result<(String, String, String), String> {
    let root_name = root
        .canonicalize()
        .map_err(|e| e.to_string())?
        .into_os_string()
        .into_string()
        .map_err(|_| "repository root must be UTF-8")?;
    let git_dir = git(root, &["rev-parse", "--absolute-git-dir"])?;
    let git_dir = Path::new(git_dir.trim())
        .canonicalize()
        .map_err(|e| e.to_string())?
        .into_os_string()
        .into_string()
        .map_err(|_| "Git directory must be UTF-8")?;
    let head = git(root, &["rev-parse", "--verify", "HEAD^{commit}"])?;
    Ok((root_name, git_dir, head.trim().into()))
}

fn index(root: &Path) -> Result<BTreeMap<String, IndexEntry>, String> {
    let mut result = BTreeMap::new();
    for line in git(root, &["ls-files", "--stage", "-z"])?
        .split('\0')
        .filter(|s| !s.is_empty())
    {
        let (meta, name) = line.split_once('\t').ok_or("invalid index record")?;
        let fields = meta.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 3 || fields[2] != "0" || !matches!(fields[0], "100644" | "100755") {
            return Err(format!(
                "{name}: conflicts, links, submodules and sparse index entries are unsupported"
            ));
        }
        result.insert(
            path(name)?,
            IndexEntry {
                mode: fields[0].into(),
                oid: fields[1].into(),
            },
        );
    }
    // Hidden tracked entries would make Git's dirty-state report incomplete.
    for line in git(root, &["ls-files", "-v", "-z"])?
        .split('\0')
        .filter(|s| !s.is_empty())
    {
        if !line.starts_with("H ") {
            return Err(format!(
                "unsupported index flag: {line:?}; sparse/assume-unchanged entries cannot be captured"
            ));
        }
    }
    let visible = git(
        root,
        &[
            "diff",
            "--cached",
            "--name-only",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            "--ita-visible-in-index",
        ],
    )?;
    let invisible = git(
        root,
        &[
            "diff",
            "--cached",
            "--name-only",
            "-z",
            "--no-renames",
            "--no-ext-diff",
            "--ita-invisible-in-index",
        ],
    )?;
    if visible != invisible {
        return Err("intent-to-add index entries are unsupported".into());
    }
    Ok(result)
}

fn tracked_at_head(root: &Path, head: &str) -> Result<BTreeSet<String>, String> {
    let mut paths = BTreeSet::new();
    for line in git(root, &["ls-tree", "-r", "-z", head])?
        .split('\0')
        .filter(|s| !s.is_empty())
    {
        let (meta, name) = line.split_once('\t').ok_or("invalid HEAD tree record")?;
        if !meta.starts_with("100644 blob ") && !meta.starts_with("100755 blob ") {
            return Err(format!("{name}: links and submodules are unsupported"));
        }
        paths.insert(path(name)?);
    }
    Ok(paths)
}

fn dirty_paths(root: &Path) -> Result<BTreeSet<String>, String> {
    git(
        root,
        &[
            "-c",
            "status.renames=false",
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--ignore-submodules=none",
        ],
    )?
    .split('\0')
    .filter(|s| !s.is_empty())
    .map(|line| {
        let name = line.get(3..).ok_or("invalid Git status record")?;
        path(name)
    })
    .collect()
}

#[cfg(unix)]
fn executable(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt as _;
    metadata.permissions().mode() & 0o111 != 0
}
#[cfg(not(unix))]
fn executable(_: &fs::Metadata) -> bool {
    false
}

fn file_state(root: &Path, relative: &str) -> Result<FileState, String> {
    let mut current = root.to_path_buf();
    let parts = relative.split('/').collect::<Vec<_>>();
    for (i, part) in parts.iter().enumerate() {
        current.push(part);
        let metadata = match fs::symlink_metadata(&current) {
            Ok(meta) => meta,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(FileState::Missing),
            Err(e) => return Err(format!("{relative}: {e}")),
        };
        if guarded_fs::is_link_like(&metadata) {
            return Err(format!(
                "{relative}: links and reparse points are unsupported"
            ));
        }
        if i + 1 != parts.len() {
            if !metadata.is_dir() {
                return Ok(FileState::Missing);
            }
            continue;
        }
        if metadata.is_dir() {
            return Ok(FileState::Directory);
        }
        if !metadata.is_file() {
            return Err(format!("{relative}: unsupported file type"));
        }
        let mut file = fs::File::open(&current).map_err(|e| format!("{relative}: {e}"))?;
        let mut digest = Sha256::new();
        let mut buffer = [0_u8; 8192];
        loop {
            let count = file
                .read(&mut buffer)
                .map_err(|e| format!("{relative}: {e}"))?;
            if count == 0 {
                break;
            }
            digest.update(&buffer[..count]);
        }
        return Ok(FileState::File {
            digest: format!("sha256:{}", hex::encode(digest.finalize())),
            executable: executable(&metadata),
        });
    }
    Err("empty worktree path".into())
}

fn observe(root: &Path, retained: &BTreeSet<String>) -> Result<WorktreeSnapshot, String> {
    let (repository_root, git_directory, head_revision) = identity(root)?;
    let index = index(root)?;
    let mut names = tracked_at_head(root, &head_revision)?;
    names.extend(index.keys().cloned());
    let untracked = git(root, &["ls-files", "--others", "--exclude-standard", "-z"])?;
    for name in untracked.split('\0').filter(|s| !s.is_empty()) {
        // A trailing slash denotes an untracked nested repository, not a file.
        names.insert(path(name)?);
    }
    names.extend(retained.iter().cloned());
    let dirty = dirty_paths(root)?;
    let paths = names
        .into_iter()
        .map(|name| {
            let worktree = file_state(root, &name)?;
            Ok(PathSnapshot {
                index: index.get(&name).cloned(),
                dirty: dirty.contains(&name),
                path: name,
                worktree,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(WorktreeSnapshot {
        schema_version: 1,
        repository_root,
        git_directory,
        head_revision,
        paths,
    })
}

pub(crate) fn capture(
    root: &Path,
    retained: &BTreeSet<String>,
) -> Result<WorktreeSnapshot, String> {
    let first = observe(root, retained)?;
    if first != observe(root, retained)? {
        return Err("worktree changed while reading; stop and capture a stable state".into());
    }
    Ok(first)
}
