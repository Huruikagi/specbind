use super::*;

#[test]
fn reports_no_active_milestone_as_no_change() {
    let root = project_fixture();

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout("NO_CHANGE NO_ACTIVE_MILESTONE: No active milestone exists.\n")
        .stderr("");

    let output = specbind_command()
        .current_dir(root.path())
        .args(["milestone", "status", "--json"])
        .output()
        .expect("milestone status runs");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let actual: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON document");
    assert_eq!(
        actual,
        serde_json::json!({
            "status": "no_change",
            "code": "NO_ACTIVE_MILESTONE",
            "data": null
        })
    );
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

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: implementation\n")
                .and(predicate::str::contains("  State health: consistent\n"))
                .and(predicate::str::contains(
                    "  Semantic alignment: not evaluated\n",
                ))
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

    let output = specbind_command()
        .current_dir(root.path())
        .args(["milestone", "status", "--json"])
        .output()
        .expect("milestone status runs");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    let mut actual: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON document");
    assert!(actual["data"]["revision"].is_string());
    actual["data"]["revision"] = serde_json::Value::String("<revision>".to_owned());
    assert_eq!(
        actual,
        serde_json::json!({
            "status": "ok",
            "code": "MILESTONE_STATUS_REPORTED",
            "data": {
                "milestoneId": "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62",
                "targetRelease": null,
                "stage": "implementation",
                "health": "consistent",
                "semanticAlignment": "not_evaluated",
                "contractReview": "not_applicable",
                "specStates": {},
                "directProgress": {"completed": 0, "total": 2},
                "revision": "<revision>",
                "baseline": "0123456789abcdef0123456789abcdef01234567",
                "items": [
                    {
                        "id": "direct:docs",
                        "summary": "Update docs",
                        "status": "pending",
                        "waitingFor": []
                    },
                    {
                        "id": "direct:publish",
                        "summary": "Publish site",
                        "status": "pending",
                        "waitingFor": ["direct:docs"]
                    }
                ],
                "actionable": [
                    {
                        "item": "direct:docs",
                        "commandOperand": "docs",
                        "action": "implementation"
                    }
                ],
                "currentBlockers": [],
                "releaseReadinessEvaluated": false,
                "releaseBlockers": null,
                "diagnostics": []
            }
        })
    );
}

#[test]
fn reports_milestone_status_failure_as_json_without_stderr_text() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        "---\ntype: SpecBind Roadmap\n---\n# Roadmap\n",
    );

    let output = specbind_command()
        .current_dir(root.path())
        .args(["milestone", "status", "--json"])
        .output()
        .expect("milestone status runs");

    assert!(!output.status.success());
    assert!(output.stderr.is_empty());
    let actual: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout is one JSON document");
    assert_eq!(actual["status"], "error");
    assert_eq!(actual["code"], "MILESTONE_STATUS_FAILED");
    assert_eq!(actual["message"], "Cannot report the active milestone.");
    assert!(
        actual["details"]
            .as_array()
            .is_some_and(|details| !details.is_empty())
    );
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

    let mut preflight = specbind_command();
    preflight
        .current_dir(root.path())
        .args(["milestone", "direct", "preflight", "docs"])
        .assert()
        .success()
        .stdout(format!(
            "OK DIRECT_COMPLETION_PREFLIGHT_READY: Direct item docs is ready for completion validation.\n  Implementation revision: {revision}\n"
        ));

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
        .success()
        .stdout("OK DIRECT_COMPLETION_RECORDED: Recorded completion for Direct item docs.\n");

    let mut retry = specbind_command();
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

    let mut invalid = specbind_command();
    invalid
        .current_dir(root.path())
        .args(["milestone", "bind-release", "bad/version"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR INVALID_RELEASE_VERSION: Cannot bind milestone release.",
        ));

    let mut bind = specbind_command();
    bind.current_dir(root.path())
        .args(["milestone", "bind-release", "v1.4.0"])
        .assert()
        .success()
        .stdout(
            "OK RELEASE_BOUND: Bound milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62 to release v1.4.0.\n  Roadmap archive: releases/v1.4.0-roadmap.md\n  Contract review archive: releases/v1.4.0-contract-review.md\n",
        )
        .stderr("");

    let mut retry = specbind_command();
    retry
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.4.0"])
        .assert()
        .success()
        .stdout("NO_CHANGE RELEASE_ALREADY_BOUND: Milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62 is already bound to release v1.4.0.\n")
        .stderr("");

    let mut confirmation = specbind_command();
    confirmation
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.5.0"])
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "ERROR RELEASE_REBIND_REQUIRED: Cannot bind milestone release.",
        ));

    let mut dirty = specbind_command();
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
    let mut collision = specbind_command();
    collision
        .current_dir(root.path())
        .args(["milestone", "bind-release", "v1.5.0", "--rebind"])
        .assert()
        .failure()
        .stderr(predicate::str::starts_with(
            "ERROR RELEASE_ARCHIVE_COLLISION: Cannot bind milestone release.",
        ));

    let mut rebind = specbind_command();
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
    let mut command = specbind_command();

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

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: contract_review\n")
                .and(predicate::str::contains("  State health: inconsistent\n"))
                .and(predicate::str::contains(
                    "  Spec states: implementation=1\n",
                ))
                .and(predicate::str::contains("MILESTONE_TASKS_BEFORE_REVIEW")),
        );
}

#[test]
fn reports_an_absent_future_review_without_a_health_diagnostic() {
    let root = project_fixture();
    write_gate_fixture(root.path());
    commit_all(root.path());

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: requirements\n")
                .and(predicate::str::contains("  State health: consistent\n"))
                .and(predicate::str::contains("  Contract review: absent\n"))
                .and(predicate::str::contains("CONTRACT_REVIEW_MISSING").not()),
        )
        .stderr("");
}

#[test]
fn reports_an_absent_actionable_review_as_expected_work() {
    let root = project_fixture();
    write_review_fixture(root.path());
    commit_all(root.path());

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: contract_review\n")
                .and(predicate::str::contains("  State health: consistent\n"))
                .and(predicate::str::contains("  Contract review: absent\n"))
                .and(predicate::str::contains("milestone action=contract_review"))
                .and(predicate::str::contains("CONTRACT_REVIEW_MISSING").not()),
        )
        .stderr("");
}

#[test]
fn reports_a_broken_required_contract_graph_in_milestone_health() {
    let root = project_fixture();
    write_review_fixture(root.path());
    write(
        root.path(),
        ".specbind/specs/outsider/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );
    write(
        root.path(),
        ".specbind/specs/outsider/contract.yaml",
        "schema_version: 1\nowns: []\nexports: []\nconsumes:\n  - { id: missing, target: { spec: absent, section: exports, id: value }, description: Missing. }\ninvariants: []\nfile_ownership: []\n",
    );
    commit_all(root.path());

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "status"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("  Stage: contract_review\n")
                .and(predicate::str::contains("  State health: inconsistent\n"))
                .and(predicate::str::contains(
                    "CONTRACT_GRAPH_TARGET_SPEC_MISSING",
                )),
        )
        .stderr("");
}

#[test]
fn accepts_a_stdin_review_candidate_and_reports_fresh_status() {
    let root = project_fixture();
    write_review_fixture(root.path());

    let mut accept = specbind_command();
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

    let mut status = specbind_command();
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

    let mut accept = specbind_command();
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

    let mut direct_only = specbind_command();
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
    let mut absent = specbind_command();
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
    let mut accept = specbind_command();
    accept
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible."))
        .assert()
        .success();

    write(
        root.path(),
        ".specbind/specs/checkout/contract.yaml",
        "schema_version: 1\nowns: []\nexports:\n  - { id: value, description: Value. }\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
    );
    let mut stale = specbind_command();
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
    let mut invalid = specbind_command();
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
    let mut internal = specbind_command();
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
    let mut not_a_file = specbind_command();
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
    let mut invalid_encoding = specbind_command();
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

    let mut invalid_json = specbind_command();
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
    let mut accept = specbind_command();
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
    let mut blocked = specbind_command();
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
        let mut accept = specbind_command();
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
