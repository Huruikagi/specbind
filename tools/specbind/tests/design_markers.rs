use specbind::design;

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn unions_exact_underscore_and_star_emphasis_markers() {
    let traceability = design::validate(
        "_Requirements: 2.1, 1.2_\n\nDiscussion.\n\n*Requirements: 10.3, 2.1*\n",
        &ids(&["1.2", "2.1", "10.3"]),
    )
    .expect("equivalent emphasis forms and repeated IDs are valid");

    assert_eq!(traceability.markers.len(), 2);
    assert_eq!(traceability.markers[0].line, 1);
    assert_eq!(traceability.markers[1].line, 5);
    assert_eq!(traceability.requirement_ids, ids(&["1.2", "2.1", "10.3"]));
}

#[test]
fn ignores_noncanonical_and_non_emphasis_references() {
    let error = design::validate(
        "Requirements: 1.1\n\n**Requirements: 1.1**\n\n`Requirements: 1.1`\n\n```markdown\n_Requirements: 1.1_\n```\n\n<!-- _Requirements: 1.1_ -->\n\n_Requirements:_ 1.1\n\n_Requirements: `1.1`_\n\n_[Requirements: 1.1](target)_\n",
        &ids(&["1.1"]),
    )
    .expect_err("only a complete plain-text emphasis node is a marker");

    let codes = error
        .issues
        .iter()
        .map(|issue| issue.code)
        .collect::<Vec<_>>();
    assert!(codes.contains(&"DESIGN_REQUIREMENT_MARKER_MISSING"));
    assert!(codes.contains(&"DESIGN_BODY_REQUIREMENT_ID_MISSING"));
}

#[test]
fn requires_the_exact_ascii_separator_and_canonical_ids() {
    for marker in [
        "_Requirements: 1.1,2.1_",
        "_Requirements: 1.1,  2.1_",
        "_requirements: 1.1_",
        "_Requirements: 01.1_",
        "_Requirements: 1.1, _",
    ] {
        let error = design::validate(marker, &ids(&["1.1"]))
            .expect_err("noncanonical marker text must be ignored");
        assert!(
            error
                .issues
                .iter()
                .any(|issue| issue.code == "DESIGN_REQUIREMENT_MARKER_MISSING"),
            "{marker}"
        );
    }
}

#[test]
fn reports_both_directions_of_set_mismatch_at_stable_lines() {
    let error = design::validate(
        "# Design\n\n_Requirements: 1.1, 3.1_\n",
        &ids(&["1.1", "2.1"]),
    )
    .expect_err("declared and marked sets differ");

    let missing_body = error
        .issues
        .iter()
        .find(|issue| issue.code == "DESIGN_BODY_REQUIREMENT_ID_MISSING")
        .expect("Front Matter-only ID");
    assert_eq!(missing_body.line, 1);
    assert!(missing_body.message.contains("2.1"));

    let missing_frontmatter = error
        .issues
        .iter()
        .find(|issue| issue.code == "DESIGN_FRONTMATTER_REQUIREMENT_ID_MISSING")
        .expect("body-only ID");
    assert_eq!(missing_frontmatter.line, 3);
    assert!(missing_frontmatter.message.contains("3.1"));
}
