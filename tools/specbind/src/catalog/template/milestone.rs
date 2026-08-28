use super::{
    DiscoveryIssue, EMBEDDED_TEMPLATES, MILESTONE_ROADMAP_SELECTOR,
    MILESTONE_ROADMAP_TEMPLATE_PATH, MilestoneTemplate, MilestoneTemplateInventory, Path,
    ProjectLanguage, TemplateSource, Utf8PathBuf, Value, fs, instruction, issue, split_frontmatter,
};

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
