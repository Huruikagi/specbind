//! Embedded default shared rules.
//!
//! A rule is project-owned policy, not product behavior. These embedded copies
//! exist only as installation assets: once installed, skills read the project
//! file, and an absent project file simply means no customization applies. The
//! embedded copy is never a runtime fallback.

/// One embedded default shared rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultRule {
    /// File name below `settings/rules/`.
    pub file_name: &'static str,
    /// One-line statement of the customizable responsibility it carries.
    pub purpose: &'static str,
    content: &'static str,
}

impl DefaultRule {
    /// Returns the raw rule Markdown.
    #[must_use]
    pub fn content(self) -> &'static str {
        self.content
    }
}

/// The complete Decision 0093 installed default set.
///
/// One English set serves both configured artifact languages; projects may
/// localize or rewrite their installed copies.
static DEFAULT_RULES: &[DefaultRule] = &[
    DefaultRule {
        file_name: "ears-format.md",
        purpose: "Preferred EARS patterns, subject choice, and testability style for Requirements.",
        content: include_str!("../assets/rules/ears-format.md"),
    },
    DefaultRule {
        file_name: "design-principles.md",
        purpose: "Project-adjustable architecture, interface, data-model, error-handling, diagram, and documentation preferences.",
        content: include_str!("../assets/rules/design-principles.md"),
    },
    DefaultRule {
        file_name: "contract-principles.md",
        purpose: "Project policy for seam ownership, compatibility posture, dependency direction, and warning severity.",
        content: include_str!("../assets/rules/contract-principles.md"),
    },
    DefaultRule {
        file_name: "tasks-generation.md",
        purpose: "Project preferences for task sizing, decomposition, completion detail, test grouping, and parallelization.",
        content: include_str!("../assets/rules/tasks-generation.md"),
    },
    DefaultRule {
        file_name: "steering-principles.md",
        purpose: "Project preferences for durable steering granularity, examples, and preservation.",
        content: include_str!("../assets/rules/steering-principles.md"),
    },
];

/// The project tree that holds installed shared rules.
pub const RULES_ROOT: &str = "settings/rules";

/// Lists every embedded default rule.
#[must_use]
pub fn defaults() -> &'static [DefaultRule] {
    DEFAULT_RULES
}
