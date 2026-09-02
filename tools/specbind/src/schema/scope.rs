//! Transient milestone scope candidate.
//!
//! Unlike `spec.yaml` and `tasks.yaml` this document is never persisted:
//! Decision 0089 makes it command input that `SpecBind` reads once and discards.
//! It lives here anyway because Decision 0103 exposes its schema, and a shape an
//! agent must author is a wire model whether or not the bytes survive the call.

pub mod v1 {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use crate::roadmap::Dependency;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    #[schemars(
        title = "SpecBind milestone scope candidate v1",
        description = "Transient input for milestone create and update-scope. Identity, baseline, release binding, and Direct completion status are never accepted from the caller."
    )]
    pub struct ScopeDocument {
        #[schemars(extend("const" = 1))]
        pub schema_version: u64,
        pub work_items: WorkItemsDocument,
        /// Existing product version represented by a reverse-only baseline.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub baseline_version: Option<String>,
        /// Free-form Roadmap Markdown body. Omitting it preserves the current
        /// prose on an update and writes a minimal body on creation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub body: Option<String>,
    }

    /// At least one category must be present, and a category appears only when
    /// it has items.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct WorkItemsDocument {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub new_specs: Option<Vec<SpecItemDocument>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub spec_updates: Option<Vec<SpecItemDocument>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reverse_specs: Option<Vec<SpecItemDocument>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub direct_changes: Option<Vec<DirectItemDocument>>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct SpecItemDocument {
        /// Canonical lowercase kebab-case Spec identity.
        pub spec: String,
        pub summary: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub depends_on: Vec<Dependency>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct DirectItemDocument {
        /// Canonical lowercase kebab-case Direct identity.
        pub id: String,
        pub summary: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub depends_on: Vec<Dependency>,
    }
}
