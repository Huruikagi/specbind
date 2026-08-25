# 0018: Compare prose revisions and Requirement IDs without semantic hashing

Status: Accepted

[Decision 0146](./0146-sequential-v1-tasks-and-per-task-checkpoints.md)
removes `parallel` from the prerelease task-plan projection before stable v1.

## Context

Gate evidence needs stable comparisons for human-authored Markdown, structured task plans, and the active Requirement ID set. These inputs change for different reasons and should not all be reduced through one generic file-normalization rule.

Most Markdown gate inputs are authoritative prose documents. A content edit should invalidate their evidence, while a platform-only CRLF/LF conversion should not. The active Requirement IDs are already a small ordered machine value and gain no clarity from hashing. `tasks.yaml`, by contrast, combines an approved plan with mutable execution state such as status and blocked information.

## Decision

### Markdown inputs

- A selected Markdown gate input is fingerprinted from its complete file bytes after line-ending normalization only.
- Every CRLF sequence and every remaining bare CR byte is replaced by LF before SHA-256 is computed.
- All other bytes remain significant, including whitespace, a UTF-8 BOM if present, and the presence or absence of a final newline.
- The resulting value uses the tagged lowercase representation from Decision 0016.
- Decision 0017 still excludes the discovered `SpecBind Brief` artifact from requirements gate evidence.

### Active Requirement IDs

- Requirements gate evidence stores the approved ordered Requirement ID array directly as `approved_requirement_ids`.
- The current `active_change.requirement_ids` and the evidence snapshot must match by exact array equality, including order.
- The array is not hashed.
- A changed value or order invalidates requirements evidence through the normal requirements-state rewind contract.

### Structured tasks

- `tasks.yaml` is not fingerprinted as a whole serialized file.
- The tasks gate fingerprints only the normalized, schema-defined `plan` projection under Decision 0028; mutable execution state and other artifacts are excluded.
- Status or checkbox-equivalent state and blocked execution details do not alter the approved plan fingerprint by themselves.
- Changes to task identity, hierarchy, meaningful sequence, plan text, explicit completion criteria, Requirement ID coverage, `parallel`, set membership in `depends_on`, or other plan-definition fields alter the plan fingerprint. Reordering set-like Requirement IDs, boundaries, contracts, or dependencies does not. Decision 0019 defines the ordering and sparse-dependency semantics; Decision 0021 defines completion-criteria omission; Decision 0022 excludes optional tasks.
- Completion uses the clean project-revision handshake and CLI recomputation accepted by Decisions 0029, 0031 through 0033, and 0080 rather than another serialized semantic projection.

## Consequences

- Normal Git line-ending conversion does not stale Markdown gate evidence.
- Other Markdown formatting edits remain visible as revision changes without requiring Markdown parsing.
- Requirements evidence remains readable and shows exactly which ordered Requirement IDs were approved.
- Normal implementation progress does not force task-plan reapproval.
- Task comparison depends on typed schema projections rather than YAML key order, comments, or presentation formatting.

## Implementation status

The Rust fingerprint producer normalizes CRLF and bare CR to LF without changing any other Markdown byte. Task-plan production accepts only an artifact-local semantically validated domain document, excludes execution state, and preserves sequence-bearing arrays while sorting set-like arrays before JCS serialization.
