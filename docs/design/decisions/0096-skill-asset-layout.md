# 0096: Author one agent-neutral source per product-managed skill

Status: Accepted

## Context

[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) fixes the v1 skill
set. [Decision 0008](./0008-customization-surface.md) makes skills
product-managed rather than a customization surface, and
[Decision 0077](./0077-v1-installation-distribution-and-migration.md) replaces
them with the current embedded versions when the repository guards permit. No
decision fixes how those assets are authored, stored, or rendered.

The inherited tree answers that question by duplication: each cc-sdd skill
exists as two complete files, one per agent. For `kiro-discovery` those are two
262-line documents that already differ across 136 diff lines. Some of that
divergence is deliberate — frontmatter fields and structural markup differ
between the platforms — but most of it is the same workflow prose maintained
twice.

That is the failure mode Decisions 0092 and 0094 were written to prevent. A
workflow obligation that exists in two copies drifts, and a fix applied to one
agent silently leaves the other behind. SpecBind already resolved the analogous
problem for shared semantics by moving them into embedded protocols; the same
reasoning applies to the skill text itself.

## Decision

### One authored source per skill

Each skill has exactly one source at
`tools/specbind/assets/skills/<skill-name>/SKILL.md`. The body is agent-neutral
English prose. There is no per-agent copy in the repository, and no agent-specific
branch inside the body.

The source begins with a neutral Front Matter block:

```yaml
---
name: specbind-status
description: Report current Spec and milestone state without changing anything.
argument-hint: "[spec]"
---
```

- `name` and `description` are required. `name` matches the Decision 0075 skill
  identity and the directory name.
- `argument-hint` is optional user-facing invocation guidance. It is retained
  only by renderers whose skill schema supports it.
- Skills are English-authored, like protocols and shared rules. A skill produces
  its user-facing output in the project's configured artifact language; the
  instructions it follows are not localized.

### Rendering per agent

Installation renders the one source into each selected agent's expected shape:

| Agent | Target | Front Matter emitted |
| --- | --- | --- |
| Claude Code | `.claude/skills/<name>/SKILL.md` | `name`, `description`, and `argument-hint` when present |
| Codex | `.agents/skills/<name>/SKILL.md` | `name`, `description` |

The Markdown body is written unchanged for both. Rendering maps declared intent
onto each platform's schema and never edits prose.

The renderer emits no permission grant or invocation restriction. In particular,
Claude Code `allowed-tools` is a permission pre-approval rather than descriptive
capability metadata, and `disable-model-invocation` changes discovery,
programmatic invocation, and subagent loading. Neither is inferred from the work
a skill describes. Skills run under the user's ordinary agent permissions and
the platform's normal skill invocation behavior. Any future product-managed
permission grant or deliberate per-agent invocation difference requires its own
accepted security and compatibility decision.

To keep the body genuinely neutral, it refers to another skill by bare name,
never with an invocation prefix, because the prefix differs by platform. A bare
name identifies the product workflow contract; it does not by itself claim that
the host can invoke another skill programmatically. Platform-specific subagent
or skill-invocation adapters remain separate installation surfaces. The body
names CLI commands and protocol selectors exactly as the CLI defines them.

### Skills are replaced, not merged

Skills are product-managed assets. An install refresh replaces the rendered file
with the current embedded version; it never merges, and a local edit to an
installed skill is not a supported customization path. Projects customize
through templates, shared rules, and steering.

Because replacement is a replacement, the Decision 0077 repository guard already
implemented for installation applies unchanged: a plan containing any
replacement requires at least one commit and a clean repository, so an
uncommitted local skill edit blocks the refresh instead of being overwritten
silently.

### Conformance is tested mechanically

A skill is prose, so the compiler cannot check its judgment. The failure mode
that matters is drift: the CLI renames a command, a protocol selector changes,
or a rule path moves, and the skill keeps naming the old one. That is
mechanically detectable and must be tested.

Every literal CLI invocation in a skill is written either as a standalone inline
code span or as one line in a shell code fence, beginning with the exact token
`specbind `. Metavariables use angle brackets and optional fragments use square
brackets. Conformance walks the real command graph and verifies the referenced
subcommand path and option names without treating those presentation
metavariables as runtime values. It does not execute the command.

The ordinary verification set checks that:

- every formatted `specbind ...` invocation references a real command route and
  only options accepted at that route
- every protocol selector a skill names is registered in the embedded protocol
  registry
- every `settings/rules/` path a skill loads is in the Decision 0093 installed
  set
- required Front Matter is present and `name` matches the directory
- installation plans and applies each skill to the accepted target for each
  selected agent

These tests belong to the ordinary verification set and run in CI.

Behavioral verification — whether an agent given the skill actually produces the
intended result — is not mechanically decidable and does not run in CI. It is
performed against a fixture project by a session with no prior context, and is
repeated when a skill changes materially. The procedure, its fixture builder, and
the accepted expectations per scenario are in
[Skill forward tests](../../skill-forward-tests.md).

### Scope

V1 renders skill documents only. Codex subagent definitions under
`.codex/agents/`, which the inherited tree carries for its review workflow, and
the marked project-instruction block accepted by Decision 0077 are separate
installation surfaces that this decision does not define.

[Decision 0129](./0129-agent-role-capability-adapters.md) later defines the
Codex subagent-definition surface while preserving this decision's one shared
skill body.

## Consequences

- One workflow obligation exists once, so a correction cannot reach one agent
  and miss the other.
- Adding an agent becomes a rendering concern rather than a request to duplicate
  and maintain seventeen more documents.
- The neutral body forces platform-specific behavior to be recognized and
  declared rather than written inline, which keeps the cross-agent contract
  visible.
- Drift between skills and the CLI becomes a failing test rather than a
  discovery made mid-workflow by a user.
- Deliberate per-agent divergence is no longer expressible in the body. If a
  real capability difference emerges, it is handled by declared rendering or by
  a later decision, not by forking the prose.

## Implementation status

Implemented for the skills embedded so far. Each of
`tools/specbind/assets/skills/specbind-status/SKILL.md` and
`tools/specbind/assets/skills/specbind-discovery/SKILL.md` is a single
agent-neutral source, and `specbind install` renders and writes each one to
`.claude/skills/` and `.agents/skills/` for every selected agent. The renderer
emits `name`, `description`, and Claude Code's `argument-hint`, writes the body
unchanged for both agents, and emits no permission grant or invocation
restriction.

The clap definitions moved to `tools/specbind/src/args.rs` so conformance can
walk the real command graph. Tests verify that every documented invocation
resolves to an existing route with only options that route accepts, that Front
Matter is complete and matches the directory, that both renderings share one
body, and that installation targets and refresh behavior hold. The drift check
was confirmed to fail on both a renamed command and an unknown option.

Adding the second skill exercised the check immediately: it rejected
`specbind template read spec brief`, whose `spec` scope is a literal positional
value rather than a subcommand. The resolver now stops walking at a leaf route,
and both drift cases were re-confirmed to fail afterwards.

Protocol-selector and shared-rule checks are implemented but currently match
nothing, because `specbind-status` consumes neither surface. They become
exercised with the first skill that does.
