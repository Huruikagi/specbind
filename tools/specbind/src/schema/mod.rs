use serde::{Deserialize, Deserializer};

pub mod generate;
pub mod runtime;
pub mod spec;
pub mod tasks;

pub const SPEC_V1_SCHEMA_JSON: &str = include_str!("../../schemas/spec/v1.schema.json");
pub const TASKS_V1_SCHEMA_JSON: &str = include_str!("../../schemas/tasks/v1.schema.json");

pub(crate) fn deserialize_optional_non_null<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
