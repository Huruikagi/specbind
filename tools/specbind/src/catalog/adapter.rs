//! Catalog of project-owned operational adapter contracts.
//!
//! Decision 0101 gives release and Git guidance one home below
//! `settings/adapters/`. The selector set is closed: an unknown file there is
//! never listed, never readable, and never acquires meaning by existing. That
//! is what keeps the directory organization rather than an extension loader.
//!
//! The embedded scaffolds are installation assets only. Once installed the
//! project owns its copy, and an absent adapter simply means no guidance; the
//! embedded copy is never a runtime fallback.

use std::{fs, path::Path};

use pulldown_cmark::{Event, Parser};

use crate::config::ProjectLanguage;

const SCAFFOLD_MARKER: &str = "<!-- specbind:adapter-scaffold -->";

/// One accepted adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Adapter {
    /// Stable selector naming this adapter.
    pub selector: &'static str,
    /// File name below `settings/adapters/`.
    pub file_name: &'static str,
    /// Exact OKF type its document declares.
    pub artifact_type: &'static str,
    /// One-line statement of the operational responsibility it carries.
    pub purpose: &'static str,
    english: &'static str,
    japanese: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterState {
    Absent,
    Scaffold,
    Active,
}

impl AdapterState {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Scaffold => "scaffold",
            Self::Active => "active",
        }
    }
}

/// The project tree that holds installed adapters.
pub const ADAPTERS_ROOT: &str = "settings/adapters";

/// The complete accepted selector set.
static ADAPTERS: &[Adapter] = &[
    Adapter {
        selector: "release",
        file_name: "release.md",
        artifact_type: "SpecBind Release Adapter",
        purpose: "Project-specific release preparation, publication, verification, and cleanup.",
        english: include_str!("../../assets/adapters/en/release.md"),
        japanese: include_str!("../../assets/adapters/ja/release.md"),
    },
    Adapter {
        selector: "git",
        file_name: "git.md",
        artifact_type: "SpecBind Git Adapter",
        purpose: "Project checkpoint granularity, commit grouping and messages, and push policy.",
        english: include_str!("../../assets/adapters/en/git.md"),
        japanese: include_str!("../../assets/adapters/ja/git.md"),
    },
    Adapter {
        selector: "deferred",
        file_name: "deferred.md",
        artifact_type: "SpecBind Deferred Findings Adapter",
        purpose: "Project destination for a review finding that is real but does not hold a gate.",
        english: include_str!("../../assets/adapters/en/deferred.md"),
        japanese: include_str!("../../assets/adapters/ja/deferred.md"),
    },
];

/// Lists every accepted adapter.
#[must_use]
pub fn all() -> &'static [Adapter] {
    ADAPTERS
}

/// Resolves one adapter by selector.
///
/// Only an accepted selector resolves. A file that happens to sit below
/// `settings/adapters/` is not an adapter.
#[must_use]
pub fn find(selector: &str) -> Option<Adapter> {
    ADAPTERS
        .iter()
        .copied()
        .find(|entry| entry.selector == selector)
}

impl Adapter {
    /// Returns the project-relative installed path.
    #[must_use]
    pub fn path(self) -> String {
        format!("{ADAPTERS_ROOT}/{}", self.file_name)
    }

    /// Returns the embedded scaffold for one configured language.
    ///
    /// The scaffold is localized because a project fills it with its own
    /// operational procedure. Only the `type` literal inside stays English,
    /// being machine identity rather than prose.
    #[must_use]
    pub fn scaffold(self, language: ProjectLanguage) -> &'static str {
        match language {
            ProjectLanguage::En => self.english,
            ProjectLanguage::Ja => self.japanese,
        }
    }

    /// Reads the project's copy, if it has one.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic when the target exists but cannot be used. Absence
    /// is `Ok(None)`: whether a missing adapter is a fault belongs to the
    /// consuming skill, not to this read.
    pub fn read(self, specbind_root: &Path) -> Result<Option<String>, AdapterError> {
        let relative = self.path();
        let target = specbind_root.join(&relative);
        let metadata = match fs::symlink_metadata(&target) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(AdapterError {
                    code: "ADAPTER_READ_FAILED",
                    message: format!("{relative}: {error}"),
                });
            }
        };
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(AdapterError {
                code: "ADAPTER_READ_TARGET_INVALID",
                message: format!("{relative} must be a regular non-symlink file"),
            });
        }
        match fs::read(&target).map(String::from_utf8) {
            Ok(Ok(content)) => Ok(Some(content)),
            Ok(Err(_)) => Err(AdapterError {
                code: "ADAPTER_READ_NOT_UTF8",
                message: format!("{relative} must be UTF-8"),
            }),
            Err(error) => Err(AdapterError {
                code: "ADAPTER_READ_FAILED",
                message: format!("{relative}: {error}"),
            }),
        }
    }

    /// Reports whether the project has this adapter.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic when the target exists but cannot be used.
    pub fn present(self, specbind_root: &Path) -> Result<bool, AdapterError> {
        self.read(specbind_root).map(|content| content.is_some())
    }

    /// Reports whether the project has active guidance rather than only an
    /// installed authoring scaffold.
    ///
    /// # Errors
    ///
    /// Returns the same target-inspection and UTF-8 diagnostics as [`Self::read`].
    pub fn state(self, specbind_root: &Path) -> Result<AdapterState, AdapterError> {
        let Some(content) = self.read(specbind_root)? else {
            return Ok(AdapterState::Absent);
        };
        if contains_scaffold_marker(&content) {
            Ok(AdapterState::Scaffold)
        } else {
            Ok(AdapterState::Active)
        }
    }
}

fn contains_scaffold_marker(content: &str) -> bool {
    Parser::new(content).any(|event| match event {
        Event::Html(value) | Event::InlineHtml(value) => value.trim() == SCAFFOLD_MARKER,
        _ => false,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterError {
    pub code: &'static str,
    pub message: String,
}
