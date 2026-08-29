pub mod v1 {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    #[schemars(
        title = "SpecBind Contract artifact v1",
        description = "Runtime contract for the strict contract.yaml v1 persistent cross-Spec seam manifest."
    )]
    pub struct ContractDocument {
        pub schema_version: SchemaVersion,
        pub owns: Vec<DescribedEntry>,
        pub exports: Vec<DescribedEntry>,
        pub consumes: Vec<ConsumesEntry>,
        pub invariants: Vec<DescribedEntry>,
        pub file_ownership: Vec<FileOwnershipEntry>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct SchemaVersion(#[schemars(extend("const" = 1))] pub u8);

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
    #[serde(transparent)]
    pub struct EntryId(#[schemars(regex(pattern = "^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$"))] pub String);

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct DescribedEntry {
        pub id: EntryId,
        #[schemars(length(min = 1))]
        pub description: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct ConsumesEntry {
        pub id: EntryId,
        pub target: ContractTarget,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct ContractTarget {
        #[schemars(regex(pattern = "^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$"))]
        pub spec: String,
        pub section: TargetSection,
        pub id: EntryId,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    pub enum TargetSection {
        #[serde(rename = "owns")]
        Owns,
        #[serde(rename = "exports")]
        Exports,
        #[serde(rename = "invariants")]
        Invariants,
        #[serde(rename = "file-ownership")]
        FileOwnership,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct FileOwnershipEntry {
        pub id: EntryId,
        #[schemars(length(min = 1), inner(length(min = 1)), extend("uniqueItems" = true))]
        pub paths: Vec<String>,
    }
}
