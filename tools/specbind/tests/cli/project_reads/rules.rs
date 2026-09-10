use super::super::*;

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
            "OK RULE_LISTED: Found 7 accepted rule(s).\n",
            "  selector=ears-format type=\"SpecBind Rule\" path=settings/rules/ears-format.md present=yes\n",
            "  selector=design-principles type=\"SpecBind Rule\" path=settings/rules/design-principles.md present=no\n",
            "  selector=design-template-selection type=\"SpecBind Rule\" path=settings/rules/design-template-selection.md present=yes\n",
            "  selector=contract-principles type=\"SpecBind Rule\" path=settings/rules/contract-principles.md present=no\n",
            "  selector=tasks-generation type=\"SpecBind Rule\" path=settings/rules/tasks-generation.md present=no\n",
            "  selector=steering-principles type=\"SpecBind Rule\" path=settings/rules/steering-principles.md present=no\n",
            "  selector=language-style type=\"SpecBind Rule\" path=settings/rules/language-style.md present=no\n",
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
