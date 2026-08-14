# 0053: Use a minimal cross-spec review state shape

Status: Accepted

## Context

The global cross-spec review must preserve the semantic classification that cannot be reproduced deterministically from file hashes alone. A detailed persistent `impacts` list containing affected contract entries and downstream consumers would duplicate facts that the CLI can derive from contract diffs, the current contract graph, and roadmap scope. It would also retain review detail that ordinary status and release guards do not need.

Decision 0052 already separates this machine record from always-loaded steering context. The remaining state should still be minimal, strict, and clear about which milestone and input revisions were accepted.

## Decision

- `state/cross-spec-review.yaml` is a strict standalone YAML object with exactly these five required top-level fields:
  - `schema_version: 1`
  - `milestone_id`
  - `passed_at`
  - `classifications`
  - `input_revisions`
- Additional top-level fields are rejected.
- `milestone_id` is the Decision 0043 UUID v7 and must exactly match the active roadmap.
- `passed_at` is the Decision 0036 timezone-qualified RFC 3339 timestamp at which the complete milestone review passed.
- `classifications` is a non-empty sparse mapping with these optional groups:
  - `specs`: canonical spec identity to `LOCAL_ONLY`, `CONTRACT_COMPATIBLE`, or `CONTRACT_BREAKING`
  - `direct_changes`: roadmap-local direct-change ID to `LOCAL_ONLY`
- A group is present only when it contains at least one entry. At least one group is required.
- Every current roadmap work item appears exactly once in `classifications`: both `new_specs` and `spec_updates` resolve into `specs`, while `direct_changes` resolves into `direct_changes`.
- A direct change with non-local contract impact is invalid and must be rerouted to a new-spec or spec-update item before the global review can pass.
- `input_revisions` is a non-empty mapping whose values use the Decision 0016 fingerprint representation. The exact owned projections, path-key rules, and normalization remain a follow-up decision.
- The record contains no `status`, duplicated accepted outcome, approval mode, workflow name, reviewer identity, affected-entry list, downstream-spec list, findings, or attempt history. Presence of a fresh record means the global review passed.
- Affected entries and downstream scope are derived when needed from the accepted revisions, released comparison, current contract graph, and roadmap. Detailed semantic findings remain run-scoped.

## Consequences

```yaml
schema_version: 1
milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
passed_at: 2026-08-14T17:30:00+09:00
classifications:
  specs:
    account-auth: CONTRACT_COMPATIBLE
    checkout: LOCAL_ONLY
  direct_changes:
    update-ci: LOCAL_ONLY
input_revisions:
  roadmap.md#cross-spec-scope: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
  specs/account-auth/contract.md: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

- The stored semantic output is small enough for concise CLI summaries and useful release history.
- Freshness remains revision-based rather than inferred from classification labels.
- The later fingerprint decision can refine `input_revisions` without reopening the accepted classification shape.
