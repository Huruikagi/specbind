use super::super::*;

#[test]
fn reports_an_identical_fresh_approval_as_no_change() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());

    let mut repeat = specbind_command();
    repeat
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
        .stdout("NO_CHANGE SPEC_REQUIREMENTS_ALREADY_APPROVED: Spec checkout already has identical fresh requirements approval.\n")
        .stderr("");
}

#[test]
fn rejects_gate_approval_from_the_wrong_state() {
    let root = project_fixture();
    write_gate_fixture(root.path());

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
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR SPEC_DESIGN_APPROVE_FAILED: Cannot approve design for spec checkout.",
            )
            .and(predicate::str::contains(
                "SPEC_DESIGN_STATE_INVALID specs/checkout/spec.yaml: design approval requires the Spec in design state",
            )),
        );
}

#[test]
fn requires_an_unambiguous_approval_authority() {
    let root = project_fixture();
    write_gate_fixture(root.path());

    let mut missing = specbind_command();
    missing
        .current_dir(root.path())
        .args([
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--approval-mode"));

    let mut explicit_with_workflow = specbind_command();
    explicit_with_workflow
        .current_dir(root.path())
        .args([
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
            "--delegation-workflow",
            "sb-plan",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "SPEC_GATE_DELEGATION_INVALID explicit approval does not accept a delegation workflow",
        ));

    let mut delegated_without_workflow = specbind_command();
    delegated_without_workflow
        .current_dir(root.path())
        .args([
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--approval-mode",
            "delegated",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "SPEC_GATE_DELEGATION_INVALID delegated approval requires --delegation-workflow",
        ));
}

#[test]
fn rejects_an_empty_unknown_or_duplicated_requirement_selection() {
    let root = project_fixture();
    write_gate_fixture(root.path());

    for (ids, code) in [
        (None, "SPEC_REQUIREMENTS_SELECTION_EMPTY"),
        (Some("9.9"), "SPEC_REQUIREMENTS_SELECTION_UNKNOWN"),
        (Some("1.1,1.1"), "SPEC_REQUIREMENTS_SELECTION_DUPLICATE"),
        (Some("1"), "SPEC_REQUIREMENTS_SELECTION_INVALID"),
    ] {
        let mut command = specbind_command();
        let mut arguments = vec![
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ];
        if let Some(ids) = ids {
            arguments.push("--requirement-ids");
            arguments.push(ids);
        }
        command
            .current_dir(root.path())
            .args(arguments)
            .assert()
            .failure()
            .stdout("")
            .stderr(predicate::str::contains(code));
    }
}

#[test]
fn blocks_tasks_approval_without_a_fresh_contract_review() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());
    approve_design(root.path());
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
            "explicit",
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR SPEC_TASKS_APPROVE_FAILED: Cannot approve tasks for spec checkout.",
            )
            .and(predicate::str::contains(
                "CONTRACT_REVIEW_TASKS_APPROVAL_BLOCKED",
            )),
        );
}

/// The review is a prerequisite of Tasks approval that lives outside the Spec.
/// Before this line existed, a Spec sitting behind the barrier reported
/// `Blockers: none` with no indication that its next transition was refused.
#[test]
fn reports_the_contract_review_barrier_from_the_tasks_state_onward() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());

    let status = |root: &Path| {
        let mut command = specbind_command();
        let output = command
            .current_dir(root)
            .args(["spec", "status", "checkout"])
            .output()
            .expect("spec status runs");
        String::from_utf8(output.stdout).expect("status is UTF-8")
    };

    // The review is not runnable until every participating Spec holds Design
    // approval, so its absence in the `design` state is expected, not a barrier.
    assert!(
        !status(root.path()).contains("Contract review:"),
        "the design state must not report a review that cannot yet run"
    );

    approve_design(root.path());
    assert!(
        status(root.path()).contains("\n  Contract review: absent\n"),
        "the tasks state must report the missing review"
    );

    let mut accept = specbind_command();
    accept
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Assessed."))
        .assert()
        .success();

    let fresh = status(root.path());
    assert!(fresh.contains("\n  Contract review: fresh\n"));
    // Decision 0078 keeps the milestone-owned review out of the per-Spec
    // invariant, so it never moves this Spec's health.
    assert!(fresh.contains("\n  State health: consistent\n"));
}

/// Delegation exists to skip a confirmation the user would otherwise give, and
/// Decision 0100 calls that skip auditable. Before this field, the only durable
/// trace was `spec.yaml` itself, which no command read back.
#[test]
fn reports_which_gates_were_crossed_by_delegation() {
    let root = project_fixture();
    write_gate_fixture(root.path());

    let status = |root: &Path| {
        let mut command = specbind_command();
        let output = command
            .current_dir(root)
            .args(["spec", "status", "checkout"])
            .output()
            .expect("spec status runs");
        String::from_utf8(output.stdout).expect("status is UTF-8")
    };

    // No gate approved yet, so the field is absent rather than empty. The two
    // states mean different things and must not render the same.
    assert!(!status(root.path()).contains("Delegated gates:"));

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
            "quick",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .success();
    assert!(status(root.path()).contains("\n  Delegated gates: requirements (quick)\n"));

    approve_design(root.path());
    let after_explicit = status(root.path());
    assert!(
        after_explicit.contains("\n  Delegated gates: requirements (quick)\n"),
        "an explicit approval adds nothing to the list: {after_explicit}"
    );
}
