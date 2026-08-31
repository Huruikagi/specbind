use super::*;

#[test]
fn reports_help() {
    let mut command = specbind_command();

    command
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "Bind durable specifications to agent-assisted software delivery.",
        ))
        .stdout(predicate::str::contains(
            "Report bugs or suggest improvements with `specbind feedback`.",
        ));
}

#[test]
fn unrecognized_nested_routes_do_not_suggest_unrelated_top_level_commands() {
    let mut command = specbind_command();

    command.arg("status").assert().failure().stderr(
        predicate::str::contains("unrecognized subcommand 'status'")
            .and(predicate::str::contains("a similar subcommand exists").not()),
    );
}

#[test]
fn reports_feedback_routes_without_requiring_a_project() {
    let outside = tempfile::tempdir().expect("temporary directory");
    let mut command = specbind_command();

    command
        .current_dir(outside.path())
        .arg("feedback")
        .assert()
        .success()
        .stdout(concat!(
            "OK FEEDBACK_REPORTED: SpecBind feedback routes.\n",
            "  Bug report: https://github.com/Huruikagi/specbind/issues/new?template=bug-report.yml\n",
            "  Improvement proposal: https://github.com/Huruikagi/specbind/issues/new?template=improvement.yml\n",
            "  Include: specbind --version, the affected command or Skill, and reproduction steps\n",
            "  Evidence: Relevant sanitized output or artifacts\n",
            "  Privacy: Remove secrets and private project content before submitting\n",
            "  No information has been transmitted.\n",
        ))
        .stderr("");
}

#[test]
fn reports_version() {
    let mut command = specbind_command();

    command
        .arg("--version")
        .assert()
        .success()
        .stdout("specbind 1.0.0-rc.3\n");
}

#[test]
fn lists_discovered_artifacts_with_the_text_result_contract() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/brief.md",
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n\nRequested change.\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\n# Research\n\nFinding.\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["artifact", "list", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK ARTIFACT_LISTED: Found 2 recognized artifact(s) for spec checkout.\n  selector=brief type=\"SpecBind Brief\" path=specs/checkout/brief.md\n  selector=research type=\"SpecBind Research\" path=specs/checkout/research.md\n",
        )
        .stderr("");
}

#[test]
fn reads_one_artifact_as_unwrapped_raw_markdown() {
    let root = project_fixture();
    let content = "---\ntype: SpecBind Brief\n---\n# Checkout brief\n\nRequested change.\n";
    write(root.path(), ".specbind/specs/checkout/brief.md", content);

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["artifact", "read", "checkout", "brief"])
        .assert()
        .success()
        .stdout(content)
        .stderr("");
}

#[test]
fn projects_live_artifact_instructions_for_the_named_use() {
    let root = project_fixture();
    let content = concat!(
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n\nRequested change.\n\n",
        "<!-- specbind:instruction maintain Preserve the request. -->\n",
        "<!-- specbind:instruction consume Requirements owns scope. -->\n",
    );
    write(root.path(), ".specbind/specs/checkout/brief.md", content);

    let mut maintain = specbind_command();
    maintain
        .current_dir(root.path())
        .args(["artifact", "read", "checkout", "brief", "--for", "maintain"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Preserve the request.")
                .and(predicate::str::contains("Requirements owns scope.").not()),
        );

    let mut consume = specbind_command();
    consume
        .current_dir(root.path())
        .args(["artifact", "read", "checkout", "brief", "--for", "consume"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Requirements owns scope.")
                .and(predicate::str::contains("Preserve the request.").not()),
        );

    let mut raw = specbind_command();
    raw.current_dir(root.path())
        .args(["artifact", "read", "checkout", "brief"])
        .assert()
        .success()
        .stdout(content);
}

#[test]
fn reads_a_valid_selector_despite_unrelated_inventory_diagnostics() {
    let root = project_fixture();
    let content = "---\ntype: SpecBind Brief\n---\n# Checkout brief\n\nRequested change.\n";
    write(root.path(), ".specbind/specs/checkout/brief.md", content);
    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["artifact", "read", "checkout", "brief"])
        .assert()
        .success()
        .stdout(content)
        .stderr(predicate::str::contains("ARTIFACT_RESEARCH_BODY_EMPTY"));
}

#[test]
fn keeps_failed_raw_reads_off_stdout_and_reports_stable_codes() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/brief.md",
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n\nRequested change.\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["artifact", "read", "checkout", "contract"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR ARTIFACT_SELECTOR_NOT_FOUND:",
        ));
}

#[test]
fn reports_partial_inventory_and_rejects_unsafe_spec_roots() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/brief.md",
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n\nRequested change.\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["artifact", "list", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::contains("ERROR ARTIFACT_LIST_FAILED:")
                .and(predicate::str::contains("selector=brief"))
                .and(predicate::str::contains("ARTIFACT_RESEARCH_BODY_EMPTY")),
        );

    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":"../escape","language":"en","agents":["codex"]}"#,
    );
    let mut unsafe_command = specbind_command();
    unsafe_command
        .current_dir(root.path())
        .args(["artifact", "list", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with("ERROR SPEC_DIR_INVALID:"));
}

#[test]
fn lists_tasks_with_group_progress_status_and_actionability() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        task_fixture(),
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["tasks", "list", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK TASKS_LISTED: Listed 4 task(s) for spec checkout (1 completed, 2 pending, 1 blocked).\n  [partial 1/2; 0 blocked] 1 Build\n    [completed] 1.1 First\n    [pending actionable] 1.2 Second\n  [blocked] 2 Integrate\n  [pending waiting] 3 Document\n",
        )
        .stderr("");
}

#[test]
fn shows_full_task_content_and_derived_prerequisites() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        task_fixture(),
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["tasks", "show", "checkout", "3"])
        .assert()
        .success()
        .stdout(
            "OK TASK_SHOWN: Found task 3 in spec checkout.\n  Status: pending waiting\n  Title: Document\n  Group: none\n  Details: none\n  Requirement IDs: 1.4\n  Boundaries: docs/\n  Contracts: none\n  Explicit prerequisites: 1.1\n  Effective prerequisites: 1.1, 2\n  Blocker: none\n  Completion criteria: none\n",
        )
        .stderr("");
}

#[test]
fn rejects_untrustworthy_task_projections_and_unknown_task_ids() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items: []\n",
    );

    let mut invalid = specbind_command();
    invalid
        .current_dir(root.path())
        .args(["tasks", "list", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with("ERROR TASKS_READ_FAILED:"));

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        task_fixture(),
    );
    let mut missing = specbind_command();
    missing
        .current_dir(root.path())
        .args(["tasks", "show", "checkout", "9"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with("ERROR TASK_NOT_FOUND:"));
}

#[test]
fn records_task_progress_in_plan_order() {
    let root = project_fixture();
    write_progress_fixture(root.path());

    let mut out_of_order = specbind_command();
    out_of_order
        .current_dir(root.path())
        .args(["tasks", "complete", "checkout", "2"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR TASK_COMPLETE_FAILED: Cannot complete task 2 in spec checkout.",
            )
            .and(predicate::str::contains(
                "TASK_PREREQUISITES_INCOMPLETE specs/checkout/tasks.yaml: task 2 cannot complete before its prerequisites: 1",
            )),
        );

    let mut first = specbind_command();
    first
        .current_dir(root.path())
        .args(["tasks", "complete", "checkout", "1"])
        .assert()
        .success()
        .stdout(
            "OK TASK_COMPLETED: Completed task 1 in spec checkout.\n  Progress: 1/2 completed, 1 pending, 0 blocked\n  Next actionable: 2\n",
        )
        .stderr("");

    let mut repeat = specbind_command();
    repeat
        .current_dir(root.path())
        .args(["tasks", "complete", "checkout", "1"])
        .assert()
        .success()
        .stdout(
            "NO_CHANGE TASK_ALREADY_COMPLETED: Task 1 in spec checkout is already completed.\n",
        );

    let tasks = fs::read_to_string(root.path().join(".specbind/specs/checkout/tasks.yaml"))
        .expect("recorded plan");
    assert!(tasks.contains("execution:"), "{tasks}");
    assert!(tasks.contains("status: completed"), "{tasks}");
}

#[test]
fn records_and_clears_a_task_blocker() {
    let root = project_fixture();
    write_progress_fixture(root.path());

    let mut invalid = specbind_command();
    invalid
        .current_dir(root.path())
        .args(["tasks", "block", "checkout", "2", "--reason", "   "])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("TASK_BLOCKED_REASON_INVALID"));

    let mut block = specbind_command();
    block
        .current_dir(root.path())
        .args([
            "tasks",
            "block",
            "checkout",
            "2",
            "--reason",
            "Waiting on the upstream API",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("OK TASK_BLOCKED: Blocked task 2 in spec checkout.\n")
                .and(predicate::str::contains(
                    "\n  Blocker: Waiting on the upstream API\n",
                ))
                .and(predicate::str::contains(
                    "\n  Progress: 0/2 completed, 1 pending, 1 blocked\n",
                )),
        );

    let mut same = specbind_command();
    same.current_dir(root.path())
        .args([
            "tasks",
            "block",
            "checkout",
            "2",
            "--reason",
            "Waiting on the upstream API",
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE TASK_ALREADY_BLOCKED:",
        ));

    let mut reopen = specbind_command();
    reopen
        .current_dir(root.path())
        .args(["tasks", "reopen", "checkout", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "OK TASK_REOPENED: Reopened task 2 in spec checkout.\n",
        ));

    let tasks = fs::read_to_string(root.path().join(".specbind/specs/checkout/tasks.yaml"))
        .expect("reopened plan");
    assert!(
        !tasks.contains("execution:"),
        "an emptied execution container is removed: {tasks}"
    );

    let mut absent = specbind_command();
    absent
        .current_dir(root.path())
        .args(["tasks", "reopen", "checkout", "2"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("NO_CHANGE TASK_NOT_RECORDED:"));
}

#[test]
fn refuses_task_progress_outside_implementation_and_for_groups() {
    let root = project_fixture();
    write_progress_fixture(root.path());
    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: group\n      title: Build\n      tasks:\n        - id: '1.1'\n          kind: task\n          title: A\n          requirement_ids: ['1.1']\n        - id: '1.2'\n          kind: task\n          title: B\n          requirement_ids: ['1.1']\n",
    );

    let mut group = specbind_command();
    group
        .current_dir(root.path())
        .args(["tasks", "complete", "checkout", "1"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("TASK_NOT_FOUND"));

    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );
    let mut idle = specbind_command();
    idle.current_dir(root.path())
        .args(["tasks", "complete", "checkout", "1.1"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "TASK_SPEC_STATE_INVALID specs/checkout/spec.yaml: task progress requires the Spec in implementation state",
        ));
}

/// Writes a Spec in `implementation` state with a two-task ordered plan.
#[test]
fn reports_composed_spec_status_with_freshness_coverage_and_progress() {
    let root = project_fixture();
    write_status_fixture(root.path());

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK SPEC_STATUS_REPORTED: Reported status for spec checkout.\n  State: implementation\n  Milestone: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  State health: consistent\n  Semantic alignment: not evaluated\n  Gates: requirements=fresh, design=fresh, tasks=fresh, completion=not_reached\n  Next action: implementation\n  Delegated gates: none\n  Task progress: 2 total, 1 completed, 0 pending, 1 blocked\n  Next task: none\n  Task blockers:\n    - 2: Waiting for review\n  Requirement coverage: design 1/1, tasks 1/1 (required)\n  Diagnostics: none\n",
        )
        .stderr("");
}

#[test]
fn reports_composed_spec_status_as_command_specific_json() {
    let root = project_fixture();
    write_status_fixture(root.path());

    let output = specbind_command()
        .current_dir(root.path())
        .args(["spec", "status", "checkout", "--json"])
        .output()
        .expect("spec status runs");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let actual: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON document");
    assert_eq!(
        actual,
        serde_json::json!({
            "status": "ok",
            "code": "SPEC_STATUS_REPORTED",
            "data": {
                "spec": "checkout",
                "state": "implementation",
                "milestone": "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62",
                "health": "consistent",
                "semanticAlignment": "not_evaluated",
                "gates": {
                    "requirements": "fresh",
                    "design": "fresh",
                    "tasks": "fresh",
                    "completion": "not_reached"
                },
                "nextAction": "implementation",
                "expectedRequirementsWork": false,
                "expectedDesignWork": null,
                "contractReview": null,
                "delegatedGates": [],
                "tasks": {
                    "total": 2,
                    "completed": 1,
                    "pending": 0,
                    "blocked": 1,
                    "nextTasks": [],
                    "blockers": [{
                        "taskId": "2",
                        "reason": "Waiting for review"
                    }]
                },
                "coverage": {
                    "active": 1,
                    "design": 1,
                    "tasks": 1,
                    "tasksRequired": true
                },
                "diagnostics": []
            }
        })
    );
}

#[test]
fn reports_validation_after_every_task_is_complete() {
    let root = project_fixture();
    write_status_fixture(root.path());
    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['1.1']\n    - id: '2'\n      kind: task\n      title: Review\n      requirement_ids: ['1.1']\nexecution:\n  tasks:\n    '1':\n      status: completed\n    '2':\n      status: completed\n",
    );

    let mut text = specbind_command();
    text.current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("  Next action: validation\n").and(
            predicate::str::contains(
                "  Task progress: 2 total, 2 completed, 0 pending, 0 blocked\n",
            ),
        ));

    let output = specbind_command()
        .current_dir(root.path())
        .args(["spec", "status", "checkout", "--json"])
        .output()
        .expect("spec status runs");
    assert!(output.status.success());
    let actual: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON document");
    assert_eq!(actual["data"]["nextAction"], "validation");
}
#[test]
fn reports_a_clean_idle_spec_without_requiring_active_artifacts() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK SPEC_STATUS_REPORTED: Reported status for spec checkout.\n  State: idle\n  Milestone: none\n  State health: consistent\n  Semantic alignment: not evaluated\n  Gates: requirements=not_reached, design=not_reached, tasks=not_reached, completion=not_reached\n  Next action: none\n  Task progress: unavailable\n  Next task: none\n  Task blockers: none\n  Requirement coverage: inactive\n  Diagnostics: none\n",
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\nRetained draft\n",
    );
    let mut inconsistent = specbind_command();
    inconsistent
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State health: inconsistent\n")
                .and(predicate::str::contains("SPEC_IDLE_ARTIFACT_PRESENT")),
        );
}

#[test]
fn reports_semantic_spec_contradictions_as_inconsistent_without_repairing() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: design\n  requirement_ids: ['1.1']\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State: design\n")
                .and(predicate::str::contains("  State health: inconsistent\n"))
                .and(predicate::str::contains("SPEC_STATE_GATE_EVIDENCE")),
        )
        .stderr("");
}

#[test]
fn reports_unstarted_design_as_expected_work_without_weakening_traceability() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());
    fs::remove_file(root.path().join(".specbind/specs/checkout/design.md"))
        .expect("remove the prewritten Design fixture");
    fs::remove_file(root.path().join(".specbind/specs/checkout/contract.yaml"))
        .expect("remove the prewritten Contract fixture");

    let mut status = specbind_command();
    status
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State: design\n")
                .and(predicate::str::contains("  State health: consistent\n"))
                .and(predicate::str::contains("  Next action: design\n"))
                .and(predicate::str::contains(
                    "  Expected work: cover 1 active requirement(s) in Design\n",
                ))
                .and(predicate::str::contains("  Next task: none\n"))
                .and(predicate::str::contains("  Task blockers: none\n"))
                .and(predicate::str::contains("  Diagnostics: none\n"))
                .and(predicate::str::contains("TRACEABILITY_DESIGN_COVERAGE_MISSING").not()),
        )
        .stderr("");

    let mut traceability = specbind_command();
    traceability
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "TRACEABILITY_DESIGN_COVERAGE_MISSING",
        ));
}

#[test]
fn reports_unstarted_requirements_as_expected_work_without_weakening_traceability() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    fs::remove_file(root.path().join(".specbind/specs/checkout/requirements.md"))
        .expect("remove the prewritten Requirements fixture");
    fs::remove_file(root.path().join(".specbind/specs/checkout/design.md"))
        .expect("remove the prewritten Design fixture");
    fs::remove_file(root.path().join(".specbind/specs/checkout/contract.yaml"))
        .expect("remove the prewritten Contract fixture");

    let mut status = specbind_command();
    status
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State: requirements\n")
                .and(predicate::str::contains("  State health: consistent\n"))
                .and(predicate::str::contains("  Next action: requirements\n"))
                .and(predicate::str::contains(
                    "  Expected work: author Requirements\n",
                ))
                .and(predicate::str::contains("  Diagnostics: none\n"))
                .and(predicate::str::contains("TRACEABILITY_REQUIREMENTS_UNAVAILABLE").not()),
        )
        .stderr("");

    let mut milestone = specbind_command();
    milestone
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State health: consistent\n")
                .and(predicate::str::contains(
                    "    - spec:checkout action=requirements command_operand=checkout\n",
                ))
                .and(predicate::str::contains(
                    "  Release readiness: not evaluated until validation\n",
                ))
                .and(predicate::str::contains("WORKTREE_NOT_CLEAN").not())
                .and(predicate::str::contains("MILESTONE_SPEC_INCONSISTENT").not()),
        );

    let mut traceability = specbind_command();
    traceability
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "TRACEABILITY_REQUIREMENTS_UNAVAILABLE",
        ));
}

#[test]
fn fails_status_when_spec_metadata_is_not_structurally_readable() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: invalid\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with("ERROR SPEC_STATUS_FAILED:"));
}

#[test]
fn reports_spec_status_failure_as_json_without_stderr_text() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: invalid\n",
    );

    let output = specbind_command()
        .current_dir(root.path())
        .args(["spec", "status", "checkout", "--json"])
        .output()
        .expect("spec status runs");

    assert!(!output.status.success());
    assert!(output.stderr.is_empty());
    let actual: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON document");
    assert_eq!(actual["status"], "error");
    assert_eq!(actual["code"], "SPEC_STATUS_FAILED");
    assert_eq!(actual["message"], "Cannot report status for spec checkout.");
    assert!(
        actual["details"]
            .as_array()
            .is_some_and(|details| !details.is_empty())
    );
}
