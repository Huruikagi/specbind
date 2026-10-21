use super::*;
use specbind::replan_completion;

fn recovery_fixture() -> (TempDir, String, String) {
    let root = spec_fixture_with_second(true);
    accept_spec_completion(&root);
    let baseline = git(root.path(), &["rev-parse", "HEAD"]);
    // A committed recovery delta is necessary but not proof of authorization.
    write(root.path(), "recovery.txt", "Reviewed recovery work\n");
    commit_all(root.path(), "recovery checkpoint");
    let recovery = git(root.path(), &["rev-parse", "HEAD"]);
    (root, baseline, recovery)
}

fn check(
    root: &TempDir,
    baseline: &str,
    recovery: &str,
) -> Result<String, replan_completion::ReplanError> {
    replan_completion::check(
        root.path(),
        &root.path().join(".specbind"),
        baseline,
        recovery,
    )
}

#[test]
fn rejects_missing_history_and_preexisting_staleness_without_mutation() {
    let (root, baseline, recovery) = recovery_fixture();
    assert_eq!(
        check(&root, "HEAD~1", &recovery).unwrap_err().code,
        "REPLAN_REVISION_INVALID"
    );
    assert!(check(&root, &"f".repeat(40), &recovery).is_err());
    assert_eq!(
        check(&root, &recovery, &baseline).unwrap_err().code,
        "REPLAN_HISTORY_CHANGED"
    );
    assert_eq!(
        check(&root, &recovery, &recovery).unwrap_err().code,
        "REPLAN_BASELINE_INVALID"
    );
    assert_eq!(git(root.path(), &["status", "--porcelain"]), "");
}

#[test]
fn reports_delta_for_semantic_review_and_requires_exact_withdrawal_checkpoints() {
    let (root, baseline, recovery) = recovery_fixture();
    let result = run_cli(
        root.path(),
        &[
            "milestone",
            "replan",
            "check",
            "--baseline",
            &baseline,
            "--recovery-revision",
            &recovery,
        ],
        None,
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(result["remaining"], serde_json::json!(["checkout"]));
    assert_eq!(
        result["recoveryChangedPaths"],
        serde_json::json!(["recovery.txt"])
    );
    assert_eq!(result["requiresAuthorityAndDeltaReview"], true);
    write(root.path(), "unrelated.txt", "not a withdrawal\n");
    commit_all(root.path(), "unrelated intervening commit");
    fs::remove_file(root.path().join("unrelated.txt")).unwrap();
    commit_all(root.path(), "revert unrelated commit");
    assert_eq!(
        check(&root, &baseline, &recovery).unwrap_err().code,
        "REPLAN_CHECKPOINT_INVALID"
    );
}

#[test]
fn rejects_scope_change_and_earlier_gate_staleness() {
    let (root, baseline, _) = recovery_fixture();
    let roadmap = ".specbind/steering/roadmap.md";
    let original = fs::read_to_string(root.path().join(roadmap)).unwrap();
    write(
        root.path(),
        roadmap,
        &original.replace("Update checkout", "Different scope"),
    );
    commit_all(root.path(), "scope change");
    let changed = git(root.path(), &["rev-parse", "HEAD"]);
    assert_eq!(
        check(&root, &baseline, &changed).unwrap_err().code,
        "REPLAN_SCOPE_CHANGED"
    );
    write(root.path(), roadmap, &original);
    let design = ".specbind/specs/checkout/design.md";
    let original = fs::read_to_string(root.path().join(design)).unwrap();
    write(
        root.path(),
        design,
        &format!("{original}\nUnapproved design change.\n"),
    );
    commit_all(root.path(), "stale design");
    let changed = git(root.path(), &["rev-parse", "HEAD"]);
    assert!(check(&root, &baseline, &changed).is_err());
}

#[test]
fn rejects_changed_requirements_authority_and_batched_withdrawal() {
    let (root, baseline, recovery) = recovery_fixture();
    let specbind = root.path().join(".specbind");
    let source = fs::read_to_string(specbind.join("specs/checkout/spec.yaml")).unwrap();
    let mut wire = serde_json::to_value(runtime::load_spec(&source).unwrap()).unwrap();
    wire["active_change"]["gate_evidence"]["requirements"]["passed_at"] =
        serde_json::json!("2026-10-21T12:00:00Z");
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        &serde_saphyr::to_string(&wire).unwrap(),
    );
    commit_all(root.path(), "change Requirements approval");
    let changed = git(root.path(), &["rev-parse", "HEAD"]);
    assert_eq!(
        check(&root, &baseline, &changed).unwrap_err().code,
        "REPLAN_REQUIREMENTS_CHANGED"
    );
    // Even a correct withdrawal mixed with another path is not a checkpoint.
    write(root.path(), ".specbind/specs/checkout/spec.yaml", &source);
    commit_all(root.path(), "restore test evidence");
    completion::spec_invalidate(root.path(), &specbind, "checkout").unwrap();
    write(root.path(), "other.txt", "extra\n");
    commit_all(root.path(), "mixed withdrawal");
    assert_eq!(
        check(&root, &baseline, &recovery).unwrap_err().code,
        "REPLAN_CHECKPOINT_INVALID"
    );
}

#[test]
fn capture_is_read_only_and_rejects_uncommitted_inputs() {
    let root = accepted_spec_fixture();
    let before = git(root.path(), &["rev-parse", "HEAD"]);
    let result = run_cli(root.path(), &["milestone", "replan", "capture"], None);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(result["baselineRevision"], before);
    assert_eq!(git(root.path(), &["status", "--porcelain"]), "");
    write(root.path(), "partial.txt", "partial work\n");
    assert_eq!(
        replan_completion::capture(root.path(), &root.path().join(".specbind"))
            .unwrap_err()
            .code,
        "REPLAN_WORKTREE_DIRTY"
    );
    assert_eq!(git(root.path(), &["rev-parse", "HEAD"]), before);
}

#[test]
fn prechecks_all_targets_and_resumes_only_ordered_single_target_checkpoints() {
    let root = spec_fixture_with_second(true);
    let specbind = root.path().join(".specbind");
    let revision = git(root.path(), &["rev-parse", "HEAD"]);
    for id in ["checkout", "catalog"] {
        completion::spec_accept(root.path(), &specbind, id, &candidate(&revision)).unwrap();
    }
    commit_all(root.path(), "accept both at the same revision");
    let baseline = git(root.path(), &["rev-parse", "HEAD"]);
    write(root.path(), "repair.txt", "authorized repair\n");
    commit_all(root.path(), "repair");
    let recovery = git(root.path(), &["rev-parse", "HEAD"]);
    let initial: serde_json::Value =
        serde_json::from_str(&check(&root, &baseline, &recovery).unwrap()).unwrap();
    let ids: Vec<String> = initial["remaining"]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| id.as_str().unwrap().to_owned())
        .collect();
    assert_eq!(ids.len(), 2);
    let dirty = format!(".specbind/specs/{}/spec.yaml", ids[1]);
    let original = fs::read_to_string(root.path().join(&dirty)).unwrap();
    write(root.path(), &dirty, &format!("{original}\n# partial\n"));
    assert_eq!(
        check(&root, &baseline, &recovery).unwrap_err().code,
        "REPLAN_WORKTREE_DIRTY"
    );
    write(root.path(), &dirty, &original);
    for (index, id) in ids.iter().enumerate() {
        completion::spec_invalidate(root.path(), &specbind, id).unwrap();
        assert_eq!(
            check(&root, &baseline, &recovery).unwrap_err().code,
            "REPLAN_WORKTREE_DIRTY"
        );
        commit_all(root.path(), "withdraw one completion");
        let progress: serde_json::Value =
            serde_json::from_str(&check(&root, &baseline, &recovery).unwrap()).unwrap();
        assert_eq!(progress["withdrawn"].as_array().unwrap().len(), index + 1);
        assert_eq!(
            progress["remaining"].as_array().unwrap().len(),
            ids.len() - index - 1
        );
    }
}
