use specbind::rule;

/// The complete Decision 0093 installed default set.
const ACCEPTED_RULES: [&str; 5] = [
    "ears-format.md",
    "design-principles.md",
    "contract-principles.md",
    "tasks-generation.md",
    "steering-principles.md",
];

const ACCEPTED_SELECTORS: [&str; 5] = [
    "ears-format",
    "design-principles",
    "contract-principles",
    "tasks-generation",
    "steering-principles",
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
