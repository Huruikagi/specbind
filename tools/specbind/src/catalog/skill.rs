//! Catalog of product-managed skills and their per-agent rendering.
//!
//! One agent-neutral package is authored per skill under `assets/skills/`.
//! This module parses the entrypoint's neutral Front Matter, renders the
//! document each supported agent expects, and carries any declared reference
//! files byte-for-byte beside it.
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
    /// Human-facing name shown by `OpenAI` Skill interfaces.
    pub display_name: &'static str,
    /// Compact human-facing description shown by `OpenAI` Skill interfaces.
    pub short_description: &'static str,
    /// Example prompt inserted by `OpenAI` Skill interfaces.
    pub default_prompt: &'static str,
    source: &'static str,
}

/// One product-managed file carried beside a skill entrypoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillResource {
    /// Portable path relative to the skill package root.
    pub relative_path: &'static str,
    source: &'static str,
}

impl SkillResource {
    /// Returns the exact agent-neutral resource content.
    #[must_use]
    pub fn content(self) -> &'static str {
        self.source
    }
}

/// One rendered package file and its project-relative install target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedSkillFile {
    pub target: String,
    pub content: String,
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
        name: "sb-contract-review",
        display_name: "SpecBind Contract Review",
        short_description: "Review the milestone contract graph.",
        default_prompt: "Use $sb-contract-review to review the active milestone contract graph.",
        source: include_str!("../../assets/skills/sb-contract-review/SKILL.md"),
    },
    Skill {
        name: "sb-configure",
        display_name: "SpecBind Configure",
        short_description: "Configure a SpecBind project safely.",
        default_prompt: "Use $sb-configure to review and update this project's SpecBind configuration.",
        source: include_str!("../../assets/skills/sb-configure/SKILL.md"),
    },
    Skill {
        name: "sb-debug",
        display_name: "SpecBind Debug",
        short_description: "Diagnose why a SpecBind workflow stopped.",
        default_prompt: "Use $sb-debug to diagnose why the current SpecBind workflow stopped.",
        source: include_str!("../../assets/skills/sb-debug/SKILL.md"),
    },
    Skill {
        name: "sb-discovery",
        display_name: "SpecBind Discovery",
        short_description: "Discover and classify milestone work.",
        default_prompt: "Use $sb-discovery to discover and classify the requested milestone work.",
        source: include_str!("../../assets/skills/sb-discovery/SKILL.md"),
    },
    Skill {
        name: "sb-drive",
        display_name: "SpecBind Drive",
        short_description: "Drive safe milestone work to release readiness.",
        default_prompt: "Use $sb-drive to advance the active milestone through safe reachable work.",
        source: include_str!("../../assets/skills/sb-drive/SKILL.md"),
    },
    Skill {
        name: "sb-gap-analysis",
        display_name: "SpecBind Gap Analysis",
        short_description: "Compare planned work with the repository.",
        default_prompt: "Use $sb-gap-analysis to compare the intended work with this repository.",
        source: include_str!("../../assets/skills/sb-gap-analysis/SKILL.md"),
    },
    Skill {
        name: "sb-implement",
        display_name: "SpecBind Implement",
        short_description: "Implement one planned milestone item.",
        default_prompt: "Use $sb-implement to implement one planned item from the active milestone.",
        source: include_str!("../../assets/skills/sb-implement/SKILL.md"),
    },
    Skill {
        name: "sb-plan",
        display_name: "SpecBind Plan",
        short_description: "Plan Spec work through approval gates.",
        default_prompt: "Use $sb-plan to plan the active Spec work through its approval gates.",
        source: include_str!("../../assets/skills/sb-plan/SKILL.md"),
    },
    Skill {
        name: "sb-release",
        display_name: "SpecBind Release",
        short_description: "Release and finalize an active milestone.",
        default_prompt: "Use $sb-release to release and finalize the active milestone.",
        source: include_str!("../../assets/skills/sb-release/SKILL.md"),
    },
    Skill {
        name: "sb-review-task",
        display_name: "SpecBind Task Review",
        short_description: "Review one implemented task independently.",
        default_prompt: "Use $sb-review-task to independently review one implemented task.",
        source: include_str!("../../assets/skills/sb-review-task/SKILL.md"),
    },
    Skill {
        name: "sb-status",
        display_name: "SpecBind Status",
        short_description: "Explain lifecycle state and next steps.",
        default_prompt: "Use $sb-status to explain the current lifecycle state and next available action.",
        source: include_str!("../../assets/skills/sb-status/SKILL.md"),
    },
    Skill {
        name: "sb-steering",
        display_name: "SpecBind Steering",
        short_description: "Maintain durable project steering.",
        default_prompt: "Use $sb-steering to establish or update this project's durable guidance.",
        source: include_str!("../../assets/skills/sb-steering/SKILL.md"),
    },
    Skill {
        name: "sb-validate-design",
        display_name: "SpecBind Design Validation",
        short_description: "Validate a SpecBind design independently.",
        default_prompt: "Use $sb-validate-design to independently validate the active Spec design.",
        source: include_str!("../../assets/skills/sb-validate-design/SKILL.md"),
    },
    Skill {
        name: "sb-validate-implementation",
        display_name: "SpecBind Implementation Validation",
        short_description: "Validate implementation against requirements.",
        default_prompt: "Use $sb-validate-implementation to validate the active Spec implementation.",
        source: include_str!("../../assets/skills/sb-validate-implementation/SKILL.md"),
    },
    Skill {
        name: "sb-verify-completion",
        display_name: "SpecBind Completion Verification",
        short_description: "Verify an explicit completion claim.",
        default_prompt: "Use $sb-verify-completion to verify this explicit completion claim.",
        source: include_str!("../../assets/skills/sb-verify-completion/SKILL.md"),
    },
];

static RETIRED_SKILL_NAMES: &[&str] = &[
    "specbind-contract-review",
    "specbind-configure",
    "specbind-debug",
    "specbind-discovery",
    "specbind-drive",
    "specbind-gap-analysis",
    "specbind-implement",
    "specbind-plan",
    "specbind-release",
    "specbind-review-task",
    "specbind-status",
    "specbind-steering",
    "specbind-validate-design",
    "specbind-validate-implementation",
    "specbind-verify-completion",
    "specbind-adopt-existing",
    "specbind-plan-design",
    "specbind-plan-requirements",
    "specbind-plan-tasks",
];

static RETIRED_ENTRYPOINT_ONLY_FILES: &[&str] = &["SKILL.md"];
static RETIRED_ADOPT_EXISTING_FILES: &[&str] =
    &["references/resume.md", "references/start.md", "SKILL.md"];
static RETIRED_CONFIGURE_FILES: &[&str] = &[
    "references/adapters.md",
    "references/aftercare.md",
    "references/installation-and-agents.md",
    "references/rules.md",
    "references/steering.md",
    "references/templates-and-reconciliation.md",
    "references/update.md",
    "SKILL.md",
];
static RETIRED_DISCOVERY_FILES: &[&str] = &[
    "references/adopt-resume.md",
    "references/adopt-start.md",
    "references/github-milestone.md",
    "references/local-files.md",
    "references/ordinary.md",
    "references/reverse.md",
    "SKILL.md",
];
static RETIRED_IMPLEMENT_FILES: &[&str] = &[
    "references/direct.md",
    "references/spec-backed.md",
    "SKILL.md",
];
static RETIRED_PLAN_FILES: &[&str] = &[
    "references/design.md",
    "references/requirements.md",
    "references/tasks.md",
    "SKILL.md",
];
static RETIRED_RELEASE_FILES: &[&str] = &["references/bootstrap-release-adapter.md", "SKILL.md"];

static CONFIGURE_RESOURCES: &[SkillResource] = &[
    SkillResource {
        relative_path: "references/adapters.md",
        source: include_str!("../../assets/skills/sb-configure/references/adapters.md"),
    },
    SkillResource {
        relative_path: "references/aftercare.md",
        source: include_str!("../../assets/skills/sb-configure/references/aftercare.md"),
    },
    SkillResource {
        relative_path: "references/installation-and-agents.md",
        source: include_str!(
            "../../assets/skills/sb-configure/references/installation-and-agents.md"
        ),
    },
    SkillResource {
        relative_path: "references/rules.md",
        source: include_str!("../../assets/skills/sb-configure/references/rules.md"),
    },
    SkillResource {
        relative_path: "references/steering.md",
        source: include_str!("../../assets/skills/sb-configure/references/steering.md"),
    },
    SkillResource {
        relative_path: "references/templates-and-reconciliation.md",
        source: include_str!(
            "../../assets/skills/sb-configure/references/templates-and-reconciliation.md"
        ),
    },
    SkillResource {
        relative_path: "references/update.md",
        source: include_str!("../../assets/skills/sb-configure/references/update.md"),
    },
];

static DISCOVERY_RESOURCES: &[SkillResource] = &[
    SkillResource {
        relative_path: "references/github-milestone.md",
        source: include_str!("../../assets/skills/sb-discovery/references/github-milestone.md"),
    },
    SkillResource {
        relative_path: "references/local-files.md",
        source: include_str!("../../assets/skills/sb-discovery/references/local-files.md"),
    },
    SkillResource {
        relative_path: "references/ordinary.md",
        source: include_str!("../../assets/skills/sb-discovery/references/ordinary.md"),
    },
    SkillResource {
        relative_path: "references/reverse.md",
        source: include_str!("../../assets/skills/sb-discovery/references/reverse.md"),
    },
];

static DISCOVERY_RETIRED_RESOURCES: &[&str] =
    &["references/adopt-resume.md", "references/adopt-start.md"];

static IMPLEMENT_RESOURCES: &[SkillResource] = &[
    SkillResource {
        relative_path: "references/direct.md",
        source: include_str!("../../assets/skills/sb-implement/references/direct.md"),
    },
    SkillResource {
        relative_path: "references/spec-backed.md",
        source: include_str!("../../assets/skills/sb-implement/references/spec-backed.md"),
    },
];

static PLAN_RESOURCES: &[SkillResource] = &[
    SkillResource {
        relative_path: "references/design.md",
        source: include_str!("../../assets/skills/sb-plan/references/design.md"),
    },
    SkillResource {
        relative_path: "references/requirements.md",
        source: include_str!("../../assets/skills/sb-plan/references/requirements.md"),
    },
    SkillResource {
        relative_path: "references/tasks.md",
        source: include_str!("../../assets/skills/sb-plan/references/tasks.md"),
    },
];

static RELEASE_RESOURCES: &[SkillResource] = &[SkillResource {
    relative_path: "references/bootstrap-release-adapter.md",
    source: include_str!("../../assets/skills/sb-release/references/bootstrap-release-adapter.md"),
}];

/// Lists every embedded skill.
#[must_use]
pub fn all() -> &'static [Skill] {
    SKILLS
}

/// Lists exact former product-managed Skill identities removed on refresh.
#[must_use]
pub fn retired_names() -> &'static [&'static str] {
    RETIRED_SKILL_NAMES
}

/// Lists the exact former product-managed files removed for one retired Skill.
#[must_use]
pub fn retired_files(name: &str) -> &'static [&'static str] {
    match name {
        "specbind-adopt-existing" => RETIRED_ADOPT_EXISTING_FILES,
        "specbind-configure" => RETIRED_CONFIGURE_FILES,
        "specbind-discovery" => RETIRED_DISCOVERY_FILES,
        "specbind-implement" => RETIRED_IMPLEMENT_FILES,
        "specbind-plan" => RETIRED_PLAN_FILES,
        "specbind-release" => RETIRED_RELEASE_FILES,
        _ => RETIRED_ENTRYPOINT_ONLY_FILES,
    }
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

    /// Returns every declared file in this neutral package.
    #[must_use]
    pub fn resources(self) -> &'static [SkillResource] {
        match self.name {
            "sb-configure" => CONFIGURE_RESOURCES,
            "sb-discovery" => DISCOVERY_RESOURCES,
            "sb-implement" => IMPLEMENT_RESOURCES,
            "sb-plan" => PLAN_RESOURCES,
            "sb-release" => RELEASE_RESOURCES,
            _ => &[],
        }
    }

    /// Lists exact former product-managed resources removed from this active
    /// Skill package on refresh.
    #[must_use]
    pub fn retired_resources(self) -> &'static [&'static str] {
        match self.name {
            "sb-discovery" => DISCOVERY_RETIRED_RESOURCES,
            _ => &[],
        }
    }

    /// Renders the entrypoint and carries declared resources for one Agent.
    ///
    /// # Errors
    ///
    /// Returns the entrypoint Front Matter diagnostic when rendering fails.
    pub fn render_files(self, agent: Agent) -> Result<Vec<RenderedSkillFile>, SkillError> {
        let mut files = vec![RenderedSkillFile {
            target: self.target(agent),
            content: self.render(agent)?,
        }];
        files.extend(self.resources().iter().map(|resource| RenderedSkillFile {
            target: self.resource_target(agent, resource.relative_path),
            content: resource.source.to_owned(),
        }));
        if agent == Agent::Codex {
            files.push(RenderedSkillFile {
                target: self.resource_target(agent, "agents/openai.yaml"),
                content: self.render_openai_yaml(),
            });
        }
        Ok(files)
    }

    /// Returns every exact install target owned by this package for one Agent.
    #[must_use]
    pub fn targets(self, agent: Agent) -> Vec<String> {
        let mut targets = std::iter::once(self.target(agent))
            .chain(
                self.resources()
                    .iter()
                    .map(|resource| self.resource_target(agent, resource.relative_path)),
            )
            .collect::<Vec<_>>();
        if agent == Agent::Codex {
            targets.push(self.resource_target(agent, "agents/openai.yaml"));
        }
        targets
    }

    /// Returns the project-relative install target for one agent.
    #[must_use]
    pub fn target(self, agent: Agent) -> String {
        let root = match agent {
            Agent::ClaudeCode => ".claude/skills",
            Agent::Codex | Agent::Generic => ".agents/skills",
        };
        format!("{root}/{}/SKILL.md", self.name)
    }

    fn resource_target(self, agent: Agent, relative_path: &str) -> String {
        let entrypoint = self.target(agent);
        let package_root = entrypoint
            .strip_suffix("/SKILL.md")
            .expect("skill entrypoint always ends in /SKILL.md");
        format!("{package_root}/{relative_path}")
    }

    fn render_openai_yaml(self) -> String {
        format!(
            "interface:\n  display_name: \"{}\"\n  short_description: \"{}\"\n  default_prompt: \"{}\"\n",
            self.display_name, self.short_description, self.default_prompt
        )
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
