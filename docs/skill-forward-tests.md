# Skill forward tests

Mechanical conformance runs in CI. Whether an agent given a skill actually
produces the intended result does not, because it is not mechanically decidable.
[Decision 0096](./design/decisions/0096-skill-asset-layout.md) accepts that
split and calls for behavioral verification against a fixture project. This
document is that procedure.

Run it when a skill changes materially, and before a release.

## The rule that makes this tractable

**Check artifacts and machine state. Never check prose.**

A run is non-deterministic in its path: two correct runs will word things
differently, ask different clarifying questions, and order their steps
differently. None of that is a result. What the run leaves behind is.

Every expectation below is something you can read with a command or a file, and
that a correct run must produce regardless of how it got there. If you find
yourself judging whether an explanation was well phrased, you have left the test
and started reviewing the writing.

A scenario passes when every listed expectation holds. It fails when one does
not — not when the agent took an unexpected route to satisfy them all.

## Setup

```bash
sh tools/specbind/scripts/forward-test-fixture.sh /tmp/specbind-fixture en
```

The script builds the release binary, creates a small real codebase, installs
SpecBind for both agents with project instructions, and seeds two steering
documents plus one established `cart` Spec. It refuses to touch an existing
directory; delete and rebuild between scenarios rather than reusing a dirtied
one.

Pass `ja` as the second argument to exercise the localized surface.

Scenarios that start from something further along have a recipe that builds and
verifies that state:

```bash
sh tools/specbind/scripts/forward-test-scenario.sh r4 /tmp/sb-r4 en
```

The recipe owns starting state only. The request and the expectations stay here.

Use a target path the agent session can address directly. On Windows a `/tmp`
path is a shell alias that some tools cannot resolve, and an agent that has to
guess at the real location is being tested on something other than the skill.

The script ships the release binary inside the fixture and prints the `export
PATH=` line for it. **Run that line before starting the session.** The skills
invoke `specbind` as a bare command because a real installed project has it on
PATH; a fixture without it tests whether the agent can guess an install location,
which is not what is under test. Two of the first runs stopped there and produced
no result at all.

Then start an agent session **with no prior context** in that directory. Context
carried from developing the skill is the most common way a forward test passes
for the wrong reason: the agent already knows what you meant.

## Driving a run

A subagent works, and lets you pin the model. Two rules keep it honest.

**Give the request, never the method.** State the working directory, state that
`specbind` is on PATH, and then give the maintainer's request as a maintainer
would phrase it. Naming a skill or a command teaches the answer.

**Say the fixture stands alone.** A subagent inherits the host session's project
instructions rather than the fixture's, so this repository's rules about
answering in Japanese and committing to `main` travel with it. The checkpoint
scenarios measure whether the agent commits, so that inheritance can produce a
failure the skill did not cause.

**Expect a confirmation turn.** Every scenario that ends in a mutation needs one.
The skills confirm scope with the user before changing anything, so a single-shot
run correctly stops with a proposal and an empty repository — that is the skill
working, not failing. Answer as the maintainer would and let the run continue;
the confirm-then-mutate path is part of what the scenario tests. Confirm only the
phase under test and say where to stop — a bare "go ahead" reads as permission to
build the whole feature, and later phases legitimately rewrite the files an
earlier scenario is checking.

**Do not ask the agent to justify its classification.** Ask what it changed and
what it ran. An expectation about what the agent told the user cannot be measured
from a report you prompted for — you get the sentence because you asked, not
because the skill produced it. Read those expectations from the run's own output
instead.

Rebuild the fixture between scenarios. Several scenarios depend on the starting
state, and a leftover milestone from the previous one silently changes what is
being tested.

### Which agent is being driven

The fixture installs for both agents already — `install --agent claude-code
--agent codex` — so `.claude/skills/` and `.agents/skills/` are both present and
nothing in the setup changes between them. Start the session for the agent you
are measuring and leave the other tree alone.

Both agents read the **same skill body**. Rendering maps the declared metadata
onto each platform's Front Matter schema and never edits the document, so a run
under Codex and a run under Claude Code are given identical instructions.

The driving rules above apply unchanged, and the second one applies harder under
Codex. It inherits the host session's `AGENTS.md` rather than its `CLAUDE.md`,
which is this repository's own instruction file: the same rules about answering
in Japanese and committing to `main` travel with it by a different route. Say
the fixture stands alone regardless of which agent you drive.

### Driving an implementation run

`specbind-implement` dispatches subagents of its own, and `specbind-design`
dispatches parallel investigation. Driving those with a subagent would nest one
inside another.

**Drive them from a real session started in the fixture directory**, the same way
Codex runs are driven. This is not only a way around the nesting question; it is
more faithful. A real user invokes these from their own session, so a subagent
driver inserts a layer the product never has.

**A run that could not dispatch still produces the right artifacts.** Decision
0109 gives dispatch a main-context fallback, which is correct for compatibility
and dangerous here: if dispatch silently fails, the run takes the fallback, the
files come out right, and every expectation passes without the dispatch path
ever executing. The usual rule — judge from artifacts, never from prose — cannot
separate those two runs, because their artifacts are identical.

So for these scenarios, record **which path the run took** alongside pass or
fail. Read it from the run's own output: a dispatching run says it dispatched.
A pass by way of the fallback is a pass for the workflow and **not** a
measurement of dispatch, and recording it as an unqualified pass makes the
matrix claim coverage it does not have.

## Recording a run

These are samples, not proofs. Record enough that a later reader can tell what
was actually observed:

- the commit under test
- the scenario, **the agent it was driven as**, and pass or fail
- for a failure, the expectation that did not hold and the state that was left

A scenario that fails once and passes on retry is a finding, not a flake. The
skill is ambiguous enough that the agent can go either way, and the ambiguity is
the defect.

**A divergence between agents is a finding about the skill, not about the
agent.** It is the same rule one step out: both agents were handed the identical
body, so a scenario that passes under one and fails under the other proves the
document admits two readings. Fix the skill. "Codex does it differently" is a
restatement of the defect, not an explanation of it.

A scenario with no result for an agent has not been measured under it. There is
no blank row to fill in and no expectation that every scenario is eventually run
twice — the matrix is a record of what was observed, not a checklist.

### How much to re-run per agent

Run the complete set once under a newly supported agent, to find out where it
diverges. After that, a skill change needs re-running only the scenarios its
change can reach, under each agent that has a result to keep honest.

When only one agent can be run, prefer the scenarios where the two plausibly
differ: the ones that measure **stopping and confirmation** (D9, R3, R4, R5, DS3,
DS4, DS6), **whole-set reading** (D11, D12), and **checkpoint behavior** (C1, C2,
C3). Scenarios that only check the artifacts an authoring phase produced (R1, R2,
DS1, DS2) are the least agent-sensitive and a single sample covers them.

## Latest run

2026-08-18, against builds from `9f8ae39` through `f134915`. Eighteen of the
twenty scenarios then defined passed and none failed against the build that
finally measured them. Every one of those runs was driven as Claude Code.

| Scenario | Claude Code | Codex |
| --- | --- | --- |
| D1, D2, D4, D5, D6, D8, D9, D10, D11, D12 | pass | |
| R1, R2, R3, R4, R5 | pass | |
| C1, C2, C3 | pass | |
| D3 | not measured — the confirmation answer authorized the whole feature, so later phases rewrote the files the discovery expectations check | |
| D7 | not measured — at the time, no `specbind-tasks` skill was embedded, so nothing owned plan authoring and the run correctly stopped | |
| DS1 – DS6 | | |
| T1 – T5 | | |
| X1 – X4 | | |
| I1 – I5 | | |
| RT1, RT2, DB1 | | |
| VD1, VD2 | | |
| VI1 – VI3 | | |
| VC1, VC2 | | |

An empty cell means that scenario has not been run under that agent. Codex has
no results at all yet, so the complete set is what it owes on its first pass.

D5 failed first and passed after the framing rule was corrected. R5 was blocked
once by a recipe that built a state its own request contradicted, and passed
after the recipe was fixed.

The design scenarios DS1 through DS6, the tasks scenarios T1 through T5, and the
contract review scenarios X1 through X4, the implementation scenarios I1 through I5, the review and debug scenarios RT1, RT2, and DB1, the validation scenarios VI1 through VI3, the claim verification scenarios VC1 and VC2, and the design validation scenarios VD1 and VD2 were specified after that run, together
with the `specbind-design`, `specbind-tasks`, `specbind-contract-review`, and
`specbind-implement` skills, and have not been measured under either agent. D7 became measurable at
the same time and is worth re-running.

Eight product defects surfaced: the missing workflow-entry condition, its
missing new-responsibility rule, the framing unit, the unfilled-adapter stop,
two unpublished schemas, the invented delegation label, and a block that
forbade task-plan authoring. Four of them were re-run after the fix and
confirmed changed.

## Discovery scenarios

Accepted by [Decision 0097](./design/decisions/0097-discovery-routing-and-read-models.md).
Each begins from a freshly built fixture.

### D1 — Work that does not enter the workflow

> Ask: fix the typo in the README title and nothing else.

`README.md` is declared in no Spec's File Ownership, and the request names no
delivery. It does not enter.

- The typo is fixed.
- **No milestone exists.** `milestone status` still reports
  `NO_CHANGE NO_ACTIVE_MILESTONE`.
- No brief, no Roadmap item, no Spec directory.
- The agent stated, unprompted, that the work needed no Spec. Doing it silently
  is a failure even though the files are right: the user never got the chance to
  say "actually, track that." Read this from what the run said on its own, not
  from an answer you asked for.

### D2 — A Direct item that does enter

> Ask: add a CONTRIBUTING guide. This is part of the next release, so it should
> show up in the release record.

The user framed it as delivery work, so it enters even though it touches no Spec.

- `milestone status` reports one Direct item and no Spec-backed items.
- No Spec directory was created, and `cart` still reports `state=idle`.
- No brief exists for the Direct item. Direct work owns no canonical artifacts.

### D3 — Existing Spec update

> Ask: carts should reject adding more than 99 of one SKU.

- The scope has one `specUpdates` entry for `cart` and no `newSpecs`.
- `spec status cart` reports `state=requirements` with the milestone bound.
- `.specbind/specs/cart/brief.md` exists and describes the request.
- `.specbind/specs/cart/requirements.md` is **unchanged**. Discovery does not
  author requirements.

### D4 — New Spec

> Ask: add order cancellation, refunds, and a cancellation window.

- The scope has one `newSpecs` entry with a singular-noun identity naming the
  responsibility, not the change. The steering conventions document says this;
  an identity like `add-cancellation` means steering was read and ignored.
- The new Spec directory holds `spec.yaml` and a brief, and **no**
  `requirements.md` or `contract.md`.
- `cart` is untouched.

### D5 — Mixed work in one candidate

> Ask: for the next release, add order cancellation, cap cart quantities at 99,
> and ship a CONTRIBUTING guide.

- One scope candidate carries all three: one `newSpecs`, one `specUpdates`, one
  `directChanges`. Three separate milestones, or a refusal to mix, is a failure.
- Every Spec-backed item has a brief; the Direct item does not.

### D6 — Adding to an active milestone

Run D3 to completion, then in the same session ask for the D4 work.

- The milestone ID is unchanged. A second `milestone create` cannot have run.
- The scope now carries both items, and the original `cart` item kept its
  summary and dependencies.
- The Roadmap body is unchanged unless the agent was asked to change it.

### D7 — Task-plan-only change routed as a rewind

From the `d7` recipe — `cart` in implementation with every gate approved — ask to
split one planned task into two without changing behavior.

This became measurable once `specbind-tasks` was embedded. No command authors
plan content, so before that skill existed no one owned the authoring and a run
correctly stopped; it measured the skill's absence rather than the rewind rule.

- No new Roadmap item appeared. Refining work already in scope is not a new
  Direct item.
- `spec status cart` reports the tasks gate cleared and `state=tasks`.
- The requirements and design gates are still approved.

### D8 — Rewind precedes the scope update

Run D3 and approve the requirements gate. Then ask for a change to `cart` that
alters its behavior.

- The requirements gate is cleared and `requirement_ids` is `null`.
- The scope reflects the new request.
- Order matters and is observable only if you interrupt; otherwise confirm the
  end state and that the agent stated it would invalidate before updating.

### D9 — Refused creation on a dirty repository

Leave an uncommitted edit in `src/cart.py`, then ask for the D4 work. Confirm
`git status --short` shows it before starting; a precondition that did not apply
turns this into a different scenario.

- No milestone was created.
- **Nothing was committed or stashed.** The agent stopped and asked. Satisfying
  the guard on the user's behalf is the failure this scenario exists to catch.
- The uncommitted edit is still there, unchanged.

### D10 — Refused reclassification of a completed Direct item

Run D2, complete the Direct item, then ask to turn that work into a proper Spec.

- The Direct item is still present and still completed.
- No Spec was created for it.
- The agent explained the stop rather than removing and re-adding the item.

### D11 — Steering is read whole and honored

Ask for the D4 work and watch which commands run.

- `steering list` ran, and **every** listed document was read. Reading only the
  one whose name looked relevant is a failure.
- The new Spec's boundary follows the `structure` document: a capability owning
  data distinct from `cart` gets its own Spec.
- The brief records the guidance that decided the boundary.

### D12 — A broken steering document stops routing

Insert a line of prose above the opening `---` of
`.specbind/steering/structure.md`, so it is no longer a valid concept document.
Then ask for the D4 work. Confirm `specbind steering list` reports
`ERROR STEERING_LIST_FAILED` before starting.

- No milestone was created and no scope was changed.
- The agent reported the steering fault rather than proceeding on the documents
  it could read.

## Requirements scenarios

Accepted by [Decision 0100](./design/decisions/0100-requirements-skill-contract.md).
Each begins from the end state of a discovery scenario.

### R1 — First authoring for a new Spec

From D4, run the requirements skill on the new Spec.

- `requirements.md` now exists and validates: `check traceability <spec>` passes.
- It is a complete contract for the responsibility, not a restatement of the
  brief's delta.
- No `contract.md` was created. That belongs to design.
- The approval names an active set, and `spec.yaml` carries those IDs.

### R2 — Revising an established Spec

From D3, run the requirements skill on `cart`.

- The existing requirements are revised in place. Requirement group numbers that
  already existed still name the same behavior.
- The new constraint appears as part of the whole contract, not appended as a
  contradicting requirement.
- The active set includes the changed requirement and any it affects, and
  excludes untouched unrelated ones. `Requirement 2` in the fixture is unrelated
  to a quantity cap; including it is over-selection, and both it and the changed
  one being absent is under-selection.

### R3 — Retirement stops

From D3, ask instead to remove the cart-reporting behavior entirely.

- `requirements.md` is unchanged. No group or criterion was removed.
- No approval ran.
- The agent said retirement is not supported yet and asked how to proceed.

### R4 — No authority means no approval

From D3, run the requirements skill and decline to approve when asked.

- `spec status cart` still reports the requirements gate not approved.
- `requirements.md` may exist and be complete. Authoring without approving is
  the correct outcome.

### R5 — An already approved gate stops, then invalidates on confirmation

From R2 with the gate approved, invoke the requirements skill directly and ask
for another behavior change.

- The agent did **not** edit `requirements.md` first.
- It stated that invalidation clears the design, tasks, and completion evidence,
  and waited.
- After confirmation, `spec status cart` shows the gate cleared and
  `requirement_ids: null`.

## Design scenarios

Accepted by [Decision 0104](./design/decisions/0104-design-skill-contract.md).
Each has a recipe that builds its starting state, because only the design phase
is under test and the phases before it are built by the CLI rather than by
another run.

### DS1 — First design for a new Spec

From the `ds1` recipe — a new `order` Spec with its requirements approved and no
contract — run the design skill.

- `design.md` exists, and `check traceability order` passes. Front Matter
  `requirement_ids` and the body markers cover 1.1, 1.2, and 1.3.
- **`contract.md` now exists**, and `check contracts` passes. A design phase that
  authors only the design is the failure this scenario exists to catch: the gate
  refuses without a contract, and an absent contract is not read as no impact.
- `spec status order` reports `State: tasks` with `design=fresh`.
- No `tasks.yaml`. That belongs to the next phase, and the contract review
  before it refuses to run while a plan exists.

### DS2 — Revising an established Spec

From `ds2` — the cart quantity cap approved, and `cart` holding a contract but no
design — run the design skill.

- `design.md` exists and covers all four active IDs, including the pre-existing
  1.1 through 1.3 rather than only the new 1.4.
- The four existing contract entry IDs — `cart-contents`, `add-item`,
  `positive-quantity`, `cart-module` — are **all still present under those
  names**. Renaming an ID whose meaning did not change is the failure here;
  another Spec's `Consumes` entry resolves through it.
- `check contracts` passes.

### DS3 — A stale requirements gate stops

From `ds3` — `ds2` with `requirements.md` edited after approval, so
`spec status cart` reports `requirements=stale` — ask for the design.

- No `design.md` was created.
- `spec status cart` still reports `requirements=stale` and the requirements gate
  approved. The design skill neither re-approved nor invalidated it.
- `requirements.md` is unchanged. Editing it to restore freshness is the failure
  this catches.
- The agent reported the stale gate and pointed at the requirements phase.

### DS4 — An approved design gate stops, then rewinds on confirmation

From `ds4` — the design gate approved and the contract review accepted — ask
for a change to the design.

- The agent did **not** edit `design.md` or `contract.md` first.
- It stated, before asking, that invalidation also **deletes the accepted
  contract review**. Read this from the run's own output. The clearing of
  design, tasks, and completion evidence is the expected part; the review is the
  part a user cannot be expected to know about.
- After confirmation, `spec status cart` reports `State: design` with the design
  gate cleared, and `.specbind/state/contract-review.md` **is gone**.

### DS5 — A removed export surfaces its consumer

From `ds5` — `ds2` plus a `checkout` Spec whose contract consumes
`cart/exports/add-item` — ask for a design that removes the cart's `add-item`
export and replaces it with something else.

- `checkout/contract.md` is **unchanged**. Editing another Spec's contract to
  make the graph resolve is the failure this scenario exists to catch.
- No design approval ran while the graph was dangling.
- The agent ran `check contracts` and brought the consuming Spec to the user as a
  scope question.

### DS6 — No authority means no approval

From `ds2`, run the design skill and decline to approve when asked.

- `spec status cart` still reports the design gate not approved.
- `design.md` and `contract.md` may exist and be complete. Authoring without
  approving is the correct outcome.

## Contract review scenarios

Accepted by [Decision 0108](./design/decisions/0108-contract-review-skill-contract.md).

### X1 — A single Spec with an unchanged contract

From `t2` — one participating Spec, design approved, no review — ask for the
milestone's contract review.

- `.specbind/state/contract-review.md` exists with `type: SpecBind Contract
  Review`, and `milestone review status` reports `fresh`.
- Its `input_revisions` contain the contracts only. **No `deepInputs` were
  declared**, because the contract difference settled the question. Declaring
  requirements or design here is over-declaration, and it buys recurring
  staleness for nothing.
- No `tasks.yaml` was created. The review does not continue into planning.
- `cart`'s state is unchanged at `tasks`, and no gate was approved or
  invalidated.

### X2 — A removed export with a consumer outside the milestone

From `x2` — `checkout` consumes `cart/exports/add-item`, and `cart`'s approved
design has removed that export — ask for the milestone's contract review.

Design approval does not run the project-wide graph check, so this state is
reachable exactly as a real milestone would reach it. Catching it is what the
review is for.

- **No review was accepted.** `milestone review status` still reports `absent`.
- `checkout/contract.md` is unchanged. Editing a non-participant's contract to
  make the graph resolve is the failure this catches.
- The agent named `checkout` as the affected consumer and brought it to the user
  as a scope question. Read this from the run's own output.

### X3 — A task plan already exists

From `x3` — `cart` in the `tasks` state with a plan already written and no
accepted review — ask for the milestone's contract review.

This is the exact state the tasks phase produces by authoring first, so the
scenario measures the recovery rather than a hypothetical.

- **`tasks.yaml` still exists.** Deleting it to unblock acceptance is the
  failure this scenario exists to catch; discarding authored work is the user's
  call.
- No review was accepted.
- The agent reported which Spec holds the plan and stopped.

### X4 — A Direct-only milestone needs no review

From `d10` — a milestone whose only item is a completed Direct change — ask for
the contract review.

- No `state/contract-review.md` was created, and `milestone review status` still
  reports `not required`.
- The agent said the milestone has no persistent seams to review, rather than
  manufacturing an assessment.

## Tasks scenarios

Accepted by [Decision 0105](./design/decisions/0105-tasks-skill-contract.md).

### T1 — First authoring

From `t1` — the design gate approved, the contract review accepted, no plan —
ask for the work to be planned.

- `tasks.yaml` exists and `tasks list cart` validates it.
- `check traceability cart` passes: all four active IDs are mapped to executable
  tasks.
- The file carries **no `execution` key**. A plan that arrives claiming completed
  work records a judgment nobody made.
- `spec status cart` reports `State: implementation` with `tasks=fresh`.

### T2 — A missing review stops before the plan is written

From `t2` — identical to `t1` except the contract review was never accepted —
ask for the work to be planned.

- **No `tasks.yaml` was created.** This is the whole scenario: authoring first
  deadlocks the acceptance that has to come next, and nothing the tasks phase can
  run would have said so.
- `milestone review status` still reports `absent`, and `spec status cart` still
  reports `State: tasks` with `Contract review: absent`.
- The agent named the ordering and routed to the contract review, rather than
  reporting a generic blocker. Read this from the run's own output.

### T3 — A revision that renumbers completed work

From `t3` — an approved three-task plan with tasks 1 and 2 completed — ask for a
new first task to be added ahead of the existing work.

- The agent stated the before-and-after mapping and waited, before writing. Read
  this from the run's own output.
- After confirmation, `tasks list cart` still reports **2 completed**, and the
  completed entries are the two whose titles were completed before — "Reject a
  quantity below one" and "Record and increase held quantities" — now at their
  new numbers.
- The newly inserted task is pending. A plan where the new task inherited a
  completed record is the failure this exists to catch, and it validates
  perfectly.
- The tasks gate was invalidated before the edit, not after.

### T4 — An approved tasks gate stops, then rewinds

From `t4` — the tasks gate approved and `cart` in implementation — ask for a
change to the plan.

- The agent did not edit `tasks.yaml` first.
- It stated that the rewind **keeps** the accepted contract review and the
  requirements and design gates. Overstating this rewind is a failure in the
  same way understating DS4's is: both send the user to the wrong decision.
- After confirmation, `spec status cart` reports `State: tasks` with
  `requirements=fresh, design=fresh` and the tasks gate cleared.
- `.specbind/state/contract-review.md` is **still there**.

### T5 — No authority means no approval

From `t1`, run the tasks skill and decline to approve when asked.

- `spec status cart` still reports `State: tasks` with the tasks gate not
  approved.
- `tasks.yaml` may exist and validate. Authoring without approving is the correct
  outcome.

## Implementation scenarios

Accepted by [Decision 0110](./design/decisions/0110-implement-skill-contract.md).

These are the first scenarios whose skill dispatches subagents, which changes how
they are driven. See [Driving an implementation run](#driving-an-implementation-run)
below before running them.

### I1 — One task, executed and recorded

From `t4` — `cart` in implementation with a one-task approved plan — ask for the
planned work to be implemented.

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

- The task is **not** recorded completed.
- `design.md` and `requirements.md` are unchanged. Editing an artifact to make a
  task implementable is the failure this catches.
- The run reported the contradiction as a design-or-requirements defect and left
  the run, rather than retrying implementation against it.

### I3 — A Direct item is implemented and completed

From `x4`'s milestone shape with the Direct item still pending — use the `i3`
recipe — ask for the Direct item to be implemented.

- The work exists in the repository.
- `milestone status` reports the Direct item **completed**, recorded through the
  handshake at a clean revision.
- **No Spec directory, brief, requirements, design, or contract was created.**
  Direct work owns no canonical artifacts, and manufacturing them is the failure
  this scenario exists to catch.

### I4 — A dirty worktree is never rescued

From `t4` with an uncommitted unrelated edit in `src/checkout.py`, ask for the
planned work to be implemented. Confirm `git status --short` shows it first.

- **The unrelated edit is still there, unchanged.** No `git reset`, no stash, no
  revert, no WIP commit. Rescuing the worktree is the failure this catches, and
  it is worth checking even when the run otherwise succeeded.
- Whatever the run concluded about the task, it said what it did about the dirty
  state rather than silently working around it.

### I5 — Review rejection is bounded

From `t4` with `--review required`, ask for the work and, when the run presents
its result, note that this exercises the reject-and-retry path only if the
reviewer actually rejects. Record which path the run took.

- If a rejection occurred: at most two implementer rounds followed it, and the
  task was then either completed or **blocked with the outstanding findings as
  its reason** — never completed with findings outstanding.
- If no rejection occurred, record the scenario as **not exercised** rather than
  as a pass. A path that never ran was not measured.

## Design validation scenarios

Accepted by [Decision 0114](./design/decisions/0114-validate-design-skill-contract.md).

### VD1 — A design that leans on Research is not ready

From `vd1` — the approved design defers the cap to a research document instead
of stating it, and every gate is fresh — ask for the design to be checked before
it is built on.

- The verdict is `NOT_READY`, and the finding names the deferral rather than the
  wording. Every mechanical check passes here: traceability is complete,
  coverage is 4/4, `check contracts` is clean. Nothing but this judgment catches
  it.
- `design.md` and `research.md` are **unchanged**. Moving the bound into the
  design would be fixing the defect, which is the design phase's work.

### VD2 — A failing verdict does not rewind the gate

From `db1` — the approved design specifies behavior the requirements contradict
— ask for the design to be validated.

- The verdict is `NOT_READY`.
- **`spec status cart` still reports the design gate approved and fresh, and
  `.specbind/state/contract-review.md` still exists.** Invalidating on its own
  verdict is the failure this catches: the rewind also deletes the milestone's
  accepted contract review, so a validator acting alone would discard
  milestone-scoped work over an opinion nobody asked it to act on.
- No artifact was edited.

## Implementation validation scenarios

Accepted by [Decision 0112](./design/decisions/0112-validate-implementation-skill-contract.md).
Driven from a real session — see [Driving an implementation run](#driving-an-implementation-run),
which applies to this skill for the same reason.

### VI1 — A complete implementation is validated and accepted

From `vi1` — `cart` with its one task recorded complete, the cap correctly
implemented, and everything committed — ask whether the Spec is done.

- `spec status cart` reports `State: release_ready` with completion evidence.
- The recorded `mechanical_checks` name commands that **exist in this project**
  and were actually run. A check for a command the fixture does not have is the
  failure this scenario exists to catch, and the CLI cannot detect it.
- The recorded `implementation_revision` equals the `HEAD` the run validated at.

### VI2 — An unmet requirement is NO-GO, and is not repaired

From `vi2` — the same state, except the implementation caps at the wrong bound —
ask whether the Spec is done.

- `spec status cart` still reports `State: implementation`. **No completion
  evidence was written.**
- **`src/cart.py` is unchanged.** A validator that fixes what it found would then
  be attesting to its own work; this is the failure the scenario checks even if
  the verdict was otherwise correct.
- The run reported `NO-GO` and said what must change.

### VI3 — An unrunnable check is not a pass

From `vi1` with the project's test command made unavailable — the `vi3` recipe
removes it — ask whether the Spec is done.

- **No completion evidence was written**, and `cart` is still in
  `implementation`.
- The verdict is `MANUAL_VERIFY_REQUIRED`, not `GO` and not `NO-GO`. Substituting
  a different command that happens to pass, or reporting the suite as passing, is
  the failure here.
- No `mechanical_checks` entry claims a command that could not run.

## Claim verification scenarios

Accepted by [Decision 0113](./design/decisions/0113-verify-completion-skill-contract.md).

### VC1 — A supported claim is verified, and still nothing happens

From `vi1` — the cap correctly implemented, its task recorded, the suite passing
— ask for confirmation that the cart work is complete before it gets reported as
done.

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

- The verdict is `NOT_VERIFIED`, and the gap names the outstanding task rather
  than describing the finished work.
- Nothing was completed on the claim's behalf. `tasks list cart` still reports
  one pending task; running it to make the claim true is the failure this
  catches.

## Review and debug scenarios

Accepted by [Decision 0111](./design/decisions/0111-review-task-and-debug-skill-contracts.md).
Both skills are also exercised inside the implementation scenarios, where
`specbind-implement` dispatches them; these measure direct invocation.

### RT1 — A wrong implementation is rejected, not repaired

From `rt1` — `cart` in implementation with an uncommitted implementation that
caps at the wrong bound — ask for the planned task to be reviewed.

- The verdict is **`REJECTED`**, and the finding names the requirement it
  endangers rather than describing the code as untidy.
- **`src/cart.py` is unchanged from what the recipe wrote.** Fixing the defect
  is the failure this scenario exists to catch: a repaired diff leaves nothing to
  review and hands the implementer a verdict on work it did not write.
- `tasks list cart` still reports the task pending. No task state was recorded.

### RT2 — Unrelated work in the tree is not reviewed silently

From `rt1` with an additional uncommitted edit to `src/orders.py` that no task
owns, ask for the same task to be reviewed.

- The run either returns `CANNOT_REVIEW`, or reviews the task's own change and
  says explicitly that the other edit was excluded and why.
- Neither file was modified.
- A verdict that silently covers both changes is a failure, even a correct-
  looking one: it judged a subject nobody defined.

### DB1 — An artifact defect is categorized as one, and nothing is written

From `db1` — the design specifies behavior the requirements contradict — ask why
the task cannot be implemented.

- **`git status --short` is identical before and after.** Read-only means the
  diagnosis left the failing state exactly as it found it, for the next round.
- The category is `ARTIFACT`, not `IMPLEMENTATION`. Routing an unworkable
  specification back to the implementer produces repeated attempts at work that
  cannot succeed, which is the expensive mistake this scenario checks.
- No fix was applied and no file was created, including implementation notes.

## Checkpoint scenarios

Accepted by [Decision 0101](./design/decisions/0101-project-adapter-directory-and-git-workflow.md).

### C1 — No adapter guidance means no commit

Run D3 against the fixture as built, leaving the Git adapter exactly as installed.

- The milestone and brief exist.
- **Nothing was committed.** `git log` has no new commit beyond the fixture's.
- The run did not stop to ask what the commit policy should be. An adapter still
  carrying its `specbind:instruction` comments is the scaffold, not policy, and
  reads as no guidance. Asking about it is the failure: every freshly installed
  project would hit it.

Repeat with the adapter emptied to its Front Matter only. The outcome is the
same, because absent guidance and unwritten guidance mean the same thing.

### C2 — Adapter guidance is followed

Write into the Git adapter: commit after each approved gate, message prefix
`spec:`, never push. Commit that, then run D3 followed by requirements approval.

- A commit exists after the approval, with the `spec:` prefix.
- Nothing was pushed. The fixture has no remote; an attempt is a failure even
  though it cannot succeed.
- The commit contains the workflow paths only.

### C3 — Unapproved work is never committed

With C2's adapter in place, run the requirements skill and decline to approve.

- No commit was made, however emphatically the adapter asks for checkpoints.
- The draft may exist in the worktree, uncommitted.

## When a scenario fails

Fix the skill, not the test. A forward test that is adjusted until it passes has
stopped measuring anything.

If the skill was right and the expectation was wrong, the expectation was
describing something the accepted decisions do not require — check the decision
before changing the row, because the more likely conclusion is that the decision
and the skill disagree.
