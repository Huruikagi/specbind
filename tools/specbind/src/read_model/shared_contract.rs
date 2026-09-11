//! Reads the optional, project-owned shared Contract without lifecycle mutation.
use crate::{domain::shared_contract::SharedContract, schema::runtime};
use std::{fs, path::Path};

pub const SHARED_CONTRACT_RELATIVE: &str = "specs/shared-contract.yaml";

/// Reads an optional regular shared Contract; malformed presence is never absence.
/// # Errors
/// Returns read, structure, or semantic diagnostics.
pub fn read(root: &Path) -> Result<Option<SharedContract>, String> {
    let path = root.join(SHARED_CONTRACT_RELATIVE);
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.to_string()),
        Ok(metadata) if !crate::guarded_fs::is_regular_file(&metadata) => {
            return Err("shared Contract must be a regular non-symlink file".into());
        }
        Ok(_) => {}
    }
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    parse(&text).map(Some)
}

/// Validates a shared Contract read from the current tree or Git baseline.
/// # Errors
/// Returns structure or semantic diagnostics.
pub fn parse(text: &str) -> Result<SharedContract, String> {
    let wire = runtime::load_shared_contract(text).map_err(|error| format!("{error:?}"))?;
    SharedContract::try_from(wire).map_err(|error| format!("{error:?}"))
}
