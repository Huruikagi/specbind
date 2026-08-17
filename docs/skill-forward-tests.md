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

Then start an agent session **with no prior context** in that directory. Context
carried from developing the skill is the most common way a forward test passes
for the wrong reason: the agent already knows what you meant.

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

## Discovery scenarios

Accepted by [Decision 0097](./design/decisions/0097-discovery-routing-and-read-models.md).
Each begins from a freshly built fixture.

### D1 — New Direct item

> Ask: fix the typo in the README title and nothing else.

- `milestone status` reports one Direct item and no Spec-backed items.
- No Spec directory was created, and `cart` still reports `state=idle`.
- No brief exists for the Direct item. Direct work owns no canonical artifacts.

### D2 — Existing Spec update

> Ask: carts should reject adding more than 99 of one SKU.

- The scope has one `specUpdates` entry for `cart` and no `newSpecs`.
- `spec status cart` reports `state=requirements` with the milestone bound.
- `.specbind/specs/cart/brief.md` exists and describes the request.
- `.specbind/specs/cart/requirements.md` is **unchanged**. Discovery does not
  author requirements.

### D3 — New Spec

> Ask: add order cancellation, refunds, and a cancellation window.

- The scope has one `newSpecs` entry with a singular-noun identity naming the
  responsibility, not the change. The steering conventions document says this;
  an identity like `add-cancellation` means steering was read and ignored.
- The new Spec directory holds `spec.yaml` and a brief, and **no**
  `requirements.md` or `contract.md`.
- `cart` is untouched.

### D4 — Mixed work in one candidate

> Ask: add order cancellation, cap cart quantities at 99, and update the README.

- One scope candidate carries all three: one `newSpecs`, one `specUpdates`, one
  `directChanges`. Three separate milestones, or a refusal to mix, is a failure.
- Every Spec-backed item has a brief; the Direct item does not.

### D5 — Adding to an active milestone

Run D2 to completion, then in the same session ask for the D3 work.

- The milestone ID is unchanged. A second `milestone create` cannot have run.
- The scope now carries both items, and the original `cart` item kept its
  summary and dependencies.
- The Roadmap body is unchanged unless the agent was asked to change it.

### D6 — Task-plan-only change routed as a rewind

Run D2, then drive `cart` through requirements, design, and tasks approval —
`spec status cart` should report `state=implementation`. Then ask to split one
planned task into two without changing behavior.

- No new Roadmap item appeared. Refining work already in scope is not a new
  Direct item.
- `spec status cart` reports the tasks gate cleared and `state=tasks`.
- The requirements and design gates are still approved.

### D7 — Rewind precedes the scope update

Run D2 and approve the requirements gate. Then ask for a change to `cart` that
alters its behavior.

- The requirements gate is cleared and `requirement_ids` is `null`.
- The scope reflects the new request.
- Order matters and is observable only if you interrupt; otherwise confirm the
  end state and that the agent stated it would invalidate before updating.

### D8 — Refused creation on a dirty repository

Leave an uncommitted edit in `src/cart.py`, then ask for the D3 work.

- No milestone was created.
- **Nothing was committed or stashed.** The agent stopped and asked. Satisfying
  the guard on the user's behalf is the failure this scenario exists to catch.
- The uncommitted edit is still there, unchanged.

### D9 — Refused reclassification of a completed Direct item

Run D1, complete the Direct item, then ask to turn that work into a proper Spec.

- The Direct item is still present and still completed.
- No Spec was created for it.
- The agent explained the stop rather than removing and re-adding the item.

### D10 — Steering is read whole and honored

Ask for the D3 work and watch which commands run.

- `steering list` ran, and **every** listed document was read. Reading only the
  one whose name looked relevant is a failure.
- The new Spec's boundary follows the `structure` document: a capability owning
  data distinct from `cart` gets its own Spec.
- The brief records the guidance that decided the boundary.

### D11 — A broken steering document stops routing

Append `not front matter` above the `---` in
`.specbind/steering/structure.md`, then ask for the D3 work.

- No milestone was created and no scope was changed.
- The agent reported the steering fault rather than proceeding on the documents
  it could read.

## Requirements scenarios

Accepted by [Decision 0100](./design/decisions/0100-requirements-skill-contract.md).
Each begins from the end state of a discovery scenario.

### R1 — First authoring for a new Spec

From D3, run the requirements skill on the new Spec.

- `requirements.md` now exists and validates: `check traceability <spec>` passes.
- It is a complete contract for the responsibility, not a restatement of the
  brief's delta.
- No `contract.md` was created. That belongs to design.
- The approval names an active set, and `spec.yaml` carries those IDs.

### R2 — Revising an established Spec

From D2, run the requirements skill on `cart`.

- The existing requirements are revised in place. Requirement group numbers that
  already existed still name the same behavior.
- The new constraint appears as part of the whole contract, not appended as a
  contradicting requirement.
- The active set includes the changed requirement and any it affects, and
  excludes untouched unrelated ones. `Requirement 2` in the fixture is unrelated
  to a quantity cap; including it is over-selection, and both it and the changed
  one being absent is under-selection.

### R3 — Retirement stops

From D2, ask instead to remove the cart-reporting behavior entirely.

- `requirements.md` is unchanged. No group or criterion was removed.
- No approval ran.
- The agent said retirement is not supported yet and asked how to proceed.

### R4 — No authority means no approval

From D2, run the requirements skill and decline to approve when asked.

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

## Checkpoint scenarios

Accepted by [Decision 0101](./design/decisions/0101-project-adapter-directory-and-git-workflow.md).

### C1 — No adapter guidance means no commit

Empty `.specbind/settings/adapters/git.md` to its Front Matter only, commit that,
then run D2.

- The milestone and brief exist.
- **Nothing was committed.** `git log` has no new commit beyond the fixture's.

### C2 — Adapter guidance is followed

Write into the Git adapter: commit after each approved gate, message prefix
`spec:`, never push. Commit that, then run D2 followed by requirements approval.

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
