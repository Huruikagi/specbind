use serde::{Deserialize, Deserializer};

pub mod contract;
pub mod generate;
pub mod runtime;
pub mod scope;
pub mod spec;
pub mod tasks;

pub const CONTRACT_V1_SCHEMA_JSON: &str = include_str!("../../schemas/contract/v1.schema.json");
pub const SPEC_V1_SCHEMA_JSON: &str = include_str!("../../schemas/spec/v1.schema.json");
pub const TASKS_V1_SCHEMA_JSON: &str = include_str!("../../schemas/tasks/v1.schema.json");
pub const SCOPE_V1_SCHEMA_JSON: &str = include_str!("../../schemas/scope/v1.schema.json");

/// One embedded structured-artifact schema.
///
/// Decision 0103 exposes these for authoring. The selector carries the wire
/// version, because an unversioned name would silently change meaning when a
/// second version is added, and a caller writing `schema_version: 1` could not
/// then ask for the schema it is actually targeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmbeddedSchema {
    /// Versioned selector, matching the conformance-test naming.
    pub selector: &'static str,
    /// The artifact file this schema governs.
    pub artifact: &'static str,
    /// Who writes that artifact, so a caller knows whether it authors one.
    pub written_by: &'static str,
    content: &'static str,
}

impl EmbeddedSchema {
    /// Returns the schema document as raw JSON.
    #[must_use]
    pub fn content(self) -> &'static str {
        self.content
    }
}

/// Every embedded schema. This is the whole accepted selector set.
static SCHEMAS: &[EmbeddedSchema] = &[
    EmbeddedSchema {
        selector: "contract/v1",
        artifact: "contract.yaml",
        written_by: "the authoring agent",
        content: CONTRACT_V1_SCHEMA_JSON,
    },
    EmbeddedSchema {
        selector: "spec/v1",
        artifact: "spec.yaml",
        written_by: "guarded CLI operations only",
        content: SPEC_V1_SCHEMA_JSON,
    },
    EmbeddedSchema {
        selector: "scope/v1",
        artifact: "milestone scope candidate (transient)",
        written_by: "the authoring agent",
        content: SCOPE_V1_SCHEMA_JSON,
    },
    EmbeddedSchema {
        selector: "tasks/v1",
        artifact: "tasks.yaml",
        written_by: "the authoring agent",
        content: TASKS_V1_SCHEMA_JSON,
    },
];

/// Lists every embedded schema.
#[must_use]
pub fn schemas() -> &'static [EmbeddedSchema] {
    SCHEMAS
}

/// Resolves one schema by its exact versioned selector.
#[must_use]
pub fn find_schema(selector: &str) -> Option<EmbeddedSchema> {
    SCHEMAS
        .iter()
        .copied()
        .find(|entry| entry.selector == selector)
}

pub(crate) fn deserialize_optional_non_null<'de, D, T>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
