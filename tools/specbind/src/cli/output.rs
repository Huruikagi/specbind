//! Shared text-result construction and stream routing.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub success: bool,
}

impl CommandOutput {
    pub(super) fn success(stdout: Vec<u8>) -> Self {
        Self {
            stdout,
            stderr: vec![],
            success: true,
        }
    }

    pub(super) fn failure(code: &str, message: impl AsRef<str>, details: Vec<String>) -> Self {
        let mut stderr = format!("ERROR {code}: {}\n", escape(message.as_ref()));
        for detail in details {
            stderr.push_str("  ");
            stderr.push_str(&escape(&detail));
            stderr.push('\n');
        }
        Self {
            stdout: vec![],
            stderr: stderr.into_bytes(),
            success: false,
        }
    }

    pub(super) fn no_change(code: &str, message: &str) -> Self {
        Self::success(format!("NO_CHANGE {code}: {message}\n").into_bytes())
    }
}

pub(super) fn push_field(output: &mut String, label: &str, value: &str) {
    output.push_str("  ");
    output.push_str(label);
    output.push_str(": ");
    output.push_str(&escape(value));
    output.push('\n');
}

pub(super) fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

pub(super) fn push_inline_list(output: &mut String, label: &str, values: &[String]) {
    let value = if values.is_empty() {
        "none".to_owned()
    } else {
        values
            .iter()
            .map(|value| escape(value))
            .collect::<Vec<_>>()
            .join(", ")
    };
    push_field(output, label, &value);
}

pub(super) fn push_list(output: &mut String, label: &str, values: &[String]) {
    if values.is_empty() {
        push_field(output, label, "none");
        return;
    }
    output.push_str("  ");
    output.push_str(label);
    output.push_str(":\n");
    for value in values {
        output.push_str("    - ");
        output.push_str(&escape(value));
        output.push('\n');
    }
}

pub(super) fn escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|value| match value {
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect(),
            '\t' => "\\t".chars().collect(),
            value if value.is_control() || value == '\u{1b}' => {
                format!("\\u{{{:x}}}", u32::from(value)).chars().collect()
            }
            value => vec![value],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{CommandOutput, push_inline_list, push_list};

    #[test]
    fn routes_and_escapes_failure_output() {
        let output = CommandOutput::failure(
            "BROKEN",
            "message\n\t\u{1b}",
            vec!["detail\r\u{7}".to_owned()],
        );

        assert!(!output.success);
        assert!(output.stdout.is_empty());
        assert_eq!(
            output.stderr,
            b"ERROR BROKEN: message\\n\\t\\u{1b}\n  detail\\r\\u{7}\n".to_vec()
        );
    }

    #[test]
    fn routes_no_change_output_to_stdout() {
        let output = CommandOutput::no_change("UNCHANGED", "Nothing changed.");

        assert!(output.success);
        assert_eq!(
            output.stdout,
            b"NO_CHANGE UNCHANGED: Nothing changed.\n".to_vec()
        );
        assert!(output.stderr.is_empty());
    }

    #[test]
    fn renders_empty_and_escaped_lists_with_the_existing_layout() {
        let mut output = String::new();
        push_inline_list(&mut output, "Inline", &[]);
        push_list(&mut output, "Items", &["first\nitem".to_owned()]);

        assert_eq!(output, "  Inline: none\n  Items:\n    - first\\nitem\n");
    }
}
