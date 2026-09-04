//! Project-independent catalog commands backed entirely by embedded assets.

use super::super::*;

/// Lists the embedded product protocols.
///
/// Protocols are compiled into this binary, so this command deliberately takes
/// no project path and works without `.specbind.json` or an installation.
#[must_use]
pub fn protocol_list() -> CommandOutput {
    let protocols = protocol::list();
    let mut output = format!(
        "OK PROTOCOL_LISTED: Found {} product protocol(s).\n",
        protocols.len()
    );
    for entry in protocols {
        writeln!(
            output,
            "  selector={} purpose=\"{}\"",
            escape(entry.selector),
            escape(entry.purpose)
        )
        .expect("writing to a String cannot fail");
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one embedded product protocol as raw Markdown.
#[must_use]
pub fn protocol_read(selector: &str) -> CommandOutput {
    match protocol::read(selector) {
        Some(entry) => CommandOutput::success(entry.content().as_bytes().to_vec()),
        None => CommandOutput::failure(
            "PROTOCOL_SELECTOR_NOT_FOUND",
            format!("Selector {selector} does not resolve to an embedded product protocol."),
            protocol::list()
                .iter()
                .map(|entry| format!("available selector {}", escape(entry.selector)))
                .collect(),
        ),
    }
}

/// Lists every embedded structured-artifact schema.
///
/// Like the protocols, these are properties of the binary. Taking no project
/// path is the structural guarantee of that rather than a convenience.
#[must_use]
pub fn schema_list() -> CommandOutput {
    let schemas = schema::schemas();
    let mut output = format!(
        "OK SCHEMA_LISTED: Found {} embedded schema(s).\n",
        schemas.len()
    );
    for entry in schemas {
        let _ = writeln!(
            output,
            "  selector={} artifact={} written_by=\"{}\"",
            escape(entry.selector),
            escape(entry.artifact),
            escape(entry.written_by)
        );
    }
    CommandOutput::success(output.into_bytes())
}

/// Reads one versioned schema selector as raw JSON.
#[must_use]
pub fn schema_read(selector: &str) -> CommandOutput {
    schema::find_schema(selector).map_or_else(
        || {
            CommandOutput::failure(
                "SCHEMA_READ_INVALID",
                format!("unknown schema selector: {selector}"),
                vec![],
            )
        },
        |entry| CommandOutput::success(entry.content().as_bytes().to_vec()),
    )
}
