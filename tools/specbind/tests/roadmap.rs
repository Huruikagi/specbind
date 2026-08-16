use specbind::{fingerprint::Fingerprint, roadmap};

const BASELINE: &str = "0123456789abcdef0123456789abcdef01234567";
const MILESTONE: &str = "0198b2d1-7c4a-7e31-9f42-8e7c3a110d62";

fn roadmap(work_items: &str) -> String {
    format!(
        "---\ntype: SpecBind Roadmap\nmilestone_id: {MILESTONE}\nbaseline_revision: {BASELINE}\ntarget_release: null\nwork_items:\n{work_items}---\n# Roadmap\n"
    )
}

#[test]
fn parses_and_normalizes_the_cross_spec_scope() {
    let first = roadmap(
        "  direct_changes:\n    - id: docs\n      summary: Update docs\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n      depends_on:\n        - direct: docs\n        - spec: account-auth\n  new_specs:\n    - spec: account-auth\n      summary: Add authentication\n",
    );
    let second = roadmap(
        "  new_specs:\n    - spec: account-auth\n      summary: Add authentication\n  spec_updates:\n    - spec: checkout\n      summary: Update checkout\n      depends_on:\n        - spec: account-auth\n        - direct: docs\n  direct_changes:\n    - id: docs\n      summary: Update docs\n      status: completed\n",
    );
    let first = roadmap::parse(&first).expect("valid Roadmap");
    let second = roadmap::parse(&second).expect("valid Roadmap");

    assert_eq!(first.spec_ids(), ["account-auth", "checkout"]);
    assert_eq!(first.cross_spec_scope(), second.cross_spec_scope());
    assert_eq!(
        Fingerprint::roadmap_cross_spec_scope(&first).expect("fingerprint"),
        Fingerprint::roadmap_cross_spec_scope(&second).expect("fingerprint")
    );
    assert_eq!(
        first.cross_spec_scope().work_items.spec_updates[0].depends_on[0].spec,
        "account-auth"
    );
}

#[test]
fn rejects_invalid_identity_dependencies_and_cycles() {
    let input = roadmap(
        "  spec_updates:\n    - spec: Bad_ID\n      summary: ''\n    - spec: checkout\n      summary: Checkout\n      depends_on:\n        - spec: missing\n        - spec: missing\n    - spec: cycle-a\n      summary: A\n      depends_on:\n        - spec: cycle-b\n    - spec: cycle-b\n      summary: B\n      depends_on:\n        - spec: cycle-a\n",
    );
    let error = roadmap::parse(&input).expect_err("invalid Roadmap");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert!(codes.contains(&"ROADMAP_ITEM_ID_INVALID"));
    assert!(codes.contains(&"ROADMAP_ITEM_SUMMARY_INVALID"));
    assert!(codes.contains(&"ROADMAP_DEPENDENCY_MISSING"));
    assert!(codes.contains(&"ROADMAP_DEPENDENCY_DUPLICATE"));
    assert!(codes.contains(&"ROADMAP_DEPENDENCY_CYCLE"));
}

#[test]
fn rejects_direct_only_empty_and_invalid_root_metadata() {
    let input = "---\ntype: Wrong\nmilestone_id: not-a-uuid\nbaseline_revision: HEAD\ntarget_release: 3\nwork_items: {}\n---\n";
    let error = roadmap::parse(input).expect_err("invalid metadata");
    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();

    assert!(codes.contains(&"ROADMAP_TYPE_INVALID"));
    assert!(codes.contains(&"ROADMAP_MILESTONE_ID_INVALID"));
    assert!(codes.contains(&"ROADMAP_BASELINE_REVISION_INVALID"));
    assert!(codes.contains(&"ROADMAP_TARGET_RELEASE_INVALID"));
    assert!(codes.contains(&"ROADMAP_WORK_ITEMS_EMPTY"));
}
