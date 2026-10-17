pub mod v1 {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    /// Project-owned Drive defaults inside `.specbind.json` schemaVersion 1.
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, JsonSchema)]
    #[serde(deny_unknown_fields)]
    #[schemars(title = "SpecBind Drive defaults v1")]
    pub struct DriveDefaults {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[schemars(with = "bool")]
        pub replan: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[schemars(with = "RecoveryMode")]
        pub recovery: Option<RecoveryMode>,
    }

    impl<'de> Deserialize<'de> for DriveDefaults {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            // A derived struct also accepts sequences; root JSON configuration
            // must match the generated object-only schema, including empty input.
            let mut fields =
                serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;
            let replan = fields
                .remove("replan")
                .map(serde_json::from_value)
                .transpose()
                .map_err(serde::de::Error::custom)?;
            let recovery = fields
                .remove("recovery")
                .map(serde_json::from_value)
                .transpose()
                .map_err(serde::de::Error::custom)?;
            if let Some(key) = fields.keys().next() {
                return Err(serde::de::Error::unknown_field(
                    key,
                    &["replan", "recovery"],
                ));
            }
            Ok(Self { replan, recovery })
        }
    }

    impl DriveDefaults {
        #[must_use]
        pub fn initial() -> Self {
            Self {
                replan: Some(true),
                recovery: Some(RecoveryMode::Progress),
            }
        }

        #[must_use]
        pub fn replan_enabled(self) -> bool {
            self.replan.unwrap_or(false)
        }

        #[must_use]
        pub fn recovery_mode(self) -> RecoveryMode {
            self.recovery.unwrap_or_default()
        }
    }

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "lowercase")]
    pub enum RecoveryMode {
        #[default]
        Bounded,
        Progress,
    }

    impl RecoveryMode {
        #[must_use]
        pub fn name(self) -> &'static str {
            match self {
                Self::Bounded => "bounded",
                Self::Progress => "progress",
            }
        }
    }
}
