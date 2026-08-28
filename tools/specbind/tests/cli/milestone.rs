use super::*;

#[test]
fn creates_the_active_milestone_from_a_confirmed_scope() {
    let root = project_fixture();
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);

    let mut create = specbind_command();
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Add payments"}],"directChanges":[{"id":"docs","summary":"Update docs","dependsOn":[{"spec":"payments"}]}]},"body":"Overview\n\nDeliver payments.\n"}"#,
        )
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("OK MILESTONE_CREATED: Created milestone ")
                .and(predicate::str::contains(format!(
                    "\n  Baseline revision: {baseline}\n"
                )))
                .and(predicate::str::contains(
                    "\n  New specs: 1\n  Spec updates: 0\n  Direct changes: 1\n",
                )),
        )
        .stderr("");

    let roadmap = fs::read_to_string(root.path().join(".specbind/steering/roadmap.md"))
        .expect("created Roadmap");
    assert!(
        roadmap.starts_with("---\ntype: SpecBind Roadmap\n"),
        "{roadmap}"
    );
    assert!(roadmap.contains("\ntarget_release: null\n"), "{roadmap}");
    assert!(
        roadmap.contains(&format!("\nbaseline_revision: {baseline}\n")),
        "{roadmap}"
    );
    assert!(
        roadmap.ends_with("Overview\n\nDeliver payments.\n"),
        "{roadmap}"
    );

    let spec = fs::read_to_string(root.path().join(".specbind/specs/payments/spec.yaml"))
        .expect("initialized Spec metadata");
    assert!(spec.contains("state: requirements"), "{spec}");
    assert!(spec.contains("requirement_ids: null"), "{spec}");
    assert!(!spec.contains("gate_evidence"), "{spec}");

    let mut status = specbind_command();
    status
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Spec states: requirements=1\n")
                .and(predicate::str::contains("CONTRACT_REVIEW_MISSING").not())
                .and(predicate::str::contains("  Contract review: absent\n")),
        );
}

#[test]
fn refuses_creation_on_a_dirty_repository_or_conflicting_state() {
    let root = project_fixture();

    let mut dirty = specbind_command();
    dirty
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR MILESTONE_CREATE_FAILED: Cannot create the active milestone.",
            )
            .and(predicate::str::contains("MILESTONE_REPOSITORY_DIRTY")),
        );

    commit_all(root.path());
    let mut conflicting = specbind_command();
    conflicting
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"checkout","summary":"Add checkout"}]}}"#,
        )
        .assert()
        .failure()
        .stderr(predicate::str::contains("MILESTONE_SPEC_ALREADY_EXISTS"));

    let mut first = specbind_command();
    first
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .success();

    let mut second = specbind_command();
    second
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .failure()
        .stderr(predicate::str::contains("MILESTONE_ALREADY_ACTIVE"));
}

#[test]
fn rejects_invalid_scope_documents() {
    let root = project_fixture();
    commit_all(root.path());

    for (scope, code) in [
        ("{", "MILESTONE_SCOPE_INVALID"),
        (
            r#"{"schemaVersion":2,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]}}"#,
            "MILESTONE_SCOPE_INVALID",
        ),
        (
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs","milestoneId":"x"}]}}"#,
            "MILESTONE_SCOPE_INVALID",
        ),
        (
            r#"{"schemaVersion":1,"workItems":{}}"#,
            "ROADMAP_WORK_ITEMS_EMPTY",
        ),
        (
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs","dependsOn":[{"direct":"missing"}]}]}}"#,
            "ROADMAP_DEPENDENCY_MISSING",
        ),
        (
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs","dependsOn":[{"direct":"docs"}]}]}}"#,
            "ROADMAP_DEPENDENCY_SELF",
        ),
    ] {
        let mut command = specbind_command();
        command
            .current_dir(root.path())
            .args(["milestone", "create", "--scope", "-"])
            .write_stdin(scope)
            .assert()
            .failure()
            .stdout("")
            .stderr(predicate::str::contains(code));
    }

    assert!(
        !root.path().join(".specbind/steering/roadmap.md").exists(),
        "a rejected scope must not create an active Roadmap"
    );
}

#[test]
fn updates_scope_while_preserving_body_and_completed_direct_state() {
    let root = project_fixture();
    commit_all(root.path());
    create_direct_milestone(root.path());
    commit_all(root.path());
    let revision = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    let mut complete = specbind_command();
    complete
        .current_dir(root.path())
        .args([
            "milestone",
            "direct",
            "complete",
            "docs",
            "--implementation-revision",
            &revision,
        ])
        .assert()
        .success();
    commit_all(root.path());

    let mut unchanged = specbind_command();
    unchanged
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_SCOPE_UNCHANGED:",
        ));

    let mut update = specbind_command();
    update
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Add payments"}],"directChanges":[{"id":"docs","summary":"Update docs"}]}}"#,
        )
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\n  New specs: 1\n  Spec updates: 0\n  Direct changes: 1\n")
                .and(predicate::str::contains("\n  Accepted review: unchanged\n")),
        );

    let roadmap = fs::read_to_string(root.path().join(".specbind/steering/roadmap.md"))
        .expect("updated Roadmap");
    assert!(roadmap.contains("status: completed"), "{roadmap}");
    assert!(roadmap.ends_with("Overview\n\nDirect only.\n"), "{roadmap}");
    assert!(
        root.path()
            .join(".specbind/specs/payments/spec.yaml")
            .exists(),
        "an added Spec must be initialized"
    );
}

#[test]
fn blocks_scope_removal_that_needs_reconciliation() {
    let root = project_fixture();
    commit_all(root.path());
    create_direct_milestone(root.path());
    commit_all(root.path());
    let revision = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    let mut complete = specbind_command();
    complete
        .current_dir(root.path())
        .args([
            "milestone",
            "direct",
            "complete",
            "docs",
            "--implementation-revision",
            &revision,
        ])
        .assert()
        .success();
    commit_all(root.path());

    let mut drop_direct = specbind_command();
    drop_direct
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Add payments"}]}}"#,
        )
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR MILESTONE_SCOPE_UPDATE_FAILED: Cannot update the milestone scope.",
            )
            .and(predicate::str::contains(
                "MILESTONE_SCOPE_DIRECT_REMOVAL_BLOCKED",
            )),
        );
}

#[test]
fn removes_the_accepted_review_only_for_spec_backed_scope_changes() {
    let root = project_fixture();
    commit_all(root.path());
    let mut create = specbind_command();
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Add payments"}],"directChanges":[{"id":"docs","summary":"Update docs"}]}}"#,
        )
        .assert()
        .success();
    write(
        root.path(),
        ".specbind/state/contract-review.md",
        "---\ntype: SpecBind Contract Review\n---\nAccepted.\n",
    );
    commit_all(root.path());
    let review = root.path().join(".specbind/state/contract-review.md");

    let mut direct_only = specbind_command();
    direct_only
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Add payments"}],"directChanges":[{"id":"docs","summary":"Refresh docs"}]}}"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("\n  Accepted review: unchanged\n"));
    assert!(review.exists(), "a Direct-only change must keep the review");

    commit_all(root.path());
    let mut spec_backed = specbind_command();
    spec_backed
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Deliver payments"}],"directChanges":[{"id":"docs","summary":"Refresh docs"}]}}"#,
        )
        .assert()
        .success()
        .stdout(predicate::str::contains("\n  Accepted review: removed\n"));
    assert!(
        !review.exists(),
        "a Spec-backed change must remove the review"
    );
}

#[test]
fn rebaselines_only_onto_an_explicit_ancestor_revision() {
    let root = project_fixture();
    commit_all(root.path());
    let first = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write(root.path(), "second.txt", "second\n");
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    create_direct_milestone(root.path());
    write(
        root.path(),
        ".specbind/state/contract-review.md",
        "---\ntype: SpecBind Contract Review\n---\nAccepted.\n",
    );
    commit_all(root.path());

    let mut invalid = specbind_command();
    invalid
        .current_dir(root.path())
        .args(["milestone", "rebaseline", "--revision", "HEAD~1"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR MILESTONE_REBASELINE_FAILED: Cannot rebaseline the active milestone.",
            )
            .and(predicate::str::contains(
                "MILESTONE_BASELINE_REVISION_INVALID",
            )),
        );

    let mut unchanged = specbind_command();
    unchanged
        .current_dir(root.path())
        .args(["milestone", "rebaseline", "--revision", &baseline])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_BASELINE_UNCHANGED:",
        ));

    let mut rebaseline = specbind_command();
    rebaseline
        .current_dir(root.path())
        .args(["milestone", "rebaseline", "--revision", &first])
        .assert()
        .success()
        .stdout(
            predicate::str::contains(format!("\n  Baseline revision: {first}\n"))
                .and(predicate::str::contains("\n  Accepted review: removed\n")),
        );

    let roadmap = fs::read_to_string(root.path().join(".specbind/steering/roadmap.md"))
        .expect("rebaselined Roadmap");
    assert!(
        roadmap.contains(&format!("baseline_revision: {first}")),
        "{roadmap}"
    );
    assert!(roadmap.ends_with("Overview\n\nDirect only.\n"), "{roadmap}");
}
