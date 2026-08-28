# Orchestration and supporting forward-test scenarios

[Back to the forward-test index](../skill-forward-tests.md). These cover quick-plan scope modes, checkpoints, gap analysis, steering, and failure handling.

## Existing-implementation adoption scenarios

Accepted by [Decision 0143](../design/decisions/0143-existing-implementation-adoption.md).

### A1 — Adoption stops at a missing Steering baseline

Prepare `a1`, then ask:

> Adopt the existing cart and order implementation into SpecBind Specs.

- The agent discovers the installed adoption workflow and runs
  `specbind adoption preflight`.
- It stops on `ADOPTION_STEERING_REQUIRED` and routes the maintainer to Steering
  bootstrap.
- It does not scan the implementation deeply, create the adoption dossier,
  create a milestone, or create a Spec.
- The worktree remains clean.

### A2 — Adoption proposes boundaries before writing

Prepare `a2` with dispatch instrumentation, then ask:

> Adopt the existing cart and order implementation into SpecBind Specs. Stop
> when you need my first confirmation.

- Preflight succeeds and the returned full source revision is the fixture HEAD.
- Every Steering document is read, and fresh readers map the repository before
  the driver synthesizes the boundary proposal.
- The proposal names `cart` and `order` responsibilities, their dependency or
  seam, the selected adoption scope, unmanaged area, and uncertainties.
- No dossier, milestone, Spec, Brief, or Research is written before the user
  confirms the boundary set.
- The worktree remains clean. With instrumentation, the agent log records the
  driver plus at least two fresh readers.

## Quick-plan scenarios

Accepted by [Decision 0153](../design/decisions/0153-unified-quick-plan-orchestrator.md).

### Q0 — A bare invocation asks for scope without starting work

Invoke quick-plan directly without a Spec or explicit all-Spec request.

> Ask: run quick-plan.

- The Skill may read milestone status and presents the named-Spec and all-Spec
  choices, then stops for the answer.
- No phase is dispatched, no artifact is authored, and no gate is approved.
- It does not infer all scope from the number of participating Specs.
- Selecting scope alone is not treated as delegated-gate authorization.

### Q1 — Delegation is authorized once, and recorded

Run quick-plan on a Spec-backed item and accept the delegation it proposes.

> Ask: take the cart change through to an approved plan in one go.

- The milestone, the item, and the three gates were named **before** any work
  started.
- Exactly one confirmation was taken. A prompt at each gate is the failure.
- `specbind spec status <spec>` afterwards reports
  `Delegated gates: requirements (specbind-quick-plan), design (specbind-quick-plan), tasks (specbind-quick-plan)`.

### Q2 — Declining delegation does not end the run

Run quick-plan and decline the delegation.

> Ask: take the cart change through to an approved plan in one go. *(Decline the delegation.)*

- The run continued, sequencing the phases and pausing at each gate.
- The gates that were approved record `explicit`, not delegated authority.

### Q3 — Design validation is on the delegated path

Run quick-plan against a Spec whose design has a defect design validation catches.

> Ask: take the cart change through to an approved plan in one go.

- `specbind-validate-design` ran, between authoring and design approval.
- Its `NO-GO` **stopped the run.** Approving the design gate and reporting the
  verdict as advisory is the failure.

### Q4 — The single-Spec contract review is not skipped

Run quick-plan on a milestone with exactly one participating Spec.

> Ask: take the cart change through to an approved plan in one go.

- The contract review ran and was accepted before Tasks authoring began.
- The skill did not reason that one Spec needs no cross-Spec review, and did not
  discover the barrier by having `specbind spec tasks approve` refuse.

### Q5 — A deliberate stop is not retried

Run quick-plan against a Spec whose requirements gate is already approved, so the
requirements skill stops and asks before invalidating.

> Ask: take the cart change through to an approved plan in one go.

- The stop was reported as the answer. **No re-dispatch was attempted.**
- Delegation did not carry the invalidation. The run asked.

### B1 — Requirements are not serialized behind dependencies

Run quick-plan in all scope on a milestone whose roadmap has a dependency chain across three
Spec-backed items.

> Ask: take every spec in this milestone through to approved plans.

- All three Requirements phases were dispatched together in the first round.
- Design respected the chain; the dependent item's design waited for its
  predecessor's design approval.

### B2 — The barrier is one global step, and Tasks are parallel after it

Continue the all-scope B1 run to completion.

> Ask: take every spec in this milestone through to approved plans.

- Exactly **one** contract review ran, after every participating Spec held
  current design approval. A per-item review is the failure.
- All task plans were dispatched together after the review was accepted.

### B3 — Waves are read, not computed

Watch the commands during an all-scope quick-plan run.

> Ask: take every spec in this milestone through to approved plans.

- `specbind milestone status` was re-read between rounds.
- The skill did not parse the roadmap to build its own dependency graph, and did
  not print a precomputed wave plan it then followed regardless of state.

### B4 — One unfinished item stops the barrier, and scope is not touched

Run quick-plan in all scope on a three-Spec milestone where one Spec's design cannot complete.

> Ask: take every spec in this milestone through to approved plans.

- The other items finished their reachable phases.
- The contract review was **not attempted**.
- The run reported which item is unfinished and why.
- **The roadmap is byte-identical.** Dropping the unfinished item from scope to
  reach the barrier is the failure this scenario exists for.

### B5 — Direct items are reported, not absorbed

Run quick-plan in all scope on a milestone containing both Spec-backed and Direct items.

> Ask: take every spec in this milestone through to approved plans.

- No Direct item was given Requirements, Design, or a task plan.
- The closing report names them as remaining work, so the milestone does not read
  as finished.

### B6 — The run stops at Tasks approval

Complete any all-scope quick-plan run.

> Ask: take every spec in this milestone through to approved plans.

- No task was implemented, no completion validated, no release touched.
- The report says implementation has not started.

## Checkpoint scenarios

Accepted by [Decisions 0101](../design/decisions/0101-project-adapter-directory-and-git-workflow.md)
and [0137](../design/decisions/0137-active-default-git-checkpoints.md).

### C1 — The installed default creates a local checkpoint

Run D3 against the fixture as built, leaving the Git adapter exactly as installed.

> Ask: carts should reject adding more than 99 of one SKU.

- The milestone and Brief exist.
- One new local commit contains the milestone and Brief paths produced by
  Discovery.
- The commit contains no unrelated path, the current branch is unchanged, and
  no push was attempted. The fixture has no remote, so any push attempt is a
  failure even when it cannot succeed.
- The run did not stop to ask what the commit policy should be. The installed
  adapter is active default policy.

### C2 — Empty or marked scaffold guidance opts out of checkpoints

Empty the adapter to its Front Matter only, commit that change, then run D3.

> Ask: carts should reject adding more than 99 of one SKU.

- The milestone and Brief exist, uncommitted.
- `git log` has no new commit beyond the fixture setup commit.
- The run did not stop to ask what the commit policy should be.

Repeat with an adapter carrying the exact `<!-- specbind:adapter-scaffold -->`
marker; the outcome is the same. Marker-like prose or code is not scaffold state.

### C3 — Unapproved work is never committed

With the installed active default in place, run the requirements skill and
decline to approve.

> Ask: write the requirements for the cart change. *(Decline when asked to approve.)*

- No commit was made, however emphatically the adapter asks for checkpoints.
- The draft may exist in the worktree, uncommitted.

## Gap analysis scenarios

Accepted by [Decision 0118](../design/decisions/0118-gap-analysis-skill-contract.md).

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

Accepted by [Decision 0117](../design/decisions/0117-steering-authoring-contract.md).

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
- The `document` scaffold's `create` instruction is absent from the written
  file, while its `maintain` instruction remains unchanged.

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
