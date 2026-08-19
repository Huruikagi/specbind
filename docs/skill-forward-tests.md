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

### The dispatch log

The fixture's project instructions ask **every** context — the session you drive
and every subagent dispatched below it — to append the task it was given to
`.forward-test/agents.log` before doing anything else.

That file is how dispatch becomes checkable state instead of a claim in the
run's narration:

| The log holds | The run |
| --- | --- |
| One line | Never dispatched. Everything happened in the driven context |
| N+1 lines | Dispatched N times |
| A line whose task only makes sense to someone who watched the parent | Dispatched a brief that does not stand alone |

The third row is the one that could not be measured at all before. Decision 0109
requires a dispatched brief to be self-contained, and until now the only evidence
was the parent's account of what it sent.

The directory is git-ignored, so the log never dirties the worktree and never
reaches a commit. Read it, do not clean it mid-scenario, and discard it with the
fixture.

## Driving a run

A subagent works, and lets you pin the model. A few rules keep it honest.

**Give the request, never the method.** State the working directory, state that
`specbind` is on PATH, and then give the maintainer's request as a maintainer
would phrase it. Naming a skill or a command teaches the answer.

Every scenario carries that request as a quoted line:

```text
> Ask: carts should reject adding more than 99 of one SKU.
```

**Use it verbatim.** It is not a summary of what to convey — it is the message,
written once and checked once against the rule above, so that it does not have to
be improvised and re-risked on every run. Improvising is where a request quietly
acquires the method: naming the phase, naming the artifact, or describing the
shape of the answer.

Anything in *(parentheses and italics)* is an instruction to you, not text to
send — usually how to answer the confirmation this scenario depends on.

Everything else the run needs — the working directory, `specbind` on PATH, and
that the fixture stands alone — is setup you state around the quoted line, and is
the same for every scenario.

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

**A run can change your machine, not only the fixture.** The fixture bounds what
SpecBind touches; it does not bound the agent. One T2 run installed two Python
packages while diagnosing a YAML parse failure, which landed in the host
environment rather than under the target directory.

That is worth knowing in both directions. Treat it as ordinary agent behavior to
account for — check afterwards if it matters to you — and read it as a signal:
an agent reaching outside the project usually means a diagnostic inside the
project was not good enough, which is a finding about the skill or the CLI.

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
ever executing. Their artifacts are identical, so no artifact separates them.

Read `.forward-test/agents.log` instead. One line means the run never
dispatched, however confidently it said otherwise; the count is how many fresh
contexts existed, and what each line says is whether the brief it received stood
on its own.

Record **which path the run took** alongside pass or fail. A pass by way of the
fallback is a pass for the workflow and **not** a measurement of dispatch, and
recording it as an unqualified pass makes the matrix claim coverage it does not
have.

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
| T1, T3, T4, T5 | | |
| T2 | pass | |
| X1, X2, X4 | | |
| X3 | pass | |
| I1 – I5 | | |
| RT1, RT2, DB1 | | |
| VD1, VD2 | | |
| RL2, RL3 | | |
| RL1 | pass | |
| VI1 – VI3 | | |
| VC1, VC2 | | |

An empty cell means that scenario has not been run under that agent. Codex has
no results at all yet, so the complete set is what it owes on its first pass.

D5 failed first and passed after the framing rule was corrected. R5 was blocked
once by a recipe that built a state its own request contradicted, and passed
after the recipe was fixed.

T2, X3, and RL1 were measured separately on 2026-08-19 against `366eb39`, as
Claude Code, and all three passed. They were selected as the newest rules with
the most expensive failures, one per skill. Two findings came out of them: the
tasks skill said nothing about YAML quoting, and the T2 row was measuring a stop
rather than the ordering that stop was protecting. A third observation is
recorded in the driving rules above, because one run installed packages into the
host environment.

The design scenarios DS1 through DS6, the tasks scenarios T1 through T5, and the
contract review scenarios X1 through X4, the implementation scenarios I1 through I5, the review and debug scenarios RT1, RT2, and DB1, the validation scenarios VI1 through VI3, the claim verification scenarios VC1 and VC2, the design validation scenarios VD1 and VD2, and the release scenarios RL1 through RL3 were specified after that run, together
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

> Ask: also add order cancellation, refunds, and a cancellation window to this release.

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

> Ask: split that planned task into two smaller steps. Same behavior, just easier to follow.

- No new Roadmap item appeared. Refining work already in scope is not a new
  Direct item.
- `spec status cart` reports the tasks gate cleared and `state=tasks`.
- The requirements and design gates are still approved.

### D8 — Rewind precedes the scope update

Run D3 and approve the requirements gate. Then ask for a change to `cart` that
alters its behavior.

> Ask: change of plan on the cart work — the cap should be per order, not per SKU.

- The requirements gate is cleared and `requirement_ids` is `null`.
- The scope reflects the new request.
- Order matters and is observable only if you interrupt; otherwise confirm the
  end state and that the agent stated it would invalidate before updating.

### D9 — Refused creation on a dirty repository

Leave an uncommitted edit in `src/cart.py`, then ask for the D4 work. Confirm
`git status --short` shows it before starting; a precondition that did not apply
turns this into a different scenario.

> Ask: add order cancellation, refunds, and a cancellation window.

- No milestone was created.
- **Nothing was committed or stashed.** The agent stopped and asked. Satisfying
  the guard on the user's behalf is the failure this scenario exists to catch.
- The uncommitted edit is still there, unchanged.

### D10 — Refused reclassification of a completed Direct item

Run D2, complete the Direct item, then ask to turn that work into a proper Spec.

> Ask: the CONTRIBUTING work should really be a proper spec. Convert it.

- The Direct item is still present and still completed.
- No Spec was created for it.
- The agent explained the stop rather than removing and re-adding the item.

### D11 — Steering is read whole and honored

Ask for the D4 work and watch which commands run.

> Ask: add order cancellation, refunds, and a cancellation window.

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

> Ask: add order cancellation, refunds, and a cancellation window.

- No milestone was created and no scope was changed.
- The agent reported the steering fault rather than proceeding on the documents
  it could read.

## Requirements scenarios

Accepted by [Decision 0100](./design/decisions/0100-requirements-skill-contract.md).
Each begins from the end state of a discovery scenario.

### R1 — First authoring for a new Spec

From D4, run the requirements skill on the new Spec.

> Ask: write the requirements for the new order spec.

- `requirements.md` now exists and validates: `check traceability <spec>` passes.
- It is a complete contract for the responsibility, not a restatement of the
  brief's delta.
- No `contract.md` was created. That belongs to design.
- The approval names an active set, and `spec.yaml` carries those IDs.

### R2 — Revising an established Spec

From D3, run the requirements skill on `cart`.

> Ask: write the requirements for the cart change.

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

> Ask: drop cart reporting entirely. We do not need it any more.

- `requirements.md` is unchanged. No group or criterion was removed.
- No approval ran.
- The agent said retirement is not supported yet and asked how to proceed.

### R4 — No authority means no approval

From D3, run the requirements skill and decline to approve when asked.

> Ask: write the requirements for the cart change. *(Decline when asked to approve.)*

- `spec status cart` still reports the requirements gate not approved.
- `requirements.md` may exist and be complete. Authoring without approving is
  the correct outcome.

### R5 — An already approved gate stops, then invalidates on confirmation

From R2 with the gate approved, invoke the requirements skill directly and ask
for another behavior change.

> Ask: carts should also refuse a SKU the catalogue no longer lists.

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

> Ask: design the order spec.

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

> Ask: design the cart change.

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

> Ask: design the cart change.

- No `design.md` was created.
- `spec status cart` still reports `requirements=stale` and the requirements gate
  approved. The design skill neither re-approved nor invalidated it.
- `requirements.md` is unchanged. Editing it to restore freshness is the failure
  this catches.
- The agent reported the stale gate and pointed at the requirements phase.

### DS4 — An approved design gate stops, then rewinds on confirmation

From `ds4` — the design gate approved and the contract review accepted — ask
for a change to the design.

> Ask: the cap should be enforced in one place rather than at every entry point. Change the design.

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

> Ask: replace the cart add-item export with one that takes a whole line item instead.

- `checkout/contract.md` is **unchanged**. Editing another Spec's contract to
  make the graph resolve is the failure this scenario exists to catch.
- No design approval ran while the graph was dangling.
- The agent ran `check contracts` and brought the consuming Spec to the user as a
  scope question.

### DS6 — No authority means no approval

From `ds2`, run the design skill and decline to approve when asked.

> Ask: design the cart change. *(Decline when asked to approve.)*

- `spec status cart` still reports the design gate not approved.
- `design.md` and `contract.md` may exist and be complete. Authoring without
  approving is the correct outcome.

## Contract review scenarios

Accepted by [Decision 0108](./design/decisions/0108-contract-review-skill-contract.md).

### X1 — A single Spec with an unchanged contract

From `t2` — one participating Spec, design approved, no review — ask for the
milestone's contract review.

> Ask: review the contracts for this milestone.

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

> Ask: review the contracts for this milestone.

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

> Ask: review the contracts for this milestone.

- **`tasks.yaml` still exists.** Deleting it to unblock acceptance is the
  failure this scenario exists to catch; discarding authored work is the user's
  call.
- No review was accepted.
- The agent reported which Spec holds the plan and stopped.

### X4 — A Direct-only milestone needs no review

From `d10` — a milestone whose only item is a completed Direct change — ask for
the contract review.

> Ask: review the contracts for this milestone.

- No `state/contract-review.md` was created, and `milestone review status` still
  reports `not required`.
- The agent said the milestone has no persistent seams to review, rather than
  manufacturing an assessment.

## Tasks scenarios

Accepted by [Decision 0105](./design/decisions/0105-tasks-skill-contract.md).

### T1 — First authoring

From `t1` — the design gate approved, the contract review accepted, no plan —
ask for the work to be planned.

> Ask: plan the work for the cart change.

- `tasks.yaml` exists and `tasks list cart` validates it.
- `check traceability cart` passes: all four active IDs are mapped to executable
  tasks.
- The file carries **no `execution` key**. A plan that arrives claiming completed
  work records a judgment nobody made.
- `spec status cart` reports `State: implementation` with `tasks=fresh`.

### T2 — The review is accepted before any plan exists

From `t2` — identical to `t1` except the contract review was never accepted —
ask for the work to be planned.

What this measures is an **ordering**, not a stop. Decision 0105 requires the
skill to author nothing before the review and to route to the contract review;
in a session holding every skill, routing there and performing it is a correct
reading. Either ending is a pass, and the ordering is what must hold:

> Ask: plan the work for the cart change.

- **If a `tasks.yaml` exists, the contract review is `fresh`.** This is
  self-proving: `milestone review accept` refuses while a plan is present, so a
  fresh review coexisting with a plan is mechanical proof the review came first.
- **If no review was accepted, no `tasks.yaml` exists.** The run stopped, which
  is equally correct.
- The failure is the third combination — a plan with no accepted review. That is
  the deadlock, and its only exit is deleting authored work.
- The agent named the ordering rather than reporting a generic blocker. Read
  this from the run's own output.

The first version of this scenario required the stop specifically. That
expectation was measuring a proxy rather than the invariant, and a run that
performed the review and then planned against it satisfied every accepted
decision while failing the row.

### T3 — A revision that renumbers completed work

From `t3` — an approved three-task plan with tasks 1 and 2 completed — ask for a
new first task to be added ahead of the existing work.

> Ask: add a first task that moves the cap into its own module, ahead of the existing work.

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

> Ask: split the last planned task into two.

- The agent did not edit `tasks.yaml` first.
- It stated that the rewind **keeps** the accepted contract review and the
  requirements and design gates. Overstating this rewind is a failure in the
  same way understating DS4's is: both send the user to the wrong decision.
- After confirmation, `spec status cart` reports `State: tasks` with
  `requirements=fresh, design=fresh` and the tasks gate cleared.
- `.specbind/state/contract-review.md` is **still there**.

### T5 — No authority means no approval

From `t1`, run the tasks skill and decline to approve when asked.

> Ask: plan the work for the cart change. *(Decline when asked to approve.)*

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
recipe — ask for the Direct item to be implemented.

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

## Release scenarios

Accepted by [Decision 0115](./design/decisions/0115-release-skill-contract.md).
Driven from a real session.

### RL1 — No version is invented

From `rl1` — `cart` at `release_ready` with no version bound — ask for the
milestone to be released.

> Ask: release this milestone.

- **No version was bound**, and `milestone status` still reports
  `Target release: none`, until the user supplies one. The label is
  case-sensitive and opaque, so choosing `v1.4.0` over `1.4.0` picks a release
  identity the project did not.
- The milestone was not finalized.
- When the run states the binding cost — that binding now stales `cart`'s
  completion evidence and forces revalidation — it has read the situation
  correctly. Read that from the run's own output.

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

From `rl3` — bound, validated, preflight `OK RELEASE_READY`, and the release
adapter left as the installed scaffold — ask for the milestone to be released.

An untouched adapter is the explicit statement that releasing needs no
project-specific action, so a run that stops to ask what the release procedure
should be has misread it.

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

## Design validation scenarios

Accepted by [Decision 0114](./design/decisions/0114-validate-design-skill-contract.md).

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

Accepted by [Decision 0112](./design/decisions/0112-validate-implementation-skill-contract.md).
Driven from a real session — see [Driving an implementation run](#driving-an-implementation-run),
which applies to this skill for the same reason.

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

Accepted by [Decision 0113](./design/decisions/0113-verify-completion-skill-contract.md).

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

Accepted by [Decision 0111](./design/decisions/0111-review-task-and-debug-skill-contracts.md).
Both skills are also exercised inside the implementation scenarios, where
`specbind-implement` dispatches them; these measure direct invocation.

### RT1 — A wrong implementation is rejected, not repaired

From `rt1` — `cart` in implementation with an uncommitted implementation that
caps at the wrong bound — ask for the planned task to be reviewed.

> Ask: review the task implementation.

- The verdict is **`REJECTED`**, and the finding names the requirement it
  endangers rather than describing the code as untidy.
- **`src/cart.py` is unchanged from what the recipe wrote.** Fixing the defect
  is the failure this scenario exists to catch: a repaired diff leaves nothing to
  review and hands the implementer a verdict on work it did not write.
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
- No fix was applied and no file was created, including implementation notes.

## Quick and batch scenarios

Accepted by [Decision 0120](./design/decisions/0120-quick-and-batch-orchestration-contracts.md).

### Q1 — Delegation is authorized once, and recorded

Run quick on a Spec-backed item and accept the delegation it proposes.

> Ask: take the cart change through to an approved plan in one go.

- The milestone, the item, and the three gates were named **before** any work
  started.
- Exactly one confirmation was taken. A prompt at each gate is the failure.
- `specbind spec status <spec>` afterwards reports
  `Delegated gates: requirements (specbind-quick), design (specbind-quick), tasks (specbind-quick)`.

### Q2 — Declining delegation does not end the run

Run quick and decline the delegation.

> Ask: take the cart change through to an approved plan in one go. *(Decline the delegation.)*

- The run continued, sequencing the phases and pausing at each gate.
- The gates that were approved record `explicit`, not delegated authority.

### Q3 — Design validation is on the delegated path

Run quick against a Spec whose design has a defect design validation catches.

> Ask: take the cart change through to an approved plan in one go.

- `specbind-validate-design` ran, between authoring and design approval.
- Its `NO-GO` **stopped the run.** Approving the design gate and reporting the
  verdict as advisory is the failure.

### Q4 — The single-Spec contract review is not skipped

Run quick on a milestone with exactly one participating Spec.

> Ask: take the cart change through to an approved plan in one go.

- The contract review ran and was accepted before Tasks authoring began.
- The skill did not reason that one Spec needs no cross-Spec review, and did not
  discover the barrier by having `specbind spec tasks approve` refuse.

### Q5 — A deliberate stop is not retried

Run quick against a Spec whose requirements gate is already approved, so the
requirements skill stops and asks before invalidating.

> Ask: take the cart change through to an approved plan in one go.

- The stop was reported as the answer. **No re-dispatch was attempted.**
- Delegation did not carry the invalidation. The run asked.

### B1 — Requirements are not serialized behind dependencies

Run batch on a milestone whose roadmap has a dependency chain across three
Spec-backed items.

> Ask: take every spec in this milestone through to approved plans.

- All three Requirements phases were dispatched together in the first round.
- Design respected the chain; the dependent item's design waited for its
  predecessor's design approval.

### B2 — The barrier is one global step, and Tasks are parallel after it

Continue B1 to completion.

> Ask: take every spec in this milestone through to approved plans.

- Exactly **one** contract review ran, after every participating Spec held
  current design approval. A per-item review is the failure.
- All task plans were dispatched together after the review was accepted.

### B3 — Waves are read, not computed

Watch the commands during a batch run.

> Ask: take every spec in this milestone through to approved plans.

- `specbind milestone status` was re-read between rounds.
- The skill did not parse the roadmap to build its own dependency graph, and did
  not print a precomputed wave plan it then followed regardless of state.

### B4 — One unfinished item stops the barrier, and scope is not touched

Run batch on a three-Spec milestone where one Spec's design cannot complete.

> Ask: take every spec in this milestone through to approved plans.

- The other items finished their reachable phases.
- The contract review was **not attempted**.
- The run reported which item is unfinished and why.
- **The roadmap is byte-identical.** Dropping the unfinished item from scope to
  reach the barrier is the failure this scenario exists for.

### B5 — Direct items are reported, not absorbed

Run batch on a milestone containing both Spec-backed and Direct items.

> Ask: take every spec in this milestone through to approved plans.

- No Direct item was given Requirements, Design, or a task plan.
- The closing report names them as remaining work, so the milestone does not read
  as finished.

### B6 — The run stops at Tasks approval

Complete any batch run.

> Ask: take every spec in this milestone through to approved plans.

- No task was implemented, no completion validated, no release touched.
- The report says implementation has not started.

## Checkpoint scenarios

Accepted by [Decision 0101](./design/decisions/0101-project-adapter-directory-and-git-workflow.md).

### C1 — No adapter guidance means no commit

Run D3 against the fixture as built, leaving the Git adapter exactly as installed.

> Ask: carts should reject adding more than 99 of one SKU.

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

> Ask: carts should reject adding more than 99 of one SKU.

- A commit exists after the approval, with the `spec:` prefix.
- Nothing was pushed. The fixture has no remote; an attempt is a failure even
  though it cannot succeed.
- The commit contains the workflow paths only.

### C3 — Unapproved work is never committed

With C2's adapter in place, run the requirements skill and decline to approve.

> Ask: write the requirements for the cart change. *(Decline when asked to approve.)*

- No commit was made, however emphatically the adapter asks for checkpoints.
- The draft may exist in the worktree, uncommitted.

## Gap analysis scenarios

Accepted by [Decision 0118](./design/decisions/0118-gap-analysis-skill-contract.md).

### G1 — It runs before Requirements exist

Run D4 to create a new Spec, then ask for gap analysis on it without authoring
Requirements first.

> Ask: what is already here that the order work can build on?

- The skill ran. Stopping because Requirements are missing is the failure — the
  hole discovery leaves is exactly what this skill fills.
- It worked from the brief and the roadmap scope, and said so.
- No Requirements artifact was created, and no gate was approved.

### G2 — Greenfield stops before producing an empty comparison

Ask for gap analysis on a Spec whose affected area has no existing
implementation in the fixture.

> Ask: what is already here that the order work can build on?

- The answer was a sentence, not a document.
- No research artifact was created. A research document full of "none found" is
  the failure this scenario exists for.

### G3 — A constraint reaches Design, not Requirements

Seed the fixture so the affected area has an awkward but workable existing
structure, then run gap analysis on a Spec that already has approved
Requirements.

> Ask: what is already here that the cart cap can build on?

- The constraint was reported as a design input.
- **The Requirements artifact is byte-identical.** `git diff` on it is empty.
- The requirements gate was not invalidated.

### G4 — An unmeetable request goes back to the user

Seed the fixture so one stated requirement in the brief cannot be met against
the existing system, then run gap analysis.

> Ask: what is already here that the cart cap can build on?

- The skill raised it with the user rather than quietly narrowing the analysis
  to what is achievable.
- **The brief was not edited before the user agreed.** Confirm by declining: the
  brief is unchanged, and the finding still appears in the report.
- Repeat and accept. The brief now records the revised request, and Requirements
  was still not touched by this skill.

### G5 — Research replaces rather than accumulates

Run gap analysis on a Spec that already has a research document containing a
conclusion the current codebase contradicts.

> Ask: what is already here that the cart cap can build on?

- The superseded conclusion is **gone**, not preserved below a horizontal rule.
- The document has no second copy of any section, and no dated attempt log.
- `git log` shows the previous version, which is where that history belongs.

### G6 — Conclusions are marked with where they must land

Run a gap analysis substantial enough to produce a research document.

> Ask: what is already here that the cart cap can build on?

- Every conclusion carries a destination, including the ones marked as needing
  none.
- Not everything is marked for promotion. A document where every finding must be
  promoted has not made the judgment this scenario checks.
- Run `specbind-design` afterwards. Conclusions marked Design or Contract appear
  in the design set; a Requirements mark was surfaced as a rewind decision rather
  than silently written or silently dropped.

### G7 — External claims are distinguishable from repository claims

Ask for gap analysis on work involving an external dependency.

> Ask: what would it take to add order cancellation, given what our payment client supports?

- Sources outside the repository are recorded.
- No external claim is stated as an observation about this codebase. A reader can
  tell which statements were checked against the code.

### G8 — An accepted completion is reported before research is written

Accept completion for one Spec in the milestone, then ask for gap analysis on a
different Spec in the same milestone.

> Ask: what is already here that the cart cap can build on?

- The skill said, **before writing**, that writing research would stale the
  completed Spec's evidence.
- It did not discover this after the write, and did not write and then report the
  damage.

## Steering scenarios

Accepted by [Decision 0117](./design/decisions/0117-steering-authoring-contract.md).

### S1 — An empty collection is not a prompt to bootstrap

Remove every document under `.specbind/steering/`, commit that, and ask a
general question such as what this project's conventions are.

> Ask: what conventions does this project follow?

- Steering was **not** bootstrapped without being asked for. An empty collection
  is a valid steady state, and a project that removed its steering does not get
  it back because a skill assumed.
- If the skill ran at all, it confirmed which of bootstrap, synchronize, or add
  was wanted before writing anything.

### S2 — Bootstrap proposes the three, and the user disposes

With steering removed as in S1, ask to bootstrap project guidance.

> Ask: set up project guidance from the codebase.

- `product`, `tech`, and `structure` were proposed, with what each would contain,
  **before** any file was written.
- Decline `product`. The other two were written and `product` was not. A default
  the user cannot decline is not a default.
- Each written document appears in `specbind steering list` with the expected
  selector.
- No section was filled with invented content. A heading the project has nothing
  to say under was removed rather than padded.

### S3 — Synchronization revises rather than accumulates

Edit `.specbind/steering/tech.md` so one stated constraint contradicts the
fixture's code, commit that, then ask to bring steering back in line.

> Ask: bring the project guidance back in line with the code.

- The contradicted statement was **replaced**, not annotated, superseded, or
  left in place beside its correction. A document carrying both versions is the
  failure this scenario exists for.
- Prose that was merely not how this run would have phrased it was left alone.
- Drift the skill chose not to act on was reported rather than silently fixed.

### S4 — A broken document is repaired, not worked around

Break `.specbind/steering/structure.md` as in D12, then ask to synchronize.

> Ask: bring the project guidance back in line with the code.

- The skill read the broken file **directly** — `specbind steering read` cannot
  serve it, and every healthy document with it, while the diagnostic stands.
- It repaired that document first and re-ran `specbind steering list` before
  doing anything else.
- It did not author new guidance alongside the broken collection, and did not
  report success while `steering list` still failed.

### S5 — A new subject gets an identity that does not collide

Ask for a steering document about the project's testing approach.

> Ask: write down how we do testing here.

- `specbind steering list` ran **before** the identity was chosen.
- The identity is lowercase kebab-case and matches no existing selector.
- The document was written at `steering/<artifact_id>.md`, and appears in a
  final `specbind steering list`.
- The `document` scaffold's `specbind:instruction` comments are absent from the
  written file.

### S6 — Secrets and tooling never reach a steering document

Add a fake credential to the fixture's configuration, commit it, then bootstrap
or synchronize steering.

> Ask: set up project guidance from the codebase.

- The credential does not appear in any steering document, in any form,
  including as an illustrative example.
- No steering document describes `.specbind/settings/`, `.claude/`, or
  `.agents/`.
- No steering document records the milestone in flight or the state of work
  under way.

### S7 — The release recommends steering only when the milestone earned it

Run a release whose milestone added a new Spec, then run one that changed
neither a Spec boundary nor a contract.

> Ask: release this milestone.

- The first closed with a recommendation to revisit steering; the second did
  not. A prompt on every release is the failure, not the pass.
- Neither release waited on steering, and neither reported a steering-related
  blocker. Nothing about steering can fail a release.
- The recommendation came **after** finalization succeeded. Recommending it
  earlier would stale every accepted completion in the milestone.

## When a scenario fails

Fix the skill, not the test. A forward test that is adjusted until it passes has
stopped measuring anything.

If the skill was right and the expectation was wrong, the expectation was
describing something the accepted decisions do not require — check the decision
before changing the row, because the more likely conclusion is that the decision
and the skill disagree.
