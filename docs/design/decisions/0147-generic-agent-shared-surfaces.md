# 0147: Install generic agents through shared Skills and instructions

Status: Accepted

## Context

SpecBind currently selects `claude-code` or `codex` as an installation host.
That closed choice was useful while each host required a distinct Skill path,
root instruction file, and subagent definition. The portable surfaces have since
converged beyond those two products.

The Agent Skills specification defines a portable `SKILL.md` package. Its client
implementation guide identifies project-local `.agents/skills/` as the widely
adopted cross-client interoperability location, while leaving installation paths
outside the core format. Separately, `AGENTS.md` is an open plain-Markdown
instruction convention implemented by Codex and other major coding agents.

Codex already receives SpecBind's neutral Skill rendering under
`.agents/skills/` and the Decision 0099 marked block in root `AGENTS.md`. A
generic integration can therefore reuse those two exact surfaces. It cannot
reuse `.codex/agents/`: subagent definitions, capabilities, and dispatch syntax
remain host-specific and have no accepted portable format.

This creates shared ownership. Decision 0141 removes the selected host's exact
targets, but deleting Codex's Skills or instruction block would be wrong when a
generic profile still requires the same paths. Storing reference counts would
duplicate information already present in the selected Agent set.

## Decision

### Generic profile

`generic` is a supported closed Agent value:

```text
specbind install --agent generic
specbind remove-agent generic
```

It installs:

- every current product-managed Skill at
  `.agents/skills/<skill-name>/SKILL.md`, using the portable rendering that emits
  required `name` and `description` Front Matter and preserves the neutral body;
- the Decision 0099 marked block in root `AGENTS.md` when
  `projectInstructions` is enabled.

It installs no command package, invocation shim, role definition, model default,
or `agentRoles` configuration. The profile promises only these shared Agent
Skills and `AGENTS.md` surfaces. It does not claim that every coding agent
discovers either convention or implements equivalent runtime behavior.

### Required surfaces are a union

Each selected Agent requires these derived surfaces:

| Agent | Skills | Project instructions | Roles |
| --- | --- | --- | --- |
| `generic` | `.agents/skills/` | `AGENTS.md` | none |
| `codex` | `.agents/skills/` | `AGENTS.md` | `.codex/agents/` |
| `claude-code` | `.claude/skills/` | `CLAUDE.md` | `.claude/agents/` |

Installation materializes the union of the selected Agents' exact derived
targets. When `generic` and `codex` are both selected, each shared Skill and the
one marked `AGENTS.md` block appears once in the plan and on disk. Codex roles
remain present because they are required only by Codex.

The selected `agents` array remains the complete ownership input. No reference
count, implicit Agent, or additional shared-surface state is persisted.

### Removal uses the remaining union

For one-Agent removal, the CLI first computes the Agent set that will remain. It
retains an exact managed target when any remaining Agent requires the same path,
and removes it only when none does. The read-only plan reports shared targets as
`retain`, so a configuration-only generic removal is explicit rather than an
empty unexplained file plan.

Therefore:

- removing `codex` while `generic` remains retains `.agents/skills/` and the
  marked `AGENTS.md` block and removes `.codex/agents/`;
- removing `generic` while `codex` remains retains all shared targets and only
  updates configuration;
- removing `generic` while only `claude-code` remains removes the exact managed
  Skill targets under `.agents/skills/` and the marked `AGENTS.md` block while
  retaining every Claude Code surface and any unrelated container content.

Project uninstall uses the same union and removes each shared exact target once.
Decision 0141's plan-by-default command boundary, final-Agent rejection,
config-last completion marker, exact catalog ownership, Git and filesystem
guards, retry behavior, and explicit durable-knowledge policy remain unchanged.

### Documentation boundary

The user guide describes `generic` as cross-client compatibility, names its two
installed surfaces, and states that it provides no subagent roles. It does not
maintain a product compatibility matrix whose entries can drift independently
of upstream clients. The guide points users to their agent's documentation to
confirm discovery of `.agents/skills/` and `AGENTS.md`.

## Consequences

- Users of compatible agents can install SpecBind without selecting Codex as a
  proxy host.
- Codex and generic clients can coexist without duplicate plan entries or
  destructive removal of their shared assets.
- The generic contract stays narrow enough to verify mechanically from paths,
  rendered bytes, configuration, and removal plans.
- Generic agents receive no optimized role adapters. A future portable role or
  dispatch standard requires a separate accepted decision.
- An agent that recognizes only one of the two conventions receives only that
  portion of the integration; SpecBind does not synthesize proprietary fallback
  configuration.

## Implementation status

Implemented by the closed Agent parser, installation path deduplication,
remaining-Agent removal planning, focused Git-fixture tests, and the public
Japanese guides.
