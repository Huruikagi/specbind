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
fn owning_workflows_wait_without_replacing_live_receivers() {
    for agent in [Agent::Codex, Agent::ClaudeCode] {
        let drive = skill::find("sb-drive")
            .expect("drive")
            .render(agent)
            .expect("render");
        let implementation = skill::find("sb-implement")
            .expect("implement")
            .render(agent)
            .expect("render");
        for required in [
            "continue that same\nreceiver once",
            "Never start a replacement\nowner over its partial work",
            "receiver identity, last progress, and exact partial",
            "An actual capacity stop follows the separate contract",
        ] {
            assert!(
                drive.contains(required),
                "missing owner recovery: {required}"
            );
        }
        assert!(implementation.contains("Keep the owning turn active"));
        assert!(implementation.contains("wait for its terminal\nresult in this turn"));
        assert!(implementation.contains("any receiver still running"));
        let planning = skill::find("sb-plan")
            .expect("plan")
            .render(agent)
            .expect("render");
        for required in [
            "Keep the owning turn active until every started internal role has returned",
            "complete planning, a single phase, shared preparation, and Drive recovery",
            "its terminal result in this turn and resume the selected procedure",
            "receiver is still running\" is not a final",
            "required restart capsule",
            "Waiting for internal work grants no additional gate",
        ] {
            assert!(
                planning.contains(required),
                "missing Plan ownership: {required}"
            );
        }
    }
    let procedure = skill_resource_text("sb-implement", "references/spec-backed.md");
    assert!(procedure.contains("relative to the configured `specDir`"));
    assert!(procedure.contains("prefix the listed path with its exact `specDir`"));
    assert!(procedure.contains("Require the implementer to read the governing artifacts itself"));
}

#[test]
fn drive_preserves_nested_and_pre_owner_capacity_handoffs() {
    let body = skill_package_text("sb-drive");

    for required in [
        "Consume completed Drive-owned receiver results first",
        "exact `handler.target` owning Skill is the first and only receiver",
        "do not dispatch a capacity probe,\napproval or readiness reviewer, diagnosis helper, or any internal role",
        "owner-domain inspection\nbegins only inside the complete owner",
        "Never release an\n   unrelated receiver",
        "same owning Skill, action, item, and\n   handler mode",
        "If the owner never started and capacity remains unavailable",
        "do not retry that\n   unchanged dispatch in this run",
        "Do not execute owner work in the Drive\n   context",
        "`Handler target and mode:`",
        "`Project cwd, executable, and PATH facts:`",
        "Return every label with its exact value, using `none` rather than omission",
        "A\nnarrative equivalent is not a restart capsule",
        "Invent no finding ledger or\nretry budget for work that did not start",
        "return the labeled capsule immediately as the terminal\nDrive result",
        "preserve that\nhandoff **verbatim** as owner-owned continuation state",
        "exact paths, finding IDs and dispositions, used or remaining budgets",
        "shared worktree\nis unsafe: stop after the reread",
        "cannot infer non-durable finding, budget, or authority facts from\nstatus",
        "every owner-returned capacity restart handoff verbatim",
    ] {
        assert!(
            body.contains(required),
            "Drive capacity contract missing {required}"
        );
    }
}

#[test]
fn implementation_checks_cli_observations_without_granting_mutation_authority() {
    let procedure = skill_resource_text("sb-implement", "references/spec-backed.md");
    for required in [
        "specbind worktree capture",
        "specbind worktree check --snapshot <external-file|-> --changed <exact-path>",
        "`WORKTREE_CHECK_MISMATCH` (exit 1)",
        "`preexistingTouchedPaths`",
        "Never recapture just to clear a mismatch",
        "prior findings and partial paths remain",
        "The orchestrator never deletes them itself",
    ] {
        assert!(
            procedure.contains(required),
            "missing worktree boundary: {required}"
        );
    }
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
fn task_recovery_package_installs_one_progress_aware_budget_for_both_agents() {
    let recovery = skill_resource_text("sb-implement", "references/recovery.md");
    for required in [
        "At most three recovery cycles per Task",
        "any prior finding still `BLOCKING` stops the Task",
        "all prior findings `RESOLVED` plus materially new blocking IDs",
        "In bounded mode: after cycle three, any blocker stops the Task",
        "Missing, duplicate, or ambiguous prior-ID mappings stop recovery",
        "no free retry",
        "at most four\ndiagnostic assessments",
        "never infer zero from Git or Task status",
        "mapped descendants share the original recovery identity",
        "A receiver\nthat never starts consumes no cycle",
        "An owner that starts and fails during\npreparation consumes it",
        "A diagnosis is never Task approval",
        "even\nwith `--review inline` or `--review off`",
        "does not read project Drive defaults",
        "Require no special reset phrase or additional confirmation",
        "Apply each instruction once\nper recovery identity",
        "An unknown historical total remains\nunknown",
        "Carried unresolved or reopened findings form the baseline",
        "diagnostic assessments per allowance",
    ] {
        assert!(
            recovery.contains(required),
            "missing Task recovery boundary: {required}"
        );
    }
    let procedure = skill_resource_text("sb-implement", "references/spec-backed.md");
    assert!(procedure.contains("[Task recovery](recovery.md)"));
    assert!(!procedure.contains("at most two rounds"));
    assert!(!procedure.contains("at-most-two-round"));
    assert!(procedure.contains("reopening permits retry, not completion"));
    assert!(!recovery.contains("A generic request to continue cannot reset"));
    let entry = skill::find("sb-implement").expect("implementation skill");
    for agent in [Agent::Codex, Agent::ClaudeCode] {
        let rendered = entry.render(agent).expect("render implementation skill");
        assert!(rendered.contains("references/recovery.md"));
        assert!(rendered.contains("Direct-only bounds"));
    }
    for owner in ["sb-drive", "sb-plan"] {
        let replan = skill_resource_text(owner, "references/replan.md");
        assert!(replan.contains("out of three for bounded"));
        assert!(replan.contains("`uncapped` remaining for progress"));
        assert!(replan.contains("allowance identity and user authorization"));
        assert!(replan.contains("historical counts separately"));
        assert!(
            replan
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .contains("independent reassessment")
        );
    }
    let direct = skill_resource_text("sb-implement", "references/direct.md");
    assert!(direct.contains("at most twice"));
    let drive = skill_package_text("sb-drive");
    assert!(drive.contains("This routing applies without `--replan`"));
    assert!(drive.contains("blocked Task may have no ordinary\nactionable implementation entry"));
}

#[test]
fn progress_recovery_is_explicit_scoped_and_preserves_independent_stops() {
    for agent in [Agent::Codex, Agent::ClaudeCode] {
        for owner in ["sb-implement", "sb-drive"] {
            let rendered = skill::find(owner).unwrap().render(agent).unwrap();
            assert!(rendered.contains("--recovery bounded|progress"));
            assert!(
                rendered.contains("not `specbind` CLI flags")
                    || rendered.contains("not a `specbind` CLI flag")
            );
            assert!(rendered.contains("read-only"));
        }
    }
    let recovery = skill_resource_text("sb-implement", "references/recovery.md");
    for boundary in [
        "resumption defaults to `bounded`",
        "progress mode allows cycle four and later",
        "Partial improvement is not enough",
        "An ongoing receiver preserves the evidenced selection",
        "Switching to bounded with three or more used cycles",
        "Switching to progress cannot override",
        "one reassessment after each started cycle",
        "Do not rediagnose unchanged",
        "Progress mode grants no free preparation retry",
        "three\nused in progress mode plus one started corrected review is four used",
        "include the reserved cycle number in the owner brief",
        "Direct, Design and Contract Review bounds remain unchanged",
    ] {
        assert!(
            recovery.contains(boundary),
            "missing progress boundary: {boundary}"
        );
    }
    for owner in ["sb-drive", "sb-plan"] {
        let replan = skill_resource_text(owner, "references/replan.md");
        assert!(replan.contains("recovery mode and its"));
        assert!(replan.contains("restart"));
        assert!(replan.contains("phase budgets") || replan.contains("phase bounds"));
    }
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
    assert!(drive_entrypoint.contains("do not ask\nfor it again"));
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
fn replan_completion_withdrawal_keeps_causality_ownership_and_convergence() {
    let drive = skill_resource_text("sb-drive", "references/replan.md");
    let validation = skill_package_text("sb-validate-implementation");
    for text in [drive, validation.as_str()] {
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        for boundary in [
            "completion-withdrawal-only authority",
            "FRESHNESS_COMPLETION_PROJECT_CHANGED",
            "pre-existing staleness",
            "checkpoint",
            "WAITING",
        ] {
            assert!(text.contains(boundary), "missing {boundary}");
        }
    }
    assert!(drive.contains("Precheck the entire affected set"));
    assert!(drive.contains("specbind milestone replan capture"));
    assert!(drive.contains("specbind milestone replan check --baseline <baselineRevision> --recovery-revision <recoveryRevision>"));
    assert!(validation.contains("specbind milestone replan check --baseline <baselineRevision> --recovery-revision <recoveryRevision>"));
    assert!(validation.contains("successful mechanical check alone"));
    assert!(validation.contains("separately dispatched validation owner"));
    assert!(validation.contains("do not invalidate completion or create"));
    assert!(drive.contains("do not retry unchanged evidence"));
    assert!(drive.contains("do not add or reset the Task recovery budget"));
    assert!(validation.contains("does not require milestone Task convergence"));
    assert!(validation.contains("Do not run completion preflight, final"));
    assert!(validation.contains("Without this evidenced delegation"));
    for agent in [Agent::Codex, Agent::ClaudeCode] {
        let entry = skill::find("sb-drive")
            .expect("Drive")
            .render(agent)
            .expect("render");
        assert!(entry.contains("The flag alone is not evidence of its cause"));
    }
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
        assert_ne!(metadata.description.trim(), "");
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
        ("sb-configure", 8),
        ("sb-discovery", 3),
        ("sb-drive", 2),
        ("sb-implement", 3),
        ("sb-plan", 6),
        ("sb-release", 1),
        ("sb-validate-implementation", 1),
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
            assert_ne!(resource.content().trim(), "");
        }
        for agent in [Agent::ClaudeCode, Agent::Codex, Agent::Generic] {
            let files = entry.render_files(agent).expect("rendered package");
            let expected_files = expected_resources + 1 + usize::from(agent == Agent::Codex);
            assert_eq!(files.len(), expected_files, "{name}: {agent:?}");
            assert!(files.iter().any(|file| file.target.ends_with("/SKILL.md")));
        }
    }
}

#[test]
fn every_local_markdown_link_resolves_inside_its_rendered_package() {
    use std::path::{Component, Path, PathBuf};

    use pulldown_cmark::{Event, Parser, Tag};

    for entry in skill::all() {
        for agent in [Agent::ClaudeCode, Agent::Codex, Agent::Generic] {
            let files = entry.render_files(agent).expect("rendered package");
            for file in &files {
                if !Path::new(&file.target)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
                {
                    continue;
                }
                for event in Parser::new(&file.content) {
                    let Event::Start(Tag::Link { dest_url, .. }) = event else {
                        continue;
                    };
                    if dest_url.contains(':') || dest_url.starts_with('#') {
                        continue;
                    }
                    let relative = dest_url.split(['#', '?']).next().unwrap();
                    if relative.is_empty() {
                        continue;
                    }
                    let joined = Path::new(&file.target).parent().unwrap().join(relative);
                    let mut target = PathBuf::new();
                    for component in joined.components() {
                        match component {
                            Component::CurDir => {}
                            Component::ParentDir => {
                                assert!(target.pop(), "link escapes package: {dest_url}");
                            }
                            Component::Normal(part) => target.push(part),
                            _ => panic!("non-relative package link: {dest_url}"),
                        }
                    }
                    assert!(
                        files
                            .iter()
                            .any(|candidate| Path::new(&candidate.target) == target),
                        "{agent:?}: {} links to undistributed {dest_url} ({})",
                        file.target,
                        target.display()
                    );
                }
            }
        }
    }
}

#[test]
fn converged_validation_has_one_owner_and_common_revision_checkpoint() {
    let drive = skill::find("sb-drive").unwrap().body().unwrap();
    let validator = skill::find("sb-validate-implementation")
        .unwrap()
        .body()
        .unwrap();
    for required in [
        "once for that\nexplicit Spec set",
        "Mark each included action key attempted",
        "Never request a separate acceptance commit per Spec",
    ] {
        assert!(drive.contains(required), "missing coordination: {required}");
    }
    for required in [
        "every target's preflight before any acceptance",
        "If any target is `NO-GO` or `MANUAL_VERIFY_REQUIRED`, accept none",
        "same implementation revision",
        "Commit that metadata set together",
        "specbind spec completion batch preflight --spec <first> --spec <second>",
        "specbind schema read completion-batch/v1",
        "specbind spec completion batch accept --evidence <external-candidate-file>",
        "specbind spec completion batch verify --evidence <external-candidate-file>",
        "do not replace it with a per-Spec accept loop",
        "`pending` is not a retry instruction",
        "specbind steering list",
        "specbind steering read <selector> --for consume",
        "must retain that exact inner command",
        "Do not roll back valid evidence, retry at unchanged state",
    ] {
        assert!(validator.contains(required), "missing boundary: {required}");
    }
}

#[test]
fn drive_authority_and_revision_stop_survive_both_agent_renderings() {
    for agent in [Agent::Codex, Agent::ClaudeCode] {
        let drive = skill::find("sb-drive").unwrap().render(agent).unwrap();
        let validation = skill::find("sb-validate-implementation")
            .unwrap()
            .render(agent)
            .unwrap();
        let drive = drive.split_whitespace().collect::<Vec<_>>().join(" ");
        let validation = validation.split_whitespace().collect::<Vec<_>>().join(" ");
        for boundary in [
            "A request to drive the active milestone authorizes ordinary completion",
            "An explicit user limit",
            "never in a registered internal role",
            "Claude Code: `general-purpose`; Codex: the ordinary/default agent",
            "return `EXTERNAL_BLOCK`; do not substitute an internal role",
            "without inheriting Drive's conversation",
            "Codex `fork_turns: \"none\"` when exposed",
            "Never reuse a completed receiver",
            "isolation never discards or resets that evidence",
            "same Skill, action, item and handler mode",
            "terminal `WAITING` for milestone convergence is a completed handoff",
            "it does not make Drive the validation owner",
            "stop this Drive run after one Git/status reread",
            "Do not retry or switch owners",
            "A later user invocation of Drive",
            "prior candidates and `GO` verdicts cannot be reused",
        ] {
            assert!(drive.contains(boundary), "{agent:?}: {boundary}");
        }
        for boundary in [
            "An ordinary Drive request supplies that authority",
            "unless the user explicitly restricted completion recording or mutation",
            "accept none and return `BLOCKED`",
            "common preflight revision, observed HEAD, dirty paths, and the failed check",
            "all preflights, checks and per-Spec assessments",
            "This is not a `RETRYABLE` handoff",
            "Do not roll back valid evidence",
        ] {
            assert!(validation.contains(boundary), "{agent:?}: {boundary}");
        }
    }
}

use super::*;
