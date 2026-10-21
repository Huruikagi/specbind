//! Transient input for an explicit set of completion acceptances.

pub mod v1 {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use crate::schema::spec::v1::MechanicalCheckKind;

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct CompletionBatch {
        #[schemars(extend("const" = 1))]
        pub schema_version: u64,
        #[schemars(regex(pattern = "^([0-9a-f]{40}|[0-9a-f]{64})$"))]
        pub implementation_revision: String,
        #[schemars(length(min = 1))]
        pub targets: Vec<Target>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct Target {
        #[schemars(regex(pattern = "^[a-z][a-z0-9]*(-[a-z0-9]+)*$"), length(max = 64))]
        pub spec: String,
        #[schemars(length(min = 1))]
        pub mechanical_checks: Vec<Check>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct Check {
        pub kind: MechanicalCheckKind,
        #[schemars(regex(pattern = "\\S"))]
        pub command: String,
        #[schemars(extend("const" = 0))]
        pub exit_code: u8,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub working_directory: Option<String>,
    }
}
