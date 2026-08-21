---
name: specbind-contract-review
description: Judge whether the milestone's changes leave every persistent seam in the project coherent, and accept the one contract review that Tasks authoring waits on.
---

# Review the contract graph

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

- **`not required`** — a Direct-only milestone has no persistent seams to
  review. Say so and stop.
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
specbind protocol read contract-review
specbind check contracts
```

Read **every** current Contract in the project, not only the participants':

```sh
specbind spec list
specbind artifact read <spec> contract
```

Then discover and read the same Contracts independently at the baseline. This
is ordinary Git, but the path is not an identity: `specDir` is configured in
`.specbind.json`, and a Contract is the lowercase Markdown artifact whose Front
Matter `type` is `SpecBind Contract`.

```sh
git ls-tree -r --name-only <baseline> -- <specDir>/specs/<spec>
git show <baseline>:<candidate-path>
```

Inspect the historical candidates and require exactly one Contract by `type`,
or establish that the Spec or Contract did not yet exist. Never substitute the
current artifact path for this discovery: a moved or renamed Contract keeps its
logical identity, and `.specbind/specs/<spec>/contract.md` is only the default
location in a default installation.

**The difference is the entry point.** A run that never established what changed
has not performed the review, however carefully it read the current graph.

The project's `settings/rules/contract-principles.md` states this project's seam
policy — shared ownership, compatibility posture, generated boundaries,
dependency direction. It is project-owned; if absent, the project removed it
deliberately and the protocol still applies.

Do not read steering here. Whether a design followed project guidance belongs to
the design phase and to `specbind-validate-design`. Your question is whether the
graph is coherent against itself and its consumers.

## 3. Judge

The protocol owns the judgment. For each changed, added, or removed entry,
establish who depends on it and what the change does to them.

Two things the CLI cannot do for you:

**Ownership overlaps and dependency cycles are warnings**, because they are
sometimes deliberate. Say why the overlap is acceptable, or treat it as a
finding. Reporting the warning back is not a judgment.

**External consumers.** A published interface, another repository, a stored data
shape, an operational contract — none of these are Specs, and nothing will
detect the impact. Name the affected consumer, state the impact, and bring it to
the user when the change requires a decision they own.

Silence here is the one finding with no downstream check that might catch it.

### Go deeper only when the conclusion depends on it

When the Contract difference and the current graph settle the question, that is
the complete review. Reading Requirements or Design is for when they do not.

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
- invoke the explicit operation — `specbind milestone update-scope`, or a gate
  invalidation — rather than editing an artifact or a contract.

A Spec added to scope must be brought through design **before** acceptance. It
cannot be left as follow-up behind a passing review: the accepted artifact has no
field a caveat could live in.

Never edit another Spec's contract to make the graph resolve.

## 5. Write the assessment and accept

The assessment is the durable explanation. Write it so a reader who did not
participate can tell what was examined and why the conclusion holds: what changed
in the graph relative to the baseline, who depends on each change, and why the
seams are coherent.

**A short assessment is often the correct one.** One participating Spec whose
contract is unchanged is a complete review — it answered the same question and
the answer was brief. Padding it produces a record whose length implies scrutiny
that did not happen.

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
  barrier passable. Invalidation happens only as the confirmed outcome of a
  finding.
- Report in the project's language: what changed in the graph, who is affected,
  what you concluded, anything you brought to the user, whether the review was
  accepted, and what runs next.
