use std::sync::LazyLock;

use jsonschema::Validator;
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

use super::{
    CONTRACT_V1_SCHEMA_JSON, SPEC_V1_SCHEMA_JSON, TASKS_V1_SCHEMA_JSON, contract, spec, tasks,
};
use crate::yaml;

static SPEC_V1_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| compile_embedded_schema(SPEC_V1_SCHEMA_JSON));
static TASKS_V1_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| compile_embedded_schema(TASKS_V1_SCHEMA_JSON));
static CONTRACT_V1_VALIDATOR: LazyLock<Validator> =
    LazyLock::new(|| compile_embedded_schema(CONTRACT_V1_SCHEMA_JSON));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaIssue {
    pub instance_path: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error(transparent)]
    Yaml(#[from] yaml::ParseError),
    #[error("structured artifact root must be a YAML mapping")]
    RootNotMapping,
    #[error("structured artifact is missing schema_version")]
    MissingSchemaVersion,
    #[error("schema_version must be a non-negative integer, found {found}")]
    InvalidSchemaVersion { found: &'static str },
    #[error("unsupported schema_version: {version}")]
    UnsupportedSchemaVersion { version: u64 },
    #[error("structured artifact failed JSON Schema validation")]
    Schema { issues: Vec<SchemaIssue> },
    #[error("schema-valid structured artifact failed wire deserialization: {message}")]
    Wire { message: String },
}

/// Loads a restricted `spec.yaml` document through parser, schema, and wire layers.
///
/// # Errors
///
/// Returns [`LoadError`] at the first invalid validation layer.
pub fn load_spec(input: &str) -> Result<spec::v1::SpecDocument, LoadError> {
    let value = yaml::parse(input)?;
    match schema_version(&value)? {
        1 => validate_and_deserialize(value, &SPEC_V1_VALIDATOR),
        version => Err(LoadError::UnsupportedSchemaVersion { version }),
    }
}

/// Loads a restricted `tasks.yaml` document through parser, schema, and wire layers.
///
/// # Errors
///
/// Returns [`LoadError`] at the first invalid validation layer.
pub fn load_tasks(input: &str) -> Result<tasks::v1::TasksDocument, LoadError> {
    let value = yaml::parse(input)?;
    match schema_version(&value)? {
        1 => validate_and_deserialize(value, &TASKS_V1_VALIDATOR),
        version => Err(LoadError::UnsupportedSchemaVersion { version }),
    }
}

/// Loads a restricted `contract.yaml` document through parser, schema, and wire layers.
///
/// # Errors
///
/// Returns [`LoadError`] at the first invalid validation layer.
pub fn load_contract(input: &str) -> Result<contract::v1::ContractDocument, LoadError> {
    let value = yaml::parse(input)?;
    match schema_version(&value)? {
        1 => validate_and_deserialize(value, &CONTRACT_V1_VALIDATOR),
        version => Err(LoadError::UnsupportedSchemaVersion { version }),
    }
}

fn compile_embedded_schema(schema_json: &str) -> Validator {
    let schema: Value =
        serde_json::from_str(schema_json).expect("embedded schema must be valid JSON");
    jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(&schema)
        .expect("embedded schema must compile as Draft 2020-12")
}

fn schema_version(value: &Value) -> Result<u64, LoadError> {
    let root = value.as_object().ok_or(LoadError::RootNotMapping)?;
    let version = root
        .get("schema_version")
        .ok_or(LoadError::MissingSchemaVersion)?;
    version
        .as_u64()
        .ok_or_else(|| LoadError::InvalidSchemaVersion {
            found: value_kind(version),
        })
}

fn validate_and_deserialize<T: DeserializeOwned>(
    value: Value,
    validator: &Validator,
) -> Result<T, LoadError> {
    let mut issues = validator
        .iter_errors(&value)
        .map(|error| SchemaIssue {
            instance_path: error.instance_path().to_string(),
            message: error.to_string(),
        })
        .collect::<Vec<_>>();

    if !issues.is_empty() {
        issues.sort_by(|left, right| {
            left.instance_path
                .cmp(&right.instance_path)
                .then_with(|| left.message.cmp(&right.message))
        });
        return Err(LoadError::Schema { issues });
    }

    serde_json::from_value(value).map_err(|error| LoadError::Wire {
        message: error.to_string(),
    })
}

fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "mapping",
    }
}
