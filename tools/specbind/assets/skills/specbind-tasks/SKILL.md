---
name: specbind-tasks
description: Decompose an approved design into the Spec's executable task plan, verify it against the schema and requirement coverage, and approve the tasks gate.
argument-hint: "<spec>"
---

# Plan the work

Turn the approved design into `tasks.yaml`: the ordered plan an implementer
executes and the CLI derives progress from.

You author one file and approve one gate. You never record progress against it —
that belongs to implementation.

## 1. Check the order before anything else

```sh
specbind spec status <spec>
specbind milestone review status
```

**The contract review must be accepted before a plan exists.** Proceed only on
`Status: fresh`.

From the `tasks` state onward `spec status` also reports `Contract review:`, so
use it as corroborating Spec-local context. `milestone review status` remains
the authoritative focused check and is always required before first authoring.

On `absent` or `stale`, author nothing and route the user to
`specbind-contract-review`. Say why, because the consequence is not visible from
either report: `milestone review accept` refuses while a `tasks.yaml` is present
(`CONTRACT_REVIEW_TASKS_ALREADY_EXIST`). Writing the plan now turns a missing
prerequisite into a deadlock whose only exit is deleting the plan you just
wrote.

This check is for **first authoring**. A plan that already exists and is being
revised does not re-enter that path — `tasks invalidate` keeps the accepted
review, because the review is still valid at the tasks state.

### Then check the prerequisite gate

`spec status` must report the design gate approved and fresh, and the Spec a
current participant of the active milestone.

If it does not, stop and say so. Route the user to `specbind-design`. Never
approve or invalidate an upstream gate, and never edit the requirements, design,
or contract to make a plan work.

## 2. Read

```sh
specbind artifact read <spec> design/main
specbind artifact read <spec> requirements
specbind artifact read <spec> contract
```

Use `specbind artifact list <spec>` first when the Spec's design is split across
several documents, and read all of them.

The design is your primary input — the plan decomposes it. The requirements give
you the active IDs the plan must deliver. The contract gives you the entry
references a task may name.

### Read the schema, do not recall it

```sh
specbind schema read tasks/v1
```

Tasks has no template. The schema **is** the structure, and it is strict:
unknown fields fail, empty arrays fail, `parallel: false` fails. Writing from a
remembered shape means debugging validation errors against a document you could
have read first.

### Steering is not read here

This is the one authoring phase that does not read steering. Requirements and
design each read it whole and wrote what they took from it into their own
documents, so the constraints are already carried by the artifacts you are
decomposing. Reading it again invites the plan to introduce an obligation no
approved artifact contains, and nothing downstream checks the plan against
steering.

If you find you cannot write the plan without consulting steering, that is a
finding about the **design**: it does not yet determine the work. Report it and
return to design rather than patching around it.

The project's `settings/rules/tasks-generation.md` is a shared rule, not
steering. Read it — it states this project's task sizing, decomposition order,
test-work convention, and known conflict areas. It is project-owned; if it is
absent, the project removed it deliberately and the protocol still applies.

## 3. Write the plan

```sh
specbind protocol read task-planning
```

The protocol owns the judgment: coverage is delivery rather than mapping, every
task is work that will be done, order carries the dependencies, and a
`parallel: true` marking must survive all five of its conditions.

Write `tasks.yaml` at the Spec's directory. A few things the schema enforces and
the protocol assumes:

- Task IDs are **positional** — `1`, `2`, `1.1`, `1.2` — and match array
  position. There are no gaps and no third level.
- A group holds at least two tasks and carries no requirement, boundary, or
  completion fields of its own.
- `requirement_ids` is required on every executable task and non-empty.
- Omit `details`, `completion_criteria`, `boundaries`, `contracts`, and
  `depends_on` rather than writing them empty.
- `parallel: true` requires a non-empty `boundaries`.
- `depends_on` names only tasks in this same file. A cross-spec dependency is a
  roadmap or contract edge, never a Task ID.

Do not write `execution`. That state belongs to implementation, and a plan that
arrives claiming completed work records a judgment nobody made.

### The YAML itself has traps

**Quote any string containing `: `.** In YAML, `Reject the addition: leave the
cart unchanged` is a *mapping*, not a sentence — and the failure surfaces as a
schema error about a field you never wrote, several lines from the real cause.
Task titles and `details` entries are where this bites. A `#` after a space
starts a comment, so quote those too.

The YAML SpecBind reads is deliberately restricted. **No anchors, aliases, merge
keys, custom tags, or multiple documents** — all are rejected outright. Write the
value out rather than reaching for a YAML feature to share it.

When `tasks list` reports something you cannot place, suspect the quoting before
suspecting the schema. Do not install a YAML or JSON-Schema tool to investigate:
the CLI is the validator, its diagnostics carry the line, and reaching outside
the project for a second opinion changes the machine rather than the plan.

## 4. Verify what you wrote

After **every** write:

```sh
specbind tasks list <spec>
specbind check traceability <spec>
```

The first proves the document parses, satisfies the strict schema, and produces
a coherent derived model. The second proves every active requirement ID is
mapped to an executable task.

Both are read-only. Run them before you present anything, so a structural fault
is something you fixed rather than something the approval refused.

## 5. Revising a plan that has recorded progress

If the Spec is in `implementation`, tasks may already be completed or blocked,
and this is where a plan revision can destroy information silently.

Identity is positional. Inserting, removing, or reordering renumbers every task
after that point, and `execution.tasks` is keyed by those same identifiers. Get
this wrong and the document still validates: `tasks list` shows a plausible mix
of completed and pending, while a completed record now sits on work nobody did.

So:

- Rewrite the `execution.tasks` keys in the **same edit** that renumbers the
  plan. Never leave the two disagreeing.
- **State the before-and-after mapping** for every renumbered task that has a
  persisted entry, and get confirmation before writing.
- Prefer a revision that leaves completed tasks where they are — appending, or
  splitting a task that has not started — over one that renumbers finished work,
  when both express the same intent.

Restructuring an approved plan is legitimate. Doing it in a way that mislabels
completed work is not.

## 6. Review and revise

Present the plan: the decomposition and its order, which active requirements each
task delivers, and **every task marked `parallel: true` with the boundary that
justifies it**.

Call the parallel markings out explicitly. Overlap between them is a warning, not
a rejection, so this is exactly the part the CLI will not refuse on your behalf.

Revise on feedback rather than approving something you know to be weak. Stop and
ask the user when the same objection survives one revision — a repeated objection
is a disagreement about intent, and rewriting again produces another variation of
the same misunderstanding.

If an objection reveals that the **design** does not determine the work, return
it there. A plan that compensates for an underspecified design moves the decision
into a document nothing verifies against the requirements.

## 7. Approve

Approve only when the protocol's judgment is satisfied **and** you hold authority
for this gate. Authority is one of two things, never their absence:

- **Explicit** — the user approved this plan after seeing it.
- **Delegated** — a run context the user intentionally started authorized this
  gate by name. Every check still runs.

  **The workflow name comes from that context. Never invent one.** If you were
  told the content is pre-approved but given no workflow name, you do not have a
  delegation — present your result and stop.

Never run a mutating command to find out what it accepts. Approval has no dry
run.

```sh
specbind spec tasks approve <spec> --approval-mode explicit
```

A delegated run names the workflow that carries the authority:

```sh
specbind spec tasks approve <spec> --approval-mode delegated --delegation-workflow <workflow>
```

There are no IDs or fingerprints to pass. The CLI derives its input from the
normalized plan projection.

No prompt appearing, a non-interactive invocation, and a scripted run grant no
authority. Without either form, present your result and stop.

Never approve to resolve a failing check. A refused approval is information about
the plan; report the diagnostic rather than working around it.

## 8. Checkpoint

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
line, and commit nothing. Do not stop to ask about a file nobody has filled in.

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

If `spec status` shows the tasks gate approved, do not edit. Editing underneath
an approved gate leaves evidence describing a plan that no longer exists, and the
CLI then refuses later work citing freshness rather than the edit that caused it.

State the cost accurately and run it only after the user confirms. This is the
**cheapest** of the three rewinds, and overstating it pushes people away from the
right operation:

- it clears the tasks and completion evidence, and
- it **keeps** the accepted contract review, and the requirements and design
  gates.

Add what the revision itself will cost when implementation has started — which
recorded progress is affected, per the mapping rule above.

```sh
specbind spec tasks invalidate <spec>
```

Confirmation cannot be inferred, and delegated authority does not cover this.
Delegation authorizes accepting gates, not discarding accepted work.

## Boundaries

- Author `tasks.yaml` only. Requirements, design, and the contract belong to
  earlier phases; execution state and implementation notes belong to
  implementation.
- Write no machine state. Never edit `spec.yaml`.
- Never run `tasks complete`, `tasks block`, or `tasks reopen`. Those record an
  implementer's judgment.
- Do not accept the contract review, and do not delete a plan to unblock one.
  If a plan exists and the review was never accepted, the order is already lost —
  report it and let the user decide, because discarding an authored plan is their
  call.
- Report in the project's language: the decomposition, the order and why, which
  requirements each task delivers, any parallel markings, whether the work was
  committed, and what runs next.
