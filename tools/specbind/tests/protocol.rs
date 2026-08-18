use specbind::protocol;

/// The complete Decision 0094 v1 selector set, in accepted order.
const ACCEPTED_SELECTORS: [&str; 11] = [
    "okf-authoring",
    "requirements-review",
    "design-discovery",
    "design-authoring",
    "design-validation",
    "gap-analysis",
    "task-planning",
    "task-implementation",
    "task-review",
    "debug",
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
