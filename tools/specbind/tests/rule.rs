use specbind::{config::ProjectLanguage, rule};

/// The complete Decision 0093 installed default set.
const ACCEPTED_RULES: [&str; 7] = [
    "ears-format.md",
    "design-principles.md",
    "design-template-selection.md",
    "contract-principles.md",
    "tasks-generation.md",
    "steering-principles.md",
    "language-style.md",
];

const ACCEPTED_SELECTORS: [&str; 7] = [
    "ears-format",
    "design-principles",
    "design-template-selection",
    "contract-principles",
    "tasks-generation",
    "steering-principles",
    "language-style",
];

#[test]
fn embeds_exactly_the_accepted_default_rule_set() {
    let names = rule::defaults()
        .iter()
        .map(|entry| entry.file_name)
        .collect::<Vec<_>>();

    assert_eq!(names, ACCEPTED_RULES);

    let selectors = rule::defaults()
        .iter()
        .map(|entry| entry.selector)
        .collect::<Vec<_>>();
    assert_eq!(selectors, ACCEPTED_SELECTORS);
    for selector in ACCEPTED_SELECTORS {
        assert!(rule::find(selector).is_some());
    }
    for unknown in ["deployment", "ears-format.md", "", "Ears-format"] {
        assert!(rule::find(unknown).is_none(), "{unknown} must not resolve");
    }
}

#[test]
fn installs_language_style_by_default_only_for_japanese() {
    let english = rule::installed_defaults(ProjectLanguage::En)
        .map(|entry| entry.selector)
        .collect::<Vec<_>>();
    let japanese = rule::installed_defaults(ProjectLanguage::Ja)
        .map(|entry| entry.selector)
        .collect::<Vec<_>>();

    assert!(!english.contains(&"language-style"));
    assert!(japanese.contains(&"language-style"));
    assert_eq!(english.len(), 6);
    assert_eq!(japanese.len(), 7);
}

#[test]
fn default_design_template_selection_classifies_main_and_ui() {
    let content = rule::find("design-template-selection")
        .expect("selection rule")
        .content();
    let selectors = vec!["design/main".to_owned(), "design/ui".to_owned()];
    assert!(rule::validate_design_template_selection(content, &selectors).is_empty());
}

#[test]
fn design_template_selection_fails_closed_for_incomplete_or_stale_policy() {
    let selectors = vec!["design/main".to_owned(), "design/ui".to_owned()];
    let content = concat!(
        "# Selection\n\n",
        "## `design/main`\n\nMode: conditional\n\n",
        "## `design/legacy`\n\nMode: disabled\n",
    );
    let issues = rule::validate_design_template_selection(content, &selectors);
    let codes = issues.iter().map(|issue| issue.code).collect::<Vec<_>>();
    assert!(codes.contains(&"RULE_DESIGN_TEMPLATE_CONDITION_MISSING"));
    assert!(codes.contains(&"RULE_DESIGN_TEMPLATE_SELECTOR_MISSING"));
    assert!(codes.contains(&"RULE_DESIGN_TEMPLATE_SELECTOR_UNKNOWN"));
    assert!(codes.contains(&"RULE_DESIGN_TEMPLATE_REQUIRED_MISSING"));
}

#[test]
fn every_default_rule_is_an_okf_rule_concept() {
    for entry in rule::defaults() {
        assert_eq!(entry.selector, entry.file_name.trim_end_matches(".md"));
        let content = entry.content();
        assert!(
            content.starts_with("---\ntype: SpecBind Rule\n---\n"),
            "{} must open with the SpecBind Rule profile",
            entry.file_name
        );
        assert!(
            !entry.purpose.trim().is_empty(),
            "{} must declare a purpose",
            entry.file_name
        );
        assert!(
            content.ends_with('\n'),
            "{} must end with a newline",
            entry.file_name
        );
        assert!(
            content.len() > 512,
            "{} must carry substantive guidance",
            entry.file_name
        );
        for forbidden in ["artifact_id:", "schema_version:"] {
            assert!(
                !content.contains(forbidden),
                "{} must not carry a {forbidden} field",
                entry.file_name
            );
        }
    }
}

#[test]
fn task_generation_chooses_a_default_test_grouping_convention() {
    let content = rule::defaults()
        .iter()
        .find(|entry| entry.file_name == "tasks-generation.md")
        .expect("task-generation rule")
        .content();

    assert!(content.contains("write tests as part of the task that introduces the behavior"));
    assert!(content.contains("Split\nverification into its own task only when"));
    assert!(content.contains("behavior delivered by several\nearlier tasks"));
    assert!(content.contains("Do not create a\nsecond task merely to restate"));
    assert!(content.contains("canonical test command or test interface does not exist yet"));
    assert!(content.contains("Never put that setup in a later task"));
}

#[test]
fn contract_principles_states_complete_live_default_policy() {
    let content = rule::find("contract-principles")
        .expect("contract principles rule")
        .content();

    assert!(content.contains("This project's default posture is conservative"));
    assert!(content.contains("every managed consumer changes\n  in the same milestone"));
    assert!(content.contains("“additive” is not an automatic compatibility pass"));
    assert!(content.contains("No additional project-specific dependency direction is declared"));
    assert!(
        content.contains("Ownership overlap is blocking until the Contracts identify one owner")
    );
    assert!(content.contains("A dependency cycle is not automatically blocking"));
    assert!(!content.contains("State how strictly this project treats seam changes"));
    assert!(!content.contains("Say which direction dependencies may run"));
    assert!(!content.contains("Name here which of them this project treats as blocking"));
}
