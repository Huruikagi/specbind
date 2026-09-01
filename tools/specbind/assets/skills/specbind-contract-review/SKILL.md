---
name: specbind-contract-review
description: Judge whether the milestone's changes leave every persistent seam in the project coherent, and accept the one contract review that Tasks authoring waits on.
---

# Review the contract graph

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

One review per milestone, between design approval and any task plan. The
question is **not** "do the Specs in this milestone agree with each other." It is:

> Does this milestone leave **every** persistent seam in the project coherent —
> including seams owned by Specs it never touched?

A Spec outside the milestone is the consumer most likely to break, precisely
because nobody is looking at it.

You accept one artifact. You author no Spec artifact and change no Spec's state.

## 1. Check that the review can run

```sh
specbind milestone review status
specbind milestone status
specbind milestone scope
```

Stop and report, rather than fixing, in these cases:

- **`Status: not_applicable`** — a Direct-only milestone has no persistent seams
  to review. Say that the review is not required and stop.
- **A participant is not ready** — acceptance needs every Spec-backed item to
  hold a fresh design gate and sit in the `tasks` state. Route the unready Spec
  to its phase. Never approve a gate to make this barrier passable.
- **A task plan already exists** — acceptance refuses with
  `CONTRACT_REVIEW_TASKS_ALREADY_EXIST`. Report which Spec holds it and stop.
  **Do not delete it.** The ordering is already lost, and discarding authored
  work is the user's decision, not a step in a review.

`milestone scope` gives you the complete current scope. For every Spec-backed
participant it names, also run:

```sh
specbind spec status <spec>
```

`milestone status` gives you the two revisions you need:

```text
  Revision: <current HEAD>
  Baseline: <what this milestone is measured against>
```

## 2. Establish what changed

```sh
specbind schema read contract/v1
specbind protocol read contract-review
specbind check contracts
```

Read **every** current Contract in the project, not only the participants':

```sh
specbind spec list
specbind artifact read <spec> contract --for consume
```

Then read the same fixed Contract paths independently at the baseline. This is
ordinary Git. Resolve `specDir` from `.specbind.json`; each persistent Spec's
Contract is exactly `<specDir>/specs/<spec>/contract.yaml`.

```sh
git show <baseline>:<specDir>/specs/<spec>/contract.yaml
```

If that path does not exist at the baseline, establish whether the Spec itself
was new or its required Contract was missing. Do not search for a renamed
Markdown concept or infer identity from Front Matter; `contract.yaml` is the
versioned structured singleton.

**The difference is the entry point.** A run that never established what changed
has not performed the review, however carefully it read the current graph.

The Roadmap scope is the second half of that comparison. Match each participant's
scoped behavior to the current Contract even when its Contract diff is empty. A
new owned boundary, exported behavior, consumed seam, invariant, or file-ownership
claim that appears in scope but nowhere in the Contract is a finding, not proof
that the seam stayed unchanged.

Read the project's seam policy through its project-owned rule surface:

```text
specbind rule read contract-principles --for consume
```

It covers shared ownership, compatibility posture, generated boundaries, and
dependency direction. `NO_CHANGE RULE_ABSENT` means no customization; the
protocol still applies. Any `ERROR` line stops the review.

Do not read steering here. Whether a design followed project guidance belongs to
the design phase and to `specbind-validate-design`. Your question is whether the
graph is coherent against itself and its consumers.

## 3. Judge

The protocol owns the judgment. For each changed, added, or removed entry,
establish who depends on it and what the change does to them.

For an unchanged Contract, establish that the scoped behavior introduces no
missing persistent seam or guarantee. If scope suggests one, go deeper into the
relevant Requirements or Design and declare that artifact in `deepInputs`. Do
not accept merely because there is no Contract diff; leave the omission for the
owning Design phase to resolve.

Two things the CLI cannot do for you:

**Ownership overlaps and dependency cycles are warnings**, because they are
sometimes deliberate. Say why the overlap is acceptable, or treat it as a
finding. Reporting the warning back is not a judgment.

**External consumers.** A published interface, another repository, a stored data
shape, an operational contract — none of these are Specs, and nothing will
detect the impact. Name the affected consumer, state the impact, and bring it to
the user when the change requires a decision they own.

Do not ask the user to repeat a decision already explicit in the delivery
request. If the requested behavior itself changes an exported seam, repository
evidence finds no managed consumer, and no external consumer is identified by
the project, record the possible unmanaged impact and the user's requested
disposition in the assessment. Likewise, a request that directly uses or changes
an existing unconsumed export is evidence to keep that seam for this milestone.
Ask only when the impact introduces a choice the request did not settle.

Silence here is the one finding with no downstream check that might catch it.

### Go deeper only when the conclusion depends on it

When the Contract difference and the current graph settle the question, that is
the complete review. Reading Requirements or Design is for when they do not.

Discover the Spec's artifact selectors before a deep read; never shorten
`design/<artifact-id>` to a guessed `design` selector:

```sh
specbind artifact list <spec>
specbind artifact read <spec> requirements --for consume
specbind artifact read <spec> design/<artifact-id> --for consume
```

Run the listing before constructing any Design selector and use only the exact
selectors it reports. Lifecycle states and action labels such as `tasks` and
`implementation` are not artifact IDs; never turn them into `design/tasks` or
`design/implementation`.

Read only the Requirements or listed Designs the conclusion needs.

For `deepInputs`, prefix the exact logical selector reported by `artifact list`
with `specs/<canonical-spec>#`. Thus `requirements` becomes
`specs/<spec>#requirements`, and `design/main` becomes
`specs/<spec>#design/main`. Do not use the reported filesystem `path`, shorten a
Design selector, or derive a selector from lifecycle state.

Declare in `deepInputs` only what the judgment actually relied on. Every declared
input is fingerprinted into the accepted artifact, so it becomes a freshness
input: editing it later makes the review stale and blocks tasks approval,
implementation validation, and release preflight. A file declared because you
opened it buys recurring invalidation for nothing.

Task plans are never inputs, and the CLI rejects them.

## 4. Remediate, at most twice

If the review does not pass, you may remediate and rerun **at most two rounds**.
After that, the affected Specs stay in design, no artifact is written, and you
report what is unresolved.

**You change nothing by yourself.** Where a Spec needs owned work:

- present the affected Spec and what is wrong with the seam;
- get confirmation when the milestone's scope changes materially;
- before every gate invalidation, present the exact Requirements, Design,
  Tasks, completion, and accepted Contract Review state that the rewind removes,
  then obtain explicit user confirmation even when milestone scope is unchanged;
- invoke the confirmed explicit operation — `specbind milestone update-scope`,
  or a gate invalidation — rather than editing an artifact or a contract.

A Spec added to scope must be brought through design **before** acceptance. It
cannot be left as follow-up behind a passing review: the accepted artifact has no
field a caveat could live in.

Never edit another Spec's contract to make the graph resolve.

The Design phase owns both the Design set and `contract.yaml`; status has no
separate Contract gate. If Requirements remain valid and the finding requires a
Design or Contract change, present the complete rewind cost, obtain explicit
confirmation, run `specbind spec design invalidate <spec>`, and hand the work to
`specbind-plan` in explicit Design-phase mode. If Requirements must change, rewind the Requirements gate
instead. Never leave the maintainer to infer the owning phase from the gate list.

A response that stops on this finding is incomplete unless it names the Design
phase as owner, enumerates the current state that invalidation removes, and gives
the exact `specbind spec design invalidate <spec>` operation before asking for
confirmation. Include those facts in the reported outcome even when no file or
state changed during the review.

## 5. Write the assessment and accept

The assessment is the durable explanation. Write it so a reader who did not
participate can tell what was examined and why the conclusion holds: what changed
in the graph relative to the baseline, who depends on each change, and why the
seams are coherent.

**A short assessment is often the correct one.** One participating Spec whose
Contract is unchanged is a complete review only after the scoped behavior has
been checked for a missing persistent seam or guarantee. When none is missing,
the review answered the same question and the answer was brief. Padding it
produces a record whose length implies scrutiny that did not happen.

Present the assessment and any findings before you accept.

```sh
specbind milestone review accept --candidate -
```

The candidate is strict JSON on stdin:

```json
{
  "schemaVersion": 1,
  "assessment": "...Markdown...",
  "deepInputs": ["specs/checkout#design/main"]
}
```

`deepInputs` may be empty. Selectors are canonical requirements or design
artifacts only. The CLI resolves every path, computes every fingerprint, and
owns the timestamp — never supply your own.

### On authority

Acceptance takes no approval mode, and that is deliberate rather than an
oversight. The accepted artifact records `type`, `milestone_id`, `passed_at`, and
`input_revisions` — there is no field for a user's approval, so this is your
judgment being recorded, not their authorization.

That is not licence to accept quietly. Present your reasoning first, and stop for
the user whenever a finding needs a decision they own — external consumer impact,
or a scope change.

**There is no partial, conditional, or provisional acceptance.** An unresolved
finding means the review has not passed. Accepting anyway records a judgment that
was not made, and every later boundary will trust it.

## 6. Checkpoint

Only after acceptance succeeds is this work eligible to commit.

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT` means there is no adapter-directed commit. Stop
there — that is an answer, not a missing file to work around.

An adapter carrying the exact `<!-- specbind:adapter-scaffold -->` marker is an
inactive scaffold, not project policy. Treat it as no guidance, say so in one
line, and commit nothing. The marker classifies the whole document: ignore every
other body line even when it looks actionable.

When the adapter has guidance, follow it. The request to perform this mutating
phase authorizes the adapter's narrow local checkpoint as its ordinary final
step. It does not authorize anything broader:

- An explicit user or root instruction that forbids commits wins, and tool
  permissions still apply.
- Commit guidance is not push guidance. Push only where the adapter says to, and
  never force-push, rewrite history, or bypass a protected branch.
- Stage only the paths this run produced. Unrelated work already in the worktree
  is left exactly as it is.
- Stop before the Git operation if the guidance is ambiguous, unsafe, or
  conflicts with something else you were told.

A failed checkpoint does not undo the acceptance. Report the work as uncommitted
and continue.

## Boundaries

- Author no Spec artifact. Requirements, design, and contracts belong to their
  phases; `tasks.yaml` belongs to the phase after this one.
- Write no machine state. Never edit `spec.yaml` or the roadmap directly.
- Never delete a task plan, and never approve or invalidate a gate to make this
  barrier passable. Invalidation happens only as the explicitly user-confirmed
  outcome of a finding after its complete rewind cost was presented.
- Report in the project's language: what changed in the graph, who is affected,
  what you concluded, anything you brought to the user, whether the review was
  accepted, and what runs next.
