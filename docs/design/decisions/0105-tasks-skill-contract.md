# 0105: Fix the tasks skill contract

Status: Accepted

## Context

The task plan is the most completely specified artifact SpecBind has.
[Decision 0013](./0013-structured-task-artifact.md) makes `tasks.yaml`
canonical, [Decisions 0019](./0019-task-ordering-and-dependencies.md) through
[0028](./0028-task-plan-fingerprint.md) fix its ordering, identifiers,
completion criteria, sparse shape, execution state, read model, locality, and
fingerprint, [Decision 0039](./0039-minimal-tasks-gate-evidence.md) fixes the
gate evidence, [Decision 0080](./0080-v1-task-contract-and-completion-details.md)
fixes boundaries and Contract references,
[Decision 0095](./0095-task-progress-cli.md) exposes progress commands, and
[Decision 0103](./0103-schema-read-surface.md) publishes the schema through the
CLI. The `task-planning` protocol carries the semantic baseline and
`tasks-generation.md` carries project preference.

The protocol names what is left: "Approval and regeneration belong to the tasks
skill." [Decision 0092](./0092-template-skill-authoring-boundary.md) has no row
for Tasks in its artifact table, because Tasks has no template — the schema is
its structure and the CLI publishes it.

Two things make this skill's contract different from the phases before it. It is
the only authoring skill whose artifact the CLI both fully validates and cannot
write, and it is the only one whose ordering constraint is enforced by a command
it never runs.

## The review comes first, and nothing says so

[Decision 0078](./0078-contract-first-review-between-design-and-tasks.md)
requires the accepted contract review between Design approval and Tasks
authoring, and states it from the review's side: "No current `tasks.yaml` is
authored until review passes." Acceptance enforces it mechanically —
`milestone review accept` refuses with `CONTRACT_REVIEW_TASKS_ALREADY_EXIST`
while a plan is present.

Read from the tasks phase, that is a trap rather than a guard. An agent that
authors the plan first has not violated a check it could see: `tasks list`
validates the plan happily, and the refusal arrives later, in a different
command, run by a different skill, naming a file the tasks phase considers its
own output. The only way out is deleting an authored plan, which looks like
destroying work rather than restoring an order.

No accepted decision states the constraint in the direction the tasks phase
travels. This decision does, and assigns the check to this skill.

When this decision was taken, `spec status` also reported nothing about the
review, so the skill's check was the only thing standing between an agent and
that trap. [Decision 0107](./0107-spec-status-contract-review-barrier.md)
subsequently made the barrier visible in Spec status from the `tasks` state
onward. The check here is unchanged and remains required: the CLI now reports
that the review is missing, but nothing except this skill states that authoring
a plan first is what makes recovering from it expensive.

## Decision

### What the skill reads

| Read | When |
| --- | --- |
| `specbind spec status <spec>` | always |
| `specbind milestone review status` | always, before authoring |
| The Spec's Design set | always |
| The Spec's Requirements | always |
| The Spec's Contract | always |
| `specbind schema read tasks/v1` | always |
| The current plan through `specbind tasks list <spec>` | only when one exists |

The Design is the primary input: the plan decomposes it, and the protocol
forbids restating its mechanism. Requirements supply the active IDs the plan
must deliver. The Contract supplies the entry references a task may name under
Decision 0080.

The schema is read rather than remembered. Tasks has no template, so the schema
is the structure, and Decision 0103 published it for exactly this consumer. A
plan written from a recalled shape produces strict-validation failures the skill
then debugs against a document it could have read.

**Steering is not read.** This is the one phase that does not, and the reason is
worth stating rather than leaving as an omission. Requirements and Design each
read steering whole and are required to write what they took from it into their
own documents, so by the time a plan is authored the project's constraints are
already carried by the artifacts the plan decomposes. Reading steering again
would invite the plan to introduce an obligation that no approved artifact
contains, which nothing downstream verifies.

The consequence is a diagnostic rather than a gap. When the plan cannot be
authored without consulting steering, the Design does not yet determine the
work — which the `design-validation` protocol already names as a defect, in its
own words, a section too vague for a task to reference directly. The skill
reports that and returns to Design instead of patching around it.

Project preference for task shape lives in `tasks-generation.md`, which the skill
does read. That is a shared rule, not steering.

### The ordering check

Before authoring or revising a plan, the skill runs
`specbind milestone review status` and proceeds only on `fresh`.

- **`missing` or `stale`** — the skill authors nothing and routes to
  `specbind-contract-review`. It states why: acceptance refuses while a plan
  exists, so authoring now converts a missing prerequisite into a deadlock whose
  only exit is deleting the plan.
- **`fresh`** — authoring proceeds.
- **`not required`** — a Direct-only milestone has no Spec-backed items, so this
  skill has nothing to plan for.

This check is skipped only when the plan already exists and the tasks gate is
being revised rather than first authored. A revision does not re-enter the
acceptance path: Decision 0088 has `tasks invalidate` keep the accepted review
precisely because the review remains valid at the `tasks` state.

### Prerequisites are checked, not repaired

The skill authors nothing until `spec status` reports the design gate approved
and fresh and the Spec a current participant. A stale or unapproved design gate
is reported and routed to `specbind-design`. The skill never approves or
invalidates an upstream gate and never edits Requirements, Design, or the
Contract to make the plan work — the same rule Decision 0104 fixes for the phase
before it, for the same reason.

### The skill writes the file, and the CLI checks it

No command authors `tasks.yaml`. The skill writes the document itself, which
makes structural validation its own responsibility rather than a side effect of
approval.

After every write, the skill runs `specbind tasks list <spec>` and
`specbind check traceability <spec>`. The first proves the document parses,
satisfies the strict schema, and produces a coherent derived model; the second
proves every active Requirement ID is mapped. Both are read-only and both are
run before presenting anything to the user, so a structural fault is repaired
silently rather than surfacing as a refused approval.

The skill never writes execution state. `execution.tasks` is owned by the
Decision 0095 commands, and those belong to `specbind-implement`. Authoring a
plan that already claims completed work would record a judgment nobody made.

### Revising a plan that has execution state

A plan revised while the Spec is in `implementation` is the one case where
authoring can destroy information, and it has no CLI to guard it.

Decision 0020 makes identity positional, so inserting, removing, or reordering
renumbers every task that follows, and Decision 0024 keys execution state by
those same identifiers. Rewriting the plan without rewriting the keys either
orphans a completed record or, worse, leaves it attached to a task that now
names different work. The second failure is silent: the document stays
schema-valid, `tasks list` reports a plausible mix of completed and pending, and
nothing anywhere states that the completed one is not the work that was done.

The skill therefore:

- rewrites `execution.tasks` keys in the same edit that renumbers the plan, so
  the two never disagree;
- states the before-and-after mapping for every renumbered task with a persisted
  execution entry, and obtains confirmation before writing;
- prefers a revision that leaves completed tasks in place — appending, or
  splitting a task that has not started — over one that renumbers work already
  done, when both express the same intent.

Decision 0019 permits restructuring an approved plan and only warns against
doing it for cosmetic reasons. This adds the handling that makes the permitted
case safe.

### Approval

Approval follows Decision 0100's contract without variation. The skill approves
through `specbind spec tasks approve` only after the `task-planning` protocol's
judgment is satisfied and it holds explicit or delegated authority, where the
delegation label comes from the run context and is never invented. Absence of a
prompt grants nothing, neither form authorizes approving to resolve a failing
check, and no mutating command is used to discover what it accepts.

`tasks approve` takes no IDs or fingerprints; Decision 0039 derives its single
input from the normalized plan projection.

The report states the decomposition and its order, which active Requirements each
task delivers, and every task marked `parallel: true` with the boundary that
justifies it. The parallel markings are called out because Decision 0080 makes
overlap a warning rather than a rejection, so they are the part of the plan the
CLI will not refuse on the skill's behalf.

### Review loop

The skill revises and re-presents rather than approving a plan it knows to be
weak, and stops to ask when the same objection survives one revision, for the
reason Decision 0100 gives.

An objection that reveals the Design does not determine the work is returned to
Design rather than absorbed into more detailed tasks. A plan that compensates for
an underspecified Design moves the decision into a document that nothing
verifies against the Requirements.

### Rewind

Invoked on a Spec whose tasks gate is approved, the skill does not edit. It
states the cost and runs `specbind spec tasks invalidate` only after explicit
user confirmation.

The cost is stated accurately in both directions, because this is the cheapest of
the three rewinds and overstating it would push a user away from the correct
operation. It clears the tasks and completion evidence and **keeps** the accepted
contract review and the requirements and design gates. When implementation has
started, the skill adds what the revision itself will cost: which recorded
progress is affected, per the mapping rule above.

Delegated authority does not cover invalidation.

### Boundary

- The skill authors `tasks.yaml` only. Requirements, Design, and the Contract
  belong to earlier phases; execution state and Implementation Notes belong to
  implementation.
- It writes no machine state and never edits `spec.yaml`.
- It does not accept the contract review, and does not delete a plan to
  unblock one. When a plan exists and the review has not been accepted, the
  ordering was already lost; the skill reports the situation and lets the user
  decide, because deleting an authored plan is their call.
- It never runs `tasks complete`, `tasks block`, or `tasks reopen`.

## Consequences

- The one ordering constraint that is enforced by a command the tasks phase
  never runs is now stated in the direction that phase travels, before the
  authoring that would trigger it.
- The plan's structural correctness is verified by the skill that wrote it,
  rather than discovered at the approval it blocks.
- Renumbering a plan with recorded progress has a stated handling, so the one
  authoring action that can silently mislabel completed work requires a
  confirmed mapping.
- Steering has an explicit stopping point, and a plan that seems to need it
  produces a Design finding instead of an unverifiable obligation.
- The parallel markings are surfaced at approval, which is where the CLI stops
  helping.
- The tasks rewind is described as the cheap one it is, so the correct recovery
  is not avoided out of misplaced caution.

## Implementation status

Implemented. `tools/specbind/assets/skills/specbind-tasks/SKILL.md` is embedded
and installed, carrying the review-status check with its deadlock explanation,
the prerequisite check that routes rather than repairs, schema-driven authoring
with the two read-only verifications after every write, the execution-key
renumbering rule with its confirmed mapping, the two forms of approval authority,
the repeated-objection stopping rule, and confirmed self-invalidation that states
what the tasks rewind does and does not clear.

Its forward tests are specified as scenarios T1 through T5 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually. D7
becomes measurable with this skill embedded.
