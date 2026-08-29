# Planning-phase forward-test scenarios

[Back to the forward-test index](../skill-forward-tests.md). These are the Discovery, Requirements, Design, Contract review, and Tasks scenario contracts.

## Discovery scenarios

Accepted by [Decision 0097](../design/decisions/0097-discovery-routing-and-read-models.md).
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
  `requirements.md` or `contract.yaml`.
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

### D13 — A project-owned Roadmap body template is materialized

From a fresh fixture, replace `settings/templates/roadmap.md` with a valid
project-owned template whose body has the distinctive heading `## Delivery
promise`, one `create` instruction, and one `maintain` instruction. Commit that
settings change so the repository is clean, then ask for the D3 work.

> Ask: carts should reject adding more than 99 of one SKU.

- The scope has one `specUpdates` entry for `cart`, and the Roadmap body contains
  `## Delivery promise` filled with the milestone-wide request.
- The live Roadmap contains the template's `maintain` instruction and does not
  contain its `create` instruction.
- The Roadmap's live Front Matter contains the CLI-generated milestone fields
  and work-item index. Template Front Matter was not copied into the body.
- `.specbind/settings/templates/roadmap.md` is byte-identical to its committed
  precondition. Discovery reads the template; it does not rewrite settings.

## Requirements scenarios

Accepted by [Decision 0100](../design/decisions/0100-requirements-skill-contract.md).
Each begins from the end state of a discovery scenario.

### R1 — First authoring for a new Spec

From D4, run the requirements skill on the new Spec.

> Ask: write the requirements for the new order spec.

- The fixture defines the cancellation boundary: the customer may cancel an
  order they placed before its cancellation window closes, and a later attempt
  is rejected. The author does not have to invent what "eligible" means.
- `requirements.md` now exists and is valid. Before approval, strict
  `check traceability <spec>` passes while coverage is inactive; after approval,
  it is expected to report missing Design coverage until Design is authored,
  while `spec status <spec>` remains phase-relative and consistent.
- It is a complete contract for the responsibility, not a restatement of the
  brief's delta.
- No `contract.yaml` was created. That belongs to design.
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

### R6 — Project-defined variables are resolved once for every reference

From the `r6` recipe — R1 with a project-owned Requirements template that binds
the Unicode variable `作成日` and references it twice — run the requirements
skill on `order`.

> Ask: write the requirements for the new order spec.

- `requirements.md` is valid and contains the binding's exact value
  `fixture-day` twice.
- Neither `{{作成日}}` nor any `create` instruction remains in the live
  artifact. The template itself remains unchanged.
- The normal R1 semantic contract and approval expectations still hold. A run
  that merely copies the template, substitutes only one reference, or invents a
  different value fails this scenario.

## Design scenarios

Accepted by [Decision 0104](../design/decisions/0104-design-skill-contract.md).
Each has a recipe that builds its starting state, because only the design phase
is under test and the phases before it are built by the CLI rather than by
another run.

### DS1 — First design for a new Spec

From the `ds1` recipe — a new `order` Spec with its requirements approved, no
contract, and its project-owned Design template relocated to
`technical-design/main.md` — run the design skill. Before the run, `spec status`
reports `Health: consistent`, `Next action: design`, and expected coverage for
three active Requirements.

> Ask: design the order spec.

- `technical-design/main.md` exists, `design.md` does not, and
  `check traceability order` passes. Front Matter
  `requirement_ids` and the body markers cover 1.1, 1.2, and 1.3.
- **`contract.yaml` now exists**, and `check contracts` passes. A design phase that
  authors only the design is the failure this scenario exists to catch: the gate
  refuses without a contract, and an absent contract is not read as no impact.
- `spec status order` reports `State: tasks` with `design=fresh`.
- No `tasks.yaml`. That belongs to the next phase, and the contract review
  before it refuses to run while a plan exists.

### DS2 — Revising an established Spec

From `ds2` — the cart quantity cap approved, `cart` holding a contract but no
Design, and its project-owned Design template relocated to
`technical-design/main.md` — run the design skill.

> Ask: design the cart change.

- `technical-design/main.md` exists, `design.md` does not, and the resolved
  document covers all four active IDs, including the pre-existing 1.1 through
  1.3 rather than only the new 1.4.
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

- The agent did **not** edit `design.md` or `contract.yaml` first.
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

- `checkout/contract.yaml` is **unchanged**. Editing another Spec's contract to
  make the graph resolve is the failure this scenario exists to catch.
- No design approval ran while the graph was dangling.
- The agent ran `check contracts` and brought the consuming Spec to the user as a
  scope question.

### DS6 — No authority means no approval

From `ds2`, run the design skill and decline to approve when asked.

> Ask: design the cart change. *(Decline when asked to approve.)*

- `spec status cart` still reports the design gate not approved.
- `design.md` and `contract.yaml` may exist and be complete. Authoring without
  approving is the correct outcome.

### DS7 — A user-visible screen selects the conditional UI Design

From `ds7` — a new `dashboard` Spec with one approved Requirement explicitly
covering a responsive account overview screen, loading, empty, error, and
keyboard-navigation behavior, plus an established dependency-free Python HTML
renderer boundary, caller-supplied account snapshot, and `unittest` convention
— run the design skill.

> Ask: design the dashboard spec.

- Before authoring, the agent reports `design/main` as required and selected,
  and `design/ui` as conditional and selected because of the user-visible screen
  responsibility.
- Both `design.md` and `ui.md` exist. Their union covers active Requirement 1.1,
  and `check traceability dashboard` passes.
- `ui.md` determines the screen inventory, navigation or interaction, visible
  states, responsive behavior, accessibility, boundaries, and UI verification;
  it is not an empty scaffold or a pixel-perfect mockup request.
- `contract.yaml` exists and the Design gate may be approved only after the
  complete selected set is ready.

### DS8 — Library-only work omits the conditional UI Design

From `ds8` — a new `parser` Spec with one approved Requirement explicitly
limited to a library API and stating that no screen, interaction, or
user-visible UI behavior changes — run the design skill.

> Ask: design the parser spec.

- Before authoring, the agent reports `design/main` as required and selected,
  and `design/ui` as conditional and omitted because the change is library-only.
- `design.md` exists and `ui.md` does not.
- `check traceability parser` passes, `contract.yaml` exists, and no empty UI
  document was created as a precaution.
- The agent does not ask whether a UI exists merely because the standard UI
  candidate is installed; the Requirements already resolve the condition.

## Contract review scenarios

Accepted by [Decision 0108](../design/decisions/0108-contract-review-skill-contract.md).

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
- `checkout/contract.yaml` is unchanged. Editing a non-participant's contract to
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

Accepted by [Decision 0105](../design/decisions/0105-tasks-skill-contract.md).

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
