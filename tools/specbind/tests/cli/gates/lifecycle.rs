use super::super::*;

#[test]
#[allow(clippy::too_many_lines)] // Keep the transition matrix beside its shared Git fixture assertions.
fn requirement_retirement_is_guarded_and_can_be_the_only_active_work() {
    let live = "### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. Keep accepting orders.\n2. Report orders.\n\n### Requirement 2: Export\n\n#### Acceptance Criteria\n\n1. Export orders.\n";
    let retired = live.replace(
        "2. Report orders.",
        "2. _Retired_ Report orders.\n   - Reporting ceases.",
    );
    let compact = "### Requirement 1: _Retired_ Checkout\n\n### Requirement 2: Export\n\n#### Acceptance Criteria\n\n1. Export orders.\n";
    let cases = [
        (live, retired.as_str(), "1.2", None),
        (
            live,
            retired.as_str(),
            "1.1",
            Some("SPEC_REQUIREMENTS_RETIREMENT_SELECTION_MISSING"),
        ),
        (
            retired.as_str(),
            live,
            "1.1",
            Some("SPEC_REQUIREMENTS_RETIRED_ID_REUSED"),
        ),
        (
            retired.as_str(),
            retired.as_str(),
            "1.2",
            Some("SPEC_REQUIREMENTS_RETIREMENT_SELECTION_INVALID"),
        ),
        (live, compact, "1.1,1.2", None),
        (
            live,
            compact,
            "1.1",
            Some("SPEC_REQUIREMENTS_RETIREMENT_SELECTION_MISSING"),
        ),
        (
            compact,
            live,
            "1.1",
            Some("SPEC_REQUIREMENTS_RETIRED_ID_REUSED"),
        ),
        (
            live,
            "### Requirement 1: _Retired_\n\n### Requirement 2: _Retired_\n",
            "1.1,1.2,2.1",
            Some("SPEC_REQUIREMENTS_SPEC_RETIREMENT_UNSUPPORTED"),
        ),
    ];
    for (before, after, selection, diagnostic) in cases {
        let root = project_fixture();
        let artifact = |body: &str| {
            format!(
                "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n{body}"
            )
        };
        write(
            root.path(),
            ".specbind/specs/checkout/requirements.md",
            &artifact(before),
        );
        write(
            root.path(),
            ".specbind/specs/checkout/spec.yaml",
            "schema_version: 1\nactive_change: null\n",
        );
        commit_all(root.path());
        let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
        write(
            root.path(),
            ".specbind/steering/roadmap.md",
            &format!(
                "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Retire obligations\n---\n# Roadmap\n"
            ),
        );
        let state = format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: requirements\n  requirement_ids: null\n"
        );
        write(root.path(), ".specbind/specs/checkout/spec.yaml", &state);
        write(
            root.path(),
            ".specbind/specs/checkout/requirements.md",
            &artifact(after),
        );
        let mut command = specbind_command();
        let assertion = command
            .current_dir(root.path())
            .args([
                "spec",
                "requirements",
                "approve",
                "checkout",
                "--approval-mode",
                "explicit",
                "--requirement-ids",
                selection,
            ])
            .assert();
        if let Some(diagnostic) = diagnostic {
            assertion
                .failure()
                .stderr(predicate::str::contains(diagnostic));
            assert_eq!(
                fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml")).unwrap(),
                state
            );
        } else {
            assertion.success();
            let scope =
                fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml")).unwrap();
            assert!(scope.contains("state: design"));
            for id in selection.split(',') {
                assert!(scope.contains(id));
            }
        }
    }
}

#[test]
fn walks_every_gate_from_requirements_to_implementation() {
    let root = project_fixture();
    write_gate_fixture(root.path());

    let mut requirements = specbind_command();
    requirements
        .current_dir(root.path())
        .args([
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK SPEC_REQUIREMENTS_APPROVED: Approved requirements for spec checkout.\n  State: design\n  Approval mode: explicit\n  Passed at: ",
            )
            .and(predicate::str::contains("\n  Approved requirement IDs: 1\n")),
        )
        .stderr("");

    let mut design = specbind_command();
    design
        .current_dir(root.path())
        .args([
            "spec",
            "design",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK SPEC_DESIGN_APPROVED: Approved design for spec checkout.\n  State: tasks\n  Approval mode: explicit\n  Passed at: ",
            )
            .and(predicate::str::contains("Approved requirement IDs").not()),
        )
        .stderr("");

    let mut review = specbind_command();
    review
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible."))
        .assert()
        .success();

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        gate_task_fixture(),
    );
    let mut tasks = specbind_command();
    tasks
        .current_dir(root.path())
        .args([
            "spec",
            "tasks",
            "approve",
            "checkout",
            "--approval-mode",
            "delegated",
            "--delegation-workflow",
            "sb-plan",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK SPEC_TASKS_APPROVED: Approved tasks for spec checkout.\n  State: implementation\n  Approval mode: delegated\n  Delegation workflow: sb-plan\n  Passed at: ",
            ),
        )
        .stderr("");

    let mut status = specbind_command();
    status
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State: implementation\n")
                .and(predicate::str::contains("  State health: consistent\n"))
                .and(predicate::str::contains(
                    "  Gates: requirements=fresh, design=fresh, tasks=fresh, completion=not_reached\n",
                )),
        );
}

#[test]
fn design_approval_does_not_read_a_retained_downstream_task_plan() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());
    let retained = "this is not a task plan\n";
    write(root.path(), ".specbind/specs/checkout/tasks.yaml", retained);

    let mut design = specbind_command();
    design
        .current_dir(root.path())
        .args([
            "spec",
            "design",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success();

    assert_eq!(
        fs::read_to_string(root.path().join(".specbind/specs/checkout/tasks.yaml")).unwrap(),
        retained
    );
    let mut complete = specbind_command();
    complete
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "ARTIFACT_TASKS_STRUCTURAL_INVALID",
        ));
}

#[test]
fn design_reapproval_accepts_an_uncommitted_cli_owned_rewind_delta() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());
    approve_design(root.path());
    let mut review = specbind_command();
    review
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible before Design recovery."))
        .assert()
        .success();
    commit_all(root.path());

    let mut invalidate = specbind_command();
    invalidate
        .current_dir(root.path())
        .args(["spec", "design", "invalidate", "checkout"])
        .assert()
        .success();

    let rewind_status = git_stdout(root.path(), &["status", "--short"]);
    assert!(rewind_status.contains(".specbind/specs/checkout/spec.yaml"));
    assert!(rewind_status.contains(".specbind/state/contract-review.md"));
    let rewind_diff = git_stdout(
        root.path(),
        &[
            "diff",
            "--",
            ".specbind/specs/checkout/spec.yaml",
            ".specbind/state/contract-review.md",
        ],
    );
    assert!(rewind_diff.contains("-  state: tasks"));
    assert!(rewind_diff.contains("+  state: design"));
    assert!(rewind_diff.contains("deleted file mode"));

    let design_path = ".specbind/specs/checkout/design.md";
    let mut design = fs::read_to_string(root.path().join(design_path)).expect("Design");
    design.push_str("\nRecovery keeps the CLI-owned rewind delta uncommitted.\n");
    write(root.path(), design_path, &design);
    let mut approve = specbind_command();
    approve
        .current_dir(root.path())
        .args([
            "spec",
            "design",
            "approve",
            "checkout",
            "--approval-mode",
            "delegated",
            "--delegation-workflow",
            "sb-plan",
        ])
        .assert()
        .success();

    let state = fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml"))
        .expect("reapproved Spec");
    assert!(state.contains("state: tasks"));
    assert!(state.contains("approval_mode: delegated"));
    assert!(state.contains("delegation_workflow: sb-plan"));
    let final_status = git_stdout(root.path(), &["status", "--short"]);
    assert!(final_status.contains(design_path));
    assert!(final_status.contains(".specbind/specs/checkout/spec.yaml"));
    assert!(final_status.contains(".specbind/state/contract-review.md"));
}

#[test]
#[allow(clippy::too_many_lines)] // Keep the complete rewind-to-repair integration path in one assertion sequence.
fn requirements_rewind_can_reach_design_with_a_retained_previous_task_plan() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());
    approve_design(root.path());

    let mut review = specbind_command();
    review
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible."))
        .assert()
        .success();
    let retained = "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['1.1']\nexecution:\n  tasks:\n    '1':\n      status: completed\n";
    write(root.path(), ".specbind/specs/checkout/tasks.yaml", retained);
    let mut tasks = specbind_command();
    tasks
        .current_dir(root.path())
        .args([
            "spec",
            "tasks",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success();
    commit_all(root.path());

    let mut invalidate = specbind_command();
    invalidate
        .current_dir(root.path())
        .args(["spec", "requirements", "invalidate", "checkout"])
        .assert()
        .success();
    write(
        root.path(),
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n\n### Requirement 2: Receipt\n\n#### Acceptance Criteria\n\n1. It records a receipt.\n",
    );
    let mut requirements = specbind_command();
    requirements
        .current_dir(root.path())
        .args([
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
            "--requirement-ids",
            "2.1",
        ])
        .assert()
        .success();
    write(
        root.path(),
        ".specbind/specs/checkout/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['2.1']\n---\n# Design\n\n_Requirements: 2.1_\n",
    );

    let mut recovery_status = specbind_command();
    recovery_status
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State health: inconsistent\n")
                .and(predicate::str::contains(
                    "  Task plan authority: not current; reconcile in Tasks phase\n",
                ))
                .and(predicate::str::contains("TRACEABILITY_TASK_SCOPE_INACTIVE")),
        );
    let recovery_json = specbind_command()
        .current_dir(root.path())
        .args(["spec", "status", "checkout", "--json"])
        .output()
        .expect("recovery status runs");
    assert!(recovery_json.status.success());
    assert!(recovery_json.stderr.is_empty());
    let recovery_json: serde_json::Value =
        serde_json::from_slice(&recovery_json.stdout).expect("status is one JSON document");
    assert_eq!(recovery_json["data"]["health"], "inconsistent");
    assert_eq!(
        recovery_json["data"]["taskPlanAuthority"],
        serde_json::json!({
            "status": "not_current",
            "nextAction": "reconcile_in_tasks_phase"
        })
    );

    let mut complete = specbind_command();
    complete
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("TRACEABILITY_TASK_SCOPE_INACTIVE"));
    let mut design_check = specbind_command();
    design_check
        .current_dir(root.path())
        .args(["check", "traceability", "checkout", "--for-design"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Design coverage: 1/1"));
    approve_design(root.path());

    assert_eq!(
        fs::read_to_string(root.path().join(".specbind/specs/checkout/tasks.yaml")).unwrap(),
        retained
    );

    let mut renewed_review = specbind_command();
    renewed_review
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible after Requirements recovery."))
        .assert()
        .success();
    assert_eq!(
        fs::read_to_string(root.path().join(".specbind/specs/checkout/tasks.yaml")).unwrap(),
        retained,
        "Contract Review must not read, rewrite, or discard the retained plan and progress"
    );

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Record receipt\n      requirement_ids: ['2.1']\n",
    );
    let mut repaired_tasks = specbind_command();
    repaired_tasks
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .success();
    let mut approve_repaired_tasks = specbind_command();
    approve_repaired_tasks
        .current_dir(root.path())
        .args([
            "spec",
            "tasks",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success();
    let repaired_status = specbind_command()
        .current_dir(root.path())
        .args(["spec", "status", "checkout", "--json"])
        .output()
        .expect("repaired status runs");
    let repaired_status: serde_json::Value =
        serde_json::from_slice(&repaired_status.stdout).expect("status is one JSON document");
    assert!(repaired_status["data"].get("taskPlanAuthority").is_none());
}
