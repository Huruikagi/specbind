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
    roadmap::{self, DirectStatus},
    schema::{runtime, spec::v1::WorkflowState},
};
use tempfile::TempDir;

const MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

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
            .any(|blocker| blocker == "WORKTREE_NOT_CLEAN")
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

fn spec_fixture() -> TempDir {
    let root = git_fixture();
    let specbind = root.path().join(".specbind");
    let baseline = git(root.path(), &["rev-parse", "HEAD"]);
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
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
    .expect("accept cross-spec review");
    commit_all(root.path(), "accept cross-spec review");

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
