use specbind::rule;

/// The complete Decision 0093 installed default set.
const ACCEPTED_RULES: [&str; 5] = [
    "ears-format.md",
    "design-principles.md",
    "contract-principles.md",
    "tasks-generation.md",
    "steering-principles.md",
];

#[test]
fn embeds_exactly_the_accepted_default_rule_set() {
    let names = rule::defaults()
        .iter()
        .map(|entry| entry.file_name)
        .collect::<Vec<_>>();

    assert_eq!(names, ACCEPTED_RULES);
}

#[test]
fn every_default_rule_is_an_okf_rule_concept() {
    for entry in rule::defaults() {
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
