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

## Driving a run with a subagent

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

## Recording a run

These are samples, not proofs. Record enough that a later reader can tell what
was actually observed:

- the commit under test
- the scenario, and pass or fail
- for a failure, the expectation that did not hold and the state that was left

A scenario that fails once and passes on retry is a finding, not a flake. The
skill is ambiguous enough that the agent can go either way, and the ambiguity is
the defect.

## Latest run

2026-08-18, against builds from `9f8ae39` through `f134915`. Eighteen of the
twenty scenarios passed and none failed against the build that finally measured
them.

| Scenario | Result |
| --- | --- |
| D1, D2, D4, D5, D6, D8, D9, D10, D11, D12 | pass |
| R1, R2, R3, R4, R5 | pass |
| C1, C2, C3 | pass |
| D3 | not measured — the confirmation answer authorized the whole feature, so later phases rewrote the files the discovery expectations check |
| D7 | not measured — no `specbind-tasks` skill is embedded, so nothing owns plan authoring and the run correctly stops |

D5 failed first and passed after the framing rule was corrected. R5 was blocked
once by a recipe that built a state its own request contradicted, and passed
after the recipe was fixed.

The design scenarios DS1 through DS6 were specified after that run, together
with the `specbind-design` skill, and have not been measured yet.

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

Run D3, then drive `cart` through requirements, design, and tasks approval —
`spec status cart` should report `state=implementation`. Then ask to split one
planned task into two without changing behavior.

This scenario cannot be measured until `specbind-tasks` is embedded. No command
authors plan content, so with no skill owning that authoring an agent has no
sanctioned way to revise the plan and correctly stops. A run against a build
without that skill measures its absence, not the rewind rule.

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
- No `tasks.yaml`. That belongs to the next phase, and the cross-spec review
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

From `ds4` — the design gate approved and the cross-spec review accepted — ask
for a change to the design.

- The agent did **not** edit `design.md` or `contract.md` first.
- It stated, before asking, that invalidation also **deletes the accepted
  cross-spec review**. Read this from the run's own output. The clearing of
  design, tasks, and completion evidence is the expected part; the review is the
  part a user cannot be expected to know about.
- After confirmation, `spec status cart` reports `State: design` with the design
  gate cleared, and `.specbind/state/cross-spec-review.md` **is gone**.

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
