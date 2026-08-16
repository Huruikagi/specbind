pub mod v1 {
    use std::collections::BTreeMap;

    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use super::super::deserialize_optional_non_null;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    #[schemars(
        title = "SpecBind task artifact v1",
        description = "Runtime contract for the accepted tasks.yaml v1 plan and sparse execution-state shape."
    )]
    pub struct TasksDocument {
        pub schema_version: SchemaVersion,
        pub plan: TaskPlan,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "TaskExecution")]
        pub execution: Option<TaskExecution>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct SchemaVersion(#[schemars(extend("const" = 1))] pub u8);

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct TaskReference(
        #[schemars(regex(pattern = "^[1-9][0-9]*(?:\\.[1-9][0-9]*)?$"))] pub String,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct TaskPlan {
        #[schemars(length(min = 1))]
        pub items: Vec<PlanItem>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(untagged)]
    pub enum PlanItem {
        Group(TaskGroup),
        Task(ExecutableTask),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct TaskGroup {
        pub id: TaskReference,
        pub kind: GroupKind,
        pub title: NonEmptyString,
        #[schemars(length(min = 2))]
        pub tasks: Vec<ExecutableTask>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub enum GroupKind {
        #[serde(rename = "group")]
        Group,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(untagged)]
    pub enum ExecutableTask {
        Parallel(ParallelTask),
        Sequential(SequentialTask),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct SequentialTask {
        pub id: TaskReference,
        pub kind: TaskKind,
        pub title: NonEmptyString,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "NonEmptyStringList")]
        pub details: Option<NonEmptyStringList>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "NonEmptyStringList")]
        pub completion_criteria: Option<NonEmptyStringList>,
        pub requirement_ids: UniqueNonEmptyStringList,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "UniqueNonEmptyStringList")]
        pub boundaries: Option<UniqueNonEmptyStringList>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "UniqueNonEmptyStringList")]
        pub contracts: Option<UniqueNonEmptyStringList>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "TaskReferenceList")]
        pub depends_on: Option<TaskReferenceList>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct ParallelTask {
        pub id: TaskReference,
        pub kind: TaskKind,
        pub title: NonEmptyString,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "NonEmptyStringList")]
        pub details: Option<NonEmptyStringList>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "NonEmptyStringList")]
        pub completion_criteria: Option<NonEmptyStringList>,
        pub requirement_ids: UniqueNonEmptyStringList,
        pub boundaries: UniqueNonEmptyStringList,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "UniqueNonEmptyStringList")]
        pub contracts: Option<UniqueNonEmptyStringList>,
        pub parallel: ParallelMarker,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "TaskReferenceList")]
        pub depends_on: Option<TaskReferenceList>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub enum TaskKind {
        #[serde(rename = "task")]
        Task,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct ParallelMarker(#[schemars(extend("const" = true))] pub bool);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct NonEmptyString(#[schemars(length(min = 1))] pub String);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct NonEmptyStringList(
        #[schemars(length(min = 1), inner(length(min = 1)))] pub Vec<String>,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct UniqueNonEmptyStringList(
        #[schemars(length(min = 1), inner(length(min = 1)), extend("uniqueItems" = true))]
        pub  Vec<String>,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct TaskReferenceList(
        #[schemars(length(min = 1), extend("uniqueItems" = true))] pub Vec<TaskReference>,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    #[schemars(extend("minProperties" = 1))]
    pub struct TaskExecution {
        pub tasks: TaskExecutionStates,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    #[schemars(extend("minProperties" = 1))]
    pub struct TaskExecutionStates(pub BTreeMap<TaskReference, TaskExecutionState>);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(untagged)]
    pub enum TaskExecutionState {
        Completed(CompletedTaskState),
        Blocked(BlockedTaskState),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct CompletedTaskState {
        pub status: CompletedStatus,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub enum CompletedStatus {
        #[serde(rename = "completed")]
        Completed,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct BlockedTaskState {
        pub status: BlockedStatus,
        pub blocked_reason: NonEmptyString,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub enum BlockedStatus {
        #[serde(rename = "blocked")]
        Blocked,
    }
}
