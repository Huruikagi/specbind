use specbind::{
    schema::runtime::{self, LoadError},
    yaml,
};

#[test]
fn loads_spec_v1_into_its_wire_model() {
    let document = runtime::load_spec("schema_version: 1\nactive_change: null\n")
        .expect("minimal spec should load");

    assert!(document.active_change.0.is_none());
}

#[test]
fn loads_tasks_v1_into_its_wire_model() {
    let document = runtime::load_tasks(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Test\n      requirement_ids: ['1.1']\n",
    )
    .expect("minimal tasks should load");

    assert_eq!(document.plan.items.len(), 1);
}

#[test]
fn loads_contract_v1_into_its_wire_model() {
    let document = runtime::load_contract(
        "schema_version: 1\nowns: []\nexports: []\nconsumes: []\ninvariants: []\nfile_ownership: []\n",
    )
    .expect("minimal Contract should load");
    assert!(document.exports.is_empty());
}

#[test]
fn reports_yaml_layer_errors() {
    let error = runtime::load_spec("schema_version: 1\nschema_version: 1\nactive_change: null\n")
        .expect_err("duplicate keys must fail in the YAML layer");

    assert!(matches!(
        error,
        LoadError::Yaml(yaml::ParseError::DuplicateKey { .. })
    ));
}

#[test]
fn reports_non_mapping_roots_before_schema_validation() {
    let error = runtime::load_spec("- schema_version\n- 1\n")
        .expect_err("sequence roots cannot select a schema");

    assert!(matches!(error, LoadError::RootNotMapping));
}

#[test]
fn reports_missing_schema_versions() {
    let error = runtime::load_spec("active_change: null\n")
        .expect_err("schema_version is required for schema selection");

    assert!(matches!(error, LoadError::MissingSchemaVersion));
}

#[test]
fn reports_invalid_schema_version_types() {
    let error = runtime::load_spec("schema_version: '1'\nactive_change: null\n")
        .expect_err("schema_version must be numeric");

    assert!(matches!(
        error,
        LoadError::InvalidSchemaVersion { found: "string" }
    ));
}

#[test]
fn reports_unsupported_schema_versions() {
    let error = runtime::load_spec("schema_version: 2\nactive_change: null\n")
        .expect_err("unsupported versions must fail before schema validation");

    assert!(matches!(
        error,
        LoadError::UnsupportedSchemaVersion { version: 2 }
    ));
}

#[test]
fn reports_owned_schema_diagnostics() {
    let error = runtime::load_spec("schema_version: 1\nlanguage: en\nactive_change: null\n")
        .expect_err("unknown fields must fail schema validation");

    let LoadError::Schema { issues } = error else {
        panic!("expected schema diagnostics");
    };
    assert!(!issues.is_empty());
    assert!(
        issues
            .iter()
            .any(|issue| issue.message.contains("language"))
    );
}

#[test]
fn validates_date_time_formats() {
    let error = runtime::load_spec(
        "schema_version: 1\nactive_change:\n  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62\n  state: design\n  requirement_ids: [REQ-1]\n  gate_evidence:\n    requirements:\n      passed_at: not-a-dateZ\n      approval_mode: explicit\n      approved_requirement_ids: [REQ-1]\n      input_revisions:\n        requirements: sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n",
    )
    .expect_err("invalid RFC 3339 timestamps must fail schema validation");

    assert!(matches!(error, LoadError::Schema { .. }));
}

#[test]
fn applies_the_artifact_specific_schema() {
    let error = runtime::load_spec(
        "schema_version: 1\nplan:\n  items:\n    - id: '1'\n      kind: task\n      title: Test\n      requirement_ids: ['1.1']\n",
    )
    .expect_err("tasks content must not load as spec metadata");

    assert!(matches!(error, LoadError::Schema { .. }));
}
