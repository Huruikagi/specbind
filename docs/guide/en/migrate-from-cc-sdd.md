# Migrate from cc-sdd

This guide covers migration of an existing `.kiro` project to SpecBind.
SpecBind automatically converts only inputs whose meaning can be established
mechanically. It stops and switches to agent-assisted migration when milestone
scope, Design traceability, artifact language, or another semantic decision is
ambiguous.

!!! warning "Preview"

    Semantic work such as legacy Spec conversion uses the agent-assisted path.
    `specbind install` does not convert `.kiro`, but after the read-only plan it
    may prepare the selected SpecBind language and agents. Always return to the
    resolution acceptance and `--apply` steps on this page before claiming the
    cutover is complete or retiring legacy assets.

## Safety boundary

- Inspect Git state and current changes first.
- Do not delete, move, or overwrite the source `.kiro` tree during planning or
  agent-assisted work. Only final `--apply` retires it after Git checks.
- Do not translate cc-sdd approval flags into SpecBind gate evidence.
- Do not invent milestones, release history, Contracts, Requirement mappings,
  or completion evidence.
- Keep legacy `kiro-*` agent assets until validation and user confirmation are
  complete.
- Final cutover stops unless every file below `.kiro`, `.cc-sdd.json`, legacy
  agent assets, and resolution state is tracked by Git. Ignored files are not
  deleted.
- Do not hand-edit CLI-owned state when a corresponding SpecBind operation
  exists.

## 1. Obtain the read-only plan

Run this from the target project root:

```sh
specbind migrate cc-sdd
```

If every conversion is unambiguous, review the reported create, convert, and
retirement actions. `--apply` recomputes the plan and verifies a
committed, clean Git recovery boundary before applying known conversions. It
stops rather than delete a legacy asset that Git cannot recover:

```sh
specbind migrate cc-sdd --apply
```

If the command returns `MANUAL_MIGRATION_REQUIRED`, do not use `--apply` to
bypass it. Preserve the complete output, including finding codes, paths, and
reasons.

The current automatic subset installs SpecBind from `.cc-sdd.json` and retires
exact known Codex or Claude Code `kiro-*` skills plus the Git-tracked cc-sdd
source at final cutover. Any
legacy Spec produces `MIGRATE_SPEC_CONVERSION_REQUIRED`; the CLI does not guess
its milestone or gate evidence.

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
| `tasks.md` | `tasks.yaml` | Convert only supported task grammar, preserve only provable progress, and drop `(P)` into conservative target order |
| Implementation Notes | `implementation-notes.md` | Separate non-empty durable notes |
| steering | `SpecBind Steering` | Confirm document responsibility and a stable `artifact_id` |
| legacy rules | new project-owned rules | Review differences as policy; do not copy whole procedural files |

A missing Contract is not proof that a Spec has no external effect. Create it
through the normal SpecBind Design workflow from current Requirements and
Design, then repeat the required review and approval steps.

## 5. Hand the migration decisions to the CLI

First prepare the SpecBind target with the selected language and agents and
validate the converted artifacts through ordinary CLI operations. Use
`specbind install` only to establish the SpecBind-side foundation, not as a
`.kiro` converter. Review and commit the converted target so Git provides a
clean recovery point.

The agent then gives the CLI a strict JSON candidate that exactly enumerates
all current findings. Read it from an external temporary file or standard
input:

```json
{
  "schemaVersion": 1,
  "assessment": "Compared the legacy rules and rewrote only current project policy.",
  "target": { "language": "en", "agents": ["codex"] },
  "resolutions": [
    {
      "code": "MIGRATE_RULE_REVIEW_REQUIRED",
      "path": ".kiro/settings/rules",
      "disposition": "converted",
      "targets": [".specbind/settings/rules/project.md"]
    }
  ]
}
```

```sh
specbind migrate cc-sdd --accept-resolution ../cc-sdd-resolution.json
```

`converted` requires at least one concrete target. Use `not_migrated` with an
empty `targets` list for an intentional omission. The candidate must cover all
current semantic findings exactly; it cannot waive a mechanical safety
finding.

The CLI revalidates the sources and targets, computes its own fingerprints,
and writes `.specbind/state/cc-sdd-migration.yaml`. Do not hand-edit this file.
Review and commit it. A later source or target change makes the resolution
stale and restores the original findings. This state is a temporary handshake:
final `--apply` removes it with the cc-sdd source while Git retains its accepted
revision.

## 6. Return to CLI validation

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

From the clean commit containing the resolution record, perform final cutover.
Running this command is the explicit retirement confirmation. The CLI
revalidates every cleanup target and deletes nothing if any file is untracked,
ignored, linked, or changed:

```sh
specbind migrate cc-sdd --apply
```

## 7. Retire the legacy workflow

On success, the configured cc-sdd source root, `.cc-sdd.json`, exact known
`kiro-*` skills, and resolution state are removed, leaving SpecBind as the only
active workflow. A rerun returns `NO_CHANGE CC_SDD_MIGRATION_COMPLETE`.

Edited, mixed, or duplicate legacy text in `AGENTS.md` or `CLAUDE.md` is not
removed by keyword guessing even though Git can recover it. Resolve and commit
that semantic edit during guided work. If final cleanup encounters a filesystem
error, restore the pre-cutover commit with Git before retrying.

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

### MIGRATE_LANGUAGE_SELECTION_REQUIRED {#migrate-language-selection-required}

Neither legacy configuration nor Spec metadata establishes English or
Japanese. Select the project-global artifact language before automatic apply.

### MIGRATE_SPEC_CONVERSION_REQUIRED {#migrate-spec-conversion-required}

Legacy Specs exist. Use the agent-assisted procedure without guessing the
active milestone, Requirement mappings, or gate evidence, then return to CLI
validation.

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

### MIGRATE_LEGACY_AGENT_ASSET_INVALID / MIGRATE_LEGACY_AGENT_ASSET_UNKNOWN / MIGRATE_LEGACY_CONTENT_UNSUPPORTED {#migrate-legacy-content-unsupported}

A known legacy agent asset is not a regular directory, an unknown `kiro-*`
asset exists, or unsupported content exists directly under `.kiro`. Inspect it
individually and record whether it is converted or intentionally not migrated.
Git retains the source history after final cutover.

### MIGRATE_RESOLUTION_STALE / MIGRATE_RESOLUTION_STATE_INVALID {#migrate-resolution-stale}

A source, target, finding, or selected installation covered by the accepted
resolution changed, or the CLI-owned state is invalid. Do not hand-edit the
state. Review the current findings and accept a new external candidate with
`--accept-resolution`.

### MIGRATION_CLEANUP_TARGET_UNTRACKED / MIGRATION_CLEANUP_TARGET_UNSAFE {#migration-cleanup-target-unsafe}

A final-cutover target contains an untracked or ignored file, a link or reparse
point, or another unsafe shape. Commit needed content or move it outside the
legacy root, then retry from a clean worktree. The CLI does not delete files
that Git cannot recover.

---

[Migration entry](../migration/cc-sdd.md) | [日本語](../../ja/guide/migrate-from-cc-sdd.md)
