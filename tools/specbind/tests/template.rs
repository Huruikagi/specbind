use std::fs;
use std::path::Path;

use specbind::{
    artifacts,
    config::ProjectLanguage,
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
    }
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
/// would: at its declared output path, with instruction comments removed.
fn materialize(specbind_root: &Path, language: ProjectLanguage) {
    for template in template::embedded_spec_templates(language) {
        let (content, _) =
            template::read_spec_template(specbind_root, language, &template.selector)
                .expect("embedded template content");
        let target = specbind_root
            .join("specs/checkout")
            .join(template.output_path.as_std_path());
        fs::create_dir_all(target.parent().expect("template parent"))
            .expect("create spec directory");
        fs::write(target, strip_instructions(&content)).expect("write materialized artifact");
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
        if rest[start..end].contains("specbind:instruction") {
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
