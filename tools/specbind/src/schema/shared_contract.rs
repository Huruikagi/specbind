pub mod v1 {
    use crate::schema::contract::v1::EntryId;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct SharedContractDocument {
        #[schemars(extend("const" = 1))]
        pub schema_version: u8,
        pub resources: Vec<Resource>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    pub struct Resource {
        pub id: EntryId,
        #[schemars(length(min = 1))]
        pub description: String,
        #[schemars(length(min = 1), inner(length(min = 1)), extend("uniqueItems" = true))]
        pub paths: Vec<String>,
        #[schemars(length(min = 1))]
        pub change_policy: String,
        #[schemars(inner(length(min = 1)))]
        pub invariants: Vec<String>,
    }
}
