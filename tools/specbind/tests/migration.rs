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
fn routes_semantic_findings_to_the_english_agent_guide() {
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
            "https://huruikagi.github.io/specbind/guide/migrate-from-cc-sdd/",
        ))
        .stderr(predicate::str::contains("No files were changed."));

    assert_eq!(before, snapshot(root.path()));
}

#[test]
fn routes_japanese_semantic_findings_to_the_same_english_agent_guide() {
    let root = migration_fixture("guided");
    write_file(
        root.path(),
        ".cc-sdd.json",
        r#"{"version":1,"agent":"claude-code-skills","lang":"ja","kiroDir":".kiro"}"#,
    );
    let checkout_metadata_path = root.path().join(".kiro/specs/checkout/spec.json");
    let checkout_metadata = fs::read_to_string(&checkout_metadata_path)
        .expect("read checkout metadata")
        .replace(r#""language": "en""#, r#""language": "ja""#);
    fs::write(checkout_metadata_path, checkout_metadata).expect("write checkout metadata");
    let before = snapshot(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("ERROR MANUAL_MIGRATION_REQUIRED"))
        .stderr(predicate::str::contains(
            "https://huruikagi.github.io/specbind/guide/migrate-from-cc-sdd/",
        ));

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
        .stdout(predicate::str::contains("Removed legacy root: .kiro"))
        .stderr("");

    assert!(root.path().join(".specbind.json").is_file());
    assert!(root.path().join(".specbind/settings").is_dir());
    assert!(
        root.path()
            .join(".agents/skills/sb-discovery/SKILL.md")
            .is_file()
    );
    assert!(!root.path().join(".agents/skills/kiro-spec-init").exists());
    assert!(!root.path().join(".kiro").exists());
    assert!(!root.path().join(".cc-sdd.json").exists());

    let mut rerun = Command::cargo_bin("specbind").expect("specbind binary should build");
    rerun
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "NO_CHANGE CC_SDD_MIGRATION_COMPLETE",
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
fn accepts_agent_resolution_and_rejoins_deterministic_apply() {
    let root = migration_fixture("minimal");
    write_file(
        root.path(),
        ".kiro/settings/rules/project.md",
        "# Legacy project rule\n",
    );
    git_commit_all(root.path());

    let mut install = Command::cargo_bin("specbind").expect("specbind binary should build");
    install
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "en"])
        .assert()
        .success();
    write_file(
        root.path(),
        ".specbind/settings/rules/project.md",
        "# SpecBind project rule\n",
    );
    git_commit_all(root.path());

    let candidate = r#"{
  "schemaVersion": 1,
  "assessment": "The project-owned rule was reviewed and rewritten for SpecBind.",
  "target": { "language": "en", "agents": ["codex"] },
  "resolutions": [{
    "code": "MIGRATE_RULE_REVIEW_REQUIRED",
    "path": ".kiro/settings/rules",
    "disposition": "converted",
    "targets": [".specbind/settings/rules/project.md"]
  }]
}"#;
    let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
    accept
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--accept-resolution", "-"])
        .write_stdin(candidate)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "OK CC_SDD_MIGRATION_RESOLUTION_ACCEPTED",
        ));
    assert!(
        root.path()
            .join(".specbind/state/cc-sdd-migration.yaml")
            .is_file()
    );

    let mut plan = Command::cargo_bin("specbind").expect("specbind binary should build");
    plan.current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .success()
        .stdout(predicate::str::contains("OK CC_SDD_MIGRATION_PLANNED"))
        .stderr("");

    git_commit_all(root.path());
    let mut apply = Command::cargo_bin("specbind").expect("specbind binary should build");
    apply
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("OK CC_SDD_MIGRATION_APPLIED")
                .and(predicate::str::contains("Removed resolution state: yes")),
        );
    assert!(!root.path().join(".agents/skills/kiro-spec-init").exists());
    assert!(!root.path().join(".kiro").exists());
    assert!(!root.path().join(".cc-sdd.json").exists());
    assert!(
        !root
            .path()
            .join(".specbind/state/cc-sdd-migration.yaml")
            .exists()
    );
}

#[test]
fn apply_refuses_ignored_legacy_files_that_git_cannot_recover() {
    let root = migration_fixture("minimal");
    write_file(root.path(), ".gitignore", ".kiro/settings/local.cache\n");
    git_commit_all(root.path());
    write_file(
        root.path(),
        ".kiro/settings/local.cache",
        "machine-local legacy data\n",
    );
    let before = snapshot(root.path());

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "MIGRATION_CLEANUP_TARGET_UNTRACKED .kiro/settings/local.cache",
        ));

    assert_eq!(before, snapshot(root.path()));
    assert!(!root.path().join(".specbind.json").exists());
}

#[test]
fn changed_resolution_input_restores_the_finding() {
    let root = migration_fixture("minimal");
    write_file(
        root.path(),
        ".kiro/settings/rules/project.md",
        "# Legacy project rule\n",
    );
    git_commit_all(root.path());
    let mut install = Command::cargo_bin("specbind").expect("specbind binary should build");
    install
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "en"])
        .assert()
        .success();
    write_file(
        root.path(),
        ".specbind/settings/rules/project.md",
        "# SpecBind project rule\n",
    );
    git_commit_all(root.path());

    let candidate = r#"{
  "schemaVersion": 1,
  "assessment": "Reviewed.",
  "target": { "language": "en", "agents": ["codex"] },
  "resolutions": [{
    "code": "MIGRATE_RULE_REVIEW_REQUIRED",
    "path": ".kiro/settings/rules",
    "disposition": "converted",
    "targets": [".specbind/settings/rules/project.md"]
  }]
}"#;
    let mut accept = Command::cargo_bin("specbind").expect("specbind binary should build");
    accept
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--accept-resolution", "-"])
        .write_stdin(candidate)
        .assert()
        .success();

    write_file(
        root.path(),
        ".kiro/settings/rules/project.md",
        "# Legacy project rule changed\n",
    );
    let mut plan = Command::cargo_bin("specbind").expect("specbind binary should build");
    plan.current_dir(root.path())
        .args(["migrate", "cc-sdd"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("MIGRATE_RULE_REVIEW_REQUIRED"))
        .stderr(predicate::str::contains("MIGRATE_RESOLUTION_STALE"));
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

#[test]
fn rejects_a_legacy_root_that_could_resolve_to_the_project_root() {
    let root = migration_fixture("minimal");
    write_file(
        root.path(),
        ".cc-sdd.json",
        r#"{"version":1,"agent":"codex-skills","lang":"en","kiroDir":"."}"#,
    );

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["migrate", "cc-sdd", "--apply"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("MIGRATION_LEGACY_CONFIG_INVALID"));
    assert!(root.path().join(".kiro").exists());
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

fn write_file(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("fixture file parent")).expect("create fixture parent");
    fs::write(path, content).expect("write fixture file");
}
