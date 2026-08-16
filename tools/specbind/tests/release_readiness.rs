use std::{fs, path::Path, process::Command as ProcessCommand};

use assert_cmd::Command;
use predicates::prelude::*;
use specbind::{
    config::ProjectLanguage,
    milestone_status::{self, DeliveryStage},
    release_finalize::{self, FinalizeOutcome},
    release_readiness::{self, MutationTargetState},
};
use tempfile::TempDir;

const MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

#[test]
fn exposes_release_preflight_through_the_cli() {
    let root = direct_fixture("v1.4.0", true);
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");

    command
        .current_dir(root.path())
        .args(["release", "preflight"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "OK RELEASE_READY: Release v1.4.0 is ready for project release work across 0 specs.\n",
        ))
        .stdout(predicate::str::contains("Milestone ID: 0198b2d1"))
        .stdout(predicate::str::contains(
            "absent releases/v1.4.0-roadmap.md",
        ))
        .stderr("");
}

#[test]
fn cli_reports_no_active_milestone_with_a_stable_code() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init", "--quiet"]);
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["codex"]}"#,
    );
    fs::create_dir(root.path().join(".specbind")).expect("create SpecBind root");

    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["release", "preflight"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR NO_ACTIVE_MILESTONE: Release preflight failed.\n",
        ));
}

#[test]
fn accepts_a_complete_direct_only_milestone_and_ignores_unrelated_dirt() {
    let root = direct_fixture("v1.4.0", true);
    write(root.path(), "notes.txt", "unrelated untracked work\n");

    let readiness = release_readiness::resolve(root.path(), &root.path().join(".specbind"))
        .expect("release should be ready");

    assert_eq!(readiness.milestone_id, MILESTONE);
    assert_eq!(readiness.version, "v1.4.0");
    assert!(readiness.specs.is_empty());
    assert_eq!(readiness.direct_changes, 1);
    assert_eq!(readiness.mutation_targets.len(), 2);
    assert!(readiness.mutation_targets.iter().any(|target| {
        target.path == "steering/roadmap.md" && target.state == MutationTargetState::Existing
    }));
    assert!(readiness.mutation_targets.iter().any(|target| {
        target.path == "releases/v1.4.0-roadmap.md" && target.state == MutationTargetState::Absent
    }));
    assert!(
        !readiness
            .mutation_targets
            .iter()
            .any(|target| target.path.contains("cross-spec-review"))
    );

    let status = milestone_status::resolve(root.path(), &root.path().join(".specbind"))
        .expect("milestone status")
        .expect("active milestone");
    assert_eq!(status.stage, DeliveryStage::ReleaseReady);
    assert!(status.release_blockers.is_empty());
}

#[test]
fn finalizes_a_direct_only_release_without_log_input_or_review_archive() {
    let root = direct_fixture("v1.4.0", true);
    let specbind = root.path().join(".specbind");

    assert_eq!(
        release_finalize::finalize(root.path(), &specbind, ProjectLanguage::En, None)
            .expect("finalize Direct-only release"),
        FinalizeOutcome::Finalized {
            version: "v1.4.0".to_owned(),
            specs: 0,
        }
    );
    assert!(!specbind.join("steering/roadmap.md").exists());
    assert!(specbind.join("releases/v1.4.0-roadmap.md").is_file());
    assert!(
        !specbind
            .join("releases/v1.4.0-cross-spec-review.md")
            .exists()
    );
    assert_eq!(
        release_finalize::finalize(root.path(), &specbind, ProjectLanguage::En, None)
            .expect("idempotent Direct-only finalize retry"),
        FinalizeOutcome::AlreadyFinalized {
            version: "v1.4.0".to_owned(),
            specs: 0,
        }
    );
}

#[test]
fn exposes_direct_only_release_finalization_through_the_cli() {
    let root = direct_fixture("v1.4.0", true);
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command
        .current_dir(root.path())
        .args(["release", "finalize"])
        .assert()
        .success()
        .stdout("OK RELEASE_FINALIZED: Finalized v1.4.0 for 0 specs.\n")
        .stderr("");

    let mut retry = Command::cargo_bin("specbind").expect("specbind binary should build");
    retry
        .current_dir(root.path())
        .args(["release", "finalize"])
        .assert()
        .success()
        .stdout("NO_CHANGE RELEASE_ALREADY_FINALIZED: Release v1.4.0 is already finalized.\n")
        .stderr("");
}

#[test]
fn reports_incomplete_direct_work_and_dirty_mutation_targets() {
    let incomplete = direct_fixture("v1.4.0", false);
    let error = release_readiness::resolve(incomplete.path(), &incomplete.path().join(".specbind"))
        .expect_err("pending Direct work must block release");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "RELEASE_DIRECT_INCOMPLETE")
    );

    let dirty = direct_fixture("v1.4.0", true);
    let current =
        fs::read_to_string(dirty.path().join(".specbind/steering/roadmap.md")).expect("Roadmap");
    write(
        dirty.path(),
        ".specbind/steering/roadmap.md",
        &format!("{current}\nDirty note.\n"),
    );
    let error = release_readiness::resolve(dirty.path(), &dirty.path().join(".specbind"))
        .expect_err("dirty Roadmap must block release");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "RELEASE_TARGET_DIRTY")
    );
}

#[test]
fn reports_archive_collisions_without_overwrite_authority() {
    let root = direct_fixture("v1.4.0", true);
    write(
        root.path(),
        ".specbind/releases/V1.4.0-ROADMAP.MD",
        "occupied\n",
    );

    let error = release_readiness::resolve(root.path(), &root.path().join(".specbind"))
        .expect_err("case-insensitive archive collision must block release");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "RELEASE_ARCHIVE_COLLISION")
    );
}

fn direct_fixture(version: &str, completed: bool) -> TempDir {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init", "--quiet"]);
    git(
        root.path(),
        &["config", "user.email", "specbind@example.com"],
    );
    git(root.path(), &["config", "user.name", "SpecBind Tests"]);
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["codex"]}"#,
    );
    write(root.path(), "baseline.txt", "baseline\n");
    commit_all(root.path(), "baseline");
    let baseline = git(root.path(), &["rev-parse", "HEAD"]);
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &roadmap(version, completed, &baseline),
    );
    commit_all(root.path(), "complete Direct milestone");
    root
}

fn roadmap(version: &str, completed: bool, baseline: &str) -> String {
    let status = if completed {
        "\n      status: completed"
    } else {
        ""
    };
    format!(
        "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: {version}\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs{status}\n---\n# Roadmap\n"
    )
}

fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create fixture parent");
    }
    fs::write(path, content).expect("write fixture");
}

fn commit_all(root: &Path, message: &str) {
    git(root, &["add", "."]);
    git(root, &["commit", "--quiet", "-m", message]);
}

fn git(root: &Path, arguments: &[&str]) -> String {
    let output = ProcessCommand::new("git")
        .current_dir(root)
        .args(arguments)
        .output()
        .expect("run Git");
    assert!(
        output.status.success(),
        "git {} failed: {}",
        arguments.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("UTF-8 Git output")
        .trim()
        .to_owned()
}
