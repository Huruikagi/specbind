---
name: sb-validate-implementation
description: Validate completed Tasks against active Requirement IDs and record completion on GO under an explicit lifecycle-validation request or ordinary Drive authority. Also owns completion withdrawal under an evidenced Drive replan recovery handoff. For a consequence-free claim check, use sb-verify-completion.
argument-hint: "<spec> [<spec> ...]"
---

# Validate Spec implementation at one committed revision

For selected `_Retired_` Requirement IDs, verify the continuation or cessation
explained beside the marker against baseline obligations, implementation and
Task evidence. Do not test the retired prose as a current promise or accept the
marker as completion evidence. Check preserved successor behavior and affected
consumers, interfaces and data where relevant. Old retirement outside the active
set does not create new verification work.

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

The gate between "every task is recorded complete" and "this Spec is done".

Enter this workflow only with explicit authority to record completion on `GO`.
An ordinary Drive request supplies that authority for the exact actionable Spec
set in its handoff, unless the user explicitly restricted completion recording
or mutation. No additional confirmation is needed for that handoff. A bare
status result or an agent's unsupported assertion of authority is insufficient.
The sole early-entry exception is an evidenced Drive replan handoff with
completion-withdrawal-only authority: go directly to "When evidence needs
clearing" before preflight. That mode grants no validation or acceptance and
does not require milestone Task convergence.
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

Normally validate one named Spec. An explicit set supplied by the user or by
Drive's current actionable entries is one validation run with a separate
assessment and verdict for each Spec. Do not expand that set yourself.
For a set, the batch CLI runs **every target's preflight before any acceptance**.
Supply the exact IDs using repeated `--spec` options:

```sh
specbind spec completion batch preflight --spec <first> --spec <second>
```

Retain its JSON `implementationRevision`, `targets` and `paths`. This is the
same implementation revision for the entire set. A failed preflight stops the
entire set without acceptance. For a single Spec, keep the existing preflight:

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

Repeat the domain reads and semantic assessments for every target in the set.
Shared canonical checks may run once at the common revision, but passing them
does not establish another Spec's requirement coverage or semantic verdict.

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

Read the project's command declarations before choosing mechanical checks:

```sh
specbind steering list
specbind steering read <selector> --for consume
```

Read the listed Steering documents that govern verification or conventions,
then the relevant project automation and documentation. A runner file's presence
or shebang does not establish the canonical invocation. Fix the declared test,
build, lint, typecheck and smoke commands **before** running anything, then run
those exact commands. Preserve the declared executable, arguments and working
directory: for example, `sh scripts/test.sh` is not permission to substitute
`./scripts/test.sh` or invoke its test framework directly. A host-shell wrapper
may establish PATH and capture status, but must retain that exact inner command.

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

These are independent questions, not a requirement to run one command per
question. One canonical project command may support several answers only when
its inspected coverage and fresh output genuinely prove each one. For a
library-only artifact with no project-defined startup command, canonical tests
may establish runtime liveness when they actually load the artifact and execute
its public first usable operation. Do not invent an ad-hoc shell, language, or
smoke command solely to fill the runtime slot. If project-owned commands and an
applicable Validation adapter do not prove that boundary, return
`MANUAL_VERIFY_REQUIRED`.

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

For a set, use "When several Specs converge" below. For one Spec:

```sh
specbind spec completion accept <spec> --evidence -
```

The single-Spec candidate is strict JSON on stdin:

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

Complete all required checks and per-Spec assessments before accepting any
target. If any target is `NO-GO` or `MANUAL_VERIFY_REQUIRED`, accept none;
return each target's findings to its owner. Do not accept a passing subset
before repairs: the repair would stale its evidence again.

After every target has `GO`, confirm HEAD still equals the common preflight
revision and the worktree is clean. If either check fails before acceptance,
accept none and return `BLOCKED`: name the entire target set, common preflight
revision, observed HEAD, dirty paths, and the failed check. Discard the old
candidates and verdicts for acceptance purposes; do not repair, commit, clean,
or retry in this run. Tell the caller to resolve the intervening work and start
a new validation run with all current targets, all preflights, checks and
per-Spec assessments at one clean revision. For a Drive caller, the restart
entry point is a new user invocation of Drive, which schedules a fresh ordinary
validation owner from current status. This is not a `RETRYABLE` handoff.

When both checks hold, read the batch input schema and submit one candidate for
the exact preflight target set. Reuse the actual checks from this run; the CLI
does not execute them or decide semantic `GO`.

```sh
specbind schema read completion-batch/v1
specbind spec completion batch accept --evidence <external-candidate-file>
```

The strict JSON contains one fixed revision and per-Spec check evidence:

```json
{
  "schemaVersion": 1,
  "implementationRevision": "<the full batch preflight revision>",
  "targets": [
    { "spec": "<first>", "mechanicalChecks": [{ "kind": "test", "command": "npm test", "exitCode": 0 }] },
    { "spec": "<second>", "mechanicalChecks": [{ "kind": "test", "command": "npm test", "exitCode": 0 }] }
  ]
}
```

Keep that candidate outside the project, or use `--evidence -` on stdin. Retain
the same candidate for checkpoint verification. The CLI validates the whole
set before writing, rechecks the fixed HEAD and exact pending changes at each
write, and returns JSON with `accepted`, `pending`, `paths` and `issues`.
It owns the sequential writes; do not replace it with a per-Spec accept loop.

Only `COMPLETION_BATCH_ACCEPTED` with every target accepted proceeds to the Git
adapter. **Commit that metadata set together**, as one commit containing only
the returned paths. Then run:

```sh
specbind spec completion batch verify --evidence <external-candidate-file>
```

Require `COMPLETION_BATCH_VERIFIED`: the CLI proves one narrow checkpoint after
the implementation revision, unchanged earlier gates and fresh evidence for
every target. Reread milestone status for the next handoff. A failed or missing
checkpoint is not a completed validation run.

On any failed acceptance, checkpoint or verification, stop immediately and
preserve the CLI report, common revision, observed HEAD and dirty paths in the
handoff. `COMPLETION_BATCH_INTERRUPTED` can leave a confirmed accepted prefix;
`pending` is not a retry instruction. An abrupt process failure may leave no
report, so inspect current status without inferring that nothing was written.
Do not roll back valid evidence, retry at unchanged state, or commit a subset
and start remaining targets at a new HEAD. Report partial acceptance without
claiming completion. A new run needs the full fresh validation described above.

If the project's Git adapter says not to commit, stop after acceptance, report
the exact uncommitted metadata set, and skip checkpoint verification.

```sh
specbind adapter read git --for consume
```

## When evidence needs clearing

For ordinary revalidation or an evidenced Drive completion-withdrawal-only
handoff, read [Completion withdrawal](references/withdrawal.md) completely before
invalidating anything. It owns confirmation, causality checks, the independent
withdrawal checkpoint, and whether to return to preflight or to Drive. Do not
load it for ordinary completion acceptance.

## Boundaries

- Validate **one Spec or an explicitly supplied set** at one revision. A Direct
  item is not validated here — Direct completion
  belongs to `sb-implement`, and no synthetic Spec is created for it.
- Repair nothing. No source changes, no weakened checks, no edits to
  requirements, design, the contract, or the plan.
- Approve no gate and record no task progress.
- Commit only the independently verified invalidation delta or the accepted
  completion metadata set at their respective boundaries, and only where the
  adapter permits it. Never combine the two checkpoints.
- Report in the project's language: the verdict, the checks you ran and their
  results, what each assessment concluded, what must change on `NO-GO` or who
  must verify on `MANUAL_VERIFY_REQUIRED`, and what runs next.
