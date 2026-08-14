# 0028: Fingerprint a normalized typed task plan

Status: Accepted

## Context

`tasks.yaml` combines an approved plan with mutable execution state. Hashing the YAML bytes would make comments, indentation, quoting, mapping-key order, and other presentation-only edits invalidate task approval. Hashing the entire parsed artifact would instead make ordinary progress updates invalidate the plan gate.

The plan also contains two kinds of arrays. Plan items, child tasks, details, and completion criteria carry meaningful sequence. Requirement IDs, boundaries, contracts, and explicit dependencies are sets whose presentation order should not affect approval freshness.

## Decision

The task-plan fingerprint is computed as follows:

1. Parse `tasks.yaml`, rejecting duplicate mapping keys, then perform schema and required semantic validation.
2. Project exactly the root `plan` value. `schema_version`, `execution`, `implementation-notes.md`, and every other artifact are excluded.
3. Preserve the order of `plan.items`, each group's `tasks`, `details`, and `completion_criteria`.
4. At every executable task, sort `requirement_ids`, `boundaries`, `contracts`, and `depends_on` independently. Sorting compares raw strings by unsigned UTF-16 code units, ascending and independent of locale, matching the property-name comparison used by JCS. Schema validation rejects duplicates before this step.
5. Serialize the normalized plan with the [RFC 8785 JSON Canonicalization Scheme](https://www.rfc-editor.org/rfc/rfc8785.html). JCS recursively sorts object properties, preserves array order, emits no insignificant whitespace, preserves string content without Unicode normalization, and produces UTF-8 bytes.
6. Compute SHA-256 over those bytes and store the tagged lowercase value accepted by Decision 0016 under the `tasks.yaml#plan` tasks-gate evidence key accepted by Decision 0039.

All schema-defined plan fields participate. Field absence remains distinct from a present value; the sparse schema already rejects empty arrays and unsupported default-like fields where omission is required.

## Consequences

- YAML comments, indentation, scalar quoting, anchors after parsing, and mapping-key order do not change the fingerprint.
- Reordering a set-like task annotation does not change the fingerprint.
- Reordering tasks, details, or completion criteria changes the fingerprint because it may change execution or interpretation.
- Changing any string content, field presence, task structure, scheduling flag, or set membership changes the fingerprint.
- Rust and other producers share an externally specified canonical JSON algorithm rather than relying on language-specific map iteration or ordinary JSON serialization.
