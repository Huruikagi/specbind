use std::fs;
use std::path::Path;

use specbind::{cross_spec_review, fingerprint::Fingerprint};
use tempfile::TempDir;

const MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("create fixture parent");
    fs::write(path, content).expect("write fixture");
}

fn fixture() -> TempDir {
    let root = tempfile::tempdir().expect("temporary SpecBind root");
    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  new_specs:\n    - spec: provider\n      summary: Add provider\n  spec_updates:\n    - spec: consumer\n      summary: Update consumer\n      depends_on:\n        - spec: provider\n---\n# Roadmap\n"
        ),
    );
    write(
        root.path(),
        "specs/provider/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        "specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `value` → `provider/exports/value`\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        "specs/consumer/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Consume value\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );
    write(
        root.path(),
        "specs/consumer/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Design\n\n_Requirements: 1.1_\n",
    );
    root
}

#[test]
fn resolves_authoritative_contract_first_input_revisions() {
    let root = fixture();
    let candidate = r##"{"schemaVersion":1,"assessment":"# Assessment\n\nCompatible.","deepInputs":["specs/consumer#requirements","specs/consumer#design/main"]}"##;

    let resolution = cross_spec_review::resolve_inputs(root.path(), candidate)
        .expect("valid review input resolution");

    assert_eq!(resolution.roadmap.milestone_id, MILESTONE);
    assert_eq!(resolution.graph.report.dependencies.len(), 1);
    assert_eq!(
        resolution
            .input_revisions
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        [
            "specs/consumer#contract",
            "specs/consumer#design/main",
            "specs/consumer#requirements",
            "specs/provider#contract",
            "steering/roadmap.md#cross-spec-scope",
        ]
    );
    let requirements =
        fs::read(root.path().join("specs/consumer/requirements.md")).expect("requirements bytes");
    assert_eq!(
        resolution.input_revisions["specs/consumer#requirements"],
        Fingerprint::markdown(&requirements)
    );
}

#[test]
fn rejects_invalid_and_duplicate_deep_inputs() {
    let root = fixture();
    let candidate = r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":["specs/consumer#contract","specs/consumer#requirements","specs/consumer#requirements"]}"#;

    let error =
        cross_spec_review::resolve_inputs(root.path(), candidate).expect_err("invalid deep inputs");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert!(codes.contains(&"CROSS_SPEC_REVIEW_DEEP_INPUT_INVALID"));
    assert!(codes.contains(&"CROSS_SPEC_REVIEW_DEEP_INPUT_DUPLICATE"));
}

#[test]
fn rejects_invalid_candidates_and_direct_only_roadmaps() {
    let root = fixture();
    let invalid = cross_spec_review::resolve_inputs(
        root.path(),
        r#"{"schemaVersion":2,"assessment":" ","deepInputs":[]}"#,
    )
    .expect_err("invalid candidate");
    assert!(
        invalid
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_CANDIDATE_VERSION_UNSUPPORTED")
    );

    write(
        root.path(),
        "steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n"
        ),
    );
    let direct = cross_spec_review::resolve_inputs(
        root.path(),
        r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":[]}"#,
    )
    .expect_err("Direct-only milestone");
    assert!(
        direct
            .issues
            .iter()
            .any(|issue| issue.code == "CROSS_SPEC_REVIEW_DIRECT_ONLY")
    );
}

#[test]
fn blocks_input_resolution_on_contract_graph_errors() {
    let root = fixture();
    write(
        root.path(),
        "specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `missing` → `provider/exports/missing`\n\n## Invariants\n\n## File Ownership\n",
    );

    let error = cross_spec_review::resolve_inputs(
        root.path(),
        r#"{"schemaVersion":1,"assessment":"Reviewed.","deepInputs":[]}"#,
    )
    .expect_err("dangling Contract target");
    assert!(
        error
            .issues
            .iter()
            .any(|issue| issue.code == "CONTRACT_GRAPH_TARGET_ENTRY_MISSING")
    );
}
