//! Non-authoritative, whole-project Contract comparison against a fixed Git tree.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};

use serde::Serialize;
use serde_json::Value;

use super::shared_contract;
use crate::{
    artifacts, config,
    contract::{ContractDocument, ContractOwner},
    contract_graph,
    fingerprint::{Fingerprint, normalized_contract, normalized_shared_contract},
    guarded_fs,
    repository::tree::Tree,
    schema::runtime,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDiff {
    pub baseline_revision: String,
    pub current_revision: String,
    pub current_source: &'static str,
    pub current_fingerprint: String,
    pub spec_dir: String,
    pub documents: Vec<DocumentDiff>,
    pub entries: Vec<EntryDiff>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentDiff {
    pub owner: String,
    pub change: &'static str,
    pub baseline_path: Option<String>,
    pub current_path: Option<String>,
    pub before: Option<Value>,
    pub after: Option<Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryDiff {
    pub selector: String,
    pub change: &'static str,
    pub before: Option<Value>,
    pub after: Option<Value>,
    pub baseline_consumers: Vec<String>,
    pub current_consumers: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub side: &'static str,
    pub severity: &'static str,
    pub code: &'static str,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Default)]
struct Snapshot {
    contracts: BTreeMap<String, ContractDocument>,
    shared: Option<crate::domain::shared_contract::SharedContract>,
    paths: BTreeMap<String, String>,
}

impl Snapshot {
    fn normalized(&self) -> Result<BTreeMap<String, Value>, String> {
        let mut documents = BTreeMap::new();
        for (spec, contract) in &self.contracts {
            documents.insert(
                format!("specs/{spec}"),
                serde_json::to_value(normalized_contract(contract)).map_err(|e| e.to_string())?,
            );
        }
        if let Some(shared) = normalized_shared_contract(self.shared.as_ref()) {
            documents.insert(
                "shared".into(),
                serde_json::to_value(shared).map_err(|e| e.to_string())?,
            );
        }
        Ok(documents)
    }

    fn fingerprint(&self) -> Result<String, String> {
        let bytes = serde_json_canonicalizer::to_vec(&(self.normalized()?, &self.paths))
            .map_err(|e| e.to_string())?;
        Ok(Fingerprint::markdown(&bytes).to_string())
    }

    // Preserve declared references even when the target was removed or is dangling.
    fn consumers(&self, selector: &str) -> Vec<String> {
        let mut consumers = BTreeSet::new();
        for (spec, document) in &self.contracts {
            for consume in &document.consumes {
                let target = contract_graph::ContractEntryRef {
                    owner: consume.target.owner.clone(),
                    section: consume.target.section,
                    entry_id: consume.target.entry_id.clone(),
                };
                if contract_graph::entry_selector(&target) == selector {
                    consumers.insert(format!("specs/{spec}#contract/consumes/{}", consume.id));
                }
            }
        }
        consumers.into_iter().collect()
    }

    fn diagnostics(&self, side: &'static str) -> Vec<Diagnostic> {
        contract_graph::evaluate_with_shared(
            &self.contracts.keys().cloned().collect(),
            self.contracts.clone(),
            self.shared.clone(),
        )
        .issues
        .into_iter()
        .map(|issue| Diagnostic {
            side,
            severity: if issue.severity == contract_graph::GraphIssueSeverity::Error {
                "error"
            } else {
                "warning"
            },
            code: issue.code,
            source: issue.source,
            message: issue.message,
        })
        .collect()
    }
}

fn parse_contract(text: &str, path: &str) -> Result<ContractDocument, String> {
    let wire = runtime::load_contract(text).map_err(|error| format!("{path}: {error:?}"))?;
    ContractDocument::try_from(wire).map_err(|error| format!("{path}: {error:?}"))
}

fn current(paths: &config::ProjectPaths, relative: &str) -> Result<Snapshot, String> {
    let root = &paths.specbind_root;
    match fs::symlink_metadata(root.join("specs")) {
        Ok(meta) if !meta.is_dir() || guarded_fs::is_link_like(&meta) => {
            return Err("specs must be a regular directory".into());
        }
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => return Err(error.to_string()),
        _ => {}
    }
    let discovery = artifacts::discover_spec_ids(root)?;
    if !discovery.faults.is_empty() {
        return Err(format!(
            "invalid persistent Spec inventory: {:?}",
            discovery.faults
        ));
    }
    let mut snapshot = Snapshot::default();
    for spec in discovery.specs {
        let directory = root.join("specs").join(&spec);
        let metadata = fs::symlink_metadata(&directory).map_err(|e| e.to_string())?;
        if !metadata.is_dir() || guarded_fs::is_link_like(&metadata) {
            return Err(format!("{spec}: expected a regular Spec directory"));
        }
        let path = format!("{relative}/specs/{spec}/contract.yaml");
        let metadata = fs::symlink_metadata(paths.project_root.join(&path))
            .map_err(|e| format!("{path}: required Contract unavailable: {e}"))?;
        if !guarded_fs::is_regular_file(&metadata) {
            return Err(format!("{path}: expected a regular Contract file"));
        }
        let text = fs::read_to_string(paths.project_root.join(&path))
            .map_err(|e| format!("{path}: {e}"))?;
        snapshot
            .contracts
            .insert(spec.clone(), parse_contract(&text, &path)?);
        snapshot.paths.insert(format!("specs/{spec}"), path);
    }
    snapshot.shared = shared_contract::read(root)?;
    if let Some(path) = shared_contract::present_relative(root)? {
        snapshot
            .paths
            .insert("shared".into(), format!("{relative}/{path}"));
    }
    Ok(snapshot)
}

fn baseline(tree: &Tree<'_>, relative: &str) -> Result<Snapshot, String> {
    let mut snapshot = Snapshot::default();
    for (spec, entry) in tree
        .directory(&format!("{relative}/specs"))?
        .unwrap_or_default()
    {
        if matches!(spec.as_str(), "index.md" | "shared-contract.yaml") {
            continue;
        }
        if !artifacts::canonical_id(&spec) || !entry.is_directory() {
            return Err(format!(
                "baseline specs/{spec}: expected a canonical Spec directory"
            ));
        }
        let path = format!("{relative}/specs/{spec}/contract.yaml");
        let text = tree
            .optional_file(&path)?
            .ok_or_else(|| format!("baseline {path}: existing Spec has no required Contract"))?;
        snapshot
            .contracts
            .insert(spec.clone(), parse_contract(&text, &path)?);
        snapshot.paths.insert(format!("specs/{spec}"), path);
    }
    if let Some((path, document)) = shared_contract::at_baseline(tree, relative)? {
        snapshot.paths.insert("shared".into(), path);
        snapshot.shared = Some(document);
    }
    Ok(snapshot)
}

fn change(before: Option<&Value>, after: Option<&Value>) -> &'static str {
    match (before, after) {
        (None, Some(_)) => "added",
        (Some(_), None) => "removed",
        _ if before == after => "unchanged",
        _ => "changed",
    }
}

fn entries(documents: &BTreeMap<String, Value>) -> BTreeMap<String, Value> {
    let mut entries = BTreeMap::new();
    for (owner, document) in documents {
        for section in [
            "owns",
            "exports",
            "consumes",
            "invariants",
            "file_ownership",
            "resources",
        ] {
            for entry in document[section].as_array().into_iter().flatten() {
                let id = entry["id"].as_str().expect("validated entry ID");
                let section = section.replace('_', "-");
                let selector = if owner == "shared" {
                    contract_graph::entry_selector(&contract_graph::ContractEntryRef {
                        owner: ContractOwner::Shared,
                        section: crate::contract::ContractSection::Resources,
                        entry_id: id.into(),
                    })
                } else {
                    format!("{owner}#contract/{section}/{id}")
                };
                entries.insert(selector, entry.clone());
            }
        }
    }
    entries
}

/// Compares complete validated manifests; graph diagnostics do not hide the diff.
/// # Errors
/// Fails on incomplete, malformed, unsafe, unreadable, or moving inputs.
pub fn resolve(paths: &config::ProjectPaths, revision: &str) -> Result<ContractDiff, String> {
    let relative = paths
        .specbind_root
        .strip_prefix(&paths.project_root)
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .replace('\\', "/");
    let tree = Tree::resolve(&paths.project_root, revision)?;
    if let Some(text) = tree.optional_file(".specbind.json")? {
        let configuration: Value =
            serde_json::from_str(&text).map_err(|e| format!("baseline .specbind.json: {e}"))?;
        if configuration["specDir"].as_str() != Some(relative.as_str()) {
            return Err("baseline specDir differs from the current root; cannot infer moved Contract identities".into());
        }
    }
    let head = Tree::resolve(&paths.project_root, "HEAD")?.revision;
    let before = baseline(&tree, &relative)?;
    let after = current(paths, &relative)?;
    let before_documents = before.normalized()?;
    let after_documents = after.normalized()?;
    let owners = before_documents
        .keys()
        .chain(after_documents.keys())
        .collect::<BTreeSet<_>>();
    let documents = owners
        .into_iter()
        .map(|owner| DocumentDiff {
            owner: owner.clone(),
            change: change(before_documents.get(owner), after_documents.get(owner)),
            baseline_path: before.paths.get(owner).cloned(),
            current_path: after.paths.get(owner).cloned(),
            before: before_documents.get(owner).cloned(),
            after: after_documents.get(owner).cloned(),
        })
        .collect();
    let before_entries = entries(&before_documents);
    let after_entries = entries(&after_documents);
    let selectors = before_entries
        .keys()
        .chain(after_entries.keys())
        .collect::<BTreeSet<_>>();
    let entries = selectors
        .into_iter()
        .filter(|selector| before_entries.get(*selector) != after_entries.get(*selector))
        .map(|selector| EntryDiff {
            selector: selector.clone(),
            change: change(before_entries.get(selector), after_entries.get(selector)),
            before: before_entries.get(selector).cloned(),
            after: after_entries.get(selector).cloned(),
            baseline_consumers: before.consumers(selector),
            current_consumers: after.consumers(selector),
        })
        .collect();
    let mut diagnostics = before.diagnostics("baseline");
    diagnostics.extend(after.diagnostics("current"));
    let fingerprint = after.fingerprint()?;
    let latest_paths = config::resolve_from(&paths.project_root).map_err(|e| e.to_string())?;
    if latest_paths != *paths
        || Tree::resolve(&paths.project_root, "HEAD")?.revision != head
        || current(paths, &relative)?.fingerprint()? != fingerprint
    {
        return Err(
            "Contract comparison inputs changed during the read; rerun and reassess".into(),
        );
    }
    Ok(ContractDiff {
        baseline_revision: tree.revision,
        current_revision: head,
        current_source: "worktree",
        current_fingerprint: fingerprint,
        spec_dir: relative,
        documents,
        entries,
        diagnostics,
    })
}
