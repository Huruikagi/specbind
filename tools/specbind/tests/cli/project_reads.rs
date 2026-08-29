use super::*;

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
        .stdout(predicate::str::starts_with("OK SPEC_LISTED: Found 2 spec(s).\n  analytics: state=idle milestone=none requirements=no contract=no\n  checkout: state=implementation milestone=")
            .and(predicate::str::contains(" requirements=yes contract=yes\n")))
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

fn steering_document(id: &str, title: &str) -> String {
    format!(
        "---
type: SpecBind Steering
artifact_id: {id}
---
# {title}
"
    )
}

#[test]
fn lists_no_steering_before_any_is_authored() {
    let root = project_fixture();

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["steering", "list"])
        // An absent steering directory is an empty inventory, not a fault.
        .assert()
        .success()
        .stdout(
            "OK STEERING_LISTED: Found 0 steering document(s).
",
        )
        .stderr("");
}

#[test]
fn lists_steering_by_artifact_id_and_excludes_other_types() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        &steering_document("product", "Product"),
    );
    write(
        root.path(),
        ".specbind/steering/nested/conventions.md",
        &steering_document("naming", "Naming"),
    );
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

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["steering", "list"])
        .assert()
        .success()
        // Ordered by artifact_id, discovered recursively, and the active Roadmap
        // in the same directory is excluded by type without being an anomaly.
        .stdout(concat!(
            "OK STEERING_LISTED: Found 2 steering document(s).
",
            "  selector=naming type=\"SpecBind Steering\" path=steering/nested/conventions.md project_path=.specbind/steering/nested/conventions.md
",
            "  selector=product type=\"SpecBind Steering\" path=steering/product.md project_path=.specbind/steering/product.md
",
        ))
        .stderr("");
}

#[test]
fn reads_one_steering_selector_as_raw_markdown() {
    let root = project_fixture();
    let content = steering_document("product", "Product");
    write(root.path(), ".specbind/steering/product.md", &content);

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["steering", "read", "product"])
        .assert()
        .success()
        .stdout(content)
        .stderr("");
}

#[test]
fn projects_steering_instructions_for_the_named_use() {
    let root = project_fixture();
    let content = format!(
        "{}\n<!-- specbind:instruction maintain Revise current guidance. -->\n<!-- specbind:instruction consume Apply this constraint. -->\n",
        steering_document("product", "Product").trim_end()
    );
    write(root.path(), ".specbind/steering/product.md", &content);

    let mut maintain = specbind_command();
    maintain
        .current_dir(root.path())
        .args(["steering", "read", "product", "--for", "maintain"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Revise current guidance.")
                .and(predicate::str::contains("Apply this constraint.").not()),
        );

    let mut consume = specbind_command();
    consume
        .current_dir(root.path())
        .args(["steering", "read", "product", "--for", "consume"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Apply this constraint.")
                .and(predicate::str::contains("Revise current guidance.").not()),
        );
}

#[test]
fn reports_an_unknown_steering_selector_without_touching_stdout() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        &steering_document("product", "Product"),
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["steering", "read", "missing"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR STEERING_READ_INVALID: unknown steering selector: missing; searched_project_path=.specbind/steering
",
        ));
}

#[test]
fn drops_both_documents_sharing_one_artifact_id() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/one.md",
        &steering_document("product", "One"),
    );
    write(
        root.path(),
        ".specbind/steering/two.md",
        &steering_document("product", "Two"),
    );

    let mut list = specbind_command();
    list.current_dir(root.path())
        .args(["steering", "list"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::contains("STEERING_ARTIFACT_ID_DUPLICATE steering/one.md")
                .and(predicate::str::contains(
                    "STEERING_ARTIFACT_ID_DUPLICATE steering/two.md",
                ))
                // Neither is offered as a usable selector.
                .and(predicate::str::contains("selector=product").not()),
        );

    let mut read = specbind_command();
    read.current_dir(root.path())
        .args(["steering", "read", "product"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::starts_with(
            "ERROR STEERING_READ_INVALID: steering selector is ambiguous: product; searched_project_path=.specbind/steering
",
        ));
}

#[test]
fn refuses_to_read_valid_guidance_while_the_collection_is_incomplete() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        &steering_document("product", "Product"),
    );
    write(
        root.path(),
        ".specbind/steering/broken.md",
        "no front matter
",
    );

    // Unlike a spec-local artifact read, an unrelated fault fails this read:
    // guidance known to be incomplete cannot be safely acted on.
    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["steering", "read", "product"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with("ERROR STEERING_READ_FAILED: ").and(
                predicate::str::contains("STEERING_FRONTMATTER_INVALID steering/broken.md"),
            ),
        );
}

#[test]
fn rejects_steering_without_a_usable_artifact_id() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/steering/product.md",
        "---
type: SpecBind Steering
---
# Product
",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["steering", "list"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "STEERING_ARTIFACT_ID_INVALID steering/product.md",
        ));
}

#[test]
fn installs_the_marked_block_into_each_agent_instruction_file() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    write(root.path(), "AGENTS.md", "# Project\n\nOur own rules.\n");

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args([
            "install",
            "--agent",
            "claude-code",
            "--agent",
            "codex",
            "--language",
            "en",
            "--project-instructions",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("- create AGENTS.md [project-instructions]").and(
                predicate::str::contains("- create CLAUDE.md [project-instructions]"),
            ),
        );

    // The project's own content survives; the block is appended after it.
    let agents = fs::read_to_string(root.path().join("AGENTS.md")).expect("AGENTS.md");
    assert!(
        agents.starts_with("# Project\n\nOur own rules.\n\n"),
        "{agents}"
    );
    assert!(agents.contains("<!-- specbind:block -->"), "{agents}");
    assert!(
        agents.trim_end().ends_with("<!-- /specbind:block -->"),
        "{agents}"
    );

    // A missing file is created holding the block alone.
    let claude = fs::read_to_string(root.path().join("CLAUDE.md")).expect("CLAUDE.md");
    assert!(claude.starts_with("<!-- specbind:block -->\n"), "{claude}");

    // Re-running changes nothing.
    let mut again = specbind_command();
    again
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("NO_CHANGE INSTALL_UP_TO_DATE"));
}

#[test]
fn plans_no_instruction_file_when_the_block_is_disabled() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args([
            "install",
            "--dry-run",
            "--agent",
            "codex",
            "--language",
            "en",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("[project-instructions]").not());
    assert!(!root.path().join("AGENTS.md").exists());
}

#[test]
fn stops_installing_instructions_on_a_malformed_marker() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    // An opening marker with no closing one: the installer never repairs text
    // the project owns.
    write(
        root.path(),
        "AGENTS.md",
        "# Project\n\n<!-- specbind:block -->\nhand written\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args([
            "install",
            "--dry-run",
            "--agent",
            "codex",
            "--language",
            "en",
            "--project-instructions",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "PROJECT_INSTRUCTIONS_MARKERS_INVALID AGENTS.md",
        ));

    let preserved = fs::read_to_string(root.path().join("AGENTS.md")).expect("AGENTS.md");
    assert_eq!(
        preserved,
        "# Project\n\n<!-- specbind:block -->\nhand written\n"
    );
}

#[test]
fn lists_accepted_adapters_with_project_presence() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/adapters/git.md",
        "---\ntype: SpecBind Git Adapter\n---\n# Git\n\nCommit after each gate.\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["adapter", "list"])
        .assert()
        .success()
        .stdout(concat!(
            "OK ADAPTER_LISTED: Found 3 accepted adapter(s).\n",
            "  selector=release type=\"SpecBind Release Adapter\" path=settings/adapters/release.md present=no state=absent\n",
            "  selector=git type=\"SpecBind Git Adapter\" path=settings/adapters/git.md present=yes state=active\n",
            "  selector=deferred type=\"SpecBind Deferred Findings Adapter\" path=settings/adapters/deferred.md present=no state=absent\n",
        ))
        .stderr("");
}

#[test]
fn reads_one_adapter_as_raw_markdown_and_reports_absence() {
    let root = project_fixture();
    let content = "---\ntype: SpecBind Git Adapter\n---\n# Git\n\nCommit after each gate.\n";
    write(root.path(), ".specbind/settings/adapters/git.md", content);

    let mut present = specbind_command();
    present
        .current_dir(root.path())
        .args(["adapter", "read", "git"])
        .assert()
        .success()
        .stdout(content)
        .stderr("");

    // Absence is reported, not judged. Whether a missing adapter is a fault
    // belongs to the consuming skill.
    let mut absent = specbind_command();
    absent
        .current_dir(root.path())
        .args(["adapter", "read", "release"])
        .assert()
        .success()
        .stdout("NO_CHANGE ADAPTER_ABSENT: The project has no release adapter.\n")
        .stderr("");
}

#[test]
fn refuses_a_selector_the_product_does_not_accept() {
    let root = project_fixture();
    // The directory is organization, not an extension loader: an unknown file
    // below it is never readable.
    write(
        root.path(),
        ".specbind/settings/adapters/deploy.md",
        "---\ntype: SpecBind Git Adapter\n---\n# Deploy\n",
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["adapter", "read", "deploy"])
        .assert()
        .failure()
        .stdout("")
        .stderr("ERROR ADAPTER_READ_INVALID: unknown adapter selector: deploy\n");
}

#[test]
fn lists_accepted_rules_with_project_presence() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/rules/ears-format.md",
        "---\ntype: SpecBind Rule\n---\n# EARS\n",
    );
    write(
        root.path(),
        ".specbind/settings/rules/design-template-selection.md",
        concat!(
            "---\ntype: SpecBind Rule\n---\n# Selection\n\n",
            "## `design/main`\n\nMode: required\n\nAlways.\n\n",
            "## `design/ui`\n\nMode: conditional\n\nFor user-visible UI changes.\n",
        ),
    );

    let mut command = specbind_command();
    command
        .current_dir(root.path())
        .args(["rule", "list"])
        .assert()
        .success()
        .stdout(concat!(
            "OK RULE_LISTED: Found 6 accepted rule(s).\n",
            "  selector=ears-format type=\"SpecBind Rule\" path=settings/rules/ears-format.md present=yes\n",
            "  selector=design-principles type=\"SpecBind Rule\" path=settings/rules/design-principles.md present=no\n",
            "  selector=design-template-selection type=\"SpecBind Rule\" path=settings/rules/design-template-selection.md present=yes\n",
            "  selector=contract-principles type=\"SpecBind Rule\" path=settings/rules/contract-principles.md present=no\n",
            "  selector=tasks-generation type=\"SpecBind Rule\" path=settings/rules/tasks-generation.md present=no\n",
            "  selector=steering-principles type=\"SpecBind Rule\" path=settings/rules/steering-principles.md present=no\n",
        ))
        .stderr("");
}

#[test]
fn design_template_selection_rule_is_required_and_matches_the_candidate_set() {
    let root = project_fixture();

    let mut absent = specbind_command();
    absent
        .current_dir(root.path())
        .args([
            "rule",
            "read",
            "design-template-selection",
            "--for",
            "consume",
        ])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("ERROR RULE_REQUIRED"));

    write(
        root.path(),
        ".specbind/settings/rules/design-template-selection.md",
        concat!(
            "---\ntype: SpecBind Rule\n---\n# Selection\n\n",
            "## `design/main`\n\nMode: required\n\nAlways.\n",
        ),
    );
    let mut incomplete = specbind_command();
    incomplete
        .current_dir(root.path())
        .args(["rule", "read", "design-template-selection"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "RULE_DESIGN_TEMPLATE_SELECTOR_MISSING design/ui",
        ));
}

#[test]
fn reads_and_projects_one_rule_and_reports_absence() {
    let root = project_fixture();
    let content = concat!(
        "---\ntype: SpecBind Rule\n---\n# EARS\n",
        "<!-- specbind:instruction maintain Preserve examples. -->\n",
        "<!-- specbind:instruction consume Apply this phrasing. -->\n",
    );
    write(
        root.path(),
        ".specbind/settings/rules/ears-format.md",
        content,
    );

    let mut raw = specbind_command();
    raw.current_dir(root.path())
        .args(["rule", "read", "ears-format"])
        .assert()
        .success()
        .stdout(content)
        .stderr("");

    let mut consume = specbind_command();
    consume
        .current_dir(root.path())
        .args(["rule", "read", "ears-format", "--for", "consume"])
        .assert()
        .success()
        .stdout(concat!(
            "---\ntype: SpecBind Rule\n---\n# EARS\n",
            "<!-- specbind:instruction consume Apply this phrasing. -->\n",
        ))
        .stderr("");

    let mut absent = specbind_command();
    absent
        .current_dir(root.path())
        .args(["rule", "read", "design-principles", "--for", "consume"])
        .assert()
        .success()
        .stdout("NO_CHANGE RULE_ABSENT: The project has no design-principles rule.\n")
        .stderr("");
}

#[test]
fn rejects_unknown_rules_and_invalid_live_rule_instructions() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/rules/ears-format.md",
        concat!(
            "---\ntype: SpecBind Rule\n---\n",
            "<!-- specbind:instruction create Draft this rule. -->\n",
        ),
    );

    let mut invalid = specbind_command();
    invalid
        .current_dir(root.path())
        .args(["rule", "read", "ears-format", "--for", "consume"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains(
            "ERROR RULE_READ_FAILED: Rule ears-format has invalid managed instructions.",
        ))
        .stderr(predicate::str::contains("ARTIFACT_CREATE_INSTRUCTION_LEAK"));

    let mut unknown = specbind_command();
    unknown
        .current_dir(root.path())
        .args(["rule", "read", "deployment"])
        .assert()
        .failure()
        .stdout("")
        .stderr("ERROR RULE_READ_INVALID: unknown rule selector: deployment\n");
}

#[test]
fn rejects_invalid_rule_read_targets_and_non_utf8_content() {
    let root = project_fixture();
    fs::create_dir_all(
        root.path()
            .join(".specbind/settings/rules/design-principles.md"),
    )
    .expect("directory at rule target");
    fs::write(
        root.path()
            .join(".specbind/settings/rules/contract-principles.md"),
        [0xff, 0xfe],
    )
    .expect("non-UTF-8 rule");

    let mut invalid_target = specbind_command();
    invalid_target
        .current_dir(root.path())
        .args(["rule", "read", "design-principles"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("ERROR RULE_READ_TARGET_INVALID:"));

    let mut non_utf8 = specbind_command();
    non_utf8
        .current_dir(root.path())
        .args(["rule", "read", "contract-principles"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("ERROR RULE_READ_NOT_UTF8:"));
}

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
            "OK SCHEMA_LISTED: Found 4 embedded schema(s).\n",
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
