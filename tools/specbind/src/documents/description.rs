//! Optional durable responsibility metadata shared by selected OKF profiles.

use serde_json::{Map, Value};

/// Missing metadata remains compatible with existing major-one documents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Description {
    Unavailable,
    Missing,
    Invalid,
    Present(String),
}

impl Description {
    #[must_use]
    pub fn from_mapping(mapping: &Map<String, Value>) -> Self {
        match mapping.get("description") {
            None => Self::Missing,
            Some(Value::String(value))
                if !value.trim().is_empty()
                    && !value
                        .chars()
                        .any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}')) =>
            {
                Self::Present(value.clone())
            }
            Some(_) => Self::Invalid,
        }
    }
}

pub const INVALID_MESSAGE: &str =
    "description must be a non-empty single-line string without control characters";
