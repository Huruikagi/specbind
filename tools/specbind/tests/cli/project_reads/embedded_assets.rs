use super::super::*;

#[test]
fn installs_localized_adapter_scaffolds_and_keeps_project_copies() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    let owned = "---\ntype: SpecBind Git Adapter\n---\n# Ours\n";
    write(root.path(), ".specbind/settings/adapters/git.md", owned);

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "ja"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains(
                "- create .specbind/settings/adapters/release.md [adapter]",
            )
            .and(predicate::str::contains(
                "- keep .specbind/settings/adapters/git.md [adapter] (project-owned settings are never overwritten)",
            )),
        );

    // The scaffold follows the configured language; the type literal does not.
    let release = fs::read_to_string(root.path().join(".specbind/settings/adapters/release.md"))
        .expect("release adapter");
    assert!(
        release.starts_with("---\ntype: SpecBind Release Adapter\n---\n"),
        "{release}"
    );
    assert!(release.contains("# リリースアダプタ"), "{release}");

    let validation = fs::read_to_string(
        root.path()
            .join(".specbind/settings/adapters/validation.md"),
    )
    .expect("validation adapter");
    assert!(
        validation.starts_with("---\ntype: SpecBind Validation Adapter\n---\n"),
        "{validation}"
    );
    assert!(validation.contains("# 検証アダプター"), "{validation}");
    assert!(
        validation.contains("specbind:adapter-scaffold"),
        "{validation}"
    );

    let git_adapter = fs::read_to_string(root.path().join(".specbind/settings/adapters/git.md"))
        .expect("git adapter");
    assert_eq!(git_adapter, owned);
}

#[test]
fn lists_and_reads_embedded_schemas_without_a_project() {
    // Like the protocols, these are properties of the binary. Running outside
    // any SpecBind project is the structural guarantee of that.
    let outside = tempfile::tempdir().expect("temporary directory");

    let mut list = specbind_command();
    list.current_dir(outside.path())
        .args(["schema", "list"])
        .assert()
        .success()
        .stdout(concat!(
            "OK SCHEMA_LISTED: Found 6 embedded schema(s).\n",
            "  selector=contract/v2 artifact=contract.yaml written_by=\"the authoring agent\"\n",
            "  selector=shared-contract/v1 artifact=shared-contract.yaml written_by=\"the authoring agent\"\n",
            "  selector=contract/v1 artifact=contract.yaml written_by=\"the authoring agent\"\n",
            "  selector=spec/v1 artifact=spec.yaml written_by=\"guarded CLI operations only\"\n",
            "  selector=scope/v1 artifact=milestone scope candidate (transient) written_by=\"the authoring agent\"\n",
            "  selector=tasks/v1 artifact=tasks.yaml written_by=\"the authoring agent\"\n",
        ))
        .stderr("");

    let mut read = specbind_command();
    read.current_dir(outside.path())
        .args(["schema", "read", "tasks/v1"])
        .assert()
        .success()
        // The read is the same bytes the runtime validator compiles, so the
        // format an agent authors against cannot drift from the one enforced.
        .stdout(predicate::eq(specbind::schema::TASKS_V1_SCHEMA_JSON))
        .stderr("");
}

#[test]
fn refuses_a_schema_selector_the_binary_does_not_carry() {
    let outside = tempfile::tempdir().expect("temporary directory");

    let mut command = specbind_command();
    command
        .current_dir(outside.path())
        // An unversioned selector is not accepted: the version is part of the
        // identity, so a caller always names the schema it is targeting.
        .args(["schema", "read", "tasks"])
        .assert()
        .failure()
        .stdout("")
        .stderr("ERROR SCHEMA_READ_INVALID: unknown schema selector: tasks\n");
}
