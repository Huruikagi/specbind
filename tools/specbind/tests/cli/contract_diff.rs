use super::*;
use serde_json::{Value, json};

const EMPTY: &str =
    "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n";
const PATH: &str = ".specbind/specs/checkout/contract.yaml";

fn fixture() -> TempDir {
    let root = project_fixture();
    write(root.path(), PATH, EMPTY);
    commit_all(root.path());
    root
}

fn report(root: &Path, baseline: &str) -> Value {
    let before = git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"]);
    let output = specbind_command()
        .current_dir(root)
        .args(["contract", "diff", "--baseline", baseline, "--json"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    assert_eq!(
        before,
        git_stdout(root, &["status", "--porcelain=v1", "--untracked-files=all"])
    );
    serde_json::from_slice(&output).unwrap()
}

fn fails(root: &Path, baseline: &str, message: &str) {
    specbind_command()
        .current_dir(root)
        .args(["contract", "diff", "--baseline", baseline, "--json"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::contains("CONTRACT_DIFF_INPUT_INVALID")
                .and(predicate::str::contains(message)),
        );
}

#[test]
fn contract_diff_reports_removed_provider_and_old_and_current_consumers() {
    let root = fixture();
    write(
        root.path(),
        PATH,
        &EMPTY.replace(
            "exports: []",
            "exports: [{ id: order, description: Old guarantee. }]",
        ),
    );
    write(
        root.path(),
        ".specbind/specs/consumer/contract.yaml",
        &EMPTY.replace(
            "consumes: []",
            "consumes: [{ id: order, target: { spec: checkout, section: exports, id: order } }]",
        ),
    );
    commit_all(root.path());
    write(root.path(), PATH, EMPTY);
    let result = report(root.path(), "HEAD");
    let entry = &result["entries"][0];
    assert_eq!(entry["selector"], "specs/checkout#contract/exports/order");
    assert_eq!(entry["change"], "removed");
    assert_eq!(entry["before"]["description"], "Old guarantee.");
    assert_eq!(entry["after"], Value::Null);
    assert_eq!(
        entry["baselineConsumers"],
        json!(["specs/consumer#contract/consumes/order"])
    );
    assert_eq!(entry["currentConsumers"], entry["baselineConsumers"]);
    assert!(
        result["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v["side"] == "current" && v["severity"] == "error")
    );
    fs::remove_dir_all(root.path().join(".specbind/specs/consumer")).unwrap();
    let result = report(root.path(), "HEAD");
    assert_eq!(result["entries"][0]["currentConsumers"], json!([]));
    assert_eq!(result["documents"][1]["change"], "removed");
}

#[test]
fn contract_diff_reads_custom_root_new_spec_and_ignores_downstream_artifacts() {
    let root = fixture();
    fs::rename(root.path().join(".specbind"), root.path().join("knowledge")).unwrap();
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":"knowledge","language":"en"}"#,
    );
    commit_all(root.path());
    write(
        root.path(),
        "knowledge/specs/new-feature/contract.yaml",
        EMPTY,
    );
    write(
        root.path(),
        "knowledge/specs/checkout/tasks.yaml",
        "invalid downstream task plan",
    );
    write(
        root.path(),
        "knowledge/specs/checkout/spec.yaml",
        "not a lifecycle input",
    );
    let result = report(root.path(), "HEAD");
    assert_eq!(result["specDir"], "knowledge");
    assert_eq!(result["currentSource"], "worktree");
    assert_eq!(result["documents"][1]["change"], "added");
    assert_eq!(result["documents"][1]["baselinePath"], Value::Null);
    assert_eq!(
        result["documents"][1]["currentPath"],
        "knowledge/specs/new-feature/contract.yaml"
    );
    assert_eq!(result["entries"], json!([]));
    specbind_command()
        .current_dir(root.path())
        .args(["contract", "diff", "--baseline", "HEAD"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("OK CONTRACT_DIFF_REPORTED")
                .and(predicate::str::contains("specs/new-feature added")),
        );
}

#[test]
fn contract_diff_normalizes_order_formatting_and_detects_semantic_changes() {
    let root = fixture();
    let before = EMPTY
        .replace(
            "owns: []",
            "owns: [{ id: b, description: B. }, { id: a, description: A. }]",
        )
        .replace(
            "file_ownership: []",
            "file_ownership: [{ id: files, paths: [src/b.rs, src/a.rs] }]",
        );
    write(root.path(), PATH, &before);
    commit_all(root.path());
    let baseline = report(root.path(), "HEAD");
    let after = before
        .replace(
            "{ id: b, description: B. }, { id: a, description: A. }",
            "{ id: a, description: A. }, { id: b, description: B. }",
        )
        .replace("src/b.rs, src/a.rs", "src/a.rs, src/b.rs");
    write(
        root.path(),
        PATH,
        &format!("# comment\n{after}").replace('\n', "\r\n"),
    );
    let normalized = report(root.path(), "HEAD");
    assert_eq!(
        normalized["currentFingerprint"],
        baseline["currentFingerprint"]
    );
    assert_eq!(normalized["entries"], json!([]));
    write(
        root.path(),
        PATH,
        &after.replace("description: A.", "description: New A."),
    );
    let changed = report(root.path(), "HEAD");
    assert_eq!(changed["entries"][0]["change"], "changed");
    assert_ne!(
        changed["currentFingerprint"],
        baseline["currentFingerprint"]
    );
}

#[test]
fn contract_diff_shared_changes_keep_legacy_paths_and_removed_references() {
    let root = fixture();
    let shared = "schema_version: 1\nresources:\n  - id: catalog\n    description: Catalog.\n    paths: [data/b.json, data/a.json]\n    change_policy: Preserve keys.\n    invariants: [B., A.]\n";
    write(root.path(), ".specbind/shared-contract.yaml", shared);
    write(root.path(), PATH, &EMPTY.replace("schema_version: 1", "schema_version: 2").replace("consumes: []", "consumes: [{ id: catalog, target: { shared: true, section: resources, id: catalog } }]"));
    commit_all(root.path());
    fs::remove_file(root.path().join(".specbind/shared-contract.yaml")).unwrap();
    write(
        root.path(),
        ".specbind/specs/shared-contract.yaml",
        &shared
            .replace("data/b.json, data/a.json", "data/a.json, data/b.json")
            .replace("[B., A.]", "[A., B.]"),
    );
    let moved = report(root.path(), "HEAD");
    assert_eq!(moved["entries"], json!([]));
    assert_eq!(moved["documents"][0]["change"], "unchanged");
    assert_eq!(
        moved["documents"][0]["baselinePath"],
        ".specbind/shared-contract.yaml"
    );
    fs::remove_file(root.path().join(".specbind/specs/shared-contract.yaml")).unwrap();
    let removed = report(root.path(), "HEAD");
    assert_eq!(
        removed["entries"][0]["selector"],
        "shared-contract#resources/catalog"
    );
    assert_eq!(
        removed["entries"][0]["currentConsumers"],
        json!(["specs/checkout#contract/consumes/catalog"])
    );
    write(
        root.path(),
        ".specbind/specs/shared-contract.yaml",
        "schema_version: 1\nresources: []\n",
    );
    write(root.path(), PATH, EMPTY);
    commit_all(root.path());
    fs::remove_file(root.path().join(".specbind/specs/shared-contract.yaml")).unwrap();
    let empty_removed = report(root.path(), "HEAD");
    assert_eq!(empty_removed["documents"][0]["change"], "removed");
    assert_eq!(empty_removed["entries"], json!([]));
}

#[test]
fn contract_diff_rejects_missing_malformed_or_unreadable_inputs_without_partial_output() {
    let root = fixture();
    fails(root.path(), "does-not-exist", "revision");
    fs::remove_file(root.path().join(PATH)).unwrap();
    fails(root.path(), "HEAD", "required Contract unavailable");
    write(root.path(), PATH, "not a contract");
    fails(root.path(), "HEAD", "contract.yaml");
    commit_all(root.path());
    write(root.path(), PATH, EMPTY);
    fails(root.path(), "HEAD", "contract.yaml");
}

#[test]
fn contract_diff_missing_baseline_contract_is_not_a_new_spec() {
    let root = fixture();
    fs::remove_file(root.path().join(PATH)).unwrap();
    write(
        root.path(),
        ".specbind/specs/checkout/brief.md",
        "existing Spec",
    );
    commit_all(root.path());
    write(root.path(), PATH, EMPTY);
    fails(
        root.path(),
        "HEAD",
        "existing Spec has no required Contract",
    );
}

#[test]
fn contract_diff_tracks_added_targets_current_only_consumers_and_document_only_changes() {
    let root = fixture();
    write(
        root.path(),
        ".specbind/specs/shared-contract.yaml",
        "schema_version: 1\nresources: []\n",
    );
    write(
        root.path(),
        PATH,
        &EMPTY.replace("schema_version: 1", "schema_version: 2"),
    );
    let result = report(root.path(), "HEAD");
    assert_eq!(result["documents"][0]["change"], "added");
    assert_eq!(result["documents"][1]["change"], "changed");
    assert_eq!(result["entries"], json!([]));
    write(
        root.path(),
        PATH,
        &EMPTY.replace(
            "exports: []",
            "exports: [{ id: api, description: New API. }]",
        ),
    );
    write(
        root.path(),
        ".specbind/specs/consumer/contract.yaml",
        &EMPTY.replace(
            "consumes: []",
            "consumes: [{ id: api, target: { spec: checkout, section: exports, id: api } }]",
        ),
    );
    let result = report(root.path(), "HEAD");
    assert_eq!(result["entries"][0]["change"], "added");
    assert_eq!(result["entries"][0]["baselineConsumers"], json!([]));
    assert_eq!(
        result["entries"][0]["currentConsumers"],
        json!(["specs/consumer#contract/consumes/api"])
    );
}

#[test]
fn contract_diff_absent_baseline_namespace_is_valid_but_link_ancestor_is_not() {
    let root = project_fixture();
    commit_all(root.path());
    write(root.path(), PATH, EMPTY);
    let result = report(root.path(), "HEAD");
    assert_eq!(result["documents"][0]["change"], "added");
    write(root.path(), "link-target.txt", "elsewhere");
    let oid = git_stdout(root.path(), &["hash-object", "-w", "link-target.txt"]);
    git(
        root.path(),
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("120000,{oid},.specbind/specs"),
        ],
    );
    git(
        root.path(),
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-m",
            "symlink namespace",
        ],
    );
    fails(root.path(), "HEAD", "regular directory");
}

#[test]
fn contract_diff_does_not_misclassify_a_changed_spec_root_as_new_specs() {
    let root = fixture();
    fs::rename(root.path().join(".specbind"), root.path().join("knowledge")).unwrap();
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":"knowledge","language":"en"}"#,
    );
    fails(root.path(), "HEAD", "baseline specDir differs");
}

#[test]
fn contract_diff_baseline_links_and_duplicate_shared_paths_fail_closed() {
    let root = fixture();
    write(root.path(), "link-target.txt", "elsewhere");
    let oid = git_stdout(root.path(), &["hash-object", "-w", "link-target.txt"]);
    git(
        root.path(),
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("120000,{oid},{PATH}"),
        ],
    );
    git(
        root.path(),
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-m",
            "symlink Contract",
        ],
    );
    fails(root.path(), "HEAD", "regular file");
    let root = fixture();
    let shared = "schema_version: 1\nresources: []\n";
    write(root.path(), ".specbind/shared-contract.yaml", shared);
    write(root.path(), ".specbind/specs/shared-contract.yaml", shared);
    fails(root.path(), "HEAD", "both");
    commit_all(root.path());
    fs::remove_file(root.path().join(".specbind/shared-contract.yaml")).unwrap();
    fails(root.path(), "HEAD", "both");
}
