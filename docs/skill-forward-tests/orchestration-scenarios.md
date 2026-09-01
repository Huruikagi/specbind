# Orchestration and supporting forward-test scenarios

[Back to the forward-test index](../skill-forward-tests.md). These cover
configuration, Plan scope modes, checkpoints, gap analysis, steering, and
failure handling.

## Drive scenarios

Accepted by
[Decision 0168](../design/decisions/0168-milestone-drive-orchestrator.md).

### DR1 — A parked Direct reroute does not stop independent delivery

Prepare `dr1`, then ask:

> Drive this active milestone as far as you safely can. Do not release it.

- The installed `sb-drive` workflow is selected from the milestone-wide
  request; the run does not collapse to one invocation of `sb-implement`.
- The `cart-contract-change` Direct item is parked for Discovery because its
  summary explicitly requires canonical Requirements work. No cart artifact or
  implementation path is changed.
- The independent `contributing-guide` Direct item is implemented, reviewed,
  checkpointed, and recorded completed through its ordinary owner.
- The run then stops with the cart reroute in its accumulated attention report;
  it does not ask at that first item while independent work is still reachable.
- Milestone scope is byte-identical apart from the CLI-owned sparse completion
  state for `contributing-guide`, the worktree is clean, and Release does not
  run.

## Configuration scenarios

Accepted by [Decision 0154](../design/decisions/0154-guided-configuration-workflow.md).

### CF1 — A role change is regenerated and verified

Use the base fixture and ask:

> Ask: Use the cheaper Codex model for implementation work in this project.

- `.specbind.json` records only the implementer capability override and keeps
  every unrelated install choice.
- `.codex/agents/sb-implementer.toml` is regenerated from that setting;
  no generated Agent file is edited as the source of truth.
- `specbind configuration show` reports the implementer as `overridden`, and a
  final `specbind install --dry-run` has no create or replace action.
- The active Git adapter produces one local checkpoint containing only the
  configuration and regenerated product-managed files.

### CF2 — A template change leaves existing artifacts alone

Use the base fixture, record the current cart Requirements bytes and lifecycle
status, then ask:

> Ask: Future Requirements need a Risks section after Acceptance criteria.
> Leave existing artifacts alone.

- `settings/templates/specs/requirements.md` gains the requested section and
  remains readable through `specbind template read spec requirements`.
- `specbind configuration show` reports `spec/requirements: project-content`.
- The established cart Requirements bytes and all lifecycle state are unchanged.
- No reconciliation preview or write is treated as required after the request
  explicitly declines it.
- The active Git adapter produces one local checkpoint containing only the
  project-owned template.

### CF3 — Project shaping establishes guidance before adding a Design candidate

Use the base fixture and ask:

> Ask: Review this project's durable guidance and then configure the Design
> templates for future API and infrastructure changes. Keep existing artifacts
> unchanged.

- The agent reads the current Steering inventory and documents before deciding
  whether a new candidate is warranted. It does not bootstrap merely because a
  collection is empty, nor treat Steering as a substitute for the selection
  Rule.
- It compares repository facts, the current main and UI candidates, and the
  Design and Contract Rules before proposing an API or infrastructure candidate.
- Because the base fixture establishes neither an external API nor independent
  infrastructure responsibility, it extends the existing main Design with
  conditional API and operations guidance rather than adding candidates.
- It does not infer a new candidate from the future technology labels. The
  candidate set, selection Rule, existing artifacts, and lifecycle state remain
  unchanged; only the project-owned main Design template changes.

### DS9 — Design proposes a one-off supplement without changing project policy

Prepare a Spec whose approved Requirements introduce a durable infrastructure
responsibility with its own deployment and recovery guarantees, while the
project has no applicable infrastructure Design candidate. Ask to author that
Spec's Design.

- The agent reports the selected project candidates, then proposes a focused
  Spec-local `design/infrastructure` supplement with its covered Requirements,
  target path, and the alternative of extending `design/main`.
- It stops for confirmation before creating the supplement.
- After confirmation, the live supplement is a valid `SpecBind Design` and is
  included in traceability and Design approval, while the project template
  inventory and `design-template-selection` Rule remain byte-identical.

## Existing-implementation adoption scenarios

Accepted by [Decision 0143](../design/decisions/0143-existing-implementation-adoption.md)
and packaged under Discovery by
[Decision 0175](../design/decisions/0175-existing-adoption-as-discovery-references.md).

### A1 — Adoption stops at a missing Steering baseline

Prepare `a1`, then ask:

> Adopt the existing cart and order implementation into SpecBind Specs.

- The agent discovers the installed Discovery adoption route and runs
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

## Plan scenarios

Accepted by [Decision 0161](../design/decisions/0161-default-plan-and-phase-skill-namespace.md)
as superseded for phase packaging by
[Decision 0174](../design/decisions/0174-plan-phase-procedures-as-references.md).

### Q0 — A bare invocation asks for scope without starting work

Request ordinary planning without naming a Spec or explicitly requesting all
Specs.

> Ask: Plan the active work.

- The Skill may read milestone status and presents the named-Spec and all-Spec
  choices, then stops for the answer.
- No phase is dispatched, no artifact is authored, and no gate is approved.
- It does not infer all scope from the number of participating Specs.
- Selecting scope alone is not treated as delegated-gate authorization.

### Q1 — Delegation is authorized once, and recorded

Request ordinary planning for a Spec-backed item and accept the delegation it
proposes.

> Ask: Plan the cart change.

- The milestone, the item, and the three gates were named **before** any work
  started.
- Exactly one confirmation was taken. A prompt at each gate is the failure.
- `specbind spec status <spec>` afterwards reports
  `Delegated gates: requirements (sb-plan), design (sb-plan), tasks (sb-plan)`.

### Q2 — Declining delegation does not end the run

Run Plan and decline the delegation.

> Ask: take the cart change through to an approved plan in one go. *(Decline the delegation.)*

- The run continued, sequencing the phases and pausing at each gate.
- The gates that were approved record `explicit`, not delegated authority.

### Q3 — Design validation is on the delegated path

Run Plan against a Spec whose design has a defect design validation catches.

> Ask: take the cart change through to an approved plan in one go.

- `sb-validate-design` ran, between authoring and design approval.
- Its `NO-GO` **stopped the run.** Approving the design gate and reporting the
  verdict as advisory is the failure.

### Q4 — The single-Spec contract review is not skipped

Run Plan on a milestone with exactly one participating Spec.

> Ask: take the cart change through to an approved plan in one go.

- The contract review ran and was accepted before Tasks authoring began.
- The skill did not reason that one Spec needs no cross-Spec review, and did not
  discover the barrier by having `specbind spec tasks approve` refuse.

### Q5 — A deliberate stop is not retried

Run Plan against a Spec whose requirements gate is already approved, so the
Requirements phase Skill stops and asks before invalidating.

> Ask: take the cart change through to an approved plan in one go.

- The stop was reported as the answer. **No re-dispatch was attempted.**
- Delegation did not carry the invalidation. The run asked.

### B0 — Explicit all-Spec intent selects the same Plan workflow

Run against any active milestone, including one with a single participating
Spec.

> Ask: take every spec in this milestone through to approved plans.

- The exact complete Spec-backed scope and the Requirements, Design, and Tasks
  delegated gates are presented before phase work starts.
- The workflow identity presented for durable gate evidence is
  `sb-plan`; none of the removed `specbind-quick-plan` or batch workflow
  identifiers is selected or suggested.
- The run stops for the one delegation confirmation before authoring.

### B1 — Requirements are not serialized behind dependencies

Run Plan in all scope on a milestone whose roadmap has a dependency chain
across three Spec-backed items.

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

Watch the commands during an all-scope Plan run.

> Ask: take every spec in this milestone through to approved plans.

- `specbind milestone status` was re-read between rounds.
- The skill did not parse the roadmap to build its own dependency graph, and did
  not print a precomputed wave plan it then followed regardless of state.

### B4 — One unfinished item stops the barrier, and scope is not touched

Run Plan in all scope on a three-Spec milestone where one Spec's design
cannot complete.

> Ask: take every spec in this milestone through to approved plans.

- The other items finished their reachable phases.
- The contract review was **not attempted**.
- The run reported which item is unfinished and why.
- **The roadmap is byte-identical.** Dropping the unfinished item from scope to
  reach the barrier is the failure this scenario exists for.

### B5 — Direct items are reported, not absorbed

Run Plan in all scope on a milestone containing both Spec-backed and Direct items.

> Ask: take every spec in this milestone through to approved plans.

- No Direct item was given Requirements, Design, or a task plan.
- The closing report names them as remaining work, so the milestone does not read
  as finished.

### B6 — The run stops at Tasks approval

Complete any all-scope Plan run.

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
- Run `sb-plan` afterwards for that Spec's Design phase. Conclusions marked Design or Contract appear
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

- The fixture has no test files, runner, manifest, CI, or established testing
  convention. The first turn reports that absence, asks the maintainer for the
  actual convention, and leaves the fixture unchanged. Answer: "We require
  focused automated tests for each changed public function, asserting
  caller-observable results rather than implementation details. Document that
  policy and stop after Steering."
- `specbind steering list` ran **before** the identity was chosen.
- The identity is lowercase kebab-case and matches no existing selector.
- The document was written only to the project-root-relative `project_path`
  reported by `template list steering` —
  `.specbind/steering/<artifact_id>.md` in this fixture — and appears in a final
  `specbind steering list`. No repository-root `steering/` path was created.
- The `document` scaffold's `create` instruction is absent from the written
  file, while its `maintain` instruction remains unchanged.
- `specbind steering check <artifact_id> --template document` succeeds, proving
  the materialized document retained complete durable instructions and no
  scaffold placeholder or creation guidance.
- The document contains the supplied policy and supported project facts only; it
  does not invent a runner, command, CI requirement, or additional obligation.

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
