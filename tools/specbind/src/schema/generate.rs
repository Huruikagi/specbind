use schemars::{JsonSchema, Schema, generate::SchemaSettings};

use super::{contract, scope, spec, tasks};

#[must_use]
pub fn contract_v1() -> Schema {
    generate::<contract::v1::ContractDocument>()
}

#[must_use]
pub fn spec_v1() -> Schema {
    generate::<spec::v1::SpecDocument>()
}

#[must_use]
pub fn scope_v1() -> Schema {
    generate::<scope::v1::ScopeDocument>()
}

#[must_use]
pub fn tasks_v1() -> Schema {
    generate::<tasks::v1::TasksDocument>()
}

fn generate<T: JsonSchema>() -> Schema {
    SchemaSettings::draft2020_12()
        .into_generator()
        .into_root_schema_for::<T>()
}

/// Serializes a generated schema as deterministic pretty-printed JSON.
///
/// # Errors
///
/// Returns an error if the schema cannot be serialized as JSON.
pub fn to_pretty_json(schema: &Schema) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(schema).map(|json| format!("{json}\n"))
}
