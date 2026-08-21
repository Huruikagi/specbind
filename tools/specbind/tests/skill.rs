use clap::CommandFactory as _;
use specbind::{agent_role, args::Cli, install::Agent, protocol, rule, skill};

const ACCEPTED_SKILLS: [&str; 17] = [
    "specbind-batch-plan",
    "specbind-contract-review",
    "specbind-debug",
    "specbind-design",
    "specbind-discovery",
    "specbind-gap-analysis",
    "specbind-implement",
    "specbind-quick-plan",
    "specbind-release",
    "specbind-requirements",
    "specbind-review-task",
    "specbind-status",
    "specbind-steering",
    "specbind-tasks",
    "specbind-validate-design",
    "specbind-validate-implementation",
    "specbind-verify-completion",
];

#[test]
fn embeds_the_accepted_skill_set_with_valid_metadata() {
    let names = skill::all()
        .iter()
        .map(|entry| entry.name)
        .collect::<Vec<_>>();
    assert_eq!(names, ACCEPTED_SKILLS);

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
fn renders_only_the_accepted_front_matter_per_agent() {
    for entry in skill::all() {
        let body = entry.body().expect("body");
        for agent in [Agent::ClaudeCode, Agent::Codex] {
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
            if agent == Agent::Codex {
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
    }
}

/// Every documented invocation must reference a real command route and only
/// options that route accepts. The command graph is walked, never executed.
#[test]
fn every_documented_invocation_resolves_against_the_command_graph() {
    let root = Cli::command();
    let mut checked = 0;
    for entry in skill::all() {
        for invocation in invocations(entry.body().expect("body")) {
            resolve(&root, entry.name, &invocation);
            checked += 1;
        }
    }
    assert!(
        checked >= 3,
        "the status skill documents several invocations; found {checked}"
    );
}

#[test]
fn every_live_markdown_read_names_its_instruction_projection() {
    for entry in skill::all() {
        for line in entry.body().expect("body").lines().map(str::trim) {
            if line.starts_with("specbind artifact read ")
                || line.starts_with("specbind steering read ")
            {
                assert!(
                    line.contains(" --for "),
                    "{} leaves a live Markdown read unprojected: {line}",
                    entry.name
                );
            }
        }
    }
}

#[test]
fn discovery_reads_the_scope_schema_before_first_creation() {
    let body = skill::all()
        .iter()
        .find(|entry| entry.name == "specbind-discovery")
        .expect("discovery skill")
        .body()
        .expect("discovery body");
    let schema = body
        .find("specbind schema read scope/v1")
        .expect("scope schema read");
    let create = body
        .find("specbind milestone create --scope -")
        .expect("milestone creation");
    assert!(
        schema < create,
        "discovery must learn the strict candidate shape before the mutating command"
    );
}

#[test]
fn discovery_rechecks_completion_immediately_before_brief_authoring() {
    let body = skill::find("specbind-discovery")
        .expect("discovery skill")
        .body()
        .expect("body");
    let protocol = body
        .find("specbind protocol read okf-authoring")
        .expect("authoring protocol read");
    let final_status = body
        .rfind("specbind milestone status")
        .expect("status read");
    let fill = body
        .find("Fill it from the request")
        .expect("brief authoring instruction");
    assert!(
        protocol < final_status && final_status < fill,
        "completion state must be checked after the protocol and before Brief authoring"
    );
}

#[test]
fn discovery_does_not_invalidate_a_gate_that_was_never_approved() {
    let body = skill::find("specbind-discovery")
        .expect("discovery skill")
        .body()
        .expect("discovery body");

    assert!(
        body.contains("`not_reached` is not approved")
            && body.contains("existing Requirements artifact")
            && body.contains("is approved."),
        "discovery must distinguish an existing artifact from approved gate evidence"
    );
}

#[test]
fn discovery_presents_an_approvable_scope_at_the_confirmation_boundary() {
    let body = skill::find("specbind-discovery")
        .expect("discovery skill")
        .body()
        .expect("discovery body");
    let invocation = body
        .find("The request to run this skill is **not** confirmation")
        .expect("invocation is not confirmation rule");
    let first_phase = body.find("## 1. Understand").expect("first phase");
    let payload = body
        .find("complete confirmation payload")
        .expect("complete confirmation payload instruction");
    let no_summary = body
        .find("or a no-change summary")
        .expect("no summary-only stop instruction");
    let work_items = body.find("Work items:").expect("work items field");
    let new_specs = body.find("New Specs:").expect("new Specs field");
    let invalidations = body
        .find("Gate invalidations:")
        .expect("gate invalidations field");
    let dependencies = body.find("Dependencies:").expect("dependencies field");
    let dependency_boundary = body
        .find("is not a work item")
        .expect("dependency endpoint boundary");
    let no_mutation = body
        .find("Do not run an invalidation")
        .expect("no pre-confirmation mutation instruction");
    let apply = body
        .find("## 6. Apply, rewinds first")
        .expect("apply phase");

    assert!(
        invocation < first_phase
            && payload < work_items
            && work_items < new_specs
            && new_specs < invalidations
            && invalidations < dependencies
            && dependencies < dependency_boundary
            && dependency_boundary < no_summary
            && no_summary < no_mutation
            && no_mutation < apply,
        "discovery must present the approvable payload before applying scope"
    );
}

#[test]
fn every_named_protocol_selector_and_rule_path_exists() {
    for entry in skill::all() {
        let body = entry.body().expect("body");
        for selector in tokens_after(body, "specbind protocol read ") {
            assert!(
                protocol::read(&selector).is_some(),
                "{}: unknown protocol selector {selector}",
                entry.name
            );
        }
        for path in rule_paths(body) {
            let file = path.rsplit('/').next().unwrap_or_default();
            assert!(
                rule::defaults()
                    .iter()
                    .any(|default| default.file_name == file),
                "{}: {path} is not an installed default rule",
                entry.name
            );
        }
    }
}

#[test]
fn every_registered_role_named_by_a_skill_exists_and_every_role_is_consumed() {
    let mut accepted = agent_role::all()
        .iter()
        .map(|role| role.name())
        .collect::<Vec<_>>();
    accepted.sort();
    let mut consumed = Vec::new();
    for entry in skill::all() {
        for role in tokens_after(entry.body().expect("body"), "registered `") {
            assert!(
                accepted.contains(&role),
                "{}: unknown registered role {role}",
                entry.name
            );
            consumed.push(role);
        }
    }
    consumed.sort();
    consumed.dedup();
    assert_eq!(
        consumed, accepted,
        "every installed role needs a skill consumer"
    );
}

#[test]
fn reviewing_skills_require_an_explicit_finding_disposition() {
    for name in [
        "specbind-requirements",
        "specbind-design",
        "specbind-validate-design",
        "specbind-review-task",
    ] {
        let body = skill::find(name)
            .expect("reviewing skill")
            .body()
            .expect("body");
        assert!(
            body.contains("[BLOCKING|DEFERRED|RESOLVED]"),
            "{name}: Decision 0122 requires the findings report shape"
        );
    }
}

#[test]
fn design_does_not_retire_an_export_only_to_silence_a_warning() {
    let body = skill::find("specbind-design")
        .expect("design skill")
        .body()
        .expect("body");

    assert!(body.contains("`CONTRACT_GRAPH_EXPORT_UNCONSUMED` is also a warning"));
    assert!(
        body.contains("An existing export that this change does not alter stays byte-identical")
    );
    assert!(body.contains("do not\n  retire an unrelated seam merely to silence the check"));
    assert!(
        body.contains("For an export this change adds or alters, name the managed or external")
    );
}

#[test]
fn planning_orchestrators_handoff_their_delegation_identity() {
    for name in ["specbind-quick-plan", "specbind-batch-plan"] {
        let body = skill::find(name)
            .expect("planning orchestrator")
            .body()
            .expect("body");
        assert!(body.contains("The request to run this skill is **not** that confirmation"));
        assert!(
            body.contains(&format!("workflow name\n`{name}`"))
                || body.contains(&format!("workflow name `{name}`"))
        );
        assert!(body.contains("authorized gate names"));
        assert!(body.contains("authorization omitted from the dispatch does not reach it"));
    }
}

#[test]
fn planning_orchestrator_metadata_routes_one_item_and_all_items_exclusively() {
    let quick = skill::find("specbind-quick-plan")
        .expect("quick-plan")
        .metadata()
        .expect("metadata");
    assert!(quick.description.contains("exactly one named or targeted"));
    assert!(quick.description.contains("approved plan in one go"));
    assert!(quick.description.contains("do not use for every Spec"));

    let batch = skill::find("specbind-batch-plan")
        .expect("batch-plan")
        .metadata()
        .expect("metadata");
    assert!(batch.description.contains("every Spec-backed item"));
    assert!(batch.description.contains("approved plans in one run"));
    assert!(
        batch
            .description
            .contains("do not use for one named or targeted item")
    );
}

#[test]
fn design_validation_puts_its_read_only_stop_rule_before_commands() {
    let body = skill::find("specbind-validate-design")
        .expect("design validation skill")
        .body()
        .expect("body");
    let first_command = body.find("```sh").expect("documented command");
    let preamble = &body[..first_command];

    assert!(preamble.contains("Read-only stop rule — before any command"));
    assert!(preamble.contains("do not run a\ngate invalidation command"));
    assert!(preamble.contains("every gate and review record exactly as you found them"));

    assert!(body.contains("Existing code is architectural context, not implementation evidence"));
    assert!(body.contains("do not\njudge whether the code already does"));
    assert!(body.contains("Fix the review scope from the status output before reading prose"));
    assert!(body.contains("do not report the Design incomplete"));
    assert!(body.contains("inactive ID"));
}

#[test]
fn task_review_puts_its_no_write_rule_before_commands() {
    let body = skill::find("specbind-review-task")
        .expect("task review skill")
        .body()
        .expect("body");
    let first_command = body.find("```sh").expect("documented command");
    let preamble = &body[..first_command];

    assert!(preamble.contains("Read-only stop rule — before any probe"));
    assert!(preamble.contains("cannot create\ncaches, coverage data, reports, lockfiles"));
    assert!(preamble.contains("git status --short` before and after"));
}

#[test]
fn contract_review_uses_scope_and_type_based_historical_discovery() {
    let body = skill::find("specbind-contract-review")
        .expect("contract review skill")
        .body()
        .expect("body");

    assert!(body.contains("specbind milestone scope"));
    assert!(body.contains("`Status: not_applicable`"));
    assert!(body.contains("git ls-tree -r --name-only <baseline>"));
    assert!(body.contains("`type` is `SpecBind Contract`"));
    assert!(!body.contains("git show <baseline>:.specbind/specs/<spec>/contract.md"));
}

#[test]
fn implementation_workflow_carries_notes_and_all_failure_routes() {
    let body = skill::find("specbind-implement")
        .expect("implementation skill")
        .body()
        .expect("body");

    for required in [
        "specbind artifact list <spec>",
        "specbind artifact read <spec> implementation-notes/<artifact-id>",
        "specbind protocol read okf-authoring",
        "specbind template read spec implementation-notes/main",
        "`CANNOT_REVIEW`",
        "Do not interrupt it, ask for an immediate\nreturn",
        "Do not skip ahead and return here afterwards.",
        "Do not stop merely because the implementation commit succeeded.",
    ] {
        assert!(
            body.contains(required),
            "implementation skill must contain {required}"
        );
    }

    let checkpoint = body
        .find("specbind adapter read git")
        .expect("checkpoint command");
    let handshake = body
        .find("specbind milestone direct preflight <direct>")
        .expect("Direct preflight command");
    assert!(
        checkpoint < handshake,
        "the Direct workflow must present checkpoint before handshake"
    );
}

#[test]
fn adapter_consumers_use_the_dedicated_scaffold_marker() {
    for name in [
        "specbind-discovery",
        "specbind-requirements",
        "specbind-design",
        "specbind-tasks",
        "specbind-contract-review",
        "specbind-implement",
        "specbind-release",
    ] {
        let body = skill::find(name)
            .expect("adapter-consuming skill")
            .body()
            .expect("body");
        assert!(
            body.contains("<!-- specbind:adapter-scaffold -->"),
            "{name} must recognize the dedicated adapter scaffold marker"
        );
        assert!(
            body.contains("marker classifies the whole\ndocument")
                || body.contains("marker classifies the whole document")
                || (name == "specbind-release"
                    && body.contains("Do not interpret\nits remaining body")),
            "{name} must make the marker override the entire adapter body"
        );
        assert!(
            !body.contains("A legacy adapter may still carry"),
            "{name} must not preserve legacy adapter compatibility"
        );
    }
}

#[test]
fn release_bootstraps_policy_and_checkpoints_only_after_finalization() {
    let body = skill::find("specbind-release")
        .expect("release skill")
        .body()
        .expect("body");

    for required in [
        "Stop after bootstrap",
        "must run its completion handshake\n   again",
        "approval authorizes only replacing the adapter",
        "Never infer that an adapter is unconfigured",
        "Do not rely on `README.md` being the only entry point",
        "Skip sections 3 through 6",
        "A local tag has not left the repository",
        "Immediately before finalization, record `git status --short`",
        "Checkpoint only the finalized lifecycle metadata",
        "Publication approval does not authorize pushing this commit",
        "move the published tag to include this later metadata commit",
    ] {
        assert!(
            body.contains(required),
            "release skill must contain {required}"
        );
    }

    let finalize = body
        .find("specbind release finalize --log-entries -")
        .expect("finalization command");
    let git = body[finalize..]
        .find("specbind adapter read git")
        .map(|offset| finalize + offset)
        .expect("post-finalization Git adapter read");
    let after_finalize = body
        .find("## 9. After finalize")
        .expect("After-finalize section");
    assert!(
        finalize < git && git < after_finalize,
        "core metadata must checkpoint before project After-finalize work"
    );
}

#[test]
fn direct_debug_surface_can_report_an_undetermined_owner() {
    let body = skill::find("specbind-debug")
        .expect("debug skill")
        .body()
        .expect("body");

    assert!(
        body.contains("- CATEGORY: IMPLEMENTATION | PLAN | ARTIFACT | ENVIRONMENT | UNDETERMINED")
    );
}

#[test]
fn implementation_completion_questions_route_to_validation_not_status() {
    let validation = skill::find("specbind-validate-implementation")
        .expect("implementation validation skill")
        .metadata()
        .expect("validation metadata");
    assert!(validation.description.contains("active Requirement IDs"));

    let status = skill::find("specbind-status")
        .expect("status skill")
        .metadata()
        .expect("status metadata");
    assert!(
        status
            .description
            .contains("do not use to judge whether completed implementation is actually done")
    );
    assert!(
        skill::find("specbind-status")
            .expect("status skill")
            .body()
            .expect("status body")
            .contains("use\n`specbind-validate-implementation`")
    );

    let claim_verification =
        skill::find("specbind-verify-completion").expect("claim verification skill");
    assert!(
        claim_verification
            .metadata()
            .expect("claim verification metadata")
            .description
            .contains("do not use to advance a named Spec")
    );
    assert!(
        claim_verification
            .body()
            .expect("claim verification body")
            .contains("use `specbind-validate-implementation` instead")
    );

    let validation_body = skill::find("specbind-validate-implementation")
        .expect("implementation validation skill")
        .body()
        .expect("implementation validation body");
    assert!(validation_body.contains("specbind check traceability <spec>"));
    assert!(validation_body.contains("Validate the **active Requirement IDs**"));
    assert!(validation_body.contains("do not report the Spec incomplete"));
    assert!(validation_body.contains("Fix that required\nset **before** running anything"));
    assert!(validation_body.contains("do not invoke its underlying test runner"));
}

#[test]
fn task_review_and_debug_discover_split_designs_before_reading_them() {
    for name in ["specbind-review-task", "specbind-debug"] {
        let body = skill::find(name)
            .expect("inspection skill")
            .body()
            .expect("body");
        assert!(body.contains("specbind artifact list <spec>"));
        assert!(body.contains("specbind artifact read <spec> design/<artifact-id>"));
        assert!(
            !body.contains("specbind artifact read <spec> design/main"),
            "{name}: a fixed selector must not precede type-based discovery"
        );
    }
}

/// Extracts each literal invocation: a standalone inline code span or one line
/// of a shell fence, beginning with the exact token `specbind `.
fn invocations(body: &str) -> Vec<Vec<String>> {
    let mut found = Vec::new();
    let mut in_fence = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            if let Some(rest) = trimmed.strip_prefix("specbind ") {
                found.push(words(rest));
            }
            continue;
        }
        let mut rest = trimmed;
        while let Some(open) = rest.find('`') {
            let after = &rest[open + 1..];
            let Some(close) = after.find('`') else { break };
            let span = &after[..close];
            if let Some(arguments) = span.strip_prefix("specbind ") {
                found.push(words(arguments));
            }
            rest = &after[close + 1..];
        }
    }
    found
}

fn words(value: &str) -> Vec<String> {
    value
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>()
}

/// Walks the subcommand path, then checks every long option against that route.
fn resolve(root: &clap::Command, skill: &str, invocation: &[String]) {
    let mut command = root;
    let mut index = 0;
    while index < invocation.len() {
        let token = invocation[index].as_str();
        if token.starts_with('-') || is_metavariable(token) {
            break;
        }
        // A leaf route takes positional values, and a literal one such as the
        // `spec` scope of `template read` is a value, not a missing command.
        if command.get_subcommands().next().is_none() {
            break;
        }
        let Some(next) = command
            .get_subcommands()
            .find(|candidate| candidate.get_name() == token)
        else {
            panic!(
                "{skill}: `specbind {}` has no command {token}",
                invocation.join(" ")
            );
        };
        command = next;
        index += 1;
    }
    for token in &invocation[index..] {
        let Some(long) = token.strip_prefix("--") else {
            continue;
        };
        let name = long.split('=').next().unwrap_or(long);
        assert!(
            command
                .get_arguments()
                .any(|argument| argument.get_long() == Some(name)),
            "{skill}: `specbind {}` uses unknown option --{name}",
            invocation.join(" ")
        );
    }
}

/// Presentation metavariables are not runtime values.
fn is_metavariable(token: &str) -> bool {
    (token.starts_with('<') && token.ends_with('>'))
        || (token.starts_with('[') && token.ends_with(']'))
}

fn tokens_after(body: &str, prefix: &str) -> Vec<String> {
    body.lines()
        .filter_map(|line| line.trim().split_once(prefix))
        .filter_map(|(_, rest)| rest.split_whitespace().next())
        .map(|token| token.trim_matches('`').to_owned())
        .filter(|token| !is_metavariable(token))
        .collect()
}

fn rule_paths(body: &str) -> Vec<String> {
    body.split_whitespace()
        .map(|token| token.trim_matches(['`', '.', ',', ')', '(']))
        .filter(|token| token.contains("settings/rules/"))
        .map(ToOwned::to_owned)
        .collect()
}
