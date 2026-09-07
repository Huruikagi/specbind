# Report an active milestone

Use this procedure only when the user asks about the active milestone or does
not name one Spec.

## Read the aggregate state

```sh
specbind milestone status
```

`NO_CHANGE NO_ACTIVE_MILESTONE` is not a failure. Report that no milestone is
active and stop.

The milestone projection is the complete first source for stage, ordered
items, dependency waits, Task progress, recorded Task blockers, actionable
work, current workflow blockers, and release blockers. Do not replace it with
artifact inspection or reconstruct an older clean snapshot.

Add only the focused read the projection calls for:

- `specbind milestone review status` when Contract Review is absent, stale, or
  invalid and the user needs the exact review condition.
- `specbind check traceability <spec>` or `specbind check contracts` when a
  diagnostic must be attributed to a specific artifact.

Do not run `spec status` or enumerate every Task merely to repeat fields already
present in the milestone projection.

## Explain progress and stopping conditions

For every blocked Spec, report:

- its completed, pending, and blocked Task counts;
- each blocked Task ID and its recorded reason;
- any independent actionable Milestone items that can still proceed; and
- that the recorded blocker must be resolved before the owning workflow can
  reopen the Task and continue.

A blocked Task is not actionable merely because its Spec remains in
`implementation`. Do not route it back to implementation until the CLI reports
a next Task or the blocker has been resolved and reopened by the owning
workflow.

`TASKS_BLOCKED` names the real Task-level stop. Do not replace it with a generic
dirty-worktree explanation. Report `WORKTREE_NOT_CLEAN` only when the CLI lists
it as an additional current blocker. Its presence grants no authority to
commit, stash, discard, or otherwise reconcile changes.

End with one compact handoff: what is complete, what is blocked and why, what
independent work remains actionable, and which condition must change next.
