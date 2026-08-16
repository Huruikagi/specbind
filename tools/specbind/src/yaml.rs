use std::collections::BTreeSet;

use saphyr_parser::{Event, Parser, ScalarStyle, Tag};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid YAML syntax: {message}")]
    Syntax { message: String },
    #[error("duplicate YAML mapping key: {key}")]
    DuplicateKey { key: String },
    #[error("YAML anchors are not supported")]
    Anchor,
    #[error("YAML aliases are not supported")]
    Alias,
    #[error("YAML merge keys are not supported")]
    MergeKey,
    #[error("custom YAML tag is not supported: {tag}")]
    CustomTag { tag: String },
    #[error("exactly one YAML document is required")]
    MultipleDocuments,
    #[error("YAML cannot be represented as a neutral JSON value: {message}")]
    NeutralValue { message: String },
}

#[derive(Debug)]
enum Container {
    Mapping {
        expecting_key: bool,
        keys: BTreeSet<String>,
    },
    Sequence,
}

/// Parses one restricted YAML document into a neutral JSON value.
///
/// Anchors, aliases, merge keys, custom tags, duplicate mapping keys, and
/// multiple documents are rejected before structured-artifact schema validation.
///
/// # Errors
///
/// Returns [`ParseError`] when the input is malformed, uses a prohibited YAML
/// feature, contains duplicate keys, or cannot be represented as JSON data.
pub fn parse(input: &str) -> Result<Value, ParseError> {
    inspect_restricted_features(input)?;

    serde_saphyr::from_str(input).map_err(|error| match error {
        serde_saphyr::DeserializeError::DuplicateMappingKey { key, .. } => {
            let key = key.unwrap_or_else(|| "<non-scalar key>".to_owned());
            ParseError::DuplicateKey { key }
        }
        error => ParseError::NeutralValue {
            message: error.to_string(),
        },
    })
}

fn inspect_restricted_features(input: &str) -> Result<(), ParseError> {
    let mut containers = Vec::new();
    let mut document_count = 0_u8;
    let mut anchor_found = false;

    for parsed in Parser::new_from_str(input) {
        let (event, _) = parsed.map_err(|error| ParseError::Syntax {
            message: error.to_string(),
        })?;

        match event {
            Event::DocumentStart(_) => {
                document_count = document_count.saturating_add(1);
                if document_count > 1 {
                    return Err(ParseError::MultipleDocuments);
                }
            }
            Event::Scalar(value, style, anchor, tag) => {
                anchor_found |= anchor != 0;
                inspect_tag(tag.as_deref())?;
                if is_mapping_key(&containers) && style == ScalarStyle::Plain && value == "<<" {
                    return Err(ParseError::MergeKey);
                }
                record_mapping_key(&mut containers, value.as_ref())?;
                finish_node(&mut containers);
            }
            Event::SequenceStart(anchor, tag) => {
                anchor_found |= anchor != 0;
                inspect_tag(tag.as_deref())?;
                containers.push(Container::Sequence);
            }
            Event::MappingStart(anchor, tag) => {
                anchor_found |= anchor != 0;
                inspect_tag(tag.as_deref())?;
                containers.push(Container::Mapping {
                    expecting_key: true,
                    keys: BTreeSet::new(),
                });
            }
            Event::Alias(_) => return Err(ParseError::Alias),
            Event::SequenceEnd | Event::MappingEnd => {
                containers.pop();
                finish_node(&mut containers);
            }
            Event::Nothing | Event::StreamStart | Event::StreamEnd | Event::DocumentEnd => {}
        }
    }

    if anchor_found {
        Err(ParseError::Anchor)
    } else {
        Ok(())
    }
}

fn inspect_tag(tag: Option<&Tag>) -> Result<(), ParseError> {
    let Some(tag) = tag else {
        return Ok(());
    };

    if tag.is_yaml_core_schema() {
        match tag.suffix.as_str() {
            "str" | "null" | "bool" | "int" | "float" | "seq" | "map" => Ok(()),
            "merge" => Err(ParseError::MergeKey),
            _ => Err(ParseError::CustomTag {
                tag: tag.to_string(),
            }),
        }
    } else {
        Err(ParseError::CustomTag {
            tag: tag.to_string(),
        })
    }
}

fn is_mapping_key(containers: &[Container]) -> bool {
    matches!(
        containers.last(),
        Some(Container::Mapping {
            expecting_key: true,
            ..
        })
    )
}

fn record_mapping_key(containers: &mut [Container], key: &str) -> Result<(), ParseError> {
    let Some(Container::Mapping {
        expecting_key: true,
        keys,
    }) = containers.last_mut()
    else {
        return Ok(());
    };

    if keys.insert(key.to_owned()) {
        Ok(())
    } else {
        Err(ParseError::DuplicateKey {
            key: key.to_owned(),
        })
    }
}

fn finish_node(containers: &mut [Container]) {
    if let Some(Container::Mapping { expecting_key, .. }) = containers.last_mut() {
        *expecting_key = !*expecting_key;
    }
}
