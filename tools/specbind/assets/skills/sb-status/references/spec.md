# Report one Spec

Use this procedure only when the user names one Spec or the conversation is
already unambiguously about one Spec.

## Read the Spec state

```sh
specbind spec status <spec>
```

This projection supplies lifecycle state, Gate freshness, Task totals, next
Tasks, recorded blockers, coverage, and diagnostics.

Use additional Task reads only when the answer needs their detail:

```sh
specbind tasks list <spec>
specbind tasks show <spec> <task-id>
```

- Use `tasks list` to show the plan order, individual progress, and effective
  waiting state.
- Use `tasks show` for a blocked Task when its details, prerequisites, or
  completion criteria materially explain what must be resolved.
- Use `specbind check traceability <spec>` only when a reported inconsistency
  must be attributed to a specific artifact.

## Explain the next step

Report what is complete, the exact next actionable Task when one exists, and
every blocked Task with its recorded reason. A blocked Task with no next Task is
a stop condition, not implementation work to retry.

State the condition that must be resolved before the owning workflow can reopen
the Task and continue. Do not infer an unrecorded solution from repository
files or strengthen the reason into plan repair, Task reordering, or an
implementation change. Do not run `tasks reopen` yourself; this Skill is
read-only. Do not append healthy State-health, fresh-Gate, or
semantic-alignment boilerplate unless the request or conclusion makes that
distinction material.
