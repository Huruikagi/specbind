//! Read-only discovery and raw reads for project-owned OKF artifact templates.

use std::{fs, path::Path};

use camino::Utf8PathBuf;
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::artifacts::{
    ArtifactKind, DiscoveryIssue, collection_id, recognized_kind, selector, split_frontmatter,
};

/// The template tree that scaffolds one Spec's artifacts.
pub const SPEC_TEMPLATE_ROOT: &str = "settings/templates/specs";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub selector: String,
    pub artifact_type: String,
    pub artifact_id: Option<String>,
    /// Location below the `SpecBind` root.
    pub template_path: Utf8PathBuf,
    /// Materialization target relative to the destination Spec directory.
    pub output_path: Utf8PathBuf,
    pub kind: ArtifactKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateInventory {
    pub templates: Vec<Template>,
    pub issues: Vec<DiscoveryIssue>,
}

/// Discovers every recognized Spec artifact template below a `SpecBind` root.
///
/// Templates are user-owned under Decision 0008, so an absent tree is a normal
/// empty inventory rather than a failure.
#[must_use]
pub fn discover_spec_templates(specbind_root: &Path) -> TemplateInventory {
    let root = specbind_root.join(SPEC_TEMPLATE_ROOT);
    if let Err(issues) = validate_template_root(&root) {
        return inventory(vec![], issues);
    }
    let mut issues = Vec::new();
    let mut templates = Vec::new();
    for entry in WalkDir::new(&root).follow_links(false).sort_by_file_name() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                issues.push(issue(
                    "TEMPLATE_SCAN_FAILED",
                    Some(Utf8PathBuf::from(SPEC_TEMPLATE_ROOT)),
                    error.to_string(),
                ));
                continue;
            }
        };
        let file_type = entry.file_type();
        if file_type.is_symlink() {
            issues.push(issue(
                "TEMPLATE_TARGET_INVALID",
                relative(specbind_root, entry.path()),
                "template entries must not be symbolic links",
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
        let Some(template_path) = relative(specbind_root, entry.path()) else {
            issues.push(issue(
                "TEMPLATE_PATH_NOT_UTF8",
                None,
                "template path must be UTF-8",
            ));
            continue;
        };
        let Some(output_path) = relative(&root, entry.path()) else {
            issues.push(issue(
                "TEMPLATE_PATH_NOT_UTF8",
                Some(template_path.clone()),
                "template output path must be UTF-8",
            ));
            continue;
        };
        match resolve_template(entry.path(), &template_path, output_path) {
            Ok((Some(template), mut found)) => {
                templates.push(template);
                issues.append(&mut found);
            }
            Ok((None, mut found)) | Err(mut found) => issues.append(&mut found),
        }
    }

    let mut deduplicated = Vec::new();
    for template in templates {
        let duplicate = deduplicated
            .iter()
            .any(|existing: &Template| existing.selector == template.selector);
        if duplicate {
            issues.push(issue(
                "TEMPLATE_SELECTOR_DUPLICATE",
                Some(template.template_path.clone()),
                format!("template selector is duplicated: {}", template.selector),
            ));
            continue;
        }
        deduplicated.push(template);
    }
    inventory(deduplicated, issues)
}

/// Accepts an absent tree and rejects any root that cannot be scanned safely.
fn validate_template_root(root: &Path) -> Result<(), Vec<DiscoveryIssue>> {
    let path = Some(Utf8PathBuf::from(SPEC_TEMPLATE_ROOT));
    match fs::symlink_metadata(root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(vec![]),
        Err(error) => Err(vec![issue(
            "TEMPLATE_ROOT_UNREADABLE",
            path,
            format!("cannot read the template root: {error}"),
        )]),
        Ok(metadata) if metadata.file_type().is_symlink() => Err(vec![issue(
            "TEMPLATE_ROOT_SYMLINK",
            path,
            "template root must not be a symbolic link",
        )]),
        Ok(metadata) if !metadata.is_dir() => Err(vec![issue(
            "TEMPLATE_ROOT_NOT_DIRECTORY",
            path,
            "template root must be a directory",
        )]),
        Ok(_) => Ok(()),
    }
}

fn resolve_template(
    native_path: &Path,
    template_path: &Utf8PathBuf,
    output_path: Utf8PathBuf,
) -> Result<(Option<Template>, Vec<DiscoveryIssue>), Vec<DiscoveryIssue>> {
    let mut issues = validate_output_path(&output_path, template_path);
    let bytes = fs::read(native_path).map_err(|error| {
        vec![issue(
            "TEMPLATE_READ_FAILED",
            Some(template_path.clone()),
            format!("cannot read template: {error}"),
        )]
    })?;
    let content = std::str::from_utf8(&bytes).map_err(|error| {
        vec![issue(
            "TEMPLATE_NOT_UTF8",
            Some(template_path.clone()),
            format!("template must be UTF-8: {error}"),
        )]
    })?;
    let (frontmatter, body) = split_frontmatter(content).map_err(|message| {
        vec![issue(
            "TEMPLATE_FRONTMATTER_INVALID",
            Some(template_path.clone()),
            message,
        )]
    })?;
    let value: Value = serde_saphyr::from_str(frontmatter).map_err(|error| {
        vec![issue(
            "TEMPLATE_FRONTMATTER_YAML_INVALID",
            Some(template_path.clone()),
            error.to_string(),
        )]
    })?;
    let mapping = value.as_object().ok_or_else(|| {
        vec![issue(
            "TEMPLATE_FRONTMATTER_NOT_MAPPING",
            Some(template_path.clone()),
            "frontmatter root must be a mapping",
        )]
    })?;
    let artifact_type = mapping
        .get("type")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            vec![issue(
                "TEMPLATE_TYPE_INVALID",
                Some(template_path.clone()),
                "frontmatter type must be a non-empty string",
            )]
        })?;
    // The body is free-form scaffold content and keeps its instruction comments;
    // only machine identity and the derived output path are validated here.
    let _ = body;
    let Some(kind) = recognized_kind(artifact_type) else {
        return Ok((None, issues));
    };

    issues.extend(validate_template_profile(kind, mapping, template_path));
    let artifact_id = collection_id(kind, mapping).map(str::to_owned);
    if matches!(
        kind,
        ArtifactKind::Design | ArtifactKind::ImplementationNotes
    ) && artifact_id.is_none()
    {
        issues.push(issue(
            "TEMPLATE_COLLECTION_ID_INVALID",
            Some(template_path.clone()),
            "collection template requires a literal stable artifact_id",
        ));
        return Ok((None, issues));
    }
    Ok((
        Some(Template {
            selector: selector(kind, artifact_id.as_deref()),
            artifact_type: artifact_type.to_owned(),
            artifact_id,
            template_path: template_path.clone(),
            output_path,
            kind,
        }),
        issues,
    ))
}

/// Enforces the Decision 0059 template-only profile rules.
///
/// A `SpecBind Design` template omits the live-only `requirement_ids` mapping,
/// which the authoring agent adds together with its body markers.
fn validate_template_profile(
    kind: ArtifactKind,
    mapping: &Map<String, Value>,
    template_path: &Utf8PathBuf,
) -> Vec<DiscoveryIssue> {
    let mut issues = Vec::new();
    match kind {
        ArtifactKind::Brief
        | ArtifactKind::Research
        | ArtifactKind::Requirements
        | ArtifactKind::Contract => {
            if mapping.contains_key("artifact_id") {
                issues.push(issue(
                    "TEMPLATE_SINGLETON_ID_FORBIDDEN",
                    Some(template_path.clone()),
                    "singleton template must omit artifact_id",
                ));
            }
        }
        ArtifactKind::Design | ArtifactKind::ImplementationNotes => {}
    }
    if kind == ArtifactKind::Design && mapping.contains_key("requirement_ids") {
        issues.push(issue(
            "TEMPLATE_DESIGN_REQUIREMENT_IDS_FORBIDDEN",
            Some(template_path.clone()),
            "design template must omit the live-only requirement_ids mapping",
        ));
    }
    issues
}

/// Rejects any output path that could escape the destination Spec directory.
fn validate_output_path(
    output_path: &Utf8PathBuf,
    template_path: &Utf8PathBuf,
) -> Vec<DiscoveryIssue> {
    let invalid = output_path.as_str().is_empty()
        || output_path.is_absolute()
        || output_path
            .components()
            .any(|component| !matches!(component, camino::Utf8Component::Normal(_)));
    if invalid {
        vec![issue(
            "TEMPLATE_OUTPUT_PATH_INVALID",
            Some(template_path.clone()),
            "template output path must stay inside the target spec directory",
        )]
    } else {
        vec![]
    }
}

/// Reads one template selector as raw UTF-8 Markdown, instruction comments included.
///
/// # Errors
///
/// Returns the resolution or read diagnostics that prevent a trustworthy read.
pub fn read_spec_template(
    specbind_root: &Path,
    requested: &str,
) -> Result<(String, TemplateInventory), TemplateInventory> {
    let inventory = discover_spec_templates(specbind_root);
    let Some(template) = inventory
        .templates
        .iter()
        .find(|template| template.selector == requested)
    else {
        return Err(inventory);
    };
    let path = specbind_root.join(template.template_path.as_std_path());
    if !fs::symlink_metadata(&path)
        .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
    {
        let mut issues = inventory.issues;
        issues.push(issue(
            "TEMPLATE_TARGET_INVALID",
            Some(template.template_path.clone()),
            "resolved template is no longer a regular non-symlink file",
        ));
        return Err(TemplateInventory {
            templates: inventory.templates,
            issues,
        });
    }
    match fs::read(&path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(content) => Ok((content, inventory)),
            Err(error) => {
                let mut issues = inventory.issues;
                issues.push(issue(
                    "TEMPLATE_NOT_UTF8",
                    Some(template.template_path.clone()),
                    error.to_string(),
                ));
                Err(TemplateInventory {
                    templates: inventory.templates,
                    issues,
                })
            }
        },
        Err(error) => {
            let mut issues = inventory.issues;
            issues.push(issue(
                "TEMPLATE_READ_FAILED",
                Some(template.template_path.clone()),
                error.to_string(),
            ));
            Err(TemplateInventory {
                templates: inventory.templates,
                issues,
            })
        }
    }
}

/// Renders one root-relative path with the `/` separator the CLI contract uses.
fn relative(base: &Path, path: &Path) -> Option<Utf8PathBuf> {
    let relative = path.strip_prefix(base).ok()?;
    let utf8 = Utf8PathBuf::from_path_buf(relative.to_path_buf()).ok()?;
    Some(Utf8PathBuf::from(utf8.as_str().replace('\\', "/")))
}

fn inventory(mut templates: Vec<Template>, mut issues: Vec<DiscoveryIssue>) -> TemplateInventory {
    templates.sort_by(|left, right| {
        (order(left.kind), left.selector.as_str())
            .cmp(&(order(right.kind), right.selector.as_str()))
    });
    issues.sort();
    issues.dedup();
    TemplateInventory { templates, issues }
}

fn order(kind: ArtifactKind) -> u8 {
    match kind {
        ArtifactKind::Brief => 0,
        ArtifactKind::Research => 1,
        ArtifactKind::Requirements => 2,
        ArtifactKind::Design => 3,
        ArtifactKind::Contract => 4,
        ArtifactKind::ImplementationNotes => 5,
    }
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
