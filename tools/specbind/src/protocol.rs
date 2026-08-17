//! Immutable product protocols embedded in this binary.
//!
//! A protocol is versioned product behavior, not project-owned policy. Every
//! selector is declared by this registry rather than inferred from a path, and
//! nothing here depends on a project root, `.specbind.json`, or installation.

/// One embedded product protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Protocol {
    /// Stable lowercase kebab-case product identifier.
    pub selector: &'static str,
    /// One-line statement of the semantic responsibility this protocol owns.
    pub purpose: &'static str,
    content: &'static str,
}

impl Protocol {
    /// Returns the raw protocol Markdown.
    #[must_use]
    pub fn content(self) -> &'static str {
        self.content
    }
}

/// The complete embedded registry, ordered for stable listing.
static PROTOCOLS: &[Protocol] = &[
    Protocol {
        selector: "okf-authoring",
        purpose: "OKF v0.2 authoring baseline, reserved-file behavior, extension preservation, and the boundary between OKF metadata and SpecBind authority.",
        content: include_str!("../assets/protocols/okf-authoring.md"),
    },
    Protocol {
        selector: "requirements-review",
        purpose: "Complete-current-contract quality, observable scope, testability, ambiguity handling, and semantic readiness before Requirements approval.",
        content: include_str!("../assets/protocols/requirements-review.md"),
    },
    Protocol {
        selector: "design-discovery",
        purpose: "Selection and escalation of repository investigation needed before a self-contained Design can be authored.",
        content: include_str!("../assets/protocols/design-discovery.md"),
    },
    Protocol {
        selector: "design-authoring",
        purpose: "Non-waivable synthesis, simplification, owned-boundary, self-containment, and Requirement/Contract realization baseline.",
        content: include_str!("../assets/protocols/design-authoring.md"),
    },
    Protocol {
        selector: "design-validation",
        purpose: "Semantic Design review baseline shared by pre-approval authoring review and independent validation.",
        content: include_str!("../assets/protocols/design-validation.md"),
    },
    Protocol {
        selector: "gap-analysis",
        purpose: "Evidence gathering, option analysis, uncertainty handling, and the boundary between milestone-local Research and authoritative artifacts.",
        content: include_str!("../assets/protocols/gap-analysis.md"),
    },
];

/// Lists every embedded protocol.
#[must_use]
pub fn list() -> &'static [Protocol] {
    PROTOCOLS
}

/// Resolves one protocol by its exact selector.
#[must_use]
pub fn read(selector: &str) -> Option<Protocol> {
    PROTOCOLS
        .iter()
        .copied()
        .find(|protocol| protocol.selector == selector)
}
