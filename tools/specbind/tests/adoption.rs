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
                .and(predicate::str::contains("specbind-steering")),
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
