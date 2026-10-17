use specbind::protocol;

/// The complete accepted protocol selector set, in stable order.
const ACCEPTED_SELECTORS: [&str; 13] = [
    "okf-authoring",
    "requirements-review",
    "source-material",
    "design-discovery",
    "design-authoring",
    "design-validation",
    "gap-analysis",
    "task-planning",
    "task-implementation",
    "task-review",
    "debug",
    "completion-verification",
    "contract-review",
];

#[test]
fn embeds_exactly_the_accepted_protocol_set() {
    let selectors = protocol::list()
        .iter()
        .map(|entry| entry.selector)
        .collect::<Vec<_>>();

    assert_eq!(selectors, ACCEPTED_SELECTORS);
}

#[test]
fn every_protocol_resolves_to_substantive_content() {
    for selector in ACCEPTED_SELECTORS {
        let entry = protocol::read(selector).expect("accepted selector must resolve");
        assert_eq!(entry.selector, selector);
        assert!(
            !entry.purpose.trim().is_empty(),
            "{selector} must declare a purpose"
        );
        let content = entry.content();
        assert!(
            content.starts_with("# "),
            "{selector} must begin with its title heading"
        );
        assert!(
            content.ends_with('\n'),
            "{selector} must end with a newline"
        );
        assert!(
            content.len() > 512,
            "{selector} must carry substantive guidance"
        );
        assert!(
            !content.starts_with("---"),
            "{selector} must not carry OKF Front Matter"
        );
    }
}

#[test]
fn rejects_unknown_selectors() {
    assert!(protocol::read("absent-protocol").is_none());
    assert!(protocol::read("").is_none());
    assert!(
        protocol::read("OKF-Authoring").is_none(),
        "selectors are exact lowercase kebab-case identifiers"
    );
}

#[test]
fn direct_review_has_its_own_complete_governing_inputs() {
    let content = protocol::read("task-review").unwrap().content();
    let direct = content.split("For a Direct item,").nth(1).unwrap();
    for required in [
        "observable done condition",
        "specbind milestone scope --include-body",
        "specbind steering list",
        "specbind steering read <selector> --for consume",
        "specbind contract shared read",
        "OK SHARED_CONTRACT_ABSENT",
        "CANNOT_REVIEW",
        "No Spec Requirements, Design,",
        "goes back to Discovery",
    ] {
        assert!(
            direct.contains(required),
            "Direct review is missing {required}"
        );
    }
}

#[test]
fn design_guidance_read_failure_is_not_a_design_verdict() {
    let content = protocol::read("design-validation").unwrap().content();
    assert!(content.contains("Read every current Steering document independently"));
    assert!(content.contains("`BLOCKED / INPUT` and no Design verdict"));
    assert!(content.contains("conflict between Design and readable Steering"));
}

#[test]
fn recovery_assessors_preserve_finding_identity_and_terminal_protocols() {
    for (selector, terminal) in [("debug", "## Diagnosis"), ("task-review", "## Review")] {
        let content = protocol::read(selector)
            .expect("assessment protocol")
            .content();
        assert!(content.contains("## Recovery findings"));
        assert!(content.contains("every prior ID exactly once"));
        assert!(content.contains("RESOLVED | BLOCKING"));
        assert!(content.contains("failed-attempt reasoning"));
        assert!(content.contains("current evidence"));
        assert!(content.rfind(terminal).unwrap() > content.rfind("## Recovery findings").unwrap());
    }
}

#[test]
fn requirements_review_requires_guarded_inline_retirement() {
    let content = protocol::read("requirements-review")
        .expect("requirements review protocol")
        .content();

    assert!(
        content.contains("Never delete or reuse retired identities"),
        "the protocol must name the current retirement boundary"
    );
    assert!(
        content.contains("a marker alone is not completion"),
        "the protocol must not contradict the requirements skill's retirement stop"
    );
}

#[test]
fn source_material_is_context_that_must_be_promoted() {
    let content = protocol::read("source-material")
        .expect("source material protocol")
        .content();

    for required in [
        "Enumerate every\nitem",
        "Every item receives a visible disposition",
        "Source items and Briefs are not fingerprinted Gate inputs",
        "Requirements and Design read the declared source items as context",
        "remote Source Item\nuses the Discovery-captured context recorded",
        "not\nsilently re-queried",
        "Permission to read a source grants no authority",
    ] {
        assert!(
            content.contains(required),
            "missing source contract: {required}"
        );
    }
}

#[test]
fn dispatched_role_protocols_define_their_parseable_result_blocks() {
    let implementation = protocol::read("task-implementation")
        .expect("task implementation protocol")
        .content();
    assert!(implementation.contains("- STATUS: READY_FOR_REVIEW | BLOCKED | NEEDS_CONTEXT"));
    assert!(implementation.contains("Verification must leave a clean handoff"));
    assert!(implementation.contains("Never clean up a pre-existing or unrelated path"));
    assert!(implementation.contains("Obey project-local operating instructions"));
    assert!(implementation.contains("ordinary, non-destructive\nproject-local bookkeeping"));
    assert!(implementation.contains("Do not stop to ask for a second approval"));
    assert!(implementation.contains("destructive/external-action boundary"));
    assert!(implementation.contains("cleanup after the run is not enough"));
    assert!(implementation.contains("Run that exact public command again"));
    assert!(implementation.contains("repeat invocation recreates untracked"));

    let review = protocol::read("task-review")
        .expect("task review protocol")
        .content();
    assert!(review.contains("different Task, Spec, or later lifecycle boundary"));
    assert!(review.contains("unimplemented downstream connection"));
    assert!(review.contains("- VERDICT: APPROVED | REJECTED | CANNOT_REVIEW"));
    assert!(review.contains("[BLOCKING|DEFERRED|RESOLVED]"));
    assert!(review.split_whitespace().collect::<Vec<_>>().join(" ").contains(
        "For that Spec-backed subject, read the Spec's Contract and every current Steering document"
    ));
    assert!(review.contains("partial or unreadable Steering set"));

    let debug = protocol::read("debug").expect("debug protocol").content();
    assert!(debug.contains(
        "- CATEGORY: IMPLEMENTATION | REVIEW | PLAN | ARTIFACT | ENVIRONMENT | UNDETERMINED"
    ));
    assert!(debug.contains("Review scope defect"));
    assert!(debug.contains("not an implementation or artifact change"));
    assert!(debug.contains("A\ncategory mentioned only in prose"));
    assert!(debug.contains("is not a\nparseable diagnosis"));
    assert!(debug.contains("the Spec's Contract, current Steering"));
    assert!(debug.contains("Do not infer that a Contract boundary is missing"));
}

#[test]
fn task_planning_requires_each_task_to_be_verifiable_when_actionable() {
    let content = protocol::read("task-planning")
        .expect("task planning protocol")
        .content();

    assert!(content.contains("independently finishable and verifiable"));
    assert!(content.contains("test interface does not\n  exist yet"));
    assert!(content.contains("later verification task cannot retroactively"));
}

#[test]
fn design_authoring_reconciles_verification_with_the_change_boundary() {
    let content = protocol::read("design-authoring")
        .expect("design authoring protocol")
        .content();

    assert!(content.contains("Verification has an implementation boundary"));
    assert!(content.contains("required command or test interface is absent"));
    assert!(content.contains("confined to one source path"));
    assert!(content.contains("verification strategy is executable"));
}

#[test]
fn completion_verification_preserves_the_exact_executed_command() {
    let content = protocol::read("completion-verification")
        .expect("completion verification protocol")
        .content();

    assert!(content.contains("preserve the exact executed command\nstring"));
    assert!(content.contains("shortened argument, placeholder"));
    assert!(content.contains("repeatable clean\ninvocation"));
    assert!(content.contains("without cleanup between the command"));
    assert!(content.contains("exit code is zero"));
}

#[test]
fn contract_review_detects_behavior_missing_from_an_unchanged_contract() {
    let content = protocol::read("contract-review")
        .expect("contract review protocol")
        .content();

    assert!(content.contains("Compare the Roadmap's scoped behavior"));
    assert!(content.contains("Contract silence does not make that change\ncompatible"));
    assert!(content.contains("leave the review unaccepted while the omission remains"));
}
