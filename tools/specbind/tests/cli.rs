use assert_cmd::Command;
use predicates::prelude::*;
use std::{fs, path::Path, process::Command as ProcessCommand};
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
