use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, path::Path, process::Command as ProcessCommand};
use tempfile::TempDir;

#[test]
fn adoption_preflight_requires_committed_steering() {
    let root = project_fixture();
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR ADOPTION_STEERING_REQUIRED:")
                .and(predicate::str::contains("sb-steering")),
        );
}

#[test]
fn adoption_preflight_returns_the_clean_source_revision() {
    let root = project_fixture();
    write_steering(root.path());
    commit_all(root.path());
    let revision = git_stdout(root.path(), &["rev-parse", "HEAD"]);

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .success()
        .stdout(format!(
            "OK ADOPTION_PREFLIGHT_READY: Existing-project adoption can begin.\n  source_revision: {revision}\n  steering_documents: 1\n"
        ))
        .stderr("");
}

#[test]
fn adoption_preflight_refuses_a_dirty_repository() {
    let root = project_fixture();
    write_steering(root.path());
    commit_all(root.path());
    write(root.path(), "README.md", "uncommitted\n");

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR ADOPTION_WORKTREE_DIRTY:",
        ));
}

#[test]
fn adoption_preflight_is_limited_to_projects_without_specs() {
    let root = project_fixture();
    write_steering(root.path());
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR ADOPTION_SPECS_PRESENT:")
                .and(predicate::str::contains("existing spec: checkout")),
        );
}

#[test]
fn adoption_preflight_refuses_an_orphan_record_from_the_retired_route() {
    let root = project_fixture();
    write_steering(root.path());
    write(
        root.path(),
        ".specbind/adoption/reverse-discovery.yaml",
        "version: 1\nsource_revision: legacy\n",
    );
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR ADOPTION_RECORD_PRESENT:")
                .and(predicate::str::contains("adoption/reverse-discovery.yaml")),
        );
}

#[test]
fn adoption_preflight_resumes_a_clean_reverse_milestone() {
    let root = project_fixture();
    write_steering(root.path());
    write(root.path(), "src/product.txt", "fixed implementation\n");
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write_reverse_checkpoint(root.path(), &baseline);
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("OK ADOPTION_RESUME_READY:")
                .and(predicate::str::contains("milestone_id: 0198b2d1"))
                .and(predicate::str::contains(format!(
                    "source_revision: {baseline}"
                )))
                .and(predicate::str::contains("baseline_version: v2.4.0"))
                .and(predicate::str::contains("stage: requirements"))
                .and(predicate::str::contains("requirements:spec:checkout")),
        )
        .stderr("");

    let output = Command::cargo_bin("specbind")
        .expect("specbind binary should build")
        .current_dir(root.path())
        .args(["milestone", "status", "--json"])
        .output()
        .expect("milestone status runs");
    assert!(output.status.success());
    let status: serde_json::Value = serde_json::from_slice(&output.stdout).expect("status JSON");
    assert_eq!(status["data"]["milestoneKind"], "reverse");
    assert_eq!(status["data"]["actionable"][0]["action"], "requirements");
    assert_eq!(
        status["data"]["actionable"][0]["handler"],
        serde_json::json!({
            "kind": "skill",
            "target": "sb-discovery",
            "mode": "reverse_resume"
        })
    );
}

#[test]
fn adoption_preflight_rejects_a_reverse_record_revision_mismatch() {
    let root = project_fixture();
    write_steering(root.path());
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write_reverse_checkpoint(root.path(), &baseline);
    write(
        root.path(),
        ".specbind/adoption/reverse-discovery.yaml",
        "schema_version: 1\nsource_revision: different\nsuspected_defects: []\n",
    );
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR ADOPTION_RESUME_RECORD_MISMATCH:",
        ));
}

#[test]
fn adoption_preflight_requires_the_reverse_checkpoint_record() {
    let root = project_fixture();
    write_steering(root.path());
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write_reverse_checkpoint(root.path(), &baseline);
    fs::remove_file(
        root.path()
            .join(".specbind/adoption/reverse-discovery.yaml"),
    )
    .expect("remove checkpoint record");
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR ADOPTION_RESUME_RECORD_REQUIRED:",
        ));
}

#[test]
fn adoption_preflight_rejects_source_drift_after_the_reverse_baseline() {
    let root = project_fixture();
    write_steering(root.path());
    write(root.path(), "src/product.txt", "fixed implementation\n");
    commit_all(root.path());
    let baseline = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    write_reverse_checkpoint(root.path(), &baseline);
    write(root.path(), "src/product.txt", "changed implementation\n");
    commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["adoption", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR ADOPTION_RESUME_SOURCE_STALE:",
        ));
}

fn project_fixture() -> TempDir {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    fs::create_dir_all(root.path().join(".specbind/steering"))
        .expect("create SpecBind fixture directories");
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["codex"]}"#,
    );
    root
}

fn write_steering(root: &Path) {
    write(
        root,
        ".specbind/steering/project.md",
        "---\ntype: SpecBind Steering\nartifact_id: project\n---\n# Project guidance\n\nEstablished product, technology, and structure guidance.\n",
    );
}

fn write_reverse_checkpoint(root: &Path, baseline: &str) {
    write(
        root,
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\nbaseline_revision: {baseline}\nbaseline_version: v2.4.0\ntarget_release: null\nwork_items:\n  reverse_specs:\n    - spec: checkout\n      summary: Establish checkout\n---\n# Roadmap\n"
        ),
    );
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It matches the fixed implementation.\n",
    );
    write(
        root,
        ".specbind/specs/checkout/contract.yaml",
        "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
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
            "schema_version: 1\nestablishment:\n  kind: reverse\n  source_revision: {baseline}\n  baseline_version: v2.4.0\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: requirements\n  requirement_ids: null\n"
        ),
    );
    write(
        root,
        ".specbind/adoption/reverse-discovery.yaml",
        &format!("schema_version: 1\nsource_revision: {baseline}\nsuspected_defects: []\n"),
    );
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
