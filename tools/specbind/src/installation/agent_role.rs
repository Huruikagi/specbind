//! Installation model for product-managed subagent roles and host rendering.
//!
//! Skills name stable, agent-neutral roles. The installer maps those roles to
//! each supported host's custom-agent files and applies only the project-owned
//! capability overrides stored in `.specbind.json`.

use serde::{Deserialize, Serialize};

/// Project overrides for the Codex rendering of one role.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RoleOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

/// Project overrides for the Claude Code rendering of one role.
///
/// A Claude Code subagent definition pins a model but exposes no
/// reasoning-effort field, so the accepted override surface stays narrower
/// than Codex rather than accepting a setting the rendering would drop.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClaudeRoleOverride {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Closed Codex role override surface in `.specbind.json`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CodexRoleOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planner: Option<RoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementer: Option<RoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<RoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debugger: Option<RoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub researcher: Option<RoleOverride>,
}

/// Closed Claude Code role override surface in `.specbind.json`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ClaudeRoleOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub planner: Option<ClaudeRoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementer: Option<ClaudeRoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<ClaudeRoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debugger: Option<ClaudeRoleOverride>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub researcher: Option<ClaudeRoleOverride>,
}

/// Agent-specific capability overrides, one closed surface per supported host.
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AgentRoleOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codex: Option<CodexRoleOverrides>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claude_code: Option<ClaudeRoleOverrides>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    None,
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl ReasoningEffort {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Xhigh => "xhigh",
            Self::Max => "max",
        }
    }
}

/// One stable role named by product-managed skills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentRole {
    pub selector: &'static str,
    pub default_model: &'static str,
    pub default_reasoning_effort: ReasoningEffort,
    /// Claude Code model selector. An alias keeps the rendering portable across
    /// model refreshes instead of pinning an identifier that ages out.
    pub default_claude_model: &'static str,
    description: &'static str,
    developer_instructions: &'static str,
}

static ROLES: &[AgentRole] = &[
    AgentRole {
        selector: "planner",
        default_model: "gpt-5.6-terra",
        default_reasoning_effort: ReasoningEffort::Medium,
        default_claude_model: "sonnet",
        description: "Use for fresh SpecBind planning-phase and contract-review dispatches.",
        developer_instructions: "Execute exactly the named SpecBind planning or contract-review skill from the task-specific prompt. Preserve its gate authority and stopping point, read the referenced artifacts yourself, make no implementation changes, and return the exact status requested by the dispatcher.",
    },
    AgentRole {
        selector: "implementer",
        default_model: "gpt-5.6-terra",
        default_reasoning_effort: ReasoningEffort::Medium,
        default_claude_model: "sonnet",
        description: "Use for one implementation or repair task dispatched by sb-implement.",
        developer_instructions: "Implement exactly one dispatched task. Treat its brief, artifact references, repository instructions, and task-implementation protocol as authoritative. Stay inside the assigned boundary, run the required verification, preserve unrelated changes, do not record task progress or create commits, and return the exact structured status requested by the dispatcher.",
    },
    AgentRole {
        selector: "reviewer",
        default_model: "gpt-5.6-terra",
        default_reasoning_effort: ReasoningEffort::Medium,
        default_claude_model: "sonnet",
        description: "Use for independent SpecBind task review and validation evidence dispatches.",
        developer_instructions: "Review only the dispatched subject from the supplied artifacts, diff, and protocol. Form an independent verdict, do not rely on the implementer's account, do not modify project files, and return the exact structured findings or status requested by the dispatcher.",
    },
    AgentRole {
        selector: "debugger",
        default_model: "gpt-5.6-sol",
        default_reasoning_effort: ReasoningEffort::High,
        default_claude_model: "opus",
        description: "Use for fresh-context root-cause diagnosis dispatched by SpecBind workflows.",
        developer_instructions: "Diagnose exactly one supplied failure from the stated inputs and debug protocol. Do not request or reconstruct failed-attempt history, do not modify project files or lifecycle state, and return the exact structured diagnosis requested by the dispatcher.",
    },
    AgentRole {
        selector: "researcher",
        default_model: "gpt-5.6-luna",
        default_reasoning_effort: ReasoningEffort::Medium,
        default_claude_model: "haiku",
        description: "Use for bounded read-only investigation dispatched by SpecBind skills.",
        developer_instructions: "Investigate only the bounded question in the task-specific prompt. Read the named repository evidence, make no project changes, return concise findings rather than raw file dumps, and leave synthesis to the dispatcher.",
    },
];

#[must_use]
pub fn all() -> &'static [AgentRole] {
    ROLES
}

impl AgentRole {
    #[must_use]
    pub fn name(self) -> String {
        format!("specbind-{}", self.selector)
    }

    #[must_use]
    pub fn target(self) -> String {
        format!(".codex/agents/{}.toml", self.name())
    }

    #[must_use]
    pub fn claude_target(self) -> String {
        format!(".claude/agents/{}.md", self.name())
    }

    #[must_use]
    pub fn override_from(self, overrides: Option<&CodexRoleOverrides>) -> Option<&RoleOverride> {
        let overrides = overrides?;
        match self.selector {
            "planner" => overrides.planner.as_ref(),
            "implementer" => overrides.implementer.as_ref(),
            "reviewer" => overrides.reviewer.as_ref(),
            "debugger" => overrides.debugger.as_ref(),
            "researcher" => overrides.researcher.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub fn claude_override_from(
        self,
        overrides: Option<&ClaudeRoleOverrides>,
    ) -> Option<&ClaudeRoleOverride> {
        let overrides = overrides?;
        match self.selector {
            "planner" => overrides.planner.as_ref(),
            "implementer" => overrides.implementer.as_ref(),
            "reviewer" => overrides.reviewer.as_ref(),
            "debugger" => overrides.debugger.as_ref(),
            "researcher" => overrides.researcher.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub fn render(self, overrides: Option<&CodexRoleOverrides>) -> String {
        let role_override = self.override_from(overrides);
        let model = role_override
            .and_then(|value| value.model.as_deref())
            .unwrap_or(self.default_model);
        let effort = role_override
            .and_then(|value| value.reasoning_effort)
            .unwrap_or(self.default_reasoning_effort);
        format!(
            "name = \"{}\"\ndescription = \"\"\"\n{}\n\"\"\"\nmodel = \"{model}\"\nmodel_reasoning_effort = \"{}\"\ndeveloper_instructions = \"\"\"\n{}\n\"\"\"\n",
            self.name(),
            self.description,
            effort.name(),
            self.developer_instructions
        )
    }

    /// Renders the Claude Code subagent definition for this role.
    ///
    /// The frontmatter omits `tools` so a dispatched role inherits the session
    /// toolset. Registration selects capability only; it grants no authority
    /// the dispatching skill did not already carry.
    #[must_use]
    pub fn render_claude(self, overrides: Option<&ClaudeRoleOverrides>) -> String {
        let model = self
            .claude_override_from(overrides)
            .and_then(|value| value.model.as_deref())
            .unwrap_or(self.default_claude_model);
        format!(
            "---\nname: {}\ndescription: {}\nmodel: {model}\n---\n\n{}\n",
            self.name(),
            self.description,
            self.developer_instructions
        )
    }
}

/// Model identifiers are rendered into TOML basic strings and YAML plain
/// scalars. Keep the accepted surface portable and injection-free rather than
/// attempting to enumerate a provider catalog that changes independently of
/// `SpecBind`.
#[must_use]
pub fn valid_model(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}
