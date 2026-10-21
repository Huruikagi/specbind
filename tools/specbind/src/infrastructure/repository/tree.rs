//! Read committed blobs without checkout, filters, or worktree mutation.
use std::{collections::BTreeMap, path::Path};

use super::output;

pub(crate) struct Tree<'a> {
    project: &'a Path,
    pub revision: String,
}

#[derive(Clone)]
pub(crate) struct Entry {
    mode: String,
    kind: String,
    oid: String,
}

impl Entry {
    pub fn is_directory(&self) -> bool {
        self.mode == "040000" && self.kind == "tree"
    }
}

impl<'a> Tree<'a> {
    pub fn resolve(project: &'a Path, revision: &str) -> Result<Self, String> {
        let revision = output(
            project,
            &[
                "rev-parse",
                "--verify",
                "--end-of-options",
                &format!("{revision}^{{commit}}"),
            ],
        )
        .map_err(|error| error.to_string())?
        .trim()
        .to_owned();
        Ok(Self { project, revision })
    }

    fn entries(&self, tree: &str) -> Result<BTreeMap<String, Entry>, String> {
        let text =
            output(self.project, &["ls-tree", "-z", tree]).map_err(|error| error.to_string())?;
        text.split('\0')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (metadata, name) = line.split_once('\t').ok_or("invalid Git tree entry")?;
                let fields = metadata.split_whitespace().collect::<Vec<_>>();
                if fields.len() != 3 {
                    return Err("invalid Git tree metadata".into());
                }
                Ok((
                    name.to_owned(),
                    Entry {
                        mode: fields[0].into(),
                        kind: fields[1].into(),
                        oid: fields[2].into(),
                    },
                ))
            })
            .collect()
    }

    /// None means a genuinely absent directory; a blob/link/gitlink is an error.
    pub fn directory(&self, path: &str) -> Result<Option<BTreeMap<String, Entry>>, String> {
        let mut tree = self.revision.clone();
        for component in path.split('/').filter(|value| !value.is_empty()) {
            if matches!(component, "." | "..") {
                return Err("invalid committed relative path".into());
            }
            let entries = self.entries(&tree)?;
            let Some(entry) = entries.get(component) else {
                return Ok(None);
            };
            if !entry.is_directory() {
                return Err(format!("{path}: expected a committed regular directory"));
            }
            tree.clone_from(&entry.oid);
        }
        self.entries(&tree).map(Some)
    }

    pub fn blob(&self, entry: &Entry, path: &str) -> Result<String, String> {
        if !matches!(entry.mode.as_str(), "100644" | "100755") || entry.kind != "blob" {
            return Err(format!("{path}: expected a committed regular file"));
        }
        output(self.project, &["cat-file", "blob", &entry.oid]).map_err(|error| error.to_string())
    }

    pub fn optional_file(&self, path: &str) -> Result<Option<String>, String> {
        let (parent, name) = path.rsplit_once('/').unwrap_or(("", path));
        let Some(entries) = self.directory(parent)? else {
            return Ok(None);
        };
        entries
            .get(name)
            .map(|entry| self.blob(entry, path))
            .transpose()
    }
}
