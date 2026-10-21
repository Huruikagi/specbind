//! Source slices for Roadmap parsing and scope-body preservation.
//!
//! Keep the established projections distinct: strict parsing accepts a BOM and
//! requires a newline after the closing delimiter; scope extraction scans whole
//! lines without stripping a BOM. Moving ownership must not change either result.

/// Returns the authored Markdown body that follows the Front Matter.
///
/// The Roadmap parser does not carry the body, so both scope replacement and the
/// scope read recover it from the original source through this one function.
pub(crate) fn scope_body(source: &str) -> &str {
    let mut offset = 0;
    let mut lines = source.split_inclusive('\n');
    let Some(first) = lines.next() else {
        return "";
    };
    if line_content(first) != "---" {
        return "";
    }
    offset += first.len();
    for line in lines {
        offset += line.len();
        if line_content(line) == "---" {
            return &source[offset..];
        }
    }
    ""
}

fn line_content(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

pub(crate) fn split_frontmatter_parts(content: &str) -> Result<(&str, &str), String> {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content);
    let rest = normalized
        .strip_prefix("---\n")
        .or_else(|| normalized.strip_prefix("---\r\n"))
        .ok_or_else(|| "ROADMAP_FRONTMATTER_MISSING: expected opening ---".to_owned())?;
    let (end, marker_len) = rest
        .find("\n---\n")
        .map(|end| (end, "\n---\n".len()))
        .or_else(|| {
            rest.find("\r\n---\r\n")
                .map(|end| (end, "\r\n---\r\n".len()))
        })
        .ok_or_else(|| "ROADMAP_FRONTMATTER_MISSING: expected closing ---".to_owned())?;
    Ok((&rest[..end], &rest[end + marker_len..]))
}

#[cfg(test)]
mod tests {
    use super::{scope_body, split_frontmatter_parts};

    #[test]
    fn scope_body_preserves_authored_bytes_and_legacy_delimiter_handling() {
        for (source, expected) in [
            (
                "---\ntype: Roadmap\n---\n# Body\n\n---\n",
                "# Body\n\n---\n",
            ),
            ("---\r\ntype: Roadmap\r\n---\r\n# Body\r\n", "# Body\r\n"),
            ("---\r\ntype: Roadmap\n---\r\n本文\n", "本文\n"),
            ("---\ntype: Roadmap\n---", ""),
            ("\u{feff}---\ntype: Roadmap\n---\n# Body\n", ""),
            ("---\ntype: Roadmap\n", ""),
            ("# No Front Matter\n", ""),
        ] {
            assert_eq!(scope_body(source), expected, "{source:?}");
        }
    }

    #[test]
    fn strict_source_split_keeps_bom_and_closing_delimiter_contracts() {
        for newline in ["\n", "\r\n"] {
            let source =
                format!("\u{feff}---{newline}type: Roadmap{newline}---{newline}本文{newline}");
            assert_eq!(
                split_frontmatter_parts(&source).unwrap(),
                ("type: Roadmap", format!("本文{newline}").as_str()),
            );
        }
        assert_eq!(
            split_frontmatter_parts("---\ntype: Roadmap\n---").unwrap_err(),
            "ROADMAP_FRONTMATTER_MISSING: expected closing ---",
        );
    }
}
