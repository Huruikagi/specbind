use specbind::{install::Agent, project_instructions};

#[test]
fn writes_each_agent_its_own_instruction_file() {
    // A shared file would leave one agent without instructions, because each
    // reads only its own.
    assert_eq!(project_instructions::target(Agent::ClaudeCode), "CLAUDE.md");
    assert_eq!(project_instructions::target(Agent::Codex), "AGENTS.md");
}

#[test]
fn creates_a_file_holding_only_the_block() {
    let applied = project_instructions::apply(None).expect("a missing file is created");
    assert!(!applied.had_block);
    assert_eq!(applied.content, project_instructions::block());
    assert!(applied.content.starts_with("<!-- specbind:block -->\n"));
    assert!(applied.content.ends_with("<!-- /specbind:block -->\n"));
    assert!(applied.content.contains("This project uses SpecBind"));
    assert!(
        applied
            .content
            .contains("validation rule, limit, or rejected case")
    );
    assert!(
        applied
            .content
            .contains("When that classification is genuinely unclear, enter the")
    );
}

#[test]
fn appends_to_existing_content_behind_one_blank_line() {
    let applied = project_instructions::apply(Some("# Project\n\nOur own rules.\n"))
        .expect("appending is safe");
    // Adding a block to an existing file removes nothing, so it is not a
    // replacement and needs no committed clean repository.
    assert!(!applied.had_block);
    assert_eq!(
        applied.content,
        format!(
            "# Project\n\nOur own rules.\n\n{}",
            project_instructions::block()
        )
    );
}

#[test]
fn normalizes_separation_without_touching_existing_text() {
    for existing in [
        "# Project",
        "# Project\n",
        "# Project\n\n",
        "# Project\n\n\n",
    ] {
        let applied = project_instructions::apply(Some(existing))
            .expect("appending is safe")
            .content;
        assert!(
            applied.starts_with("# Project\n\n"),
            "{existing:?} produced {applied:?}"
        );
        assert!(applied.ends_with(&project_instructions::block()));
    }
}

#[test]
fn replaces_only_the_marked_region() {
    let existing = "# Project\n\nBefore.\n\n<!-- specbind:block -->\nstale wording\n<!-- /specbind:block -->\n\nAfter.\n";
    let applied = project_instructions::apply(Some(existing)).expect("one pair replaces cleanly");
    assert!(applied.had_block);
    let applied = applied.content;
    assert!(applied.starts_with("# Project\n\nBefore.\n\n"), "{applied}");
    assert!(applied.ends_with("\n\nAfter.\n"), "{applied}");
    assert!(!applied.contains("stale wording"), "{applied}");
    assert!(
        applied.contains(&project_instructions::block()),
        "{applied}"
    );
}

#[test]
fn is_idempotent_once_applied() {
    let once = project_instructions::apply(Some("# Project\n"))
        .expect("append")
        .content;
    let twice = project_instructions::apply(Some(&once))
        .expect("replace in place")
        .content;
    assert_eq!(once, twice);
}

#[test]
fn refuses_to_guess_which_of_two_blocks_is_authoritative() {
    let doubled = format!(
        "{}{}",
        project_instructions::block(),
        project_instructions::block()
    );
    let error = project_instructions::apply(Some(&doubled)).expect_err("duplicate markers stop");
    assert_eq!(error.code, "PROJECT_INSTRUCTIONS_MARKERS_INVALID");
}

#[test]
fn refuses_an_unpaired_or_reversed_marker() {
    for (content, code) in [
        (
            "<!-- specbind:block -->\nbody\n",
            "PROJECT_INSTRUCTIONS_MARKERS_INVALID",
        ),
        (
            "body\n<!-- /specbind:block -->\n",
            "PROJECT_INSTRUCTIONS_MARKERS_INVALID",
        ),
        (
            "<!-- /specbind:block -->\nbody\n<!-- specbind:block -->\n",
            "PROJECT_INSTRUCTIONS_MARKERS_REVERSED",
        ),
    ] {
        let error = project_instructions::apply(Some(content)).expect_err("malformed markers stop");
        assert_eq!(error.code, code, "{content:?}");
    }
}

/// Resolving this would need a Markdown parse whose outcome a reader cannot
/// easily predict, and stopping is recoverable.
#[test]
fn counts_a_marker_inside_a_code_fence() {
    let documented = "# Project\n\n```markdown\n<!-- specbind:block -->\n```\n";
    let error =
        project_instructions::apply(Some(documented)).expect_err("an unpaired marker stops");
    assert_eq!(error.code, "PROJECT_INSTRUCTIONS_MARKERS_INVALID");
}
