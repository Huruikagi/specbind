use super::*;

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
fn reverse_design_approval_enters_adoption_ready_without_tasks() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\nbaseline_version: v2.4.0\ntarget_release: null\nwork_items:\n  reverse_specs:\n    - spec: checkout\n      summary: Establish checkout\n---\n# Roadmap\n"
        ),
    );
    let spec_path = root.path().join(".specbind/specs/checkout/spec.yaml");
    let active = fs::read_to_string(&spec_path).expect("active Spec");
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nestablishment:\n  kind: reverse\n  source_revision: {baseline}\n  baseline_version: v2.4.0\n  milestone_id: {REVIEW_MILESTONE}\n{}",
            active
                .strip_prefix("schema_version: 1\n")
                .expect("schema prefix")
        ),
    );
    approve_requirements(root.path());

    let mut design = specbind_command();
    design
        .current_dir(root.path())
        .args([
            "spec",
            "design",
            "approve",
            "checkout",
            "--approval-mode",
            "delegated",
            "--delegation-workflow",
            "sb-discovery",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\n  State: adoption_ready\n"));

    let mut status = specbind_command();
    status
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\n  Next action: contract_review\n")
                .and(predicate::str::contains("tasks.yaml").not()),
        );
}

#[test]
fn reverse_finalize_archives_a_baseline_without_creating_a_release() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\nbaseline_version: v2.4.0\ntarget_release: null\nwork_items:\n  reverse_specs:\n    - spec: checkout\n      summary: Establish checkout\n---\n# Roadmap\n"
        ),
    );
    let active = fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml"))
        .expect("active Spec");
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nestablishment:\n  kind: reverse\n  source_revision: {baseline}\n  baseline_version: v2.4.0\n  milestone_id: {REVIEW_MILESTONE}\n{}",
            active
                .strip_prefix("schema_version: 1\n")
                .expect("schema prefix")
        ),
    );
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
        ".specbind/adoption/reverse-discovery.yaml",
        &format!(
            "schema_version: 1\nsource_revision: {baseline}\nsuspected_defects:\n  - locator: README.md:1\n    claim: Product name typo.\n    destination: .specbind/deferred.md\n"
        ),
    );
    write(
        root.path(),
        ".specbind/deferred.md",
        "---\ntype: Deferred Findings\n---\n\n# Deferred findings\n\n- Product name typo.\n",
    );
    commit_all(root.path());

    let mut finalize = specbind_command();
    finalize
        .current_dir(root.path())
        .args([
            "milestone",
            "reverse",
            "finalize",
            "--log-entries",
            "-",
        ])
        .write_stdin(
            r#"{"log_entries":[{"spec":"checkout","summary":"Established checkout from the existing product."}]}"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "OK ADOPTION_FINALIZED: Adopted baseline v2.4.0 across 1 specs; no product release was created.",
        ));

    let spec = fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml"))
        .expect("final Spec");
    assert!(spec.contains("establishment:"), "{spec}");
    assert!(spec.contains("active_change: null"), "{spec}");
    let log = fs::read_to_string(root.path().join(".specbind/specs/checkout/log.md"))
        .expect("baseline log");
    assert!(log.contains("**Baseline v2.4.0**"), "{log}");
    assert!(
        root.path()
            .join(".specbind/baselines/v2.4.0-roadmap.md")
            .is_file()
    );
    assert!(
        root.path()
            .join(".specbind/baselines/v2.4.0-contract-review.md")
            .is_file()
    );
    assert!(!root.path().join(".specbind/releases").exists());
    assert!(root.path().join(".specbind/deferred.md").is_file());
}

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

#[test]
fn invalidates_one_gate_and_clears_only_its_downstream_evidence() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());
    approve_design(root.path());
    commit_all(root.path());

    let mut invalidate = specbind_command();
    invalidate
        .current_dir(root.path())
        .args(["spec", "design", "invalidate", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK SPEC_DESIGN_INVALIDATED: Invalidated design for spec checkout.\n  State: design\n  Accepted review: unchanged\n",
        )
        .stderr("");

    let spec = fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml"))
        .expect("rewound spec metadata");
    assert!(spec.contains("state: design"), "{spec}");
    assert!(spec.contains("requirements:"), "{spec}");
    assert!(!spec.contains("design:"), "{spec}");

    let mut repeat = specbind_command();
    repeat
        .current_dir(root.path())
        .args(["spec", "design", "invalidate", "checkout"])
        .assert()
        .success()
        .stdout(
            "NO_CHANGE SPEC_DESIGN_NOT_APPROVED: Spec checkout has no design approval to invalidate.\n",
        );

    commit_all(root.path());
    let mut requirements = specbind_command();
    requirements
        .current_dir(root.path())
        .args(["spec", "requirements", "invalidate", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK SPEC_REQUIREMENTS_INVALIDATED: Invalidated requirements for spec checkout.\n  State: requirements\n  Accepted review: unchanged\n",
        );
    let rewound = fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml"))
        .expect("rewound spec metadata");
    assert!(rewound.contains("state: requirements"), "{rewound}");
    assert!(rewound.contains("requirement_ids: null"), "{rewound}");
    assert!(!rewound.contains("gate_evidence"), "{rewound}");
}

#[test]
fn removes_the_accepted_review_when_an_earlier_gate_rewinds() {
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
    let accepted = root.path().join(".specbind/state/contract-review.md");
    assert!(accepted.exists());
    commit_all(root.path());

    // A Tasks rewind happens after the review is accepted and must keep it.
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
        .success();
    commit_all(root.path());
    let mut tasks_rewind = specbind_command();
    tasks_rewind
        .current_dir(root.path())
        .args(["spec", "tasks", "invalidate", "checkout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\n  Accepted review: unchanged\n"));
    assert!(
        accepted.exists(),
        "a Tasks rewind must not remove the accepted review"
    );

    commit_all(root.path());
    let mut design_rewind = specbind_command();
    design_rewind
        .current_dir(root.path())
        .args(["spec", "design", "invalidate", "checkout"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\n  Accepted review: removed\n"));
    assert!(
        !accepted.exists(),
        "a Design rewind must remove the accepted review"
    );

    let mut status = specbind_command();
    status
        .current_dir(root.path())
        .args(["milestone", "review", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\n  Status: absent\n"));
}

#[test]
fn refuses_gate_invalidation_with_a_dirty_target() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());
    commit_all(root.path());
    approve_design(root.path());

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["spec", "design", "invalidate", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR SPEC_DESIGN_INVALIDATE_FAILED:").and(
                predicate::str::contains(
                    "SPEC_DESIGN_TARGET_DIRTY specs/checkout/spec.yaml: gate invalidation refuses to overwrite a dirty or staged spec.yaml",
                ),
            ),
        );
}
