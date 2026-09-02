# 0183: Render branded Codex Skill interface metadata

Status: Accepted

## Context

[Decision 0096](./0096-skill-asset-layout.md) gives every product-managed Skill
one agent-neutral `SKILL.md` and renders only the Front Matter supported by each
Agent. [Decision 0176](./0176-skill-namespace-separation.md) makes the compact
`sb-*` identifier the installed Skill identity. That identifier is appropriate
for explicit invocation, but it does not present the SpecBind product name in a
human-facing Codex Skill list.

[OpenAI Skill packages](https://developers.openai.com/codex/skills) can carry
`agents/openai.yaml` separately from `SKILL.md`. Its `interface` fields describe
presentation and an example prompt without changing the Skill identity or
workflow instructions.

## Decision

- Every product-managed Skill keeps the same `sb-*` directory and `SKILL.md`
  `name`. The identifier remains the exact explicit invocation name.
- The embedded Skill catalog declares three Codex interface values for every
  Skill:
  - a unique `display_name` beginning with `SpecBind `;
  - a `short_description` containing 25 to 64 characters; and
  - a one-sentence `default_prompt` naming the exact `$sb-*` Skill.
- When Codex is selected, installation renders those values to
  `.agents/skills/<name>/agents/openai.yaml` beneath an `interface` mapping.
  The file is a product-managed Skill target and participates in guarded
  install, refresh, and removal like `SKILL.md` and packaged references.
- Claude Code does not receive this OpenAI-specific file. The generic Agent
  profile remains portable and also does not receive it. When Codex and generic
  are selected together, the shared `.agents/skills/<name>/SKILL.md` remains
  deduplicated while Codex adds the interface file.
- No icon path or `brand_color` is emitted until SpecBind owns an accepted
  reusable icon and color contract. No `policy` is emitted, so normal implicit
  selection remains unchanged. No `dependencies` are emitted because the
  Skills have no universal MCP dependency.

## Consequences

- Codex can show `SpecBind Plan`, `SpecBind Status`, and corresponding branded
  names while `$sb-plan`, `$sb-status`, and the rest remain stable identifiers.
- The default prompt is a discoverability aid rather than a second workflow
  contract; the authoritative instructions remain in `SKILL.md`.
- Adding icons, brand color, invocation policy, or MCP dependencies requires
  evidence for that field rather than speculative metadata.

## Verification

Mechanical tests parse every rendered `openai.yaml`, enforce the display-name,
description-length, and exact-invocation invariants, prove that only Codex
receives the file, and exercise installation and removal targets.
