use clap::CommandFactory as _;
use specbind::{agent_role, args::Cli, install::Agent, protocol, rule, skill};

const ACCEPTED_SKILLS: [&str; 15] = [
    "sb-contract-review",
    "sb-configure",
    "sb-debug",
    "sb-discovery",
    "sb-drive",
    "sb-gap-analysis",
    "sb-implement",
    "sb-plan",
    "sb-release",
    "sb-review-task",
    "sb-status",
    "sb-steering",
    "sb-validate-design",
    "sb-validate-implementation",
    "sb-verify-completion",
];

#[test]
fn drive_uses_authoritative_actions_and_parks_local_attention() {
    let drive = skill::find("sb-drive").expect("drive skill");
    let metadata = drive.metadata().expect("drive metadata");
    assert!(metadata.description.contains("safe reachable"));
    let body = drive.body().expect("drive body");
    for required in [
        "specbind milestone status --json",
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
}

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
        ("sb-discovery", 5),
        ("sb-implement", 2),
        ("sb-plan", 3),
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

#[test]
fn configure_aftercare_names_the_exact_git_adapter_read_command() {
    let configure = skill::find("sb-configure").expect("configure skill");
    let aftercare = configure
        .resources()
        .iter()
        .find(|resource| resource.relative_path == "references/aftercare.md")
        .expect("configure aftercare reference")
        .content();

    assert!(
        aftercare.contains("specbind adapter read git"),
        "configure aftercare must name the exact Git adapter read command"
    );
}

#[test]
fn configure_update_routes_and_preserves_the_two_phase_refresh_contract() {
    let configure = skill::find("sb-configure").expect("configure skill");
    let metadata = configure.metadata().expect("configure metadata");
    assert!(metadata.description.contains("explicitly update SpecBind"));

    let update = configure
        .resources()
        .iter()
        .find(|resource| resource.relative_path == "references/update.md")
        .expect("configure update reference")
        .content();
    for required in [
        "mise tool github:Huruikagi/specbind --json",
        "requested_versions",
        "config_source",
        "An exact pin does not advance without an explicit target",
        "mise upgrade github:Huruikagi/specbind",
        "mise use github:Huruikagi/specbind@<version>",
        "specbind install --dry-run",
        "Present every reported `create`, `replace`, `keep`, and `remove` action.",
        "Mandatory post-replacement reload",
        ".agents/skills/sb-configure/references/update.md",
        ".claude/skills/sb-configure/references/aftercare.md",
        "Resume at **Post-refresh continuation**",
        "binary-selection checkpoint",
        "asset-refresh checkpoint",
        "Updating never authorizes a push",
    ] {
        assert!(
            update.contains(required),
            "configure update procedure must preserve: {required}"
        );
    }
    for forbidden in ["self-update command", "background network operation"] {
        assert!(
            update.contains(forbidden),
            "configure update procedure must explicitly reject: {forbidden}"
        );
    }
}

#[test]
fn configuration_and_final_validation_share_the_validation_adapter_contract() {
    let configure = skill::find("sb-configure").expect("configure skill");
    let adapters = configure
        .resources()
        .iter()
        .find(|resource| resource.relative_path == "references/adapters.md")
        .expect("configure adapters reference")
        .content();
    for required in [
        "The `validation` adapter adds project-specific work",
        "Present a complete replacement for a scaffold\nor an exact diff for active guidance.",
        "cannot replace, waive, narrow, or declare them passed",
        "The adapter grants no credential, external mutation, source edit, or finding\n  repair authority.",
    ] {
        assert!(
            adapters.contains(required),
            "configure must preserve the Validation adapter boundary: {required}"
        );
    }

    let validation = skill::find("sb-validate-implementation")
        .expect("implementation validation skill")
        .body()
        .expect("implementation validation body");
    for required in [
        "specbind adapter read validation",
        "Fix the complete required set before running anything.",
        "is `MANUAL_VERIFY_REQUIRED`",
        "it must not\nedit source or repair a finding it will judge",
        "as `custom` only when an exact command actually\nran and returned zero",
        "remain run-scoped semantic evidence",
    ] {
        assert!(
            validation.contains(required),
            "final validation must preserve the adapter contract: {required}"
        );
    }
}

#[test]
fn configure_recommends_steering_before_project_shaping_but_preserves_narrow_edits() {
    let configure = skill::find("sb-configure").expect("configure skill");
    let body = configure.body().expect("configure body");
    let templates = configure
        .resources()
        .iter()
        .find(|resource| resource.relative_path == "references/templates-and-reconciliation.md")
        .expect("configure templates reference")
        .content();

    for required in [
        "Establish or synchronize durable Steering through `sb-steering`.",
        "Technology labels alone are not a template boundary.",
        "does not replace\nthe per-Spec applicability decision in `design-template-selection`.",
        "Do not bootstrap it merely because it is\nempty",
        "do not infer\none.",
    ] {
        assert!(
            body.contains(required),
            "configure skill must preserve project-shaping guidance: {required}"
        );
    }
    for required in [
        "distinct recurring design decisions and traceability",
        "framework presence alone is insufficient",
        "conditions must describe the responsibility",
        "If a request names a future technology but the current Steering and repository",
    ] {
        assert!(
            templates.contains(required),
            "configure template procedure must preserve candidate criteria: {required}"
        );
    }
}

#[test]
fn design_materializes_spec_local_supplements_without_polluting_project_policy() {
    let body = skill_package_text("sb-plan");
    for required in [
        "Spec-local one-off Design\nsupplement",
        "target `specs/<spec>/design/<artifact_id>.md`",
        "Materialize one as part of the current Design\ndraft",
        "candidate other than `design/main` communicates that\n  responsibility clearly",
        "`design/main` is the alternative to\nassess",
        "array of quoted YAML strings such as `\"1.1\"`",
        "Before authoring or revising any Design, assess whether the current change needs",
        "Runtime deployment, configuration or Secret ownership, health signals,",
        "Do not add a project template or change\n`design-template-selection`",
        "recommend\nthat `sb-configure` evaluate promotion",
    ] {
        assert!(
            body.contains(required),
            "Design procedure must preserve one-off supplement contract: {required}"
        );
    }
}

#[test]
fn discovery_reverse_route_keeps_evidence_separate_and_owns_non_release_finalization() {
    let body = skill_package_text("sb-discovery");
    for required in [
        "references/adopt-start.md",
        "references/adopt-resume.md",
        "references/reverse.md",
        "specbind adoption preflight",
        "one complete proposal",
        "temporary adoption record",
        "baseline_version",
        "reverseSpecs",
        "source_revision",
        "Existing code and tests are **evidence**",
        "blocking semantic unknown",
        "suspected defect",
        "`adoption_ready`",
        "No Tasks, implementation change, or product release",
        "specbind milestone reverse finalize --log-entries",
    ] {
        assert!(
            body.contains(required),
            "Discovery adoption package must contain {required}"
        );
    }
    assert!(
        !body.to_ascii_lowercase().contains("dossier"),
        "Discovery adoption package must describe the temporary record without treating dossier as a product term"
    );
}

#[test]
fn reverse_discovery_resolves_and_checkpoints_deferred_findings_after_creation() {
    let body = skill_resource_text("sb-discovery", "references/reverse.md");
    let list = body
        .find("specbind adapter list")
        .expect("adapter discovery");
    let read = body
        .find("specbind adapter read deferred")
        .expect("exact deferred selector");
    let create = body
        .find("specbind milestone create --scope")
        .expect("milestone creation");
    let write = body
        .find("Only after milestone creation and provenance verification")
        .expect("post-creation finding write");
    assert!(list < read && read < create && create < write);
    assert!(body.contains("do not guess `deferred-findings`"));
    assert!(body.contains("pending adapter records"));
    assert!(body.contains("verified deferred destination when written as one\nDiscovery unit"));
    assert!(body.contains("one closed contradiction ledger"));
    assert!(body.contains("Give every difference exactly one visible disposition"));
    assert!(body.contains("Do not silently drop a naming, behavior, or\nboundary mismatch"));
}

/// Every documented invocation must reference a real command route and only
/// options that route accepts. The command graph is walked, never executed.
#[test]
fn every_documented_invocation_resolves_against_the_command_graph() {
    let root = Cli::command();
    let mut checked = 0;
    for entry in skill::all() {
        for document in skill_documents(*entry) {
            for invocation in invocations(document) {
                resolve(&root, entry.name, &invocation);
                checked += 1;
            }
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
        for document in skill_documents(*entry) {
            for line in document.lines().map(str::trim) {
                if line.starts_with("specbind artifact read ")
                    || line.starts_with("specbind steering read ")
                    || line.starts_with("specbind rule read ")
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
}

#[test]
fn discovery_reads_the_scope_schema_before_first_creation() {
    let body = skill::all()
        .iter()
        .find(|entry| entry.name == "sb-discovery")
        .expect("discovery skill")
        .body()
        .expect("discovery body");
    let schema = body
        .find("specbind schema read scope/v1")
        .expect("scope schema read");
    let roadmap_template = body
        .find("specbind template read milestone roadmap")
        .expect("Roadmap template read");
    let create = body
        .find("specbind milestone create --scope -")
        .expect("milestone creation");
    assert!(
        roadmap_template < schema && schema < create,
        "discovery must read the Roadmap scaffold and strict candidate shape before mutation"
    );
}

#[test]
fn discovery_rechecks_completion_immediately_before_brief_authoring() {
    let body = skill::find("sb-discovery")
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
    let body = skill::find("sb-discovery")
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
    let body = skill::find("sb-discovery")
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
fn local_source_collection_is_complete_confirmed_and_preserved() {
    let discovery = skill_package_text("sb-discovery");
    for required in [
        "references/local-files.md",
        "specbind protocol read source-material",
        "every Source Item",
        "Source coverage:",
        "complete\nprovenance and coverage mapping",
        "exact project-relative Source Items",
        "Direct items still have no Brief",
    ] {
        assert!(
            discovery.contains(required),
            "Discovery package must contain {required}"
        );
    }
}

#[test]
fn github_milestone_source_collection_is_complete_read_only_and_preserved() {
    let discovery = skill_package_text("sb-discovery");
    for required in [
        "references/github-milestone.md",
        "or the exact canonical URL",
        "Parse only that exact URL shape",
    ] {
        assert!(
            discovery.contains(required),
            "Discovery package must contain {required}"
        );
    }

    let provider = skill_resource_text("sb-discovery", "references/github-milestone.md");
    for required in [
        "gh auth status",
        "https://github.com/OWNER/REPO/milestone/NUMBER",
        "Accept no other URL shape",
        "query, fragment, percent-encoded path component",
        "--paginate --slurp",
        "state=all",
        "pull_request",
        "Do not read comments or timeline events",
        "partial acquisition stops before classification",
        "they do not\nsilently re-query GitHub",
    ] {
        assert!(
            provider.contains(required),
            "GitHub provider missing {required}"
        );
    }
}

#[test]
fn planning_promotes_declared_source_items_into_canonical_artifacts() {
    let requirements = skill_resource_text("sb-plan", "references/requirements.md");
    for required in [
        "If the Brief declares Source Items",
        "specbind protocol read source-material",
        "every exact project-relative item",
        "Restate every accepted behavioral",
        "do not make an acceptance criterion depend on following a source link",
    ] {
        assert!(
            requirements.contains(required),
            "Requirements missing {required}"
        );
    }

    let design = skill_resource_text("sb-plan", "references/design.md");
    for required in [
        "If the Brief declares Source Items",
        "specbind protocol read source-material",
        "every exact project-relative item",
        "Requirements remains authoritative for\nbehavior",
        "Restate every technical\nconclusion",
    ] {
        assert!(design.contains(required), "Design missing {required}");
    }
}

#[test]
fn every_named_protocol_and_rule_selector_exists() {
    for entry in skill::all() {
        let body = entry.body().expect("body");
        for document in skill_documents(*entry) {
            for selector in tokens_after(document, "specbind protocol read ") {
                assert!(
                    protocol::read(&selector).is_some(),
                    "{}: unknown protocol selector {selector}",
                    entry.name
                );
            }
            for selector in tokens_after(document, "specbind rule read ") {
                assert!(
                    rule::find(&selector).is_some(),
                    "{}: unknown rule selector {selector}",
                    entry.name
                );
            }
        }
        assert!(
            !body.contains("settings/rules/"),
            "{} reads project rules by path instead of through the CLI",
            entry.name
        );
    }
}

fn skill_documents(entry: skill::Skill) -> Vec<&'static str> {
    std::iter::once(entry.body().expect("body"))
        .chain(entry.resources().iter().map(|resource| resource.content()))
        .collect()
}

fn skill_package_text(name: &str) -> String {
    let entry = skill::find(name).unwrap_or_else(|| panic!("missing skill {name}"));
    skill_documents(entry).join("\n")
}

fn skill_resource_text(name: &str, relative_path: &str) -> &'static str {
    skill::find(name)
        .unwrap_or_else(|| panic!("missing skill {name}"))
        .resources()
        .iter()
        .find(|resource| resource.relative_path == relative_path)
        .unwrap_or_else(|| panic!("missing resource {name}/{relative_path}"))
        .content()
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
        for document in skill_documents(*entry) {
            for role in tokens_after(document, "registered `") {
                assert!(
                    accepted.contains(&role),
                    "{}: unknown registered role {role}",
                    entry.name
                );
                consumed.push(role);
            }
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
    for (name, body) in [
        (
            "sb-plan requirements",
            skill_resource_text("sb-plan", "references/requirements.md"),
        ),
        (
            "sb-plan design",
            skill_resource_text("sb-plan", "references/design.md"),
        ),
        (
            "sb-validate-design",
            skill::find("sb-validate-design")
                .expect("reviewing skill")
                .body()
                .expect("body"),
        ),
        (
            "sb-review-task",
            skill::find("sb-review-task")
                .expect("reviewing skill")
                .body()
                .expect("body"),
        ),
    ] {
        assert!(
            body.contains("[BLOCKING|DEFERRED|RESOLVED]"),
            "{name}: Decision 0122 requires the findings report shape"
        );
    }
}

#[test]
fn design_does_not_retire_an_export_only_to_silence_a_warning() {
    let body = skill_resource_text("sb-plan", "references/design.md");

    assert!(body.contains("`CONTRACT_GRAPH_EXPORT_UNCONSUMED` is also a warning"));
    assert!(body.contains(
        "An existing export that this change does not alter stays semantically unchanged"
    ));
    assert!(body.contains("do not\n  retire an unrelated seam merely to silence the check"));
    assert!(
        body.contains("For an export this change adds or alters, name the managed or external")
    );
}

#[test]
fn reverse_design_checks_tolerate_only_waiting_participant_contracts() {
    for (name, body) in [
        (
            "sb-plan design",
            skill_resource_text("sb-plan", "references/design.md"),
        ),
        (
            "sb-validate-design",
            skill::find("sb-validate-design")
                .expect("validation skill")
                .body()
                .expect("validation body"),
        ),
    ] {
        let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");
        for required in [
            "all errors are `CONTRACT_GRAPH_CONTRACT_UNAVAILABLE`",
            "another participant in this same reverse milestone",
            "waiting for an earlier Design dependency",
            "The current Spec's Contract must be readable",
            "complete graph remains mandatory",
        ] {
            assert!(
                normalized.contains(required),
                "{name} must contain {required}"
            );
        }
    }

    let orchestrator = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("orchestrator body");
    assert!(orchestrator.contains("Contract Review accepts no provisional graph"));
}

#[test]
fn design_queries_direct_contract_neighbors_before_reading_them() {
    let body = skill_resource_text("sb-plan", "references/design.md");
    let dependencies = body
        .find("specbind contract dependencies <spec>")
        .expect("direct dependency query");
    let consumers = body
        .find("specbind contract consumers <spec>")
        .expect("reverse consumer query");
    let neighbor = body
        .find("specbind artifact read <other-spec> contract --for consume")
        .expect("neighbor Contract read");
    assert!(
        dependencies < consumers && consumers < neighbor,
        "design must resolve both directions before reading neighboring Contracts"
    );
    assert!(
        body.contains("not a semantic impact verdict")
            && body.contains("current graph cannot name the other side yet")
            && body.contains("external consumer"),
        "design must retain the topology, new-seam, and unmanaged-consumer boundaries"
    );
}

#[test]
fn planning_orchestrator_handoffs_its_delegation_identity() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("body");
    assert!(body.contains("request to run this skill is **not**"));
    assert!(body.contains("workflow name `sb-plan`"));
    assert!(body.contains("authorized gate names"));
    assert!(body.contains("authorization omitted"));
    assert!(body.contains("from the dispatch does not reach it"));
}

#[test]
fn planning_orchestrator_requires_clean_checkpointed_phase_handoffs() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("orchestrator body");
    assert!(body.contains("adapter-directed checkpoint"));
    assert!(body.contains("git status --short"));
    assert!(body.contains("clean handoff"));
    assert!(body.contains("must not\ncreate a checkpoint owned by the dispatched phase"));
}

#[test]
fn planning_dispatch_carries_project_local_execution_environment() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("orchestrator body");

    for required in [
        "exact project working directory",
        "project-local instruction files",
        "confirmed `specbind`\nexecutable, version, and required environment facts",
        "must not fall back to another `specbind`",
        "grant no additional scope",
    ] {
        assert!(body.contains(required), "plan must contain {required}");
    }
}

#[test]
fn planning_orchestrator_bounds_the_unapproved_design_handoff() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("orchestrator body");
    assert!(body.contains("one deliberate exception"));
    assert!(body.contains("Design artifact paths"));
    assert!(body.contains("Contract path"));
    assert!(
        body.contains("exact project-relative destination named by the active deferred adapter")
    );
    assert!(body.contains("not infer it from a conventional filename"));
    assert!(body.contains("verified deferred destination when\npresent"));
    assert!(body.contains("`spec.yaml`"));
    assert!(body.contains("After `READY`"));
    assert!(body.contains("normal clean handoff remains mandatory"));
}

#[test]
fn design_phase_checkpoints_its_verified_deferred_destination_after_validation() {
    let body = skill_resource_text("sb-plan", "references/design.md");
    assert!(body.contains("approval re-dispatch after independent validation"));
    assert!(body.contains("exact active deferred-adapter destination"));
    assert!(body.contains("gate-updated `spec.yaml`"));

    let validation = skill::find("sb-validate-design").expect("validation skill");
    let validation_body = validation.body().expect("validation body");
    assert!(validation_body.contains("adapter write happens only after the verdict"));
    assert!(validation_body.contains("Report the exact\nproject-relative destination"));
}

#[test]
fn planning_orchestrator_validates_design_before_delegated_approval() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("orchestrator body");
    assert!(body.contains("without Design-gate authority"));
    assert!(body.contains("Only"));
    assert!(body.contains("retroactively"));
}

#[test]
fn planning_orchestrator_routes_design_no_go_through_one_owned_revision() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("orchestrator body");
    assert!(body.contains("validator\nverdict, not a phase status"));
    assert!(body.contains("one revision"));
    assert!(body.contains("fresh validation"));
    assert!(body.contains("requirements rewind"));
    assert!(body.contains("Never approve"));
}

#[test]
fn requirements_audits_existing_obligations_before_approval() {
    let body = skill_resource_text("sb-plan", "references/requirements.md");

    assert!(body.contains("mandatory preservation audit before\napproval"));
    assert!(body.contains("git diff -- <requirements-path>"));
    assert!(body.contains("every pre-existing requirement group and acceptance criterion"));
    assert!(body.contains("Context, Scope, and Objective"));
    assert!(body.contains("stop before approval"));
    assert!(body.contains("Never use the approve command"));
    assert!(body.contains("including its opening marker, body, and closing marker"));
    assert!(body.contains("Never invent aliases such as `R2.AC1`"));
    assert!(body.contains("specbind check traceability <spec>"));
    assert!(body.contains("private preservation\n  ledger in working memory"));
    assert!(body.contains("zero obligations were lost"));
    assert!(body.contains("repeat the preservation-ledger reconciliation"));
    assert!(body.contains("discovering a loss after approval is a failed workflow"));
    assert!(body.contains("Do not create a Requirement solely to require tests"));
    assert!(body.contains("`SPEC_REQUIREMENTS_RETIREMENT_UNSUPPORTED`"));
    assert!(body.contains("not a reason to invalidate a\ngate"));
    assert!(body.contains("retry approval once"));
    assert!(body.contains("`SPEC_REQUIREMENTS_BASELINE_READ_FAILED`"));
}

#[test]
fn requirements_preserves_abstract_boundaries_and_avoids_new_spec_contract_probe() {
    let body = skill_resource_text("sb-plan", "references/requirements.md");

    assert!(body.contains("Preserve an intentionally abstract but observable boundary"));
    assert!(body.contains("without inventing a duration"));
    assert!(body.contains("cannot determine an observable\naccepted or rejected outcome"));
    assert!(body.contains("Do not run a Contract read in this branch"));
    assert!(body.contains("artifact inventory as the non-error existence check"));
    assert!(body.contains("specbind artifact list <spec>"));

    let new_spec = body.find("- **New Spec**").expect("new Spec branch");
    let existing_spec = body
        .find("- **Existing Spec**")
        .expect("existing Spec branch");
    let contract_read = body
        .find("specbind artifact read <spec> contract --for consume")
        .expect("conditional Contract read");
    assert!(new_spec < existing_spec && existing_spec < contract_read);
}

#[test]
fn requirements_treats_the_listed_steering_inventory_as_closed() {
    let body = skill_resource_text("sb-plan", "references/requirements.md");
    for required in [
        "Treat that listing as the complete, closed set for this read.",
        "the active Roadmap is milestone state stored beside Steering",
        "must not be passed to `steering read`",
    ] {
        assert!(
            body.contains(required),
            "Requirements must prevent unlisted Roadmap-as-Steering reads: {required}"
        );
    }
}

#[test]
fn requirements_resolves_the_new_artifact_project_path_before_writing() {
    let body = skill_resource_text("sb-plan", "references/requirements.md");
    let new_spec = body.find("- **New Spec**").expect("new Spec branch");
    let resolve = body[new_spec..]
        .find("specbind template resolve spec <spec> requirements")
        .expect("Requirements target resolution");
    let read = body[new_spec..]
        .find("specbind template read spec requirements")
        .expect("Requirements template read");
    assert!(resolve < read);
    assert!(body.contains("Write the authored document only to the resolved `Project path`."));
    assert!(body.contains("do not reconstruct it from an artifact"));
    assert!(body.contains("inventory `path`, the template-relative `Output path`"));
}

#[test]
fn implementation_validation_preserves_exact_executed_command_text() {
    let validation =
        skill::find("sb-validate-implementation").expect("implementation validation skill");
    let body = validation.body().expect("validation body");

    assert!(body.contains("Preserve the executed command verbatim"));
    assert!(body.contains("shortened form, placeholder"));
    assert!(body.contains("Compare the JSON candidate"));
    assert!(body.contains("Around each canonical project command"));
    assert!(body.contains("Do not clean between the command"));
    assert!(body.contains("command itself becomes repeatably clean"));
}

#[test]
fn planning_metadata_exposes_complete_and_single_phase_modes() {
    let plan = skill::find("sb-plan")
        .expect("plan")
        .metadata()
        .expect("metadata");
    assert!(plan.description.contains("one named Spec"));
    assert!(
        plan.description
            .contains("explicitly requested planning phase")
    );
    assert_eq!(
        plan.argument_hint.as_deref(),
        Some("[<spec> | --all] [requirements|design|tasks]")
    );
    for removed in [
        "specbind-quick-plan",
        "specbind-requirements",
        "specbind-design",
        "specbind-tasks",
        "specbind-batch-plan",
        "specbind-plan-requirements",
        "specbind-plan-design",
        "specbind-plan-tasks",
    ] {
        assert!(skill::find(removed).is_none(), "removed alias {removed}");
    }
}

#[test]
fn planning_phase_procedures_are_directly_routed_references() {
    let plan = skill::find("sb-plan").expect("plan");
    let body = plan.body().expect("body");
    for (path, artifact) in [
        ("references/requirements.md", "Requirements"),
        ("references/design.md", "Design"),
        ("references/tasks.md", "Tasks"),
    ] {
        assert!(body.contains(path), "Plan must route {path}");
        let procedure = skill_resource_text("sb-plan", path);
        assert!(procedure.contains(artifact), "{path}");
        assert!(procedure.contains("selected by `sb-plan`"), "{path}");
    }
    assert!(body.contains("single-phase mode"));
    assert!(body.contains("Never infer single-phase mode from lifecycle state"));
    assert!(body.contains("stop after\nits phase result"));
    assert!(body.contains("exact\ninstalled path to the applicable reference"));
}

#[test]
fn planning_orchestrator_requires_explicit_scope_without_mutation() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("body");
    assert!(body.contains("neither a named target nor explicit all-Spec intent"));
    assert!(body.contains("stop for the answer before any phase dispatch"));
    assert!(body.contains("Do not infer all scope from the number\nof participants"));
    assert!(body.contains("Scope selection is not delegated-gate authorization"));
    assert!(body.contains("Never infer single-phase mode from lifecycle state"));
    assert!(body.contains("stopping response itself must name the available Spec choices"));
    assert!(body.contains("not the required scope\nquestion"));
    assert!(body.contains("First action: fail closed on an unspecified scope"));
    assert!(body.contains("before reading phase procedures, Spec\nartifacts"));
    assert!(body.contains("only workflow reads permitted are the language-style Rule below and\n`specbind milestone status`"));
    assert!(body.contains("This first-action guard takes\nprecedence"));
}

#[test]
fn planning_orchestrator_keeps_named_scope_inside_the_global_barrier() {
    let body = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("body");
    assert!(body.contains("Never expand named\nscope"));
    assert!(body.contains("outside-scope blocker"));
    assert!(body.contains("Once **every participating Spec**"));
}

#[test]
fn tasks_skill_audits_verification_readiness_before_approval() {
    let body = skill_resource_text("sb-plan", "references/tasks.md");

    assert!(body.contains("execution-readiness audit"));
    assert!(body.contains("canonical test command that is currently\nabsent"));
    assert!(body.contains("do not approve a plan that implements behavior first"));
}

#[test]
fn design_skill_investigates_the_real_verification_foundation() {
    let body = skill_resource_text("sb-plan", "references/design.md");

    assert!(body.contains("Confirm that each named command exists"));
    assert!(body.contains("own creation of the missing script or test\ninterface"));
    assert!(body.contains("confined to source files"));
}

#[test]
fn design_validation_puts_its_read_only_stop_rule_before_commands() {
    let body = skill::find("sb-validate-design")
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
    let body = skill::find("sb-review-task")
        .expect("task review skill")
        .body()
        .expect("body");
    let first_command = body.find("```sh").expect("documented command");
    let preamble = &body[..first_command];

    assert!(preamble.contains("Read-only stop rule — before any probe"));
    assert!(preamble.contains("cannot create\ncaches, coverage data, reports, lockfiles"));
    assert!(preamble.contains("before investigation and\nagain before recording"));
    assert!(body.contains("This is the one permitted repository mutation"));
    assert!(
        body.find("git status --short")
            .expect("initial status capture")
            < body.find("git diff").expect("diff read")
    );
    assert!(body.contains("after the\nbefore/after probe status matched"));

    for required in [
        "specbind artifact read <spec> contract --for consume",
        "specbind steering list",
        "specbind steering read <selector> --for consume",
        "return `CANNOT_REVIEW` instead of approving from a partial view",
    ] {
        assert!(
            body.contains(required),
            "task review must contain {required}"
        );
    }
}

#[test]
fn contract_review_uses_scope_and_the_fixed_historical_yaml_path() {
    let body = skill::find("sb-contract-review")
        .expect("contract review skill")
        .body()
        .expect("body");

    assert!(body.contains("specbind milestone scope"));
    assert!(body.contains("`Status: not_applicable`"));
    assert!(body.contains("specbind schema read contract/v1"));
    assert!(body.contains("git show <baseline>:<specDir>/specs/<spec>/contract.yaml"));
    assert!(body.contains("Do not ask the user to repeat a decision already explicit"));
    assert!(body.contains("scoped behavior introduces no\nmissing persistent seam or guarantee"));
    assert!(body.contains("Do\nnot accept merely because there is no Contract diff"));
    assert!(body.contains("specbind artifact list <spec>"));
    assert!(body.contains("specbind artifact read <spec> design/<artifact-id> --for consume"));
    assert!(body.contains("never shorten\n`design/<artifact-id>`"));
    assert!(body.contains("prefix the exact logical selector reported by `artifact list`"));
    assert!(body.contains("`specs/<spec>#design/main`"));
    assert!(body.contains("Lifecycle states and action labels such as `tasks` and\n`implementation` are not artifact IDs"));
    assert!(
        body.contains("Ask only when the impact introduces a choice the request did not settle")
    );
    assert!(body.contains("before every gate invalidation"));
    assert!(
        body.contains("obtain explicit user confirmation even when milestone scope is unchanged")
    );
    assert!(body.contains("The Design phase owns both the Design set and `contract.yaml`"));
    assert!(body.contains("specbind spec design invalidate <spec>"));
    assert!(body.contains("A response that stops on this finding is incomplete"));
    assert!(body.contains("Include those facts in the reported outcome"));
    assert!(!body.contains("git ls-tree -r --name-only <baseline>"));
}

#[test]
fn steering_add_stops_before_inventing_missing_project_policy() {
    let body = skill::find("sb-steering")
        .expect("steering skill")
        .body()
        .expect("body");

    assert!(
        body.contains("authority to document an\nexisting practice, not to choose a new policy")
    );
    assert!(body.contains("stop before creating a file"));
    assert!(body.contains("Do not\ncombine an accurate statement that tooling is absent with an invented normative\npolicy"));
    assert!(body.contains("write-safety preflight for accepted completion"));
    assert!(body.contains("supplies no document\n  content"));
    assert!(body.contains("A duplicate-identity diagnostic names every colliding path"));
    assert!(body.contains("Matching content or a copy-like filename is\n  not proof"));
    assert!(body.contains("write it only\nto the `project_path` reported"));
}

#[test]
fn implementation_workflow_carries_notes_and_all_failure_routes() {
    let body = skill_package_text("sb-implement");

    for required in [
        "specbind artifact list <spec>",
        "specbind artifact read <spec> implementation-notes/<artifact-id>",
        "specbind protocol read okf-authoring",
        "specbind template read spec implementation-notes/main",
        "`CANNOT_REVIEW`",
        "Do not interrupt it, ask for an immediate\nreturn",
        "New caches, reports, coverage data",
        "The orchestrator never deletes them itself",
        "Do not skip ahead and return here afterwards.",
        "Do not stop merely because the implementation commit succeeded.",
        "This is a separate metadata checkpoint",
        "Adapter guidance is closed-world authority.",
        "`Revision` is unavailable until it is\nreconciled",
        "default is `required` for Spec-backed work and\n`inline` for Direct work",
        "there is no\nseparate project setting to discover",
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
fn direct_review_and_debug_resolve_omitted_subjects_without_guessing() {
    let review = skill::find("sb-review-task")
        .expect("task review skill")
        .body()
        .expect("body");
    let debug = skill::find("sb-debug")
        .expect("debug skill")
        .body()
        .expect("body");

    for body in [review, debug] {
        assert!(body.contains("specbind milestone status"));
        assert!(body.contains("specbind tasks list <spec>"));
        assert!(body.contains("exactly one"));
        assert!(body.contains("ask the user"));
    }
}

#[test]
fn status_names_machine_health_without_claiming_semantic_alignment() {
    let body = skill::find("sb-status")
        .expect("status skill")
        .body()
        .expect("body");

    assert!(body.contains("`State health: consistent`"));
    assert!(body.contains("`Semantic alignment: not evaluated`"));
    assert!(body.contains("Never use state health to rule out an artifact contradiction"));
}

#[test]
fn authoring_skills_produce_each_template_output_once_for_all_references() {
    for name in [
        "sb-discovery",
        "sb-plan",
        "sb-gap-analysis",
        "sb-implement",
        "sb-steering",
    ] {
        let body = skill_package_text(name);
        let same_output = body.contains("same produced output") || body.contains("same\noutput");
        assert!(
            body.contains("`create output=<name>`")
                && body.contains("Replace every")
                && body.contains("reference")
                && same_output
                && body.contains("Markdown fragment"),
            "{name} must own named-output template materialization"
        );
    }
}

#[test]
fn implementation_workflow_is_sequential_and_checkpoints_each_completed_task() {
    let body = skill_package_text("sb-implement");

    for required in [
        "Task execution is sequential.",
        "One task per cycle. Do not batch.",
        "Only a task recorded `completed` is an eligible implementation checkpoint.",
        "before selecting another task",
        "Never defer\nseveral eligible Task checkpoints to the end of the run.",
    ] {
        assert!(
            body.contains(required),
            "implementation skill must contain {required}"
        );
    }

    assert!(!body.contains("`parallel: true`"));
}

#[test]
fn implementation_dispatch_carries_project_local_operating_authority() {
    let body = skill_package_text("sb-implement");

    assert!(body.contains("project-local instruction files"));
    assert!(body.contains("required\n  non-destructive bookkeeping inside the project"));
    assert!(body.contains("does not need a second user approval"));
}

#[test]
fn adapter_consumers_use_the_dedicated_scaffold_marker() {
    for name in [
        "sb-discovery",
        "sb-plan",
        "sb-contract-review",
        "sb-implement",
        "sb-release",
    ] {
        let body = skill_package_text(name);
        assert!(
            body.contains("<!-- specbind:adapter-scaffold -->"),
            "{name} must recognize the dedicated adapter scaffold marker"
        );
        assert!(
            body.contains("marker classifies the whole\ndocument")
                || body.contains("marker classifies the whole document")
                || (name == "sb-release" && body.contains("Do not interpret\nits remaining body")),
            "{name} must make the marker override the entire adapter body"
        );
        assert!(
            !body.contains("A legacy adapter may still carry"),
            "{name} must not preserve legacy adapter compatibility"
        );
    }
}

#[test]
fn release_bootstraps_policy_and_checkpoints_binding_and_finalization() {
    let body = skill_package_text("sb-release");

    for required in [
        "Stop after bootstrap",
        "must run its completion handshake\n   again",
        "approval authorizes only replacing the adapter",
        "Never infer that an adapter is unconfigured",
        "the absence of that approval is\nnot a reason to omit the proposal",
        "Do not rely on `README.md` being the only entry point",
        "Skip sections 3 through 6",
        "A local tag has not left the repository",
        "Binding and explicit rebinding are evidence-preserving",
        "one narrow checkpoint containing\nonly that Roadmap transition",
        "evidence remains fresh but\nrelease preflight still needs a clean project checkpoint",
        "Immediately before finalization, record `git status --short`",
        "Checkpoint only the finalized lifecycle metadata",
        "Publication approval does not authorize pushing this commit",
        "move the published tag to include this later metadata commit",
        "post-release review through `sb-configure`",
        "update Steering, Rules, Templates, Adapters",
    ] {
        assert!(
            body.contains(required),
            "release skill must contain {required}"
        );
    }

    let first_git = body
        .find("specbind adapter read git")
        .expect("initial Git adapter read");
    let bind = body
        .find("specbind milestone bind-release <version>")
        .expect("release binding command");
    assert!(
        first_git < bind,
        "Git checkpoint policy must be known before release binding"
    );

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
    let debug = skill::find("sb-debug").expect("debug skill");
    let metadata = debug.metadata().expect("debug metadata");
    assert!(
        metadata
            .description
            .contains("Use directly when the user asks why a Task failed")
    );
    assert!(metadata.description.contains("never starts implementation"));
    let body = debug.body().expect("body");

    assert!(
        body.contains("- CATEGORY: IMPLEMENTATION | PLAN | ARTIFACT | ENVIRONMENT | UNDETERMINED")
    );
    let first_command = body.find("```sh").expect("documented command");
    let preamble = &body[..first_command];
    assert!(preamble.contains("Final response contract — before any investigation"));
    assert!(preamble.contains("final response is incomplete unless it ends"));
    assert!(preamble.contains("Naming a category in prose does not satisfy"));

    for required in [
        "specbind artifact read <spec> contract --for consume",
        "specbind steering list",
        "specbind steering read <selector> --for consume",
        "Return `UNDETERMINED` and make the failed read the evidence step",
    ] {
        assert!(
            body.contains(required),
            "debug skill must contain {required}"
        );
    }

    let implement = skill::find("sb-implement")
        .expect("implementation skill")
        .metadata()
        .expect("implementation metadata");
    assert!(
        implement
            .description
            .contains("Do not use for a diagnosis-only request")
    );
}

#[test]
fn implementation_completion_questions_route_to_validation_not_status() {
    let validation = skill::find("sb-validate-implementation")
        .expect("implementation validation skill")
        .metadata()
        .expect("validation metadata");
    assert!(validation.description.contains("active Requirement IDs"));
    assert!(
        validation
            .description
            .contains("whether a named Spec with every Task complete is done")
    );

    let status = skill::find("sb-status")
        .expect("status skill")
        .metadata()
        .expect("status metadata");
    assert!(
        status
            .description
            .contains("do not use to judge whether completed implementation is actually done")
    );
    assert!(
        skill::find("sb-status")
            .expect("status skill")
            .body()
            .expect("status body")
            .contains("use\n`sb-validate-implementation`")
    );

    let claim_verification = skill::find("sb-verify-completion").expect("claim verification skill");
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
            .contains("use `sb-validate-implementation` instead")
    );

    let validation_body = skill::find("sb-validate-implementation")
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
    for name in ["sb-review-task", "sb-debug"] {
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
        let clap_builtin = index == 0 && matches!(name, "help" | "version");
        assert!(
            clap_builtin
                || command
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
