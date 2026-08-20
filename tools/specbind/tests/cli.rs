use assert_cmd::Command;
use predicates::prelude::*;
use specbind::artifacts::resolve_gate_inputs;
use std::{fmt::Write as _, fs, path::Path, process::Command as ProcessCommand};
use tempfile::TempDir;

#[test]
fn reports_help() {
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");

    command
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "Bind durable specifications to agent-assisted software delivery.",
        ));
}

#[test]
fn reports_version() {
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");

    command
        .arg("--version")
        .assert()
        .success()
        .stdout("specbind 1.0.0\n");
}

#[test]
fn lists_discovered_artifacts_with_the_text_result_contract() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/brief.md",
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\n# Research\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let content = "---\ntype: SpecBind Brief\n---\n# Checkout brief\n";
    write(root.path(), ".specbind/specs/checkout/brief.md", content);

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["artifact", "read", "checkout", "brief"])
        .assert()
        .success()
        .stdout(content)
        .stderr("");
}

#[test]
fn reads_a_valid_selector_despite_unrelated_inventory_diagnostics() {
    let root = project_fixture();
    let content = "---\ntype: SpecBind Brief\n---\n# Checkout brief\n";
    write(root.path(), ".specbind/specs/checkout/brief.md", content);
    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut unsafe_command = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["tasks", "list", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK TASKS_LISTED: Listed 4 task(s) for spec checkout (1 completed, 2 pending, 1 blocked).\n  [partial 1/2; 0 blocked] 1 Build\n    [completed] 1.1 First\n    [pending actionable] 1.2 Second\n  [blocked] 2 Integrate\n  [pending actionable] 3 Document\n",
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

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["tasks", "show", "checkout", "3"])
        .assert()
        .success()
        .stdout(
            "OK TASK_SHOWN: Found task 3 in spec checkout.\n  Status: pending actionable\n  Title: Document\n  Group: none\n  Details: none\n  Requirement IDs: 1.4\n  Boundaries: docs/\n  Contracts: none\n  Explicit prerequisites: 1.1\n  Effective prerequisites: 1.1\n  Blocker: none\n  Completion criteria: none\n",
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

    let mut invalid = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut missing = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut out_of_order = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut first = Command::cargo_bin("specbind").expect("specbind binary should build");
    first
        .current_dir(root.path())
        .args(["tasks", "complete", "checkout", "1"])
        .assert()
        .success()
        .stdout(
            "OK TASK_COMPLETED: Completed task 1 in spec checkout.\n  Progress: 1/2 completed, 1 pending, 0 blocked\n  Next actionable: 2\n",
        )
        .stderr("");

    let mut repeat = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut invalid = Command::cargo_bin("specbind").expect("specbind binary should build");
    invalid
        .current_dir(root.path())
        .args(["tasks", "block", "checkout", "2", "--reason", "   "])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("TASK_BLOCKED_REASON_INVALID"));

    let mut block = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut same = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut reopen = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut absent = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut group = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut idle = Command::cargo_bin("specbind").expect("specbind binary should build");
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
fn write_progress_fixture(root: &Path) {
    write(
        root,
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: implementation\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: sha256:0000000000000000000000000000000000000000000000000000000000000000\n"
        ),
    );
    write(
        root,
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: First\n      requirement_ids: ['1.1']\n    - id: '2'\n      kind: task\n      title: Second\n      requirement_ids: ['1.1']\n",
    );
}

#[test]
fn reports_composed_spec_status_with_freshness_coverage_and_progress() {
    let root = project_fixture();
    write_status_fixture(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK SPEC_STATUS_REPORTED: Reported status for spec checkout.\n  State: implementation\n  Milestone: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  Health: consistent\n  Gates: requirements=fresh, design=fresh, tasks=fresh, completion=not_reached\n  Delegated gates: none\n  Task progress: 2 total, 1 completed, 0 pending, 1 blocked\n  Next actionable: none\n  Blockers:\n    - 2: Waiting for review\n  Requirement coverage: design 1/1, tasks 1/1 (required)\n  Diagnostics: none\n",
        )
        .stderr("");
}

#[test]
fn reports_a_clean_idle_spec_without_requiring_active_artifacts() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK SPEC_STATUS_REPORTED: Reported status for spec checkout.\n  State: idle\n  Milestone: none\n  Health: consistent\n  Gates: requirements=not_reached, design=not_reached, tasks=not_reached, completion=not_reached\n  Task progress: unavailable\n  Next actionable: none\n  Blockers: none\n  Requirement coverage: inactive\n  Diagnostics: none\n",
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\nRetained draft\n",
    );
    let mut inconsistent = Command::cargo_bin("specbind").expect("specbind binary should build");
    inconsistent
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Health: inconsistent\n")
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

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State: design\n")
                .and(predicate::str::contains("  Health: inconsistent\n"))
                .and(predicate::str::contains("SPEC_STATE_GATE_EVIDENCE")),
        )
        .stderr("");
}

#[test]
fn fails_status_when_spec_metadata_is_not_structurally_readable() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: invalid\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with("ERROR SPEC_STATUS_FAILED:"));
}

#[test]
fn reports_no_active_milestone_as_no_change() {
    let root = project_fixture();

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout("NO_CHANGE NO_ACTIVE_MILESTONE: No active milestone exists.\n")
        .stderr("");
}

#[test]
fn reports_direct_milestone_dependencies_and_actionable_work() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        "---\ntype: SpecBind Roadmap\nmilestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n    - id: publish\n      summary: Publish site\n      depends_on:\n        - direct: docs\n---\n# Roadmap\n",
    );
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: implementation\n")
                .and(predicate::str::contains("  Health: consistent\n"))
                .and(predicate::str::contains(
                    "  Contract review: not_applicable\n",
                ))
                .and(predicate::str::contains(
                    "  Direct progress: 0/2 completed\n",
                ))
                .and(predicate::str::contains(
                    "direct:publish status=pending waiting_for=direct:docs",
                ))
                .and(predicate::str::contains(
                    "direct:docs action=implementation",
                )),
        )
        .stderr("");
}

#[test]
fn exposes_the_direct_completion_handshake_with_stable_results() {
    let root = project_fixture();
    write(root.path(), "baseline.txt", "baseline\n");
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n# Roadmap\n"
        ),
    );
    commit_all(root.path());
    let revision = git_stdout(root.path(), &["rev-parse", "HEAD"]);

    let mut preflight = Command::cargo_bin("specbind").expect("specbind binary should build");
    preflight
        .current_dir(root.path())
        .args(["milestone", "direct", "preflight", "docs"])
        .assert()
        .success()
        .stdout(format!(
            "OK DIRECT_COMPLETION_PREFLIGHT_READY: Direct item docs is ready for completion validation.\n  Implementation revision: {revision}\n"
        ));

    let mut complete = Command::cargo_bin("specbind").expect("specbind binary should build");
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
        .success()
        .stdout("OK DIRECT_COMPLETION_RECORDED: Recorded completion for Direct item docs.\n");

    let mut retry = Command::cargo_bin("specbind").expect("specbind binary should build");
    retry
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
        .success()
        .stdout(
            "NO_CHANGE DIRECT_COMPLETION_ALREADY_RECORDED: Direct item docs is already completed.\n",
        );
}

#[test]
fn binds_and_explicitly_rebinds_a_milestone_release() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        "---\ntype: SpecBind Roadmap\nmilestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n# Roadmap\n",
    );
    commit_all(root.path());

    let mut invalid = Command::cargo_bin("specbind").expect("specbind binary should build");
    invalid
        .current_dir(root.path())
        .args(["milestone", "bind-release", "bad/version"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR INVALID_RELEASE_VERSION: Cannot bind milestone release.",
        ));

    let mut bind = Command::cargo_bin("specbind").expect("specbind binary should build");
    bind.current_dir(root.path())
        .args(["milestone", "bind-release", "v1.4.0"])
        .assert()
        .success()
        .stdout(
            "OK RELEASE_BOUND: Bound milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62 to release v1.4.0.\n  Roadmap archive: releases/v1.4.0-roadmap.md\n  Contract review archive: releases/v1.4.0-contract-review.md\n",
        )
        .stderr("");

    let mut retry = Command::cargo_bin("specbind").expect("specbind binary should build");
    retry
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.4.0"])
        .assert()
        .success()
        .stdout("NO_CHANGE RELEASE_ALREADY_BOUND: Milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62 is already bound to release v1.4.0.\n")
        .stderr("");

    let mut confirmation = Command::cargo_bin("specbind").expect("specbind binary should build");
    confirmation
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.5.0"])
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "ERROR RELEASE_REBIND_REQUIRED: Cannot bind milestone release.",
        ));

    let mut dirty = Command::cargo_bin("specbind").expect("specbind binary should build");
    dirty
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.5.0", "--rebind"])
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "ERROR MILESTONE_ROADMAP_DIRTY: Cannot bind milestone release.",
        ));

    commit_all(root.path());
    write(
        root.path(),
        ".specbind/releases/V1.5.0-ROADMAP.MD",
        "occupied\n",
    );
    let mut collision = Command::cargo_bin("specbind").expect("specbind binary should build");
    collision
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.5.0", "--rebind"])
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "ERROR RELEASE_ARCHIVE_COLLISION: Cannot bind milestone release.",
        ));

    let mut rebind = Command::cargo_bin("specbind").expect("specbind binary should build");
    rebind
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.6.0", "--rebind"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "OK RELEASE_REBOUND: Rebound milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62 from release v1.4.0 to v1.6.0.",
        ));
    let roadmap = fs::read_to_string(root.path().join(".specbind/steering/roadmap.md"))
        .expect("read rebound Roadmap");
    assert!(roadmap.ends_with("# Roadmap\n"));
    assert!(roadmap.contains("target_release: v1.6.0"));
}

#[test]
fn reads_spec_completion_evidence_from_explicit_stdin() {
    let root = project_fixture();
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");

    command
        .current_dir(root.path())
        .args([
            "spec",
            "completion",
            "accept",
            "checkout",
            "--evidence",
            "-",
        ])
        .write_stdin("{}")
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR COMPLETION_EVIDENCE_INVALID: Cannot accept Spec completion evidence.",
        ));
}

#[test]
fn reports_tasks_authored_before_the_required_milestone_review() {
    let root = project_fixture();
    write_status_fixture(root.path());
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        "---\ntype: SpecBind Roadmap\nmilestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: contract_review\n")
                .and(predicate::str::contains("  Health: inconsistent\n"))
                .and(predicate::str::contains(
                    "  Spec states: implementation=1\n",
                ))
                .and(predicate::str::contains("MILESTONE_TASKS_BEFORE_REVIEW")),
        );
}

#[test]
fn accepts_a_stdin_review_candidate_and_reports_fresh_status() {
    let root = project_fixture();
    write_review_fixture(root.path());

    let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
    accept
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible."))
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!(
                "OK MILESTONE_REVIEW_ACCEPTED: Accepted contract review for milestone {REVIEW_MILESTONE}.\n  Passed at: "
            ))
            .and(predicate::str::contains("\n  Inputs: 2\n")),
        )
        .stderr("");

    let mut status = Command::cargo_bin("specbind").expect("specbind binary should build");
    status
        .current_dir(root.path())
        .args(["milestone", "review", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!(
                "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported contract review status for milestone {REVIEW_MILESTONE}.\n  Status: fresh\n  Passed at: "
            ))
            .and(predicate::str::contains("\n  Inputs: 2\n"))
            .and(predicate::str::contains("Compatible.").not())
            .and(predicate::str::contains("sha256:").not())
            .and(predicate::str::contains("Diagnostics").not()),
        )
        .stderr("");
}

#[test]
fn accepts_a_repository_external_candidate_file() {
    let root = project_fixture();
    write_review_fixture(root.path());
    let outside = tempfile::tempdir().expect("temporary external directory");
    let candidate = outside.path().join("candidate.json");
    fs::write(&candidate, review_candidate("Externally reviewed.")).expect("write candidate");

    let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
    accept
        .current_dir(root.path())
        .args([
            "milestone",
            "review",
            "accept",
            "--candidate",
            candidate.to_str().expect("UTF-8 candidate path"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "OK MILESTONE_REVIEW_ACCEPTED: Accepted contract review for milestone",
        ))
        .stderr("");

    let accepted = fs::read_to_string(root.path().join(".specbind/state/contract-review.md"))
        .expect("accepted review content");
    assert!(accepted.ends_with("Externally reviewed.\n"));
}

#[test]
fn reports_not_applicable_and_absent_review_state_as_success() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: 0123456789abcdef0123456789abcdef01234567\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n# Roadmap\n"
        ),
    );

    let mut direct_only = Command::cargo_bin("specbind").expect("specbind binary should build");
    direct_only
        .current_dir(root.path())
        .args(["milestone", "review", "status"])
        .assert()
        .success()
        .stdout(format!(
            "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported contract review status for milestone {REVIEW_MILESTONE}.\n  Status: not_applicable\n"
        ))
        .stderr("");

    write_review_fixture(root.path());
    let mut absent = Command::cargo_bin("specbind").expect("specbind binary should build");
    absent
        .current_dir(root.path())
        .args(["milestone", "review", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(format!(
                "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported contract review status for milestone {REVIEW_MILESTONE}.\n  Status: absent\n"
            ))
            .and(predicate::str::contains("Passed at:").not())
            .and(predicate::str::contains("\n  Inputs:").not())
            .and(predicate::str::contains("CONTRACT_REVIEW_MISSING")),
        )
        .stderr("");
}

#[test]
fn reports_stale_review_state_as_success_and_invalid_state_as_failure() {
    let root = project_fixture();
    write_review_fixture(root.path());
    let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
    accept
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible."))
        .assert()
        .success();

    write(
        root.path(),
        ".specbind/specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    let mut stale = Command::cargo_bin("specbind").expect("specbind binary should build");
    stale
        .current_dir(root.path())
        .args(["milestone", "review", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\n  Status: stale\n")
                .and(predicate::str::contains("\n  Passed at: "))
                .and(predicate::str::contains(
                    "    - CONTRACT_REVIEW_INPUTS_STALE",
                )),
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/state/contract-review.md",
        &format!(
            "---\ntype: Wrong Type\nmilestone_id: {REVIEW_MILESTONE}\npassed_at: yesterday\ninput_revisions:\n  steering/roadmap.md#cross-spec-scope: SHA256:BAD\n---\n"
        ),
    );
    let mut invalid = Command::cargo_bin("specbind").expect("specbind binary should build");
    invalid
        .current_dir(root.path())
        .args(["milestone", "review", "status"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR MILESTONE_REVIEW_STATUS_FAILED: Cannot report the contract review status.",
            )
            .and(predicate::str::contains("CONTRACT_REVIEW_TYPE_INVALID")),
        );
}

#[test]
fn rejects_unsafe_and_unreadable_review_candidates() {
    let root = project_fixture();
    write_review_fixture(root.path());
    let outside = tempfile::tempdir().expect("temporary external directory");

    write(root.path(), "candidate.json", &review_candidate("Inside."));
    let mut internal = Command::cargo_bin("specbind").expect("specbind binary should build");
    internal
        .current_dir(root.path())
        .args([
            "milestone",
            "review",
            "accept",
            "--candidate",
            "candidate.json",
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR MILESTONE_REVIEW_ACCEPT_FAILED: Cannot accept the contract review.",
            )
            .and(predicate::str::contains(
                "MILESTONE_REVIEW_CANDIDATE_TARGET_INVALID Review candidate file must be outside the project worktree.",
            )),
        );

    let directory = outside.path().join("candidate-directory");
    fs::create_dir(&directory).expect("create candidate directory");
    let mut not_a_file = Command::cargo_bin("specbind").expect("specbind binary should build");
    not_a_file
        .current_dir(root.path())
        .args([
            "milestone",
            "review",
            "accept",
            "--candidate",
            directory.to_str().expect("UTF-8 candidate path"),
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "MILESTONE_REVIEW_CANDIDATE_TARGET_INVALID Review candidate must be a regular non-symlink file.",
        ));

    let not_utf8 = outside.path().join("candidate.bin");
    fs::write(&not_utf8, [0x7b, 0xff, 0xfe, 0x7d]).expect("write non-UTF-8 candidate");
    let mut invalid_encoding =
        Command::cargo_bin("specbind").expect("specbind binary should build");
    invalid_encoding
        .current_dir(root.path())
        .args([
            "milestone",
            "review",
            "accept",
            "--candidate",
            not_utf8.to_str().expect("UTF-8 candidate path"),
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "MILESTONE_REVIEW_CANDIDATE_READ_FAILED",
        ));

    let mut invalid_json = Command::cargo_bin("specbind").expect("specbind binary should build");
    invalid_json
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin("{\"schemaVersion\":1}")
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR MILESTONE_REVIEW_ACCEPT_FAILED: Cannot accept the contract review.",
            )
            .and(predicate::str::contains("CONTRACT_REVIEW_CANDIDATE_")),
        );

    assert!(
        !root
            .path()
            .join(".specbind/state/contract-review.md")
            .exists(),
        "rejected candidates must not create accepted state"
    );
}

#[test]
fn keeps_the_accepted_review_unchanged_when_a_guard_fails() {
    let root = project_fixture();
    write_review_fixture(root.path());
    let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
    accept
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Originally reviewed."))
        .assert()
        .success();
    let path = root.path().join(".specbind/state/contract-review.md");
    let original = fs::read_to_string(&path).expect("accepted review content");

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        task_fixture(),
    );
    let mut blocked = Command::cargo_bin("specbind").expect("specbind binary should build");
    blocked
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Replacement assessment."))
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR MILESTONE_REVIEW_ACCEPT_FAILED: Cannot accept the contract review.",
            )
            .and(predicate::str::contains(
                "CONTRACT_REVIEW_TASKS_ALREADY_EXIST",
            )),
        );

    assert_eq!(
        fs::read_to_string(&path).expect("preserved review content"),
        original
    );
}

#[test]
fn reaccepts_a_currently_fresh_review() {
    let root = project_fixture();
    write_review_fixture(root.path());
    for assessment in ["First assessment.", "Revised assessment."] {
        let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
        accept
            .current_dir(root.path())
            .args(["milestone", "review", "accept", "--candidate", "-"])
            .write_stdin(review_candidate(assessment))
            .assert()
            .success()
            .stdout(predicate::str::starts_with(
                "OK MILESTONE_REVIEW_ACCEPTED: Accepted contract review for milestone",
            ));
    }

    let accepted = fs::read_to_string(root.path().join(".specbind/state/contract-review.md"))
        .expect("accepted review content");
    assert!(accepted.ends_with("Revised assessment.\n"));
    assert!(!accepted.contains("First assessment."));
}

#[test]
fn plans_an_initial_installation_without_writing() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args([
            "install",
            "--dry-run",
            "--agent",
            "codex",
            "--agent",
            "claude-code",
            "--language",
            "ja",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK INSTALL_PLANNED: Planned 45 action(s) for 2 agent(s).\n",
            )
            .and(predicate::str::contains("\n  Mode: initial\n"))
            .and(predicate::str::contains("\n  Language: ja\n"))
            .and(predicate::str::contains("\n  Agents: claude-code, codex\n"))
            .and(predicate::str::contains(
                "\n  Project instructions: disabled\n",
            ))
            .and(predicate::str::contains(
                "- create .specbind.json [config]\n",
            ))
            .and(predicate::str::contains(
                "- create .specbind/settings/templates/specs/requirements.md [template]\n",
            ))
            .and(predicate::str::contains(
                "\n  Summary: 45 create, 0 replace, 0 keep\n",
            )),
        )
        .stderr("");

    assert!(
        !root.path().join(".specbind.json").exists(),
        "a dry run must not write the configuration"
    );
    assert!(
        !root.path().join(".specbind").exists(),
        "a dry run must not create the spec root"
    );
}

#[test]
fn requires_explicit_inputs_for_an_initial_installation() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut missing_language =
        Command::cargo_bin("specbind").expect("specbind binary should build");
    missing_language
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "codex"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR INSTALL_PLAN_FAILED: Cannot plan the SpecBind installation.",
            )
            .and(predicate::str::contains("INSTALL_LANGUAGE_REQUIRED")),
        );

    let mut missing_agent = Command::cargo_bin("specbind").expect("specbind binary should build");
    missing_agent
        .current_dir(root.path())
        .args(["install", "--dry-run", "--language", "en"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_AGENT_REQUIRED"));
}

#[test]
fn keeps_project_owned_settings_and_guards_replacements() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/templates/specs/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\n---\n# Project design scaffold\n",
    );

    let mut unchanged = Command::cargo_bin("specbind").expect("specbind binary should build");
    unchanged
        .current_dir(root.path())
        .args(["install", "--dry-run"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\n  Mode: refresh\n")
                .and(predicate::str::contains(
                    "- keep .specbind.json [config] (already matches the requested inputs)\n",
                ))
                .and(predicate::str::contains(
                    "- keep .specbind/settings/templates/specs/design.md [template] (project-owned settings are never overwritten)\n",
                ))
                .and(predicate::str::contains(
                    "\n  Summary: 26 create, 0 replace, 2 keep\n",
                )),
        );

    let mut dirty = Command::cargo_bin("specbind").expect("specbind binary should build");
    dirty
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "claude-code"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_COMMIT_REQUIRED"));

    commit_all(root.path());
    let mut replaceable = Command::cargo_bin("specbind").expect("specbind binary should build");
    replaceable
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "claude-code"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("- replace .specbind.json [config]\n")
                .and(predicate::str::contains("\n  Agents: claude-code, codex\n")),
        );

    write(root.path(), "dirty.txt", "dirty\n");
    let mut blocked = Command::cargo_bin("specbind").expect("specbind binary should build");
    blocked
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "claude-code"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_REPOSITORY_DIRTY"));
}

#[test]
fn applies_an_initial_installation_and_is_idempotent() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = Command::cargo_bin("specbind").expect("specbind binary should build");
    apply
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "en"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK INSTALL_APPLIED: Applied 28 action(s) for 1 agent(s).\n",
            )
            .and(predicate::str::contains(
                "\n  Summary: 28 created, 0 replaced, 0 kept\n",
            )),
        )
        .stderr("");

    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("written config");
    assert_eq!(
        config,
        "{\n  \"schemaVersion\": 1,\n  \"specDir\": \".specbind\",\n  \"language\": \"en\",\n  \"agents\": [\"codex\"]\n}\n"
    );
    for relative in [
        ".specbind/settings/templates/specs/requirements.md",
        ".specbind/settings/templates/specs/design.md",
        ".specbind/settings/rules/ears-format.md",
        ".specbind/settings/rules/steering-principles.md",
    ] {
        assert!(root.path().join(relative).is_file(), "missing {relative}");
    }

    let mut installed = Command::cargo_bin("specbind").expect("specbind binary should build");
    installed
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "selector=requirements source=project",
        ));

    let mut again = Command::cargo_bin("specbind").expect("specbind binary should build");
    again
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("NO_CHANGE INSTALL_UP_TO_DATE:"))
        .stderr("");
}

#[test]
fn installs_product_managed_skills_for_each_selected_agent() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = Command::cargo_bin("specbind").expect("specbind binary should build");
    apply
        .current_dir(root.path())
        .args([
            "install",
            "--agent",
            "codex",
            "--agent",
            "claude-code",
            "--language",
            "en",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("- create .claude/skills/specbind-status/SKILL.md [skill]\n")
                .and(predicate::str::contains(
                    "- create .agents/skills/specbind-status/SKILL.md [skill]\n",
                )),
        );

    let claude = fs::read_to_string(root.path().join(".claude/skills/specbind-status/SKILL.md"))
        .expect("rendered Claude Code skill");
    let codex = fs::read_to_string(root.path().join(".agents/skills/specbind-status/SKILL.md"))
        .expect("rendered Codex skill");
    assert!(
        claude.starts_with("---\nname: specbind-status\n"),
        "{claude}"
    );
    assert!(claude.contains("argument-hint:"), "{claude}");
    assert!(!codex.contains("argument-hint:"), "{codex}");
    for forbidden in ["allowed-tools", "disable-model-invocation"] {
        assert!(!claude.contains(forbidden), "{claude}");
        assert!(!codex.contains(forbidden), "{codex}");
    }
    let body = |rendered: &str| {
        rendered
            .split_once("\n---\n")
            .and_then(|(_, rest)| rest.split_once("\n---\n"))
            .map_or_else(|| rendered.to_owned(), |(_, body)| body.to_owned())
    };
    assert_eq!(
        claude.rsplit_once("\n---\n").expect("body").1,
        codex.rsplit_once("\n---\n").expect("body").1,
        "both agents receive the same body"
    );
    let _ = body;

    // A local edit is not a customization path, and the repository guard refuses
    // to overwrite it while it is uncommitted.
    write(
        root.path(),
        ".agents/skills/specbind-status/SKILL.md",
        "---\nname: specbind-status\ndescription: edited\n---\n# Local\n",
    );
    let mut refresh = Command::cargo_bin("specbind").expect("specbind binary should build");
    refresh
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_COMMIT_REQUIRED"));

    commit_all(root.path());
    let mut restored = Command::cargo_bin("specbind").expect("specbind binary should build");
    restored
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "- replace .agents/skills/specbind-status/SKILL.md [skill]\n",
        ));
    assert_eq!(
        fs::read_to_string(root.path().join(".agents/skills/specbind-status/SKILL.md"))
            .expect("refreshed skill"),
        codex,
        "a refresh restores the product asset"
    );
}

#[test]
fn never_overwrites_project_owned_settings_when_applying() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    write(
        root.path(),
        ".specbind/settings/rules/ears-format.md",
        "---\ntype: SpecBind Rule\n---\n# Project owned\n",
    );

    let mut apply = Command::cargo_bin("specbind").expect("specbind binary should build");
    apply
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "ja"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\n  Summary: 27 created, 0 replaced, 1 kept\n",
        ));

    assert_eq!(
        fs::read_to_string(root.path().join(".specbind/settings/rules/ears-format.md"))
            .expect("preserved rule"),
        "---\ntype: SpecBind Rule\n---\n# Project owned\n"
    );
    let template = fs::read_to_string(
        root.path()
            .join(".specbind/settings/templates/specs/requirements.md"),
    )
    .expect("installed template");
    assert!(
        template.contains("requirement: 要件"),
        "the configured language must select the installed template"
    );
}

#[test]
fn guards_a_configuration_replacement_when_applying() {
    let root = project_fixture();

    let mut dirty = Command::cargo_bin("specbind").expect("specbind binary should build");
    dirty
        .current_dir(root.path())
        .args(["install", "--agent", "claude-code"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR INSTALL_FAILED: Cannot apply the SpecBind installation.",
            )
            .and(predicate::str::contains("INSTALL_COMMIT_REQUIRED")),
        );

    commit_all(root.path());
    let mut allowed = Command::cargo_bin("specbind").expect("specbind binary should build");
    allowed
        .current_dir(root.path())
        .args(["install", "--agent", "claude-code"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "- replace .specbind.json [config]",
        ));

    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("rewritten config");
    assert!(
        config.contains("\"agents\": [\"claude-code\", \"codex\"]"),
        "{config}"
    );
}

#[test]
fn reads_embedded_protocols_without_a_project() {
    let outside = tempfile::tempdir().expect("directory without a SpecBind project");

    let mut list = Command::cargo_bin("specbind").expect("specbind binary should build");
    list.current_dir(outside.path())
        .args(["protocol", "list"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("OK PROTOCOL_LISTED: Found ").and(
                predicate::str::contains("selector=okf-authoring purpose=\""),
            ),
        )
        .stderr("");

    let mut read = Command::cargo_bin("specbind").expect("specbind binary should build");
    read.current_dir(outside.path())
        .args(["protocol", "read", "okf-authoring"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("# OKF authoring protocol\n")
                .and(predicate::str::contains("Open Knowledge Format v0.2"))
                .and(predicate::str::contains("OK PROTOCOL").not()),
        )
        .stderr("");

    let mut unknown = Command::cargo_bin("specbind").expect("specbind binary should build");
    unknown
        .current_dir(outside.path())
        .args(["protocol", "read", "absent-protocol"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR PROTOCOL_SELECTOR_NOT_FOUND:")
                .and(predicate::str::contains("available selector okf-authoring")),
        );
}

#[test]
fn verifies_traceability_and_fails_closed_on_missing_coverage() {
    let root = project_fixture();
    write_status_fixture(root.path());

    let mut pass = Command::cargo_bin("specbind").expect("specbind binary should build");
    pass.current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK TRACEABILITY_VERIFIED: Verified traceability for spec checkout.\n  Requirements: 1\n  Active requirement IDs: 1\n  Design coverage: 1/1\n  Task coverage: 1/1 (required)\n",
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['9.9']\n",
    );
    let mut fail = Command::cargo_bin("specbind").expect("specbind binary should build");
    fail.current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR TRACEABILITY_FAILED: Traceability for spec checkout has diagnostics.",
            )
            .and(predicate::str::contains(
                "TRACEABILITY_TASK_COVERAGE_MISSING",
            ))
            .and(predicate::str::contains(
                "TRACEABILITY_TASK_REQUIREMENT_UNKNOWN",
            )),
        );
}

#[test]
fn reports_an_idle_spec_without_active_coverage_ratios() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .success()
        .stdout(
            "OK TRACEABILITY_VERIFIED: Verified traceability for spec checkout.\n  Requirements: 1\n  Active requirement IDs: none\n",
        )
        .stderr("");
}

#[test]
fn verifies_the_contract_graph_and_keeps_review_warnings_non_fatal() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root.path(),
        ".specbind/specs/provider/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n- `value` — Value.\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n\n- `shared-tree` — `src/shared/**`\n",
    );
    write(
        root.path(),
        ".specbind/specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `value` → `provider/exports/value`\n\n## Invariants\n\n## File Ownership\n\n- `shared-tree` — `src/shared/**`\n",
    );

    let mut warned = Command::cargo_bin("specbind").expect("specbind binary should build");
    warned
        .current_dir(root.path())
        .args(["check", "contracts"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK CONTRACTS_VERIFIED: Verified 3 contract(s) and 1 dependency reference(s).\n",
            )
            .and(predicate::str::contains("\n  Dependency cycles: 0\n"))
            .and(predicate::str::contains("\n  Warnings:\n")),
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/specs/consumer/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n- `missing` → `provider/exports/missing`\n\n## Invariants\n\n## File Ownership\n",
    );
    let mut failed = Command::cargo_bin("specbind").expect("specbind binary should build");
    failed
        .current_dir(root.path())
        .args(["check", "contracts"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR CONTRACTS_FAILED: Contract graph has structural diagnostics.",
            )
            .and(predicate::str::contains(
                "CONTRACT_GRAPH_TARGET_ENTRY_MISSING",
            )),
        );
}

#[test]
fn lists_and_reads_project_owned_spec_templates() {
    let root = project_fixture();
    write_template_fixture(root.path());

    let mut list = Command::cargo_bin("specbind").expect("specbind binary should build");
    list.current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("OK TEMPLATE_LISTED: Found 6 recognized spec template(s).\n")
                .and(predicate::str::contains(
                    "selector=brief source=project type=\"SpecBind Brief\" template_path=settings/templates/specs/brief.md output_path=brief.md\n",
                ))
                .and(predicate::str::contains(
                    "selector=design/main source=project type=\"SpecBind Design\" artifact_id=main template_path=settings/templates/specs/technical-design/main.md output_path=technical-design/main.md\n",
                ))
                .and(predicate::str::contains(
                    "selector=requirements source=embedded type=\"SpecBind Requirements\"",
                ))
                .and(predicate::str::contains(
                    "selector=implementation-notes/main source=embedded",
                )),
        )
        .stderr("");

    let mut read = Command::cargo_bin("specbind").expect("specbind binary should build");
    read.current_dir(root.path())
        .args(["template", "read", "spec", "design/main"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "<!-- specbind:instruction Describe one owned decision. -->",
        ))
        .stderr("");
}

#[test]
fn lists_and_reads_the_steering_template_scope() {
    let root = project_fixture();

    let mut list = Command::cargo_bin("specbind").expect("specbind binary should build");
    list.current_dir(root.path())
        .args(["template", "list", "steering"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK TEMPLATE_LISTED: Found 4 recognized steering template(s).\n",
            )
            .and(predicate::str::contains(
                "selector=product source=embedded type=\"SpecBind Steering\" artifact_id=product template_path=en/steering/product.md output_path=steering/product.md\n",
            ))
            .and(predicate::str::contains(
                "selector=document source=embedded type=\"SpecBind Steering\" template_path=en/steering/document.md output_path=<authored>\n",
            )),
        )
        .stderr("");

    let mut read = Command::cargo_bin("specbind").expect("specbind binary should build");
    read.current_dir(root.path())
        .args(["template", "read", "steering", "document"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("type: SpecBind Steering")
                .and(predicate::str::contains("artifact_id:").not())
                .and(predicate::str::contains("specbind:instruction")),
        )
        .stderr("");

    let mut missing = Command::cargo_bin("specbind").expect("specbind binary should build");
    missing
        .current_dir(root.path())
        .args(["template", "read", "steering", "product-overview"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "ERROR TEMPLATE_SELECTOR_NOT_FOUND",
        ));
}

#[test]
fn falls_back_to_embedded_defaults_in_the_configured_language() {
    let root = project_fixture();

    let mut english = Command::cargo_bin("specbind").expect("specbind binary should build");
    english
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK TEMPLATE_LISTED: Found 6 recognized spec template(s).\n",
            )
            .and(predicate::str::contains("template_path=en/specs/brief.md"))
            .and(predicate::str::contains("source=project").not()),
        )
        .stderr("");

    let mut read = Command::cargo_bin("specbind").expect("specbind binary should build");
    read.current_dir(root.path())
        .args(["template", "read", "spec", "requirements"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("type: SpecBind Requirements")
                .and(predicate::str::contains("requirement: Requirement"))
                .and(predicate::str::contains("### Requirement 1:"))
                .and(predicate::str::contains("specbind:instruction")),
        );

    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"ja","agents":["codex"]}"#,
    );
    let mut japanese = Command::cargo_bin("specbind").expect("specbind binary should build");
    japanese
        .current_dir(root.path())
        .args(["template", "read", "spec", "requirements"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("requirement: 要件")
                .and(predicate::str::contains("### 要件 1:"))
                .and(predicate::str::contains("#### 受け入れ基準")),
        );
}

#[test]
fn rejects_unknown_template_selectors_and_invalid_template_profiles() {
    let root = project_fixture();
    write_template_fixture(root.path());

    let mut missing = Command::cargo_bin("specbind").expect("specbind binary should build");
    missing
        .current_dir(root.path())
        .args(["template", "read", "spec", "design/absent"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR TEMPLATE_SELECTOR_NOT_FOUND:",
        ));

    write(
        root.path(),
        ".specbind/settings/templates/specs/design-live.md",
        "---\ntype: SpecBind Design\nartifact_id: live\nrequirement_ids: ['1.1']\n---\n# Design\n",
    );
    let mut invalid = Command::cargo_bin("specbind").expect("specbind binary should build");
    invalid
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR TEMPLATE_LIST_FAILED:").and(
                predicate::str::contains("TEMPLATE_DESIGN_REQUIREMENT_IDS_FORBIDDEN"),
            ),
        );
}

#[test]
fn reports_an_unreadable_template_root_without_falling_back_silently() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/templates/specs",
        "not a directory\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR TEMPLATE_LIST_FAILED:")
                .and(predicate::str::contains("TEMPLATE_ROOT_NOT_DIRECTORY")),
        );
}

fn write_template_fixture(root: &Path) {
    write(
        root,
        ".specbind/settings/templates/specs/brief.md",
        "---\ntype: SpecBind Brief\n---\n<!-- specbind:instruction State the requested outcome. -->\n",
    );
    write(
        root,
        ".specbind/settings/templates/specs/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root,
        ".specbind/settings/templates/specs/technical-design/main.md",
        "---\ntype: SpecBind Design\nartifact_id: main\n---\n# Design\n\n<!-- specbind:instruction Describe one owned decision. -->\n",
    );
}

#[test]
fn creates_the_active_milestone_from_a_confirmed_scope() {
    let root = project_fixture();
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);

    let mut create = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut status = Command::cargo_bin("specbind").expect("specbind binary should build");
    status
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(predicate::str::contains("  Spec states: requirements=1\n"));
}

#[test]
fn refuses_creation_on_a_dirty_repository_or_conflicting_state() {
    let root = project_fixture();

    let mut dirty = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut conflicting = Command::cargo_bin("specbind").expect("specbind binary should build");
    conflicting
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"checkout","summary":"Add checkout"}]}}"#,
        )
        .assert()
        .failure()
        .stderr(predicate::str::contains("MILESTONE_SPEC_ALREADY_EXISTS"));

    let mut first = Command::cargo_bin("specbind").expect("specbind binary should build");
    first
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .success();

    let mut second = Command::cargo_bin("specbind").expect("specbind binary should build");
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
        let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut complete = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut unchanged = Command::cargo_bin("specbind").expect("specbind binary should build");
    unchanged
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_SCOPE_UNCHANGED:",
        ));

    let mut update = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut complete = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut drop_direct = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut create = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut direct_only = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut spec_backed = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut invalid = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut unchanged = Command::cargo_bin("specbind").expect("specbind binary should build");
    unchanged
        .current_dir(root.path())
        .args(["milestone", "rebaseline", "--revision", &baseline])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_BASELINE_UNCHANGED:",
        ));

    let mut rebaseline = Command::cargo_bin("specbind").expect("specbind binary should build");
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

fn direct_scope() -> &'static str {
    r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]},"body":"Overview\n\nDirect only.\n"}"#
}

fn create_direct_milestone(root: &Path) {
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root)
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .success();
}

#[test]
fn walks_every_gate_from_requirements_to_implementation() {
    let root = project_fixture();
    write_gate_fixture(root.path());

    let mut requirements = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut design = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut review = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut tasks = Command::cargo_bin("specbind").expect("specbind binary should build");
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
            "specbind-quick",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK SPEC_TASKS_APPROVED: Approved tasks for spec checkout.\n  State: implementation\n  Approval mode: delegated\n  Delegation workflow: specbind-quick\n  Passed at: ",
            ),
        )
        .stderr("");

    let mut status = Command::cargo_bin("specbind").expect("specbind binary should build");
    status
        .current_dir(root.path())
        .args(["spec", "status", "checkout"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  State: implementation\n")
                .and(predicate::str::contains("  Health: consistent\n"))
                .and(predicate::str::contains(
                    "  Gates: requirements=fresh, design=fresh, tasks=fresh, completion=not_reached\n",
                )),
        );
}

#[test]
fn reports_an_identical_fresh_approval_as_no_change() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    approve_requirements(root.path());

    let mut repeat = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut design = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut missing = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut explicit_with_workflow =
        Command::cargo_bin("specbind").expect("specbind binary should build");
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
            "specbind-quick",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "SPEC_GATE_DELEGATION_INVALID explicit approval does not accept a delegation workflow",
        ));

    let mut delegated_without_workflow =
        Command::cargo_bin("specbind").expect("specbind binary should build");
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
        let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut tasks = Command::cargo_bin("specbind").expect("specbind binary should build");
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
        let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    assert!(fresh.contains("\n  Health: consistent\n"));
}

/// Delegation exists to skip a confirmation the user would otherwise give, and
/// Decision 0100 calls that skip auditable. Before this field, the only durable
/// trace was `spec.yaml` itself, which no command read back.
#[test]
fn reports_which_gates_were_crossed_by_delegation() {
    let root = project_fixture();
    write_gate_fixture(root.path());

    let status = |root: &Path| {
        let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut approve = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut invalidate = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut repeat = Command::cargo_bin("specbind").expect("specbind binary should build");
    repeat
        .current_dir(root.path())
        .args(["spec", "design", "invalidate", "checkout"])
        .assert()
        .success()
        .stdout(
            "NO_CHANGE SPEC_DESIGN_NOT_APPROVED: Spec checkout has no design approval to invalidate.\n",
        );

    commit_all(root.path());
    let mut requirements = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut review = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut tasks = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut tasks_rewind = Command::cargo_bin("specbind").expect("specbind binary should build");
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
    let mut design_rewind = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut status = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
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

fn approve_requirements(root: &Path) {
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root)
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
        .success();
}

fn approve_design(root: &Path) {
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root)
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
}

fn gate_task_fixture() -> &'static str {
    "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['1.1']\n"
}

/// Writes a Spec-backed milestone whose one participating Spec sits in the
/// `requirements` state with no approval evidence yet.
fn write_gate_fixture(root: &Path) {
    write(root, "baseline.txt", "baseline\n");
    commit_all(root);
    let baseline = git_stdout(root, &["rev-parse", "HEAD"]);
    write(
        root,
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
    );
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );
    write(
        root,
        ".specbind/specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root,
        ".specbind/specs/checkout/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Design\n\n_Requirements: 1.1_\n",
    );
    write(
        root,
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: requirements\n  requirement_ids: null\n"
        ),
    );
}

const REVIEW_MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

fn review_candidate(assessment: &str) -> String {
    format!(r#"{{"schemaVersion":1,"assessment":"{assessment}","deepInputs":[]}}"#)
}

/// Writes a Spec-backed milestone whose one participating Spec sits in `tasks`
/// state with a fresh Design gate and no current `tasks.yaml`.
fn write_review_fixture(root: &Path) {
    write(root, "baseline.txt", "baseline\n");
    commit_all(root);
    let baseline = git_stdout(root, &["rev-parse", "HEAD"]);
    write(
        root,
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
    );
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );
    write(
        root,
        ".specbind/specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root,
        ".specbind/specs/checkout/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Design\n\n_Requirements: 1.1_\n",
    );
    let inputs = resolve_gate_inputs(&root.join(".specbind"), "checkout");
    assert!(
        inputs.inventory.issues.is_empty(),
        "{:?}",
        inputs.inventory.issues
    );
    let requirements = inputs
        .inputs
        .requirements
        .expect("requirements fingerprint");
    let design_yaml = inputs
        .inputs
        .design
        .expect("design fingerprints")
        .iter()
        .fold(String::new(), |mut output, (key, value)| {
            writeln!(output, "        {key}: {value}").expect("writing to a String cannot fail");
            output
        });
    write(
        root,
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: tasks\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {requirements}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n{design_yaml}"
        ),
    );
}

fn project_fixture() -> TempDir {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    fs::create_dir_all(root.path().join(".specbind/specs/checkout"))
        .expect("create SpecBind fixture directories");
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["codex"]}"#,
    );
    root
}

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create fixture parent");
    }
    fs::write(path, content).expect("write fixture");
}

fn git(root: &Path, arguments: &[&str]) {
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .expect("start Git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(root: &Path, arguments: &[&str]) -> String {
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .expect("start Git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("UTF-8 Git output")
        .trim()
        .to_owned()
}

fn commit_all(root: &Path) {
    git(root, &["add", "."]);
    git(
        root,
        &[
            "-c",
            "user.name=SpecBind Test",
            "-c",
            "user.email=specbind@example.invalid",
            "commit",
            "-m",
            "fixture",
        ],
    );
}

fn task_fixture() -> &'static str {
    "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: group\n      title: Build\n      tasks:\n        - id: '1.1'\n          kind: task\n          title: First\n          requirement_ids: ['1.1']\n        - id: '1.2'\n          kind: task\n          title: Second\n          requirement_ids: ['1.2']\n    - id: '2'\n      kind: task\n      title: Integrate\n      requirement_ids: ['1.3']\n    - id: '3'\n      kind: task\n      title: Document\n      requirement_ids: ['1.4']\n      boundaries: ['docs/']\n      parallel: true\n      depends_on: ['1.1']\nexecution:\n  tasks:\n    '1.1':\n      status: completed\n    '2':\n      status: blocked\n      blocked_reason: Waiting for an API decision\n"
}

fn write_status_fixture(root: &Path) {
    let specbind = root.join(".specbind");
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );
    write(
        root,
        ".specbind/specs/checkout/contract.md",
        "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n",
    );
    write(
        root,
        ".specbind/specs/checkout/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n",
    );
    write(
        root,
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['1.1']\n    - id: '2'\n      kind: task\n      title: Review\n      requirement_ids: ['1.1']\nexecution:\n  tasks:\n    '1':\n      status: completed\n    '2':\n      status: blocked\n      blocked_reason: Waiting for review\n",
    );
    let inputs = resolve_gate_inputs(&specbind, "checkout");
    assert!(
        inputs.inventory.issues.is_empty(),
        "{:?}",
        inputs.inventory.issues
    );
    let requirements = inputs
        .inputs
        .requirements
        .expect("requirements fingerprint");
    let tasks = inputs.inputs.task_plan.expect("task-plan fingerprint");
    let design = inputs.inputs.design.expect("design fingerprints");
    let design_yaml = design
        .iter()
        .fold(String::new(), |mut output, (key, value)| {
            writeln!(output, "        {key}: {value}").expect("writing to a String cannot fail");
            output
        });
    write(
        root,
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: implementation\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {requirements}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n{design_yaml}    tasks:\n      passed_at: 2026-08-16T12:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        tasks.yaml#plan: {tasks}\n"
        ),
    );
}

#[test]
fn lists_no_specs_before_the_specs_directory_exists() {
    let root = project_fixture();
    fs::remove_dir_all(root.path().join(".specbind/specs"))
        .expect("remove the specs directory created by the fixture");

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["spec", "list"])
        // Installation creates settings before any Spec exists. Discovery must
        // be able to ask for the empty list without materializing the directory.
        .assert()
        .success()
        .stdout("OK SPEC_LISTED: Found 0 spec(s).\n")
        .stderr("");
}

#[test]
fn lists_specs_in_identity_order_with_lifecycle_and_artifact_presence() {
    let root = project_fixture();
    write_status_fixture(root.path());
    fs::create_dir_all(root.path().join(".specbind/specs/analytics"))
        .expect("create idle spec directory");
    write(
        root.path(),
        ".specbind/specs/analytics/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["spec", "list"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK SPEC_LISTED: Found 2 spec(s).\n  analytics: state=idle milestone=none requirements=no contract=no\n  checkout: state=implementation milestone=")
            .and(predicate::str::contains(" requirements=yes contract=yes\n")))
        .stderr("");
}

#[test]
fn lists_an_unreadable_spec_instead_of_failing_the_listing() {
    let root = project_fixture();
    write_status_fixture(root.path());
    fs::create_dir_all(root.path().join(".specbind/specs/analytics"))
        .expect("create broken spec directory");
    write(
        root.path(),
        ".specbind/specs/analytics/spec.yaml",
        "schema_version: 1\nactive_change: {state: nonsense}\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        // The broken Spec is reported, and the healthy one beside it survives.
        .args(["spec", "list"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Found 2 spec(s).")
                .and(predicate::str::contains("\n  analytics: unreadable: "))
                .and(predicate::str::contains("\n  checkout: state=")),
        )
        .stderr("");
}

#[test]
fn reports_no_change_reading_scope_without_an_active_milestone() {
    let root = project_fixture();

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        .stdout("NO_CHANGE NO_ACTIVE_MILESTONE: No active milestone exists.\n")
        .stderr("");
}

#[test]
fn refuses_to_emit_a_partial_scope_from_an_invalid_roadmap() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        "---\ntype: SpecBind Roadmap\n---\n# Roadmap\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR MILESTONE_SCOPE_FAILED: Cannot read the active milestone scope.\n",
        ));
}

#[test]
fn writes_the_current_scope_as_a_replacement_candidate() {
    let root = project_fixture();
    commit_all(root.path());

    let mut create = Command::cargo_bin("specbind").expect("specbind binary should build");
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Add payments"}],"directChanges":[{"id":"docs","summary":"Update docs","dependsOn":[{"spec":"payments"}]}]},"body":"Overview\n\nDeliver payments.\n"}"#,
        )
        .assert()
        .success();

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    let output = command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let document = String::from_utf8(output).expect("UTF-8 scope document");

    // The serialization is a byte-exact contract: declared field order,
    // two-space indentation, no body, no per-item status, one trailing newline.
    assert_eq!(
        document,
        concat!(
            "{\n",
            "  \"schemaVersion\": 1,\n",
            "  \"workItems\": {\n",
            "    \"newSpecs\": [\n",
            "      {\n",
            "        \"spec\": \"payments\",\n",
            "        \"summary\": \"Add payments\"\n",
            "      }\n",
            "    ],\n",
            "    \"directChanges\": [\n",
            "      {\n",
            "        \"id\": \"docs\",\n",
            "        \"summary\": \"Update docs\",\n",
            "        \"dependsOn\": [\n",
            "          { \"spec\": \"payments\" }\n",
            "        ]\n",
            "      }\n",
            "    ]\n",
            "  }\n",
            "}\n",
        )
    );

    // The round trip is the invariant Decision 0097 accepts: feeding the read
    // straight back into the replacement changes nothing.
    let mut round_trip = Command::cargo_bin("specbind").expect("specbind binary should build");
    round_trip
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(document)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_SCOPE_UNCHANGED",
        ))
        .stderr("");
}

#[test]
fn omits_completed_direct_status_from_the_emitted_scope() {
    let root = project_fixture();
    commit_all(root.path());

    let mut create = Command::cargo_bin("specbind").expect("specbind binary should build");
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]}}"#,
        )
        .assert()
        .success();
    commit_all(root.path());

    let revision = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    let mut complete = Command::cargo_bin("specbind").expect("specbind binary should build");
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

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        // Status is CLI-owned and preserved by identity, so a candidate that
        // carried it would be rejected by the command it feeds.
        .stdout(predicate::str::contains("\"status\"").not())
        .stderr("");
}

#[test]
fn emits_the_complete_body_only_when_deliberately_requested() {
    let root = project_fixture();
    commit_all(root.path());

    let mut create = Command::cargo_bin("specbind").expect("specbind binary should build");
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]},"body":"Overview\n\nDeliver docs.\n"}"#,
        )
        .assert()
        .success();

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    let output = command
        .current_dir(root.path())
        .args(["milestone", "scope", "--include-body"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let document = String::from_utf8(output).expect("UTF-8 scope document");

    // The body is complete and follows the work items, so a caller edits one
    // whole value rather than composing a replacement from a fragment.
    assert!(
        document.contains("  \"body\": \"Overview\\n\\nDeliver docs.\\n\"\n}\n"),
        "{document}"
    );

    // The round trip holds for this form too.
    let mut round_trip = Command::cargo_bin("specbind").expect("specbind binary should build");
    round_trip
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(document)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_SCOPE_UNCHANGED",
        ))
        .stderr("");

    // The default read stays body-free, so an ordinary round trip cannot
    // rewrite authored prose.
    let mut default = Command::cargo_bin("specbind").expect("specbind binary should build");
    default
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"body\"").not())
        .stderr("");
}

fn steering_document(id: &str, title: &str) -> String {
    format!(
        "---
type: SpecBind Steering
artifact_id: {id}
---
# {title}
"
    )
}

#[test]
fn lists_no_steering_before_any_is_authored() {
    let root = project_fixture();

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["steering", "list"])
        // An absent steering directory is an empty inventory, not a fault.
        .assert()
        .success()
        .stdout(
            "OK STEERING_LISTED: Found 0 steering document(s).
",
        )
        .stderr("");
}

#[test]
fn lists_steering_by_artifact_id_and_excludes_other_types() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        &steering_document("product", "Product"),
    );
    write(
        root.path(),
        ".specbind/steering/nested/conventions.md",
        &steering_document("naming", "Naming"),
    );
    commit_all(root.path());
    let mut create = Command::cargo_bin("specbind").expect("specbind binary should build");
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]}}"#,
        )
        .assert()
        .success();

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["steering", "list"])
        .assert()
        .success()
        // Ordered by artifact_id, discovered recursively, and the active Roadmap
        // in the same directory is excluded by type without being an anomaly.
        .stdout(concat!(
            "OK STEERING_LISTED: Found 2 steering document(s).
",
            "  selector=naming type=\"SpecBind Steering\" path=steering/nested/conventions.md
",
            "  selector=product type=\"SpecBind Steering\" path=steering/product.md
",
        ))
        .stderr("");
}

#[test]
fn reads_one_steering_selector_as_raw_markdown() {
    let root = project_fixture();
    let content = steering_document("product", "Product");
    write(root.path(), ".specbind/steering/product.md", &content);

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["steering", "read", "product"])
        .assert()
        .success()
        .stdout(content)
        .stderr("");
}

#[test]
fn reports_an_unknown_steering_selector_without_touching_stdout() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        &steering_document("product", "Product"),
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["steering", "read", "missing"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR STEERING_READ_INVALID: unknown steering selector: missing
",
        ));
}

#[test]
fn drops_both_documents_sharing_one_artifact_id() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/one.md",
        &steering_document("product", "One"),
    );
    write(
        root.path(),
        ".specbind/steering/two.md",
        &steering_document("product", "Two"),
    );

    let mut list = Command::cargo_bin("specbind").expect("specbind binary should build");
    list.current_dir(root.path())
        .args(["steering", "list"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::contains("STEERING_ARTIFACT_ID_DUPLICATE steering/one.md")
                .and(predicate::str::contains(
                    "STEERING_ARTIFACT_ID_DUPLICATE steering/two.md",
                ))
                // Neither is offered as a usable selector.
                .and(predicate::str::contains("selector=product").not()),
        );

    let mut read = Command::cargo_bin("specbind").expect("specbind binary should build");
    read.current_dir(root.path())
        .args(["steering", "read", "product"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR STEERING_READ_INVALID: steering selector is ambiguous: product
",
        ));
}

#[test]
fn refuses_to_read_valid_guidance_while_the_collection_is_incomplete() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        &steering_document("product", "Product"),
    );
    write(
        root.path(),
        ".specbind/steering/broken.md",
        "no front matter
",
    );

    // Unlike a spec-local artifact read, an unrelated fault fails this read:
    // guidance known to be incomplete cannot be safely acted on.
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["steering", "read", "product"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR STEERING_READ_FAILED: ").and(
                predicate::str::contains("STEERING_FRONTMATTER_INVALID steering/broken.md"),
            ),
        );
}

#[test]
fn rejects_steering_without_a_usable_artifact_id() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        "---
type: SpecBind Steering
---
# Product
",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["steering", "list"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "STEERING_ARTIFACT_ID_INVALID steering/product.md",
        ));
}

#[test]
fn installs_the_marked_block_into_each_agent_instruction_file() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    write(root.path(), "AGENTS.md", "# Project\n\nOur own rules.\n");

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args([
            "install",
            "--agent",
            "claude-code",
            "--agent",
            "codex",
            "--language",
            "en",
            "--project-instructions",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("- create AGENTS.md [project-instructions]").and(
                predicate::str::contains("- create CLAUDE.md [project-instructions]"),
            ),
        );

    // The project's own content survives; the block is appended after it.
    let agents = fs::read_to_string(root.path().join("AGENTS.md")).expect("AGENTS.md");
    assert!(
        agents.starts_with("# Project\n\nOur own rules.\n\n"),
        "{agents}"
    );
    assert!(agents.contains("<!-- specbind:block -->"), "{agents}");
    assert!(
        agents.trim_end().ends_with("<!-- /specbind:block -->"),
        "{agents}"
    );

    // A missing file is created holding the block alone.
    let claude = fs::read_to_string(root.path().join("CLAUDE.md")).expect("CLAUDE.md");
    assert!(claude.starts_with("<!-- specbind:block -->\n"), "{claude}");

    // Re-running changes nothing.
    let mut again = Command::cargo_bin("specbind").expect("specbind binary should build");
    again
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("NO_CHANGE INSTALL_UP_TO_DATE"));
}

#[test]
fn plans_no_instruction_file_when_the_block_is_disabled() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args([
            "install",
            "--dry-run",
            "--agent",
            "codex",
            "--language",
            "en",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("[project-instructions]").not());
    assert!(!root.path().join("AGENTS.md").exists());
}

#[test]
fn stops_installing_instructions_on_a_malformed_marker() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    // An opening marker with no closing one: the installer never repairs text
    // the project owns.
    write(
        root.path(),
        "AGENTS.md",
        "# Project\n\n<!-- specbind:block -->\nhand written\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args([
            "install",
            "--dry-run",
            "--agent",
            "codex",
            "--language",
            "en",
            "--project-instructions",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "PROJECT_INSTRUCTIONS_MARKERS_INVALID AGENTS.md",
        ));

    let preserved = fs::read_to_string(root.path().join("AGENTS.md")).expect("AGENTS.md");
    assert_eq!(
        preserved,
        "# Project\n\n<!-- specbind:block -->\nhand written\n"
    );
}

#[test]
fn lists_accepted_adapters_with_project_presence() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/adapters/git.md",
        "---\ntype: SpecBind Git Adapter\n---\n# Git\n\nCommit after each gate.\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adapter", "list"])
        .assert()
        .success()
        .stdout(concat!(
            "OK ADAPTER_LISTED: Found 3 accepted adapter(s).\n",
            "  selector=release type=\"SpecBind Release Adapter\" path=settings/adapters/release.md present=no\n",
            "  selector=git type=\"SpecBind Git Adapter\" path=settings/adapters/git.md present=yes\n",
            "  selector=deferred type=\"SpecBind Deferred Findings Adapter\" path=settings/adapters/deferred.md present=no\n",
        ))
        .stderr("");
}

#[test]
fn reads_one_adapter_as_raw_markdown_and_reports_absence() {
    let root = project_fixture();
    let content = "---\ntype: SpecBind Git Adapter\n---\n# Git\n\nCommit after each gate.\n";
    write(root.path(), ".specbind/settings/adapters/git.md", content);

    let mut present = Command::cargo_bin("specbind").expect("specbind binary should build");
    present
        .current_dir(root.path())
        .args(["adapter", "read", "git"])
        .assert()
        .success()
        .stdout(content)
        .stderr("");

    // Absence is reported, not judged. Whether a missing adapter is a fault
    // belongs to the consuming skill.
    let mut absent = Command::cargo_bin("specbind").expect("specbind binary should build");
    absent
        .current_dir(root.path())
        .args(["adapter", "read", "release"])
        .assert()
        .success()
        .stdout("NO_CHANGE ADAPTER_ABSENT: The project has no release adapter.\n")
        .stderr("");
}

#[test]
fn refuses_a_selector_the_product_does_not_accept() {
    let root = project_fixture();
    // The directory is organization, not an extension loader: an unknown file
    // below it is never readable.
    write(
        root.path(),
        ".specbind/settings/adapters/deploy.md",
        "---\ntype: SpecBind Git Adapter\n---\n# Deploy\n",
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adapter", "read", "deploy"])
        .assert()
        .failure()
        .stdout("")
        .stderr("ERROR ADAPTER_READ_INVALID: unknown adapter selector: deploy\n");
}

#[test]
fn installs_localized_adapter_scaffolds_and_keeps_project_copies() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    let owned = "---\ntype: SpecBind Git Adapter\n---\n# Ours\n";
    write(root.path(), ".specbind/settings/adapters/git.md", owned);

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "ja"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains(
                "- create .specbind/settings/adapters/release.md [adapter]",
            )
            .and(predicate::str::contains(
                "- keep .specbind/settings/adapters/git.md [adapter] (project-owned settings are never overwritten)",
            )),
        );

    // The scaffold follows the configured language; the type literal does not.
    let release = fs::read_to_string(root.path().join(".specbind/settings/adapters/release.md"))
        .expect("release adapter");
    assert!(
        release.starts_with("---\ntype: SpecBind Release Adapter\n---\n"),
        "{release}"
    );
    assert!(release.contains("# リリースアダプタ"), "{release}");

    let git_adapter = fs::read_to_string(root.path().join(".specbind/settings/adapters/git.md"))
        .expect("git adapter");
    assert_eq!(git_adapter, owned);
}

#[test]
fn lists_and_reads_embedded_schemas_without_a_project() {
    // Like the protocols, these are properties of the binary. Running outside
    // any SpecBind project is the structural guarantee of that.
    let outside = tempfile::tempdir().expect("temporary directory");

    let mut list = Command::cargo_bin("specbind").expect("specbind binary should build");
    list.current_dir(outside.path())
        .args(["schema", "list"])
        .assert()
        .success()
        .stdout(concat!(
            "OK SCHEMA_LISTED: Found 3 embedded schema(s).\n",
            "  selector=spec/v1 artifact=spec.yaml written_by=\"guarded CLI operations only\"\n",
            "  selector=scope/v1 artifact=milestone scope candidate (transient) written_by=\"the authoring agent\"\n",
            "  selector=tasks/v1 artifact=tasks.yaml written_by=\"the authoring agent\"\n",
        ))
        .stderr("");

    let mut read = Command::cargo_bin("specbind").expect("specbind binary should build");
    read.current_dir(outside.path())
        .args(["schema", "read", "tasks/v1"])
        .assert()
        .success()
        // The read is the same bytes the runtime validator compiles, so the
        // format an agent authors against cannot drift from the one enforced.
        .stdout(predicate::eq(specbind::schema::TASKS_V1_SCHEMA_JSON))
        .stderr("");
}

#[test]
fn refuses_a_schema_selector_the_binary_does_not_carry() {
    let outside = tempfile::tempdir().expect("temporary directory");

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(outside.path())
        // An unversioned selector is not accepted: the version is part of the
        // identity, so a caller always names the schema it is targeting.
        .args(["schema", "read", "tasks"])
        .assert()
        .failure()
        .stdout("")
        .stderr("ERROR SCHEMA_READ_INVALID: unknown schema selector: tasks\n");
}
