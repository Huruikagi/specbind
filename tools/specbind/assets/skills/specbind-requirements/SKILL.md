---
name: specbind-requirements
description: Author or revise a Spec's requirements as its complete current behavioral contract, choose the requirement IDs this milestone must deliver or re-verify, and approve the requirements gate.
argument-hint: "<spec>"
---

# Write the requirements

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

Read all the steering, not a promising-looking subset. The listing gives you a
selector, a type, and a path, and nothing in that says whether a document
constrains this Spec's behavior. A project constraint missed here is absent from
the contract every later phase checks against, so it is missed everywhere.

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
  specbind template read spec requirements
  ```

- **Existing Spec** — read the current requirements and revise them in place:

  ```sh
  specbind artifact read <spec> requirements --for maintain
  ```

Read the contract when the Spec has one, as context for the boundary it owns:

```sh
specbind artifact read <spec> contract --for consume
```

Never author the contract here. A new Spec has none until the design phase runs,
and that is correct.

## 2. Write the contract

Read the protocol that owns semantic quality, and the project's writing
preference, before writing:

```sh
specbind protocol read requirements-review
specbind protocol read okf-authoring
```

The project's `settings/rules/ears-format.md` states how this project prefers
requirements to be phrased. It is project-owned; if it is absent, the project
removed it deliberately and the protocol still applies.

Write constraints you took from steering **into the document**, in the
requirements' own terms. Steering is not fingerprinted, so nothing detects a
steering document that changes after this gate is approved. A requirement that
merely points at guidance is a requirement whose meaning can drift silently.

For a new document, omit `create` instructions and copy every `maintain` and
`consume` instruction unchanged. For an existing document, preserve the durable
comments already returned by the maintain projection.

Keep technology, structure, and mechanism out. Those belong to design, which is
also why steering that is technical in nature cannot be carried in this
document — do not try to smuggle it in as a requirement.

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

## 4. Review and revise

Present the document and the selection. Revise on feedback rather than approving
something you know to be weak.

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
