//! Catalog of embedded default shared rules.
//!
//! A rule is project-owned policy, not product behavior. These embedded copies
//! exist only as installation assets: once installed, skills read the project
//! copy through the CLI, and an absent project file simply means no
//! customization applies. The embedded copy is never a runtime fallback.

use std::{fs, path::Path};

/// One embedded default shared rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultRule {
    /// Stable selector naming this rule.
    pub selector: &'static str,
    /// File name below `settings/rules/`.
    pub file_name: &'static str,
    /// One-line statement of the customizable responsibility it carries.
    pub purpose: &'static str,
    content: &'static str,
}

impl DefaultRule {
    /// Returns the project-relative installed path.
    #[must_use]
    pub fn path(self) -> String {
        format!("{RULES_ROOT}/{}", self.file_name)
    }

    /// Returns the raw rule Markdown.
    #[must_use]
    pub fn content(self) -> &'static str {
        self.content
    }

    /// Reads the project's copy, if it has one.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic when the target exists but is not a regular
    /// non-symlink UTF-8 file. Absence means no project customization.
    pub fn read(self, specbind_root: &Path) -> Result<Option<String>, RuleError> {
        let relative = self.path();
        let target = specbind_root.join(&relative);
        let metadata = match fs::symlink_metadata(&target) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(RuleError {
                    code: "RULE_READ_FAILED",
                    message: format!("{relative}: {error}"),
                });
            }
        };
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(RuleError {
                code: "RULE_READ_TARGET_INVALID",
                message: format!("{relative} must be a regular non-symlink file"),
            });
        }
        match fs::read(&target).map(String::from_utf8) {
            Ok(Ok(content)) => Ok(Some(content)),
            Ok(Err(_)) => Err(RuleError {
                code: "RULE_READ_NOT_UTF8",
                message: format!("{relative} must be UTF-8"),
            }),
            Err(error) => Err(RuleError {
                code: "RULE_READ_FAILED",
                message: format!("{relative}: {error}"),
            }),
        }
    }
}

/// The complete Decision 0093 installed default set.
///
/// One English set serves both configured artifact languages; projects may
/// localize or rewrite their installed copies.
static DEFAULT_RULES: &[DefaultRule] = &[
    DefaultRule {
        selector: "ears-format",
        file_name: "ears-format.md",
        purpose: "Preferred EARS patterns, subject choice, and testability style for Requirements.",
        content: include_str!("../../assets/rules/ears-format.md"),
    },
    DefaultRule {
        selector: "design-principles",
        file_name: "design-principles.md",
        purpose: "Project-adjustable architecture, interface, data-model, error-handling, diagram, and documentation preferences.",
        content: include_str!("../../assets/rules/design-principles.md"),
    },
    DefaultRule {
        selector: "contract-principles",
        file_name: "contract-principles.md",
        purpose: "Project policy for seam ownership, compatibility posture, dependency direction, and warning severity.",
        content: include_str!("../../assets/rules/contract-principles.md"),
    },
    DefaultRule {
        selector: "tasks-generation",
        file_name: "tasks-generation.md",
        purpose: "Project preferences for task sizing, decomposition, completion detail, test grouping, and parallelization.",
        content: include_str!("../../assets/rules/tasks-generation.md"),
    },
    DefaultRule {
        selector: "steering-principles",
        file_name: "steering-principles.md",
        purpose: "Project preferences for durable steering granularity, examples, and preservation.",
        content: include_str!("../../assets/rules/steering-principles.md"),
    },
];

/// The project tree that holds installed shared rules.
pub const RULES_ROOT: &str = "settings/rules";

/// Lists every embedded default rule.
#[must_use]
pub fn defaults() -> &'static [DefaultRule] {
    DEFAULT_RULES
}

/// Resolves one rule by its accepted selector.
#[must_use]
pub fn find(selector: &str) -> Option<DefaultRule> {
    DEFAULT_RULES
        .iter()
        .copied()
        .find(|entry| entry.selector == selector)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleError {
    pub code: &'static str,
    pub message: String,
}
