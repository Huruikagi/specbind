use std::{fs, path::Path, process::Command};

use specbind::{
    artifacts, cli, config::ProjectLanguage, description::Description, instruction,
    migration::project, steering, template,
};

fn write(root: &Path, path: &str, content: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, content).unwrap();
}

fn setup() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    assert!(
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(root.path())
            .status()
            .unwrap()
            .success()
    );
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":"knowledge","language":"en"}"#,
    );
    fs::create_dir_all(root.path().join("knowledge")).unwrap();
    root
}

fn requirements(description: &str) -> String {
    format!(
        "---\ntype: SpecBind Requirements\n{description}heading_labels:\n  requirement: Requirement\n  acceptance_criteria: Acceptance Criteria\n---\n### Requirement 1: Store items\n#### Acceptance Criteria\n1. Items persist.\n"
    )
}

#[test]
fn descriptions_preserve_missing_compatibility_and_reject_invalid_values() {
    let root = setup();
    let knowledge = root.path().join("knowledge");
    for field in [
        "",
        "description: ''\n",
        "description: null\n",
        "description: 42\n",
        "description: |\n  multiple\n  lines\n",
        "description: \"control\\tvalue\"\n",
        "description: \"unicode\u{2028}separator\"\n",
    ] {
        write(
            &knowledge,
            "specs/example/requirements.md",
            &requirements(field),
        );
        let inventory = artifacts::discover_spec(&knowledge, "example");
        assert_eq!(inventory.artifacts.len(), 1);
        if field.is_empty() {
            assert!(inventory.issues.is_empty(), "{:?}", inventory.issues);
            assert_eq!(inventory.artifacts[0].description, Description::Missing);
        } else {
            assert_eq!(inventory.artifacts[0].description, Description::Invalid);
            assert!(
                inventory
                    .issues
                    .iter()
                    .any(|issue| issue.code == "ARTIFACT_DESCRIPTION_INVALID")
            );
        }
    }
}

#[test]
fn lists_escape_descriptions_and_retain_raw_reads_and_partial_inventory() {
    let root = setup();
    let knowledge = root.path().join("knowledge");
    let description = "description: 'Stores \"quoted\" items.'\n";
    let req = requirements(description);
    write(&knowledge, "specs/example/requirements.md", &req);
    write(
        &knowledge,
        "specs/example/design.md",
        &format!(
            "---\ntype: SpecBind Design\nartifact_id: main\n{description}requirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n"
        ),
    );
    let guidance = format!(
        "---\ntype: SpecBind Steering\nartifact_id: product\n{description}---\n# Purpose\n"
    );
    write(&knowledge, "steering/product.md", &guidance);
    for output in [
        cli::spec_list(root.path()),
        cli::artifact_list(root.path(), "example"),
        cli::steering_list(root.path()),
    ] {
        assert!(
            output.success,
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(
            text.contains("description=\"Stores \\\"quoted\\\" items.\""),
            "{text}"
        );
    }
    assert_eq!(
        cli::artifact_read(root.path(), "example", "requirements", None).stdout,
        req.as_bytes()
    );
    assert_eq!(steering::read(&knowledge, "product").unwrap(), guidance);
    write(
        &knowledge,
        "specs/example/bad.md",
        "---\ntype: SpecBind Design\nartifact_id: broken\ndescription: 0\nrequirement_ids: ['1.1']\n---\n_Requirements: 1.1_\n",
    );
    write(
        &knowledge,
        "steering/bad.md",
        "---\ntype: SpecBind Steering\nartifact_id: broken\ndescription: false\n---\n",
    );
    for output in [
        cli::artifact_list(root.path(), "example"),
        cli::steering_list(root.path()),
    ] {
        assert!(!output.success);
        let text = String::from_utf8(output.stderr).unwrap();
        assert!(text.contains("description=invalid"));
        assert!(text.contains("Stores"));
        assert!(text.contains("DESCRIPTION_INVALID"));
    }
}

#[test]
fn spec_listing_distinguishes_missing_invalid_and_unavailable_requirements() {
    let root = setup();
    let knowledge = root.path().join("knowledge");
    for (content, expected) in [
        (requirements(""), "missing"),
        (requirements("description: []\n"), "invalid"),
        ("not an OKF document".to_owned(), "unavailable"),
    ] {
        write(&knowledge, "specs/example/requirements.md", &content);
        let output = cli::spec_list(root.path());
        assert!(output.success);
        assert!(
            String::from_utf8(output.stdout)
                .unwrap()
                .contains(&format!("description={expected}"))
        );
    }
}

#[test]
fn embedded_and_custom_templates_and_one_off_materialization() {
    let root = setup();
    let knowledge = root.path().join("knowledge");
    for language in [ProjectLanguage::En, ProjectLanguage::Ja] {
        for selector in ["requirements", "design/main", "design/ui"] {
            let content = template::read_embedded(language, selector).unwrap();
            assert!(content.contains("\ndescription: "));
        }
        for selector in ["product", "tech", "structure", "document"] {
            let content = template::read_embedded_steering(language, selector).unwrap();
            assert!(content.contains("\ndescription: "));
        }
    }
    write(
        &knowledge,
        "settings/templates/specs/storage.md",
        "---\ntype: SpecBind Design\nartifact_id: storage\n---\n# Storage\n",
    );
    assert!(
        template::discover_spec_templates(&knowledge, ProjectLanguage::En)
            .issues
            .is_empty()
    );
    write(
        &knowledge,
        "settings/templates/steering/security.md",
        "---\ntype: SpecBind Steering\nartifact_id: security\n---\n# Security\n",
    );
    assert!(
        template::discover_steering_templates(&knowledge, ProjectLanguage::En)
            .issues
            .is_empty()
    );
    let scaffold = "---\ntype: SpecBind Design\nartifact_id: storage\ndescription: Owns persistence boundaries.\n---\n# Storage\n";
    let live = scaffold.replace("---\n#", "requirement_ids: ['1.1']\n---\n#");
    assert!(instruction::verify_materialization(scaffold, &live).is_empty());
    let missing = live.replace("description: Owns persistence boundaries.\n", "");
    assert!(
        instruction::verify_materialization(scaffold, &missing)
            .iter()
            .any(|issue| issue.code == "ARTIFACT_DESCRIPTION_REQUIRED")
    );
    let changed = live.replace("Owns persistence boundaries.", "Owns something else.");
    assert!(
        instruction::verify_materialization(scaffold, &changed)
            .iter()
            .any(|issue| issue.code == "ARTIFACT_DESCRIPTION_MISMATCH")
    );
    // A one-off document has no project template prerequisite.
    let main_scaffold = scaffold.replace("artifact_id: storage", "artifact_id: main");
    assert!(instruction::verify_materialization(&main_scaffold, &changed).is_empty());
    write(
        &knowledge,
        "specs/example/storage.md",
        &(live + "_Requirements: 1.1_\n"),
    );
    assert!(
        artifacts::discover_spec(&knowledge, "example")
            .issues
            .is_empty()
    );
    for (path, kind, id) in [
        ("settings/templates/specs/storage.md", "Design", "storage"),
        (
            "settings/templates/steering/security.md",
            "Steering",
            "security",
        ),
    ] {
        write(
            &knowledge,
            path,
            &format!("---\ntype: SpecBind {kind}\nartifact_id: {id}\ndescription: []\n---\n"),
        );
    }
    assert!(
        template::discover_spec_templates(&knowledge, ProjectLanguage::En)
            .issues
            .iter()
            .any(|issue| issue.code == "TEMPLATE_DESCRIPTION_INVALID")
    );
    assert!(
        template::discover_steering_templates(&knowledge, ProjectLanguage::En)
            .issues
            .iter()
            .any(|issue| issue.code == "TEMPLATE_DESCRIPTION_INVALID")
    );
}

#[test]
fn artifact_creation_check_enforces_literal_and_one_off_descriptions() {
    let root = setup();
    let knowledge = root.path().join("knowledge");
    let scaffold = "---\ntype: SpecBind Design\nartifact_id: main\ndescription: Owns overall integration.\n---\n# Design\n";
    write(&knowledge, "settings/templates/specs/design.md", scaffold);
    let live =
        scaffold.replace("---\n#", "requirement_ids: ['1.1']\n---\n#") + "_Requirements: 1.1_\n";
    write(&knowledge, "specs/example/design.md", &live);
    let output = cli::artifact_check(root.path(), "example", "design/main", "design/main");
    assert!(
        output.success,
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let changed = live.replace("Owns overall integration.", "Owns deployment and recovery.");
    write(&knowledge, "specs/example/design.md", &changed);
    let output = cli::artifact_check(root.path(), "example", "design/main", "design/main");
    assert!(!output.success);
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("ARTIFACT_DESCRIPTION_MISMATCH")
    );
    write(&knowledge, "specs/example/design.md", &live);
    write(
        &knowledge,
        "specs/example/runtime.md",
        &changed.replace("artifact_id: main", "artifact_id: runtime"),
    );
    assert!(cli::artifact_check(root.path(), "example", "design/runtime", "design/main").success);
    assert!(!cli::artifact_check(root.path(), "example", "design/runtime", "design/ui").success);
    assert!(!cli::artifact_check(root.path(), "example", "design/missing", "design/main").success);
}

#[test]
fn migration_probe_works_before_normal_load_and_resumes_from_remaining_targets() {
    let root = setup();
    let knowledge = root.path().join("knowledge");
    write(
        root.path(),
        ".specbind.json",
        r#"{"schemaVersion":99,"specDir":"knowledge","language":"old-language"}"#,
    );
    write(
        &knowledge,
        "specs/example/requirements.md",
        &requirements(""),
    );
    write(&knowledge, "specs/example/log.md", "# History\n");
    write(&knowledge, "specs/example/spec.yaml", "old state format\n");
    let before = fs::read(knowledge.join("specs/example/requirements.md")).unwrap();
    let first = project::plan(root.path(), "1.4.4", Some("1.5.0")).unwrap();
    let req = first
        .entries
        .iter()
        .find(|entry| entry.id == "requirements-description")
        .unwrap();
    assert!(matches!(req.status, project::Status::Pending));
    assert_eq!(req.targets, ["specs/example/requirements.md"]);
    assert_eq!(
        before,
        fs::read(knowledge.join("specs/example/requirements.md")).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&first).unwrap(),
        serde_json::to_value(project::plan(root.path(), "1.4.4", Some("1.5.0")).unwrap()).unwrap()
    );
    write(
        &knowledge,
        "specs/example/requirements.md",
        &requirements("description: Stores items.\n"),
    );
    let next = project::plan(root.path(), "1.4.4", Some("1.5.0")).unwrap();
    let req = next
        .entries
        .iter()
        .find(|entry| entry.id == "requirements-description")
        .unwrap();
    assert!(matches!(req.status, project::Status::Complete));
    assert!(req.targets.is_empty());
    assert_eq!(
        fs::read_to_string(knowledge.join("specs/example/spec.yaml")).unwrap(),
        "old state format\n"
    );
    write(
        &knowledge,
        "specs/example/requirements.md",
        &requirements("description: null\n"),
    );
    let output = cli::project_migration_plan(root.path(), "1.4.4", Some("1.5.0"), true);
    assert!(!output.success);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["entries"][1]["status"], "blocked");
}

#[test]
fn metadata_reconciliation_changes_fingerprints_without_touching_gate_state() {
    let root = setup();
    let knowledge = root.path().join("knowledge");
    write(
        &knowledge,
        "specs/example/requirements.md",
        &requirements(""),
    );
    write(
        &knowledge,
        "specs/example/spec.yaml",
        "schema_version: 1\nactive_change: null\n",
    );
    let old = artifacts::resolve_gate_inputs(&knowledge, "example")
        .inputs
        .requirements
        .unwrap();
    let state = fs::read(knowledge.join("specs/example/spec.yaml")).unwrap();
    write(
        &knowledge,
        "specs/example/requirements.md",
        &requirements("description: Stores current items.\n"),
    );
    let new = artifacts::resolve_gate_inputs(&knowledge, "example")
        .inputs
        .requirements
        .unwrap();
    assert_ne!(old, new);
    assert_eq!(
        state,
        fs::read(knowledge.join("specs/example/spec.yaml")).unwrap()
    );
}

#[test]
fn raw_probe_rejects_escaping_roots_and_reopens_missing_metadata() {
    let root = setup();
    for spec_dir in ["../outside", "/absolute", ".", "knowledge/../outside"] {
        write(
            root.path(),
            ".specbind.json",
            &format!(r#"{{"specDir":"{spec_dir}"}}"#),
        );
        assert!(project::plan(root.path(), "1.4.4", Some("1.5.0")).is_err());
    }
    write(root.path(), ".specbind.json", r#"{"specDir":"knowledge"}"#);
    let knowledge = root.path().join("knowledge");
    write(
        &knowledge,
        "steering/product.md",
        "---\ntype: SpecBind Steering\nartifact_id: product\ndescription: Sets product scope.\n---\n",
    );
    assert!(matches!(
        project::plan(root.path(), "1.4.4", Some("1.5.0"))
            .unwrap()
            .entries[2]
            .status,
        project::Status::Complete
    ));
    write(
        &knowledge,
        "steering/product.md",
        "---\ntype: SpecBind Steering\nartifact_id: product\n---\n",
    );
    assert!(matches!(
        project::plan(root.path(), "1.4.4", Some("1.5.0"))
            .unwrap()
            .entries[2]
            .status,
        project::Status::Pending
    ));
}

#[cfg(unix)]
#[test]
fn raw_probe_never_follows_symlinked_directories_or_documents() {
    let root = setup();
    let outside = tempfile::tempdir().unwrap();
    write(outside.path(), "requirements.md", &requirements(""));
    std::os::unix::fs::symlink(outside.path(), root.path().join("knowledge/specs")).unwrap();
    let plan = project::plan(root.path(), "1.4.4", Some("1.5.0")).unwrap();
    assert!(matches!(plan.entries[1].status, project::Status::Blocked));
    assert!(plan.entries[1].targets.is_empty());
}
