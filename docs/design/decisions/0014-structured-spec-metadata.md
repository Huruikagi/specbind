# 0014: Use spec.yaml as the canonical spec metadata artifact

Status: Accepted

## Context

The inherited cc-sdd workflow stores per-spec lifecycle metadata in `spec.json`. Target SpecBind metadata now includes an active-change state, milestone ID, the active Requirement ID set, gate evidence, approval mode, and artifact revisions. This is machine-validated state that users may occasionally inspect but do not primarily review as prose. Decision 0041 identifies the per-spec change through milestone and canonical spec identity instead of another stored ID.

[Decision 0013](./0013-structured-task-artifact.md) establishes `tasks.yaml` as the canonical structured task artifact. Keeping related per-spec lifecycle state in JSON would require two structured authoring formats without providing a meaningful product boundary.

## Decision

- `spec.yaml` is the only canonical per-spec lifecycle metadata artifact in the target SpecBind workflow.
- The Rust CLI and target skills read and update `spec.yaml`; they do not dual-write `spec.json`.
- `spec.yaml` uses a versioned, machine-validated schema with stable English field names.
- The top-level schema version is required.
- Duplicate keys, unknown fields, YAML anchors, aliases, merge keys, and custom tags are rejected.
- IDs, timestamps, and fingerprints are represented as strings under their eventual field schemas.
- Human-authored prose follows the spec's configured product language where applicable; machine keys and enum values remain stable.
- Decision 0044 fixes the strict top-level and active-change shape after the gate-evidence and revision decisions it references.

## Scope boundary

This decision changes only metadata inside each spec directory.

It does not change:

- the persisted CLI configuration file `.specbind.json`
- installation manifests currently distributed as JSON
- requirements, design, contract, brief, per-spec release log, roadmap, or other prose artifacts
- the `tasks.yaml` schema accepted separately by Decision 0013

Those formats change only through their own explicit decisions.

Decision 0043 keeps UUID v7 generation local and adds no milestone-allocation state to `.specbind.json` or `spec.yaml`.

## Lifecycle and writes

- New target specs are created with `spec.yaml` and no `spec.json`.
- CLI state-changing events perform guarded writes to `spec.yaml` and validate the result before reporting success.
- Agent skills do not infer lifecycle state from file presence when canonical metadata is available.
- Direct edits are permitted as repository edits but may produce consistency diagnostics, invalidate gate evidence, or require an explicit repair transition.
- YAML comments are not semantic schema content and are not guaranteed to survive a CLI rewrite.

## Gate revisions

The CLI never fingerprints the entire `spec.yaml` as a gate input. Doing so would make gate evidence self-referential and would invalidate approvals whenever unrelated lifecycle fields changed.

Each gate instead defines an explicit projection of relevant metadata fields, such as the active Requirement ID set, and combines that projection with the applicable artifact revisions. Exact projections belong to the gate-evidence and fingerprint schema decisions.

## Migration

- Existing `spec.json` is a migration input, not a second steady-state format.
- Migration validates the complete inherited phase and approval combination before creating target lifecycle state.
- Contradictory booleans, missing artifacts, or unsupported values produce diagnostics rather than guessed state.
- Migration writes `spec.yaml` only after the converted state passes target schema and lifecycle validation.
- The original `spec.json` is preserved or removed through an explicit, recoverable migration operation; normal target workflows do not keep it synchronized.
- Historical `spec.json` files remain available at existing commits and any project-created release references.

## Consequences

- Target structured spec artifacts consistently use YAML while prose artifacts remain Markdown.
- Lifecycle and task schemas can share validation and diagnostic conventions without becoming one combined file.
- The accepted active Requirement ID set remains inside per-spec active-change metadata; Decision 0014 changes its serialized artifact from `spec.json` to `spec.yaml` without changing Decision 0003's ownership boundary.
- Current templates, skills, and the current artifact index continue to describe shipped `spec.json` behavior until migration is implemented.
- Rust migration fixtures must cover valid, contradictory, and partially complete `spec.json` conversions.

## Open schema details

- Migration diagnostics and recovery behavior for every contradictory inherited `spec.json` flag combination.
- YAML normalization and stable serialization rules.
- Recoverable migration flow and handling of locally edited `spec.json` files.
