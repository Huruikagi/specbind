use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;
use specbind::schema::{SPEC_V1_SCHEMA_JSON, TASKS_V1_SCHEMA_JSON, generate, spec, tasks};

#[test]
fn checked_in_schemas_are_current_and_valid_draft_2020_12() {
    let cases = [
        ("spec/v1", SPEC_V1_SCHEMA_JSON, generate::spec_v1()),
        ("tasks/v1", TASKS_V1_SCHEMA_JSON, generate::tasks_v1()),
    ];

    for (name, checked_in, generated) in cases {
        let expected = generate::to_pretty_json(&generated).expect("schema should serialize");
        assert_eq!(checked_in, expected, "checked-in {name} schema is stale");

        let value: serde_json::Value =
            serde_json::from_str(checked_in).expect("checked-in schema should be JSON");
        jsonschema::draft202012::meta::validate(&value)
            .unwrap_or_else(|error| panic!("{name} schema is not valid Draft 2020-12: {error}"));
    }
}

#[test]
fn spec_v1_fixtures_conform() {
    assert_fixture_set::<spec::v1::SpecDocument>("spec/v1", SPEC_V1_SCHEMA_JSON);
}

#[test]
fn tasks_v1_fixtures_conform() {
    assert_fixture_set::<tasks::v1::TasksDocument>("tasks/v1", TASKS_V1_SCHEMA_JSON);
}

fn assert_fixture_set<T: DeserializeOwned>(artifact: &str, schema_json: &str) {
    let schema: serde_json::Value =
        serde_json::from_str(schema_json).expect("embedded schema should be JSON");
    let validator = jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(&schema)
        .expect("embedded schema should compile");
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/schemas")
        .join(artifact);

    for path in fixture_paths(&fixture_root.join("valid")) {
        let value = parse_fixture(&path);
        if let Err(error) = validator.validate(&value) {
            panic!(
                "valid fixture {} failed schema validation: {error}",
                path.display()
            );
        }
        serde_json::from_value::<T>(value).unwrap_or_else(|error| {
            panic!(
                "valid fixture {} failed wire deserialization: {error}",
                path.display()
            )
        });
    }

    for path in fixture_paths(&fixture_root.join("invalid")) {
        let value = parse_fixture(&path);
        assert!(
            !validator.is_valid(&value),
            "invalid fixture {} unexpectedly passed schema validation",
            path.display()
        );
    }
}

fn fixture_paths(directory: &Path) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
        .map(|entry| {
            entry
                .expect("fixture directory entry should be readable")
                .path()
        })
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yaml")
        })
        .collect::<Vec<_>>();
    paths.sort();
    assert!(
        !paths.is_empty(),
        "fixture set {} is empty",
        directory.display()
    );
    paths
}

fn parse_fixture(path: &Path) -> serde_json::Value {
    let yaml = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    serde_saphyr::from_str(&yaml)
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}
