use super::*;
use specbind::completion::batch;

fn evidence(revision: &str) -> String {
    serde_json::json!({"schemaVersion":1,"implementationRevision":revision,
        "targets":[
            {"spec":"checkout","mechanicalChecks":[{"kind":"test","command":"cargo test","exitCode":0}]},
            {"spec":"catalog","mechanicalChecks":[{"kind":"test","command":"cargo test","exitCode":0}]}
        ]}).to_string()
}

#[test]
fn cli_accepts_the_explicit_set_at_one_revision_and_verifies_one_checkpoint() {
    let root = spec_fixture_with_second(true);
    let sb = root.path().join(".specbind");
    let head = git(root.path(), &["rev-parse", "HEAD"]);
    let before = read_spec(&sb);
    let output = run_cli(
        root.path(),
        &[
            "spec",
            "completion",
            "batch",
            "preflight",
            "--spec",
            "checkout",
            "--spec",
            "catalog",
        ],
        None,
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let preflight: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(preflight["implementationRevision"], head);
    let input = evidence(&head);
    let output = run_cli(
        root.path(),
        &["spec", "completion", "batch", "accept", "--evidence", "-"],
        Some(&input),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report["accepted"],
        serde_json::json!(["checkout", "catalog"])
    );
    assert_eq!(report["pending"], serde_json::json!([]));
    assert_eq!(git(root.path(), &["rev-parse", "HEAD"]), head);
    assert!(batch::verify(root.path(), &sb, &input).is_err());
    let retry = batch::accept(root.path(), &sb, &input).unwrap();
    assert!(!retry.succeeded());
    assert_eq!(retry.accepted, [] as [String; 0]);
    commit_all(root.path(), "Accept completion set");
    let output = run_cli(
        root.path(),
        &["spec", "completion", "batch", "verify", "--evidence", "-"],
        Some(&input),
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let after = read_spec(&sb);
    let old_gates = before.active_change.0.unwrap().gate_evidence.unwrap();
    let new_gates = after.active_change.0.unwrap().gate_evidence.unwrap();
    assert_eq!(new_gates.requirements, old_gates.requirements);
    assert_eq!(new_gates.design, old_gates.design);
    assert_eq!(new_gates.tasks, old_gates.tasks);
    assert!(
        batch::accept(root.path(), &sb, &input)
            .unwrap()
            .issues
            .iter()
            .any(|i| i.code == "COMPLETION_REVISION_CHANGED")
    );
}

#[test]
fn invalid_later_evidence_or_target_never_writes_the_first_spec() {
    let root = spec_fixture_with_second(true);
    let sb = root.path().join(".specbind");
    let head = git(root.path(), &["rev-parse", "HEAD"]);
    let input = evidence(&head);
    for replacement in ["missing", "../catalog", "checkout"] {
        let result = batch::accept(root.path(), &sb, &input.replace("catalog", replacement));
        assert!(result.is_err() || !result.unwrap().succeeded());
        assert_eq!(git(root.path(), &["status", "--porcelain"]), "");
    }
    let mut value: serde_json::Value = serde_json::from_str(&input).unwrap();
    value["targets"][1]["mechanicalChecks"][0]["exitCode"] = 1.into();
    assert!(batch::accept(root.path(), &sb, &value.to_string()).is_err());
    value["targets"][1]["mechanicalChecks"][0]["exitCode"] = 0.into();
    value["targets"][1]["approved"] = true.into();
    assert!(batch::accept(root.path(), &sb, &value.to_string()).is_err());
    assert_eq!(git(root.path(), &["status", "--porcelain"]), "");
    assert!(batch::preflight(root.path(), &sb, &[]).is_err());
}

#[test]
fn rejects_dirty_moving_and_stale_state_without_acceptance() {
    let root = spec_fixture_with_second(true);
    let sb = root.path().join(".specbind");
    let head = git(root.path(), &["rev-parse", "HEAD"]);
    write(root.path(), "new.txt", "unrelated");
    let report = batch::accept(root.path(), &sb, &evidence(&head)).unwrap();
    assert_eq!(report.result, "COMPLETION_BATCH_REJECTED");
    assert_eq!(report.accepted, [] as [String; 0]);
    assert_eq!(report.pending, ["checkout", "catalog"]);
    commit_all(root.path(), "Unrelated later change");
    assert!(
        !batch::accept(root.path(), &sb, &evidence(&head))
            .unwrap()
            .succeeded()
    );
    write(
        root.path(),
        ".specbind/specs/catalog/requirements.md",
        "# Changed requirement\n",
    );
    commit_all(root.path(), "Stale later target");
    let head = git(root.path(), &["rev-parse", "HEAD"]);
    let report = batch::accept(root.path(), &sb, &evidence(&head)).unwrap();
    assert!(!report.succeeded());
    assert_eq!(report.accepted, [] as [String; 0]);
    assert_eq!(git(root.path(), &["status", "--porcelain"]), "");
}

#[test]
fn refuses_unrelated_or_earlier_gate_changes_in_a_checkpoint() {
    for extra in [true, false] {
        let root = spec_fixture_with_second(true);
        let sb = root.path().join(".specbind");
        let input = evidence(&git(root.path(), &["rev-parse", "HEAD"]));
        assert!(batch::accept(root.path(), &sb, &input).unwrap().succeeded());
        if extra {
            write(root.path(), "extra.txt", "unrelated");
        } else {
            let path = sb.join("specs/catalog/spec.yaml");
            let text = fs::read_to_string(&path)
                .unwrap()
                .replace("2026-08-16T10:00:00Z", "2026-08-16T10:01:00Z");
            fs::write(path, text).unwrap();
        }
        commit_all(root.path(), "Mixed checkpoint");
        assert!(batch::verify(root.path(), &sb, &input).is_err());
    }
}

#[test]
fn refuses_split_completion_checkpoints() {
    let root = spec_fixture_with_second(true);
    let sb = root.path().join(".specbind");
    let input = evidence(&git(root.path(), &["rev-parse", "HEAD"]));
    assert!(batch::accept(root.path(), &sb, &input).unwrap().succeeded());
    git(root.path(), &["add", ".specbind/specs/checkout/spec.yaml"]);
    git(root.path(), &["commit", "-qm", "First subset"]);
    commit_all(root.path(), "Second subset");
    assert!(batch::verify(root.path(), &sb, &input).is_err());
}
