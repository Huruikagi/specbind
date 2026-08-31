//! Catalog of embedded default shared rules.
//!
//! A rule is project-owned policy, not product behavior. These embedded copies
//! exist only as installation assets: once installed, skills read the project
//! copy through the CLI, and an absent ordinary preference file simply means no
//! customization applies. Decision 0152's Design-template selection rule is a
//! required routing input. The embedded copy is never a runtime fallback.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use crate::config::ProjectLanguage;

/// One embedded default shared rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultRule {
    /// Stable selector naming this rule.
    pub selector: &'static str,
    /// File name below `settings/rules/`.
    pub file_name: &'static str,
    /// One-line statement of the customizable responsibility it carries.
    pub purpose: &'static str,
    /// Configured language for which installation offers this default.
    /// `None` means every supported language.
    default_language: Option<ProjectLanguage>,
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

    /// Whether installation should offer this default for the configured language.
    #[must_use]
    pub fn installs_for(self, language: ProjectLanguage) -> bool {
        self.default_language
            .is_none_or(|expected| expected == language)
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

/// The complete accepted Rule catalog from Decisions 0093, 0152, and 0169.
static DEFAULT_RULES: &[DefaultRule] = &[
    DefaultRule {
        selector: "ears-format",
        file_name: "ears-format.md",
        purpose: "Preferred EARS patterns, subject choice, and testability style for Requirements.",
        default_language: None,
        content: include_str!("../../assets/rules/ears-format.md"),
    },
    DefaultRule {
        selector: "design-principles",
        file_name: "design-principles.md",
        purpose: "Project-adjustable architecture, interface, data-model, error-handling, diagram, and documentation preferences.",
        default_language: None,
        content: include_str!("../../assets/rules/design-principles.md"),
    },
    DefaultRule {
        selector: "design-template-selection",
        file_name: "design-template-selection.md",
        purpose: "Required, conditional, or disabled selection policy for every Design template.",
        default_language: None,
        content: include_str!("../../assets/rules/design-template-selection.md"),
    },
    DefaultRule {
        selector: "contract-principles",
        file_name: "contract-principles.md",
        purpose: "Project policy for seam ownership, compatibility posture, dependency direction, and warning severity.",
        default_language: None,
        content: include_str!("../../assets/rules/contract-principles.md"),
    },
    DefaultRule {
        selector: "tasks-generation",
        file_name: "tasks-generation.md",
        purpose: "Project preferences for task sizing, decomposition, completion detail, and test grouping.",
        default_language: None,
        content: include_str!("../../assets/rules/tasks-generation.md"),
    },
    DefaultRule {
        selector: "steering-principles",
        file_name: "steering-principles.md",
        purpose: "Project preferences for durable steering granularity, examples, and preservation.",
        default_language: None,
        content: include_str!("../../assets/rules/steering-principles.md"),
    },
    DefaultRule {
        selector: "language-style",
        file_name: "language-style.md",
        purpose: "Project preferences for natural-language prose while preserving exact product and machine identifiers.",
        default_language: Some(ProjectLanguage::Ja),
        content: include_str!("../../assets/rules/language-style.md"),
    },
];

/// The project tree that holds installed shared rules.
pub const RULES_ROOT: &str = "settings/rules";

/// Lists every embedded default rule.
#[must_use]
pub fn defaults() -> &'static [DefaultRule] {
    DEFAULT_RULES
}

/// Lists the defaults installation offers for one configured language.
pub fn installed_defaults(
    language: ProjectLanguage,
) -> impl Iterator<Item = DefaultRule> + 'static {
    DEFAULT_RULES
        .iter()
        .copied()
        .filter(move |entry| entry.installs_for(language))
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

#[derive(Default)]
struct DesignSelectionSection {
    selector: String,
    line: usize,
    lines: Vec<String>,
}

fn finish_design_selection_section(
    section: DesignSelectionSection,
    entries: &mut Vec<(String, String, usize)>,
    issues: &mut Vec<RuleError>,
) {
    if section.selector.is_empty() {
        return;
    }
    let mut non_empty = section.lines.iter().filter(|line| !line.trim().is_empty());
    let Some(mode_line) = non_empty.next() else {
        issues.push(RuleError {
            code: "RULE_DESIGN_TEMPLATE_MODE_MISSING",
            message: format!(
                "line {}: {} must declare Mode",
                section.line, section.selector
            ),
        });
        return;
    };
    let Some(mode) = mode_line.trim().strip_prefix("Mode: ") else {
        issues.push(RuleError {
            code: "RULE_DESIGN_TEMPLATE_MODE_MISSING",
            message: format!(
                "line {}: {} must begin with Mode: required, conditional, or disabled",
                section.line, section.selector
            ),
        });
        return;
    };
    if !matches!(mode, "required" | "conditional" | "disabled") {
        issues.push(RuleError {
            code: "RULE_DESIGN_TEMPLATE_MODE_INVALID",
            message: format!(
                "line {}: {} has unsupported mode {mode}",
                section.line, section.selector
            ),
        });
        return;
    }
    if mode == "conditional" && non_empty.next().is_none() {
        issues.push(RuleError {
            code: "RULE_DESIGN_TEMPLATE_CONDITION_MISSING",
            message: format!(
                "line {}: {} is conditional but has no applicability condition",
                section.line, section.selector
            ),
        });
    }
    entries.push((section.selector, mode.to_owned(), section.line));
}

/// Deterministically validates the selector and mode declarations in the
/// project-owned Design-template selection rule.
///
/// Applicability prose remains agent-interpreted project policy. This parser
/// only proves that every discovered Design template is classified exactly
/// once and that a conditional classification actually carries a condition.
#[must_use]
pub fn validate_design_template_selection(
    content: &str,
    design_selectors: &[String],
) -> Vec<RuleError> {
    let mut entries = Vec::new();
    let mut issues = Vec::new();
    let mut current = DesignSelectionSection::default();
    for (index, line) in content.lines().enumerate() {
        if let Some(heading) = line.strip_prefix("## ") {
            finish_design_selection_section(current, &mut entries, &mut issues);
            current = DesignSelectionSection::default();
            let Some(selector) = heading
                .strip_prefix('`')
                .and_then(|value| value.strip_suffix('`'))
                .filter(|value| {
                    value.starts_with("design/") && !value["design/".len()..].is_empty()
                })
            else {
                issues.push(RuleError {
                    code: "RULE_DESIGN_TEMPLATE_SELECTOR_INVALID",
                    message: format!(
                        "line {}: level-two headings must name a `design/<artifact_id>` selector",
                        index + 1
                    ),
                });
                continue;
            };
            selector.clone_into(&mut current.selector);
            current.line = index + 1;
        } else if !current.selector.is_empty() {
            current.lines.push(line.to_owned());
        }
    }
    finish_design_selection_section(current, &mut entries, &mut issues);

    let mut classified = BTreeMap::new();
    for (selector, mode, line) in entries {
        if classified.insert(selector.clone(), mode).is_some() {
            issues.push(RuleError {
                code: "RULE_DESIGN_TEMPLATE_SELECTOR_DUPLICATE",
                message: format!("line {line}: {selector} is classified more than once"),
            });
        }
    }
    let discovered = design_selectors.iter().cloned().collect::<BTreeSet<_>>();
    for selector in &discovered {
        if !classified.contains_key(selector) {
            issues.push(RuleError {
                code: "RULE_DESIGN_TEMPLATE_SELECTOR_MISSING",
                message: format!("{selector} is not classified by the selection rule"),
            });
        }
    }
    for selector in classified.keys() {
        if !discovered.contains(selector) {
            issues.push(RuleError {
                code: "RULE_DESIGN_TEMPLATE_SELECTOR_UNKNOWN",
                message: format!("{selector} does not resolve to a discovered Design template"),
            });
        }
    }
    if !classified.values().any(|mode| mode == "required") {
        issues.push(RuleError {
            code: "RULE_DESIGN_TEMPLATE_REQUIRED_MISSING",
            message: "at least one Design template must be classified required".to_owned(),
        });
    }
    issues
}
