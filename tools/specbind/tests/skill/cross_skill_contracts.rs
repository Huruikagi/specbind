#[test]
fn drive_uses_authoritative_actions_and_parks_local_attention() {
    let drive = skill::find("sb-drive").expect("drive skill");
    let metadata = drive.metadata().expect("drive metadata");
    assert!(metadata.description.contains("safe reachable"));
    let body = drive.body().expect("drive body");
    for required in [
        "specbind milestone status --json",
        "typed `handler`",
        "Never\nreconstruct an owner from `action`",
        "handler.kind=skill",
        "handler.kind=guarded_cli",
        "handler.kind=boundary",
        "project working directory",
        "project-local executable and PATH facts",
        "exact applicable project\n  instruction paths",
        "before it delegates any internal role",
        "complete owning workflow package",
        "never dispatch `specbind-implementer`",
        "An internal\n  `READY_FOR_REVIEW` result is not an owning-workflow result",
        "handler.mode=reverse_resume",
        "CONTINUE_ELSEWHERE",
        "STOP_RUN",
        "HUMAN_DECISION",
        "Create no queue, checkpoint, batch status, or authority artifact.",
        "Never dispatch",
        "`sb-release`",
        "One mutating owner at a time",
    ] {
        assert!(body.contains(required), "drive must contain {required}");
    }
    assert!(!body.contains("| Status action | Owner |"));
}

#[test]
fn implementation_re_reviews_only_a_diagnosed_review_scope_defect_within_budget() {
    let procedure = skill_resource_text("sb-implement", "references/spec-backed.md");
    for required in [
        "diagnosis returns `REVIEW`",
        "no approved current-Task input must change",
        "dispatch a fresh independent reviewer",
        "but no desired verdict",
        "never adds or resets an attempt",
        "Diagnosis is not approval",
        "You are the cycle owner",
        "Never delegate this whole cycle",
        "stop after an `APPROVED` review",
        "you must resume as cycle owner and run\n`tasks complete`",
        "`PLAN`\nand `ARTIFACT` still leave the run",
    ] {
        assert!(
            procedure.contains(required),
            "implementation recovery must contain {required}"
        );
    }

    let review = skill::find("sb-review-task")
        .expect("review skill")
        .body()
        .expect("review body");
    assert!(review.contains("Another Task, Spec, or later lifecycle boundary"));
    assert!(review.contains("unimplemented downstream connection"));
}

#[test]
fn drive_replan_delegation_reaches_owners_without_expanding_requirements() {
    for agent in [Agent::Codex, Agent::ClaudeCode] {
        let rendered = skill::find("sb-drive")
            .expect("drive")
            .render(agent)
            .expect("render");
        assert!(rendered.contains("--replan"));
    }
    let drive_entrypoint = skill::find("sb-drive")
        .expect("drive")
        .body()
        .expect("body");
    assert!(drive_entrypoint.contains("references/replan.md"));
    assert!(drive_entrypoint.contains("do not ask for it again"));
    let drive = skill_resource_text("sb-drive", "references/replan.md");
    for text in [
        "do not ask for a second delegation confirmation",
        "original approved Requirements",
        "same unresolved defect after recovery is parked",
        "resume\nimplementation and validation in this same Drive run",
        "installed `references/replan.md` path from that Skill",
    ] {
        assert!(
            drive.contains(text),
            "missing Drive recovery boundary: {text}"
        );
    }
    let replan = skill_resource_text("sb-plan", "references/replan.md");
    for text in [
        "workflow\n`sb-drive`",
        "Requirements and\nscope/dependencies remain fixed",
        "recoverable Git revision",
        "every Spec-backed participant",
        "fresh independent `sb-validate-design`",
        "Removed or changed work must not inherit a\ncompleted record",
        "Return to the invoking Drive",
        "finding\nhistory survive all recovery dispatches",
    ] {
        assert!(
            replan.contains(text),
            "missing Plan recovery boundary: {text}"
        );
    }
    for phase in ["design", "tasks"] {
        let procedure = skill_resource_text("sb-plan", &format!("references/{phase}.md"));
        assert!(procedure.contains("explicit Drive replan authority"));
        assert!(procedure.contains("ordinary delegated authority does not cover this"));
    }
    let implementation = skill_resource_text("sb-implement", "references/spec-backed.md");
    assert!(implementation.contains("specbind tasks reopen <spec> <task-id>"));
    assert!(implementation.contains("Reopening grants no completion evidence"));
}

#[test]
fn embeds_the_accepted_skill_set_with_valid_metadata() {
    let names = skill::all()
        .iter()
        .map(|entry| entry.name)
        .collect::<Vec<_>>();
    assert_eq!(names, ACCEPTED_SKILLS);
    let durable = skill::installed(false)
        .map(|entry| entry.name)
        .collect::<Vec<_>>();
    assert_eq!(durable.len(), 15);
    assert!(!durable.contains(&"sb-adopt"));
    assert_eq!(skill::installed(true).count(), ACCEPTED_SKILLS.len());

    for entry in skill::all() {
        let metadata = entry.metadata().expect("parseable Front Matter");
        assert_eq!(metadata.name, entry.name, "name must match the directory");
        assert!(!metadata.description.trim().is_empty());
        let body = entry.body().expect("body");
        assert!(
            body.trim_start().starts_with("# "),
            "{}: body must open with a title heading",
            entry.name
        );
    }
}

#[test]
fn codex_skill_interface_metadata_is_branded_and_invocable() {
    let mut display_names = std::collections::BTreeSet::new();
    for entry in skill::all() {
        assert!(
            entry.display_name.starts_with("SpecBind "),
            "{}",
            entry.name
        );
        assert!(display_names.insert(entry.display_name), "{}", entry.name);
        assert!(
            (25..=64).contains(&entry.short_description.chars().count()),
            "{}: short_description must contain 25 to 64 characters",
            entry.name
        );
        assert!(
            entry.default_prompt.contains(&format!("${}", entry.name)),
            "{}: default_prompt must name the exact Skill",
            entry.name
        );

        for agent in [Agent::ClaudeCode, Agent::Codex, Agent::Generic] {
            let files = entry.render_files(agent).expect("rendered package");
            let metadata = files
                .iter()
                .find(|file| file.target.ends_with("/agents/openai.yaml"));
            if agent == Agent::Codex {
                let metadata = metadata.expect("Codex UI metadata");
                let value: serde_json::Value =
                    serde_saphyr::from_str(&metadata.content).expect("valid openai.yaml");
                assert_eq!(
                    value["interface"]["display_name"].as_str(),
                    Some(entry.display_name)
                );
                assert_eq!(
                    value["interface"]["short_description"].as_str(),
                    Some(entry.short_description)
                );
                assert_eq!(
                    value["interface"]["default_prompt"].as_str(),
                    Some(entry.default_prompt)
                );
                assert!(value.get("policy").is_none());
                assert!(value.get("dependencies").is_none());
            } else {
                assert!(
                    metadata.is_none(),
                    "{agent:?} must not receive OpenAI metadata"
                );
            }
        }
    }
}

#[test]
fn every_product_skill_consumes_the_shared_language_style_rule() {
    for entry in skill::all() {
        let body = entry.body().expect("body");
        assert_eq!(
            body.matches("specbind rule read language-style --for consume")
                .count(),
            1,
            "{} must read the language-style Rule exactly once",
            entry.name
        );
        assert!(
            body.contains("Apply returned policy only to natural-language prose."),
            "{} must keep exact machine text outside the prose policy",
            entry.name
        );
    }
}

#[test]
fn renders_only_the_accepted_front_matter_per_agent() {
    for entry in skill::all() {
        let body = entry.body().expect("body");
        for agent in [Agent::ClaudeCode, Agent::Codex, Agent::Generic] {
            let rendered = entry.render(agent).expect("rendered skill");
            assert!(
                rendered.ends_with(body),
                "{}: {agent:?} must keep the body unchanged",
                entry.name
            );
            let frontmatter = rendered
                .strip_prefix("---\n")
                .and_then(|rest| rest.split_once("\n---\n"))
                .expect("rendered Front Matter")
                .0;
            assert!(frontmatter.contains("name: "));
            assert!(frontmatter.contains("description: "));
            // A permission grant or invocation restriction is never inferred
            // from skill content.
            for forbidden in ["allowed-tools", "disable-model-invocation"] {
                assert!(
                    !rendered.contains(forbidden),
                    "{}: {agent:?} must not emit {forbidden}",
                    entry.name
                );
            }
            if matches!(agent, Agent::Codex | Agent::Generic) {
                assert!(
                    !frontmatter.contains("argument-hint"),
                    "{}: Codex Front Matter is name and description only",
                    entry.name
                );
            }
        }
    }
}

#[test]
fn installs_each_skill_to_the_accepted_target() {
    for entry in skill::all() {
        assert_eq!(
            entry.target(Agent::ClaudeCode),
            format!(".claude/skills/{}/SKILL.md", entry.name)
        );
        assert_eq!(
            entry.target(Agent::Codex),
            format!(".agents/skills/{}/SKILL.md", entry.name)
        );
        assert_eq!(
            entry.target(Agent::Generic),
            format!(".agents/skills/{}/SKILL.md", entry.name)
        );
        assert!(
            entry
                .targets(Agent::Codex)
                .contains(&format!(".agents/skills/{}/agents/openai.yaml", entry.name))
        );
        assert!(
            !entry
                .targets(Agent::ClaudeCode)
                .iter()
                .any(|target| target.ends_with("/agents/openai.yaml"))
        );
        assert!(
            !entry
                .targets(Agent::Generic)
                .iter()
                .any(|target| target.ends_with("/agents/openai.yaml"))
        );
    }
}

#[test]
fn progressive_skill_packages_carry_only_directly_routed_reference_files() {
    for (name, expected_resources) in [
        ("sb-configure", 7),
        ("sb-discovery", 3),
        ("sb-drive", 1),
        ("sb-implement", 2),
        ("sb-plan", 6),
        ("sb-release", 1),
    ] {
        let entry = skill::find(name).expect("progressive skill package");
        let resources = entry.resources();
        assert_eq!(resources.len(), expected_resources, "{name}");
        let body = entry.body().expect("body");
        for resource in resources {
            assert!(resource.relative_path.starts_with("references/"));
            assert!(!resource.relative_path.contains(".."));
            assert!(
                body.contains(resource.relative_path),
                "{name} entrypoint must directly route {}",
                resource.relative_path
            );
            assert!(!resource.content().trim().is_empty());
        }
        for agent in [Agent::ClaudeCode, Agent::Codex, Agent::Generic] {
            let files = entry.render_files(agent).expect("rendered package");
            let expected_files = expected_resources + 1 + usize::from(agent == Agent::Codex);
            assert_eq!(files.len(), expected_files, "{name}: {agent:?}");
            assert!(files.iter().any(|file| file.target.ends_with("/SKILL.md")));
        }
    }
}

use super::*;
