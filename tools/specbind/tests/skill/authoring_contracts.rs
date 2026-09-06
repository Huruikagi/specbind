#[test]
fn discovery_entrypoint_has_no_reverse_adoption_route() {
    let entry = skill::find("sb-discovery").expect("discovery skill");
    let body = entry.body().expect("discovery body");

    for reference in [
        "references/ordinary.md",
        "references/local-files.md",
        "references/github-milestone.md",
    ] {
        assert!(
            body.contains(reference),
            "missing direct route to {reference}"
        );
    }
    assert!(body.contains("Do not load a provider procedure for an ordinary"));
    assert!(!body.contains("reverse"));
    assert!(!body.contains("adoption"));
    assert!(!body.contains("specbind milestone status"));
    assert!(!body.contains("specbind adoption preflight"));

    let ordinary = skill_resource_text("sb-discovery", "references/ordinary.md");
    assert!(ordinary.contains("The request to run this skill is **not** confirmation"));
    assert!(ordinary.contains("specbind milestone create --scope -"));
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
        aftercare.contains("specbind adapter read git --for consume"),
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
        "specbind adapter read validation --for consume",
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
fn adoption_skill_keeps_evidence_separate_and_owns_non_release_finalization() {
    let body = skill_package_text("sb-adopt");
    for required in [
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
        "{\"log_entries\":[{\"spec\":\"<spec-id>\",\"summary\":\"<one-line summary>\"}]}",
        "ADOPTION_RESUME_READY",
        "handler.mode=reverse_resume",
        "explicitly asked to resume",
    ] {
        assert!(
            body.contains(required),
            "Adoption package must contain {required}"
        );
    }
    for retired in ["references/adopt-start.md", "references/adopt-resume.md"] {
        assert!(
            !body.contains(retired),
            "Adoption package must not retain retired resource {retired}"
        );
    }
    assert!(
        !body.to_ascii_lowercase().contains("dossier"),
        "Adoption package must describe the temporary record without treating dossier as a product term"
    );
}

#[test]
fn reverse_resume_preserves_contract_review_ownership_and_readiness() {
    let reverse = skill::find("sb-adopt")
        .expect("adoption skill")
        .body()
        .expect("body");
    assert!(reverse.contains("dispatch the installed\n`sb-contract-review` workflow"));
    assert!(reverse.contains("dispatch the initial evidence readers again"));
    assert!(reverse.contains("assessment and findings must still be presented before\nacceptance"));
    assert!(reverse.contains("Do not pause for another confirmation"));

    let review = skill::find("sb-contract-review")
        .expect("contract review skill")
        .body()
        .expect("contract review body");
    assert!(review.contains("use the reported `Milestone kind`"));
    assert!(review.contains("`adoption_ready`, with no Tasks"));
}

#[test]
fn reverse_discovery_resolves_and_checkpoints_deferred_findings_after_creation() {
    let body = skill::find("sb-adopt")
        .expect("adoption skill")
        .body()
        .expect("body");
    let read = body
        .find("specbind adapter read deferred --for consume")
        .expect("exact deferred selector");
    let create = body
        .find("specbind milestone create --scope")
        .expect("milestone creation");
    let write = body
        .find("Only after milestone creation and provenance verification")
        .expect("post-creation finding write");
    assert!(read < create && create < write);
    assert!(body.contains("do not guess `deferred-findings`"));
    assert!(body.contains("pending adapter records"));
    assert!(body.contains("verified deferred destination when written as one\nDiscovery unit"));
    assert!(body.contains("one closed contradiction ledger"));
    assert!(body.contains("Give every difference exactly one visible disposition"));
    assert!(body.contains("Do not silently drop a naming, behavior, or\nboundary mismatch"));
    assert!(body.contains("use that value literally"));
    assert!(body.contains("never\n`.specbind/specs/adoption/reverse-discovery.yaml`"));
    assert!(body.contains(
        "outside it prevents the record from\nentering Spec discovery or the Contract graph"
    ));
    assert!(body.contains("exact project-relative `destination`"));
    assert!(body.contains("distinguish the adapter output from source\ndrift"));
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
    let body = skill_resource_text("sb-discovery", "references/ordinary.md");
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
    let body = skill_resource_text("sb-discovery", "references/ordinary.md");
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
    assert!(body.contains("There is no CLI `artifact write` subcommand"));
    assert!(body.contains("host's ordinary file\nediting capability"));
}

#[test]
fn discovery_does_not_invalidate_a_gate_that_was_never_approved() {
    let body = skill_resource_text("sb-discovery", "references/ordinary.md");

    assert!(
        body.contains("`not_reached` is not approved")
            && body.contains("existing Requirements artifact")
            && body.contains("is approved."),
        "discovery must distinguish an existing artifact from approved gate evidence"
    );
}

#[test]
fn discovery_uses_the_contract_graph_for_explicit_path_ownership() {
    let body = skill_resource_text("sb-discovery", "references/ordinary.md");
    assert!(body.contains("specbind contract owners <path>"));
    assert!(body.contains("Any returned owner proves that path enters the workflow"));
    assert!(body.contains(
        "`Owners: none` proves only that current Contracts declare no owner for\nthat path"
    ));
    assert!(body.contains("do not infer one merely to run this lookup"));
}

#[test]
fn implementation_requires_a_matching_pending_roadmap_item() {
    let body = skill::find("sb-implement")
        .expect("implementation skill")
        .body()
        .expect("implementation body");
    assert!(body.contains("must match a pending Spec-backed or Direct item"));
    assert!(body.contains("A bare change request is not a Direct item"));
    assert!(body.contains("stop before reading or changing implementation"));
    assert!(body.contains("route the\nrequest through `sb-discovery`"));
}

#[test]
fn discovery_presents_an_approvable_scope_at_the_confirmation_boundary() {
    let body = skill_resource_text("sb-discovery", "references/ordinary.md");
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
        "Bind a version-shaped Milestone title",
        "specbind milestone bind-release <version>",
        "scope approval alone does not\nauthorize that replacement",
        "do not trim it, extract a version from prose",
        "ordinary step 8 checkpoint **before binding**",
        "second\nnarrow checkpoint containing only the Roadmap's binding change",
        "do not\ninvent checkpoint authority",
        "Stop on a binding\nerror",
    ] {
        assert!(
            provider.contains(required),
            "GitHub provider missing {required}"
        );
    }
}

#[test]
fn github_milestone_title_recognition_preserves_portable_version_labels() {
    let provider = skill_resource_text("sb-discovery", "references/github-milestone.md");
    let pattern = provider
        .split_once("```regex\n")
        .expect("version recognition grammar")
        .1
        .split_once("\n```")
        .expect("grammar fence")
        .0;
    let schema = serde_json::json!({
        "type": "string",
        "maxLength": 64,
        "allOf": [
            { "pattern": "^[A-Za-z0-9][A-Za-z0-9._+-]{0,63}$" },
            { "pattern": pattern }
        ]
    });
    let validator = jsonschema::validator_for(&schema).expect("valid title grammar");
    for title in [
        "v1",
        "1.4",
        "v1.4.0",
        "1.4.0-rc.1",
        "1.4.0+build.7",
        "2026-09-06",
    ] {
        assert!(validator.is_valid(&serde_json::json!(title)), "{title}");
    }
    for title in [
        "",
        "Backlog",
        "release_42",
        "Release v1.4.0",
        " v1.4.0",
        "v1.4.0 ",
        "v1/4/0",
        "v１.４.０",
        "v1..4",
        "v1.4.0\n",
    ] {
        assert!(!validator.is_valid(&serde_json::json!(title)), "{title:?}");
    }
    assert!(!validator.is_valid(&serde_json::json!("1".repeat(65))));
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

use super::*;
