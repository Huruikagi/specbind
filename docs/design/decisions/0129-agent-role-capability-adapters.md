# 0129: Install agent-role capability adapters with project overrides

Status: Accepted

## Context

[Decision 0109](./0109-subagent-dispatch-contract.md) makes fresh-context
dispatch a product obligation and deliberately defers registered roles. It also
records the intended later use: pinning a Codex model and reasoning effort per
role without changing the self-contained brief or protocol selector.

That optimization is now needed. Implementation, review, diagnosis, planning,
and bounded research have different capability requirements. Running all of
them on the session's strongest model spends unnecessarily, while putting a
cheap model everywhere weakens the independent judgments for which dispatch
exists.

The free-form project adapters under Decision 0101 are not the right mechanism.
They are agent-interpreted operational policy and intentionally are not machine
syntax. Model selection must reach the host deterministically before a
subagent starts.

## Decision

### Stable neutral roles

Product-managed skills may name these registered roles:

| Role | Work | Codex default |
| --- | --- | --- |
| `specbind-planner` | planning phases and contract review | `gpt-5.6-terra`, `medium` |
| `specbind-implementer` | one implementation or repair task | `gpt-5.6-terra`, `medium` |
| `specbind-reviewer` | independent task review and validation evidence | `gpt-5.6-terra`, `medium` |
| `specbind-debugger` | fresh root-cause diagnosis | `gpt-5.6-sol`, `high` |
| `specbind-researcher` | bounded read-only investigation | `gpt-5.6-luna`, `medium` |

The names describe semantic roles, not a host mechanism. A skill uses the
registered role when the host provides it and otherwise dispatches an ordinary
fresh subagent with the same brief and protocol. Registration changes
capability only; it grants no scope, mutation, gate, Git, or approval authority.

This is the declared role-registration extension reserved by Decisions 0096
and 0109. The skill body stays shared across agents. A supported agent without
an installed rendering follows the fallback rather than receiving a forked
skill body.

### Product semantics, project capability

SpecBind owns each role's name, description, and developer instructions. Those
parts are rendered product assets and are refreshed like skills. A project may
override only `model` and `reasoningEffort`, under the optional configuration:

```json
{
  "agentRoles": {
    "codex": {
      "implementer": {
        "model": "gpt-5.6-luna",
        "reasoningEffort": "medium"
      }
    }
  }
}
```

Omitted roles and omitted fields use the current product defaults. This keeps
defaults refreshable instead of copying them into every project configuration.
The role and agent sets are closed. Reasoning effort is one of `none`, `low`,
`medium`, `high`, `xhigh`, or `max`; model identifiers are non-empty portable
tokens rather than a hard-coded provider catalog.

An override is policy, not availability proof. The host remains responsible
for whether the configured model is available to the user. A model-start
failure is reported as an environment or configuration failure; a skill does
not silently change the configured capability.

### Codex rendering and ownership

When Codex is selected, `specbind install` renders:

```text
.codex/agents/specbind-planner.toml
.codex/agents/specbind-implementer.toml
.codex/agents/specbind-reviewer.toml
.codex/agents/specbind-debugger.toml
.codex/agents/specbind-researcher.toml
```

These files are derived product-managed assets. Local edits are not the
customization surface and are replaced only under Decision 0077's committed,
clean-repository guard. Projects edit `.specbind.json` and run the installer.
Other agents may gain renderings later without changing role names or skill
bodies.

The installed developer instructions remain deliberately small. Task-specific
scope, artifact paths, acceptance conditions, verification commands, and the
protocol selector continue to travel in every dispatch brief under Decision
0109. Registration must not become hidden workflow logic.

## Consequences

- Common implementation and review work defaults to Terra, while bounded
  research can use Luna and rare diagnosis retains Sol.
- Projects can tune cost and capability without editing product-managed skills
  or role semantics.
- Free-form operational adapters remain separate from deterministic host
  configuration.
- Model availability can still vary by account and host, so an accepted setting
  can fail at dispatch time without invalidating SpecBind lifecycle state.

## Implementation status

Implemented. The Rust installer validates optional role overrides, installs the
five Codex role files, refreshes them under the product-asset repository guard,
and the dispatching skills select the applicable role with an explicit
fresh-subagent fallback. Skill conformance rejects an unknown registered role
and requires every installed role to have a consumer.
