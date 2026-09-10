use super::super::*;

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
            "  selector=naming type=\"SpecBind Steering\" path=steering/nested/conventions.md project_path=.specbind/steering/nested/conventions.md description=missing
",
            "  selector=product type=\"SpecBind Steering\" path=steering/product.md project_path=.specbind/steering/product.md description=missing
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
