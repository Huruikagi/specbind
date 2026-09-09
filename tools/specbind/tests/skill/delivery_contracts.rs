#[test]
fn planning_orchestrator_requires_explicit_scope_without_mutation() {
    let entrypoint = skill::find("sb-plan")
        .expect("planning orchestrator")
        .body()
        .expect("body");
    let complete = skill_resource_text("sb-plan", "references/complete-route.md");
    let body = format!("{entrypoint}\n{complete}");
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
    let body = skill_resource_text("sb-plan", "references/complete-route.md");
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
    assert!(body.contains("the input or state it\nneeds, how that will be supplied"));
    assert!(body.contains("before\npresenting the plan as ready"));
    assert!(body.contains("replace required real-connection proof with boundary tests"));
    assert!(body.contains("Only when another Spec supplies a prerequisite"));
    assert!(body.contains("compare availability with `specbind milestone scope`"));
    assert!(body.contains("does not authorize editing that Spec"));
    assert!(body.contains("do not require a new public API, injection"));
    assert!(body.contains("route to its owner, and stop this\nplan's approval"));
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
    assert!(
        body.contains("Fix the review scope from CLI-owned lifecycle state before reading prose")
    );
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
fn steering_checkpoints_only_its_verified_document_changes() {
    let body = skill::find("sb-steering")
        .expect("steering skill")
        .body()
        .expect("body");

    let verification = body
        .find("## 5. Verify what you wrote")
        .expect("verification section");
    let checkpoint = body.find("## 6. Checkpoint").expect("checkpoint section");
    let report = body.find("## 7. Report").expect("report section");
    assert!(verification < checkpoint && checkpoint < report);
    for required in [
        "A request to stop after Steering or\nafter synchronization still proceeds through this section",
        "Only an explicit instruction that forbids\ncommits skips",
        "specbind adapter read git --for consume",
        "NO_CHANGE ADAPTER_ABSENT",
        "NO_CHANGE ADAPTER_SCAFFOLD",
        "git status --short",
        "Stage only those verified Steering paths.",
        "if an unrelated change overlaps a touched path, stop",
        "Do not amend, rebase, push",
        "whether the checkpoint\nwas committed, intentionally absent or scaffolded, or failed",
    ] {
        assert!(
            body.contains(required),
            "steering checkpoint contract must contain {required}"
        );
    }
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
        .find("specbind adapter read git --for consume")
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
fn status_interprets_machine_health_without_routinely_reporting_it() {
    let body = skill::find("sb-status")
        .expect("status skill")
        .body()
        .expect("body");

    assert!(body.contains("`State health: consistent`"));
    assert!(body.contains("`Semantic alignment: not evaluated`"));
    assert!(body.contains("Never use state health to rule out an artifact contradiction"));
    assert!(
        body.contains("Treat the CLI projection as input to judgment, not as a report template")
    );
    assert!(
        body.contains("These are interpretation boundaries, not routine\nuser-facing disclaimers")
    );
    assert!(body.contains("Always report inconsistent health"));
    assert!(body.contains("`Task plan authority: not current; reconcile in\nTasks phase`"));
    assert!(body.contains("Never label the plan `retained`, `preserved`, or\n`premature`"));
    assert!(body.contains("do not choose retain, revise-and-reset, or\nremove"));
    assert!(body.contains("a future owner, not the immediate next action"));
    assert!(body.contains("complete the intervening Design and\nContract Review work"));
    assert!(body.contains("Never direct\nthe user straight to Tasks"));
    assert!(
        body.contains("Filtering may compress facts but may not strengthen them into a remedy")
    );
}

#[test]
fn status_routes_named_and_milestone_reports_to_distinct_procedures() {
    let entrypoint = skill::find("sb-status")
        .expect("status skill")
        .body()
        .expect("body");
    let milestone = skill_resource_text("sb-status", "references/milestone.md");
    let spec = skill_resource_text("sb-status", "references/spec.md");

    assert!(entrypoint.contains("references/milestone.md"));
    assert!(entrypoint.contains("references/spec.md"));
    assert!(milestone.contains("specbind milestone status"));
    assert!(milestone.contains("completed, pending, and blocked Task counts"));
    assert!(milestone.contains("each blocked Task ID and its recorded reason"));
    assert!(milestone.contains("`TASKS_BLOCKED`"));
    assert!(milestone.contains("Do not replace it with a generic\ndirty-worktree explanation"));
    assert!(
        milestone
            .contains("Do not append healthy State-health, fresh-review, or semantic-alignment")
    );
    assert!(
        milestone
            .contains("do not prescribe Task reordering, plan repair, or an implementation change")
    );
    assert!(spec.contains("specbind spec status <spec>"));
    assert!(spec.contains("specbind tasks show <spec> <task-id>"));
    assert!(spec.contains("A blocked Task with no next Task is\na stop condition"));
    assert!(spec.contains("Do not append healthy State-health, fresh-Gate, or"));
    assert!(spec.contains(
        "strengthen the reason into plan repair, Task reordering, or an\nimplementation change"
    ));
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
fn adapter_consumers_use_the_active_guidance_projection() {
    for name in [
        "sb-discovery",
        "sb-plan",
        "sb-contract-review",
        "sb-implement",
        "sb-release",
        "sb-review-task",
        "sb-validate-design",
        "sb-validate-implementation",
    ] {
        let body = skill_package_text(name);
        for command in body
            .lines()
            .filter(|line| line.starts_with("specbind adapter read "))
        {
            assert!(
                command.ends_with(" --for consume"),
                "{name} consumer must not read raw adapter content: {command}"
            );
        }
        assert!(
            body.contains("ADAPTER_SCAFFOLD"),
            "{name} must handle the projected inactive state"
        );
    }

    let configure = skill_package_text("sb-configure");
    assert!(configure.contains("specbind adapter read <selector>"));
    assert!(configure.contains("specbind adapter read git --for consume"));
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
        .find("specbind adapter read git --for consume")
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
        .find("specbind adapter read git --for consume")
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

    assert!(body.contains(
        "- CATEGORY: IMPLEMENTATION | REVIEW | PLAN | ARTIFACT | ENVIRONMENT | UNDETERMINED"
    ));
    assert!(body.contains("Uncertain ownership is `UNDETERMINED`, not\n  `REVIEW`"));
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
    assert!(
        implement
            .description
            .contains("Implement or resume one roadmap item")
    );
    assert!(
        implement
            .description
            .contains("through returned review or diagnosis")
    );
}

#[test]
fn authorized_lifecycle_completion_routes_to_validation_not_status() {
    let validation = skill::find("sb-validate-implementation")
        .expect("implementation validation skill")
        .metadata()
        .expect("validation metadata");
    assert!(validation.description.contains("active Requirement IDs"));
    assert!(
        validation
            .description
            .contains("explicitly asks for lifecycle validation")
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
    assert!(
        claim_verification
            .body()
            .expect("claim verification body")
            .contains("exact `Active requirement set`")
    );
    assert!(
        claim_verification
            .body()
            .expect("claim verification body")
            .contains("If both readings appear possible")
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
    assert!(validation_body.contains("Enter this workflow only with explicit authority"));
}

#[test]
fn project_instructions_route_dedicated_intents_without_granting_completion() {
    let body = specbind::project_instructions::BODY;
    for route in [
        "`sb-contract-review`",
        "`sb-gap-analysis`",
        "`sb-release`",
        "`sb-validate-design`",
        "`sb-verify-completion`",
    ] {
        assert!(
            body.contains(route),
            "project instructions must route {route}"
        );
    }
    assert!(body.contains("answer must change nothing"));
    assert!(body.contains("recording\n  completion evidence on `GO`"));
    assert!(body.contains("do not by\n  themselves authorize that mutation"));
    assert!(!body.contains("installed `specbind-*` Skills"));
}

#[test]
fn forward_test_clarifications_preserve_read_and_semantic_boundaries() {
    let design = skill::find("sb-validate-design")
        .expect("design validation skill")
        .body()
        .expect("design validation body");
    assert!(design.contains("exact `Active requirement set`"));
    assert!(design.contains("never derive the review scope from those markers"));

    let validation = skill::find("sb-validate-implementation")
        .expect("implementation validation skill")
        .body()
        .expect("implementation validation body");
    assert!(validation.contains("means only that this lifecycle and checkout state"));
    assert!(validation.contains("does not inspect the project Validation adapter"));

    let drive = skill::find("sb-drive")
        .expect("drive skill")
        .body()
        .expect("drive body");
    assert!(drive.contains("Dispatch its exact item and summary"));
    assert!(drive.contains("normalize the handoff as\n`REROUTABLE` plus `HUMAN_DECISION`"));

    let reverse = skill::find("sb-adopt")
        .expect("adoption skill")
        .body()
        .expect("body");
    let preflight = reverse
        .find("specbind adoption preflight")
        .expect("reverse preflight");
    let selected_area = reverse
        .find("require the maintainer to name the selected area")
        .expect("selected area requirement");
    assert!(preflight < selected_area);
    assert!(reverse.contains("separate\nmaintainer-gated workflow"));
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

use super::*;
