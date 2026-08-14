# 0037: Use a minimal strict completion evidence object

Status: Accepted

## Context

Prior decisions have assigned upstream artifact freshness to the gate-local chain, cross-spec review to a project-level state artifact, semantic success to the meaning of accepted completion evidence, and failed validation attempts to run-scoped output. The remaining durable completion record needs only to identify when validation passed, which immutable implementation revision was validated, and which successful mechanical commands grounded the result.

## Decision

- `gate_evidence.completion` is a strict object with exactly three required fields:
  - `passed_at`: the Decision 0036 timezone-qualified RFC 3339 timestamp
  - `implementation_revision`: the Decision 0031 full Git commit object ID
  - `mechanical_checks`: the Decision 0033 non-empty ordered list of successful mechanical checks
- All three fields are required and additional fields are rejected.
- Completion evidence has no approval mode or delegation workflow because `IMPLEMENTATION_VALIDATED` is a validation result, not a user-approval gate.
- Completion evidence contains no upstream artifact fingerprints, semantic pass flags, duplicated `GO` value, cross-spec review data, logs, or attempt history under Decisions 0032, 0034, 0050, and 0052.
- The record is written only by the guarded Decision 0029 acceptance call and is cleared when completion becomes stale.

## Consequences

- Completion status is compact, deterministic to validate, and easy for `spec status` to summarize.
- Each retained field carries information not already owned by another lifecycle artifact or gate.
- Future evidence additions require an explicit schema-version or compatibility decision rather than ad hoc keys.
