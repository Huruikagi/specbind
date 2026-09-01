use assert_cmd::Command;
use predicates::prelude::*;
use specbind::artifacts::resolve_gate_inputs;
use std::{fmt::Write as _, fs, path::Path, process::Command as ProcessCommand};
use tempfile::TempDir;

#[path = "cli/catalog.rs"]
mod catalog;
#[path = "cli/core.rs"]
mod core;
#[path = "cli/gates.rs"]
mod gates;
#[path = "cli/installation.rs"]
mod installation;
#[path = "cli/milestone.rs"]
mod milestone;
#[path = "cli/project_reads.rs"]
mod project_reads;
#[path = "cli/review.rs"]
mod review;

fn specbind_command() -> Command {
    Command::cargo_bin("specbind").expect("specbind binary should build")
}

fn write_progress_fixture(root: &Path) {
    write(
        root,
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: implementation\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: sha256:0000000000000000000000000000000000000000000000000000000000000000\n"
        ),
    );
    write(
        root,
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: First\n      requirement_ids: ['1.1']\n    - id: '2'\n      kind: task\n      title: Second\n      requirement_ids: ['1.1']\n",
    );
}

fn write_template_fixture(root: &Path) {
    write(
        root,
        ".specbind/settings/templates/specs/brief.md",
        "---\ntype: SpecBind Brief\n---\n<!-- specbind:instruction maintain State the requested outcome. -->\n",
    );
    write(
        root,
        ".specbind/settings/templates/specs/contract.yaml",
        "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
    );
    write(
        root,
        ".specbind/settings/templates/specs/technical-design/main.md",
        "---\ntype: SpecBind Design\nartifact_id: main\n---\n# Design\n\n<!-- specbind:instruction maintain Describe one owned decision. -->\n",
    );
}

fn direct_scope() -> &'static str {
    r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]},"body":"Overview\n\nDirect only.\n"}"#
}

fn create_direct_milestone(root: &Path) {
    let mut command = specbind_command();
    command
        .current_dir(root)
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(direct_scope())
        .assert()
        .success();
}

fn approve_requirements(root: &Path) {
    let mut command = specbind_command();
    command
        .current_dir(root)
        .args([
            "spec",
            "requirements",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
            "--requirement-ids",
            "1.1",
        ])
        .assert()
        .success();
}

fn approve_design(root: &Path) {
    let mut command = specbind_command();
    command
        .current_dir(root)
        .args([
            "spec",
            "design",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success();
}

fn gate_task_fixture() -> &'static str {
    "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['1.1']\n"
}

/// Writes a Spec-backed milestone whose one participating Spec sits in the
/// `requirements` state with no approval evidence yet.
fn write_gate_fixture(root: &Path) {
    write(root, "baseline.txt", "baseline\n");
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
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
        "schema_version: 1\nactive_change: null\n",
    );
    commit_all(root);
    let baseline = git_stdout(root, &["rev-parse", "HEAD"]);
    write(
        root,
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
    );
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
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
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: requirements\n  requirement_ids: null\n"
        ),
    );
}

const REVIEW_MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

fn review_candidate(assessment: &str) -> String {
    format!(r#"{{"schemaVersion":1,"assessment":"{assessment}","deepInputs":[]}}"#)
}

/// Writes a Spec-backed milestone whose one participating Spec sits in `tasks`
/// state with a fresh Design gate and no current `tasks.yaml`.
fn write_review_fixture(root: &Path) {
    write(root, "baseline.txt", "baseline\n");
    commit_all(root);
    let baseline = git_stdout(root, &["rev-parse", "HEAD"]);
    write(
        root,
        ".specbind/steering/roadmap.md",
        &format!(
            "---\ntype: SpecBind Roadmap\nmilestone_id: {REVIEW_MILESTONE}\nbaseline_revision: {baseline}\ntarget_release: null\nwork_items:\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n---\n# Roadmap\n"
        ),
    );
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
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
    let inputs = resolve_gate_inputs(&root.join(".specbind"), "checkout");
    assert!(
        inputs.inventory.issues.is_empty(),
        "{:?}",
        inputs.inventory.issues
    );
    let requirements = inputs
        .inputs
        .requirements
        .expect("requirements fingerprint");
    let design_yaml = inputs
        .inputs
        .design
        .expect("design fingerprints")
        .iter()
        .fold(String::new(), |mut output, (key, value)| {
            writeln!(output, "        {key}: {value}").expect("writing to a String cannot fail");
            output
        });
    write(
        root,
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: {REVIEW_MILESTONE}\n  state: tasks\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {requirements}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n{design_yaml}"
        ),
    );
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

fn task_fixture() -> &'static str {
    "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: group\n      title: Build\n      tasks:\n        - id: '1.1'\n          kind: task\n          title: First\n          requirement_ids: ['1.1']\n        - id: '1.2'\n          kind: task\n          title: Second\n          requirement_ids: ['1.2']\n    - id: '2'\n      kind: task\n      title: Integrate\n      requirement_ids: ['1.3']\n    - id: '3'\n      kind: task\n      title: Document\n      requirement_ids: ['1.4']\n      boundaries: ['docs/']\n      depends_on: ['1.1']\nexecution:\n  tasks:\n    '1.1':\n      status: completed\n    '2':\n      status: blocked\n      blocked_reason: Waiting for an API decision\n"
}

fn write_status_fixture(root: &Path) {
    let specbind = root.join(".specbind");
    write(
        root,
        ".specbind/specs/checkout/requirements.md",
        "---\ntype: SpecBind Requirements\nheading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n# Requirements\n\n### Requirement 1: Checkout\n\n#### Acceptance Criteria\n\n1. It works.\n",
    );
    write(
        root,
        ".specbind/specs/checkout/contract.yaml",
        "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
    );
    write(
        root,
        ".specbind/specs/checkout/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n",
    );
    write(
        root,
        ".specbind/specs/checkout/tasks.yaml",
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Build\n      requirement_ids: ['1.1']\n    - id: '2'\n      kind: task\n      title: Review\n      requirement_ids: ['1.1']\nexecution:\n  tasks:\n    '1':\n      status: completed\n    '2':\n      status: blocked\n      blocked_reason: Waiting for review\n",
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
    let tasks = inputs.inputs.task_plan.expect("task-plan fingerprint");
    let design = inputs.inputs.design.expect("design fingerprints");
    let design_yaml = design
        .iter()
        .fold(String::new(), |mut output, (key, value)| {
            writeln!(output, "        {key}: {value}").expect("writing to a String cannot fail");
            output
        });
    write(
        root,
        ".specbind/specs/checkout/spec.yaml",
        &format!(
            "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: implementation\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {requirements}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n{design_yaml}    tasks:\n      passed_at: 2026-08-16T12:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        tasks.yaml#plan: {tasks}\n"
        ),
    );
}
