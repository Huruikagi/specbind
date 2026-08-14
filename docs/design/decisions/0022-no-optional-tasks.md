# 0022: Do not carry optional tasks into tasks.yaml v1

Status: Accepted

## Context

Inherited cc-sdd permits the special `- [ ]*` marker for deferrable, test-only coverage tied to acceptance criteria. It is not a general plan concept and can remain incomplete while the feature is treated as complete.

In practice this marker is uncommon, and it conflicts with SpecBind's simpler completion contract: the active task plan should state the work required for the current change, while deferred work should be removed from current scope or planned as later work rather than remain indefinitely incomplete inside a completed plan.

## Decision

- `tasks.yaml` v1 has no `optional` field or optional-task kind.
- Every executable task present in the active plan is required for spec-level completion.
- A useful deferrable test task must either remain a normal required task or move into explicitly revised future scope.
- Removing or deferring such work is a task-plan and, when applicable, requirements-scope revision; it is not an execution-state shortcut.
- The status model does not add `skipped` merely to reproduce inherited optional-task behavior.
- Target task generation, status, validation, and release logic do not recognize an optional task category.
- Supplying an `optional` key in target `tasks.yaml` fails strict schema validation as an unsupported field once executable task objects are wired into the root schema.

## Migration

- Migration detects inherited `- [ ]*` entries and stops for explicit resolution.
- The user must choose whether each entry becomes a normal required task or is removed from the active plan through the appropriate scope revision.
- Migration must not silently drop an incomplete optional task or preserve a hidden completion exemption.
- Historical source content remains available through Git and any project-created release references.

## Consequences

- Completion means all tasks in the active plan are complete and unblocked.
- Status output needs no separate optional count.
- Task-plan review has a clearer boundary between current required work and future ideas.
- Migration has one deliberate compatibility break from the inherited checkbox grammar.
