use specbind::{
    domain::{spec::Spec, tasks::Tasks},
    schema::runtime,
};

fn semantic_spec(input: &str) -> Result<Spec, specbind::domain::SemanticIssues> {
    runtime::load_spec(input)
        .expect("test input must be structurally valid")
        .try_into()
}

fn semantic_tasks(input: &str) -> Result<Tasks, specbind::domain::SemanticIssues> {
    runtime::load_tasks(input)
        .expect("test input must be structurally valid")
        .try_into()
}

#[test]
fn accepts_idle_and_consistent_active_specs() {
    semantic_spec("schema_version: 1\nactive_change: null\n")
        .expect("idle metadata is semantically valid");
    semantic_spec(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: design\n  requirement_ids: ['1.1', '2.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1', '2.1']\n      input_revisions:\n        requirements: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
    )
    .expect("consistent design metadata is semantically valid");
}

#[test]
fn reports_all_state_and_requirement_contradictions() {
    let error = semantic_spec(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: tasks\n  requirement_ids: null\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
    )
    .expect_err("tasks state needs IDs and design evidence");

    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert_eq!(
        codes,
        ["SPEC_REQUIREMENT_IDS_MISSING", "SPEC_STATE_GATE_EVIDENCE"]
    );
}

#[test]
fn validates_requirement_id_format_order_and_approval_match() {
    let error = semantic_spec(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: design\n  requirement_ids: ['2.1', '1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1', '2.1']\n      input_revisions:\n        requirements: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
    )
    .expect_err("active IDs must be ordered and exactly approved");

    assert!(
        error
            .issues
            .iter()
            .any(|issue| issue.code == "SPEC_REQUIREMENT_ID_ORDER")
    );
    assert!(
        error
            .issues
            .iter()
            .any(|issue| issue.code == "SPEC_REQUIREMENT_IDS_MISMATCH")
    );
}

#[test]
fn rejects_noncanonical_requirement_ids() {
    let error = semantic_spec(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: design\n  requirement_ids: ['REQ-1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['REQ-1']\n      input_revisions:\n        requirements: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
    )
    .expect_err("Requirement IDs use the accepted N.M grammar");

    assert!(
        error
            .issues
            .iter()
            .any(|issue| issue.code == "SPEC_REQUIREMENT_ID_FORMAT")
    );
}

#[test]
fn accepts_valid_task_positions_dependencies_and_execution() {
    semantic_tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: group\n      title: Work\n      tasks:\n        - id: '1.1'\n          kind: task\n          title: First\n          requirement_ids: ['1.1']\n        - id: '1.2'\n          kind: task\n          title: Second\n          requirement_ids: ['1.2']\n          depends_on: ['1.1']\n    - id: '2'\n      kind: task\n      title: Finish\n      requirement_ids: ['1.3']\nexecution:\n  tasks:\n    '1.1':\n      status: completed\n",
    )
    .expect("consistent task plan is semantically valid");
}

#[test]
fn reports_positional_and_unresolved_references() {
    let error = semantic_tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '2'\n      kind: task\n      title: Work\n      requirement_ids: ['1.1']\n      depends_on: ['9']\nexecution:\n  tasks:\n    '1':\n      status: completed\n",
    )
    .expect_err("IDs and references must resolve semantically");

    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert!(codes.contains(&"TASK_POSITIONAL_ID"));
    assert!(codes.contains(&"TASK_DEPENDENCY_MISSING"));
    assert!(codes.contains(&"TASK_EXECUTION_UNKNOWN"));
}

#[test]
fn rejects_noncanonical_task_requirement_ids() {
    let error = semantic_tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Work\n      requirement_ids: ['REQ-1']\n",
    )
    .expect_err("Task Requirement IDs use the accepted N.M grammar");

    assert!(
        error
            .issues
            .iter()
            .any(|issue| issue.code == "TASK_REQUIREMENT_ID_FORMAT")
    );
}

#[test]
fn rejects_self_dependencies_and_effective_cycles() {
    let self_error = semantic_tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Work\n      requirement_ids: ['1.1']\n      depends_on: ['1']\n",
    )
    .expect_err("self dependency is invalid");
    assert!(
        self_error
            .issues
            .iter()
            .any(|issue| issue.code == "TASK_DEPENDENCY_SELF")
    );

    let cycle_error = semantic_tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: First\n      requirement_ids: ['1.1']\n      depends_on: ['2']\n    - id: '2'\n      kind: task\n      title: Second\n      requirement_ids: ['1.2']\n",
    )
    .expect_err("explicit and implicit edges form a cycle");
    assert!(
        cycle_error
            .issues
            .iter()
            .any(|issue| issue.code == "TASK_DEPENDENCY_CYCLE")
    );
}
