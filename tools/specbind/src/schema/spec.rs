pub mod v1 {
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
    use serde::{Deserialize, Serialize};

    use super::super::deserialize_optional_non_null;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    #[schemars(
        title = "SpecBind spec metadata v1",
        description = "Runtime contract for the accepted strict spec.yaml root, active change, and gate evidence."
    )]
    pub struct SpecDocument {
        pub schema_version: SchemaVersion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub establishment: Option<Establishment>,
        pub active_change: Nullable<ActiveChange>,
    }

    /// Immutable provenance for a Spec first established from existing code.
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct Establishment {
        pub kind: EstablishmentKind,
        pub source_revision: ImplementationRevision,
        pub baseline_version: NonEmptyString,
        pub milestone_id: MilestoneId,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum EstablishmentKind {
        Reverse,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct SchemaVersion(#[schemars(extend("const" = 1))] pub u8);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Nullable<T>(pub Option<T>);

    impl<T: JsonSchema> JsonSchema for Nullable<T> {
        fn inline_schema() -> bool {
            true
        }

        fn schema_name() -> Cow<'static, str> {
            format!("RequiredNullable_{}", T::schema_name()).into()
        }

        fn schema_id() -> Cow<'static, str> {
            format!("{}::Nullable<{}>", module_path!(), T::schema_id()).into()
        }

        fn json_schema(generator: &mut SchemaGenerator) -> Schema {
            json_schema!({
                "anyOf": [
                    generator.subschema_for::<T>(),
                    { "type": "null" }
                ]
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct ActiveChange {
        pub milestone_id: MilestoneId,
        pub state: WorkflowState,
        pub requirement_ids: Nullable<RequirementIdList>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "GateEvidence")]
        pub gate_evidence: Option<GateEvidence>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct MilestoneId(
        #[schemars(regex(
            pattern = "^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$"
        ))]
        pub String,
    );

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum WorkflowState {
        Requirements,
        Design,
        Tasks,
        AdoptionReady,
        Implementation,
        ReleaseReady,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct RequirementIdList(
        #[schemars(length(min = 1), inner(length(min = 1)), extend("uniqueItems" = true))]
        pub  Vec<String>,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    #[schemars(extend("minProperties" = 1))]
    pub struct GateEvidence {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "RequirementsGateEvidence")]
        pub requirements: Option<RequirementsGateEvidence>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "DesignGateEvidence")]
        pub design: Option<DesignGateEvidence>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "TasksGateEvidence")]
        pub tasks: Option<TasksGateEvidence>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "CompletionGateEvidence")]
        pub completion: Option<CompletionGateEvidence>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(untagged)]
    pub enum RequirementsGateEvidence {
        Explicit(ExplicitRequirementsGateEvidence),
        Delegated(DelegatedRequirementsGateEvidence),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct ExplicitRequirementsGateEvidence {
        pub passed_at: PassedAt,
        pub approval_mode: ExplicitApprovalMode,
        pub approved_requirement_ids: RequirementIdList,
        pub input_revisions: RequirementsInputRevisions,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct DelegatedRequirementsGateEvidence {
        pub passed_at: PassedAt,
        pub approval_mode: DelegatedApprovalMode,
        pub delegation_workflow: NonEmptyString,
        pub approved_requirement_ids: RequirementIdList,
        pub input_revisions: RequirementsInputRevisions,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(untagged)]
    pub enum DesignGateEvidence {
        Explicit(ExplicitDesignGateEvidence),
        Delegated(DelegatedDesignGateEvidence),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct ExplicitDesignGateEvidence {
        pub passed_at: PassedAt,
        pub approval_mode: ExplicitApprovalMode,
        pub input_revisions: DesignInputRevisions,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct DelegatedDesignGateEvidence {
        pub passed_at: PassedAt,
        pub approval_mode: DelegatedApprovalMode,
        pub delegation_workflow: NonEmptyString,
        pub input_revisions: DesignInputRevisions,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(untagged)]
    pub enum TasksGateEvidence {
        Explicit(ExplicitTasksGateEvidence),
        Delegated(DelegatedTasksGateEvidence),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct ExplicitTasksGateEvidence {
        pub passed_at: PassedAt,
        pub approval_mode: ExplicitApprovalMode,
        pub input_revisions: TasksInputRevisions,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct DelegatedTasksGateEvidence {
        pub passed_at: PassedAt,
        pub approval_mode: DelegatedApprovalMode,
        pub delegation_workflow: NonEmptyString,
        pub input_revisions: TasksInputRevisions,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub enum ExplicitApprovalMode {
        #[serde(rename = "explicit")]
        Explicit,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub enum DelegatedApprovalMode {
        #[serde(rename = "delegated")]
        Delegated,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct PassedAt(
        #[schemars(
            regex(pattern = "(?:[Zz]|[+-][0-9]{2}:[0-9]{2})$"),
            extend("format" = "date-time")
        )]
        pub String,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct Fingerprint(#[schemars(regex(pattern = "^sha256:[0-9a-f]{64}$"))] pub String);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct ImplementationRevision(
        #[schemars(regex(pattern = "^(?:[0-9a-f]{40}|[0-9a-f]{64})$"))] pub String,
    );

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct NonEmptyString(#[schemars(length(min = 1))] pub String);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct RequirementsInputRevisions {
        #[serde(rename = "requirements")]
        pub requirements: Fingerprint,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct DesignInputRevisions(pub BTreeMap<String, Fingerprint>);

    impl JsonSchema for DesignInputRevisions {
        fn inline_schema() -> bool {
            true
        }

        fn schema_name() -> Cow<'static, str> {
            "DesignInputRevisions".into()
        }

        fn json_schema(generator: &mut SchemaGenerator) -> Schema {
            json_schema!({
                "type": "object",
                "properties": {
                    "contract": generator.subschema_for::<Fingerprint>()
                },
                "patternProperties": {
                    "^design/[a-z][a-z0-9]*(?:-[a-z0-9]+)*$": generator.subschema_for::<Fingerprint>()
                },
                "required": ["contract"],
                "additionalProperties": false,
                "minProperties": 2
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct TasksInputRevisions {
        #[serde(rename = "tasks.yaml#plan")]
        pub plan: Fingerprint,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct CompletionGateEvidence {
        pub passed_at: PassedAt,
        pub implementation_revision: ImplementationRevision,
        #[schemars(length(min = 1))]
        pub mechanical_checks: Vec<MechanicalCheck>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct MechanicalCheck {
        pub kind: MechanicalCheckKind,
        pub command: NonEmptyString,
        pub exit_code: SuccessfulExitCode,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_non_null"
        )]
        #[schemars(with = "NonEmptyString")]
        pub working_directory: Option<NonEmptyString>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum MechanicalCheckKind {
        Test,
        Build,
        Smoke,
        Lint,
        Typecheck,
        Custom,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct SuccessfulExitCode(#[schemars(extend("const" = 0))] pub u8);
}
