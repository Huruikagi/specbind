//! Foundation service for project and SpecBind-root resolution.

use std::{
    fmt, fs,
    path::{Component, Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    guarded_fs,
    repository::{self, RepositoryError},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectPaths {
    pub project_root: PathBuf,
    pub specbind_root: PathBuf,
    pub language: ProjectLanguage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectLanguage {
    En,
    Ja,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub code: &'static str,
    pub message: String,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RootConfig {
    schema_version: u64,
    spec_dir: String,
    language: ProjectLanguage,
}

/// Resolves only the containing Git project root.
///
/// Installation planning needs this before `.specbind.json` exists.
///
/// # Errors
///
/// Returns a stable Git diagnostic when no project root can be resolved.
pub fn project_root_from(start: &Path) -> Result<PathBuf, ConfigError> {
    git_project_root(start)
}

/// Resolves the containing Git project and its configured `SpecBind` root.
///
/// This read-only projection validates the configuration fields required by
/// current project commands, including the language used for CLI-authored artifacts.
/// Agent-list validation stays with the later complete installation boundary.
///
/// # Errors
///
/// Returns a stable configuration or Git diagnostic when no safe root can be used.
pub fn resolve_from(start: &Path) -> Result<ProjectPaths, ConfigError> {
    let project_root = git_project_root(start)?;
    let config_path = project_root.join(".specbind.json");
    let config_metadata = fs::symlink_metadata(&config_path).map_err(|error| ConfigError {
        code: "CONFIG_READ_FAILED",
        message: format!("cannot inspect {}: {error}", config_path.display()),
    })?;
    if guarded_fs::is_link_like(&config_metadata) || !config_metadata.is_file() {
        return Err(ConfigError {
            code: "CONFIG_TARGET_INVALID",
            message: ".specbind.json must be a regular non-symlink file".to_owned(),
        });
    }
    let input = fs::read_to_string(&config_path).map_err(|error| ConfigError {
        code: "CONFIG_READ_FAILED",
        message: format!("cannot read {}: {error}", config_path.display()),
    })?;
    let config = serde_json::from_str::<RootConfig>(&input).map_err(|error| ConfigError {
        code: "CONFIG_INVALID",
        message: format!(".specbind.json is invalid: {error}"),
    })?;
    if config.schema_version != 1 {
        return Err(ConfigError {
            code: "CONFIG_VERSION_UNSUPPORTED",
            message: "schemaVersion must be 1".to_owned(),
        });
    }
    validate_spec_dir(&config.spec_dir)?;
    let mut specbind_root = project_root.clone();
    for segment in config.spec_dir.split('/') {
        specbind_root.push(segment);
        let metadata = fs::symlink_metadata(&specbind_root).map_err(|error| ConfigError {
            code: "SPEC_ROOT_UNAVAILABLE",
            message: format!("cannot inspect configured specDir: {error}"),
        })?;
        if guarded_fs::is_link_like(&metadata) || !metadata.is_dir() {
            return Err(ConfigError {
                code: "SPEC_ROOT_INVALID",
                message: "configured specDir must traverse only regular directories".to_owned(),
            });
        }
    }
    reject_nested_submodule(&project_root, &config.spec_dir)?;
    Ok(ProjectPaths {
        project_root,
        specbind_root,
        language: config.language,
    })
}

fn reject_nested_submodule(project_root: &Path, spec_dir: &str) -> Result<(), ConfigError> {
    let output = repository::output_bytes(project_root, &["ls-files", "--stage", "-z"]).map_err(
        |error| match error {
            RepositoryError::Start(_) => ConfigError {
                code: "GIT_START_FAILED",
                message: error.to_string(),
            },
            _ => ConfigError {
                code: "SPEC_ROOT_SUBMODULE_CHECK_FAILED",
                message: error.to_string(),
            },
        },
    )?;
    let nested = output
        .split(|byte| *byte == 0)
        .filter_map(|record| {
            record
                .iter()
                .position(|byte| *byte == b'\t')
                .map(|index| (&record[..index], &record[index + 1..]))
        })
        .filter(|(metadata, _)| metadata.starts_with(b"160000 "))
        .map(|(_, path)| path)
        .any(|path| {
            let spec_dir = spec_dir.as_bytes();
            path == spec_dir
                || path
                    .strip_prefix(spec_dir)
                    .is_some_and(|suffix| suffix.starts_with(b"/"))
                || spec_dir
                    .strip_prefix(path)
                    .is_some_and(|suffix| suffix.starts_with(b"/"))
        });
    if nested {
        return Err(ConfigError {
            code: "SPEC_ROOT_IN_SUBMODULE",
            message: "configured specDir must not be inside a nested submodule".to_owned(),
        });
    }
    Ok(())
}

fn git_project_root(start: &Path) -> Result<PathBuf, ConfigError> {
    let root =
        repository::output(start, &["rev-parse", "--show-toplevel"]).map_err(
            |error| match error {
                RepositoryError::Start(_) => ConfigError {
                    code: "GIT_START_FAILED",
                    message: error.to_string(),
                },
                RepositoryError::NonUtf8(error) => ConfigError {
                    code: "GIT_OUTPUT_INVALID",
                    message: format!("Git project root is not UTF-8: {error}"),
                },
                _ => ConfigError {
                    code: "PROJECT_ROOT_NOT_FOUND",
                    message: error.to_string(),
                },
            },
        )?;
    Ok(PathBuf::from(root.trim()))
}

fn validate_spec_dir(value: &str) -> Result<(), ConfigError> {
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
    if invalid {
        Err(ConfigError {
            code: "SPEC_DIR_INVALID",
            message: "specDir must be a portable project-root-relative child directory".to_owned(),
        })
    } else {
        Ok(())
    }
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
