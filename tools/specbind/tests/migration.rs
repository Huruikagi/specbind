use assert_cmd::Command;
use predicates::prelude::*;
use std::{collections::BTreeMap, fs, path::Path, process::Command as ProcessCommand};
use tempfile::TempDir;

#[test]
fn plans_a_minimal_cc_sdd_project_without_writes() {
    let root = migration_fixture("minimal");
    let before = snapshot(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK CC_SDD_MIGRATION_PLANNED"))
        .stdout(predicate::str::contains(
            "source=.cc-sdd.json target=.specbind.json",
        ))
        .stdout(predicate::str::contains("Original").not())
        .stdout(predicate::str::contains("No files were changed."))
        .stderr("");

    assert_eq!(before, snapshot(root.path()));
    assert!(!root.path().join(".specbind.json").exists());
}

#[test]
fn routes_semantic_findings_to_the_neutral_agent_guide() {
    let root = migration_fixture("guided");
    let before = snapshot(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("ERROR MANUAL_MIGRATION_REQUIRED"))
        .stderr(predicate::str::contains("MIGRATE_ACTIVE_SCOPE_AMBIGUOUS"))
        .stderr(predicate::str::contains(
            "MIGRATE_DESIGN_TRACEABILITY_REQUIRED",
        ))
        .stderr(predicate::str::contains("MIGRATE_LANGUAGE_MIXED"))
        .stderr(predicate::str::contains("MIGRATE_RULE_REVIEW_REQUIRED"))
        .stderr(predicate::str::contains(
            "https://huruikagi.github.io/specbind/guide/migration/cc-sdd/",
        ))
        .stderr(predicate::str::contains("No files were changed."));

    assert_eq!(before, snapshot(root.path()));
}

#[test]
fn apply_requires_a_clean_committed_recovery_boundary() {
    let root = migration_fixture("minimal");
    let before = snapshot(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ERROR MIGRATION_APPLY_FAILED"))
        .stderr(predicate::str::contains("MIGRATION_COMMIT_REQUIRED"));

    assert_eq!(before, snapshot(root.path()));
}

#[test]
fn applies_and_rejoins_an_exact_deterministic_migration() {
    let root = migration_fixture("minimal");
    git_commit_all(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK CC_SDD_MIGRATION_APPLIED"))
        .stdout(predicate::str::contains("Removed legacy assets: 1"))
        .stdout(predicate::str::contains(
            "Original .kiro tree remains intact.",
        ))
        .stderr("");

    assert!(root.path().join(".specbind.json").is_file());
    assert!(root.path().join(".specbind/settings").is_dir());
    assert!(
        root.path()
            .join(".agents/skills/specbind-discovery/SKILL.md")
            .is_file()
    );
    assert!(!root.path().join(".agents/skills/kiro-spec-init").exists());
    assert!(root.path().join(".kiro").is_dir());

    let mut rerun = Command::cargo_bin("specbind").expect("specbind binary should build");
    rerun
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "NO_CHANGE CC_SDD_MIGRATION_UP_TO_DATE",
        ))
        .stderr("");
}

#[test]
fn apply_refuses_unrelated_changes_without_writing() {
    let root = migration_fixture("minimal");
    git_commit_all(root.path());
    fs::write(root.path().join("unrelated.txt"), "user work\n").expect("write unrelated change");
    let before = snapshot(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("MIGRATION_REPOSITORY_DIRTY"));

    assert_eq!(before, snapshot(root.path()));
    assert!(!root.path().join(".specbind.json").exists());
}

#[test]
fn unknown_kiro_prefixed_agent_assets_require_guided_review() {
    let root = migration_fixture("minimal");
    fs::create_dir_all(root.path().join(".agents/skills/kiro-project-custom"))
        .expect("create unknown legacy skill");
    fs::write(
        root.path()
            .join(".agents/skills/kiro-project-custom/SKILL.md"),
        "# Project-owned legacy workflow\n",
    )
    .expect("write unknown legacy skill");

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "MIGRATE_LEGACY_AGENT_ASSET_UNKNOWN",
        ));
}

#[test]
fn existing_nonconverged_target_is_never_overwritten() {
    let root = migration_fixture("minimal");
    fs::create_dir_all(root.path().join(".specbind")).expect("create conflicting target root");
    fs::write(root.path().join(".specbind.json"), "{}\n").expect("write conflicting config");
    let before = snapshot(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("MIGRATE_TARGET_ALREADY_EXISTS"));

    assert_eq!(before, snapshot(root.path()));
}

#[test]
fn does_not_invent_a_ready_state_invariant_that_cc_sdd_never_maintained() {
    let root = migration_fixture("minimal");
    fs::create_dir_all(root.path().join(".kiro/specs/checkout"))
        .expect("create legacy Spec directory");
    fs::write(
        root.path().join(".kiro/specs/checkout/spec.json"),
        r#"{
  "language":"en",
  "phase":"tasks-generated",
  "approvals":{
    "requirements":{"generated":true,"approved":true},
    "design":{"generated":true,"approved":true},
    "tasks":{"generated":true,"approved":true}
  },
  "ready_for_implementation":false
}"#,
    )
    .expect("write historical state regression fixture");
    for artifact in ["requirements.md", "design.md", "tasks.md"] {
        fs::write(
            root.path().join(".kiro/specs/checkout").join(artifact),
            "# Legacy artifact\n",
        )
        .expect("write legacy artifact");
    }

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "MIGRATE_DESIGN_TRACEABILITY_REQUIRED",
        ))
        .stderr(predicate::str::contains("MIGRATE_SPEC_STATE_INVALID").not());
}

#[test]
fn reports_the_historical_default_root_when_cc_sdd_is_absent() {
    let root = tempfile::tempdir().expect("temporary project root");
    git_init(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ERROR MIGRATION_PLAN_FAILED"))
        .stderr(predicate::str::contains(
            "MIGRATION_LEGACY_ROOT_NOT_FOUND .kiro:",
        ));
}

fn migration_fixture(name: &str) -> TempDir {
    let root = tempfile::tempdir().expect("temporary project root");
    git_init(root.path());
    copy_tree(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/migration")
            .join(name),
        root.path(),
    );
    root
}

fn copy_tree(source: &Path, target: &Path) {
    for entry in fs::read_dir(source).expect("read fixture directory") {
        let entry = entry.expect("read fixture entry");
        let destination = target.join(entry.file_name());
        if entry.file_type().expect("read fixture type").is_dir() {
            fs::create_dir_all(&destination).expect("create fixture directory");
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).expect("copy fixture file");
        }
    }
}

fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut files = BTreeMap::new();
    collect_files(root, root, &mut files);
    files
}

fn collect_files(root: &Path, directory: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
    for entry in fs::read_dir(directory).expect("read snapshot directory") {
        let entry = entry.expect("read snapshot entry");
        if entry.file_name() == ".git" {
            continue;
        }
        if entry.file_type().expect("read snapshot type").is_dir() {
            collect_files(root, &entry.path(), files);
        } else {
            let relative = entry
                .path()
                .strip_prefix(root)
                .expect("fixture-relative path")
                .to_string_lossy()
                .replace('\\', "/");
            files.insert(
                relative,
                fs::read(entry.path()).expect("read snapshot file"),
            );
        }
    }
}

fn git_init(root: &Path) {
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(root)
        .arg("init")
        .output()
        .expect("start Git");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_commit_all(root: &Path) {
    git(root, &["add", "--all"]);
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
