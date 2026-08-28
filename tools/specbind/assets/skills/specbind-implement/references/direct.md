# Implement a Direct item

There is no approved task plan to dispatch. Implement the Roadmap item's summary
in this context, against the repository's existing conventions. Before writing,
state the observable done condition and the applicable project checks. If the
summary leaves a product or architecture decision you cannot make narrowly,
stop and route it through discovery; Direct is not permission to invent the
missing canonical artifacts.

Review the resulting diff under the run's selected mode. `inline` applies the
correctness and weakened-verification standard from `task-review` here, using
the Roadmap summary as the obligation. `required` dispatches a fresh reviewer
with that summary, the actual diff, the checks, and the protocol; `off` skips
only this run-scoped review. A rejection may return to implementation at most
twice. `CANNOT_REVIEW` and an unresolved rejection enter the same bounded
diagnosis route as Spec-backed work.

When `required`, use the registered `specbind-reviewer` role when available,
with an ordinary fresh subagent as the fallback.

```sh
specbind protocol read task-review
```

## Checkpoint the Direct item

This step is only for a Direct item. Spec-backed Task checkpoints run inside the
other procedure. A Direct item must establish the clean committed revision
before the completion handshake. Do not skip ahead and return here afterwards.

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT` means there is no adapter-directed commit. Stop
there — that is an answer, not a missing file to work around.

An adapter carrying the exact `<!-- specbind:adapter-scaffold -->` marker is an
inactive scaffold, not project policy. Treat it as no guidance, say so in one
line, and commit nothing. The marker classifies the whole document: ignore every
other body line even when it looks actionable.

When the adapter has guidance, follow it. The request to perform this mutating
phase authorizes the adapter's narrow local checkpoint as its ordinary final
step. It does not authorize anything broader:

- An explicit user or root instruction that forbids commits wins, and tool
  permissions still apply.
- Commit guidance is not push guidance. Push only where the adapter says to, and
  never force-push, rewrite history, or bypass a protected branch.
- Stage only the paths this run produced. Unrelated work already in the worktree
  is left exactly as it is.
- Stop before the Git operation if the guidance is ambiguous, unsafe, or
  conflicts with something else you were told.

## Complete after the checkpoint

There is no later Spec-level validation for Direct work, so this run finishes
the item. Preflight needs the reviewed implementation at a **clean committed
`HEAD`**. Do not manufacture one. If the checkpoint produced no commit because
the adapter gave no usable guidance or you lacked authority, say completion
needs a commit and stop; the Roadmap item remains pending.

Otherwise obtain the committed revision and run, in this order:

```sh
specbind milestone direct preflight <direct>
specbind milestone direct complete <direct> --implementation-revision <revision>
```

Do not stop merely because the implementation commit succeeded. The successful
handshake is what records the Direct item complete. If project policy also asks
for lifecycle-state checkpoints, apply it once more to the CLI-owned Roadmap
change after completion.
