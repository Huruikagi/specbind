//! Shared lexical grammar for portable project-relative paths.

use std::path::{Component, Path};

/// Checks path spelling only; callers retain target policy and filesystem guards.
pub(crate) fn is_project_relative(value: &str) -> bool {
    let path = Path::new(value);
    let invalid = value.is_empty()
        || value.contains('\\')
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::CurDir
                    | Component::ParentDir
                    | Component::RootDir
                    | Component::Prefix(_)
            )
        })
        || value.split('/').any(invalid_portable_segment);
    !invalid
}

fn invalid_portable_segment(segment: &str) -> bool {
    if segment.is_empty()
        || segment.ends_with([' ', '.'])
        || segment
            .chars()
            .any(|value| value.is_control() || r#"<>:"\|?*"#.contains(value))
    {
        return true;
    }
    let stem = segment
        .split('.')
        .next()
        .unwrap_or(segment)
        .to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || stem
            .strip_prefix("COM")
            .or_else(|| stem.strip_prefix("LPT"))
            .is_some_and(|suffix| {
                matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portable_relative_spelling_preserves_accepted_names() {
        for value in [
            ".specbind",
            "docs/specs",
            "仕様/内部 メモ.md",
            ".hidden/file.ext",
            "COM0",
            "COM10.log",
            "LPT0",
            "LPT10",
            "console.txt",
        ] {
            assert!(is_project_relative(value), "{value:?}");
        }
    }

    #[test]
    fn rejects_nonportable_or_nonrelative_spelling() {
        for value in [
            "",
            "/absolute",
            "C:/absolute",
            "C:relative",
            "dir\\child",
            ".",
            "..",
            "./child",
            "dir/./child",
            "dir/../child",
            "dir//child",
            "dir/",
            "dir/name.",
            "dir/name ",
            "dir/control\nname",
            "dir/control\0name",
            "dir/<name",
            "dir/>name",
            "dir/:name",
            "dir/\"name",
            "dir/|name",
            "dir/?name",
            "dir/*name",
            "CON",
            "prn.txt",
            "dir/Aux",
            "dir/nul.log",
            "com1.log",
            "COM9",
            "lpt1.txt",
            "LPT9",
        ] {
            assert!(!is_project_relative(value), "{value:?}");
        }
    }
}
