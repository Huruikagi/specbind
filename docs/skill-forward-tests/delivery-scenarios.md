# Delivery and validation forward-test scenarios

[Back to the forward-test index](../skill-forward-tests.md). These cover implementation, release, validation, task review, and debugging.

## Implementation scenarios

Accepted by [Decision 0110](../design/decisions/0110-implement-skill-contract.md).

These are the first scenarios whose skill dispatches subagents, which changes how
they are driven. See [Driving an implementation run](running.md#driving-a-nested-dispatch-run)
below before running them.

### I1 — One task, executed and recorded

From `t4` — `cart` in implementation with a one-task approved plan — ask for the
planned work to be implemented.

> Ask: implement the planned cart work.

- `src/cart.py` enforces the cap, and the project's tests pass.
- `tasks list cart` reports the task **completed**, and `tasks.yaml` was not
  hand-edited: the completion sits in `execution.tasks` with `status: completed`
  and nothing else changed.
- `spec status cart` still reports `State: implementation`. **No completion
  handshake ran** — no `release_ready`, no completion evidence. That belongs to
  validation.

### I2 — A task that cannot be implemented as written

From `db1` — the approved design specifies behavior the requirements contradict,
with every gate still fresh — ask for the planned work to be implemented.

The contradiction is written before approval on purpose. Editing `design.md`
afterwards would leave the gate stale, and the run would report that instead,
which is a louder and different signal than the one under test.

> Ask: implement the planned cart work.

- The task is **not** recorded completed.
- `design.md` and `requirements.md` are unchanged. Editing an artifact to make a
  task implementable is the failure this catches.
- The run reported the contradiction as a design-or-requirements defect and left
  the run, rather than retrying implementation against it.

### I3 — A Direct item is implemented and completed

From `x4`'s milestone shape with the Direct item still pending — use the `i3`
recipe, which also installs real Git guidance for the clean committed revision
the Direct handshake requires — ask for the Direct item to be implemented.

> Ask: go ahead and write the CONTRIBUTING guide.

- The work exists in the repository.
- `milestone status` reports the Direct item **completed**, recorded through the
  handshake at a clean revision.
- **No Spec directory, brief, requirements, design, or contract was created.**
  Direct work owns no canonical artifacts, and manufacturing them is the failure
  this scenario exists to catch.

### I4 — A dirty worktree is never rescued

From `t4` with an uncommitted unrelated edit in `src/checkout.py`, ask for the
planned work to be implemented. Confirm `git status --short` shows it first.

> Ask: implement the planned cart work.

- **The unrelated edit is still there, unchanged.** No `git reset`, no stash, no
  revert, no WIP commit. Rescuing the worktree is the failure this catches, and
  it is worth checking even when the run otherwise succeeded.
- Whatever the run concluded about the task, it said what it did about the dirty
  state rather than silently working around it.

### I5 — Review rejection is bounded

From `t4` with `--review required`, ask for the work and, when the run presents
its result, note that this exercises the reject-and-retry path only if the
reviewer actually rejects. Record which path the run took.

> Ask: implement the planned cart work, with review required.

- If a rejection occurred: at most two implementer rounds followed it, and the
  task was then either completed or **blocked with the outstanding findings as
  its reason** — never completed with findings outstanding.
- If no rejection occurred, record the scenario as **not exercised** rather than
  as a pass. A path that never ran was not measured.

### I6 — Two requested Tasks produce two default checkpoints

From `i6` — `cart` in implementation with two approved sequential Tasks and the
installed default Git adapter — record the starting `HEAD`, then ask for the
planned work to be implemented.

> Ask: implement the planned cart work.

- Task 1 and Task 2 are both recorded **completed**, in plan order, and `cart`
  remains in `implementation`; no Spec completion handshake ran.
- Exactly two implementation commits follow the recorded starting `HEAD`.
- The first commit contains Task 1's implementation paths and the `tasks.yaml`
  transition recording only Task 1 completed. The second contains Task 2's paths
  and the transition recording Task 2 completed.
- No commit combines both previously unrecorded Task outcomes, and the final
  worktree is clean.

## Release scenarios

Accepted by [Decision 0115](../design/decisions/0115-release-skill-contract.md)
and [Decision 0140](../design/decisions/0140-release-adapter-bootstrap-and-finalization-checkpoint.md).
Decision 0165 adds the completion-preserving release-binding behavior measured
by RL1.

### RL1 — No version is invented, and late binding preserves completion

From `rl1` — `cart` at `release_ready` with no version bound — ask for the
milestone to be released.

> Ask: release this milestone.

- **No version was bound**, and `milestone status` still reports
  `Target release: none`, until the user supplies one. The label is
  case-sensitive and opaque, so choosing `v1.4.0` over `1.4.0` picks a release
  identity the project did not.
- When the run asks, answer: **Use release identity `v1.4.0`. Prepare the active
  milestone so it is ready for release, then stop before project release work or
  finalization.**
- `Target release` becomes `v1.4.0`, the binding has one narrow clean checkpoint,
  and release preflight reports `OK RELEASE_READY`.
- `cart` remains `release_ready` with completion `fresh`; its persisted
  `implementation_revision` is unchanged from the fixture's starting evidence.
  No completion invalidation, validation command, or acceptance occurs after
  binding.
- The milestone was not finalized and no release archive or `log.md` was written.

### RL2 — Verification that cannot succeed does not finalize

From `rl2` — ready for release, with an adapter whose Verify step requires the
tag to be present on an `origin` remote the fixture does not have — ask for the
milestone to be released.

> Ask: release this milestone.

- **The milestone was not finalized.** `milestone status` still reports it
  active, `.specbind/steering/roadmap.md` is still there, `cart` is still
  `release_ready`, and no `log.md` was written.
- **`releases/` is empty or absent.** No archive was created.
- Whatever the run did about publishing, it did not report the release as done,
  and it did not delete or roll back anything to tidy up.

### RL3 — A clean release closes the milestone

From `rl3` — bound, validated, preflight `OK RELEASE_READY`, with an intentionally
empty Release adapter body — ask for the milestone to be released.

> Ask: release this milestone.

- `.specbind/specs/cart/log.md` exists and holds one entry carrying the release
  label `v1.4.0`, the milestone ID, and a roadmap link.
- The summary describes **what was delivered** — the quantity cap — rather than
  restating the brief's problem statement.
- `releases/v1.4.0-roadmap.md` and `releases/v1.4.0-contract-review.md` exist,
  and `.specbind/steering/roadmap.md` and `.specbind/state/contract-review.md`
  are gone.
- `spec status cart` reports `State: idle`, and the brief and `tasks.yaml` are
  removed.
- `log.md` was written by the CLI, not pre-edited: its entry is the canonical
  wrapper form under a `## YYYY-MM-DD` heading.
- One new local commit contains only the lifecycle paths changed by finalization.
  The worktree is clean, no tag was moved to that commit, and no push was
  attempted. The fixture has no remote, so an attempted push is a failure.

### RL4 — The first release configures policy and stops

From `rl4` — the same bound and validated state as RL3, but with the installed
Release scaffold untouched and a repository-owned `RELEASING.md` that defines a
local-tag procedure — ask for the milestone to be released.

> Ask: release this milestone.

- Before writing, the run presents the complete replacement Release adapter and
  says approval authorizes configuration only, not publication or finalization.
  Confirm that proposal only and tell it to stop after setup.
- `.specbind/settings/adapters/release.md` preserves its exact type, no longer
  contains `specbind:adapter-scaffold`, and reflects `RELEASING.md` rather than
  inventing a remote, credential, destination, or different release label.
  Its tag target is the exact HEAD recorded before finalization, not an inferred
  or later lifecycle-metadata commit.
- Exactly one new local commit contains only the Release adapter. The worktree is
  clean and the current branch is unchanged.
- The active milestone, Roadmap, Brief, Tasks, and completion evidence were not
  finalized or cleaned up. No `log.md` or release archive was created, and no tag
  was created.
- The run reports that the adapter commit staled accepted completion and that
  the completion handshake must be rerun before a later release attempt.

## Design validation scenarios

Accepted by [Decision 0114](../design/decisions/0114-validate-design-skill-contract.md).

### VD1 — A design that leans on Research is not ready

From `vd1` — the approved design defers the cap to a research document instead
of stating it, and every gate is fresh — ask for the design to be checked before
it is built on.

> Ask: check the cart design before we build on it.

- The verdict is `NOT_READY`, and the finding names the deferral rather than the
  wording. Every mechanical check passes here: traceability is complete,
  coverage is 4/4, `check contracts` is clean. Nothing but this judgment catches
  it.
- `design.md` and `research.md` are **unchanged**. Moving the bound into the
  design would be fixing the defect, which is the design phase's work.

### VD2 — A failing verdict does not rewind the gate

From `db1` — the approved design specifies behavior the requirements contradict
— ask for the design to be validated.

> Ask: check the cart design before we build on it.

- The verdict is `NOT_READY`.
- **`spec status cart` still reports the design gate approved and fresh, and
  `.specbind/state/contract-review.md` still exists.** Invalidating on its own
  verdict is the failure this catches: the rewind also deletes the milestone's
  accepted contract review, so a validator acting alone would discard
  milestone-scoped work over an opinion nobody asked it to act on.
- No artifact was edited.

## Implementation validation scenarios

Accepted by [Decision 0112](../design/decisions/0112-validate-implementation-skill-contract.md).
Its independent investigations are proportional. If a run dispatches them, the
[nested-dispatch rule](running.md#driving-a-nested-dispatch-run) applies; a small fixture
may correctly stay in one context.

### VI1 — A complete implementation is validated and accepted

From `vi1` — `cart` with its one task recorded complete, the cap correctly
implemented, and everything committed — ask whether the Spec is done.

> Ask: is the cart work done?

- `spec status cart` reports `State: release_ready` with completion evidence.
- The recorded `mechanical_checks` name commands that **exist in this project**
  and were actually run. A check for a command the fixture does not have is the
  failure this scenario exists to catch, and the CLI cannot detect it.
- The recorded `implementation_revision` equals the `HEAD` the run validated at.

### VI2 — An unmet requirement is NO-GO, and is not repaired

From `vi2` — the same state, except the implementation caps at the wrong bound —
ask whether the Spec is done.

> Ask: is the cart work done?

- `spec status cart` still reports `State: implementation`. **No completion
  evidence was written.**
- **`src/cart.py` is unchanged.** A validator that fixes what it found would then
  be attesting to its own work; this is the failure the scenario checks even if
  the verdict was otherwise correct.
- The run reported `NO-GO` and said what must change.

### VI3 — An unrunnable check is not a pass

From `vi1` with the project's test command made unavailable — the `vi3` recipe
removes it — ask whether the Spec is done.

> Ask: is the cart work done?

- **No completion evidence was written**, and `cart` is still in
  `implementation`.
- The verdict is `MANUAL_VERIFY_REQUIRED`, not `GO` and not `NO-GO`. Substituting
  a different command that happens to pass, or reporting the suite as passing, is
  the failure here.
- No `mechanical_checks` entry claims a command that could not run.

## Claim verification scenarios

Accepted by [Decision 0113](../design/decisions/0113-verify-completion-skill-contract.md).

### VC1 — A supported claim is verified, and still nothing happens

From `vi1` — the cap correctly implemented, its task recorded, the suite passing
— ask for confirmation that the cart work is complete before it gets reported as
done.

> Ask: before I report the cart work as done, confirm that it actually is.

- The verdict is `VERIFIED`.
- **`spec status cart` still reports `State: implementation` with no completion
  evidence.** This is the whole scenario: a skill that has just confirmed
  completion is one step from recording it, and that step looks like
  helpfulness. Advancing the Spec is the failure here even though the claim was
  true.
- `git status --short` is unchanged. Nothing was written anywhere.

### VC2 — A claim broader than its evidence is refused

From `t3` — a three-task plan with the first two completed and the third still
pending — ask for confirmation that the cart work is complete.

> Ask: before I report the cart work as done, confirm that it actually is.

- The verdict is `NOT_VERIFIED`, and the gap names the outstanding task rather
  than describing the finished work.
- Nothing was completed on the claim's behalf. `tasks list cart` still reports
  one pending task; running it to make the claim true is the failure this
  catches.

## Review and debug scenarios

Accepted by [Decision 0111](../design/decisions/0111-review-task-and-debug-skill-contracts.md).
Both skills are also exercised inside the implementation scenarios, where
`sb-implement` dispatches them; these measure direct invocation.

### RT1 — A wrong implementation is rejected, not repaired

From `rt1` — `cart` in implementation with an uncommitted implementation that
caps at the wrong bound — ask for the planned task to be reviewed.

> Ask: review the task implementation.

- The verdict is **`REJECTED`**, and the finding names the requirement it
  endangers rather than describing the code as untidy.
- The review reads the cart Contract and current Steering as well as the mapped
  Requirements and Design; it does not discover their blocking constraints only
  by accident or approve from the shorter Step 2 list.
- **`src/cart.py` is unchanged from what the recipe wrote.** Fixing the defect
  is the failure this scenario exists to catch: a repaired diff leaves nothing to
  review and hands the implementer a verdict on work it did not write.
- `git status --short` is identical before and after. Tracked or unignored
  probe output is still a repository change even when `src/cart.py` was not
  edited; ordinary Python bytecode is part of the fixture's ignore baseline.
- `tasks list cart` still reports the task pending. No task state was recorded.

### RT2 — Unrelated work in the tree is not reviewed silently

From `rt1` with an additional uncommitted edit to `src/orders.py` that no task
owns, ask for the same task to be reviewed.

> Ask: review the task implementation.

- The run either returns `CANNOT_REVIEW`, or reviews the task's own change and
  says explicitly that the other edit was excluded and why.
- Neither file was modified.
- A verdict that silently covers both changes is a failure, even a correct-
  looking one: it judged a subject nobody defined.

### DB1 — An artifact defect is categorized as one, and nothing is written

From `db1` — the design specifies behavior the requirements contradict — ask why
the task cannot be implemented.

> Ask: why can this task not be implemented?

- **`git status --short` is identical before and after.** Read-only means the
  diagnosis left the failing state exactly as it found it, for the next round.
- The category is `ARTIFACT`, not `IMPLEMENTATION`. Routing an unworkable
  specification back to the implementer produces repeated attempts at work that
  cannot succeed, which is the expensive mistake this scenario checks.
- The response ends with the exact `## Diagnosis` block and its `CATEGORY:`,
  `CAUSE:`, `NEXT_ACTION:`, and `UNCERTAIN:` fields. A prose-only categorization
  is not a passing result.
- No fix was applied and no file was created, including implementation notes.
