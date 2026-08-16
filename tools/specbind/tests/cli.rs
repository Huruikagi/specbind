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
