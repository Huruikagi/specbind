use super::super::*;

#[test]
fn requirements_approval_rejects_ids_removed_since_the_milestone_baseline() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n2. It reports the result.\n",
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
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
    );
    write(
        root.path(),
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works differently.\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: requirements\n  requirement_ids: null\n"
        ),
    );

    let mut approve = specbind_command();
    approve
        .current_dir(root.path())
        .args([
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--approval-mode",
            "delegated",
            "--delegation-workflow",
            "sb-plan",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "SPEC_REQUIREMENTS_RETIREMENT_UNSUPPORTED",
        ));

    let spec = fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml"))
        .expect("read unchanged spec state");
    assert!(spec.contains("state: requirements"));
    assert!(!spec.contains("gate_evidence:"));
}

#[test]
fn reports_worktree_dirt_only_when_a_clean_revision_would_unlock_progress() {
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

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        gate_task_fixture(),
    );
    let mut approve_tasks = specbind_command();
    approve_tasks
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

    let mut complete = specbind_command();
    complete
        .current_dir(root.path())
        .args(["tasks", "complete", "checkout", "1"])
        .assert()
        .success();

    let mut dirty = specbind_command();
    dirty
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: implementation\n")
                .and(predicate::str::contains("  Actionable: none\n"))
                .and(predicate::str::contains(
                    "  Current blockers: WORKTREE_NOT_CLEAN\n",
                ))
                .and(predicate::str::contains(
                    "  Worktree action: review and commit or otherwise reconcile current changes to continue\n",
                ))
                .and(predicate::str::contains(
                    "  Release readiness: not evaluated until validation\n",
                ))
                .and(predicate::str::contains("Release blockers:").not()),
        );

    commit_all(root.path());
    let mut clean = specbind_command();
    clean
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: validation\n")
                .and(predicate::str::contains(
                    "    - spec:checkout action=validation command_operand=checkout handler=skill:sb-validate-implementation mode=item\n",
                ))
                .and(predicate::str::contains("Current blockers:").not())
                .and(predicate::str::contains("  Release blockers:")),
        );
}

#[test]
fn reports_blocked_task_progress_and_reason_without_misclassifying_dirty_state() {
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

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        gate_task_fixture(),
    );
    let mut approve_tasks = specbind_command();
    approve_tasks
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

    let mut block = specbind_command();
    block
        .current_dir(root.path())
        .args([
            "tasks",
            "block",
            "checkout",
            "1",
            "--reason",
            "Waiting for representative page inputs",
        ])
        .assert()
        .success();

    let mut text = specbind_command();
    text.current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: implementation\n")
                .and(predicate::str::contains(
                    "      Tasks: 0/1 completed, 0 pending, 1 blocked\n",
                ))
                .and(predicate::str::contains("      Next tasks: none\n"))
                .and(predicate::str::contains(
                    "      Blocked task 1: Waiting for representative page inputs\n",
                ))
                .and(predicate::str::contains("  Actionable: none\n"))
                .and(predicate::str::contains(
                    "  Current blockers: TASKS_BLOCKED\n",
                ))
                .and(predicate::str::contains("WORKTREE_NOT_CLEAN").not())
                .and(predicate::str::contains("  Revision: unavailable\n").not()),
        );

    let output = specbind_command()
        .current_dir(root.path())
        .args(["milestone", "status", "--json"])
        .output()
        .expect("milestone status runs");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let actual: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON document");
    assert!(actual["data"]["revision"].is_string());
    assert_eq!(actual["data"]["actionable"], serde_json::json!([]));
    assert_eq!(
        actual["data"]["currentBlockers"],
        serde_json::json!(["TASKS_BLOCKED"])
    );
    assert_eq!(
        actual["data"]["items"][0]["taskProgress"],
        serde_json::json!({
            "total": 1,
            "completed": 0,
            "pending": 0,
            "blocked": 1,
            "nextTasks": []
        })
    );
    assert_eq!(
        actual["data"]["items"][0]["taskBlockers"],
        serde_json::json!([{
            "taskId": "1",
            "reason": "Waiting for representative page inputs"
        }])
    );
}
