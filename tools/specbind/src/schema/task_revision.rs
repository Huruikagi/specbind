//! Transient guarded Task revision input; the persisted Tasks wire is unchanged.

pub mod v1 {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use crate::schema::tasks::v1::{TaskPlan, TaskReference};

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct TaskRevision {
        #[schemars(extend("const" = 1))]
        pub schema_version: u64,
        #[schemars(regex(pattern = "^sha256:[0-9a-f]{64}$"))]
        pub source_fingerprint: String,
        pub plan: TaskPlan,
        pub progress: Vec<Disposition>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    #[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
    pub enum Disposition {
        Preserve {
            from: TaskReference,
            to: TaskReference,
            #[schemars(regex(pattern = "\\S"))]
            reason: String,
        },
        Reset {
            from: TaskReference,
            to: TaskReference,
            #[schemars(regex(pattern = "\\S"))]
            reason: String,
        },
        Remove {
            from: TaskReference,
            #[schemars(regex(pattern = "\\S"))]
            reason: String,
        },
    }

    impl Disposition {
        #[must_use]
        pub fn source(&self) -> &TaskReference {
            match self {
                Self::Preserve { from, .. }
                | Self::Reset { from, .. }
                | Self::Remove { from, .. } => from,
            }
        }

        #[must_use]
        pub fn target(&self) -> Option<&TaskReference> {
            match self {
                Self::Preserve { to, .. } | Self::Reset { to, .. } => Some(to),
                Self::Remove { .. } => None,
            }
        }
    }
}
