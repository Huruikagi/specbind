use std::{fs, path::Path, process::Command};

use specbind::{
    artifacts::resolve_gate_inputs,
    completion::{
        self, DirectCompleteOutcome, DirectPreflightOutcome, SpecAcceptOutcome,
        SpecInvalidateOutcome, SpecPreflightOutcome,
    },
    cross_spec_review,
    fingerprint::Fingerprint,
    milestone_status::{self, DeliveryStage},
    release_finalize::{self, FinalizeOutcome},
    release_log::{self, LogUpdate},
    release_readiness,
    roadmap::{self, DirectStatus},
    schema::{runtime, spec::v1::WorkflowState},
};
use tempfile::TempDir;
use time::OffsetDateTime;

const MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

#[test]
fn derives_spec_backed_release_readiness_after_accepted_completion_is_committed() {
    let root = accepted_spec_fixture();
    let specbind = root.path().join(".specbind");

    let readiness = release_readiness::resolve(root.path(), &specbind)
        .expect("Spec-backed milestone should be release-ready");
    assert_eq!(readiness.version, "v1.4.0");
    assert_eq!(readiness.specs, vec!["checkout"]);
    assert!(
        readiness
            .mutation_targets
            .iter()
            .any(|target| target.path == "state/contract-review.md")
    );
    assert!(
        readiness
            .mutation_targets
            .iter()
            .any(|target| target.path == "releases/v1.4.0-contract-review.md")
    );

    let milestone = milestone_status::resolve(root.path(), &specbind)
        .expect("milestone status")
        .expect("active milestone");
    assert_eq!(milestone.stage, DeliveryStage::ReleaseReady);
    assert!(milestone.release_blockers.is_empty());
}

#[test]
fn finalizes_a_spec_backed_release_with_log_cleanup_and_archives() {
    let root = accepted_spec_fixture();
    let specbind = root.path().join(".specbind");

    let input =
        r#"{"log_entries":[{"spec":"checkout","summary":"Added authenticated checkout."}]}"#;
    assert_eq!(
        release_finalize::finalize(
            root.path(),
            &specbind,
            specbind::config::ProjectLanguage::En,
            Some(input),
        )
        .expect("finalize release"),
        FinalizeOutcome::Finalized {
            version: "v1.4.0".to_owned(),
            specs: 1,
        }
    );
    assert!(!specbind.join("steering/roadmap.md").exists());
    assert!(specbind.join("releases/v1.4.0-roadmap.md").is_file());
    assert!(
        specbind
            .join("releases/v1.4.0-contract-review.md")
            .is_file()
    );
    assert!(!specbind.join("state/contract-review.md").exists());
    assert!(!specbind.join("specs/checkout/brief.md").exists());
    assert!(!specbind.join("specs/checkout/research.md").exists());
    assert!(!specbind.join("specs/checkout/tasks.yaml").exists());
    assert!(read_spec(&specbind).active_change.0.is_none());
    let log = fs::read_to_string(specbind.join("specs/checkout/log.md")).expect("log");
    assert!(log.starts_with("# Checkout change log\n\n## "));
    assert!(log.contains(
        "* **Release v1.4.0** — Added authenticated checkout. ([roadmap](../../releases/v1.4.0-roadmap.md), milestone `0198b2d1-7c4a-7e31-9f42-8e7c3a110d62`)"
    ));
    assert_eq!(
        release_finalize::finalize(
            root.path(),
            &specbind,
            specbind::config::ProjectLanguage::En,
            Some(input),
        )
        .expect("idempotent finalize retry"),
        FinalizeOutcome::AlreadyFinalized {
            version: "v1.4.0".to_owned(),
            specs: 1,
        }
    );
}

#[test]
fn resumes_an_interrupted_spec_finalization_before_the_roadmap_marker_moves() {
    let root = accepted_spec_fixture();
    let specbind = root.path().join(".specbind");

    let input =
        r#"{"log_entries":[{"spec":"checkout","summary":"Added authenticated checkout."}]}"#;
    let log_path = specbind.join("specs/checkout/log.md");
    let existing = fs::read_to_string(&log_path).expect("log");
    let LogUpdate::Updated(log) = release_log::update_log(
        &existing,
        specbind::config::ProjectLanguage::En,
        OffsetDateTime::now_local().expect("local date").date(),
        "v1.4.0",
        MILESTONE,
        "../../releases/v1.4.0-roadmap.md",
        "Added authenticated checkout.",
        "specs/checkout/log.md",
    )
    .expect("render log") else {
        panic!("new milestone should update log");
    };
    fs::write(log_path, log).expect("simulate completed log step");
    let mut wire = read_spec(&specbind);
    wire.active_change.0 = None;
    let mut idle = serde_saphyr::to_string(&wire).expect("render idle Spec");
    if !idle.ends_with('\n') {
        idle.push('\n');
    }
    fs::write(specbind.join("specs/checkout/spec.yaml"), idle)
        .expect("simulate completed Spec step");
    fs::remove_file(specbind.join("specs/checkout/brief.md")).expect("remove Brief");
    fs::remove_file(specbind.join("specs/checkout/research.md")).expect("remove Research");
    fs::remove_file(specbind.join("specs/checkout/tasks.yaml")).expect("remove Tasks");
    fs::create_dir(specbind.join("releases")).expect("create releases");
    fs::rename(
        specbind.join("state/contract-review.md"),
        specbind.join("releases/v1.4.0-contract-review.md"),
    )
    .expect("simulate completed review archive step");

    assert_eq!(
        release_finalize::finalize(
            root.path(),
            &specbind,
            specbind::config::ProjectLanguage::En,
            Some(input),
        )
        .expect("resume interrupted finalization"),
        FinalizeOutcome::Finalized {
            version: "v1.4.0".to_owned(),
            specs: 1,
        }
    );
    assert!(!specbind.join("steering/roadmap.md").exists());
    assert!(specbind.join("releases/v1.4.0-roadmap.md").is_file());
}

#[test]
fn rejects_missing_spec_log_entries_without_mutation() {
    let root = accepted_spec_fixture();
    let specbind = root.path().join(".specbind");
    let before = fs::read_to_string(specbind.join("steering/roadmap.md")).expect("Roadmap");

    let error = release_finalize::finalize(
        root.path(),
        &specbind,
        specbind::config::ProjectLanguage::En,
        None,
    )
    .expect_err("Spec-backed finalization requires log entries");
    assert_eq!(error.issues[0].code, "LOG_ENTRIES_REQUIRED");
    assert_eq!(
        fs::read_to_string(specbind.join("steering/roadmap.md")).expect("Roadmap"),
        before
    );
    assert!(specbind.join("specs/checkout/brief.md").exists());
    assert!(specbind.join("specs/checkout/tasks.yaml").exists());
}

#[test]
fn release_preflight_reports_an_invalid_existing_log_profile() {
    let root = spec_fixture();
    write(
        root.path(),
        ".specbind/specs/checkout/log.md",
        "# Log\n\n## not-a-date\n\n* Entry.\n",
    );
    commit_all(root.path(), "malformed existing log");
    accept_spec_completion(&root);
    let specbind = root.path().join(".specbind");

    let error = release_readiness::resolve(root.path(), &specbind)
        .expect_err("invalid log profile must block preflight");
    assert!(
        error
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "LOG_PROFILE_INVALID")
    );
}

#[test]
fn creates_a_missing_log_with_the_project_language_wrapper() {
    let root = spec_fixture();
    let specbind = root.path().join(".specbind");
    fs::remove_file(specbind.join("specs/checkout/log.md")).expect("remove initial log");
    commit_all(root.path(), "leave log for release finalization");
    accept_spec_completion(&root);

    let input =
        r#"{"log_entries":[{"spec":"checkout","summary":"認証済みチェックアウトを追加した。"}]}"#;
    release_finalize::finalize(
        root.path(),
        &specbind,
        specbind::config::ProjectLanguage::Ja,
        Some(input),
    )
    .expect("finalize release with a newly created Japanese log");
    let log = fs::read_to_string(specbind.join("specs/checkout/log.md")).expect("log");
    assert!(log.starts_with("# スペック更新ログ\n\n## "));
    assert!(log.contains("**リリース v1.4.0**"));
    assert!(log.contains("[ロードマップ]"));
}

#[test]
fn accepts_invalidates_and_idempotently_reports_spec_completion() {
    let root = spec_fixture();
    let specbind = root.path().join(".specbind");
    let preflight = completion::spec_preflight(root.path(), &specbind, "checkout")
        .expect("completion preflight");
    let SpecPreflightOutcome::Ready {
        implementation_revision,
    } = preflight
    else {
        panic!("implementation should require validation");
    };
    let candidate = candidate(&implementation_revision);

    assert_eq!(
        completion::spec_accept(root.path(), &specbind, "checkout", &candidate)
            .expect("accept completion"),
        SpecAcceptOutcome::Accepted {
            implementation_revision: implementation_revision.clone()
        }
    );
    let wire = read_spec(&specbind);
    let active = wire.active_change.0.as_ref().expect("active change");
    assert_eq!(active.state, WorkflowState::ReleaseReady);
    assert_eq!(
        active
            .gate_evidence
            .as_ref()
            .and_then(|evidence| evidence.completion.as_ref())
            .expect("completion evidence")
            .implementation_revision
            .0,
        implementation_revision
    );
    let milestone = milestone_status::resolve(root.path(), &specbind)
        .expect("milestone status")
        .expect("active milestone");
    assert_eq!(milestone.stage, DeliveryStage::ReleasePending);
    assert_eq!(
        milestone.current_revision.as_deref(),
        Some(implementation_revision.as_str())
    );
    assert!(
        milestone
            .release_blockers
            .iter()
            .any(|blocker| blocker == "RELEASE_TARGET_DIRTY")
    );
    assert!(matches!(
        completion::spec_accept(root.path(), &specbind, "checkout", &candidate)
            .expect("idempotent acceptance"),
        SpecAcceptOutcome::AlreadyAccepted { .. }
    ));
    let dirty_error = completion::spec_invalidate(root.path(), &specbind, "checkout")
        .expect_err("uncommitted spec target must not be overwritten");
    assert!(
        dirty_error
            .issues
            .iter()
            .any(|issue| issue.code == "SPEC_COMPLETION_TARGET_DIRTY")
    );

    commit_all(root.path(), "accept completion");
    write(root.path(), "src.txt", "later implementation change\n");
    commit_all(root.path(), "later implementation");
    assert_eq!(
        completion::spec_invalidate(root.path(), &specbind, "checkout")
            .expect("invalidate stale completion"),
        SpecInvalidateOutcome::Invalidated
    );
    let wire = read_spec(&specbind);
    let active = wire.active_change.0.as_ref().expect("active change");
    assert_eq!(active.state, WorkflowState::Implementation);
    assert!(
        active
            .gate_evidence
            .as_ref()
            .and_then(|evidence| evidence.completion.as_ref())
            .is_none()
    );
}

#[test]
fn rejects_invalid_completion_candidates_without_mutation() {
    let root = spec_fixture();
    let specbind = root.path().join(".specbind");
    let before = fs::read_to_string(specbind.join("specs/checkout/spec.yaml")).expect("spec");
    let error = completion::spec_accept(
        root.path(),
        &specbind,
        "checkout",
        r#"{"schemaVersion":1,"implementationRevision":"HEAD","mechanicalChecks":[]}"#,
    )
    .expect_err("invalid candidate");

    assert!(
        error
            .issues
            .iter()
            .all(|issue| issue.code == "COMPLETION_EVIDENCE_INVALID")
    );
    assert_eq!(
        fs::read_to_string(specbind.join("specs/checkout/spec.yaml")).expect("spec"),
        before
    );
}

#[test]
fn preflights_completes_and_idempotently_reports_a_direct_item() {
    let root = direct_fixture();
    let specbind = root.path().join(".specbind");
    let preflight =
        completion::direct_preflight(root.path(), &specbind, "docs").expect("Direct preflight");
    let DirectPreflightOutcome::Ready {
        implementation_revision,
    } = preflight
    else {
        panic!("pending Direct item should be ready");
    };

    assert_eq!(
        completion::direct_complete(root.path(), &specbind, "docs", &implementation_revision)
            .expect("complete Direct item"),
        DirectCompleteOutcome::Recorded
    );
    let content = fs::read_to_string(specbind.join("steering/roadmap.md")).expect("Roadmap");
    assert!(content.ends_with("# Direct milestone\n"));
    let roadmap = roadmap::parse(&content).expect("valid mutated Roadmap");
    assert_eq!(
        roadmap.direct_changes[0].status,
        Some(DirectStatus::Completed)
    );
    assert_eq!(
        completion::direct_complete(root.path(), &specbind, "docs", &implementation_revision)
            .expect("idempotent Direct completion"),
        DirectCompleteOutcome::AlreadyCompleted
    );
    assert_eq!(
        completion::direct_preflight(root.path(), &specbind, "docs")
            .expect("completed Direct preflight"),
        DirectPreflightOutcome::AlreadyCompleted
    );
}

fn accepted_spec_fixture() -> TempDir {
    let root = spec_fixture();
    accept_spec_completion(&root);
    root
}

fn accept_spec_completion(root: &TempDir) {
    let specbind = root.path().join(".specbind");
    let preflight = completion::spec_preflight(root.path(), &specbind, "checkout")
        .expect("completion preflight");
    let SpecPreflightOutcome::Ready {
        implementation_revision,
    } = preflight
    else {
        panic!("implementation should require validation");
    };
    completion::spec_accept(
        root.path(),
        &specbind,
        "checkout",
        &candidate(&implementation_revision),
    )
    .expect("accept completion");
    commit_all(root.path(), "accept completion");
}

fn spec_fixture() -> TempDir {
    let root = git_fixture();
    let specbind = root.path().join(".specbind");
    let baseline = git(root.path(), &["rev-parse", "HEAD"]);
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: v1.4.0\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
    );
    write(
        root.path(),
        ".specbind/specs/checkout/brief.md",
        "---\ntype: SpecBind Brief\n---\n# Checkout brief\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/research.md",
        "---\ntype: SpecBind Research\n---\n# Checkout research\n",
    );
    write(
        root.path(),
        ".specbind/specs/checkout/log.md",
        "# Checkout change log\n",
    );
    let requirements = "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n";
    let contract = "---\ntype: SpecBind Contract\n---\n# Contract\n\n## Owns\n\n## Exports\n\n## Consumes\n\n## Invariants\n\n## File Ownership\n";
    let design = "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Design\n\n_Requirements: 1.1_\n";
    write(
        root.path(),
        ".specbind/specs/checkout/requirements.md",
        requirements,
    );
    write(
        root.path(),
        ".specbind/specs/checkout/contract.md",
        contract,
    );
    write(root.path(), ".specbind/specs/checkout/design.md", design);
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {MILESTONE}\n  state: tasks\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        contract: {}\n        design/main: {}\n",
            Fingerprint::markdown(requirements.as_bytes()),
            Fingerprint::markdown(contract.as_bytes()),
            Fingerprint::markdown(design.as_bytes()),
        ),
    );
    commit_all(root.path(), "approved design");
    cross_spec_review::accept(
        root.path(),
        &specbind,
        r#"{"schemaVersion":1,"assessment":"Compatible.","deepInputs":[]}"#,
    )
    .expect("accept contract review");
    commit_all(root.path(), "accept contract review");

    write(
        root.path(),
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['1.1']\nexecution:\n  tasks:\n    '1':\n      status: completed\n",
    );
    let inputs = resolve_gate_inputs(&specbind, "checkout");
    assert!(
        inputs.inventory.issues.is_empty(),
        "{:?}",
        inputs.inventory.issues
    );
    let requirements = inputs
        .inputs
        .requirements
        .expect("requirements fingerprint");
    let design = inputs.inputs.design.expect("design fingerprints");
    let tasks = inputs.inputs.task_plan.expect("task fingerprint");
    write(
        root.path(),
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {MILESTONE}\n  state: implementation\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {requirements}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        contract: {}\n        design/main: {}\n    tasks:\n      passed_at: 2026-08-16T12:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        tasks.yaml#plan: {tasks}\n",
            design["contract"], design["design/main"]
        ),
    );
    commit_all(root.path(), "complete implementation");
    root
}

fn direct_fixture() -> TempDir {
    let root = git_fixture();
    let baseline = git(root.path(), &["rev-parse", "HEAD"]);
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  direct_changes:\n    - id: docs\n      summary: Update docs\n---\n# Direct milestone\n"
        ),
    );
    commit_all(root.path(), "Direct implementation");
    root
}

fn git_fixture() -> TempDir {
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
    root
}

fn candidate(revision: &str) -> String {
    format!(
        r#"{{"schemaVersion":1,"implementationRevision":"{revision}","mechanicalChecks":[{{"kind":"test","command":"cargo test","exitCode":0,"workingDirectory":"tools/specbind"}}]}}"#
    )
}

fn read_spec(specbind: &Path) -> specbind::schema::spec::v1::SpecDocument {
    let input = fs::read_to_string(specbind.join("specs/checkout/spec.yaml")).expect("spec.yaml");
    runtime::load_spec(&input).expect("valid spec.yaml")
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
    let output = Command::new("git")
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
