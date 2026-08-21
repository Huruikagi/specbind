use std::fs;
use std::path::Path;

use specbind::{
    artifacts::{ArtifactKind, discover_spec, resolve_gate_inputs},
    fingerprint::Fingerprint,
};
use tempfile::TempDir;

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("create fixture parent");
    fs::write(path, content).expect("write fixture");
}

fn root() -> TempDir {
    tempfile::tempdir().expect("create temporary SpecBind root")
}

#[test]
fn discovers_recognized_artifacts_in_inventory_order() {
    let root = root();
    write(
        root.path(),
        "specs/example/requirements.md",
        "---\r\ntype: SpecBind Requirements\r\nheading_labels:\r\n  requirement: Requirement\r\n  acceptance_criteria: Acceptance Criteria\r\n---\r\n# Requirements\r\n\r\n### Requirement 1: Example\r\n\r\n#### Acceptance Criteria\r\n\r\n1. It works.\r\n",
    );
    write(
        root.path(),
        "specs/example/technical/storage.md",
        "---\ntype: SpecBind Design\nartifact_id: storage\nrequirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n",
    );
    write(
        root.path(),
        "specs/example/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n",
    );
    write(
        root.path(),
        "specs/example/contract.md",
        "---\ntype: SpecBind Contract\nproject_default: &default stable\nproject_copy: *default\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        "specs/example/brief.md",
        "---\ntype: SpecBind Brief\n---\nChange request\n",
    );
    write(
        root.path(),
        "specs/example/research.md",
        "---\ntype: SpecBind Research\n---\nFinding\n",
    );
    write(
        root.path(),
        "specs/example/notes.md",
        "---\ntype: SpecBind Implementation Notes\nartifact_id: runtime\n---\nRemember this.\n",
    );
    write(root.path(), "specs/example/log.md", "# History\n");
    write(root.path(), "specs/example/index.md", "# Index\n");
    write(
        root.path(),
        "specs/example/extension.md",
        "---\ntype: Project Extension\n---\nExtension\n",
    );

    let inventory = discover_spec(root.path(), "example");

    assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
    assert_eq!(
        inventory
            .artifacts
            .iter()
            .map(|artifact| artifact.selector.as_str())
            .collect::<Vec<_>>(),
        [
            "brief",
            "research",
            "requirements",
            "design/main",
            "design/storage",
            "contract",
            "implementation-notes/runtime",
        ]
    );
    assert_eq!(
        inventory.artifacts[4].path.as_str(),
        "specs/example/technical/storage.md"
    );
    assert_eq!(inventory.artifacts[4].kind, ArtifactKind::Design);
}

#[test]
fn reports_invalid_concepts_and_keeps_valid_partial_inventory() {
    let root = root();
    write(
        root.path(),
        "specs/example/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        "specs/example/missing-frontmatter.md",
        "# Not an OKF concept\n",
    );
    write(
        root.path(),
        "specs/example/bad-brief.md",
        "---\ntype: SpecBind Brief\nartifact_id: forbidden\n---\nBody\n",
    );
    write(
        root.path(),
        "specs/example/bad-design.md",
        "---\ntype: SpecBind Design\nartifact_id: Bad_ID\nrequirement_ids: []\n---\nBody\n",
    );
    write(
        root.path(),
        "specs/example/research.md",
        "---\ntype: SpecBind Research\n---\n",
    );
    write(
        root.path(),
        "specs/example/notes.md",
        "---\ntype: SpecBind Implementation Notes\nartifact_id: main\n---\n<!-- only a comment -->\n",
    );

    let inventory = discover_spec(root.path(), "example");
    let codes = inventory
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert_eq!(
        inventory
            .artifacts
            .iter()
            .map(|artifact| artifact.selector.as_str())
            .collect::<Vec<_>>(),
        ["brief", "research", "contract", "implementation-notes/main"]
    );
    assert!(codes.contains(&"ARTIFACT_FRONTMATTER_INVALID"));
    assert!(codes.contains(&"ARTIFACT_SINGLETON_ID_FORBIDDEN"));
    assert!(codes.contains(&"ARTIFACT_COLLECTION_ID_INVALID"));
    assert!(codes.contains(&"ARTIFACT_DESIGN_REQUIREMENT_IDS_INVALID"));
    assert!(codes.contains(&"ARTIFACT_RESEARCH_BODY_EMPTY"));
    assert!(codes.contains(&"ARTIFACT_IMPLEMENTATION_NOTES_BODY_EMPTY"));
}

#[test]
fn removes_ambiguous_duplicate_selectors_from_partial_inventory() {
    let root = root();
    for name in ["requirements-a.md", "requirements-b.md"] {
        write(
            root.path(),
            &format!("specs/example/{name}"),
            "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Example\n\n#### Acceptance Criteria\n\n1. It works.\n",
        );
    }
    write(
        root.path(),
        "specs/example/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n",
    );

    let inventory = discover_spec(root.path(), "example");

    assert_eq!(inventory.artifacts.len(), 1);
    assert_eq!(inventory.artifacts[0].selector, "design/main");
    assert_eq!(
        inventory
            .issues
            .iter()
            .filter(|issue| issue.code == "ARTIFACT_SELECTOR_DUPLICATE")
            .count(),
        2
    );
}

#[test]
fn rejects_create_instructions_but_accepts_durable_live_instructions() {
    let root = root();
    write(
        root.path(),
        "specs/example/brief.md",
        "---\ntype: SpecBind Brief\n---\n<!-- specbind:instruction create Replace this. -->\n<!-- specbind:instruction maintain Keep this. -->\n<!-- specbind:instruction consume Read this. -->\n",
    );

    let inventory = discover_spec(root.path(), "example");

    assert_eq!(inventory.artifacts[0].selector, "brief");
    assert!(
        inventory
            .issues
            .iter()
            .any(|issue| issue.code == "ARTIFACT_CREATE_INSTRUCTION_LEAK")
    );
    assert_eq!(inventory.issues.len(), 1);
}

#[test]
fn reports_requirements_body_issues_with_document_lines() {
    let root = root();
    write(
        root.path(),
        "specs/example/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Missing criteria\n",
    );

    let inventory = discover_spec(root.path(), "example");
    let issue = inventory
        .issues
        .iter()
        .find(|issue| issue.code == "REQUIREMENTS_ACCEPTANCE_HEADING_MISSING")
        .expect("body diagnostic");

    assert_eq!(
        issue.path.as_deref(),
        Some("specs/example/requirements.md".into())
    );
    assert!(issue.message.starts_with("line 9:"), "{}", issue.message);
}

#[test]
fn reports_design_marker_mismatches_with_document_lines() {
    let root = root();
    write(
        root.path(),
        "specs/example/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1', '2.1']\n---\n# Design\n\n_Requirements: 1.1, 3.1_\n",
    );

    let inventory = discover_spec(root.path(), "example");
    let codes = inventory
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert!(codes.contains(&"DESIGN_BODY_REQUIREMENT_ID_MISSING"));
    let body_only = inventory
        .issues
        .iter()
        .find(|issue| issue.code == "DESIGN_FRONTMATTER_REQUIREMENT_ID_MISSING")
        .expect("body-only Requirement ID");
    assert!(
        body_only.message.starts_with("line 8:"),
        "{}",
        body_only.message
    );
}

#[test]
fn reports_contract_body_issues_with_document_lines() {
    let root = root();
    write(
        root.path(),
        "specs/example/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n- `Bad_ID` — Description\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );

    let inventory = discover_spec(root.path(), "example");
    let issue = inventory
        .issues
        .iter()
        .find(|issue| issue.code == "CONTRACT_DESCRIBED_ENTRY_INVALID")
        .expect("Contract body diagnostic");

    assert_eq!(
        issue.path.as_deref(),
        Some("specs/example/contract.md".into())
    );
    assert!(issue.message.starts_with("line 8:"), "{}", issue.message);
}

#[test]
fn rejects_invalid_spec_ids_and_missing_directories() {
    let root = root();

    let invalid = discover_spec(root.path(), "Bad_ID");
    assert_eq!(invalid.issues[0].code, "ARTIFACT_SPEC_ID_INVALID");

    let missing = discover_spec(root.path(), "missing");
    assert_eq!(missing.issues[0].code, "ARTIFACT_SPEC_DIR_UNREADABLE");
}

#[test]
fn resolves_discovered_markdown_and_fixed_task_plan_inputs() {
    let root = root();
    let requirements = "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Example\n\n#### Acceptance Criteria\n\n1. It works.\n";
    let contract = "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n";
    let design = "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n";
    write(root.path(), "specs/example/requirements.md", requirements);
    write(root.path(), "specs/example/contract.md", contract);
    write(root.path(), "specs/example/design.md", design);
    write(
        root.path(),
        "specs/example/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Work\n      requirement_ids: ['1.1']\n",
    );

    let resolution = resolve_gate_inputs(root.path(), "example");

    assert!(resolution.inventory.issues.is_empty());
    assert_eq!(
        resolution.inputs.requirements,
        Some(Fingerprint::markdown(requirements.as_bytes()))
    );
    let current_design = resolution.inputs.design.expect("design input set");
    assert_eq!(
        current_design["contract"],
        Fingerprint::markdown(contract.as_bytes())
    );
    assert_eq!(
        current_design["design/main"],
        Fingerprint::markdown(design.as_bytes())
    );
    assert!(resolution.inputs.task_plan.is_some());
}

#[test]
fn reports_invalid_fixed_task_artifacts_without_guessing_a_fingerprint() {
    let root = root();
    write(
        root.path(),
        "specs/example/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '2'\n      kind: task\n      title: Work\n      requirement_ids: ['1.1']\n",
    );

    let resolution = resolve_gate_inputs(root.path(), "example");

    assert!(resolution.inputs.task_plan.is_none());
    assert!(
        resolution
            .inventory
            .issues
            .iter()
            .any(|issue| issue.code == "TASK_POSITIONAL_ID")
    );
}
