//! Reads the optional, project-owned shared Contract without lifecycle mutation.
use crate::{domain::shared_contract::SharedContract, schema::runtime};
use std::{fs, path::Path};

pub const SHARED_CONTRACT_RELATIVE: &str = "specs/shared-contract.yaml";
pub const LEGACY_SHARED_CONTRACT_RELATIVE: &str = "shared-contract.yaml";

/// Reads an optional regular shared Contract; malformed presence is never absence.
/// # Errors
/// Returns read, structure, or semantic diagnostics.
pub fn read(root: &Path) -> Result<Option<SharedContract>, String> {
    let Some(relative) = present_relative(root)? else {
        return Ok(None);
    };
    let path = root.join(relative);
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    parse(&text).map(Some)
}

/// Resolves the canonical path or the v1.5.0 compatibility path without ambiguity.
/// # Errors
/// Returns metadata errors, unsafe file types, or duplicate-path ambiguity.
pub fn present_relative(root: &Path) -> Result<Option<&'static str>, String> {
    let mut present = None;
    for relative in [SHARED_CONTRACT_RELATIVE, LEGACY_SHARED_CONTRACT_RELATIVE] {
        match fs::symlink_metadata(root.join(relative)) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
            Ok(metadata) if !crate::guarded_fs::is_regular_file(&metadata) => {
                return Err(format!(
                    "shared Contract at {relative} must be a regular non-symlink file"
                ));
            }
            Ok(_) if present.is_some() => {
                return Err(format!(
                    "shared Contract exists at both {SHARED_CONTRACT_RELATIVE} and {LEGACY_SHARED_CONTRACT_RELATIVE}"
                ));
            }
            Ok(_) => present = Some(relative),
        }
    }
    Ok(present)
}

/// Validates a shared Contract read from the current tree or Git baseline.
/// # Errors
/// Returns structure or semantic diagnostics.
pub fn parse(text: &str) -> Result<SharedContract, String> {
    let wire = runtime::load_shared_contract(text).map_err(|error| format!("{error:?}"))?;
    SharedContract::try_from(wire).map_err(|error| format!("{error:?}"))
}
