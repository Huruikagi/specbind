# 0023: Use strict group and task objects with sparse exception fields

Status: Accepted

## Context

Decisions 0019 through 0022 establish ordered execution, positional IDs, conditionally optional completion criteria, and the absence of optional tasks. The remaining plan shape needs to preserve cc-sdd's useful task content without filling YAML with false booleans and empty arrays.

Safe or empty defaults are clearer when represented by field absence. Exception markers and non-empty supplemental lists should appear only when they carry information.

## Decision

### Root plan

- `tasks.yaml` requires `schema_version: 1` and a `plan` object.
- `plan.items` is a non-empty ordered array of top-level groups or executable tasks.
- The plan shape is independent of the sparse execution-state object accepted separately by Decision 0024.

### Groups

- A group requires `id`, `kind: group`, `title`, and `tasks`.
- A group contains at least two executable tasks. A single child is collapsed into a top-level task.
- Groups cannot contain groups and carry no execution, scheduling, requirement, boundary, contract, detail, or completion fields.

### Executable tasks

- An executable task requires `id`, `kind: task`, `title`, and a non-empty `requirement_ids` array.
- `details`, `completion_criteria`, `boundaries`, `contracts`, and `depends_on` are omitted when empty. If present, each is a non-empty array.
- `parallel` is omitted for ordinary sequential tasks. If present, its only valid value is `true`; `parallel: false` is invalid.
- `parallel: true` requires a non-empty `boundaries` array so the parallel-safety claim has an explicit responsibility scope.
- `depends_on` contains only sparse additional dependencies within the same `tasks.yaml` under Decisions 0019 and 0027; an empty array is invalid.
- `optional` is unsupported under Decision 0022.
- Unknown fields fail strict schema validation.

### Validation boundary

- JSON Schema validates object variants, required fields, non-empty collections, the positive-only parallel marker, and the parallel-boundary condition.
- Rust semantic validation checks positional numbering, array/ID alignment, Requirement ID coverage, boundary and contract references where applicable, effective dependencies, and whether omitted completion criteria are justified.

## Consequences

- A straightforward task remains compact while exceptional planning information stays visible.
- Missing `parallel` is conservatively sequential; missing `depends_on` means no additional dependency beyond ordering rules.
- There is no ambiguous difference between an absent list and an explicitly empty list.
- The accepted plan remains a stable root projection while execution-state fields change independently.
