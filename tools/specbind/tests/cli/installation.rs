use super::*;

fn assert_codex_status_metadata(root: &Path) {
    let metadata = fs::read_to_string(root.join(".agents/skills/sb-status/agents/openai.yaml"))
        .expect("rendered Codex skill metadata");
    assert!(
        metadata.contains("display_name: \"SpecBind Status\""),
        "{metadata}"
    );
    assert!(metadata.contains("Use $sb-status"), "{metadata}");
}

#[test]
fn plans_an_initial_installation_without_writing() {
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
            "--agent",
            "claude-code",
            "--language",
            "ja",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK INSTALL_PLANNED: Planned 105 action(s) for 2 agent(s).\n",
            )
            .and(predicate::str::contains("\n  Mode: initial\n"))
            .and(predicate::str::contains("\n  Language: ja\n"))
            .and(predicate::str::contains("\n  Agents: claude-code, codex\n"))
            .and(predicate::str::contains(
                "\n  Project instructions: disabled\n",
            ))
            .and(predicate::str::contains(
                "- create .specbind.json [config]\n",
            ))
            .and(predicate::str::contains(
                "- create .specbind/settings/templates/specs/requirements.md [template]\n",
            ))
            .and(predicate::str::contains(
                "- create .specbind/settings/templates/roadmap.md [template]\n",
            ))
            .and(predicate::str::contains(
                "- create .specbind/settings/rules/language-style.md [rule]\n",
            ))
            .and(predicate::str::contains(
                "\n  Summary: 105 create, 0 replace, 0 keep, 0 remove\n",
            ))
            .and(predicate::str::contains("Next:").not()),
        )
        .stderr("");

    assert!(
        !root.path().join(".specbind.json").exists(),
        "a dry run must not write the configuration"
    );
    assert!(
        !root.path().join(".specbind").exists(),
        "a dry run must not create the spec root"
    );
}
#[test]
fn requires_explicit_inputs_for_an_initial_installation() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut missing_language = specbind_command();
    missing_language
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "codex"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR INSTALL_PLAN_FAILED: Cannot plan the SpecBind installation.",
            )
            .and(predicate::str::contains("INSTALL_LANGUAGE_REQUIRED")),
        );

    let mut missing_agent = specbind_command();
    missing_agent
        .current_dir(root.path())
        .args(["install", "--dry-run", "--language", "en"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_AGENT_REQUIRED"));
}

#[test]
fn keeps_project_owned_settings_and_guards_replacements() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind/settings/templates/specs/design.md",
        "---\ntype: SpecBind Design\nartifact_id: main\n---\n# Project design scaffold\n",
    );

    let mut unchanged = specbind_command();
    unchanged
        .current_dir(root.path())
        .args(["install", "--dry-run"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("\n  Mode: refresh\n")
                .and(predicate::str::contains(
                    "- keep .specbind.json [config] (already matches the requested inputs)\n",
                ))
                .and(predicate::str::contains(
                    "- keep .specbind/settings/templates/specs/design.md [template] (project-owned settings are never overwritten)\n",
                ))
                .and(predicate::str::contains(
                    "\n  Summary: 65 create, 0 replace, 2 keep, 0 remove\n",
                )),
        );

    let mut dirty = specbind_command();
    dirty
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "claude-code"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_COMMIT_REQUIRED"));

    commit_all(root.path());
    let mut replaceable = specbind_command();
    replaceable
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "claude-code"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("- replace .specbind.json [config]\n")
                .and(predicate::str::contains("\n  Agents: claude-code, codex\n")),
        );

    write(root.path(), "dirty.txt", "dirty\n");
    let mut blocked = specbind_command();
    blocked
        .current_dir(root.path())
        .args(["install", "--dry-run", "--agent", "claude-code"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_REPOSITORY_DIRTY"));
}

#[test]
fn applies_an_initial_installation_and_is_idempotent() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "en"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK INSTALL_APPLIED: Applied 67 action(s) for 1 agent(s).\n",
            )
            .and(predicate::str::contains(
                "\n  Summary: 67 created, 0 replaced, 0 kept, 0 removed\n",
            ))
            .and(predicate::str::contains(
                "\n  Next: Ask your coding agent to use sb-configure to review and configure SpecBind for this project.\n",
            )),
        )
        .stderr("");

    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("written config");
    assert_eq!(
        config,
        "{\n  \"schemaVersion\": 1,\n  \"specDir\": \".specbind\",\n  \"language\": \"en\",\n  \"agents\": [\"codex\"]\n}\n"
    );
    for relative in [
        ".specbind/settings/templates/specs/requirements.md",
        ".specbind/settings/templates/specs/design.md",
        ".specbind/settings/templates/specs/ui.md",
        ".specbind/settings/templates/roadmap.md",
        ".specbind/settings/rules/ears-format.md",
        ".specbind/settings/rules/design-template-selection.md",
        ".specbind/settings/rules/steering-principles.md",
        ".agents/skills/sb-discovery/references/adopt-start.md",
        ".agents/skills/sb-configure/SKILL.md",
        ".agents/skills/sb-configure/references/aftercare.md",
        ".agents/skills/sb-drive/SKILL.md",
        ".agents/skills/sb-implement/references/direct.md",
        ".agents/skills/sb-release/references/bootstrap-release-adapter.md",
    ] {
        assert!(root.path().join(relative).is_file(), "missing {relative}");
    }
    assert!(
        !root
            .path()
            .join(".specbind/settings/rules/language-style.md")
            .exists(),
        "English installation must not offer the Japanese style default"
    );

    let mut installed = specbind_command();
    installed
        .current_dir(root.path())
        .args(["template", "list", "spec"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "selector=requirements source=project",
        ));

    let mut again = specbind_command();
    again
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with("NO_CHANGE INSTALL_UP_TO_DATE:")
                .and(predicate::str::contains("Next:").not()),
        )
        .stderr("");
}

#[test]
fn installs_product_managed_skills_for_each_selected_agent() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args([
            "install",
            "--agent",
            "codex",
            "--agent",
            "claude-code",
            "--language",
            "en",
        ])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("- create .claude/skills/sb-status/SKILL.md [skill]\n").and(
                predicate::str::contains("- create .agents/skills/sb-status/SKILL.md [skill]\n")
                    .and(predicate::str::contains(
                        "- create .agents/skills/sb-status/agents/openai.yaml [skill]\n",
                    )),
            ),
        );

    let claude = fs::read_to_string(root.path().join(".claude/skills/sb-status/SKILL.md"))
        .expect("rendered Claude Code skill");
    let codex = fs::read_to_string(root.path().join(".agents/skills/sb-status/SKILL.md"))
        .expect("rendered Codex skill");
    assert_codex_status_metadata(root.path());
    for relative in [
        ".claude/skills/sb-plan/SKILL.md",
        ".agents/skills/sb-plan/SKILL.md",
        ".claude/skills/sb-plan/references/requirements.md",
        ".agents/skills/sb-plan/references/requirements.md",
        ".claude/skills/sb-plan/references/design.md",
        ".agents/skills/sb-plan/references/design.md",
        ".claude/skills/sb-plan/references/tasks.md",
        ".agents/skills/sb-plan/references/tasks.md",
        ".claude/skills/sb-drive/SKILL.md",
        ".agents/skills/sb-drive/SKILL.md",
        ".claude/skills/sb-configure/references/aftercare.md",
        ".agents/skills/sb-configure/references/aftercare.md",
        ".claude/skills/sb-discovery/references/adopt-start.md",
        ".agents/skills/sb-discovery/references/adopt-start.md",
        ".claude/skills/sb-implement/references/spec-backed.md",
        ".agents/skills/sb-implement/references/spec-backed.md",
        ".claude/skills/sb-release/references/bootstrap-release-adapter.md",
        ".agents/skills/sb-release/references/bootstrap-release-adapter.md",
    ] {
        assert!(root.path().join(relative).is_file(), "missing {relative}");
    }
    assert_retired_skill_files_are_absent(root.path());
    assert!(claude.starts_with("---\nname: sb-status\n"), "{claude}");
    assert!(claude.contains("argument-hint:"), "{claude}");
    assert!(!codex.contains("argument-hint:"), "{codex}");
    for forbidden in ["allowed-tools", "disable-model-invocation"] {
        assert!(!claude.contains(forbidden), "{claude}");
        assert!(!codex.contains(forbidden), "{codex}");
    }
    let body = |rendered: &str| {
        rendered
            .split_once("\n---\n")
            .and_then(|(_, rest)| rest.split_once("\n---\n"))
            .map_or_else(|| rendered.to_owned(), |(_, body)| body.to_owned())
    };
    assert_eq!(
        claude.rsplit_once("\n---\n").expect("body").1,
        codex.rsplit_once("\n---\n").expect("body").1,
        "both agents receive the same body"
    );
    let _ = body;

    // A local edit is not a customization path, and the repository guard refuses
    // to overwrite it while it is uncommitted.
    write(
        root.path(),
        ".agents/skills/sb-status/SKILL.md",
        "---\nname: sb-status\ndescription: edited\n---\n# Local\n",
    );
    let mut refresh = specbind_command();
    refresh
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .failure()
        .stdout("")
        .stderr(predicate::str::contains("INSTALL_COMMIT_REQUIRED"));

    commit_all(root.path());
    let mut restored = specbind_command();
    restored
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "- replace .agents/skills/sb-status/SKILL.md [skill]\n",
        ));
    assert_eq!(
        fs::read_to_string(root.path().join(".agents/skills/sb-status/SKILL.md"))
            .expect("refreshed skill"),
        codex,
        "a refresh restores the product asset"
    );
}

#[test]
fn refresh_removes_retired_skill_packages_under_the_repository_guard() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    let mut install = specbind_command();
    install
        .current_dir(root.path())
        .args([
            "install",
            "--agent",
            "codex",
            "--agent",
            "claude-code",
            "--language",
            "en",
        ])
        .assert()
        .success();
    commit_all(root.path());

    for agent_root in [".agents/skills", ".claude/skills"] {
        for retired in specbind::skill::retired_names() {
            for file in specbind::skill::retired_files(retired) {
                write(
                    root.path(),
                    &format!("{agent_root}/{retired}/{file}"),
                    "former product-managed skill file\n",
                );
            }
        }
    }
    write(
        root.path(),
        ".agents/skills/specbind-adopt-existing/notes.md",
        "project-owned extra content\n",
    );
    let mut dirty_retire = specbind_command();
    dirty_retire
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("INSTALL_REPOSITORY_DIRTY"));

    commit_all(root.path());
    let mut retire = specbind_command();
    retire
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "- remove .agents/skills/specbind-plan-design/SKILL.md [skill]",
        ))
        .stdout(predicate::str::contains(
            "- remove .agents/skills/specbind-adopt-existing/references/start.md [skill]",
        ));
    assert_retired_skill_files_are_absent(root.path());
    assert!(
        root.path()
            .join(".agents/skills/sb-plan/references/design.md")
            .is_file()
    );
    assert!(
        root.path()
            .join(".agents/skills/specbind-adopt-existing/notes.md")
            .is_file(),
        "refresh must preserve extra project content"
    );
    assert!(
        !root
            .path()
            .join(".claude/skills/specbind-adopt-existing")
            .exists(),
        "refresh removes the empty retired package directory"
    );
}

fn assert_retired_skill_files_are_absent(root: &std::path::Path) {
    for removed in specbind::skill::retired_names() {
        if *removed == "specbind-adopt-existing" {
            continue;
        }
        for agent_root in [".claude/skills", ".agents/skills"] {
            assert!(
                !root.join(agent_root).join(removed).exists(),
                "removed retired package {agent_root}/{removed}"
            );
        }
    }
    for agent_root in [".claude/skills", ".agents/skills"] {
        for file in specbind::skill::retired_files("specbind-adopt-existing") {
            assert!(
                !root
                    .join(agent_root)
                    .join("specbind-adopt-existing")
                    .join(file)
                    .exists(),
                "retired adoption file {agent_root}/{file}"
            );
        }
    }
}

#[test]
fn shows_the_complete_configuration_without_claiming_global_readiness() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    let mut install = specbind_command();
    install
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "en"])
        .assert()
        .success();

    let mut show = specbind_command();
    show.current_dir(root.path())
        .args(["configuration", "show"])
        .assert()
        .success()
        .stdout(
            predicate::str::starts_with(
                "OK CONFIGURATION_SHOWN: Current SpecBind configuration.\n",
            )
            .and(predicate::str::contains("    Agents: codex\n"))
            .and(predicate::str::contains(
                "    codex/implementer: state=default model=gpt-5.6-terra reasoning_effort=medium\n",
            ))
            .and(predicate::str::contains(
                "    spec/requirements: current-default\n",
            ))
            .and(predicate::str::contains(
                "    design-template-selection: current-default\n",
            ))
            .and(predicate::str::contains("    language-style: absent\n"))
            .and(predicate::str::contains("    release: scaffold\n"))
            .and(predicate::str::contains("    validation: scaffold\n"))
            .and(predicate::str::contains("    Documents: 0\n"))
            .and(predicate::str::contains(
                "    - Release adapter is not configured\n",
            ))
            .and(predicate::str::contains(
                "    - no Steering documents are present\n",
            ))
            .and(predicate::str::contains("READY").not())
            .and(predicate::str::contains("CONFIGURED").not()),
        )
        .stderr("");

    write(
        root.path(),
        ".specbind/settings/rules/design-template-selection.md",
        concat!(
            "---\ntype: SpecBind Rule\n---\n# Selection\n\n",
            "## `design/main`\n\nMode: required\n\nAlways.\n",
        ),
    );
    let mut invalid = specbind_command();
    invalid
        .current_dir(root.path())
        .args(["configuration", "show"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR CONFIGURATION_SHOW_FAILED: Cannot summarize the current SpecBind configuration.\n",
            )
            .and(predicate::str::contains(
                "RULE_DESIGN_TEMPLATE_SELECTOR_MISSING design/ui",
            )),
        );
}

#[test]
fn installs_generic_shared_surfaces_once_without_generic_roles() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = specbind_command();
    let output = apply
        .current_dir(root.path())
        .args([
            "install",
            "--agent",
            "generic",
            "--agent",
            "codex",
            "--language",
            "en",
            "--project-instructions",
        ])
        .output()
        .expect("generic install should run");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 output");
    assert_eq!(
        stdout
            .matches(".agents/skills/sb-status/SKILL.md [skill]")
            .count(),
        1,
        "shared Skill target must be planned once: {stdout}"
    );
    assert_eq!(
        stdout.matches("AGENTS.md [project-instructions]").count(),
        1,
        "shared instruction target must be planned once: {stdout}"
    );
    assert!(
        root.path()
            .join(".agents/skills/sb-status/SKILL.md")
            .is_file()
    );
    assert!(root.path().join("AGENTS.md").is_file());
    assert!(
        root.path()
            .join(".codex/agents/specbind-implementer.toml")
            .is_file(),
        "Codex still receives its host-specific roles"
    );
    assert!(
        !root.path().join(".generic/agents").exists(),
        "generic defines no host-specific role surface"
    );
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(config.contains(r#""agents": ["codex", "generic"]"#));
}

#[test]
fn installs_generic_without_any_host_specific_roles() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args([
            "install",
            "--agent",
            "generic",
            "--language",
            "ja",
            "--project-instructions",
        ])
        .assert()
        .success();

    assert!(
        root.path()
            .join(".agents/skills/sb-status/SKILL.md")
            .is_file()
    );
    assert!(root.path().join("AGENTS.md").is_file());
    assert!(!root.path().join(".codex/agents").exists());
    assert!(!root.path().join(".claude/agents").exists());
    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("config");
    assert!(config.contains(r#""agents": ["generic"]"#));
}

#[test]
fn installs_codex_roles_with_cost_aware_defaults() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "en"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "- create .codex/agents/specbind-implementer.toml [agent-role]",
        ));

    let role = |name: &str| {
        fs::read_to_string(
            root.path()
                .join(format!(".codex/agents/specbind-{name}.toml")),
        )
        .expect("installed Codex role")
    };
    let implementer = role("implementer");
    assert!(implementer.contains("model = \"gpt-5.6-terra\""));
    assert!(implementer.contains("model_reasoning_effort = \"medium\""));
    assert!(role("reviewer").contains("model = \"gpt-5.6-terra\""));
    assert!(role("debugger").contains("model = \"gpt-5.6-sol\""));
    assert!(role("researcher").contains("model = \"gpt-5.6-luna\""));
}

#[test]
fn applies_only_project_capability_overrides_to_codex_roles() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind.json",
        r#"{
  "schemaVersion": 1,
  "specDir": ".specbind",
  "language": "en",
  "agents": ["codex"],
  "agentRoles": {
    "codex": {
      "implementer": { "model": "gpt-5.6-luna", "reasoningEffort": "low" },
      "reviewer": { "reasoningEffort": "high" }
    }
  }
}"#,
    );
    commit_all(root.path());

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success();

    let implementer =
        fs::read_to_string(root.path().join(".codex/agents/specbind-implementer.toml"))
            .expect("overridden implementer role");
    assert!(implementer.contains("model = \"gpt-5.6-luna\""));
    assert!(implementer.contains("model_reasoning_effort = \"low\""));
    assert!(implementer.contains("Implement exactly one dispatched task."));

    let reviewer = fs::read_to_string(root.path().join(".codex/agents/specbind-reviewer.toml"))
        .expect("overridden reviewer role");
    assert!(reviewer.contains("model = \"gpt-5.6-terra\""));
    assert!(reviewer.contains("model_reasoning_effort = \"high\""));

    let mut show = specbind_command();
    show.current_dir(root.path())
        .args(["configuration", "show"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains(
                "    codex/implementer: state=overridden model=gpt-5.6-luna reasoning_effort=low\n",
            )
            .and(predicate::str::contains(
                "    codex/reviewer: state=overridden model=gpt-5.6-terra reasoning_effort=high\n",
            ))
            .and(predicate::str::contains(
                "    codex/planner: state=default model=gpt-5.6-terra reasoning_effort=medium\n",
            )),
        );
}

#[test]
fn installs_claude_roles_with_cost_aware_defaults() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args(["install", "--agent", "claude-code", "--language", "en"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "- create .claude/agents/specbind-implementer.md [agent-role]",
        ));

    let role = |name: &str| {
        fs::read_to_string(
            root.path()
                .join(format!(".claude/agents/specbind-{name}.md")),
        )
        .expect("installed Claude Code role")
    };
    let implementer = role("implementer");
    assert!(implementer.starts_with("---\nname: specbind-implementer\n"));
    assert!(implementer.contains("\nmodel: sonnet\n"));
    assert!(implementer.contains("Implement exactly one dispatched task."));
    assert!(
        !implementer.contains("reasoning"),
        "Claude Code subagents expose no reasoning-effort field"
    );
    assert!(role("planner").contains("\nmodel: sonnet\n"));
    assert!(role("reviewer").contains("\nmodel: sonnet\n"));
    assert!(role("debugger").contains("\nmodel: opus\n"));
    assert!(role("researcher").contains("\nmodel: haiku\n"));

    assert!(
        !root.path().join(".codex/agents").exists(),
        "an unselected agent receives no role rendering"
    );
}

#[test]
fn applies_only_project_capability_overrides_to_claude_roles() {
    let root = project_fixture();
    write(
        root.path(),
        ".specbind.json",
        r#"{
  "schemaVersion": 1,
  "specDir": ".specbind",
  "language": "en",
  "agents": ["claude-code"],
  "agentRoles": {
    "claudeCode": {
      "researcher": { "model": "sonnet" }
    }
  }
}"#,
    );
    commit_all(root.path());

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args(["install"])
        .assert()
        .success();

    let researcher = fs::read_to_string(root.path().join(".claude/agents/specbind-researcher.md"))
        .expect("overridden researcher role");
    assert!(researcher.contains("\nmodel: sonnet\n"));
    assert!(researcher.contains("Investigate only the bounded question"));

    let debugger = fs::read_to_string(root.path().join(".claude/agents/specbind-debugger.md"))
        .expect("default debugger role");
    assert!(debugger.contains("\nmodel: opus\n"));

    assert!(
        fs::read_to_string(root.path().join(".specbind.json"))
            .expect("configuration")
            .contains("\"claudeCode\""),
        "the configuration keeps the accepted override"
    );
}

#[test]
fn rejects_invalid_or_unselected_claude_role_overrides() {
    let invalid_model = project_fixture();
    write(
        invalid_model.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["claude-code"],"agentRoles":{"claudeCode":{"researcher":{"model":"claude sonnet"}}}}"#,
    );
    let mut invalid = specbind_command();
    invalid
        .current_dir(invalid_model.path())
        .args(["install", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("INSTALL_AGENT_ROLE_MODEL_INVALID"));

    let effort = project_fixture();
    write(
        effort.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["claude-code"],"agentRoles":{"claudeCode":{"researcher":{"reasoningEffort":"high"}}}}"#,
    );
    let mut unsupported = specbind_command();
    unsupported
        .current_dir(effort.path())
        .args(["install", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("INSTALL_CONFIG_INVALID"));

    let unselected = project_fixture();
    write(
        unselected.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["codex"],"agentRoles":{"claudeCode":{}}}"#,
    );
    let mut unused = specbind_command();
    unused
        .current_dir(unselected.path())
        .args(["install", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("INSTALL_AGENT_ROLE_UNUSED"));
}

#[test]
fn rejects_invalid_or_unselected_codex_role_overrides() {
    let invalid_model = project_fixture();
    write(
        invalid_model.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["codex"],"agentRoles":{"codex":{"implementer":{"model":"gpt-5.6/luna"}}}}"#,
    );
    let mut invalid = specbind_command();
    invalid
        .current_dir(invalid_model.path())
        .args(["install", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("INSTALL_AGENT_ROLE_MODEL_INVALID"));

    let unselected = project_fixture();
    write(
        unselected.path(),
        ".specbind.json",
        r#"{"schemaVersion":1,"specDir":".specbind","language":"en","agents":["claude-code"],"agentRoles":{"codex":{}}}"#,
    );
    let mut unused = specbind_command();
    unused
        .current_dir(unselected.path())
        .args(["install", "--dry-run"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("INSTALL_AGENT_ROLE_UNUSED"));
}

#[test]
fn never_overwrites_project_owned_settings_when_applying() {
    let root = tempfile::tempdir().expect("temporary project root");
    git(root.path(), &["init"]);
    write(
        root.path(),
        ".specbind/settings/rules/ears-format.md",
        "---\ntype: SpecBind Rule\n---\n# Project owned\n",
    );
    write(
        root.path(),
        ".specbind/settings/templates/roadmap.md",
        "---\ntype: SpecBind Roadmap\n---\n# Project roadmap\n",
    );

    let mut apply = specbind_command();
    apply
        .current_dir(root.path())
        .args(["install", "--agent", "codex", "--language", "ja"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\n  Summary: 66 created, 0 replaced, 2 kept, 0 removed\n",
        ));

    assert_eq!(
        fs::read_to_string(root.path().join(".specbind/settings/rules/ears-format.md"))
            .expect("preserved rule"),
        "---\ntype: SpecBind Rule\n---\n# Project owned\n"
    );
    assert_eq!(
        fs::read_to_string(root.path().join(".specbind/settings/templates/roadmap.md"))
            .expect("preserved roadmap template"),
        "---\ntype: SpecBind Roadmap\n---\n# Project roadmap\n"
    );
    let template = fs::read_to_string(
        root.path()
            .join(".specbind/settings/templates/specs/requirements.md"),
    )
    .expect("installed template");
    assert!(
        template.contains("requirement: 要件"),
        "the configured language must select the installed template"
    );
    assert!(
        root.path()
            .join(".specbind/settings/rules/language-style.md")
            .is_file(),
        "Japanese installation must offer the language-style default"
    );

    let mut show = specbind_command();
    show.current_dir(root.path())
        .args(["configuration", "show"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "    language-style: current-default\n",
        ));
}

#[test]
fn guards_a_configuration_replacement_when_applying() {
    let root = project_fixture();

    let mut dirty = specbind_command();
    dirty
        .current_dir(root.path())
        .args(["install", "--agent", "claude-code"])
        .assert()
        .failure()
        .stdout("")
        .stderr(
            predicate::str::starts_with(
                "ERROR INSTALL_FAILED: Cannot apply the SpecBind installation.",
            )
            .and(predicate::str::contains("INSTALL_COMMIT_REQUIRED")),
        );

    commit_all(root.path());
    let mut allowed = specbind_command();
    allowed
        .current_dir(root.path())
        .args(["install", "--agent", "claude-code"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "- replace .specbind.json [config]",
        ));

    let config = fs::read_to_string(root.path().join(".specbind.json")).expect("rewritten config");
    assert!(
        config.contains("\"agents\": [\"claude-code\", \"codex\"]"),
        "{config}"
    );
}
