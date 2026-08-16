use std::fs;
use std::path::Path;

use specbind::{
    artifacts::resolve_traceability,
    traceability::{self, DesignRequirementSet},
};
use tempfile::TempDir;

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn design(selector: &str, values: &[&str]) -> DesignRequirementSet {
    DesignRequirementSet {
        selector: selector.to_owned(),
        requirement_ids: strings(values),
    }
}

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("fixture parent")).expect("create fixture parent");
    fs::write(path, content).expect("write fixture");
}

#[test]
fn accepts_inactive_design_mappings_and_complete_active_coverage() {
    let report = traceability::evaluate(
        &strings(&["1.1", "2.1", "3.1"]),
        vec![
            design("design/main", &["1.1", "2.1"]),
            design("design/legacy", &["3.1"]),
        ],
        Some(strings(&["1.1", "2.1"])),
    );

    assert!(report.issues.is_empty());
    assert_eq!(
        report.design_requirement_ids,
        strings(&["1.1", "2.1", "3.1"])
    );
}

#[test]
fn distinguishes_unknown_ids_from_missing_active_coverage() {
    let report = traceability::evaluate(
        &strings(&["1.1", "2.1"]),
        vec![design("design/main", &["1.1", "9.1"])],
        Some(strings(&["1.1", "2.1", "8.1"])),
    );
    let codes = report
        .issues
        .iter()
        .map(|issue| (issue.code, issue.requirement_id.as_str()))
        .collect::<Vec<_>>();

    assert!(codes.contains(&("TRACEABILITY_DESIGN_REQUIREMENT_UNKNOWN", "9.1")));
    assert!(codes.contains(&("TRACEABILITY_ACTIVE_REQUIREMENT_UNKNOWN", "8.1")));
    assert!(codes.contains(&("TRACEABILITY_DESIGN_COVERAGE_MISSING", "2.1")));
    assert!(!codes.contains(&("TRACEABILITY_DESIGN_COVERAGE_MISSING", "8.1")));
}

#[test]
fn skips_active_coverage_when_no_active_set_is_established() {
    let report = traceability::evaluate(&strings(&["1.1"]), Vec::new(), None);

    assert!(report.issues.is_empty());
    assert!(report.active_requirement_ids.is_none());
}

#[test]
fn resolves_current_artifacts_and_active_scope_with_owned_paths() {
    let root = TempDir::new().expect("temporary SpecBind root");
    write(
        root.path(),
        "specs/example/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n### Requirement 1: First\n#### Acceptance Criteria\n1. One.\n### Requirement 2: Second\n#### Acceptance Criteria\n1. Two.\n### Requirement 3: Inactive\n#### Acceptance Criteria\n1. Three.\n",
    );
    write(
        root.path(),
        "specs/example/main.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1', '9.1']\n---\n_Requirements: 1.1, 9.1_\n",
    );
    write(
        root.path(),
        "specs/example/legacy.md",
        "---\ntype: SpecBind Design\nartifact_id: legacy\nrequirement_ids: ['3.1']\n---\n_Requirements: 3.1_\n",
    );
    write(
        root.path(),
        "specs/example/invalid-coverage.md",
        "---\ntype: SpecBind Design\nartifact_id: invalid-coverage\nrequirement_ids: ['2.1', '2.1']\n---\n_Requirements: 2.1_\n",
    );
    write(
        root.path(),
        "specs/example/spec.yaml",
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: design\n  requirement_ids: ['1.1', '2.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1', '2.1']\n      input_revisions:\n        requirements: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
    );

    let resolution = resolve_traceability(root.path(), "example");
    let report = resolution.report.expect("foundational inputs are valid");

    assert_eq!(report.requirement_ids, strings(&["1.1", "2.1", "3.1"]));
    assert_eq!(
        report.active_requirement_ids,
        Some(strings(&["1.1", "2.1"]))
    );
    let unknown = resolution
        .inventory
        .issues
        .iter()
        .find(|issue| issue.code == "TRACEABILITY_DESIGN_REQUIREMENT_UNKNOWN")
        .expect("unknown Design reference");
    assert_eq!(
        unknown.path.as_ref().map(|path| path.as_str()),
        Some("specs/example/main.md")
    );
    let missing = resolution
        .inventory
        .issues
        .iter()
        .find(|issue| issue.code == "TRACEABILITY_DESIGN_COVERAGE_MISSING")
        .expect("missing active Design coverage");
    assert_eq!(
        missing.path.as_ref().map(|path| path.as_str()),
        Some("specs/example/spec.yaml")
    );
    assert!(missing.message.contains("2.1"));
    assert!(resolution.inventory.issues.iter().any(|issue| {
        issue.code == "ARTIFACT_DESIGN_REQUIREMENT_IDS_INVALID"
            && issue.path.as_ref().map(|path| path.as_str())
                == Some("specs/example/invalid-coverage.md")
    }));
}
