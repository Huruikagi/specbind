use std::fs;
use std::path::Path;

use specbind::{
    artifacts,
    config::ProjectLanguage,
    instruction,
    template::{self, TemplateSource},
};
use tempfile::TempDir;

const EXPECTED_SELECTORS: [&str; 6] = [
    "brief",
    "research",
    "requirements",
    "design/main",
    "contract",
    "implementation-notes/main",
];

#[test]
fn embeds_one_official_template_per_artifact_type_in_every_language() {
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        let templates = template::embedded_spec_templates(language);
        let selectors = templates
            .iter()
            .map(|template| template.selector.as_str())
            .collect::<Vec<_>>();
        for expected in EXPECTED_SELECTORS {
            assert!(
                selectors.contains(&expected),
                "{language:?} is missing template {expected}: {selectors:?}"
            );
        }
        assert_eq!(
            templates.len(),
            EXPECTED_SELECTORS.len(),
            "{language:?} embeds unexpected templates: {selectors:?}"
        );
        assert!(
            templates
                .iter()
                .all(|template| template.source == TemplateSource::Embedded)
        );
        for selector in EXPECTED_SELECTORS {
            let resolved = templates
                .iter()
                .find(|template| template.selector == selector)
                .expect("resolved embedded template");
            let content = template::read_embedded(language, selector).expect("embedded template");
            assert!(
                instruction::validate_spec_template(&content, resolved.artifact_id.as_deref())
                    .is_empty(),
                "{language:?} template {selector} has invalid scoped instructions or bindings"
            );
        }
    }
}

#[test]
fn embeds_one_milestone_roadmap_template_in_every_language() {
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        let root = tempfile::tempdir().expect("temporary SpecBind root");
        let inventory = template::discover_milestone_templates(root.path(), language);
        assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
        assert_eq!(inventory.templates.len(), 1);
        let roadmap = &inventory.templates[0];
        assert_eq!(roadmap.selector, "roadmap");
        assert_eq!(roadmap.artifact_type, "SpecBind Roadmap");
        assert_eq!(roadmap.source, TemplateSource::Embedded);
        let (content, _) = template::read_milestone_template(root.path(), language, "roadmap")
            .expect("read embedded milestone template");
        assert!(content.contains("type: SpecBind Roadmap"));
        assert!(instruction::validate_template(&content).is_empty());
    }
}

#[test]
fn project_milestone_template_overrides_the_embedded_default() {
    let root = tempfile::tempdir().expect("temporary SpecBind root");
    let target = root.path().join("settings/templates/roadmap.md");
    fs::create_dir_all(target.parent().expect("template parent")).expect("create parent");
    let content = "---\ntype: SpecBind Roadmap\n---\n# Custom roadmap\n";
    fs::write(&target, content).expect("write project milestone template");

    let (read, inventory) =
        template::read_milestone_template(root.path(), ProjectLanguage::En, "roadmap")
            .expect("read project milestone template");
    assert_eq!(read, content);
    assert!(inventory.issues.is_empty());
    assert_eq!(inventory.templates[0].source, TemplateSource::Project);
    assert_eq!(
        inventory.templates[0].template_path.as_str(),
        "settings/templates/roadmap.md"
    );
}

#[test]
fn non_spec_templates_accept_agent_bound_variables() {
    let root = tempfile::tempdir().expect("temporary SpecBind root");
    let milestone = root.path().join("settings/templates/roadmap.md");
    fs::create_dir_all(milestone.parent().expect("milestone parent")).expect("create parent");
    fs::write(
        milestone,
        concat!(
            "---\ntype: SpecBind Roadmap\n---\n",
            "<!-- specbind:instruction create bind=対象読者 Ask who will read it. -->\n",
            "# {{対象読者}}向けRoadmap\n",
        ),
    )
    .expect("write milestone template");

    let steering = root.path().join("settings/templates/steering/audience.md");
    fs::create_dir_all(steering.parent().expect("steering parent")).expect("create parent");
    fs::write(
        steering,
        concat!(
            "---\ntype: SpecBind Steering\nartifact_id: audience\n---\n",
            "<!-- specbind:instruction create bind=対象読者 Ask who will read it. -->\n",
            "# {{対象読者}}向けGuidance\n",
        ),
    )
    .expect("write steering template");

    let milestone_inventory =
        template::discover_milestone_templates(root.path(), ProjectLanguage::En);
    assert!(
        milestone_inventory.issues.is_empty(),
        "{:?}",
        milestone_inventory.issues
    );
    let steering_inventory =
        template::discover_steering_templates(root.path(), ProjectLanguage::En);
    assert!(
        steering_inventory.issues.is_empty(),
        "{:?}",
        steering_inventory.issues
    );
}

#[test]
fn milestone_template_rejects_cli_owned_frontmatter() {
    let root = tempfile::tempdir().expect("temporary SpecBind root");
    let target = root.path().join("settings/templates/roadmap.md");
    fs::create_dir_all(target.parent().expect("template parent")).expect("create parent");
    fs::write(
        &target,
        "---\ntype: SpecBind Roadmap\nwork_items: {}\n---\n# Roadmap\n",
    )
    .expect("write invalid project milestone template");

    let inventory = template::discover_milestone_templates(root.path(), ProjectLanguage::En);
    assert!(inventory.templates.is_empty());
    assert_eq!(
        inventory.issues[0].code,
        "TEMPLATE_ROADMAP_MACHINE_FIELD_FORBIDDEN"
    );
}

#[test]
fn materialized_embedded_templates_are_recognized_live_artifacts() {
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        let root = tempfile::tempdir().expect("temporary SpecBind root");
        materialize(root.path(), language);

        let inventory = artifacts::discover_spec(root.path(), "checkout");
        let selectors = inventory
            .artifacts
            .iter()
            .map(|artifact| artifact.selector.as_str())
            .collect::<Vec<_>>();
        for expected in EXPECTED_SELECTORS {
            assert!(
                selectors.contains(&expected),
                "{language:?} materialization is missing {expected}: {selectors:?}"
            );
        }
        // The Design template deliberately omits the live-only requirement_ids
        // mapping under Decision 0059, so exactly that diagnostic is expected
        // until the authoring agent adds the mapping and its body markers.
        let codes = inventory
            .issues
            .iter()
            .map(|issue| issue.code)
            .collect::<Vec<_>>();
        assert_eq!(
            codes,
            ["ARTIFACT_DESIGN_REQUIREMENT_IDS_INVALID"],
            "{language:?} materialization has unexpected diagnostics: {:?}",
            inventory.issues
        );
    }
}

#[test]
fn materialized_requirements_and_design_satisfy_traceability() {
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        let root = tempfile::tempdir().expect("temporary SpecBind root");
        materialize(root.path(), language);
        complete_design(root.path());
        fs::write(
            root.path().join("specs/checkout/spec.yaml"),
            "schema_version: 1\nactive_change: null\n",
        )
        .expect("write spec metadata");

        let resolution = artifacts::resolve_traceability(root.path(), "checkout");
        assert!(
            resolution.inventory.issues.is_empty(),
            "{language:?} has inventory diagnostics: {:?}",
            resolution.inventory.issues
        );
        let report = resolution.report.expect("traceability report");
        assert_eq!(
            report.requirement_ids,
            ["1.1"],
            "{language:?} requirements template must yield Requirement ID 1.1"
        );
        assert!(
            report.issues.is_empty(),
            "{language:?} has traceability diagnostics: {:?}",
            report.issues
        );
    }
}

/// Writes every embedded template into one Spec exactly as an authoring agent
/// would: at its declared output path, with only create instructions removed.
fn materialize(specbind_root: &Path, language: ProjectLanguage) {
    for template in template::embedded_spec_templates(language) {
        let content = template::read_spec_template(specbind_root, language, &template.selector)
            .expect("read embedded template content")
            .0;
        let target = specbind_root
            .join("specs/checkout")
            .join(template.output_path.as_std_path());
        fs::create_dir_all(target.parent().expect("template parent"))
            .expect("create spec directory");
        let scaffold = strip_instructions(&resolve_default_bindings(
            &content,
            "checkout",
            template.artifact_id.as_deref(),
        ));
        let authored = complete_required_content(&template.selector, language, &scaffold);
        fs::write(target, authored).expect("write materialized artifact");
    }
}

fn complete_required_content(selector: &str, language: ProjectLanguage, scaffold: &str) -> String {
    let addition = match (selector, language) {
        ("brief", ProjectLanguage::En) => "\nRequested change.\n",
        ("brief", ProjectLanguage::Ja) => "\n依頼された変更。\n",
        ("research", ProjectLanguage::En) => "\nDurable finding.\n",
        ("research", ProjectLanguage::Ja) => "\n保持する調査結果。\n",
        ("requirements", ProjectLanguage::En) => concat!(
            "\n### Requirement 1: Checkout\n\n",
            "#### Acceptance Criteria\n\n",
            "1. Checkout works.\n",
        ),
        ("requirements", ProjectLanguage::Ja) => concat!(
            "\n### 要件 1: チェックアウト\n\n",
            "#### 受け入れ基準\n\n",
            "1. チェックアウトが動作する。\n",
        ),
        ("implementation-notes/main", ProjectLanguage::En) => "\nRemember this.\n",
        ("implementation-notes/main", ProjectLanguage::Ja) => "\nこの知識を保持する。\n",
        _ => "",
    };
    format!("{}{}", scaffold.trim_end(), addition)
}

#[test]
fn unfilled_default_scaffolds_fail_closed() {
    let cases = [
        ("brief", "ARTIFACT_BRIEF_BODY_EMPTY"),
        ("research", "ARTIFACT_RESEARCH_BODY_EMPTY"),
        ("requirements", "REQUIREMENTS_GROUP_MISSING"),
        (
            "implementation-notes/main",
            "ARTIFACT_IMPLEMENTATION_NOTES_BODY_EMPTY",
        ),
    ];
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        for (selector, expected) in cases {
            let root = tempfile::tempdir().expect("temporary SpecBind root");
            let template = template::embedded_spec_templates(language)
                .into_iter()
                .find(|template| template.selector == selector)
                .expect("embedded template");
            let content = template::read_spec_template(root.path(), language, selector)
                .expect("read template")
                .0;
            let rendered =
                resolve_default_bindings(&content, "checkout", template.artifact_id.as_deref());
            let target = root
                .path()
                .join("specs/checkout")
                .join(template.output_path.as_std_path());
            fs::create_dir_all(target.parent().expect("template parent"))
                .expect("create spec directory");
            fs::write(target, strip_instructions(&rendered)).expect("write unfilled scaffold");

            let inventory = artifacts::discover_spec(root.path(), "checkout");
            assert!(
                inventory.issues.iter().any(|issue| issue.code == expected),
                "{language:?} {selector} should report {expected}: {:?}",
                inventory.issues
            );
        }
    }
}

#[test]
fn reads_agent_bound_variables_and_preserves_create_guidance() {
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        for selector in [
            "brief",
            "research",
            "requirements",
            "design/main",
            "implementation-notes/main",
        ] {
            let root = tempfile::tempdir().expect("temporary SpecBind root");
            let raw = template::read_spec_template(root.path(), language, selector)
                .expect("read template")
                .0;
            assert!(raw.contains("{{spec}}"));
            assert!(raw.contains("specbind:instruction create bind=spec"));
            if matches!(selector, "design/main" | "implementation-notes/main") {
                assert!(raw.contains("{{artifact_id}}"));
                assert!(raw.contains("specbind:instruction create bind=artifact_id"));
            }
        }
    }
}

#[test]
fn accepts_arbitrary_bound_variables_and_rejects_binding_faults() {
    let cases = [
        (
            "---\ntype: SpecBind Brief\n---\n# `{{spec}}` Brief\n",
            "TEMPLATE_VARIABLE_BINDING_MISSING",
        ),
        (
            concat!(
                "---\ntype: SpecBind Brief\nlabel: '{{spec}}'\n---\n",
                "<!-- specbind:instruction create bind=spec Use it. -->\n",
                "# Brief\n",
            ),
            "TEMPLATE_VARIABLE_FRONTMATTER_FORBIDDEN",
        ),
    ];
    for (content, expected) in cases {
        let root = tempfile::tempdir().expect("temporary SpecBind root");
        let target = root.path().join("settings/templates/specs/brief.md");
        fs::create_dir_all(target.parent().expect("template parent")).expect("create parent");
        fs::write(target, content).expect("write template");
        let inventory = template::discover_spec_templates(root.path(), ProjectLanguage::En);
        assert!(
            inventory.issues.iter().any(|issue| issue.code == expected),
            "expected {expected}: {:?}",
            inventory.issues
        );
    }

    let root = tempfile::tempdir().expect("temporary SpecBind root");
    let target = root.path().join("settings/templates/specs/brief.md");
    fs::create_dir_all(target.parent().expect("template parent")).expect("create parent");
    fs::write(
        target,
        concat!(
            "---\ntype: SpecBind Brief\n---\n",
            "<!-- specbind:instruction create bind=今日の天気 Fetch Tokyo weather. -->\n",
            "{{今日の天気}}の日に作成。{{今日の天気}}に合わせる。\n",
        ),
    )
    .expect("write arbitrary variable template");
    let inventory = template::discover_spec_templates(root.path(), ProjectLanguage::En);
    assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
}

fn resolve_default_bindings(content: &str, spec: &str, artifact_id: Option<&str>) -> String {
    let content = content.replace("{{spec}}", spec);
    match artifact_id {
        Some(artifact_id) => content.replace("{{artifact_id}}", artifact_id),
        None => content,
    }
}

fn complete_design(specbind_root: &Path) {
    let path = specbind_root.join("specs/checkout/design.md");
    let content = fs::read_to_string(&path).expect("materialized design");
    let content = content.replacen(
        "artifact_id: main\n",
        "artifact_id: main\nrequirement_ids: ['1.1']\n",
        1,
    );
    fs::write(
        path,
        format!("{}\n_Requirements: 1.1_\n", content.trim_end()),
    )
    .expect("write completed design");
}

fn strip_instructions(content: &str) -> String {
    let mut output = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find("<!--") {
        let end = rest[start..]
            .find("-->")
            .map_or(rest.len(), |offset| start + offset + "-->".len());
        if rest[start..end].contains("specbind:instruction create") {
            output.push_str(&rest[..start]);
            rest = rest[end..].strip_prefix('\n').unwrap_or(&rest[end..]);
        } else {
            output.push_str(&rest[..end]);
            rest = &rest[end..];
        }
    }
    output.push_str(rest);
    output
}

#[test]
fn project_owned_templates_override_the_embedded_default() {
    let root: TempDir = tempfile::tempdir().expect("temporary SpecBind root");
    let override_path = root.path().join("settings/templates/specs/contract.md");
    fs::create_dir_all(override_path.parent().expect("template parent"))
        .expect("create template directory");
    fs::write(
        &override_path,
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    )
    .expect("write project template");

    let inventory = template::discover_spec_templates(root.path(), ProjectLanguage::En);
    assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
    let contract = inventory
        .templates
        .iter()
        .find(|template| template.selector == "contract")
        .expect("contract template");
    assert_eq!(contract.source, TemplateSource::Project);
    assert_eq!(
        contract.template_path.as_str(),
        "settings/templates/specs/contract.md"
    );
    assert_eq!(
        inventory.templates.len(),
        EXPECTED_SELECTORS.len(),
        "an override must not add a selector"
    );
}

#[test]
fn rejects_unscoped_and_unknown_template_instructions() {
    let root: TempDir = tempfile::tempdir().expect("temporary SpecBind root");
    let template_root = root.path().join("settings/templates/specs");
    fs::create_dir_all(&template_root).expect("create template directory");
    fs::write(
        template_root.join("brief.md"),
        "---\ntype: SpecBind Brief\n---\n<!-- specbind:instruction -->\n",
    )
    .expect("write unscoped template");
    fs::write(
        template_root.join("research.md"),
        "---\ntype: SpecBind Research\n---\n<!-- specbind:instruction revise Unknown. -->\n",
    )
    .expect("write unknown-scope template");

    let inventory = template::discover_spec_templates(root.path(), ProjectLanguage::En);
    assert!(
        inventory
            .issues
            .iter()
            .any(|issue| issue.code == "INSTRUCTION_SCOPE_MISSING")
    );
    assert_eq!(
        inventory
            .issues
            .iter()
            .filter(|issue| matches!(
                issue.code,
                "INSTRUCTION_SCOPE_MISSING" | "INSTRUCTION_SCOPE_INVALID"
            ))
            .count(),
        2
    );
}

const EXPECTED_STEERING_SELECTORS: [&str; 4] = ["document", "product", "structure", "tech"];

#[test]
fn embeds_the_steering_scaffold_set_in_every_language() {
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        let root = tempfile::tempdir().expect("temporary SpecBind root");
        let inventory = template::discover_steering_templates(root.path(), language);
        assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
        let selectors = inventory
            .templates
            .iter()
            .map(|template| template.selector.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            selectors, EXPECTED_STEERING_SELECTORS,
            "{language:?} embeds unexpected steering templates"
        );

        for template in &inventory.templates {
            assert_eq!(template.source, TemplateSource::Embedded);
            assert_eq!(template.artifact_type, "SpecBind Steering");
        }
        // The three bootstrap defaults carry literal identity; the scaffold
        // whose subject the author chooses cannot, under Decision 0117.
        for selector in ["product", "tech", "structure"] {
            let template = steering_template(&inventory, selector);
            assert_eq!(template.artifact_id.as_deref(), Some(selector));
            assert_eq!(
                template
                    .output_path
                    .as_ref()
                    .expect("fixed output path")
                    .as_str(),
                format!("steering/{selector}.md")
            );
        }
        let document = steering_template(&inventory, "document");
        assert_eq!(document.artifact_id, None);
        assert_eq!(document.output_path, None);
    }
}

#[test]
fn materialized_steering_templates_are_recognized_documents() {
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        let root = tempfile::tempdir().expect("temporary SpecBind root");
        for selector in ["product", "tech", "structure"] {
            let (content, _) = template::read_steering_template(root.path(), language, selector)
                .expect("embedded steering template");
            write_steering(
                root.path(),
                &format!("{selector}.md"),
                &strip_instructions(&content),
            );
        }
        // The authoring skill supplies the identity the scaffold omits.
        let (scaffold, _) = template::read_steering_template(root.path(), language, "document")
            .expect("embedded steering scaffold");
        let authored = strip_instructions(&scaffold).replacen(
            "type: SpecBind Steering\n",
            "type: SpecBind Steering\nartifact_id: api-standards\n",
            1,
        );
        write_steering(root.path(), "api-standards.md", &authored);

        let inventory = specbind::steering::discover(root.path()).expect("steering inventory");
        assert!(
            inventory.issues.is_empty(),
            "{language:?} materialization has diagnostics: {:?}",
            inventory.issues
        );
        let selectors = inventory
            .documents
            .iter()
            .map(|document| document.selector.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            selectors,
            ["api-standards", "product", "structure", "tech"],
            "{language:?} materialization is not the expected collection"
        );
    }
}

#[test]
fn project_owned_steering_templates_override_the_embedded_default() {
    let root: TempDir = tempfile::tempdir().expect("temporary SpecBind root");
    write_steering_template(
        root.path(),
        "product.md",
        "---\ntype: SpecBind Steering\nartifact_id: product-overview\n---\n\n# Product\n",
    );

    let inventory = template::discover_steering_templates(root.path(), ProjectLanguage::En);
    assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
    let product = steering_template(&inventory, "product");
    assert_eq!(product.source, TemplateSource::Project);
    assert_eq!(product.artifact_id.as_deref(), Some("product-overview"));
    assert_eq!(
        product
            .output_path
            .as_ref()
            .expect("fixed output path")
            .as_str(),
        "steering/product-overview.md",
        "the declared identity locates the output, not the file stem"
    );
    assert_eq!(
        inventory.templates.len(),
        EXPECTED_STEERING_SELECTORS.len(),
        "an override must not add a selector"
    );
}

#[test]
fn reports_steering_templates_that_would_materialize_onto_one_another() {
    let root: TempDir = tempfile::tempdir().expect("temporary SpecBind root");
    write_steering_template(
        root.path(),
        "overview.md",
        "---\ntype: SpecBind Steering\nartifact_id: product\n---\n\n# Overview\n",
    );

    let inventory = template::discover_steering_templates(root.path(), ProjectLanguage::En);
    let codes = inventory
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert_eq!(
        codes,
        ["TEMPLATE_STEERING_ID_DUPLICATE"],
        "{:?}",
        inventory.issues
    );
}

#[test]
fn rejects_a_steering_template_identity_that_discovery_could_not_use() {
    let root: TempDir = tempfile::tempdir().expect("temporary SpecBind root");
    write_steering_template(
        root.path(),
        "product.md",
        "---\ntype: SpecBind Steering\nartifact_id: Product Overview\n---\n\n# Product\n",
    );

    let inventory = template::discover_steering_templates(root.path(), ProjectLanguage::En);
    let codes = inventory
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert_eq!(
        codes,
        ["TEMPLATE_STEERING_ID_INVALID"],
        "{:?}",
        inventory.issues
    );
    assert_eq!(
        steering_template(&inventory, "product").source,
        TemplateSource::Embedded,
        "a rejected override must not shadow the embedded default"
    );
}

#[test]
fn skips_a_project_file_of_another_type_without_reporting_it() {
    let root: TempDir = tempfile::tempdir().expect("temporary SpecBind root");
    write_steering_template(
        root.path(),
        "notes.md",
        "---\ntype: SpecBind Rule\n---\n\n# Notes\n",
    );

    let inventory = template::discover_steering_templates(root.path(), ProjectLanguage::En);
    assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
    let selectors = inventory
        .templates
        .iter()
        .map(|template| template.selector.as_str())
        .collect::<Vec<_>>();
    assert_eq!(selectors, EXPECTED_STEERING_SELECTORS);
}

fn steering_template<'a>(
    inventory: &'a template::SteeringTemplateInventory,
    selector: &str,
) -> &'a template::SteeringTemplate {
    inventory
        .templates
        .iter()
        .find(|template| template.selector == selector)
        .unwrap_or_else(|| panic!("steering template {selector}"))
}

fn write_steering(specbind_root: &Path, name: &str, content: &str) {
    let target = specbind_root.join("steering").join(name);
    fs::create_dir_all(target.parent().expect("steering parent")).expect("create steering root");
    fs::write(target, content).expect("write steering document");
}

fn write_steering_template(specbind_root: &Path, name: &str, content: &str) {
    let target = specbind_root.join("settings/templates/steering").join(name);
    fs::create_dir_all(target.parent().expect("template parent"))
        .expect("create template directory");
    fs::write(target, content).expect("write project template");
}
