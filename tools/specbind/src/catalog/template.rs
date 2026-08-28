//! Catalog discovery and raw reads for OKF artifact templates.
//!
//! A project-owned copy below `settings/templates/` overrides the official
//! default embedded in this binary, one selector at a time.

use std::{fs, path::Path};

use camino::Utf8PathBuf;
use include_dir::{Dir, include_dir};
use serde_json::{Map, Value};
use walkdir::WalkDir;

use crate::artifacts::{
    ArtifactKind, DiscoveryIssue, collection_id, is_kebab_id, recognized_kind, selector,
    split_frontmatter,
};
use crate::config::ProjectLanguage;
use crate::instruction;

/// The project tree that scaffolds one Spec's artifacts.
pub const SPEC_TEMPLATE_ROOT: &str = "settings/templates/specs";

static EMBEDDED_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/templates");

/// Where one resolved template came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateSource {
    /// A user-owned copy below the project `SpecBind` root.
    Project,
    /// The official default embedded in this binary.
    Embedded,
}

impl TemplateSource {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Embedded => "embedded",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub source: TemplateSource,
    pub selector: String,
    pub artifact_type: String,
    pub artifact_id: Option<String>,
    /// Location below the `SpecBind` root, or below the embedded asset tree.
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

/// Resolves every recognized Spec artifact template for one project.
///
/// Project-owned templates are user-owned under Decision 0008, so an absent tree
/// is normal: the embedded official defaults for the configured language answer
/// every selector the project does not override.
#[must_use]
pub fn discover_spec_templates(
    specbind_root: &Path,
    language: ProjectLanguage,
) -> TemplateInventory {
    let (mut templates, mut issues) = discover_project_templates(specbind_root);
    let (embedded_templates, mut embedded_issues) = discover_embedded_spec_templates(language);
    issues.append(&mut embedded_issues);
    for embedded in embedded_templates {
        if templates
            .iter()
            .any(|template| template.selector == embedded.selector)
        {
            continue;
        }
        templates.push(embedded);
    }
    issues.sort();
    issues.dedup();
    inventory(templates, issues)
}

/// Selectors whose structure a project may own, installed under Decision 0091.
///
/// Every other artifact type keeps an embedded scaffold only.
pub const INSTALLED_SELECTORS: [&str; 3] = ["requirements", "design/main", "design/ui"];

/// Resolves the embedded defaults that `specbind install` writes into the
/// project customization surface.
#[must_use]
pub fn installed_default_templates(language: ProjectLanguage) -> Vec<Template> {
    embedded_spec_templates(language)
        .into_iter()
        .filter(|template| INSTALLED_SELECTORS.contains(&template.selector.as_str()))
        .collect()
}

/// Resolves only the official defaults embedded in this binary.
#[must_use]
pub fn embedded_spec_templates(language: ProjectLanguage) -> Vec<Template> {
    discover_embedded_spec_templates(language).0
}

fn discover_embedded_spec_templates(
    language: ProjectLanguage,
) -> (Vec<Template>, Vec<DiscoveryIssue>) {
    let root = match language {
        ProjectLanguage::En => "en/specs",
        ProjectLanguage::Ja => "ja/specs",
    };
    let Some(directory) = EMBEDDED_TEMPLATES.get_dir(root) else {
        return (vec![], vec![]);
    };
    let mut templates = Vec::new();
    let mut issues = Vec::new();
    for file in directory.files() {
        let Some(path) = file.path().to_str() else {
            continue;
        };
        let template_path = Utf8PathBuf::from(path.replace('\\', "/"));
        let Some(output_path) = template_path
            .as_str()
            .strip_prefix(&format!("{root}/"))
            .map(Utf8PathBuf::from)
        else {
            continue;
        };
        let Some(content) = file.contents_utf8() else {
            continue;
        };
        match resolve_template(
            content,
            TemplateSource::Embedded,
            &template_path,
            output_path,
        ) {
            Ok((Some(template), mut found)) => {
                templates.push(template);
                issues.append(&mut found);
            }
            Ok((None, mut found)) | Err(mut found) => issues.append(&mut found),
        }
    }
    (templates, issues)
}

/// Reads one embedded default by selector without touching a project.
#[must_use]
pub fn read_embedded(language: ProjectLanguage, selector: &str) -> Option<String> {
    let template = embedded_spec_templates(language)
        .into_iter()
        .find(|template| template.selector == selector)?;
    EMBEDDED_TEMPLATES
        .get_file(template.template_path.as_str())
        .and_then(include_dir::File::contents_utf8)
        .map(ToOwned::to_owned)
}

fn discover_project_templates(specbind_root: &Path) -> (Vec<Template>, Vec<DiscoveryIssue>) {
    let root = specbind_root.join(SPEC_TEMPLATE_ROOT);
    if let Err(issues) = validate_template_root(&root, SPEC_TEMPLATE_ROOT) {
        return (vec![], issues);
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
        let (Some(template_path), Some(output_path)) = (
            relative(specbind_root, entry.path()),
            relative(&root, entry.path()),
        ) else {
            issues.push(issue(
                "TEMPLATE_PATH_NOT_UTF8",
                None,
                "template path must be UTF-8",
            ));
            continue;
        };
        let content = match fs::read(entry.path()).map(String::from_utf8) {
            Ok(Ok(content)) => content,
            Ok(Err(error)) => {
                issues.push(issue(
                    "TEMPLATE_NOT_UTF8",
                    Some(template_path),
                    format!("template must be UTF-8: {error}"),
                ));
                continue;
            }
            Err(error) => {
                issues.push(issue(
                    "TEMPLATE_READ_FAILED",
                    Some(template_path),
                    format!("cannot read template: {error}"),
                ));
                continue;
            }
        };
        match resolve_template(
            &content,
            TemplateSource::Project,
            &template_path,
            output_path,
        ) {
            Ok((Some(template), mut found)) => {
                templates.push(template);
                issues.append(&mut found);
            }
            Ok((None, mut found)) | Err(mut found) => issues.append(&mut found),
        }
    }

    let mut deduplicated: Vec<Template> = Vec::new();
    for template in templates {
        if deduplicated
            .iter()
            .any(|existing| existing.selector == template.selector)
        {
            issues.push(issue(
                "TEMPLATE_SELECTOR_DUPLICATE",
                Some(template.template_path.clone()),
                format!("template selector is duplicated: {}", template.selector),
            ));
            continue;
        }
        deduplicated.push(template);
    }
    (deduplicated, issues)
}

/// Accepts an absent tree and rejects any root that cannot be scanned safely.
fn validate_template_root(root: &Path, label: &str) -> Result<(), Vec<DiscoveryIssue>> {
    let path = Some(Utf8PathBuf::from(label));
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
    content: &str,
    source: TemplateSource,
    template_path: &Utf8PathBuf,
    output_path: Utf8PathBuf,
) -> Result<(Option<Template>, Vec<DiscoveryIssue>), Vec<DiscoveryIssue>> {
    let mut issues = validate_output_path(&output_path, template_path);
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
    let Some(kind) = recognized_kind(artifact_type) else {
        return Ok((None, issues));
    };
    let artifact_id = collection_id(kind, mapping).map(str::to_owned);

    issues.extend(
        instruction::validate_template_frontmatter(frontmatter)
            .into_iter()
            .map(|fault| issue(fault.code, Some(template_path.clone()), fault.message)),
    );
    issues.extend(
        instruction::validate_spec_template(body, artifact_id.as_deref())
            .into_iter()
            .map(|fault| issue(fault.code, Some(template_path.clone()), fault.message)),
    );
    issues.extend(validate_template_profile(kind, mapping, template_path));
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
            source,
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
    language: ProjectLanguage,
    requested: &str,
) -> Result<(String, TemplateInventory), TemplateInventory> {
    let inventory = discover_spec_templates(specbind_root, language);
    let Some(template) = inventory
        .templates
        .iter()
        .find(|template| template.selector == requested)
        .cloned()
    else {
        return Err(inventory);
    };
    match template.source {
        TemplateSource::Embedded => match EMBEDDED_TEMPLATES
            .get_file(template.template_path.as_str())
            .and_then(include_dir::File::contents_utf8)
        {
            Some(content) => Ok((content.to_owned(), inventory)),
            None => Err(with_issue(
                inventory,
                "TEMPLATE_READ_FAILED",
                &template.template_path.clone(),
                "embedded template is unavailable",
            )),
        },
        TemplateSource::Project => {
            let path = specbind_root.join(template.template_path.as_std_path());
            if !fs::symlink_metadata(&path)
                .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
            {
                let template_path = template.template_path.clone();
                return Err(with_issue(
                    inventory,
                    "TEMPLATE_TARGET_INVALID",
                    &template_path,
                    "resolved template is no longer a regular non-symlink file",
                ));
            }
            match fs::read(&path).map(String::from_utf8) {
                Ok(Ok(content)) => Ok((content, inventory)),
                Ok(Err(error)) => {
                    let template_path = template.template_path.clone();
                    Err(with_issue(
                        inventory,
                        "TEMPLATE_NOT_UTF8",
                        &template_path,
                        error.to_string(),
                    ))
                }
                Err(error) => {
                    let template_path = template.template_path.clone();
                    Err(with_issue(
                        inventory,
                        "TEMPLATE_READ_FAILED",
                        &template_path,
                        error.to_string(),
                    ))
                }
            }
        }
    }
}

/// The project tree that scaffolds steering documents.
pub const STEERING_TEMPLATE_ROOT: &str = "settings/templates/steering";

/// One resolved steering document template.
///
/// Decision 0117 gives this scope the identity exception no other scope needs.
/// A template that declares `artifact_id` materializes at `steering/<id>.md`;
/// one that omits it is a scaffold whose identity the authoring skill supplies,
/// so it has no fixed output path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringTemplate {
    pub source: TemplateSource,
    /// The template file stem, which is this scope's selector.
    pub selector: String,
    pub artifact_type: String,
    pub artifact_id: Option<String>,
    /// Location below the `SpecBind` root, or below the embedded asset tree.
    pub template_path: Utf8PathBuf,
    /// Materialization target below the `SpecBind` root, when identity is fixed.
    pub output_path: Option<Utf8PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SteeringTemplateInventory {
    pub templates: Vec<SteeringTemplate>,
    pub issues: Vec<DiscoveryIssue>,
}

/// Resolves every recognized steering template for one project.
///
/// Decision 0117 embeds this set without installing it, so an absent project
/// tree is the ordinary case and the embedded defaults answer every selector.
#[must_use]
pub fn discover_steering_templates(
    specbind_root: &Path,
    language: ProjectLanguage,
) -> SteeringTemplateInventory {
    let (mut templates, mut issues) = discover_project_steering_templates(specbind_root);
    for embedded in embedded_steering_templates(language) {
        if templates
            .iter()
            .any(|template| template.selector == embedded.selector)
        {
            continue;
        }
        templates.push(embedded);
    }
    templates.sort_by(|left, right| left.selector.cmp(&right.selector));
    issues.extend(duplicate_steering_ids(&templates));
    issues.sort();
    issues.dedup();
    SteeringTemplateInventory { templates, issues }
}

/// Resolves only the official steering defaults embedded in this binary.
#[must_use]
pub fn embedded_steering_templates(language: ProjectLanguage) -> Vec<SteeringTemplate> {
    let root = match language {
        ProjectLanguage::En => "en/steering",
        ProjectLanguage::Ja => "ja/steering",
    };
    let Some(directory) = EMBEDDED_TEMPLATES.get_dir(root) else {
        return vec![];
    };
    let mut templates = Vec::new();
    for file in directory.files() {
        let Some(path) = file.path().to_str() else {
            continue;
        };
        let template_path = Utf8PathBuf::from(path.replace('\\', "/"));
        let Some(selector) = steering_selector(&template_path) else {
            continue;
        };
        let Some(content) = file.contents_utf8() else {
            continue;
        };
        if let Ok((Some(template), _)) =
            resolve_steering_template(content, TemplateSource::Embedded, &template_path, selector)
        {
            templates.push(template);
        }
    }
    templates
}

/// Reads one official embedded Steering template by selector.
#[must_use]
pub fn read_embedded_steering(language: ProjectLanguage, selector: &str) -> Option<String> {
    let template = embedded_steering_templates(language)
        .into_iter()
        .find(|template| template.selector == selector)?;
    EMBEDDED_TEMPLATES
        .get_file(template.template_path.as_str())
        .and_then(include_dir::File::contents_utf8)
        .map(ToOwned::to_owned)
}

/// Discovers project-owned steering templates directly below the scope root.
///
/// Nesting is not scanned: the selector is a bare file stem, so a subdirectory
/// could only contribute a selector that already exists.
fn discover_project_steering_templates(
    specbind_root: &Path,
) -> (Vec<SteeringTemplate>, Vec<DiscoveryIssue>) {
    let root = specbind_root.join(STEERING_TEMPLATE_ROOT);
    if let Err(issues) = validate_template_root(&root, STEERING_TEMPLATE_ROOT) {
        return (vec![], issues);
    }
    let mut issues = Vec::new();
    let mut templates: Vec<SteeringTemplate> = Vec::new();
    for entry in WalkDir::new(&root)
        .follow_links(false)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                issues.push(issue(
                    "TEMPLATE_SCAN_FAILED",
                    Some(Utf8PathBuf::from(STEERING_TEMPLATE_ROOT)),
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
        let Some(selector) = steering_selector(&template_path) else {
            issues.push(issue(
                "TEMPLATE_SELECTOR_INVALID",
                Some(template_path),
                "steering template file name must be a non-empty stem",
            ));
            continue;
        };
        let content = match fs::read(entry.path()).map(String::from_utf8) {
            Ok(Ok(content)) => content,
            Ok(Err(error)) => {
                issues.push(issue(
                    "TEMPLATE_NOT_UTF8",
                    Some(template_path),
                    format!("template must be UTF-8: {error}"),
                ));
                continue;
            }
            Err(error) => {
                issues.push(issue(
                    "TEMPLATE_READ_FAILED",
                    Some(template_path),
                    format!("cannot read template: {error}"),
                ));
                continue;
            }
        };
        match resolve_steering_template(&content, TemplateSource::Project, &template_path, selector)
        {
            Ok((Some(template), mut found)) => {
                templates.push(template);
                issues.append(&mut found);
            }
            Ok((None, mut found)) | Err(mut found) => issues.append(&mut found),
        }
    }
    (templates, issues)
}

/// Derives the bare selector this scope uses from a template file name.
fn steering_selector(template_path: &Utf8PathBuf) -> Option<String> {
    template_path
        .file_stem()
        .filter(|stem| !stem.is_empty())
        .map(ToOwned::to_owned)
}

fn resolve_steering_template(
    content: &str,
    source: TemplateSource,
    template_path: &Utf8PathBuf,
    selector: String,
) -> Result<(Option<SteeringTemplate>, Vec<DiscoveryIssue>), Vec<DiscoveryIssue>> {
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
    if artifact_type != crate::steering::TYPE_STEERING {
        return Ok((None, vec![]));
    }
    let artifact_id = match mapping.get("artifact_id") {
        None => None,
        Some(value) => {
            let Some(id) = value.as_str().filter(|id| is_kebab_id(id)) else {
                return Err(vec![issue(
                    "TEMPLATE_STEERING_ID_INVALID",
                    Some(template_path.clone()),
                    "steering template artifact_id must be a lowercase kebab-case token",
                )]);
            };
            Some(id.to_owned())
        }
    };
    let output_path = artifact_id
        .as_ref()
        .map(|id| Utf8PathBuf::from(format!("steering/{id}.md")));
    let mut issues = instruction::validate_template_frontmatter(frontmatter)
        .into_iter()
        .map(|fault| issue(fault.code, Some(template_path.clone()), fault.message))
        .collect::<Vec<_>>();
    issues.extend(
        instruction::validate_template(body)
            .into_iter()
            .map(|fault| issue(fault.code, Some(template_path.clone()), fault.message)),
    );
    Ok((
        Some(SteeringTemplate {
            source,
            selector,
            artifact_type: artifact_type.to_owned(),
            artifact_id,
            template_path: template_path.clone(),
            output_path,
        }),
        issues,
    ))
}

/// Reports templates that would materialize onto one another.
///
/// Two templates declaring one `artifact_id` cannot both be used: Decision 0057
/// drops both documents from the steering inventory when the identity collides.
fn duplicate_steering_ids(templates: &[SteeringTemplate]) -> Vec<DiscoveryIssue> {
    let mut issues = Vec::new();
    for (index, template) in templates.iter().enumerate() {
        let Some(artifact_id) = &template.artifact_id else {
            continue;
        };
        if templates[..index]
            .iter()
            .any(|earlier| earlier.artifact_id.as_ref() == Some(artifact_id))
        {
            issues.push(issue(
                "TEMPLATE_STEERING_ID_DUPLICATE",
                Some(template.template_path.clone()),
                format!("steering template artifact_id is duplicated: {artifact_id}"),
            ));
        }
    }
    issues
}

/// Reads one steering template selector as raw UTF-8 Markdown.
///
/// # Errors
///
/// Returns the resolution or read diagnostics that prevent a trustworthy read.
pub fn read_steering_template(
    specbind_root: &Path,
    language: ProjectLanguage,
    requested: &str,
) -> Result<(String, SteeringTemplateInventory), SteeringTemplateInventory> {
    let inventory = discover_steering_templates(specbind_root, language);
    let Some(template) = inventory
        .templates
        .iter()
        .find(|template| template.selector == requested)
        .cloned()
    else {
        return Err(inventory);
    };
    match template.source {
        TemplateSource::Embedded => match EMBEDDED_TEMPLATES
            .get_file(template.template_path.as_str())
            .and_then(include_dir::File::contents_utf8)
        {
            Some(content) => Ok((content.to_owned(), inventory)),
            None => Err(with_steering_issue(
                inventory,
                "TEMPLATE_READ_FAILED",
                &template.template_path,
                "embedded template is unavailable",
            )),
        },
        TemplateSource::Project => {
            let path = specbind_root.join(template.template_path.as_std_path());
            if !fs::symlink_metadata(&path)
                .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
            {
                return Err(with_steering_issue(
                    inventory,
                    "TEMPLATE_TARGET_INVALID",
                    &template.template_path,
                    "resolved template is no longer a regular non-symlink file",
                ));
            }
            match fs::read(&path).map(String::from_utf8) {
                Ok(Ok(content)) => Ok((content, inventory)),
                Ok(Err(error)) => Err(with_steering_issue(
                    inventory,
                    "TEMPLATE_NOT_UTF8",
                    &template.template_path,
                    error.to_string(),
                )),
                Err(error) => Err(with_steering_issue(
                    inventory,
                    "TEMPLATE_READ_FAILED",
                    &template.template_path,
                    error.to_string(),
                )),
            }
        }
    }
}

fn with_steering_issue(
    inventory: SteeringTemplateInventory,
    code: &'static str,
    path: &Utf8PathBuf,
    message: impl Into<String>,
) -> SteeringTemplateInventory {
    let mut issues = inventory.issues;
    issues.push(issue(code, Some(path.clone()), message));
    SteeringTemplateInventory {
        templates: inventory.templates,
        issues,
    }
}

/// The project-owned singleton that scaffolds the active Roadmap body.
pub const MILESTONE_ROADMAP_TEMPLATE_PATH: &str = "settings/templates/roadmap.md";

const MILESTONE_ROADMAP_SELECTOR: &str = "roadmap";

/// One resolved milestone-wide template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneTemplate {
    pub source: TemplateSource,
    pub selector: String,
    pub artifact_type: String,
    pub template_path: Utf8PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MilestoneTemplateInventory {
    pub templates: Vec<MilestoneTemplate>,
    pub issues: Vec<DiscoveryIssue>,
}

/// Resolves the project-owned Roadmap body template ahead of the embedded default.
#[must_use]
pub fn discover_milestone_templates(
    specbind_root: &Path,
    language: ProjectLanguage,
) -> MilestoneTemplateInventory {
    let project_path = specbind_root.join(MILESTONE_ROADMAP_TEMPLATE_PATH);
    match fs::symlink_metadata(&project_path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            embedded_milestone_inventory(language)
        }
        Err(error) => MilestoneTemplateInventory {
            templates: vec![],
            issues: vec![issue(
                "TEMPLATE_READ_FAILED",
                Some(Utf8PathBuf::from(MILESTONE_ROADMAP_TEMPLATE_PATH)),
                error.to_string(),
            )],
        },
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            MilestoneTemplateInventory {
                templates: vec![],
                issues: vec![issue(
                    "TEMPLATE_TARGET_INVALID",
                    Some(Utf8PathBuf::from(MILESTONE_ROADMAP_TEMPLATE_PATH)),
                    "milestone template must be a regular non-symlink file",
                )],
            }
        }
        Ok(_) => match fs::read(&project_path).map(String::from_utf8) {
            Ok(Ok(content)) => milestone_inventory_from_content(
                &content,
                TemplateSource::Project,
                &Utf8PathBuf::from(MILESTONE_ROADMAP_TEMPLATE_PATH),
            ),
            Ok(Err(error)) => MilestoneTemplateInventory {
                templates: vec![],
                issues: vec![issue(
                    "TEMPLATE_NOT_UTF8",
                    Some(Utf8PathBuf::from(MILESTONE_ROADMAP_TEMPLATE_PATH)),
                    error.to_string(),
                )],
            },
            Err(error) => MilestoneTemplateInventory {
                templates: vec![],
                issues: vec![issue(
                    "TEMPLATE_READ_FAILED",
                    Some(Utf8PathBuf::from(MILESTONE_ROADMAP_TEMPLATE_PATH)),
                    error.to_string(),
                )],
            },
        },
    }
}

/// Reads the one milestone template as raw UTF-8 Markdown.
///
/// # Errors
///
/// Returns the inventory and its diagnostics when the requested selector does
/// not resolve or the selected template cannot be read safely.
pub fn read_milestone_template(
    specbind_root: &Path,
    language: ProjectLanguage,
    requested: &str,
) -> Result<(String, MilestoneTemplateInventory), MilestoneTemplateInventory> {
    let inventory = discover_milestone_templates(specbind_root, language);
    let Some(template) = inventory
        .templates
        .iter()
        .find(|template| template.selector == requested)
        .cloned()
    else {
        return Err(inventory);
    };
    match template.source {
        TemplateSource::Embedded => match embedded_milestone_content(language) {
            Some(content) => Ok((content.to_owned(), inventory)),
            None => Err(with_milestone_issue(
                inventory,
                "TEMPLATE_READ_FAILED",
                &template.template_path,
                "embedded template is unavailable",
            )),
        },
        TemplateSource::Project => {
            let path = specbind_root.join(template.template_path.as_std_path());
            if !fs::symlink_metadata(&path)
                .is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink())
            {
                return Err(with_milestone_issue(
                    inventory,
                    "TEMPLATE_TARGET_INVALID",
                    &template.template_path,
                    "resolved template is no longer a regular non-symlink file",
                ));
            }
            match fs::read(&path).map(String::from_utf8) {
                Ok(Ok(content)) => Ok((content, inventory)),
                Ok(Err(error)) => Err(with_milestone_issue(
                    inventory,
                    "TEMPLATE_NOT_UTF8",
                    &template.template_path,
                    error.to_string(),
                )),
                Err(error) => Err(with_milestone_issue(
                    inventory,
                    "TEMPLATE_READ_FAILED",
                    &template.template_path,
                    error.to_string(),
                )),
            }
        }
    }
}

/// Reads the official Roadmap template used by installation.
#[must_use]
pub fn read_embedded_milestone(language: ProjectLanguage) -> Option<String> {
    embedded_milestone_content(language).map(ToOwned::to_owned)
}

fn embedded_milestone_inventory(language: ProjectLanguage) -> MilestoneTemplateInventory {
    let path = milestone_embedded_path(language);
    let Some(content) = embedded_milestone_content(language) else {
        return MilestoneTemplateInventory {
            templates: vec![],
            issues: vec![issue(
                "TEMPLATE_READ_FAILED",
                Some(Utf8PathBuf::from(path)),
                "embedded template is unavailable",
            )],
        };
    };
    milestone_inventory_from_content(content, TemplateSource::Embedded, &Utf8PathBuf::from(path))
}

fn milestone_inventory_from_content(
    content: &str,
    source: TemplateSource,
    template_path: &Utf8PathBuf,
) -> MilestoneTemplateInventory {
    match resolve_milestone_template(content, source, template_path) {
        Ok(template) => MilestoneTemplateInventory {
            templates: vec![template],
            issues: vec![],
        },
        Err(issues) => MilestoneTemplateInventory {
            templates: vec![],
            issues,
        },
    }
}

fn resolve_milestone_template(
    content: &str,
    source: TemplateSource,
    template_path: &Utf8PathBuf,
) -> Result<MilestoneTemplate, Vec<DiscoveryIssue>> {
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
    let Some(mapping) = value.as_object() else {
        return Err(vec![issue(
            "TEMPLATE_FRONTMATTER_NOT_MAPPING",
            Some(template_path.clone()),
            "frontmatter root must be a mapping",
        )]);
    };
    let artifact_type = mapping.get("type").and_then(Value::as_str);
    if artifact_type != Some("SpecBind Roadmap") {
        return Err(vec![issue(
            "TEMPLATE_TYPE_INVALID",
            Some(template_path.clone()),
            "milestone template type must be SpecBind Roadmap",
        )]);
    }
    let mut issues = [
        "milestone_id",
        "baseline_revision",
        "target_release",
        "work_items",
    ]
    .into_iter()
    .filter(|field| mapping.contains_key(*field))
    .map(|field| {
        issue(
            "TEMPLATE_ROADMAP_MACHINE_FIELD_FORBIDDEN",
            Some(template_path.clone()),
            format!("milestone template must omit CLI-owned field {field}"),
        )
    })
    .collect::<Vec<_>>();
    issues.extend(
        instruction::validate_template_frontmatter(frontmatter)
            .into_iter()
            .map(|fault| issue(fault.code, Some(template_path.clone()), fault.message)),
    );
    issues.extend(
        instruction::validate_template(body)
            .into_iter()
            .map(|fault| issue(fault.code, Some(template_path.clone()), fault.message)),
    );
    if !issues.is_empty() {
        return Err(issues);
    }
    Ok(MilestoneTemplate {
        source,
        selector: MILESTONE_ROADMAP_SELECTOR.to_owned(),
        artifact_type: "SpecBind Roadmap".to_owned(),
        template_path: template_path.clone(),
    })
}

fn milestone_embedded_path(language: ProjectLanguage) -> &'static str {
    match language {
        ProjectLanguage::En => "en/milestone/roadmap.md",
        ProjectLanguage::Ja => "ja/milestone/roadmap.md",
    }
}

fn embedded_milestone_content(language: ProjectLanguage) -> Option<&'static str> {
    EMBEDDED_TEMPLATES
        .get_file(milestone_embedded_path(language))
        .and_then(include_dir::File::contents_utf8)
}

fn with_milestone_issue(
    inventory: MilestoneTemplateInventory,
    code: &'static str,
    path: &Utf8PathBuf,
    message: impl Into<String>,
) -> MilestoneTemplateInventory {
    let mut issues = inventory.issues;
    issues.push(issue(code, Some(path.clone()), message));
    MilestoneTemplateInventory {
        templates: inventory.templates,
        issues,
    }
}

fn with_issue(
    inventory: TemplateInventory,
    code: &'static str,
    path: &Utf8PathBuf,
    message: impl Into<String>,
) -> TemplateInventory {
    let mut issues = inventory.issues;
    issues.push(issue(code, Some(path.clone()), message));
    TemplateInventory {
        templates: inventory.templates,
        issues,
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
