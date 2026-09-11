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

    let orchestrator = skill_resource_text("sb-plan", "references/complete-route.md");
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
    let body = skill_resource_text("sb-plan", "references/complete-route.md");
    assert!(body.contains("request to run this skill is **not**"));
    assert!(body.contains("workflow name `sb-plan`"));
    assert!(body.contains("authorized gate names"));
    assert!(body.contains("authorization omitted"));
    assert!(body.contains("from the dispatch does not reach it"));
}

#[test]
fn planning_orchestrator_requires_clean_checkpointed_phase_handoffs() {
    let body = skill_resource_text("sb-plan", "references/complete-route.md");
    assert!(body.contains("adapter-directed checkpoint"));
    assert!(body.contains("git status --short"));
    assert!(body.contains("clean handoff"));
    assert!(body.contains("must not\ncreate a checkpoint owned by the dispatched phase"));
}

#[test]
fn planning_dispatch_carries_project_local_execution_environment() {
    let body = skill_resource_text("sb-plan", "references/complete-route.md");

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
    let body = skill_resource_text("sb-plan", "references/complete-route.md");
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
fn design_recovery_carries_only_the_proven_cli_owned_rewind_delta() {
    let orchestrator = skill_resource_text("sb-plan", "references/complete-route.md");
    let design = skill_resource_text("sb-plan", "references/design.md");
    let recovery = skill_resource_text("sb-plan", "references/replan.md");
    let review = skill::find("sb-contract-review")
        .expect("contract review skill")
        .body()
        .expect("contract review body");
    let validator = skill::find("sb-validate-design")
        .expect("validation skill")
        .body()
        .expect("validation body");

    for body in [orchestrator, design, recovery, review, validator] {
        assert!(body.contains("Design-rewind delta"));
    }
    assert!(orchestrator.contains("accepted Contract\nReview removal when one existed"));
    assert!(orchestrator.contains("exact lifecycle changed-path set"));
    assert!(orchestrator.contains("Missing provenance, a changed\npath set or diff"));
    assert!(design.contains("Do not edit, discard,\nstash, or checkpoint the delta by itself"));
    assert!(review.contains("Do not edit, discard, stash, or commit it separately"));
    assert!(validator.contains("Treat it as read-only lifecycle context"));
    assert!(design.contains("Do not create a\n  separate invalidation checkpoint"));
}

#[test]
fn design_workflows_use_the_design_scoped_traceability_projection() {
    let design = skill_resource_text("sb-plan", "references/design.md");
    let validation = skill::find("sb-validate-design").expect("validation skill");
    let validation_body = validation.body().expect("validation body");

    for body in [design, validation_body] {
        assert!(body.contains("specbind check traceability <spec> --for-design"));
        assert!(body.contains("retained downstream"));
        assert!(body.contains("Task coverage"));
    }
    assert!(design.contains("Tasks authoring and\napproval will run the strict complete check"));
    assert!(validation_body.contains("not a claim that complete traceability passes"));
}

#[test]
fn renewed_contract_review_preserves_retained_delivery_tasks_until_owned_repair() {
    let orchestrator = skill_resource_text("sb-plan", "references/complete-route.md");
    let tasks = skill_resource_text("sb-plan", "references/tasks.md");
    let recovery = skill_resource_text("sb-plan", "references/replan.md");
    let review = skill::find("sb-contract-review").expect("contract review skill");
    let review = review.body().expect("contract review body");

    assert!(review.contains("That file is not a review input"));
    assert!(review.contains("do not read, validate, edit, move, or delete it"));
    assert!(review.contains("`tasks`, `implementation`, or `release_ready`"));
    assert!(orchestrator.contains("does not require unaffected progress to be rewound"));
    assert!(tasks.contains("retained `tasks.yaml` is not a review input"));
    assert!(tasks.contains("plan and execution records unchanged"));
    assert!(recovery.contains("without rewinding otherwise unaffected Tasks gates or progress"));
    assert!(
        recovery
            .contains("Leave every retained\n   `tasks.yaml` and its execution records in place")
    );
    assert!(!recovery.contains("remove only those exact participant `tasks.yaml`"));

    for required in [
        "Give\nevery retained Task with execution state exactly one disposition",
        "reset it to pending when changed active work remains",
        "remove it from the active plan and remove its keyed execution entry",
        "historical-only Task that references no active Requirement cannot remain",
        "renewed-review checkpoint",
        "Never\nrelabel an old completed entry as evidence for a new Requirement",
        "do not delete it before\nthis mapping",
        "sole exception to the\nproject instruction against hand-editing execution state",
        "permits only keeping,\nresetting, remapping, or removing entries that already existed",
        "Never create a\nnew completed or blocked judgment",
    ] {
        assert!(
            tasks.contains(required),
            "retained mapping missing {required}"
        );
    }
}

#[test]
fn design_phase_distinguishes_route_validation_from_gate_mechanics() {
    let body = skill_resource_text("sb-plan", "references/design.md");
    assert!(body.contains("This Design-phase receiver never\ninvokes it"));
    assert!(body.contains("CLI gate does not mechanically require it"));
    assert!(body.contains("complete\nPlan route or a Drive recovery route"));
    assert!(body.contains("requires fresh independent validation before it\nre-dispatches"));
    assert!(body.contains("standalone single-phase\nmode"));
    assert!(body.contains("optional second opinion"));
}

#[test]
fn planning_orchestrator_validates_design_before_delegated_approval() {
    let body = skill_resource_text("sb-plan", "references/complete-route.md");
    assert!(body.contains("without Design-gate authority"));
    assert!(body.contains("Only"));
    assert!(body.contains("retroactively"));
}

#[test]
fn planning_orchestrator_bounds_design_remediation_per_spec_and_finding_history() {
    let body = skill_resource_text("sb-plan", "references/complete-route.md");
    assert!(body.contains("a\nvalidator verdict, not a phase status"));
    assert!(body.contains("at most **two** Design-owned revisions"));
    assert!(body.contains("every prior blocking finding\nID exactly once"));
    assert!(body.contains("The fresh validator owns semantic identity"));
    assert!(body.contains("only new finding IDs block readiness"));
    assert!(body.contains("not per Milestone or per finding"));
    assert!(body.contains("ordinary unfinished Design result"));
    assert!(body.contains("requirements rewind"));
    assert!(body.contains("Never approve"));

    let validation = skill::find("sb-validate-design").expect("validation skill");
    let validation_body = validation.body().expect("validation body");
    assert!(validation_body.contains("finding an ID scoped to this Spec's current Plan run"));
    assert!(validation_body.contains("every prior `BLOCKING` finding ID exactly once"));
    assert!(validation_body.contains("reuse its ID and\nmark it `RESOLVED` or still `BLOCKING`"));
    assert!(validation_body.contains("comparison is incomplete or ambiguous"));
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
    assert!(body.contains("“evidence” means gate evidence in `spec.yaml`"));
    assert!(body.contains(
        "does not delete retained Design, Contract, `tasks.yaml`, Task\nexecution records"
    ));
    assert!(body.contains(
        "remain visible repair input until\ntheir owning phases explicitly reconcile them"
    ));
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
    assert!(body.contains("not a requirement to run one command per\nquestion"));
    assert!(body.contains("library-only artifact"));
    assert!(body.contains("Do not invent an ad-hoc shell, language, or\nsmoke command"));
    assert!(body.contains("return\n`MANUAL_VERIFY_REQUIRED`"));
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
        Some("[<spec> | --all | --shared] [requirements|design|tasks]")
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
    assert!(body.contains("references/complete-route.md"));
    let complete = skill_resource_text("sb-plan", "references/complete-route.md");
    assert!(complete.contains("exact\ninstalled path to the applicable reference"));
}

#[test]
fn completion_revalidation_checkpoints_evidence_withdrawal_before_fresh_preflight() {
    let validation =
        skill::find("sb-validate-implementation").expect("implementation validation skill");
    let body = validation.body().expect("validation body");

    for required in [
        "specbind spec completion invalidate <spec>",
        "only the target `spec.yaml`",
        "only\ncompletion evidence was removed",
        "specbind adapter read git --for consume",
        "independent, narrow checkpoint",
        "cannot resume until that checkpoint is\nclosed",
        "checkpointed separately as fresh completion metadata",
        "Never combine the two checkpoints",
    ] {
        assert!(
            body.contains(required),
            "completion revalidation missing {required}"
        );
    }
}

#[test]
fn contract_review_transfers_requirements_and_scope_repairs_to_their_owners() {
    let review = skill::find("sb-contract-review")
        .expect("contract review skill")
        .body()
        .expect("contract review body");
    let requirements = skill_resource_text("sb-plan", "references/requirements.md");
    let route = skill_resource_text("sb-plan", "references/complete-route.md");

    for required in [
        "do not invoke**\n`specbind spec requirements invalidate <spec>`",
        "operation-specific confirmation",
        "The Requirements phase owns the invalidation, repair, approval",
        "Do not invoke**\n`specbind milestone update-scope`",
        "route it to `sb-discovery`",
        "Never remove completed Direct\nitems, retained Tasks, or another Spec's progress",
    ] {
        assert!(review.contains(required), "review missing {required}");
    }
    assert!(requirements.contains("A Contract\nReview remediation handoff is different"));
    assert!(requirements.contains("relayed confirmation authorizes this Requirements receiver"));
    assert!(route.contains("The Requirements receiver invokes the rewind"));
    assert!(route.contains("Review must not invoke `milestone update-scope`"));
}

use super::*;
