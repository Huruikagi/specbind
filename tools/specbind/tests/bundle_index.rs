use specbind::{bundle_index, config::ProjectLanguage};

#[test]
fn creates_a_localized_okf_root_index() {
    let applied = bundle_index::apply(None, ProjectLanguage::Ja).expect("missing index is created");
    assert!(!applied.had_block);
    assert!(
        applied
            .content
            .starts_with("---\nokf_version: \"0.2\"\n---\n\n")
    );
    assert!(applied.content.contains("このディレクトリは"));
    assert!(applied.content.contains("<!-- specbind:index -->"));
    assert!(applied.content.ends_with("<!-- /specbind:index -->\n"));
}

#[test]
fn adds_the_declaration_and_block_without_removing_project_content() {
    let current = "# Project knowledge\n\nKeep this.\n";
    let applied = bundle_index::apply(Some(current), ProjectLanguage::En).expect("additive update");
    assert!(!applied.had_block);
    assert!(
        applied
            .content
            .starts_with("---\nokf_version: \"0.2\"\n---\n\n")
    );
    assert!(applied.content.contains(current));
    assert!(applied.content.ends_with("<!-- /specbind:index -->\n"));
}

#[test]
fn adds_only_a_missing_version_to_existing_frontmatter() {
    let current = "---\nproject_extension: retained\n---\n# Project\n";
    let applied = bundle_index::apply(Some(current), ProjectLanguage::En).expect("version added");
    assert!(
        applied.content.starts_with(
            "---\nokf_version: \"0.2\"\nproject_extension: retained\n---\n# Project\n"
        )
    );
}

#[test]
fn replaces_only_the_product_block() {
    let current = concat!(
        "---\nokf_version: \"0.2\"\n---\n\n# Project\n\n",
        "<!-- specbind:index -->\nstale\n<!-- /specbind:index -->\n\n",
        "User notes.\n",
    );
    let applied = bundle_index::apply(Some(current), ProjectLanguage::En).expect("replace block");
    assert!(applied.had_block);
    assert!(!applied.content.contains("stale"));
    assert!(applied.content.ends_with("\n\nUser notes.\n"));
    assert!(applied.content.contains("stable entry points"));
}

#[test]
fn rejects_an_incompatible_version() {
    let error = bundle_index::apply(
        Some("---\nokf_version: \"0.3\"\n---\n# Project\n"),
        ProjectLanguage::En,
    )
    .expect_err("another OKF target must not be overwritten");
    assert_eq!(error.code, "BUNDLE_INDEX_VERSION_INCOMPATIBLE");
}

#[test]
fn rejects_invalid_frontmatter_and_ambiguous_markers() {
    let invalid = bundle_index::apply(Some("---\n[invalid\n---\n"), ProjectLanguage::En)
        .expect_err("invalid YAML stops");
    assert_eq!(invalid.code, "BUNDLE_INDEX_FRONTMATTER_INVALID");

    let unpaired = bundle_index::apply(
        Some("---\nokf_version: \"0.2\"\n---\n<!-- specbind:index -->\n"),
        ProjectLanguage::En,
    )
    .expect_err("unpaired marker stops");
    assert_eq!(unpaired.code, "BUNDLE_INDEX_MARKERS_INVALID");
}
