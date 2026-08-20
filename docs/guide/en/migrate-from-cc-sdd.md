# Migrate from cc-sdd

This guide covers migration of an existing `.kiro` project to SpecBind.
SpecBind automatically converts only inputs whose meaning can be established
mechanically. It stops and switches to agent-assisted migration when milestone
scope, Design traceability, artifact language, or another semantic decision is
ambiguous.

!!! warning "Preview"

    The current Preview provides only the read-only
    `specbind migrate cc-sdd` plan. `--apply` stops with
    `MIGRATION_APPLY_UNAVAILABLE` and changes no files. Do not use ordinary
    `specbind install` to migrate `.kiro`.

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
preserve, and removal actions. Applying them is not available in the current
Preview:

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
| `.cc-sdd.json` | `.specbind.json` | Validate legacy `kiroDir`, language, and agent values, then create a new SpecBind configuration |
| `spec.json` | `spec.yaml` | Validate the complete phase and approval combination; do not recreate gate evidence |
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

### MIGRATE_TARGET_ALREADY_EXISTS {#migrate-target-already-exists}

`.specbind.json` or `.specbind` already exists. Reconcile the legacy input
with the current target state without overwriting valid SpecBind artifacts.

### MIGRATE_AGENT_SELECTION_REQUIRED / MIGRATE_AGENT_UNSUPPORTED {#migrate-agent-selection-required}

The target cannot be established as Codex or Claude Code, or the configured
legacy agent is outside SpecBind v1. Confirm the agent to install.

### MIGRATE_LANGUAGE_UNSUPPORTED {#migrate-language-unsupported}

The legacy configuration or Spec metadata uses a language outside SpecBind
v1 English and Japanese. Confirm the target language and translation scope.

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

### MIGRATE_RULE_REVIEW_REQUIRED / MIGRATE_TEMPLATE_REVIEW_REQUIRED {#migrate-rule-review-required}

Legacy rules or templates exist. Compare them with current SpecBind defaults,
retain project-owned policy or overrides, and do not copy procedural files
wholesale.

### MIGRATE_STEERING_REVIEW_REQUIRED {#migrate-steering-review-required}

Legacy steering documents exist. Confirm each responsibility and stable
`artifact_id`, then validate the resulting SpecBind Steering artifact.

### MIGRATE_SPEC_DIRECTORY_INVALID / MIGRATE_SPEC_ID_INVALID {#migrate-spec-path-invalid}

A legacy Spec path is not a regular directory or its ID is not canonical
kebab-case. Do not follow links; confirm the intended Spec ID and location.

### MIGRATE_SPEC_METADATA_MISSING / MIGRATE_SPEC_STATE_INVALID {#migrate-spec-state-invalid}

`spec.json` is missing, or its phase, generated, and approved combination is
not a valid legacy state. Investigate artifacts and history; do not invent gate
evidence.

### MIGRATE_LEGACY_AGENT_ASSET_INVALID / MIGRATE_LEGACY_CONTENT_UNSUPPORTED {#migrate-legacy-content-unsupported}

A known legacy agent asset is not a regular directory, or unsupported content
exists directly under `.kiro`. Inspect it individually and keep it outside
automatic conversion and removal.

---

[Migration entry](../migration/cc-sdd.md) | [日本語](../ja/migrate-from-cc-sdd.md)
