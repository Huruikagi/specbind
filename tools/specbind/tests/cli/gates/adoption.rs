use super::super::*;

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
            "sb-adopt",
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

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "invalid reverse task plan\n",
    );
    let mut review = specbind_command();
    review
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Reverse contracts are coherent."))
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "CONTRACT_REVIEW_REVERSE_TASKS_FORBIDDEN",
        ));
}

#[test]
fn reverse_finalize_archives_a_baseline_without_creating_a_release() {
    let root = project_fixture();
    commit_all(root.path());
    specbind_command()
        .current_dir(root.path())
        .args(["install", "--with-adoption"])
        .assert()
        .success();
    write_gate_fixture(root.path());
    write_empty_shared_contract(root.path());
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
    specbind_command()
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
        .stdout(
            predicate::str::contains(
                "OK ADOPTION_FINALIZED: Adopted baseline v2.4.0 across 1 specs; no product release was created.",
            )
            .and(predicate::str::contains(
                "Adoption Skill: retired for every configured agent",
            )),
        );

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
    assert!(
        root.path()
            .join(".specbind/specs/shared-contract.yaml")
            .is_file()
    );
    assert!(root.path().join(".specbind/deferred.md").is_file());
    assert!(!root.path().join(".agents/skills/sb-adopt").exists());
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(!config.contains("adoption"), "{config}");
}

#[test]
fn failed_reverse_finalize_retains_the_temporary_adoption_skill() {
    let root = project_fixture();
    commit_all(root.path());
    let mut install_adoption = specbind_command();
    install_adoption
        .current_dir(root.path())
        .args(["install", "--with-adoption"])
        .assert()
        .success();
    commit_all(root.path());

    let mut finalize = specbind_command();
    finalize
        .current_dir(root.path())
        .args(["milestone", "reverse", "finalize"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("NO_ACTIVE_MILESTONE"));

    assert!(
        root.path()
            .join(".agents/skills/sb-adopt/SKILL.md")
            .is_file()
    );
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(config.contains("\"adoption\": true"), "{config}");
}

fn write_empty_shared_contract(root: &Path) {
    write(
        root,
        ".specbind/specs/shared-contract.yaml",
        "schema_version: 1\nresources: []\n",
    );
}
