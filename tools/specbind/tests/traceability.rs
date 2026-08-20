use std::fs;
use std::path::Path;

use specbind::{
    artifacts::resolve_traceability,
    traceability::{self, DesignRequirementSet, TaskRequirementSet},
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

fn task(task_id: &str, values: &[&str]) -> TaskRequirementSet {
    TaskRequirementSet {
        task_id: task_id.to_owned(),
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
        Some(vec![task("1", &["1.1", "2.1"])]),
        true,
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
        None,
        false,
    );
    let codes = report
        .issues
        .iter()
        .filter_map(|issue| Some((issue.code, issue.requirement_id.as_deref()?)))
        .collect::<Vec<_>>();

    assert!(codes.contains(&("TRACEABILITY_DESIGN_REQUIREMENT_UNKNOWN", "9.1")));
    assert!(codes.contains(&("TRACEABILITY_ACTIVE_REQUIREMENT_UNKNOWN", "8.1")));
    assert!(codes.contains(&("TRACEABILITY_DESIGN_COVERAGE_MISSING", "2.1")));
    assert!(!codes.contains(&("TRACEABILITY_DESIGN_COVERAGE_MISSING", "8.1")));
}

#[test]
fn skips_active_coverage_when_no_active_set_is_established() {
    let report = traceability::evaluate(&strings(&["1.1"]), Vec::new(), None, None, false);

    assert!(report.issues.is_empty());
    assert!(report.active_requirement_ids.is_none());
}

#[test]
fn validates_task_references_and_required_active_coverage() {
    let report = traceability::evaluate(
        &strings(&["1.1", "2.1", "3.1"]),
        vec![design("design/main", &["1.1", "2.1"])],
        Some(strings(&["1.1", "2.1"])),
        Some(vec![task("1", &["1.1", "9.1"]), task("2", &["3.1"])]),
        true,
    );
    let codes = report
        .issues
        .iter()
        .filter_map(|issue| Some((issue.code, issue.requirement_id.as_deref()?)))
        .collect::<Vec<_>>();

    assert!(codes.contains(&("TRACEABILITY_TASK_REQUIREMENT_UNKNOWN", "9.1")));
    assert!(codes.contains(&("TRACEABILITY_TASK_COVERAGE_MISSING", "2.1")));
    assert!(!codes.contains(&("TRACEABILITY_TASK_COVERAGE_MISSING", "3.1")));
    assert_eq!(report.task_requirement_ids, strings(&["1.1", "3.1", "9.1"]));
}

#[test]
fn requires_tasks_only_at_the_task_required_boundary() {
    let not_required = traceability::evaluate(
        &strings(&["1.1"]),
        vec![design("design/main", &["1.1"])],
        Some(strings(&["1.1"])),
        None,
        false,
    );
    assert!(not_required.issues.is_empty());

    let required = traceability::evaluate(
        &strings(&["1.1"]),
        vec![design("design/main", &["1.1"])],
        Some(strings(&["1.1"])),
        None,
        true,
    );
    assert_eq!(required.issues[0].code, "TRACEABILITY_TASKS_UNAVAILABLE");
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
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: implementation\n  requirement_ids: ['1.1', '2.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1', '2.1']\n      input_revisions:\n        requirements: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        contract: sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\n        design/main: sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc\n    tasks:\n      passed_at: 2026-08-16T12:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        tasks.yaml#plan: sha256:dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd\n",
    );
    write(
        root.path(),
        "specs/example/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Implement first\n      requirement_ids: ['1.1', '8.1']\n",
    );

    let resolution = resolve_traceability(root.path(), "example");
    let report = resolution.report.expect("foundational inputs are valid");

    assert_eq!(report.requirement_ids, strings(&["1.1", "2.1", "3.1"]));
    assert_eq!(
        report.active_requirement_ids,
        Some(strings(&["1.1", "2.1"]))
    );
    assert!(report.tasks_required);
    assert_eq!(report.task_requirement_ids, strings(&["1.1", "8.1"]));
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
    let design_missing = resolution
        .inventory
        .issues
        .iter()
        .find(|issue| issue.code == "TRACEABILITY_DESIGN_COVERAGE_MISSING")
        .expect("missing active Design coverage");
    assert_eq!(
        design_missing.path.as_ref().map(|path| path.as_str()),
        Some("specs/example/spec.yaml")
    );
    assert!(design_missing.message.contains("2.1"));
    let task_unknown = resolution
        .inventory
        .issues
        .iter()
        .find(|issue| issue.code == "TRACEABILITY_TASK_REQUIREMENT_UNKNOWN")
        .expect("unknown Task reference");
    assert_eq!(
        task_unknown.path.as_ref().map(|path| path.as_str()),
        Some("specs/example/tasks.yaml")
    );
    assert!(resolution.inventory.issues.iter().any(|issue| {
        issue.code == "TRACEABILITY_TASK_COVERAGE_MISSING" && issue.message.contains("2.1")
    }));
    assert!(resolution.inventory.issues.iter().any(|issue| {
        issue.code == "ARTIFACT_DESIGN_REQUIREMENT_IDS_INVALID"
            && issue.path.as_ref().map(|path| path.as_str())
                == Some("specs/example/invalid-coverage.md")
    }));
}

#[test]
fn flags_a_task_that_serves_no_active_requirement() {
    // The reverse direction of coverage. 3.1 exists and is legitimately part of
    // the Spec's contract, but the milestone is not accountable for it, so a
    // task that serves only 3.1 is work nothing asked for.
    let report = traceability::evaluate(
        &strings(&["1.1", "2.1", "3.1"]),
        vec![design("design/main", &["1.1", "2.1", "3.1"])],
        Some(strings(&["1.1", "2.1"])),
        Some(vec![
            task("1", &["1.1", "2.1"]),
            task("2", &["1.1", "3.1"]),
            task("3", &["3.1"]),
        ]),
        true,
    );
    let flagged = report
        .issues
        .iter()
        .filter(|issue| issue.code == "TRACEABILITY_TASK_SCOPE_INACTIVE")
        .map(|issue| issue.source.as_deref().expect("task source"))
        .collect::<Vec<_>>();

    // Serving one active Requirement is enough; only the task serving none is
    // reported.
    assert_eq!(flagged, vec!["tasks/3"]);
}

#[test]
fn keeps_task_scope_unjudged_without_an_active_set() {
    let report = traceability::evaluate(
        &strings(&["1.1"]),
        vec![design("design/main", &["1.1"])],
        None,
        Some(vec![task("1", &["1.1"])]),
        false,
    );

    assert!(report.issues.is_empty(), "{:?}", report.issues);
}
