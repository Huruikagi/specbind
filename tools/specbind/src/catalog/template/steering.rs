use super::{
    DiscoveryIssue, EMBEDDED_TEMPLATES, Path, ProjectLanguage, STEERING_TEMPLATE_ROOT,
    SteeringTemplate, SteeringTemplateInventory, TemplateSource, Utf8PathBuf, Value, WalkDir, fs,
    instruction, is_kebab_id, issue, relative, split_frontmatter, validate_template_root,
};

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
