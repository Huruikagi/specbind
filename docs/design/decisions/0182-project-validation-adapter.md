# 0182: Add project-specific final validation guidance

Status: Accepted

## Context

Decision 0112 requires `sb-validate-implementation` to derive and run the
canonical project commands and to assess runtime liveness, active Requirement
coverage, integration, Design alignment, and blocked work. Repositories also
have completion checks that are operational rather than canonical commands:
starting a service with a particular fixture, exercising a browser or device,
reading an external system through an available connector, comparing a rendered
result with a fixed reference, or performing a named manual observation.

Requirements and Design own what the implementation must satisfy. Shared Rules
own project-adjustable judgment and authoring policy. Neither is the right place
for environment-specific instructions describing how the final validator obtains
additional evidence. Inferring those procedures afresh from the repository on
every validation is incomplete and makes an unavailable capability easy to
silently skip.

Decision 0101 keeps the adapter directory a closed product catalog of free-form,
agent-interpreted operational guidance. A new adapter therefore needs an exact
selector and profile, owning consumer, absence semantics, installation behavior,
authority boundary, and conflict rule.

## Decision

### Profile and installation

The accepted adapter catalog adds:

| Selector | Path | OKF type | Presence |
| --- | --- | --- | --- |
| `validation` | `settings/adapters/validation.md` | `SpecBind Validation Adapter` | optional at runtime |

`specbind install` creates a localized inactive scaffold when the project copy
is absent and never overwrites an existing project-owned copy. The exact
`<!-- specbind:adapter-scaffold -->` marker classifies the whole body as inactive.
Absence, a scaffold, or Front Matter with an empty body means that the project
adds no procedure beyond the mandatory product protocol and the canonical checks
established by its repository. None of those states weakens ordinary validation.

The body is free-form Markdown. It may describe applicability, setup, fixtures,
environment, account roles, devices, commands, browser interaction, connected
tools such as MCP servers, manual observations, observable success and failure,
and cleanup. Headings and code blocks are not machine syntax or automatically
executable hooks.

### One final-validation consumer

`sb-validate-implementation` is the initial and required consumer. After its
read-only completion preflight and before it fixes the complete required check
set, it reads:

```text
specbind adapter read validation --for consume
```

`ADAPTER_ABSENT` and `ADAPTER_SCAFFOLD` add no project procedure. Every
applicable procedure in returned active guidance joins the required set for that
run. Adapter guidance supplements the completion-verification protocol,
Requirements, Design, and canonical project commands. It cannot replace, waive,
narrow, or declare any of them passed. A material ambiguity about applicability,
the required action, or the observable result prevents verification rather than
being guessed away.

An observed mismatch or failed required step contributes to `NO-GO`. A mandatory
step that cannot be performed because its command, environment, credential,
device, manual observer, or connected tool is unavailable produces
`MANUAL_VERIFY_REQUIRED`. It is never silently omitted or replaced with a weaker
check. Only the existing complete synthesis may return `GO`.

`sb-implement` does not consume this adapter as a hidden implementation recipe.
Implementation remains grounded in canonical artifacts and its task-local
verification, while the final validator independently applies the additional
project procedure.

### Evidence representation

Decision 0033 remains unchanged. When an adapter procedure is an exact command
that actually ran and returned zero, the validator may preserve it as a
`custom` mechanical check. Browser, device, connected-tool, and manual
observations without an exact command remain run-scoped semantic evidence. The
validator does not invent a command, persisted pass flag, or durable external
success record to represent them.

This distinction keeps the completion schema concise while still making every
required observation part of the `GO` judgment. A later need for durable typed
non-command observations requires its own evidence-schema decision.

### Configuration and authority

`sb-configure` owns proposal and maintenance. It may inspect repository evidence
such as scripts, CI, runtime documentation, browser or device setup, fixtures,
and existing external-tool integration, then present a complete replacement for
a scaffold or an exact diff for active guidance. It never invents commands,
credentials, environments, destinations, tool availability, or success evidence.

The adapter records policy but grants no credential use, external mutation,
source edit, finding repair, or broader user authority. The validator may perform
setup and cleanup already authorized by the request and tool boundary, but it
cannot repair a finding it will judge. Requirements, Design, the immutable
product protocol, canonical repository checks, and normal safety boundaries win
every conflict.

A committed Validation adapter change is ordinary project content. Existing
completion evidence becomes stale through the current implementation-revision
freshness rule; no adapter-specific invalidation state is added.

## Consequences

- Projects have one supported place for additional final implementation
  validation procedures across commands, browsers, devices, connected tools,
  and manual observations.
- Tool-specific capabilities remain in the agent host rather than becoming Rust
  CLI dependencies or a generic plugin loader.
- An unavailable project-required capability is visible as cannot-verify
  evidence instead of being silently skipped.
- Project guidance cannot weaken product validation or turn prose into authority.
- Completion evidence remains compact; non-command semantic observations are
  mandatory for the run but not persisted as synthetic mechanical checks.

## Verification

Rust catalog, installation, configuration, and CLI tests cover the fourth closed
selector, localized inactive scaffolds, state reporting, preservation of
project-owned content, and raw reads. Skill contract tests require both
`sb-configure` proposal boundaries and the `sb-validate-implementation` read,
verdict mapping, evidence distinction, and no-repair boundary. A fresh forward
test must show that an active project procedure is included in the required set
and that an unavailable required capability yields `MANUAL_VERIFY_REQUIRED`
without completion evidence.
