# 0176: Separate the Skill namespace from the CLI name

Status: Accepted

Supersedes the installed-Skill naming portions of [Decision 0075](./0075-v1-skill-and-orchestration-scope.md), [Decision 0161](./0161-default-plan-and-phase-skill-namespace.md), [Decision 0168](./0168-milestone-drive-orchestrator.md), [Decision 0174](./0174-plan-phase-procedures-as-references.md), and [Decision 0175](./0175-existing-adoption-as-discovery-references.md). Their workflow and ownership contracts remain authoritative.

## Context

The `specbind` executable and installed Skill names such as `specbind-plan`
share the same visible prefix. Although a hyphen distinguishes the Skill
identifier structurally, agents and maintainers can still read it as a CLI
subcommand. Documentation and installed instructions have had to teach that
distinction explicitly.

## Decision

- The CLI executable remains `specbind`. Every deterministic command continues
  to begin with that token.
- Product-managed Skills use the `sb-` namespace: `sb-plan`, `sb-discovery`,
  `sb-implement`, and the rest of the embedded catalog.
- `sb-*` names are selected through the Agent platform. They are not shell
  commands and must not be translated into `specbind ...` syntax.
- Refresh removes every formerly installed `specbind-*` product Skill,
  including its packaged reference files, before installing the corresponding
  `sb-*` package. No compatibility alias or stub is shipped.
- Historical Decisions and past forward-test records retain the identifiers
  that were current when they were written. Current source, generated
  installation surface, consumer instructions, public guides, and active
  forward-test contracts use `sb-*`.

## Consequences

- `specbind` unambiguously denotes the CLI in prose, diagnostics, and command
  examples.
- Skill discovery remains compact and namespaced without competing with CLI
  syntax.
- Updating an installed project is a breaking Skill-identifier migration; the
  installer performs the replacement atomically within its existing guarded
  plan and apply workflow.

## Verification

Mechanical tests cover the catalog names, both Agent install targets, complete
old-package removal on refresh, and the unchanged CLI command forms. Focused
forward tests confirm fresh agents discover the new Skill names without being
told a Skill or command name.
