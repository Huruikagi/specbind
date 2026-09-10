use super::super::*;

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
            "OK ADAPTER_LISTED: Found 4 accepted adapter(s).\n",
            "  selector=release type=\"SpecBind Release Adapter\" path=settings/adapters/release.md present=no state=absent\n",
            "  selector=git type=\"SpecBind Git Adapter\" path=settings/adapters/git.md present=yes state=active\n",
            "  selector=deferred type=\"SpecBind Deferred Findings Adapter\" path=settings/adapters/deferred.md present=no state=absent\n",
            "  selector=validation type=\"SpecBind Validation Adapter\" path=settings/adapters/validation.md present=no state=absent\n",
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
fn projects_only_active_adapter_guidance_for_consumers() {
    let root = project_fixture();
    let active = "---\ntype: SpecBind Git Adapter\n---\n# Git\n\nCommit after each gate.\n";
    let scaffold = concat!(
        "---\ntype: SpecBind Validation Adapter\n---\n# Validation\n\n",
        "<!-- specbind:adapter-scaffold -->\n\nDescribe project checks.\n",
    );
    write(root.path(), ".specbind/settings/adapters/git.md", active);
    write(
        root.path(),
        ".specbind/settings/adapters/validation.md",
        scaffold,
    );

    let mut active_command = specbind_command();
    active_command
        .current_dir(root.path())
        .args(["adapter", "read", "git", "--for", "consume"])
        .assert()
        .success()
        .stdout(active)
        .stderr("");

    let mut scaffold_command = specbind_command();
    scaffold_command
        .current_dir(root.path())
        .args(["adapter", "read", "validation", "--for", "consume"])
        .assert()
        .success()
        .stdout(
            "NO_CHANGE ADAPTER_SCAFFOLD: The project validation adapter is an inactive scaffold.\n",
        )
        .stderr("");

    let mut absent_command = specbind_command();
    absent_command
        .current_dir(root.path())
        .args(["adapter", "read", "release", "--for", "consume"])
        .assert()
        .success()
        .stdout("NO_CHANGE ADAPTER_ABSENT: The project has no release adapter.\n")
        .stderr("");

    let mut raw_scaffold = specbind_command();
    raw_scaffold
        .current_dir(root.path())
        .args(["adapter", "read", "validation"])
        .assert()
        .success()
        .stdout(scaffold)
        .stderr("");

    let mut invalid_purpose = specbind_command();
    invalid_purpose
        .current_dir(root.path())
        .args(["adapter", "read", "git", "--for", "maintain"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "invalid value 'maintain' for '--for <PURPOSE>'",
        ));
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
