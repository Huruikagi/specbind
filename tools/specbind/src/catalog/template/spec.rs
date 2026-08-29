use super::{
    ArtifactKind, DiscoveryIssue, EMBEDDED_TEMPLATES, Map, Path, ProjectLanguage,
    SPEC_TEMPLATE_ROOT, Template, TemplateInventory, TemplateSource, Utf8PathBuf, Value, WalkDir,
    collection_id, fs, instruction, issue, recognized_kind, relative, selector, split_frontmatter,
    validate_template_root,
};

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
        let is_markdown = entry
            .path()
            .extension()
            .is_some_and(|extension| extension == "md");
        let is_contract = relative(&root, entry.path()).is_some_and(|path| path == "contract.yaml");
        if !file_type.is_file() || (!is_markdown && !is_contract) {
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
fn resolve_template(
    content: &str,
    source: TemplateSource,
    template_path: &Utf8PathBuf,
    output_path: Utf8PathBuf,
) -> Result<(Option<Template>, Vec<DiscoveryIssue>), Vec<DiscoveryIssue>> {
    let mut issues = validate_output_path(&output_path, template_path);
    if output_path == "contract.yaml" {
        issues.extend(validate_contract_template(content, template_path));
        return Ok((
            Some(Template {
                source,
                selector: "contract".to_owned(),
                artifact_type: "SpecBind Contract".to_owned(),
                artifact_id: None,
                template_path: template_path.clone(),
                output_path,
                kind: ArtifactKind::Contract,
            }),
            issues,
        ));
    }
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

fn validate_contract_template(content: &str, template_path: &Utf8PathBuf) -> Vec<DiscoveryIssue> {
    let mut issues = Vec::new();
    match crate::schema::runtime::load_contract(content) {
        Ok(wire) => {
            if let Err(error) = crate::domain::contract::Contract::try_from(wire) {
                for semantic in error.issues {
                    issues.push(issue(
                        semantic.code,
                        Some(template_path.clone()),
                        format!("{}: {}", semantic.path, semantic.message),
                    ));
                }
            }
        }
        Err(error) => issues.push(issue(
            "TEMPLATE_CONTRACT_INVALID",
            Some(template_path.clone()),
            error.to_string(),
        )),
    }
    issues
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
        ArtifactKind::Brief | ArtifactKind::Research | ArtifactKind::Requirements => {
            if mapping.contains_key("artifact_id") {
                issues.push(issue(
                    "TEMPLATE_SINGLETON_ID_FORBIDDEN",
                    Some(template_path.clone()),
                    "singleton template must omit artifact_id",
                ));
            }
        }
        ArtifactKind::Design | ArtifactKind::Contract | ArtifactKind::ImplementationNotes => {}
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
