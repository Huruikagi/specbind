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
    assert_eq!(first.target_release, None);
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

#[test]
fn rejects_a_nonportable_bound_release() {
    let input = roadmap("  direct_changes:\n    - id: docs\n      summary: Update docs\n")
        .replace("target_release: null", "target_release: bad/version");
    let error = roadmap::parse(&input).expect_err("nonportable release must fail");
    assert!(
        error
            .issues
            .iter()
            .any(|issue| issue.code == "ROADMAP_TARGET_RELEASE_INVALID")
    );
}

#[test]
fn marks_one_direct_item_completed_and_preserves_the_body() {
    let input = roadmap(
        "  direct_changes:\n    - id: docs\n      summary: Update docs\n    - id: release-notes\n      summary: Write notes\n      depends_on:\n        - direct: docs\n",
    );
    let updated = roadmap::complete_direct(&input, "docs").expect("complete Direct item");
    let roadmap::DirectCompletionEdit::Updated(updated) = updated else {
        panic!("pending item should be updated");
    };

    assert!(updated.ends_with("# Roadmap\n"));
    let parsed = roadmap::parse(&updated).expect("mutated Roadmap remains valid");
    assert_eq!(
        parsed.direct_changes[0].status,
        Some(roadmap::DirectStatus::Completed)
    );
    assert_eq!(
        roadmap::complete_direct(&updated, "docs").expect("idempotent completion"),
        roadmap::DirectCompletionEdit::NoChange
    );
}

#[test]
fn binds_and_explicitly_rebinds_a_release_while_preserving_the_body() {
    let input = roadmap("  direct_changes:\n    - id: docs\n      summary: Update docs\n");
    let updated = roadmap::bind_release(&input, "v1.4.0", false).expect("bind release");
    let roadmap::ReleaseBindingEdit::Updated(updated) = updated else {
        panic!("unbound Roadmap should be updated");
    };
    assert!(updated.ends_with("# Roadmap\n"));
    assert_eq!(
        roadmap::parse(&updated)
            .expect("valid bound Roadmap")
            .target_release
            .as_deref(),
        Some("v1.4.0")
    );
    assert_eq!(
        roadmap::bind_release(&updated, "v1.4.0", false).expect("idempotent binding"),
        roadmap::ReleaseBindingEdit::NoChange
    );
    assert_eq!(
        roadmap::bind_release(&updated, "v1.5.0", false).expect("guarded rebinding"),
        roadmap::ReleaseBindingEdit::RebindRequired {
            current: "v1.4.0".to_owned()
        }
    );
    let rebound = roadmap::bind_release(&updated, "v1.5.0", true).expect("explicit rebind");
    let roadmap::ReleaseBindingEdit::Updated(rebound) = rebound else {
        panic!("explicit rebind should update");
    };
    assert_eq!(
        roadmap::parse(&rebound)
            .expect("valid rebound Roadmap")
            .target_release
            .as_deref(),
        Some("v1.5.0")
    );
}
