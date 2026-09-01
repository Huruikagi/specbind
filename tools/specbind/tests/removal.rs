use std::{fs, path::Path, process::Command as ProcessCommand};

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn command(root: &Path) -> Command {
    let mut command = Command::cargo_bin("specbind").expect("specbind binary should build");
    command.current_dir(root);
    command
}

fn git(root: &Path, arguments: &[&str]) {
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .expect("git should start");
    assert!(
        output.status.success(),
        "git {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn installed(agents: &[&str], project_instructions: bool) -> TempDir {
    let root = tempfile::tempdir().expect("fixture root");
    git(root.path(), &["init", "-q"]);
    git(root.path(), &["config", "user.email", "test@example.com"]);
    git(root.path(), &["config", "user.name", "Test User"]);
    fs::write(root.path().join("README.md"), "# Fixture\n").expect("seed project");
    fs::write(root.path().join("AGENTS.md"), "# Project\n\nCodex rules.\n")
        .expect("codex instructions");
    fs::write(
        root.path().join("CLAUDE.md"),
        "# Project\n\nClaude rules.\n",
    )
    .expect("claude instructions");
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-qm", "seed"]);

    let mut install = command(root.path());
    install.args(["install", "--language", "en"]);
    for agent in agents {
        install.args(["--agent", agent]);
    }
    if project_instructions {
        install.arg("--project-instructions");
    }
    install.assert().success();
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-qm", "install"]);
    root
}

#[test]
fn plans_then_applies_one_agent_removal_without_touching_the_other_agent_or_knowledge() {
    let root = installed(&["codex", "claude-code"], true);
    fs::write(
        root.path().join(".specbind.json"),
        r#"{
  "schemaVersion": 1,
  "specDir": ".specbind",
  "language": "en",
  "agents": ["claude-code", "codex"],
  "projectInstructions": true,
  "agentRoles": {
    "codex": { "planner": { "model": "gpt-5.6-sol" } },
    "claudeCode": { "planner": { "model": "opus" } }
  }
}
"#,
    )
    .expect("agent role overrides");
    git(root.path(), &["add", ".specbind.json"]);
    git(root.path(), &["commit", "-qm", "configure role policy"]);
    command(root.path()).arg("install").assert().success();
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-qm", "configure roles"]);

    command(root.path())
        .args(["remove-agent", "codex"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK AGENT_REMOVAL_PLANNED:"))
        .stdout(predicate::str::contains(
            "remove .agents/skills/sb-status/SKILL.md [skill]",
        ))
        .stdout(predicate::str::contains(
            "remove .agents/skills/sb-configure/references/aftercare.md [skill]",
        ))
        .stdout(predicate::str::contains("retain AGENTS.md").not());
    assert!(
        root.path()
            .join(".agents/skills/sb-status/SKILL.md")
            .is_file(),
        "planning must not mutate"
    );

    command(root.path())
        .args(["remove-agent", "codex", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK AGENT_REMOVAL_APPLIED:"));

    assert!(
        !root
            .path()
            .join(".agents/skills/sb-status/SKILL.md")
            .exists()
    );
    assert!(
        !root
            .path()
            .join(".agents/skills/sb-configure/references/aftercare.md")
            .exists()
    );
    assert!(
        !root
            .path()
            .join(".codex/agents/specbind-planner.toml")
            .exists()
    );
    assert!(
        root.path()
            .join(".claude/skills/sb-status/SKILL.md")
            .is_file()
    );
    assert!(
        root.path()
            .join(".claude/agents/specbind-planner.md")
            .is_file()
    );
    assert!(root.path().join(".specbind").is_dir());
    let agents = fs::read_to_string(root.path().join("AGENTS.md")).expect("AGENTS.md retained");
    assert_eq!(agents, "# Project\n\nCodex rules.\n\n");
    let claude = fs::read_to_string(root.path().join("CLAUDE.md")).expect("CLAUDE.md retained");
    assert!(claude.contains("<!-- specbind:block -->"));
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(config.contains("claude-code"));
    assert!(!config.contains("codex"));
    assert!(config.contains("claudeCode"));
    assert!(config.contains("opus"));
    let claude_planner = fs::read_to_string(root.path().join(".claude/agents/specbind-planner.md"))
        .expect("remaining Claude role");
    assert!(claude_planner.contains("model: opus"));

    command(root.path())
        .args(["remove-agent", "codex", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE AGENT_ALREADY_REMOVED:",
        ));
}

#[test]
fn refuses_to_remove_the_last_agent() {
    let root = installed(&["codex"], false);
    command(root.path())
        .args(["remove-agent", "codex"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("REMOVE_AGENT_LAST_AGENT"));
}

#[test]
fn removing_codex_retains_surfaces_shared_with_generic() {
    let root = installed(&["codex", "generic"], true);

    command(root.path())
        .args(["remove-agent", "codex"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "retain .agents/skills/sb-status/SKILL.md [skill]",
        ))
        .stdout(predicate::str::contains(
            "retain .agents/skills/sb-configure/references/aftercare.md [skill]",
        ))
        .stdout(predicate::str::contains(
            "remove .codex/agents/specbind-planner.toml [agent-role]",
        ))
        .stdout(predicate::str::contains(
            "retain AGENTS.md [project-instructions]",
        ));

    command(root.path())
        .args(["remove-agent", "codex", "--apply"])
        .assert()
        .success();

    assert!(
        root.path()
            .join(".agents/skills/sb-status/SKILL.md")
            .is_file()
    );
    assert!(root.path().join("AGENTS.md").is_file());
    assert!(
        !root
            .path()
            .join(".codex/agents/specbind-planner.toml")
            .exists()
    );
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(config.contains("generic"));
    assert!(!config.contains("codex"));
}

#[test]
fn removing_generic_retains_surfaces_required_by_codex() {
    let root = installed(&["codex", "generic"], true);

    command(root.path())
        .args(["remove-agent", "generic", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "retain .agents/skills/sb-status/SKILL.md [skill]",
        ))
        .stdout(predicate::str::contains(
            "retain AGENTS.md [project-instructions]",
        ));

    assert!(
        root.path()
            .join(".agents/skills/sb-status/SKILL.md")
            .is_file()
    );
    assert!(
        root.path()
            .join(".codex/agents/specbind-planner.toml")
            .is_file()
    );
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(config.contains("codex"));
    assert!(!config.contains("generic"));
}

#[test]
fn removing_generic_drops_only_its_unshared_surfaces() {
    let root = installed(&["generic", "claude-code"], true);

    command(root.path())
        .args(["remove-agent", "generic", "--apply"])
        .assert()
        .success();

    assert!(
        !root
            .path()
            .join(".agents/skills/sb-status/SKILL.md")
            .exists()
    );
    assert_eq!(
        fs::read_to_string(root.path().join("AGENTS.md")).expect("AGENTS.md"),
        "# Project\n\nCodex rules.\n\n"
    );
    assert!(
        root.path()
            .join(".claude/skills/sb-status/SKILL.md")
            .is_file()
    );
    assert!(root.path().join("CLAUDE.md").is_file());
}

#[test]
fn uninstall_deduplicates_generic_and_codex_shared_surfaces() {
    let root = installed(&["generic", "codex"], true);

    command(root.path())
        .args(["uninstall", "--knowledge", "retain", "--apply"])
        .assert()
        .success();

    assert!(!root.path().join(".specbind.json").exists());
    assert!(
        !root
            .path()
            .join(".agents/skills/sb-status/SKILL.md")
            .exists()
    );
    assert!(
        !root
            .path()
            .join(".codex/agents/specbind-planner.toml")
            .exists()
    );
    assert_eq!(
        fs::read_to_string(root.path().join("AGENTS.md")).expect("AGENTS.md"),
        "# Project\n\nCodex rules.\n\n"
    );
}

#[test]
fn uninstall_retain_keeps_the_complete_spec_dir_and_surrounding_instructions() {
    let root = installed(&["codex"], true);
    let durable = root.path().join(".specbind/specs/example/requirements.md");
    fs::create_dir_all(durable.parent().expect("requirements parent")).expect("spec directory");
    fs::write(&durable, "# Requirements\n").expect("durable requirement");
    git(root.path(), &["add", "."]);
    git(root.path(), &["commit", "-qm", "knowledge"]);

    command(root.path())
        .args(["uninstall", "--knowledge", "retain"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK PROJECT_UNINSTALL_PLANNED:"))
        .stdout(predicate::str::contains("retain .specbind [knowledge]"));
    assert!(root.path().join(".specbind.json").is_file());

    command(root.path())
        .args(["uninstall", "--knowledge", "retain", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK PROJECT_UNINSTALL_APPLIED:"));
    assert!(!root.path().join(".specbind.json").exists());
    assert!(durable.is_file());
    assert!(
        !root
            .path()
            .join(".agents/skills/sb-status/SKILL.md")
            .exists()
    );
    assert_eq!(
        fs::read_to_string(root.path().join("AGENTS.md")).expect("project instructions"),
        "# Project\n\nCodex rules.\n\n"
    );

    command(root.path())
        .args(["uninstall", "--knowledge", "retain", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "NO_CHANGE PROJECT_ALREADY_UNINSTALLED:",
        ));
}

#[test]
fn uninstall_remove_deletes_the_tracked_spec_dir_and_project_integration() {
    let root = installed(&["codex"], true);
    command(root.path())
        .args(["uninstall", "--knowledge", "remove", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK PROJECT_UNINSTALL_APPLIED:"))
        .stdout(predicate::str::contains("remove .specbind [knowledge]"));
    assert!(!root.path().join(".specbind").exists());
    assert!(!root.path().join(".specbind.json").exists());
    assert!(
        !root
            .path()
            .join(".agents/skills/sb-status/SKILL.md")
            .exists()
    );
}

#[test]
fn uninstall_remove_rejects_ignored_content_below_spec_dir() {
    let root = installed(&["codex"], false);
    fs::write(root.path().join(".gitignore"), ".specbind/local.tmp\n").expect("ignore rule");
    git(root.path(), &["add", ".gitignore"]);
    git(root.path(), &["commit", "-qm", "ignore local data"]);
    fs::write(root.path().join(".specbind/local.tmp"), "machine local\n").expect("ignored file");

    command(root.path())
        .args(["uninstall", "--knowledge", "remove"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("REMOVAL_TARGET_IGNORED"));
    assert!(root.path().join(".specbind/local.tmp").is_file());
    assert!(root.path().join(".specbind.json").is_file());
}

#[test]
fn removal_rejects_unrelated_dirty_worktree_state() {
    let root = installed(&["codex", "claude-code"], false);
    fs::write(root.path().join("README.md"), "dirty\n").expect("dirty project file");
    command(root.path())
        .args(["remove-agent", "codex"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("REMOVAL_REPOSITORY_DIRTY"))
        .stderr(predicate::str::contains("README.md"));
}

#[test]
fn apply_converges_from_exact_already_removed_targets() {
    let root = installed(&["codex", "claude-code"], true);
    fs::remove_file(root.path().join(".agents/skills/sb-status/SKILL.md"))
        .expect("simulate interrupted skill removal");
    let agents_path = root.path().join("AGENTS.md");
    let agents = fs::read_to_string(&agents_path).expect("AGENTS.md");
    let without_block = specbind::project_instructions::remove(&agents)
        .expect("valid markers")
        .expect("block exists");
    fs::write(&agents_path, without_block).expect("simulate completed block removal");

    command(root.path())
        .args(["remove-agent", "codex", "--apply"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("OK AGENT_REMOVAL_APPLIED:"))
        .stdout(predicate::str::contains(
            "absent .agents/skills/sb-status/SKILL.md",
        ))
        .stdout(predicate::str::contains("absent AGENTS.md"));
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(!config.contains("codex"));
    assert!(
        root.path()
            .join(".claude/skills/sb-status/SKILL.md")
            .is_file()
    );
}
