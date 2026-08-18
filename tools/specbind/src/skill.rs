//! Embedded product-managed skills and their per-agent rendering.
//!
//! One agent-neutral source is authored per skill under `assets/skills/`. This
//! module parses its neutral Front Matter and renders the document each
//! supported agent expects. Rendering maps declared intent onto a platform
//! schema and never edits the body.
//!
//! The renderer emits no permission grant and no invocation restriction. Those
//! are security and discovery controls rather than descriptions of the work a
//! skill performs, so they are never inferred from skill content.

use std::fmt;

use crate::install::Agent;

/// One embedded product-managed skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skill {
    /// Skill identity, matching the source directory name.
    pub name: &'static str,
    source: &'static str,
}

/// The neutral Front Matter a skill source declares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub argument_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillError {
    pub code: &'static str,
    pub message: String,
}

impl fmt::Display for SkillError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SkillError {}

/// The complete embedded skill set.
static SKILLS: &[Skill] = &[
    Skill {
        name: "specbind-contract-review",
        source: include_str!("../assets/skills/specbind-contract-review/SKILL.md"),
    },
    Skill {
        name: "specbind-debug",
        source: include_str!("../assets/skills/specbind-debug/SKILL.md"),
    },
    Skill {
        name: "specbind-design",
        source: include_str!("../assets/skills/specbind-design/SKILL.md"),
    },
    Skill {
        name: "specbind-discovery",
        source: include_str!("../assets/skills/specbind-discovery/SKILL.md"),
    },
    Skill {
        name: "specbind-implement",
        source: include_str!("../assets/skills/specbind-implement/SKILL.md"),
    },
    Skill {
        name: "specbind-requirements",
        source: include_str!("../assets/skills/specbind-requirements/SKILL.md"),
    },
    Skill {
        name: "specbind-review-task",
        source: include_str!("../assets/skills/specbind-review-task/SKILL.md"),
    },
    Skill {
        name: "specbind-status",
        source: include_str!("../assets/skills/specbind-status/SKILL.md"),
    },
    Skill {
        name: "specbind-tasks",
        source: include_str!("../assets/skills/specbind-tasks/SKILL.md"),
    },
];

/// Lists every embedded skill.
#[must_use]
pub fn all() -> &'static [Skill] {
    SKILLS
}

impl Skill {
    /// Returns the authored source, Front Matter included.
    #[must_use]
    pub fn source(self) -> &'static str {
        self.source
    }

    /// Returns the agent-neutral Markdown body.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic when the source has no parseable Front Matter.
    pub fn body(self) -> Result<&'static str, SkillError> {
        self.split().map(|(_, body)| body)
    }

    /// Parses the declared neutral metadata.
    ///
    /// # Errors
    ///
    /// Returns a diagnostic when Front Matter is missing, unparseable, or does
    /// not declare the required identity.
    pub fn metadata(self) -> Result<SkillMetadata, SkillError> {
        let (frontmatter, _) = self.split()?;
        let value: serde_json::Value =
            serde_saphyr::from_str(frontmatter).map_err(|error| SkillError {
                code: "SKILL_FRONTMATTER_INVALID",
                message: format!("{}: {error}", self.name),
            })?;
        let field = |key: &str| {
            value
                .get(key)
                .and_then(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
        };
        let (Some(name), Some(description)) = (field("name"), field("description")) else {
            return Err(SkillError {
                code: "SKILL_FRONTMATTER_INCOMPLETE",
                message: format!("{}: name and description are required", self.name),
            });
        };
        if name != self.name {
            return Err(SkillError {
                code: "SKILL_NAME_MISMATCH",
                message: format!("{}: declared name is {name}", self.name),
            });
        }
        Ok(SkillMetadata {
            name,
            description,
            argument_hint: field("argument-hint"),
        })
    }

    /// Renders this skill for one agent.
    ///
    /// # Errors
    ///
    /// Returns the source diagnostic when the skill cannot be parsed.
    pub fn render(self, agent: Agent) -> Result<String, SkillError> {
        let metadata = self.metadata()?;
        let body = self.body()?;
        let hint = match (agent, &metadata.argument_hint) {
            (Agent::ClaudeCode, Some(hint)) => format!("argument-hint: \"{hint}\"\n"),
            _ => String::new(),
        };
        Ok(format!(
            "---\nname: {}\ndescription: {}\n{hint}---\n{body}",
            metadata.name, metadata.description
        ))
    }

    /// Returns the project-relative install target for one agent.
    #[must_use]
    pub fn target(self, agent: Agent) -> String {
        let root = match agent {
            Agent::ClaudeCode => ".claude/skills",
            Agent::Codex => ".agents/skills",
        };
        format!("{root}/{}/SKILL.md", self.name)
    }

    fn split(self) -> Result<(&'static str, &'static str), SkillError> {
        let rest = self
            .source
            .strip_prefix("---\n")
            .ok_or_else(|| SkillError {
                code: "SKILL_FRONTMATTER_MISSING",
                message: format!("{}: source must open with Front Matter", self.name),
            })?;
        let end = rest.find("\n---\n").ok_or_else(|| SkillError {
            code: "SKILL_FRONTMATTER_MISSING",
            message: format!("{}: Front Matter is not terminated", self.name),
        })?;
        Ok((&rest[..end], &rest[end + "\n---\n".len()..]))
    }
}

/// Resolves one skill by name.
#[must_use]
pub fn find(name: &str) -> Option<Skill> {
    SKILLS.iter().copied().find(|skill| skill.name == name)
}
