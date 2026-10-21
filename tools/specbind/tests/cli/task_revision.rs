use super::*;
use serde_json::{Value, json};

const TASKS_PATH: &str = ".specbind/specs/checkout/tasks.yaml";

#[test]
fn revision_repairs_retained_inactive_coverage_without_requiring_old_traceability() {
    let root = setup();
    let source = fs::read_to_string(root.path().join(TASKS_PATH)).unwrap();
    let mut old: Value = specbind::yaml::parse(&source).unwrap();
    old["plan"]["items"][2]["requirement_ids"] = json!(["9.9"]);
    write(root.path(), TASKS_PATH, &old.to_string());
    let mut input = candidate(root.path());
    input["plan"]["items"].as_array_mut().unwrap().pop();
    input["progress"][2] =
        json!({"action":"remove","from":"3","reason":"Historical work outside current scope"});
    apply(root.path(), &input, false).success();
    let current = snapshot(root.path());
    assert_eq!(
        current["current"]["execution"]["tasks"]
            .as_object()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        current["current"]["execution"]["tasks"]["1"]["status"],
        "completed"
    );
}

#[test]
fn revision_rejects_duplicate_json_and_nonregular_source() {
    let root = setup();
    let input = candidate(root.path());
    let before = fs::read(root.path().join(TASKS_PATH)).unwrap();
    let duplicate = input.to_string().replacen('{', "{\"schemaVersion\":1,", 1);
    specbind_command()
        .current_dir(root.path())
        .args(["tasks", "revision", "apply", "checkout", "--input", "-"])
        .write_stdin(duplicate)
        .assert()
        .failure()
        .stderr(predicate::str::contains("TASK_REVISION_INPUT_INVALID"));
    assert_eq!(before, fs::read(root.path().join(TASKS_PATH)).unwrap());
    fs::remove_file(root.path().join(TASKS_PATH)).unwrap();
    fs::create_dir(root.path().join(TASKS_PATH)).unwrap();
    apply(root.path(), &input, false)
        .failure()
        .stderr(predicate::str::contains("TASK_REVISION_TARGET_INVALID"));
    assert!(root.path().join(TASKS_PATH).is_dir());
}

#[test]
fn revision_rejects_changed_governing_state_and_wrong_spec() {
    let root = setup();
    let input = candidate(root.path());
    let spec_path = ".specbind/specs/checkout/spec.yaml";
    let source = fs::read_to_string(root.path().join(spec_path)).unwrap();
    write(
        root.path(),
        spec_path,
        &format!("{source}\n# state reread required\n"),
    );
    assert_unchanged(root.path(), &input, "TASK_REVISION_SOURCE_CHANGED");
    specbind_command()
        .current_dir(root.path())
        .args(["tasks", "revision", "preflight", "../checkout"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("TASK_REVISION_TARGET_INVALID"));
}

fn setup() -> TempDir {
    let root = project_fixture();
    write_review_fixture(root.path());
    specbind_command()
        .current_dir(root.path())
        .args(["milestone", "review", "accept", "--candidate", "-"])
        .write_stdin(review_candidate("Compatible"))
        .assert()
        .success();
    let plan = json!({"schema_version": 1, "plan": {"items": [
        {"id":"1","kind":"task","title":"First","requirement_ids":["1.1"]},
        {"id":"2","kind":"task","title":"Second","requirement_ids":["1.1"]},
        {"id":"3","kind":"task","title":"Third","requirement_ids":["1.1"]}
    ]}});
    write(root.path(), TASKS_PATH, &plan.to_string());
    specbind_command()
        .current_dir(root.path())
        .args([
            "spec",
            "tasks",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success();
    specbind_command()
        .current_dir(root.path())
        .args(["tasks", "complete", "checkout", "1"])
        .assert()
        .success();
    for id in ["2", "3"] {
        specbind_command()
            .current_dir(root.path())
            .args([
                "tasks",
                "block",
                "checkout",
                id,
                "--reason",
                "Waiting on API",
            ])
            .assert()
            .success();
    }
    commit_all(root.path());
    specbind_command()
        .current_dir(root.path())
        .args(["spec", "tasks", "invalidate", "checkout"])
        .assert()
        .success();
    root
}

fn snapshot(root: &Path) -> Value {
    let output = specbind_command()
        .current_dir(root)
        .args(["tasks", "revision", "preflight", "checkout"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&output).unwrap()
}

fn candidate(root: &Path) -> Value {
    let snapshot = snapshot(root);
    json!({"schemaVersion":1, "sourceFingerprint":snapshot["sourceFingerprint"],
    "plan":snapshot["current"]["plan"], "progress":[
        {"from":"1","to":"1","action":"preserve","reason":"Same obligation and evidence"},
        {"from":"2","to":"2","action":"preserve","reason":"Same blocked obligation"},
        {"from":"3","to":"3","action":"preserve","reason":"Same blocked obligation"}
    ]})
}

fn apply(root: &Path, candidate: &Value, dry_run: bool) -> assert_cmd::assert::Assert {
    let mut command = specbind_command();
    command
        .current_dir(root)
        .args(["tasks", "revision", "apply", "checkout", "--input", "-"]);
    if dry_run {
        command.arg("--dry-run");
    }
    command.write_stdin(candidate.to_string()).assert()
}

fn assert_unchanged(root: &Path, candidate: &Value, code: &str) {
    let before = fs::read(root.join(TASKS_PATH)).unwrap();
    apply(root, candidate, false)
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(code));
    assert_eq!(fs::read(root.join(TASKS_PATH)).unwrap(), before);
}

#[test]
fn revision_renumbers_resets_and_removes_without_changing_gates() {
    let root = setup();
    let mut input = candidate(root.path());
    input["plan"]["items"].as_array_mut().unwrap().pop();
    input["plan"]["items"][0]["title"] = json!("Revised second");
    input["plan"]["items"][1]["title"] = json!("First");
    input["progress"] = json!([
        {"from":"1","to":"2","action":"preserve","reason":"Same first obligation"},
        {"from":"2","to":"1","action":"reset","reason":"Changed second obligation"},
        {"from":"3","action":"remove","reason":"No active work"}
    ]);
    let before = fs::read(root.path().join(TASKS_PATH)).unwrap();
    let spec = fs::read(root.path().join(".specbind/specs/checkout/spec.yaml")).unwrap();
    apply(root.path(), &input, true)
        .success()
        .stdout(predicate::str::contains("TASK_REVISION_CHECKED"));
    assert_eq!(before, fs::read(root.path().join(TASKS_PATH)).unwrap());
    apply(root.path(), &input, false)
        .success()
        .stdout(predicate::str::contains("TASK_REVISION_APPLIED"));
    let after = snapshot(root.path());
    assert_eq!(
        after["current"]["execution"]["tasks"],
        json!({"2":{"status":"completed"}})
    );
    assert_ne!(after["sourceFingerprint"], input["sourceFingerprint"]);
    assert_eq!(
        spec,
        fs::read(root.path().join(".specbind/specs/checkout/spec.yaml")).unwrap()
    );
    assert_unchanged(root.path(), &input, "TASK_REVISION_SOURCE_CHANGED");
    specbind_command()
        .current_dir(root.path())
        .args(["check", "traceability", "checkout"])
        .assert()
        .success();
    specbind_command()
        .current_dir(root.path())
        .args([
            "spec",
            "tasks",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success();
}

#[test]
fn revision_preserves_blocked_reason_and_maps_into_groups() {
    let root = setup();
    let mut input = candidate(root.path());
    let mut tasks = input["plan"]["items"].clone();
    for (index, task) in tasks.as_array_mut().unwrap().iter_mut().enumerate() {
        task["id"] = json!(format!("1.{}", index + 1));
        input["progress"][index]["to"] = task["id"].clone();
    }
    input["plan"]["items"] = json!([{"id":"1","kind":"group","title":"Group","tasks":tasks}]);
    apply(root.path(), &input, false).success();
    let current = snapshot(root.path());
    assert_eq!(
        current["current"]["execution"]["tasks"]["1.2"],
        json!({"status":"blocked","blocked_reason":"Waiting on API"})
    );
    assert_eq!(
        current["current"]["execution"]["tasks"]["1.1"]["status"],
        "completed"
    );
}

#[test]
fn revision_rejects_incomplete_duplicate_and_invented_mappings() {
    let root = setup();
    let base = candidate(root.path());
    let mut input = base.clone();
    input["progress"].as_array_mut().unwrap().pop();
    assert_unchanged(root.path(), &input, "TASK_REVISION_MAPPING_INCOMPLETE");
    for (field, value) in [("from", "1"), ("from", "9"), ("to", "1"), ("to", "9")] {
        let mut input = base.clone();
        input["progress"][1][field] = json!(value);
        assert_unchanged(root.path(), &input, "TASK_REVISION_MAPPING_INVALID");
    }
}

#[test]
fn revision_rejects_injected_state_bad_schema_and_invalid_plan() {
    let root = setup();
    let base = candidate(root.path());
    for (field, value) in [
        ("execution", json!({"tasks":{"1":{"status":"completed"}}})),
        ("schemaVersion", json!(2)),
    ] {
        let mut input = base.clone();
        input[field] = value;
        assert_unchanged(root.path(), &input, "TASK_REVISION_INPUT_INVALID");
    }
    let mut input = base.clone();
    input["progress"][0]["blocked_reason"] = json!("invented");
    assert_unchanged(root.path(), &input, "TASK_REVISION_INPUT_INVALID");
    let mut input = base.clone();
    input["progress"][0]["reason"] = json!("  ");
    assert_unchanged(root.path(), &input, "TASK_REVISION_INPUT_INVALID");
    let mut input = base.clone();
    input["plan"]["items"][0]["id"] = json!("4");
    assert_unchanged(root.path(), &input, "TASK_POSITIONAL_ID");
    let mut input = base.clone();
    input["plan"]["items"][0]["depends_on"] = json!(["2"]);
    assert_unchanged(root.path(), &input, "TASK_REVISION_PLAN_INVALID");
    let mut input = base;
    input["plan"]["items"][0]["requirement_ids"] = json!(["9.9"]);
    assert_unchanged(root.path(), &input, "TASK_REVISION_TRACEABILITY_INVALID");
}

#[test]
fn revision_rejects_changed_source_and_prior_gates() {
    let root = setup();
    let input = candidate(root.path());
    let original = fs::read_to_string(root.path().join(TASKS_PATH)).unwrap();
    write(
        root.path(),
        TASKS_PATH,
        &original.replace("Waiting on API", "New blocker"),
    );
    assert_unchanged(root.path(), &input, "TASK_REVISION_SOURCE_CHANGED");
    write(root.path(), TASKS_PATH, &original);
    write(
        root.path(),
        ".specbind/specs/checkout/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\nrequirement_ids: ['1.1']\n---\n# Changed\n_Requirements: 1.1_\n",
    );
    assert_unchanged(root.path(), &input, "TASK_REVISION_PRIOR_STALE");
}

#[test]
fn revision_requires_tasks_state_and_fresh_review() {
    let root = setup();
    let input = candidate(root.path());
    specbind_command()
        .current_dir(root.path())
        .args([
            "spec",
            "tasks",
            "approve",
            "checkout",
            "--approval-mode",
            "explicit",
        ])
        .assert()
        .success();
    assert_unchanged(root.path(), &input, "TASK_REVISION_STATE_INVALID");
    commit_all(root.path());
    specbind_command()
        .current_dir(root.path())
        .args(["spec", "design", "invalidate", "checkout"])
        .assert()
        .success();
    approve_design(root.path());
    assert_unchanged(root.path(), &input, "TASK_REVISION_REVIEW_REQUIRED");
}

#[test]
fn revision_handles_no_execution_and_refuses_repository_input() {
    let root = setup();
    let mut input = candidate(root.path());
    for mapping in input["progress"].as_array_mut().unwrap() {
        mapping["action"] = json!("reset");
    }
    apply(root.path(), &input, false).success();
    let current = snapshot(root.path());
    assert!(current["current"].get("execution").is_none());
    let mut input = json!({"schemaVersion":1,"sourceFingerprint":current["sourceFingerprint"],"plan":current["current"]["plan"],"progress":[]});
    input["plan"]["items"][0]["title"] = json!("Revised pending work");
    write(root.path(), "candidate.json", &input.to_string());
    specbind_command()
        .current_dir(root.path())
        .args([
            "tasks",
            "revision",
            "apply",
            "checkout",
            "--input",
            "candidate.json",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "TASK_REVISION_INPUT_TARGET_INVALID",
        ));
    apply(root.path(), &input, false).success();
}
