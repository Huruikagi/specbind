# Migrate from cc-sdd

This guide covers migration of an existing `.kiro` project to SpecBind.
SpecBind automatically converts only inputs whose meaning can be established
mechanically. It stops and switches to agent-assisted migration when milestone
scope, Design traceability, artifact language, or another semantic decision is
ambiguous.

!!! warning "Preview"

    `specbind migrate cc-sdd` is not implemented in the current Preview CLI.
    This page publishes the accepted procedure before the command is released.
    Do not use ordinary `specbind install` to migrate `.kiro`.

## Safety boundary

- Inspect Git state and current changes first.
- Do not delete, move, or overwrite the source `.kiro` tree.
- Do not translate cc-sdd approval flags into SpecBind gate evidence.
- Do not invent milestones, release history, Contracts, Requirement mappings,
  or completion evidence.
- Keep legacy `kiro-*` agent assets until validation and user confirmation are
  complete.
- Do not hand-edit CLI-owned state when a corresponding SpecBind operation
  exists.

## 1. Obtain the read-only plan

Run this from the target project root:

```sh
specbind migrate cc-sdd
```

If every conversion is unambiguous, review the reported create, convert,
preserve, and removal actions before applying them:

```sh
specbind migrate cc-sdd --apply
```

If the command returns `MANUAL_MIGRATION_REQUIRED`, do not use `--apply` to
bypass it. Preserve the complete output, including finding codes, paths, and
reasons.

## 2. Ask an agent to assist

Give Codex or Claude Code the complete CLI output and this page URL:

```text
Read the official guide below and migrate this repository from cc-sdd.
Start by inspecting the specbind migrate cc-sdd findings and named files.
Follow every stop condition, do not invent approval or completion evidence,
and return to CLI validation before claiming that the cutover is complete.

https://huruikagi.github.io/specbind/guide/en/migrate-from-cc-sdd/
```

Repository instructions in `AGENTS.md`, `CLAUDE.md`, and equivalent scoped
files still apply. This guide does not grant additional Git or project
authority.

## 3. Resolve user-owned decisions

The agent first investigates everything the repository can establish. It asks
the user only when choices such as these remain semantic:

- whether multiple active legacy Specs belong to the same active milestone;
- which one project-global language should replace mixed artifact languages;
- whether an apparently completed legacy Spec is accepted as the implemented
  baseline at cutover;
- which customized rule content remains project policy; or
- how much of an edited legacy quickstart block should be removed.

The agent stops at the affected boundary when the user has not confirmed the
choice.

## 4. Convert the artifacts

The agent limits its work to the findings reported by the CLI.

| cc-sdd input | SpecBind target | Boundary |
| --- | --- | --- |
| `spec.json` | `.specbind.json`, `spec.yaml` | Validate the complete phase and approval combination; do not recreate gate evidence |
| `requirements.md` | `SpecBind Requirements` | Validate Requirement IDs from recognized headings and Acceptance Criteria |
| `design.md` | `SpecBind Design` | Make Front Matter and body-marker Requirement mappings equal |
| `tasks.md` | `tasks.yaml` | Convert only supported task grammar and preserve only provable progress |
| Implementation Notes | `implementation-notes.md` | Separate non-empty durable notes |
| steering | `SpecBind Steering` | Confirm document responsibility and a stable `artifact_id` |
| legacy rules | new project-owned rules | Review differences as policy; do not copy whole procedural files |

A missing Contract is not proof that a Spec has no external effect. Create it
through the normal SpecBind Design workflow from current Requirements and
Design, then repeat the required review and approval steps.

## 5. Return to CLI validation

After guided work, run the read-only plan again:

```sh
specbind migrate cc-sdd
```

Resolve only the remaining findings. Do not regenerate and overwrite an
already valid target artifact from legacy input.

Do not declare completion until the migration implementation recognizes the
guided work, plans only safe remaining actions, and applicable ordinary
SpecBind checks pass. Depending on the target state, inspect commands such as:

```sh
specbind artifact list <spec>
specbind check traceability <spec>
specbind check contracts
specbind spec status <spec>
specbind milestone status
```

## 6. Retire the legacy workflow

Only after the converted state is valid and the user confirms cutover may the
CLI remove exact known `kiro-*` agent assets and an exact known legacy
quickstart block. Edited, mixed, or duplicate instructions require individual
review and are never removed by keyword guessing.

The original `.kiro` tree remains after migration. Git can recover the target
changes if necessary, but do not let legacy and SpecBind skills update workflow
state concurrently.

## Finding codes

### MIGRATE_ACTIVE_SCOPE_AMBIGUOUS {#migrate-active-scope-ambiguous}

Several legacy Specs appear active, but the repository does not prove that
they form one active milestone. Inspect the legacy roadmap, dependencies, and
current intent, then ask the user to confirm the scope.

### MIGRATE_DESIGN_TRACEABILITY_REQUIRED {#migrate-design-traceability-required}

The legacy Design cannot mechanically provide complete SpecBind Requirement
traceability. Read Requirements and Design, make each Design artifact's Front
Matter and body-marker sets equal, and validate them with the CLI.

### MIGRATE_LANGUAGE_MIXED {#migrate-language-mixed}

Legacy Specs use mixed artifact languages. SpecBind uses one project-global
artifact language, so stop until the user chooses the language and translation
scope.

### MIGRATE_LEGACY_INSTRUCTIONS_AMBIGUOUS {#migrate-legacy-instructions-ambiguous}

Legacy guidance in `AGENTS.md` or `CLAUDE.md` is not an exact known block. Do
not delete text based on the word `kiro`; preserve surrounding project-owned
instructions and confirm the intended removal with the user.

---

[Migration entry](../migration/cc-sdd.md) | [日本語](../ja/migrate-from-cc-sdd.md)
