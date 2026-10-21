//! Transient observation, never a mutation or ownership authorization.
pub mod v1 {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct WorktreeSnapshot {
        #[schemars(extend("const" = 1))]
        pub schema_version: u64,
        #[schemars(length(min = 1))]
        pub repository_root: String,
        #[schemars(length(min = 1))]
        pub git_directory: String,
        #[schemars(regex(pattern = "^(?:[0-9a-f]{40}|[0-9a-f]{64})$"))]
        pub head_revision: String,
        pub paths: Vec<PathSnapshot>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    pub struct PathSnapshot {
        #[schemars(length(min = 1))]
        pub path: String,
        pub index: Option<IndexEntry>,
        pub worktree: FileState,
        pub dirty: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct IndexEntry {
        #[schemars(regex(pattern = "^100(?:644|755)$"))]
        pub mode: String,
        #[schemars(regex(pattern = "^(?:[0-9a-f]{40}|[0-9a-f]{64})$"))]
        pub oid: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
    pub enum FileState {
        Missing,
        Directory,
        File {
            #[schemars(regex(pattern = "^sha256:[0-9a-f]{64}$"))]
            digest: String,
            executable: bool,
        },
    }
}
