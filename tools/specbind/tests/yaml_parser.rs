use specbind::yaml::{self, ParseError};

#[test]
fn rejects_duplicate_keys() {
    let error = yaml::parse("schema_version: 1\nschema_version: 1\nactive_change: null\n")
        .expect_err("duplicate keys must fail");

    assert!(
        matches!(error, ParseError::DuplicateKey { .. }),
        "unexpected error: {error:?}"
    );
}

#[test]
fn rejects_anchors() {
    let error = yaml::parse("schema_version: &version 1\nactive_change: null\n")
        .expect_err("anchors must fail");

    assert!(matches!(error, ParseError::Anchor));
}

#[test]
fn rejects_aliases() {
    let error =
        yaml::parse("base: &base {value: 1}\ncopy: *base\n").expect_err("aliases must fail");

    assert!(matches!(error, ParseError::Alias));
}

#[test]
fn rejects_merge_keys() {
    let error = yaml::parse("value:\n  <<: {nested: true}\n").expect_err("merge keys must fail");

    assert!(matches!(error, ParseError::MergeKey));
}

#[test]
fn quoted_merge_text_is_an_ordinary_key() {
    let value = yaml::parse("value:\n  \"<<\": text\n")
        .expect("quoted merge text should not invoke YAML merge semantics");

    assert_eq!(value["value"]["<<"], "text");
}

#[test]
fn rejects_custom_tags() {
    let error = yaml::parse("schema_version: !specbind 1\nactive_change: null\n")
        .expect_err("custom tags must fail");

    assert!(matches!(error, ParseError::CustomTag { .. }));
}

#[test]
fn rejects_multiple_documents() {
    let error = yaml::parse("---\na: 1\n---\na: 2\n").expect_err("multiple documents must fail");

    assert!(matches!(error, ParseError::MultipleDocuments));
}

#[test]
fn rejects_malformed_yaml() {
    let error = yaml::parse("value: [unterminated\n").expect_err("malformed YAML must fail");

    assert!(matches!(error, ParseError::Syntax { .. }));
}

#[test]
fn rejects_values_that_cannot_be_represented_as_json() {
    let error = yaml::parse("? [complex, key]\n: value\n")
        .expect_err("non-string JSON object keys must fail");

    assert!(matches!(error, ParseError::NeutralValue { .. }));
}
