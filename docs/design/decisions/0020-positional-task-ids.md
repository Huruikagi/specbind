# 0020: Keep cc-sdd-style positional numeric Task IDs

Status: Accepted

## Context

The inherited cc-sdd plans use compact numeric identifiers such as `1` and `1.2` for display, task selection, requirement coverage review, dependency annotations, and implementation progress. In practice, task reordering has not been common enough to justify introducing a second stable identifier scheme.

`tasks.yaml` is milestone-local and a plan revision already invalidates the tasks gate, so preserving identity across arbitrary reorder operations is not a v1 requirement.

## Decision

- Task and group IDs are stored as strings so `1.2` is never interpreted as a decimal number.
- Top-level items use one-based sequential IDs matching their array position: `1`, `2`, `3`, and so on.
- Child tasks use `<group>.<child>` IDs matching their group and one-based position: `1.1`, `1.2`, `2.1`, and so on.
- IDs have at most two numeric levels. Zero, leading zeroes, empty segments, and deeper forms such as `1.2.1` are invalid.
- Groups contain only child tasks; nested groups are invalid.
- IDs are unique within one `tasks.yaml` plan and contain no gaps relative to their containing array.
- `depends_on` and other task references use the same string form but must resolve to executable tasks in the same `tasks.yaml`, not container groups or another spec; see Decision 0027.
- Reordering, insertion, or removal that changes positions renumbers affected IDs and updates every reference and execution-state key in one guarded plan mutation.
- Positional IDs are stable only while the approved plan order is unchanged. They are not cross-milestone or historical identities.

The JSON Schema accepts the lexical form `^[1-9][0-9]*(?:\.[1-9][0-9]*)?$`. Position matching, gap detection, reference resolution, and group/task distinctions remain semantic CLI validation.

## Consequences

- Existing cc-sdd task selection habits remain valid.
- Human and CLI output can use the same short identifiers without separate display numbers.
- Plan reordering has a larger diff and must update references, but it is already an approval-invalidating plan change.
- The schema does not introduce opaque UUIDs or slug lifecycle rules for milestone-local tasks.
