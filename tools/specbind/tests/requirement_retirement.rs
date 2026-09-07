use specbind::requirements;

#[test]
fn retirement_preserves_positions_and_nested_explanations_are_not_requirements() {
    let doc = requirements::parse("### Requirement 3: Cart\n\n#### Acceptance Criteria\n\n1. Add items.\n2. _Retired_ Limit items.\n   - Remove the limit; retain stock checks.\n3. Show oldest first.\n", "Requirement", "Acceptance Criteria").unwrap();
    assert_eq!(doc.requirement_ids(), ["3.1", "3.2", "3.3"]);
    assert_eq!(doc.live_requirement_ids(), ["3.1", "3.3"]);
    assert!(doc.is_retired("3.2"));
    assert!(!doc.is_retired("3.3"));
}

#[test]
fn retired_groups_work_with_retained_or_omitted_body() {
    for suffix in ["", "\n\n#### 受入条件\n\n1. 元の義務。\n2. 別の義務。\n"] {
        let doc = requirements::parse(
            &format!("### 要件 3: _Retired_ カート\n{suffix}"),
            "要件",
            "受入条件",
        )
        .unwrap();
        assert!(doc.live_requirement_ids().is_empty());
        assert!(doc.is_retired("3.1"));
        assert!(doc.is_retired("3.2"));
        assert!(!doc.is_retired("3.01"));
        assert!(!doc.is_retired("3.foo"));
        assert!(!doc.is_retired("4.1"));
    }
}

#[test]
fn only_exact_leading_markers_retire_obligations() {
    let doc = requirements::parse("### Requirement 1: Mention _Retired_\n\n#### Acceptance Criteria\n\n1. Mention _Retired_.\n2. `_Retired_` is code.\n3. \\_Retired_ is escaped.\n4. _Retired_extra is text.\n5. *Retired* is not the marker.\n6. _Retired_\n", "Requirement", "Acceptance Criteria").unwrap();
    assert_eq!(
        doc.live_requirement_ids(),
        ["1.1", "1.2", "1.3", "1.4", "1.5"]
    );
    assert!(doc.is_retired("1.6"));
}

#[test]
fn code_blocks_and_nested_marker_examples_do_not_retire_the_parent() {
    let doc = requirements::parse("### Requirement 1: Examples\n\n#### Acceptance Criteria\n\n1.     _Retired_\n2. ```text\n   _Retired_\n   ```\n3. Parent.\n   - _Retired_\n4.\n   _Retired_ Old promise.\n", "Requirement", "Acceptance Criteria").unwrap();
    assert_eq!(doc.live_requirement_ids(), ["1.1", "1.2", "1.3"]);
    assert!(doc.is_retired("1.4"));
}

#[test]
fn group_markers_follow_the_mapped_label_and_ignore_later_title_emphasis() {
    let doc = requirements::parse("### Domain: Requirement 3: _Retired_ Old: title\n\n### Domain: Requirement 4: Current: _Retired_ example\n\n#### Acceptance Criteria\n\n1. Live.\n", "Domain: Requirement", "Acceptance Criteria").unwrap();
    assert!(doc.is_retired("3.1"));
    assert!(!doc.is_retired("4.1"));
    assert_eq!(doc.live_requirement_ids(), ["4.1"]);
}
