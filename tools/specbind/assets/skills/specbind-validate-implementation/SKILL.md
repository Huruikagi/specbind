---
name: specbind-validate-implementation
description: Decide whether a named Spec's completed implementation is actually done. Validate it against requirements and design, then record completion evidence when — and only when — the verdict is GO.
argument-hint: "<spec>"
---

# Validate one Spec's implementation

The gate between "every task is recorded complete" and "this Spec is done".

Your verdict writes durable evidence, so it has to be earned. **You validate;
you never repair.**

## 1. Preflight

```sh
specbind spec status <spec>
specbind milestone status
specbind spec completion preflight <spec>
```

Preflight is read-only and checks what you cannot: state, gate freshness, task
completion, contract review freshness, and a clean committed `HEAD`. On success
it returns one thing you need:

```text
Implementation revision: <full commit id>
```

That revision is what you validate against and what you will submit. If
preflight fails, report the diagnostic and stop — it is naming a real
precondition, not an obstacle to route around.

**Validate at the final revision.** A later commit anywhere in the project
stales completion evidence, so a Spec validated before other work landed needs
revalidating rather than trusting an earlier verdict.

That includes the release version. `milestone bind-release` writes the roadmap,
which is a non-metadata change, so binding **after** you accept completion stales
what you just recorded. If the version is already known, say it should be bound
before this handshake rather than after.

## 2. Read what completion would mean

```sh
specbind artifact read <spec> requirements
specbind artifact read <spec> design/main
specbind artifact read <spec> contract
specbind tasks list <spec>
specbind check traceability <spec>
```

Use `specbind artifact list <spec>` when the design is split. Read implementation
notes if the Spec has them.

Validate the **active Requirement IDs**, not every Requirement retained in the
current Requirements document. A clean traceability result establishes that the
union of the approved Design artifacts' `requirement_ids` is exactly the active
set. Use that set for coverage. A Requirement outside it is not a completion
blocker for this active change.

Then the standard you are held to:

```sh
specbind protocol read completion-verification
```

Its core rule governs everything below: **reject any claim broader than its
evidence.** "The whole Spec is implemented" is the broadest claim in the
workflow.

## 3. Gather the evidence

Derive the required mechanical checks from this project's own automation and
conventions — its test command, build, lint, typecheck, smoke. Fix that required
set **before** running anything, then run those exact commands.

- Record what they actually returned. Never report a command you did not
  execute, and never submit a check whose exit status you did not see.
- The CLI cannot detect this. It accepts command text and cannot know whether
  anything ran, so the honesty of the record rests entirely here.
- A failing check is `NO-GO`. Never swap in a cheaper command that passes,
  narrow a check, or skip a case to reach green.
- A declared canonical command that is missing or cannot execute is
  `MANUAL_VERIFY_REQUIRED`. Stop: do not invoke its underlying test runner
  directly, reconstruct what the script probably did, or substitute a command
  that reaches the same tests. That is still a different check.

### Dispatch the independent dimensions

When the Spec is large enough that reading every result here would crowd out the
judgment, dispatch these as fresh subagents with self-contained briefs, each
returning **structured findings, not raw output**:

- full-suite results
- runtime liveness: does the built artifact reach its first usable state
- active requirement coverage: genuinely delivered, not merely referenced
- cross-task integration: do the parts work together, not just individually
- design alignment, end to end
- anything left blocked

**Synthesize the verdict here, never in a subagent.** The decision needs the
whole picture and no dispatched part has it. For a small Spec whose checks are
two commands, skip dispatch.

## 4. Decide

Three verdicts, and only one of them writes anything:

- **`GO`** — every check passed and every assessment holds.
- **`NO-GO`** — something is wrong, and you can say what.
- **`MANUAL_VERIFY_REQUIRED`** — a mandatory check could not be performed: no
  canonical command, the environment is unavailable, a manual step cannot be run
  here.

**`MANUAL_VERIFY_REQUIRED` is not a weaker `NO-GO`, and never a route to `GO`.**
They carry different information — one says something is known to be wrong, the
other says nothing is known either way. Turning the second into a pass is how an
unverified Spec reaches `release_ready`.

A passing test suite alone is never enough for `GO`. Tests exercise what someone
thought to test; they do not show the thing runs, that the requirements are
covered, or that the pieces fit.

For `NO-GO`, name each issue and what must change. Findings go back to
`specbind-implement`. **Never fix them yourself** — a validator that repairs is
attesting to its own work.

## 5. Accept, on GO only

```sh
specbind spec completion accept <spec> --evidence -
```

The candidate is strict JSON on stdin:

```json
{
  "schemaVersion": 1,
  "implementationRevision": "<the full preflight revision>",
  "mechanicalChecks": [
    { "kind": "test", "command": "npm test", "exitCode": 0 },
    { "kind": "build", "command": "npm run build", "exitCode": 0, "workingDirectory": "app" }
  ]
}
```

`kind` is one of `test`, `build`, `smoke`, `lint`, `typecheck`, `custom`. Every
`exitCode` is `0` — a failing check is a `NO-GO`, not an entry. `workingDirectory`
is project-root-relative and omitted when it is the project root. Supply no
timestamp, fingerprint, or pass flag; the CLI owns those.

Put no secret in a command string. Names of environment variables are fine;
values are not.

### When several Specs converge

The first acceptance at a revision needs a completely clean worktree. Each
acceptance writes that Spec's `spec.yaml`, so later ones at the same revision
tolerate exactly the other participants' `implementation` → `release_ready`
transitions — **and nothing else**. Any other change fails closed.

**Commit that metadata set together**, as one commit containing only those
files. Do not interleave other work into it.

If the project's Git adapter says not to commit, stop after the acceptances and
report that the metadata set is uncommitted, rather than leaving the milestone
in a state only you understand.

```sh
specbind adapter read git
```

## When evidence needs clearing

A Spec at `release_ready` whose evidence no longer holds is revalidated. State
what invalidation clears — completion evidence only; requirements, design, and
tasks stay approved — and run it after the user confirms:

```sh
specbind spec completion invalidate <spec>
```

Never use it to clear a path to a new `GO`. A refused validation is information
about the implementation.

## Boundaries

- Validate **one Spec**. A Direct item is not validated here — Direct completion
  belongs to `specbind-implement`, and no synthetic Spec is created for it.
- Repair nothing. No source changes, no weakened checks, no edits to
  requirements, design, the contract, or the plan.
- Approve no gate and record no task progress.
- Commit only the accepted completion metadata set, and only where the adapter
  permits it.
- Report in the project's language: the verdict, the checks you ran and their
  results, what each assessment concluded, what must change on `NO-GO` or who
  must verify on `MANUAL_VERIFY_REQUIRED`, and what runs next.
