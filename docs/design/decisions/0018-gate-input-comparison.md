# 0018: Compare prose revisions and Requirement IDs without semantic hashing

Status: Accepted

## Context

Gate evidence needs stable comparisons for human-authored Markdown, structured task plans, and the active Requirement ID set. These inputs change for different reasons and should not all be reduced through one generic file-normalization rule.

Most Markdown gate inputs are authoritative prose documents. A content edit should invalidate their evidence, while a platform-only CRLF/LF conversion should not. The active Requirement IDs are already a small ordered machine value and gain no clarity from hashing. `tasks.yaml`, by contrast, combines an approved plan with mutable execution state such as status, blocked information, and implementation notes.

## Decision

### Markdown inputs

- A selected Markdown gate input is fingerprinted from its complete file bytes after line-ending normalization only.
- Every CRLF sequence and every remaining bare CR byte is replaced by LF before SHA-256 is computed.
- All other bytes remain significant, including whitespace, a UTF-8 BOM if present, and the presence or absence of a final newline.
- The resulting value uses the tagged lowercase representation from Decision 0016.
- Decision 0017 still excludes `brief.md` from requirements gate evidence.

### Active Requirement IDs

- Requirements gate evidence stores the approved ordered Requirement ID array directly as `approved_requirement_ids`.
- The current `active_change.requirement_ids` and the evidence snapshot must match by exact array equality, including order.
- The array is not hashed.
- A changed value or order invalidates requirements evidence through the normal requirements-state rewind contract.

### Structured tasks

- `tasks.yaml` is not fingerprinted as a whole serialized file.
- The tasks gate fingerprints a schema-defined plan projection that excludes mutable execution state.
- Status or checkbox-equivalent state, blocked execution details, and implementation notes do not alter the approved plan fingerprint by themselves.
- Changes to task identity, hierarchy, plan text, Requirement ID coverage, dependencies, optionality, or other plan-definition fields do alter the plan projection.
- The exact v1 task fields, completion projection, and canonical serialization of projections remain follow-up schema decisions.

## Consequences

- Normal Git line-ending conversion does not stale Markdown gate evidence.
- Other Markdown formatting edits remain visible as revision changes without requiring Markdown parsing.
- Requirements evidence remains readable and shows exactly which ordered Requirement IDs were approved.
- Normal implementation progress does not force task-plan reapproval.
- Task comparison depends on typed schema projections rather than YAML key order, comments, or presentation formatting.
