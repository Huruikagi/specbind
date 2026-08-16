use std::{collections::BTreeMap, fmt::Write as _, fs, path::Path, process::Command};

use specbind::{
    domain::{spec::Spec, tasks::Tasks},
    fingerprint::Fingerprint,
    freshness::{self, CompletionRevisionAssessment, CurrentGateInputs, FreshnessStatus},
    schema::runtime,
};

fn implementation_spec(inputs: &CurrentGateInputs) -> Spec {
    let requirements = inputs.requirements.expect("requirements fingerprint");
    let design = inputs.design.as_ref().expect("design fingerprints");
    let tasks = inputs.task_plan.expect("task fingerprint");
    let design_yaml = design
        .iter()
        .fold(String::new(), |mut output, (key, value)| {
            writeln!(output, "        {key}: {value}").expect("writing to a String cannot fail");
            output
        });
    let yaml = format!(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: implementation\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {requirements}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n{design_yaml}    tasks:\n      passed_at: 2026-08-16T12:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        tasks.yaml#plan: {tasks}\n"
    );
    runtime::load_spec(&yaml)
        .expect("fixture is structurally valid")
        .try_into()
        .expect("fixture is semantically valid")
}

fn current_inputs() -> CurrentGateInputs {
    CurrentGateInputs {
        requirements: Some(Fingerprint::markdown(b"requirements\n")),
        design: Some(BTreeMap::from([
            ("contract".to_owned(), Fingerprint::markdown(b"contract\n")),
            ("design/main".to_owned(), Fingerprint::markdown(b"design\n")),
        ])),
        task_plan: Some(Fingerprint::markdown(b"typed task plan projection")),
        ..CurrentGateInputs::default()
    }
}

fn tasks(execution: &str) -> Tasks {
    let yaml = format!(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Implement it\n      requirement_ids: ['1.1']\n{execution}"
    );
    runtime::load_tasks(&yaml)
        .expect("fixture is structurally valid")
        .try_into()
        .expect("fixture is semantically valid")
}

fn release_ready_spec(inputs: &CurrentGateInputs, revision: &str) -> Spec {
    let requirements = inputs.requirements.expect("requirements fingerprint");
    let design = inputs.design.as_ref().expect("design fingerprints");
    let tasks = inputs.task_plan.expect("task fingerprint");
    let design_yaml = design
        .iter()
        .fold(String::new(), |mut output, (key, value)| {
            writeln!(output, "        {key}: {value}").expect("writing to a String cannot fail");
            output
        });
    let yaml = format!(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: release_ready\n  requirement_ids: ['1.1']\n  gate_evidence:\n    requirements:\n      passed_at: 2026-08-16T10:00:00Z\n      approval_mode: explicit\n      approved_requirement_ids: ['1.1']\n      input_revisions:\n        requirements: {requirements}\n    design:\n      passed_at: 2026-08-16T11:00:00Z\n      approval_mode: explicit\n      input_revisions:\n{design_yaml}    tasks:\n      passed_at: 2026-08-16T12:00:00Z\n      approval_mode: explicit\n      input_revisions:\n        tasks.yaml#plan: {tasks}\n    completion:\n      passed_at: 2026-08-16T13:00:00Z\n      implementation_revision: {revision}\n      mechanical_checks:\n        - kind: test\n          command: cargo test\n          exit_code: 0\n"
    );
    runtime::load_spec(&yaml)
        .expect("fixture is structurally valid")
        .try_into()
        .expect("fixture is semantically valid")
}

#[test]
fn reports_all_reached_artifact_gates_fresh() {
    let current = current_inputs();
    let spec = implementation_spec(&current);
    let report = freshness::evaluate(&spec, &current);

    assert_eq!(report.requirements.status, FreshnessStatus::Fresh);
    assert_eq!(report.design.status, FreshnessStatus::Fresh);
    assert_eq!(report.tasks.status, FreshnessStatus::Fresh);
    assert_eq!(report.completion.status, FreshnessStatus::NotReached);
}

#[test]
fn cascades_stale_requirements_through_downstream_gates() {
    let accepted = current_inputs();
    let spec = implementation_spec(&accepted);
    let mut current = accepted;
    current.requirements = Some(Fingerprint::markdown(b"changed requirements\n"));
    let report = freshness::evaluate(&spec, &current);

    assert_eq!(report.requirements.status, FreshnessStatus::Stale);
    assert_eq!(report.design.status, FreshnessStatus::Stale);
    assert_eq!(report.tasks.status, FreshnessStatus::Stale);
    assert!(
        report
            .tasks
            .issues
            .iter()
            .any(|issue| issue.code == "FRESHNESS_PREREQUISITE_STALE")
    );
}

#[test]
fn reports_design_key_set_and_content_changes() {
    let accepted = current_inputs();
    let spec = implementation_spec(&accepted);
    let mut current = accepted;
    let design = current.design.as_mut().expect("design fingerprints");
    design.remove("design/main");
    design.insert(
        "design/storage".to_owned(),
        Fingerprint::markdown(b"storage\n"),
    );
    design.insert(
        "contract".to_owned(),
        Fingerprint::markdown(b"changed contract\n"),
    );
    let report = freshness::evaluate(&spec, &current);

    let codes = report
        .design
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert!(codes.contains(&"FRESHNESS_DESIGN_INPUT_MISSING"));
    assert!(codes.contains(&"FRESHNESS_DESIGN_INPUT_ADDED"));
    assert!(codes.contains(&"FRESHNESS_DESIGN_INPUT_CHANGED"));
    assert_eq!(report.tasks.status, FreshnessStatus::Stale);
}

#[test]
fn keeps_unreached_gates_distinct_from_stale_gates() {
    let wire = runtime::load_spec(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: requirements\n  requirement_ids: null\n",
    )
    .expect("fixture is structurally valid");
    let spec = Spec::try_from(wire).expect("fixture is semantically valid");
    let report = freshness::evaluate(&spec, &CurrentGateInputs::default());

    assert_eq!(report.requirements.status, FreshnessStatus::NotReached);
    assert_eq!(report.design.status, FreshnessStatus::NotReached);
    assert_eq!(report.tasks.status, FreshnessStatus::NotReached);
    assert_eq!(report.completion.status, FreshnessStatus::NotReached);
}

#[test]
fn reports_completion_fresh_when_tasks_and_revision_are_current() {
    let mut current = current_inputs();
    current.tasks = Some(tasks(
        "execution:\n  tasks:\n    '1':\n      status: completed\n",
    ));
    current.completion_revision = Some(CompletionRevisionAssessment { issues: vec![] });
    let spec = release_ready_spec(&current, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    let report = freshness::evaluate(&spec, &current);

    assert_eq!(report.completion.status, FreshnessStatus::Fresh);
    assert!(report.completion.issues.is_empty());
}

#[test]
fn reports_incomplete_and_blocked_tasks_as_stale_completion() {
    let mut current = current_inputs();
    current.tasks = Some(tasks(
        "execution:\n  tasks:\n    '1':\n      status: blocked\n      blocked_reason: Waiting\n",
    ));
    current.completion_revision = Some(CompletionRevisionAssessment { issues: vec![] });
    let spec = release_ready_spec(&current, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    let report = freshness::evaluate(&spec, &current);

    assert_eq!(report.completion.status, FreshnessStatus::Stale);
    assert!(
        report
            .completion
            .issues
            .iter()
            .any(|issue| issue.code == "FRESHNESS_COMPLETION_TASK_BLOCKED")
    );

    current.tasks = Some(tasks(""));
    let report = freshness::evaluate(&spec, &current);
    assert!(
        report
            .completion
            .issues
            .iter()
            .any(|issue| issue.code == "FRESHNESS_COMPLETION_TASK_INCOMPLETE")
    );
}

#[test]
fn accepts_only_the_expected_completion_metadata_successor() {
    let root = tempfile::tempdir().expect("temporary project root");
    let specbind = root.path().join(".specbind");
    let spec_dir = specbind.join("specs/checkout");
    fs::create_dir_all(&spec_dir).expect("create spec directory");
    git(root.path(), &["init"]);
    git(root.path(), &["config", "user.email", "test@example.com"]);
    git(root.path(), &["config", "user.name", "SpecBind Test"]);

    let inputs = current_inputs();
    let baseline = implementation_spec(&inputs);
    write_spec(&spec_dir, &baseline);
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-m", "implementation"]);
    let revision = git(root.path(), &["rev-parse", "HEAD"]);

    let current = release_ready_spec(&inputs, &revision);
    write_spec(&spec_dir, &current);
    let dirty = freshness::assess_completion_revision(root.path(), &specbind, "checkout", &current);
    assert!(dirty.issues.is_empty(), "{:?}", dirty.issues);

    git(root.path(), &["add", ".specbind/specs/checkout/spec.yaml"]);
    git(root.path(), &["commit", "-m", "accept completion"]);
    let committed =
        freshness::assess_completion_revision(root.path(), &specbind, "checkout", &current);
    assert!(committed.issues.is_empty(), "{:?}", committed.issues);

    fs::write(root.path().join("unrelated.txt"), "changed\n").expect("write unrelated file");
    git(root.path(), &["add", "unrelated.txt"]);
    git(root.path(), &["commit", "-m", "unrelated change"]);
    let stale = freshness::assess_completion_revision(root.path(), &specbind, "checkout", &current);
    assert!(
        stale
            .issues
            .iter()
            .any(|issue| issue.code == "FRESHNESS_COMPLETION_PROJECT_CHANGED")
    );

    fs::remove_file(root.path().join("unrelated.txt")).expect("remove unrelated file");
    git(root.path(), &["add", "unrelated.txt"]);
    git(root.path(), &["commit", "-m", "revert unrelated change"]);
    let reverted =
        freshness::assess_completion_revision(root.path(), &specbind, "checkout", &current);
    assert!(
        reverted
            .issues
            .iter()
            .any(|issue| issue.code == "FRESHNESS_COMPLETION_PROJECT_CHANGED")
    );
}

#[test]
fn accepts_multiple_completion_metadata_transitions_at_one_revision() {
    let root = tempfile::tempdir().expect("temporary project root");
    let specbind = root.path().join(".specbind");
    let checkout = specbind.join("specs/checkout");
    let account = specbind.join("specs/account");
    fs::create_dir_all(&checkout).expect("create checkout directory");
    fs::create_dir_all(&account).expect("create account directory");
    git(root.path(), &["init"]);
    git(root.path(), &["config", "user.email", "test@example.com"]);
    git(root.path(), &["config", "user.name", "SpecBind Test"]);

    let inputs = current_inputs();
    let baseline = implementation_spec(&inputs);
    write_spec(&checkout, &baseline);
    write_spec(&account, &baseline);
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-m", "implementation"]);
    let revision = git(root.path(), &["rev-parse", "HEAD"]);

    let checkout_current = release_ready_spec(&inputs, &revision);
    let account_current = release_ready_spec(&inputs, &revision);
    write_spec(&checkout, &checkout_current);
    write_spec(&account, &account_current);
    for (spec, current) in [
        ("checkout", &checkout_current),
        ("account", &account_current),
    ] {
        let assessment =
            freshness::assess_completion_revision(root.path(), &specbind, spec, current);
        assert!(
            assessment.issues.is_empty(),
            "{spec}: {:?}",
            assessment.issues
        );
    }

    git(root.path(), &["add", "."]);
    git(
        root.path(),
        &["commit", "-m", "accept milestone completion"],
    );
    for (spec, current) in [
        ("checkout", &checkout_current),
        ("account", &account_current),
    ] {
        let assessment =
            freshness::assess_completion_revision(root.path(), &specbind, spec, current);
        assert!(
            assessment.issues.is_empty(),
            "{spec}: {:?}",
            assessment.issues
        );
    }
}

fn write_spec(directory: &Path, spec: &Spec) {
    let yaml = serde_saphyr::to_string(spec.as_wire()).expect("serialize spec fixture");
    fs::write(directory.join("spec.yaml"), yaml).expect("write spec fixture");
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
