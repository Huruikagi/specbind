use super::super::*;

#[test]
fn shared_reads_and_ownership_follow_custom_spec_dir_and_keep_absence_distinct() {
    let root = project_fixture();
    fs::create_dir_all(root.path().join("knowledge")).unwrap();
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":"knowledge","language":"en","agents":["codex"]}"#,
    );
    specbind_command()
        .current_dir(root.path())
        .args(["contract", "shared", "read"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SHARED_CONTRACT_ABSENT"));
    specbind_command()
        .current_dir(root.path())
        .args(["contract", "shared", "consumers", "translations"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "SHARED_CONTRACT_RESOURCE_NOT_FOUND",
        ));
    write(
        root.path(),
        "knowledge/shared-contract.yaml",
        "schema_version: 1\nresources:\n  - id: translations\n    description: Catalogs\n    paths: [locales/**]\n    change_policy: Preserve keys.\n    invariants: []\n",
    );
    specbind_command()
        .current_dir(root.path())
        .args(["contract", "owners", "locales/en.json"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("shared-contract#resources/translations").and(
                predicate::str::contains("Ambiguous management boundaries: no"),
            ),
        );
    write(
        root.path(),
        "knowledge/specs/broken/contract.yaml",
        "broken: [",
    );
    // Reading the agreement does not depend on unrelated graph health.
    specbind_command()
        .current_dir(root.path())
        .args(["contract", "shared", "read"])
        .assert()
        .success()
        .stdout(predicate::str::contains("id: translations"));
    specbind_command()
        .current_dir(root.path())
        .args(["check", "contracts"])
        .assert()
        .failure();
}

#[test]
fn lists_no_specs_before_the_specs_directory_exists() {
    let root = project_fixture();
    fs::remove_dir_all(root.path().join(".specbind/specs"))
        .expect("remove the specs directory created by the fixture");

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["spec", "list"])
        // Installation creates settings before any Spec exists. Discovery must
        // be able to ask for the empty list without materializing the directory.
        .assert()
        .success()
        .stdout("OK SPEC_LISTED: Found 0 spec(s).\n")
        .stderr("");
}

#[test]
fn lists_specs_in_identity_order_with_lifecycle_and_artifact_presence() {
    let root = project_fixture();
    write_status_fixture(root.path());
    fs::create_dir_all(root.path().join(".specbind/specs/analytics"))
        .expect("create idle spec directory");
    write(
        root.path(),
        ".specbind/specs/analytics/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["spec", "list"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK SPEC_LISTED: Found 2 spec(s).\n  analytics: state=idle milestone=none requirements=no contract=no description=unavailable\n  checkout: state=implementation milestone=")
            .and(predicate::str::contains(" requirements=yes contract=yes description=missing\n")))
        .stderr("");
}

#[test]
fn lists_an_unreadable_spec_instead_of_failing_the_listing() {
    let root = project_fixture();
    write_status_fixture(root.path());
    fs::create_dir_all(root.path().join(".specbind/specs/analytics"))
        .expect("create broken spec directory");
    write(
        root.path(),
        ".specbind/specs/analytics/spec.yaml",
        "schema_version: 1\nactive_change: {state: nonsense}\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        // The broken Spec is reported, and the healthy one beside it survives.
        .args(["spec", "list"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Found 2 spec(s).")
                .and(predicate::str::contains("\n  analytics: unreadable: "))
                .and(predicate::str::contains("\n  checkout: state=")),
        )
        .stderr("");
}

#[test]
fn reports_no_change_reading_scope_without_an_active_milestone() {
    let root = project_fixture();

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        .stdout("NO_CHANGE NO_ACTIVE_MILESTONE: No active milestone exists.\n")
        .stderr("");
}

#[test]
fn refuses_to_emit_a_partial_scope_from_an_invalid_roadmap() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/roadmap.md",
        "---\ntype: SpecBind Roadmap\n---\n# Roadmap\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR MILESTONE_SCOPE_FAILED: Cannot read the active milestone scope.\n",
        ));
}

#[test]
fn writes_the_current_scope_as_a_replacement_candidate() {
    let root = project_fixture();
    commit_all(root.path());

    let mut create = specbind_command();
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"newSpecs":[{"spec":"payments","summary":"Add payments"}],"directChanges":[{"id":"docs","summary":"Update docs","dependsOn":[{"spec":"payments"}]}]},"body":"Overview\n\nDeliver payments.\n"}"#,
        )
        .assert()
        .success();

    let mut command = specbind_command();
    let output = command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let document = String::from_utf8(output).expect("UTF-8 scope document");

    // The serialization is a byte-exact contract: declared field order,
    // two-space indentation, no body, no per-item status, one trailing newline.
    assert_eq!(
        document,
        concat!(
            "{\n",
            "  \"schemaVersion\": 1,\n",
            "  \"workItems\": {\n",
            "    \"newSpecs\": [\n",
            "      {\n",
            "        \"spec\": \"payments\",\n",
            "        \"summary\": \"Add payments\"\n",
            "      }\n",
            "    ],\n",
            "    \"directChanges\": [\n",
            "      {\n",
            "        \"id\": \"docs\",\n",
            "        \"summary\": \"Update docs\",\n",
            "        \"dependsOn\": [\n",
            "          { \"spec\": \"payments\" }\n",
            "        ]\n",
            "      }\n",
            "    ]\n",
            "  }\n",
            "}\n",
        )
    );

    // The round trip is the invariant Decision 0097 accepts: feeding the read
    // straight back into the replacement changes nothing.
    let mut round_trip = specbind_command();
    round_trip
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(document)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_SCOPE_UNCHANGED",
        ))
        .stderr("");
}

#[test]
fn omits_completed_direct_status_from_the_emitted_scope() {
    let root = project_fixture();
    commit_all(root.path());

    let mut create = specbind_command();
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]}}"#,
        )
        .assert()
        .success();
    commit_all(root.path());

    let revision = git_stdout(root.path(), &["rev-parse", "HEAD"]);
    let mut complete = specbind_command();
    complete
        .current_dir(root.path())
        .args([
            "milestone",
            "direct",
            "complete",
            "docs",
            "--implementation-revision",
            &revision,
        ])
        .assert()
        .success();

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        // Status is CLI-owned and preserved by identity, so a candidate that
        // carried it would be rejected by the command it feeds.
        .stdout(predicate::str::contains("\"status\"").not())
        .stderr("");
}

#[test]
fn emits_the_complete_body_only_when_deliberately_requested() {
    let root = project_fixture();
    commit_all(root.path());

    let mut create = specbind_command();
    create
        .current_dir(root.path())
        .args(["milestone", "create", "--scope", "-"])
        .write_stdin(
            r#"{"schemaVersion":1,"workItems":{"directChanges":[{"id":"docs","summary":"Update docs"}]},"body":"Overview\n\nDeliver docs.\n"}"#,
        )
        .assert()
        .success();

    let mut command = specbind_command();
    let output = command
        .current_dir(root.path())
        .args(["milestone", "scope", "--include-body"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let document = String::from_utf8(output).expect("UTF-8 scope document");

    // The body is complete and follows the work items, so a caller edits one
    // whole value rather than composing a replacement from a fragment.
    assert!(
        document.contains("  \"body\": \"Overview\\n\\nDeliver docs.\\n\"\n}\n"),
        "{document}"
    );

    // The round trip holds for this form too.
    let mut round_trip = specbind_command();
    round_trip
        .current_dir(root.path())
        .args(["milestone", "update-scope", "--scope", "-"])
        .write_stdin(document)
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE MILESTONE_SCOPE_UNCHANGED",
        ))
        .stderr("");

    // The default read stays body-free, so an ordinary round trip cannot
    // rewrite authored prose.
    let mut default = specbind_command();
    default
        .current_dir(root.path())
        .args(["milestone", "scope"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"body\"").not())
        .stderr("");
}
