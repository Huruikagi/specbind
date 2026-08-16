use std::{collections::BTreeMap, fmt::Write as _};

use specbind::{
    domain::spec::Spec,
    fingerprint::Fingerprint,
    freshness::{self, CurrentGateInputs, FreshnessStatus},
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
    }
}

#[test]
fn reports_all_reached_artifact_gates_fresh() {
    let current = current_inputs();
    let spec = implementation_spec(&current);
    let report = freshness::evaluate(&spec, &current);

    assert_eq!(report.requirements.status, FreshnessStatus::Fresh);
    assert_eq!(report.design.status, FreshnessStatus::Fresh);
    assert_eq!(report.tasks.status, FreshnessStatus::Fresh);
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
}
