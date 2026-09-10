use super::super::*;

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
