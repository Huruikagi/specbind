---
name: sb-validate-implementation
description: Use only when every Task for a named Spec is complete and the user explicitly asks for lifecycle validation against the active Requirement IDs that records completion on GO. For a consequence-free claim check, use sb-verify-completion.
argument-hint: "<spec>"
---

# Validate one Spec's implementation

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

The gate between "every task is recorded complete" and "this Spec is done".

Enter this workflow only with explicit authority to record completion on `GO`.
When the request could instead be a consequence-free truth check, use
`sb-verify-completion`; completed Tasks and words such as "done" do not resolve
that ambiguity or authorize mutation.

Your verdict writes durable evidence, so it has to be earned. **You validate;
you never repair.**

**Scope the claim to the active Requirement IDs.** The Requirements document can
retain requirements that are outside this active change. They are not completion
blockers: do not report the Spec incomplete because one of them is unimplemented.
After traceability passes, the approved Design artifacts' `requirement_ids` are
the exact active set you assess.

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

`READY` means only that this lifecycle and checkout state may now be validated.
It does not inspect the project Validation adapter, identify required commands,
or claim that any validation passed; those judgments begin in sections 2 and 3.

That revision is what you validate against and what you will submit. If
preflight fails, report the diagnostic and stop — it is naming a real
precondition, not an obstacle to route around.

**Validate at the final implementation state.** A later project-content commit
stales completion evidence, so a Spec validated before other work landed needs
revalidating rather than trusting an earlier verdict.

One exact lifecycle exception does not change that implementation state:
`milestone bind-release` may change only the active Roadmap's `target_release`.
Rust proves that transition structurally, so binding or explicit rebinding after
acceptance preserves completion freshness. Any Roadmap scope or body change,
release-policy edit, version bump in project source, or other project change
still stales it.

## 2. Read what completion would mean

```sh
specbind artifact read <spec> requirements --for consume
specbind artifact read <spec> design/main --for consume
specbind artifact read <spec> contract --for consume
specbind tasks list <spec>
specbind check traceability <spec>
```

Use `specbind artifact list <spec>` when the design is split. Read implementation
notes if the Spec has them.

Validate the **active Requirement IDs**, not every Requirement retained in the
current Requirements document. A clean traceability result establishes that the
union of the approved Design artifacts' `requirement_ids` is exactly the active
set. Use that set for coverage.

Then the standard you are held to:

```sh
specbind protocol read completion-verification
```

Its core rule governs everything below: **reject any claim broader than its
evidence.** "The whole Spec is implemented" is the broadest claim in the
workflow.

## 3. Gather the evidence

Read the project-specific validation procedure before fixing the required set:

```sh
specbind adapter read validation --for consume
```

`NO_CHANGE ADAPTER_ABSENT`, `NO_CHANGE ADAPTER_SCAFFOLD`, or an intentionally
empty body adds no project-specific work. Returned active guidance adds every
applicable procedure to this run. It supplements the
mandatory protocol and canonical project checks; it never replaces, waives,
narrows, or declares them passed.

Interpret the free-form guidance rather than treating Markdown or code blocks as
an executable hook. It may require commands, browser or device interaction,
connected tools such as MCP servers, manual checks, setup, observable success,
or cleanup. Apply the user's request and normal tool, credential, and external
mutation boundaries independently: the adapter grants no permission. Never put
a secret in recorded evidence.

Fix the complete required set before running anything. An applicable adapter
check that returns a known mismatch is `NO-GO`. One that is mandatory but cannot
be performed because its command, environment, credential, device, or tool is
unavailable is `MANUAL_VERIFY_REQUIRED`; do not invent or substitute a weaker
route. The validator may perform authorized setup and cleanup, but it must not
edit source or repair a finding it will judge.

Derive the required mechanical checks from this project's own automation and
conventions — its test command, build, lint, typecheck, smoke. Fix that required
set **before** running anything, then run those exact commands.

- Record what they actually returned. Never report a command you did not
  execute, and never submit a check whose exit status you did not see.
- The CLI cannot detect this. It accepts command text and cannot know whether
  anything ran, so the honesty of the record rests entirely here.
- Preserve the executed command verbatim in `mechanicalChecks.command`, including
  environment assignments, the complete argument string, and quoting needed to
  identify what ran. A label, shortened form, placeholder, or reconstructed
  equivalent is not evidence. Compare the JSON candidate with the command you
  executed before calling `completion accept`.
- A failing check is `NO-GO`. Never swap in a cheaper command that passes,
  narrow a check, or skip a case to reach green.
- Around each canonical project command, capture `git status --short`
  immediately before and after. Do not clean between the command and the second
  snapshot. A zero exit code that creates caches, reports, or other untracked
  output is `NO-GO`; name the paths and return them to implementation so the
  command itself becomes repeatably clean.
- A declared canonical command that is missing or cannot execute is
  `MANUAL_VERIFY_REQUIRED`. Stop: do not invoke its underlying test runner
  directly, reconstruct what the script probably did, or substitute a command
  that reaches the same tests. That is still a different check.

### Dispatch the independent dimensions

When the Spec is large enough that reading every result here would crowd out the
judgment, dispatch these as fresh subagents with self-contained briefs, each
returning **structured findings, not raw output**:

Use the registered `specbind-reviewer` role when available, with ordinary fresh
subagents as the fallback. These dispatches collect independent evidence; they
do not own the final verdict.
Fallback is only for an absent role. A configured role whose model cannot start
is a configuration or environment failure, not permission to change models.

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
`sb-implement`. **Never fix them yourself** — a validator that repairs is
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

Record a validation-adapter step as `custom` only when an exact command actually
ran and returned zero. Browser, device, connected-tool, or manual observations
without an exact command remain run-scoped semantic evidence; do not invent a
command or a persisted pass flag for them. They are still mandatory for this
run's `GO` when the adapter requires them.

### When several Specs converge

The first acceptance at a revision needs a completely clean worktree. Each
acceptance writes that Spec's `spec.yaml`, so later ones at the same revision
tolerate exactly the other participants' `implementation` → `release_ready`
transitions — **and nothing else**. Any other change fails closed.

A pending release binding is not an acceptance exception. Checkpoint it first;
then later Specs may validate at the new clean `HEAD` while earlier completion
evidence remains fresh at its original implementation revision.

**Commit that metadata set together**, as one commit containing only those
files. Do not interleave other work into it.

If the project's Git adapter says not to commit, stop after the acceptances and
report that the metadata set is uncommitted, rather than leaving the milestone
in a state only you understand.

```sh
specbind adapter read git --for consume
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
  belongs to `sb-implement`, and no synthetic Spec is created for it.
- Repair nothing. No source changes, no weakened checks, no edits to
  requirements, design, the contract, or the plan.
- Approve no gate and record no task progress.
- Commit only the accepted completion metadata set, and only where the adapter
  permits it.
- Report in the project's language: the verdict, the checks you ran and their
  results, what each assessment concluded, what must change on `NO-GO` or who
  must verify on `MANUAL_VERIFY_REQUIRED`, and what runs next.
