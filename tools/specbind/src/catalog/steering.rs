//! Catalog and read model for project-level steering documents.
//!
//! Decision 0098 makes durable project guidance an OKF collection identified by
//! `type: SpecBind Steering` and a stable `artifact_id`, read only through the
//! CLI. Discovery is deliberately narrower than the spec-local artifact model:
//! there is one profile, its `artifact_id` is its selector, and any diagnostic
//! at all makes a read fail, because a consumer cannot safely act on project
//! guidance it knows to be incomplete.

use std::{fs, path::Path};

use camino::Utf8PathBuf;
use serde_json::Value;
use walkdir::WalkDir;

use crate::artifacts::{DiscoveryIssue, is_kebab_id, split_frontmatter};
use crate::instruction;

pub const TYPE_STEERING: &str = "SpecBind Steering";

const STEERING_ROOT: &str = "steering";

/// One recognized steering document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringDocument {
    /// The bare `artifact_id`, which is this profile's selector.
    pub selector: String,
    pub artifact_type: String,
    pub path: Utf8PathBuf,
}

/// The compact inventory, with any per-document faults reported beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringInventory {
    pub documents: Vec<SteeringDocument>,
    pub issues: Vec<DiscoveryIssue>,
}

/// Why one steering read could not produce the requested document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringReadFailure {
    pub code: &'static str,
    pub message: String,
    pub issues: Vec<DiscoveryIssue>,
}

/// Discovers every steering document below the `SpecBind` root.
///
/// # Errors
///
/// Returns a message only when `steering/` itself cannot be used: an absent
/// directory is an empty inventory, because steering is optional and a project
/// that has never authored any correctly has none.
pub fn discover(specbind_root: &Path) -> Result<SteeringInventory, String> {
    let root = specbind_root.join(STEERING_ROOT);
    match fs::symlink_metadata(&root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(SteeringInventory {
                documents: vec![],
                issues: vec![],
            });
        }
        Err(error) => return Err(error.to_string()),
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            return Err(format!(
                "{STEERING_ROOT} must be a regular non-symlink directory"
            ));
        }
        Ok(_) => {}
    }

    let mut documents = Vec::<SteeringDocument>::new();
    let mut issues = Vec::new();
    for entry in WalkDir::new(&root).follow_links(false).sort_by_file_name() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                issues.push(issue(
                    "STEERING_SCAN_FAILED",
                    Some(Utf8PathBuf::from(STEERING_ROOT)),
                    error.to_string(),
                ));
                continue;
            }
        };
        let file_type = entry.file_type();
        if file_type.is_symlink() {
            issues.push(issue(
                "STEERING_TARGET_INVALID",
                relative(specbind_root, entry.path()),
                "steering entries must not be symbolic links",
            ));
            continue;
        }
        if !file_type.is_file()
            || entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "md")
        {
            continue;
        }
        let Some(path) = relative(specbind_root, entry.path()) else {
            issues.push(issue(
                "STEERING_PATH_NOT_UTF8",
                None,
                "steering path must be UTF-8",
            ));
            continue;
        };
        match classify(entry.path(), &path) {
            Ok((Some(document), mut found)) => {
                documents.push(document);
                issues.append(&mut found);
            }
            Ok((None, mut found)) => issues.append(&mut found),
            Err(fault) => issues.push(fault),
        }
    }

    // Decision 0057 makes a duplicate collection ID a hard discovery error. Both
    // documents are dropped rather than one being chosen, because a selector
    // that resolves to two documents cannot be read at all.
    documents.sort_by(|left, right| left.selector.cmp(&right.selector));
    let mut unique = Vec::with_capacity(documents.len());
    let mut index = 0;
    while index < documents.len() {
        let span = documents[index..]
            .iter()
            .take_while(|document| document.selector == documents[index].selector)
            .count();
        if span == 1 {
            unique.push(documents[index].clone());
        } else {
            for document in &documents[index..index + span] {
                issues.push(issue(
                    "STEERING_ARTIFACT_ID_DUPLICATE",
                    Some(document.path.clone()),
                    format!("steering artifact_id is duplicated: {}", document.selector),
                ));
            }
        }
        index += span;
    }
    Ok(SteeringInventory {
        documents: unique,
        issues,
    })
}

/// Reads one steering selector as raw UTF-8 Markdown.
///
/// # Errors
///
/// Returns a focused diagnostic when the selector cannot resolve, and the
/// collection-wide failure when it resolves but the inventory carries any
/// diagnostic at all.
pub fn read(specbind_root: &Path, selector: &str) -> Result<String, SteeringReadFailure> {
    let inventory = discover(specbind_root).map_err(|message| SteeringReadFailure {
        code: "STEERING_READ_FAILED",
        message,
        issues: vec![],
    })?;

    // Resolution of the requested selector takes precedence over collection-wide
    // diagnostics, so a caller naming a document that does not exist is told
    // exactly that rather than being handed an unrelated fault.
    let Some(document) = inventory
        .documents
        .iter()
        .find(|document| document.selector == selector)
    else {
        let ambiguous = inventory.issues.iter().any(|issue| {
            issue.code == "STEERING_ARTIFACT_ID_DUPLICATE" && issue.message.ends_with(selector)
        });
        return Err(SteeringReadFailure {
            code: "STEERING_READ_INVALID",
            message: if ambiguous {
                format!("steering selector is ambiguous: {selector}")
            } else {
                format!("unknown steering selector: {selector}")
            },
            issues: vec![],
        });
    };

    // Decision 0098 is stricter here than the spec-local artifact read. Guidance
    // known to be incomplete cannot be safely acted on, so an unrelated fault
    // still fails the read instead of being reported alongside a valid document.
    if !inventory.issues.is_empty() {
        return Err(SteeringReadFailure {
            code: "STEERING_READ_FAILED",
            message: "steering inventory has diagnostics".to_owned(),
            issues: inventory.issues,
        });
    }

    let path = specbind_root.join(document.path.as_str());
    match fs::symlink_metadata(&path) {
        Ok(metadata) if !metadata.file_type().is_symlink() && metadata.is_file() => {}
        Ok(_) => {
            return Err(SteeringReadFailure {
                code: "STEERING_READ_TARGET_INVALID",
                message: format!("{} must be a regular non-symlink file", document.path),
                issues: vec![],
            });
        }
        Err(error) => {
            return Err(SteeringReadFailure {
                code: "STEERING_READ_FAILED",
                message: format!("{}: {error}", document.path),
                issues: vec![],
            });
        }
    }
    match fs::read(&path).map(String::from_utf8) {
        Ok(Ok(content)) => Ok(content),
        Ok(Err(_)) => Err(SteeringReadFailure {
            code: "STEERING_READ_NOT_UTF8",
            message: format!("{} must be UTF-8", document.path),
            issues: vec![],
        }),
        Err(error) => Err(SteeringReadFailure {
            code: "STEERING_READ_FAILED",
            message: format!("{}: {error}", document.path),
            issues: vec![],
        }),
    }
}

/// Recognizes one candidate file.
///
/// `Ok(None)` means the file is a valid OKF concept of another type, which
/// Decision 0098 keeps out of this read model without reporting it. The active
/// Roadmap lives in this directory and is excluded by exactly that rule.
fn classify(
    path: &Path,
    relative: &Utf8PathBuf,
) -> Result<(Option<SteeringDocument>, Vec<DiscoveryIssue>), DiscoveryIssue> {
    let content = match fs::read(path).map(String::from_utf8) {
        Ok(Ok(content)) => content,
        Ok(Err(_)) => {
            return Err(issue(
                "STEERING_NOT_UTF8",
                Some(relative.clone()),
                "steering document must be UTF-8",
            ));
        }
        Err(error) => {
            return Err(issue(
                "STEERING_READ_FAILED",
                Some(relative.clone()),
                error.to_string(),
            ));
        }
    };
    let (frontmatter, body) = split_frontmatter(&content).map_err(|message| {
        issue(
            "STEERING_FRONTMATTER_INVALID",
            Some(relative.clone()),
            message,
        )
    })?;
    let value: Value = serde_saphyr::from_str(frontmatter).map_err(|error| {
        issue(
            "STEERING_FRONTMATTER_YAML_INVALID",
            Some(relative.clone()),
            error.to_string(),
        )
    })?;
    let mapping = value.as_object().ok_or_else(|| {
        issue(
            "STEERING_FRONTMATTER_NOT_MAPPING",
            Some(relative.clone()),
            "frontmatter root must be a mapping",
        )
    })?;
    let artifact_type = mapping
        .get("type")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            issue(
                "STEERING_TYPE_INVALID",
                Some(relative.clone()),
                "frontmatter type must be a non-empty string",
            )
        })?;
    if artifact_type != TYPE_STEERING {
        return Ok((None, vec![]));
    }
    let artifact_id = mapping
        .get("artifact_id")
        .and_then(Value::as_str)
        .filter(|value| is_kebab_id(value))
        .ok_or_else(|| {
            issue(
                "STEERING_ARTIFACT_ID_INVALID",
                Some(relative.clone()),
                "steering frontmatter requires a lowercase kebab-case artifact_id",
            )
        })?;
    let issues = instruction::validate_live(body)
        .into_iter()
        .map(|fault| issue(fault.code, Some(relative.clone()), fault.message))
        .collect();
    Ok((
        Some(SteeringDocument {
            selector: artifact_id.to_owned(),
            artifact_type: artifact_type.to_owned(),
            path: relative.clone(),
        }),
        issues,
    ))
}

fn relative(root: &Path, path: &Path) -> Option<Utf8PathBuf> {
    let relative = path.strip_prefix(root).ok()?;
    Utf8PathBuf::from_path_buf(relative.to_path_buf())
        .ok()
        .map(|path| Utf8PathBuf::from(path.as_str().replace('\\', "/")))
}

fn issue(
    code: &'static str,
    path: Option<Utf8PathBuf>,
    message: impl Into<String>,
) -> DiscoveryIssue {
    DiscoveryIssue {
        code,
        path,
        message: message.into(),
    }
}
