use super::{DiscoveryIssue, Path, Utf8PathBuf, fs};

/// Accepts an absent tree and rejects any root that cannot be scanned safely.
pub(super) fn validate_template_root(root: &Path, label: &str) -> Result<(), Vec<DiscoveryIssue>> {
    let path = Some(Utf8PathBuf::from(label));
    match fs::symlink_metadata(root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(vec![]),
        Err(error) => Err(vec![issue(
            "TEMPLATE_ROOT_UNREADABLE",
            path,
            format!("cannot read the template root: {error}"),
        )]),
        Ok(metadata) if metadata.file_type().is_symlink() => Err(vec![issue(
            "TEMPLATE_ROOT_SYMLINK",
            path,
            "template root must not be a symbolic link",
        )]),
        Ok(metadata) if !metadata.is_dir() => Err(vec![issue(
            "TEMPLATE_ROOT_NOT_DIRECTORY",
            path,
            "template root must be a directory",
        )]),
        Ok(_) => Ok(()),
    }
}

/// Renders one root-relative path with the `/` separator the CLI contract uses.
pub(super) fn relative(base: &Path, path: &Path) -> Option<Utf8PathBuf> {
    let relative = path.strip_prefix(base).ok()?;
    let utf8 = Utf8PathBuf::from_path_buf(relative.to_path_buf()).ok()?;
    Some(Utf8PathBuf::from(utf8.as_str().replace('\\', "/")))
}

pub(super) fn issue(
    code: &'static str,
    path: Option<Utf8PathBuf>,
    message: impl Into<String>,
) -> DiscoveryIssue {
    DiscoveryIssue {
        code,
        path,
        message: message.into(),
    }
}
