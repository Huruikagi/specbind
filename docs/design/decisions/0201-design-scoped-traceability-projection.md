# 0201: Give Design workflows a Design-scoped traceability projection

Status: Accepted

## Context

Requirements invalidation clears downstream Design, Tasks, and completion gate
evidence but deliberately preserves their documents as repair inputs. After a
new active Requirement set is approved, the retained `tasks.yaml` can therefore
describe the previous set until the workflow reaches Tasks authoring again.

The complete `check traceability` command correctly reports that mismatch.
However, both Design authoring and independent Design validation used that
complete result as a structural precondition even though neither workflow owns
Tasks. A retained Task that references only inactive Requirements could then
stop Design validation, while Tasks could not be revised until after Design
approval and Contract Review.

The Design approval guard already excludes Task-owned traceability diagnostics.
Teaching an Agent to reinterpret a failed complete check from lifecycle state
and diagnostic prefixes would duplicate that deterministic boundary in prose
and make fresh-context recovery unreliable.

## Decision

`specbind check traceability <spec>` gains the explicit `--for-design` option.
It resolves and checks Requirements, the CLI-owned active Requirement set, and
the complete Design set. It does not read, parse, validate, or report coverage
from `tasks.yaml`.

The successful result is distinct from complete traceability:

```text
OK TRACEABILITY_DESIGN_VERIFIED
Scope: Requirements and Design
Task coverage: not evaluated (Design scope)
```

It continues to report the exact active Requirement set and Design coverage.
Requirements or Design discovery, structure, reference, or coverage faults fail
with `TRACEABILITY_DESIGN_FAILED`. The option is an explicit ownership
projection, not a lifecycle-state exception, so it has the same meaning whenever
an active Design is independently reviewed.

The unqualified command remains the strict complete projection and retains its
existing result codes and output. Tasks authoring, Tasks approval, implementation
validation, status diagnosis, and other full-workflow consumers continue to use
it and therefore continue to reject stale or invalid Task plans.

Design authoring and `sb-validate-design` use `--for-design`. Design approval
uses the same underlying Design-scoped resolver rather than loading a downstream
Task plan and filtering its diagnostics afterward. A retained `tasks.yaml` and
its execution state are not deleted or rewritten; the Tasks phase repairs or
replaces them under its existing ownership.

This narrows Decisions 0104 and 0114 where they required the complete
traceability command for Design authoring and validation. Decision 0123's
Task-only reverse-coverage failure remains strict in the complete projection and
at Tasks approval.

## Consequences

- A Requirements rewind can reach Design validation and approval without
  treating the previous Task plan as current Design input.
- Agents receive an ordinary success or failure for the responsibility they are
  checking; they do not reinterpret a failed command from conversation context.
- The output cannot be presented as complete traceability because its result
  code and scope line state that Tasks were not evaluated.
- No artifact schema, persisted lifecycle field, deletion rule, or Task
  execution-state migration is introduced.

## Verification

CLI tests prove that Design-scoped traceability succeeds with a stale or
structurally invalid retained Task plan, still fails on Design coverage faults,
and leaves the unqualified command strict. Gate tests prove Design approval uses
the same projection while Tasks approval remains strict. Skill conformance and a
fresh-fixture forward test prove Design validation reaches semantic judgment
without editing the retained Task plan.
