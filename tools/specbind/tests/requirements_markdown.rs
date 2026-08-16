use specbind::requirements;

#[test]
fn derives_canonical_ids_from_english_and_japanese_documents() {
    let english = requirements::parse(
        "# Requirements\n\n### Requirement 3: Account **Locking**\n\n#### Acceptance Criteria\n\n1. Lock after failures.\n1. Unlock after delay.\n\n### Requirement 1: Sign in\n\n#### Acceptance Criteria\n\n1. Accept valid credentials.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect("valid English requirements");
    assert_eq!(english.requirement_ids(), ["1.1", "3.1", "3.2"]);
    assert_eq!(english.groups[1].title, "Account Locking");

    let japanese = requirements::parse(
        "### 要件 7: アカウントロック\n\n#### 受入条件\n\n1. 失敗時にロックする。\n2. 時間経過後に解除する。\n",
        "要件",
        "受入条件",
    )
    .expect("valid Japanese requirements");
    assert_eq!(japanese.requirement_ids(), ["7.1", "7.2"]);
}

#[test]
fn counts_only_direct_items_in_the_single_acceptance_list() {
    let document = requirements::parse(
        "1. An unrelated list.\n\n```markdown\n1. Not a criterion.\n```\n\n### Requirement 2: Nested details\n\n#### Acceptance Criteria\n\n> 1. Quoted example, not a criterion.\n\n1. First criterion.\n   1. Nested detail.\n2. Second criterion.\n\n##### Example\n\n1. Not a criterion.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect("only the direct Acceptance Criteria items count");

    assert_eq!(document.requirement_ids(), ["2.1", "2.2"]);
    assert_eq!(
        document.groups[0]
            .criteria
            .iter()
            .map(|criterion| criterion.line)
            .collect::<Vec<_>>(),
        [13, 15]
    );
}

#[test]
fn reports_malformed_and_duplicate_requirement_headings_with_lines() {
    let error = requirements::parse(
        "### Requirement 01: Invalid\n\n### Requirement 2: First\n\n#### Acceptance Criteria\n\n1. Criterion.\n\n### Requirement 2: Duplicate\n\n#### Acceptance Criteria\n\n1. Criterion.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect_err("invalid group headings must fail");

    assert!(
        error
            .issues
            .iter()
            .any(|issue| { issue.code == "REQUIREMENTS_HEADING_MALFORMED" && issue.line == 1 })
    );
    assert!(
        error
            .issues
            .iter()
            .any(|issue| { issue.code == "REQUIREMENTS_GROUP_DUPLICATE" && issue.line == 9 })
    );
}

#[test]
fn reports_missing_and_duplicate_acceptance_headings() {
    let missing = requirements::parse(
        "### Requirement 1: Missing\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect_err("missing heading must fail");
    assert_eq!(
        missing.issues[0].code,
        "REQUIREMENTS_ACCEPTANCE_HEADING_MISSING"
    );

    let duplicate = requirements::parse(
        "### Requirement 1: Duplicate\n\n#### Acceptance Criteria\n\n1. One.\n\n#### Acceptance Criteria\n\n1. Two.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect_err("duplicate headings must fail");
    assert!(
        duplicate
            .issues
            .iter()
            .any(|issue| issue.code == "REQUIREMENTS_ACCEPTANCE_HEADING_DUPLICATE")
    );
}

#[test]
fn allows_other_level_four_headings_with_similar_text() {
    let document = requirements::parse(
        "### Requirement 1: Other section\n\n#### Acceptance Criteria notes\n\nThis is ordinary rationale.\n\n#### Acceptance Criteria\n\n1. Criterion.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect("only the exact mapped heading has structural meaning");

    assert_eq!(document.requirement_ids(), ["1.1"]);
}

#[test]
fn enforces_one_nonempty_top_level_ordered_list_starting_at_one() {
    let unordered = requirements::parse(
        "### Requirement 1: Unordered\n\n#### Acceptance Criteria\n\n- Not ordered.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect_err("unordered list must not create IDs");
    assert_eq!(
        unordered.issues[0].code,
        "REQUIREMENTS_ACCEPTANCE_LIST_MISSING"
    );

    let multiple = requirements::parse(
        "### Requirement 1: Multiple\n\n#### Acceptance Criteria\n\n1. One.\n\nIntervening prose.\n\n1. Two.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect_err("separate lists are ambiguous");
    assert_eq!(
        multiple.issues[0].code,
        "REQUIREMENTS_ACCEPTANCE_LIST_MULTIPLE"
    );

    let wrong_start = requirements::parse(
        "### Requirement 1: Wrong start\n\n#### Acceptance Criteria\n\n2. Two.\n3. Three.\n",
        "Requirement",
        "Acceptance Criteria",
    )
    .expect_err("list must begin at one");
    assert_eq!(
        wrong_start.issues[0].code,
        "REQUIREMENTS_ACCEPTANCE_LIST_START"
    );
}
