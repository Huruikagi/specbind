---
name: specbind-design
description: Investigate the system, author a Spec's complete current technical design, maintain the Contract that other Specs depend on, and approve the design gate.
argument-hint: "<spec>"
---

# Design the change

Produce the document an implementer builds from and a reviewer judges against:
this Spec's **complete current technical design**, plus the **Contract** that
states what other Specs may rely on.

You author two artifacts and approve one gate. The requirements belong to the
phase before you, the task plan to the phase after.

## 1. Read

Always:

```sh
specbind spec status <spec>
specbind steering list
```

### Check the prerequisite first

`spec status` must report the requirements gate approved and fresh, and the Spec
a current participant of the active milestone.

A Spec that has reached the design state and has no design artifact yet reports
`Health: consistent`, `Next action: design`, and one aggregate `Expected work`
line naming the uncovered active Requirement count. That is the work you are
here to do. `check traceability` remains strict and lists each missing coverage
entry until the Design is complete.

If the requirements gate is not approved and fresh, stop and say so. Route the
user to `specbind-requirements`.
Never approve or invalidate the requirements gate yourself, and never edit
`requirements.md` to make your own work possible — editing an approved artifact
invalidates its gate as a side effect, and the user gets a freshness diagnostic
instead of the decision they would have made.

### Then read the inputs

```sh
specbind artifact read <spec> requirements
specbind artifact read <spec> brief
```

The requirements are the obligation you must realize. The brief is why this
milestone is changing it.

Read **every** steering document the listing named:

```sh
specbind steering read <selector>
```

All of them, not a promising-looking subset. This is where the project's
technical guidance finally lands: the requirements phase is required to keep
technology, structure, and mechanism out of its document, so a constraint on how
this project builds things cannot have reached you through it. If you also read
selectively, that guidance reaches no authoritative artifact at all.

If `steering list` or `steering read` prints an `ERROR` line, stop. `Found 0
steering document(s).` is a complete answer and you continue.

### Read what the Spec already has

```sh
specbind artifact list <spec>
```

Read every design artifact and the contract when they exist. Read the research
when one exists — and treat it as background only. It is deleted at release and
is not fingerprinted, so a design whose meaning depends on it becomes incomplete
the moment the milestone closes. Any conclusion you need, restate here.

**Research marks where each of its conclusions has to land.** Those marks are
addressed to you, and the milestone deletes the document that carries them:

- **Design** or **Contract** — promote it here. That is what the mark is for.
- **Requirements** — you cannot write it. Surface it: a change to an obligation
  is a requirements rewind, with the cost that carries.
- **Steering** — surface it for `specbind-steering`. Do not write steering here.
- **Brief** — already routed back to the user by the analysis; nothing to do.

A mark you neither promote nor surface expires silently when the milestone
closes. If you judge one unnecessary, say so — that is a decision, and it should
be visible as one.

When the change touches a seam, read the contracts of the Specs on the other
side of it:

```sh
specbind spec list
specbind artifact read <other-spec> contract
```

## 2. Investigate before you decide

```sh
specbind protocol read design-discovery
```

The protocol owns what must be established and when to go deeper. Read the code
you are changing rather than inferring it from names; confirm an external API
against its current source rather than from memory. A design written from
assumption produces a contract describing a seam the code does not have, and
tasks that cannot be executed as written.

A question that blocks choosing the approach is resolved or escalated now. It
cannot be deferred into the design as an open item.

### Dispatch the independent parts

When the investigation is large enough that reading it all here would crowd out
the design, dispatch the independent areas as **fresh subagents** and synthesize
what comes back. Typical splits — adjust to the change rather than filling a
template:

Use the registered `specbind-researcher` role when the host provides it;
otherwise use ordinary fresh subagents. The role changes capability only; each
brief still owns its exact evidence boundary.
Fallback is only for an absent role. A configured role whose model cannot start
is a configuration or environment failure, not permission to change models.

- how the affected area works today, and what already exists to extend
- external dependencies: current API, version compatibility, constraints
- the seam: which contract entries this touches, and who consumes them

Give each one a brief that stands alone — what to establish, which paths and
identifiers to read, and what a useful answer contains — plus the protocol to
read:

```sh
specbind protocol read design-discovery
```

Require a **findings summary, not raw material**. A subagent that returns file
dumps has moved the crowding rather than removed it.

**Synthesize here, never in a subagent.** Choosing the approach needs the whole
picture, and the whole picture is what only this context has. If a returned
summary is unusable, ask once for the summary alone rather than reading around
it.

For a change that follows an established pattern in an area you already
understand, skip dispatch entirely and check the pattern directly.

## 3. Write the design

```sh
specbind protocol read design-authoring
specbind protocol read okf-authoring
```

The project's `settings/rules/design-principles.md` and
`settings/rules/contract-principles.md` state its own preferences. They are
project-owned; if one is absent, the project removed it deliberately and the
protocol still applies.

**No Design exists — whether the Spec is new or established** — list the
template set first:

```sh
specbind template list spec
```

The listing is the complete initial Design decomposition and also tells you
whether each scaffold is `project` or `embedded`. For every listed
`design/<artifact_id>` selector, resolve and read it:

```sh
specbind template resolve spec <spec> <design-selector>
specbind template read spec <design-selector>
```

Write the authored document only to the reported `Target path`. Do not infer a
filename from `artifact_id`, the Requirements path, or another Spec. Remove
every `specbind:instruction` comment and add the live-only traceability fields
while authoring.

**A Design set exists** — revise the current design artifacts in place.

The design set is the Spec's **complete current design**, persistent the way the
requirements are. Fold this milestone's change into the document that owns that
concern. Do not append a milestone-shaped supplement: a reader arriving after
release must understand the system from the design set alone, with no knowledge
of which milestone contributed which paragraph.

Coverage is a separate axis. Every **active** requirement ID must be covered by
the union of the set; requirements outside the active set may stay mapped by the
existing design and are not re-argued.

Write the traceability exactly as the profile requires — a Front Matter
`requirement_ids` array, and an italic `_Requirements: 1.1, 1.2_` marker beside
each section that satisfies them. The Front Matter set and the union of the body
markers must match exactly.

### Splitting

One `main` document is the default. Split only when the design holds
responsibility seams a reader would follow independently, and give each
`artifact_id` a name for a durable concern rather than a slice of this
milestone's work.

Identity churn is expensive here. Adding or removing a design identity
invalidates approval by itself, so reorganizing an established Spec's design set
needs a stated reason — it is not housekeeping to do in passing.

If the design turns out to hold seams that could move separately, raise
splitting the Spec or revisiting roadmap scope with the user. Do not create or
rescope Specs yourself.

## 4. Maintain the contract

Every Spec reaching design approval has exactly one contract, and this phase is
what puts it there. A missing contract refuses approval; it is never read as an
absence of cross-spec impact.

**No contract yet** — including every Spec this milestone created:

```sh
specbind template resolve spec <spec> contract
specbind template read spec contract
```

Write the authored Contract only to the reported `Target path`. The `Source`
field explains whether its scaffold is project-owned or embedded; both use the
same raw read command.

A Spec with no cross-spec seams gets the canonical empty contract: five
headings, no entries. That is a complete and deliberate statement, not a
placeholder.

**Contract exists** — revise it in place when this change adds, alters, or
removes a seam, and leave it **byte-identical** when it does not. Rewording an
untouched entry is not free: the whole file is fingerprinted, so a cosmetic edit
invalidates approval and forces a new contract review.

The `design-authoring` protocol carries the test for what belongs in it. Entry
IDs are stable — do not rename an ID whose meaning is unchanged, because another
Spec's `Consumes` entry resolves through it.

Removing an entry is allowed. This is not requirement retirement: a requirement
ID is an identity that design, tasks, and completion verification each have to
cover, while a contract entry's only structural dependents are other Specs'
`Consumes` entries, which resolve by name and are checked. Remove it, then let
the check and the contract review judge it.

## 5. Check before you present

```sh
specbind check traceability <spec>
specbind check contracts
```

Approval enforces both anyway. Running them first turns a refused approval into
a diagnostic you can act on.

Resolve what `check contracts` reports. A reference left dangling by a removal is
either fixed in this Spec, or the consuming Spec needs owned work — which is a
scope question for the user, not something you fix. **Never edit another Spec's
contract to make your own graph clean.**

Ownership overlaps and dependency cycles are warnings, because they are
sometimes deliberate. Say why the overlap is acceptable, or treat it as a
finding. Passing it silently to contract review is not a judgment.

`CONTRACT_GRAPH_EXPORT_UNCONSUMED` is also a warning, not evidence that an
export should be removed. The managed graph cannot see external consumers:

- An existing export that this change does not alter stays byte-identical. Keep
  its stable ID and carry the warning into the contract-review report; do not
  retire an unrelated seam merely to silence the check.
- For an export this change adds or alters, name the managed or external
  consumer that needs it. If there is no consumer, state the deliberate reason
  the project would pay for that boundary in advance. An answer the design
  cannot establish is a finding for the user, not a plausible consumer to
  invent.

The later contract review owns the final cross-Spec judgment. This phase makes
its own change defensible and preserves untouched current-state contracts.

## 6. Review your own design

```sh
specbind protocol read design-validation
```

This is the same standard an independent validation would apply. A design that
would fail it is not ready to submit for approval, so apply it to your own draft
before presenting.

Report every finding with its protocol disposition:

```text
- [BLOCKING|DEFERRED|RESOLVED] <finding> — <obligation, destination, or resolution>
```

Do not leave a real observation unclassified. `RESOLVED` remains visible in the
review summary; it is evidence of what changed, not an open gate condition.

`specbind-validate-design` is a separate skill the user invokes when they want a
second opinion. It is not a step you run, and not a precondition of this gate.

Present the design, what it decided and why, and what changed in the contract.
Revise on feedback rather than approving something you know to be weak.

Stop and ask the user when the same objection survives one revision. A repeated
objection is a disagreement about intent, and rewriting again produces another
variation of the same misunderstanding.

If the objection reveals that the **requirements** are ambiguous, contradictory,
or underspecified, say so and stop. Returning to requirements is the answer;
inventing design detail that hides the gap is not. You do not perform that
rewind yourself.

An observation that is real but does not hold this gate is a **deferred**
finding under the protocol, and it needs the destination this project names:

```sh
specbind adapter list
specbind adapter read deferred
```

The listing must report `state=active` for `deferred` before you follow it.
`state=absent` or `NO_CHANGE ADAPTER_ABSENT` means there is no destination: say
so in one line and record nothing. Otherwise follow the adapter as written.
Nothing recorded there is a source of work for you; an entry re-enters this
workflow only when a person puts it on the Roadmap.

## 7. Approve

Approve only when the validation protocol's judgment is satisfied **and** you
hold authority for this gate. Authority is one of two things, never their
absence:

- **Explicit** — the user approved this design after seeing it.
- **Delegated** — a run context the user intentionally started authorized this
  gate by name. The user does not confirm this document, because delegation is
  exactly the decision to skip that pause. Every check still runs.

  **The workflow name comes from that context. Never invent one.** If you were
  told the content is pre-approved but given no workflow name, you do not have a
  delegation — present your result and stop.

Never run a mutating command to find out what it accepts. Approval has no dry
run, so a probe with a placeholder value records a real approval.

```sh
specbind spec design approve <spec> --approval-mode explicit
```

A delegated run names the workflow that carries the authority:

```sh
specbind spec design approve <spec> --approval-mode delegated --delegation-workflow <workflow>
```

There are no IDs, paths, or fingerprints to pass. The CLI derives the complete
input set from the current contract and design files.

No prompt appearing, a non-interactive invocation, and a scripted run grant no
authority. Without either form, present your result and stop.

Never approve to resolve a failing check. A refused approval is information
about the artifacts; report the diagnostic rather than working around it.

## 8. Checkpoint

Only after the approval succeeds is this work eligible to commit. A draft you
have not yet approved is never committed, however often the project wants
checkpoints. If you stopped short of approving, you also stop short of this.

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT` means there is no adapter-directed commit. Stop
there — that is an answer, not a missing file to work around.

A legacy adapter may still carry `specbind:instruction` comments. That copy is
an inactive scaffold, not policy the project wrote. Treat it as no guidance, say
so in one line, and commit nothing. Do not stop to ask about a file nobody has
filled in.

When the adapter has guidance, follow it. The request to perform this mutating
phase authorizes the adapter's narrow local checkpoint as its ordinary final
step. It does not authorize anything broader:

- An explicit user or root instruction that forbids commits wins, and tool
  permissions still apply.
- Delegated approval authorizes the gate, while the orchestrated phase request
  authorizes only this local checkpoint. Neither authorizes pushing.
- Commit guidance is not push guidance. Push only where the adapter says to, and
  never force-push, rewrite history, or bypass a protected branch.
- Stage only the paths this run produced. Unrelated work already in the worktree
  is left exactly as it is.
- Stop before the Git operation if the guidance is ambiguous, unsafe, or
  conflicts with something else you were told.

A failed checkpoint does not undo or weaken the approval. The gate stays
approved; report the work as uncommitted and continue.

## When the gate is already approved

If `spec status` shows the design gate approved, do not edit. Editing underneath
an approved gate leaves evidence describing a revision that no longer exists, and
the CLI then refuses later gates citing freshness rather than the edit that
caused it.

State the full cost and run it only after the user confirms:

- it clears the design, tasks, and completion evidence for this Spec, **and**
- it **deletes the accepted contract review** for the whole milestone, because
  that review is accepted after design approval and cannot survive a rewind past
  it.

The second one is the part a user will not know to expect. Say it before you
ask.

```sh
specbind spec design invalidate <spec>
```

Confirmation cannot be inferred, and delegated authority does not cover this.
Delegation authorizes accepting gates, not discarding accepted work.

## Boundaries

- Author the design set and **this** Spec's contract. Requirements belong to the
  previous phase, `tasks.yaml` to the next, and brownfield comparison and
  research to `specbind-gap-analysis`.
- Write no machine state. Never edit `spec.yaml`.
- Do not accept the contract review, add roadmap items, or create Specs.
  Surface the need and let the owning operation perform it.
- Do not author research, and do not park an unresolved design gap there.
- Report in the project's language: what the design decides, how it realizes each
  active requirement, what changed in the contract, whether the work was
  committed, and what runs next.
