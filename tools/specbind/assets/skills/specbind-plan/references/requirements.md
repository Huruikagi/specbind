# Requirements phase

## Apply project language style

Before authoring any artifact or user-facing prose, read this once unless this
receiver already read it for the current `specbind-plan` run:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

This is the complete Requirements procedure selected by `specbind-plan`. Read it
only for a Requirements phase, whether the parent is running the complete
planning workflow or the maintainer explicitly requested this phase alone.

Produce the document every later phase is verified against: this Spec's
**complete current behavioral contract**, not the delta this milestone requested.

You author one artifact and approve one gate. Everything else — the contract,
the design, the task plan — belongs to other phases.

## 1. Read

Always:

```sh
specbind spec status <spec>
specbind steering list
```

Read the Spec's brief, and read **every** steering document the listing named:

```sh
specbind artifact read <spec> brief --for consume
specbind steering read <selector> --for consume
```

If the Brief declares Source Items, read the shared contract once and then read
every exact project-relative item it names:

```sh
specbind protocol read source-material
```

Do not reopen the whole collection or infer neighboring files; Discovery already
recorded this Spec's relevant subset. A missing, unreadable, unsupported, or
project-external item makes the request context partial, so stop before
authoring. Source material is context rather than approved scope, and a locator
alone never becomes a Requirement.

Read all the steering, not a promising-looking subset. The listing gives you a
selector, a type, and a path, and nothing in that says whether a document
constrains this Spec's behavior. A project constraint missed here is absent from
the contract every later phase checks against, so it is missed everywhere.

Treat that listing as the complete, closed set for this read. Do not enumerate
the storage directory or invent another selector from a nearby filename. In
particular, the active Roadmap is milestone state stored beside Steering; it is
not a Steering document and must not be passed to `steering read`.

If `steering list` or `steering read` prints an `ERROR` line, stop. Authoring a
contract against a knowingly partial view of the project's constraints produces
something nobody can trust. `Found 0 steering document(s).` is a complete answer
and you continue.

### Then branch on what exists

A Spec created by this milestone holds only its brief: `milestone create` writes
machine state, and discovery does not author requirements. `spec status`
reporting a missing requirements artifact is the expected starting state here,
not a fault.

- **New Spec** — start from the template and author the Spec's first complete
  contract from the brief:

  ```sh
  specbind template resolve spec <spec> requirements
  specbind template read spec requirements
  ```

  Write the authored document only to the resolved `Project path`. It already
  includes the configured SpecBind root; do not reconstruct it from an artifact
  inventory `path`, the template-relative `Output path`, or the default root.

  Follow every `create output=<name>` instruction once to produce its named
  output. An output may be a short string or a Markdown fragment. Replace every
  reference to that name with the same produced output, and omit the `create`
  instruction from the live artifact. Copy each complete `maintain` and `consume` comment,
  including its opening marker, body, and closing marker, byte-for-byte. The
  Requirements section is deliberately empty and is not a valid live artifact.
  Replace it with at least one real Requirement and Acceptance Criterion before
  the first write; never persist the scaffold as completion.

  Do not run a Contract read in this branch. The lifecycle state already says
  that Design has not created one, so `ARTIFACT_SELECTOR_NOT_FOUND` would add no
  information.

- **Existing Spec** — read the current requirements and revise them in place:

  ```sh
  specbind artifact read <spec> requirements --for maintain
  ```

  Use the artifact inventory as the non-error existence check, then read the
  Contract as boundary context only when the inventory lists it:

  ```sh
  specbind artifact list <spec>
  specbind artifact read <spec> contract --for consume
  ```

  Immediately after this read and before drafting, build a private preservation
  ledger in working memory. Enumerate every existing Requirement group and
  acceptance criterion by its canonical ID, plus the unaffected owned behavior
  expressed by Context, Scope, and Objective. Do not write this ledger to the
  project. If you cannot account for the complete maintain projection, stop
  before editing rather than authoring from a partial baseline.

Never author the contract here. A new Spec has none until the design phase runs,
and that is correct.

## 2. Write the contract

Preserve an intentionally abstract but observable boundary from the Brief. For
example, if it says an action is accepted before a named window closes and
rejected afterward, express those outcomes without inventing a duration,
closing event, trigger mechanism, or other unstated policy. Ask for
clarification only when the supplied boundary cannot determine an observable
accepted or rejected outcome without inventing behavior.

Read the protocol that owns semantic quality, and the project's writing
preference, before writing:

```sh
specbind protocol read requirements-review
specbind protocol read okf-authoring
```

Read the project's phrasing preferences through the project-owned rule surface:

```text
specbind rule read ears-format --for consume
```

`NO_CHANGE RULE_ABSENT` means the project supplies no customization; the
protocol still applies. Any `ERROR` line stops this workflow.

Write constraints you took from steering **into the document**, in the
requirements' own terms. Steering is not fingerprinted, so nothing detects a
steering document that changes after this gate is approved. A requirement that
merely points at guidance is a requirement whose meaning can drift silently.

Apply the same rule to Source Items. Restate every accepted behavioral
obligation in the Requirements' own terms so the complete current contract
survives the milestone and can be fingerprinted. Keep provenance in the Brief;
do not make an acceptance criterion depend on following a source link. When two
items disagree about intended behavior, surface the conflict to the user rather
than selecting one silently.

For a new document, omit `create` instructions and copy every `maintain` and
`consume` instruction unchanged. For an existing document, preserve the durable
comments already returned by the maintain projection.

Keep technology, structure, and mechanism out. Those belong to design, which is
also why steering that is technical in nature cannot be carried in this
document — do not try to smuggle it in as a requirement.

A request for automated tests, coverage, or a canonical verification command is
delivery evidence, not by itself observable product behavior. Write the behavior
that evidence must verify; Design and Tasks own the testing mechanism and
coverage work. Do not create a Requirement solely to require tests unless the
test capability is itself part of the user-visible or system-visible product
contract.

Do not renumber existing requirement groups to close gaps. Identity is
positional, so renumbering silently reassigns IDs that `spec.yaml`, design
traceability, and task coverage already reference. Gaps are fine and permanent.

### Retirement is not supported yet

Do not remove a requirement group or an acceptance criterion from an established
Spec. There is no retired-ID registry and no way to prove downstream that an
obligation ceased to exist, so a removal would leave design, tasks, and
completion verification with nothing to cover.

When the requested result needs part of the current contract to disappear, stop
before editing and say that requirement retirement is not supported yet. Ask the
user how they want to proceed. Retiring everything a Spec owns is Spec
retirement, not an empty requirements document.

This does not freeze behavior:

- **Revising** a criterion in place is fine when the Spec keeps the
  responsibility and the same ID still names the changed obligation.
- **Adding** groups or criteria is fine.
- Only removing an obligation without leaving a live identity is blocked.

## 3. Choose the active set

The active set is the requirement IDs this milestone must **deliver or
re-verify**. Not the whole document, and not only the literal diff.

- Requirements whose behavior this work changes or adds are always in.
- Requirements whose correctness depends on that work are in even when their
  text is untouched, because they must be re-verified.
- Requirements unrelated to this work stay out, so the milestone is not forced
  to re-plan and re-test the Spec's entire contract.

**When membership is genuinely unclear, include it.** The two mistakes are not
symmetric. Over-including costs design and task effort on behavior that did not
need it. Under-including means design and tasks never cover behavior this
milestone actually changed — and nothing catches it, because coverage is checked
against the set you chose.

State the selection and why each ID is in it. This is part of what gets
approved, not a detail derived quietly at the end.

Use only the canonical positional form the Requirements parser derives: group
number plus acceptance-criterion list position, such as `2.1`, `2.2`, and
`2.3`. Never invent aliases such as `R2.AC1`. Before presenting the selection,
run `specbind check traceability <spec>` against the authored draft; while the
active set is still absent, it must recognize the full Requirements document
without a Requirements diagnostic.

## 4. Review and revise

Present the document and the selection. Revise on feedback rather than approving
something you know to be weak.

For an existing Spec, the review includes a mandatory preservation audit before
approval. Compare the authored file with the current Requirements you read at
the start (and inspect `git diff -- <requirements-path>` when Git is available).
Reconcile the authored file against the private preservation ledger. Account for
every pre-existing requirement group and acceptance criterion by its original
ID: it must still be present as that live identity, with only the
in-place revision this milestone actually needs. Context, Scope, and Objective
must also continue to describe unaffected owned behavior. A rewritten document
that silently narrows the Spec is not ready even when its new criteria are
well-formed. If any existing obligation disappeared, stop before approval;
restore an accidental omission in the same draft unless the requested result
truly needs retirement, in which case use the unsupported-retirement stop above.
Before presenting or approving, state to yourself that every ledger entry is
accounted for and zero obligations were lost. Never use the approve command as
the operation that reveals this loss.

When the review has findings, report each one in this shape:

```text
- [BLOCKING|DEFERRED|RESOLVED] <requirement or behavior at risk> — <where> — <consequence>
```

Every finding carries exactly one disposition from the protocol. An approval
with an undisposed finding is not a verdict, and a finding mentioned elsewhere
in the report does not count as disposed.

Stop and ask the user when the same objection survives one revision. A repeated
objection means the disagreement is about intent, not wording, and rewriting
again produces another variation of the same misunderstanding. You have run out
of information you can supply yourself; ask for it.

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

## 5. Approve

Approve only when the protocol's judgment is satisfied **and** you hold authority
for this gate. Authority is one of two things, never their absence:

Immediately before any approval command, rerun the traceability check. For an
existing Spec, also repeat the preservation-ledger reconciliation and require a
zero-lost-obligations result. This is a blocking preflight, including under
delegated approval; discovering a loss after approval is a failed workflow, not
a reason to invalidate the gate and repair it afterwards.

- **Explicit** — the user approved this document and this selection after seeing
  them.
- **Delegated** — a run context the user intentionally started authorized this
  gate by name. The user does not confirm this document, because delegation is
  exactly the decision to skip that pause for artifacts not yet written. Every
  check still runs, and you still state the selection in your report so the
  delegation stays auditable.

  **The workflow name comes from that context. Never invent one.** It identifies
  the authority you are exercising, so a name you made up identifies nothing and
  makes the record of a skipped confirmation false. If you were told the content
  is pre-approved but given no workflow name, you do not have a delegation —
  present your result and stop.

Never run a mutating command to find out what it accepts. Approval has no dry
run, so a probe with a placeholder value records a real approval.

```sh
specbind spec requirements approve <spec> --approval-mode explicit --requirement-ids <ids>
```

`--requirement-ids` takes the comma-separated canonical IDs. A delegated run
names the workflow that carries the authority:

```sh
specbind spec requirements approve <spec> --approval-mode delegated --delegation-workflow <workflow> --requirement-ids <ids>
```

No prompt appearing, a non-interactive invocation, and a scripted run grant no
authority. Without either form, present your result and stop.

Never approve to resolve a failing check. A refused approval is information
about the artifact; report the diagnostic rather than working around it.

`SPEC_REQUIREMENTS_RETIREMENT_UNSUPPORTED` means the approval wrote no gate
evidence and the Spec remains in Requirements. It is mechanical confirmation
that this phase's preservation preflight failed, not a reason to invalidate a
gate. Restore every named baseline ID and its unaffected behavior from the
original maintain projection, repeat the complete preservation reconciliation
and traceability check, then retry approval once. If the diagnostic repeats,
stop. `SPEC_REQUIREMENTS_BASELINE_READ_FAILED` is not authoring feedback; stop
without retry because the immutable comparison source is unavailable.

## 6. Checkpoint

Only after the approval succeeds is this work eligible to commit. A draft you
have not yet approved is never committed, however often the project wants
checkpoints. If you stopped short of approving, you also stop short of this.

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT` means there is no adapter-directed commit. Stop
there — that is an answer, not a missing file to work around.

An adapter carrying the exact `<!-- specbind:adapter-scaffold -->` marker is an
inactive scaffold, not project policy. Treat it as no guidance, say so in one
line, and commit nothing. The marker classifies the whole document: ignore every
other body line even when it looks actionable. Do not stop to ask about a file
nobody has filled in.

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

Routed work arrives with the gate already rewound, because discovery performs
confirmed invalidations before it changes scope. If you were invoked directly and
`spec status` shows the requirements gate approved, do not edit.

Editing underneath an approved gate leaves evidence describing a revision that no
longer exists, and the CLI then refuses later gates citing freshness rather than
the edit that caused it.

Tell the user what invalidation costs — it clears the design, tasks, and
completion evidence downstream — and run it only after they confirm:

```sh
specbind spec requirements invalidate <spec>
```

Confirmation cannot be inferred, and delegated authority does not cover this.
Delegation authorizes accepting gates, not discarding accepted work.

## Boundaries

- Author the requirements artifact only. The contract belongs to design;
  comparing an existing codebase against intended behavior belongs to
  `specbind-gap-analysis`.
- Write no machine state. Never edit `spec.yaml`. The active set reaches it only
  through the approve command.
- Report in the project's language: what the contract now says, what changed,
  which IDs are active and why, whether the work was committed, and what runs
  next.
