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
            "OK SPEC_STATUS_REPORTED: Reported status for spec checkout.\n  State: implementation\n  Milestone: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  Health: consistent\n  Gates: requirements=fresh, design=fresh, tasks=fresh, completion=not_reached\n  Task progress: 2 total, 1 completed, 0 pending, 1 blocked\n  Next actionable: none\n  Blockers:\n    - 2: Waiting for review\n  Requirement coverage: design 1/1, tasks 1/1 (required)\n  Diagnostics: none\n",
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
                    "  Cross-spec review: not_applicable\n",
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
            "OK RELEASE_BOUND: Bound milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62 to release v1.4.0.\n  Roadmap archive: releases/v1.4.0-roadmap.md\n  Cross-spec review archive: releases/v1.4.0-cross-spec-review.md\n",
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
            predicate::str::contains("  Stage: cross_spec_review\n")
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
                "OK MILESTONE_REVIEW_ACCEPTED: Accepted cross-spec review for milestone {REVIEW_MILESTONE}.\n  Passed at: "
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
                "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported cross-spec review status for milestone {REVIEW_MILESTONE}.\n  Status: fresh\n  Passed at: "
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
            "OK MILESTONE_REVIEW_ACCEPTED: Accepted cross-spec review for milestone",
        ))
        .stderr("");

    let accepted = fs::read_to_string(root.path().join(".specbind/state/cross-spec-review.md"))
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
            "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported cross-spec review status for milestone {REVIEW_MILESTONE}.\n  Status: not_applicable\n"
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
                "OK MILESTONE_REVIEW_STATUS_REPORTED: Reported cross-spec review status for milestone {REVIEW_MILESTONE}.\n  Status: absent\n"
            ))
            .and(predicate::str::contains("Passed at:").not())
            .and(predicate::str::contains("\n  Inputs:").not())
            .and(predicate::str::contains("CROSS_SPEC_REVIEW_MISSING")),
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
                    "    - CROSS_SPEC_REVIEW_INPUTS_STALE",
                )),
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/state/cross-spec-review.md",
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
                "ERROR MILESTONE_REVIEW_STATUS_FAILED: Cannot report the cross-spec review status.",
            )
            .and(predicate::str::contains("CROSS_SPEC_REVIEW_TYPE_INVALID")),
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
                "ERROR MILESTONE_REVIEW_ACCEPT_FAILED: Cannot accept the cross-spec review.",
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
                "ERROR MILESTONE_REVIEW_ACCEPT_FAILED: Cannot accept the cross-spec review.",
            )
            .and(predicate::str::contains("CROSS_SPEC_REVIEW_CANDIDATE_")),
        );

    assert!(
        !root
            .path()
            .join(".specbind/state/cross-spec-review.md")
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
    let path = root.path().join(".specbind/state/cross-spec-review.md");
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
                "ERROR MILESTONE_REVIEW_ACCEPT_FAILED: Cannot accept the cross-spec review.",
            )
            .and(predicate::str::contains(
                "CROSS_SPEC_REVIEW_TASKS_ALREADY_EXIST",
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
                "OK MILESTONE_REVIEW_ACCEPTED: Accepted cross-spec review for milestone",
            ));
    }

    let accepted = fs::read_to_string(root.path().join(".specbind/state/cross-spec-review.md"))
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
                "OK INSTALL_PLANNED: Planned 8 action(s) for 2 agent(s).\n",
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
                "\n  Summary: 8 create, 0 replace, 0 keep\n",
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
                    "\n  Summary: 6 create, 0 replace, 2 keep\n",
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
                "OK INSTALL_APPLIED: Applied 8 action(s) for 1 agent(s).\n",
            )
            .and(predicate::str::contains(
                "\n  Summary: 8 created, 0 replaced, 0 kept\n",
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
            "\n  Summary: 7 created, 0 replaced, 1 kept\n",
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
        ".specbind/state/cross-spec-review.md",
        "---\ntype: SpecBind Cross-Spec Review\n---\nAccepted.\n",
    );
    commit_all(root.path());
    let review = root.path().join(".specbind/state/cross-spec-review.md");

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
        ".specbind/state/cross-spec-review.md",
        "---\ntype: SpecBind Cross-Spec Review\n---\nAccepted.\n",
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
fn blocks_tasks_approval_without_a_fresh_cross_spec_review() {
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
                "CROSS_SPEC_REVIEW_TASKS_APPROVAL_BLOCKED",
            )),
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
            "OK SPEC_DESIGN_INVALIDATED: Invalidated design for spec checkout.\n  State: design\n",
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
            "OK SPEC_REQUIREMENTS_INVALIDATED: Invalidated requirements for spec checkout.\n  State: requirements\n",
        );
    let rewound = fs::read_to_string(root.path().join(".specbind/specs/checkout/spec.yaml"))
        .expect("rewound spec metadata");
    assert!(rewound.contains("state: requirements"), "{rewound}");
    assert!(rewound.contains("requirement_ids: null"), "{rewound}");
    assert!(!rewound.contains("gate_evidence"), "{rewound}");
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
