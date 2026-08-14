# 0026: Keep implementation notes as an unstructured runtime list

Status: Accepted

## Context

The inherited `## Implementation Notes` section is a simple list of cross-task learnings discovered during implementation. Its primary consumer is a later AI implementation or validation run, not SpecBind's lifecycle engine.

Adding structured Task ID links, authors, timestamps, or note categories would create maintenance and migration rules without improving the core purpose: preserving short runtime guidance that prevents repeated mistakes.

## Decision

- `execution.implementation_notes` is an ordered array of non-empty strings.
- The field is omitted when no notes exist; an empty array is invalid.
- Notes are spec-wide implementation context rather than children of one task.
- An author may mention a Task ID or other scope in the string when useful, but SpecBind does not parse, resolve, validate, or rewrite such references.
- Notes carry no required author, timestamp, category, stable ID, or structured task relationship.
- Workflows normally append newly discovered information in discovery order. Because this is runtime guidance rather than an audit log, stale or incorrect notes may be edited or removed through an explicit artifact update.
- Implementation notes do not affect the task-plan fingerprint, task status, gate approval, or lifecycle state by themselves.
- A note cannot substitute for `blocked`, a plan revision, a requirements/design rewind, or completion evidence when one of those state changes is required.
- `tasks list --json` and `tasks show --json` expose the ordered list verbatim for agent consumers. Human task views may render a separate implementation-notes section.
- Successful release finalization removes `tasks.yaml`; the immutable release reference preserves the pre-finalization notes with the rest of the task artifact.

## Migration

- Bullets under an inherited `## Implementation Notes` heading become strings in the same order.
- Unstructured Task ID text remains text and receives no generated relationship metadata.
- Empty headings produce no `implementation_notes` field.

## Consequences

- Later agents receive the same practical context as the Markdown workflow with minimal schema overhead.
- Plan renumbering does not attempt unsafe rewrites inside free-form notes.
- Consumers that need reliable dependencies, blockers, or evidence must use the corresponding structured fields instead of interpreting note prose.
