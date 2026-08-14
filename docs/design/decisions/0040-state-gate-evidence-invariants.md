# 0040: Use sparse cumulative gate evidence with semantic state invariants

Status: Accepted

## Context

The four gate evidence object shapes are accepted, but `active_change` must still distinguish valid lifecycle progress from contradictory combinations. Encoding every state combination as a deeply nested JSON Schema union would reject damaged or hand-edited metadata before the CLI could report a precise lifecycle diagnosis.

The schema and lifecycle validator already have separate responsibilities under Decision 0015: JSON Schema validates structural shape, while Rust semantic validation checks cross-field and transition invariants.

## Decision

- `active_change.gate_evidence` is sparse and cumulative. It allows only the keys `requirements`, `design`, `tasks`, and `completion`, each referencing its accepted strict evidence definition.
- When no gate has passed, `gate_evidence` is omitted. An empty `gate_evidence: {}` object is invalid.
- A consistent declared state has exactly this evidence-key set:

| State | Required and permitted evidence keys |
| --- | --- |
| `requirements` | none; omit `gate_evidence` |
| `design` | `requirements` |
| `tasks` | `requirements`, `design` |
| `implementation` | `requirements`, `design`, `tasks` |
| `release_ready` | `requirements`, `design`, `tasks`, `completion` |

- Evidence keys for a later state are prohibited, and no earlier cumulative key may be missing.
- `requirement_ids` is `null` in `requirements`. From `design` onward it is a non-empty, unique, deterministically ordered list, and must exactly match the requirements gate's `approved_requirement_ids`.
- JSON Schema validates the evidence container's allowed keys, non-empty shape, and nested evidence structures. The lifecycle semantic validator enforces the state-to-evidence key set, `requirement_ids` state rules, exact approved-ID equality, freshness, and artifact presence.
- A structurally readable but semantically contradictory file remains loadable for `spec status` and repair diagnostics. State-changing commands reject it until repaired; they never infer a later state or manufacture missing evidence.

## Consequences

- Normal files remain concise and contain no placeholder evidence object.
- Status can identify a missing, premature, or stale gate instead of returning only a generic schema-union failure.
- The declared state stays authoritative, while health is derived from evidence, artifacts, and freshness.
- Approved spec-backed changes always contain at least one active Requirement ID.
