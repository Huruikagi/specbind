# 0111: Fix the task review and debug skill contracts

Status: Accepted

## Context

`specbind-review-task` and `specbind-debug` are the two skills that exist in
both of the shapes [Decision 0109](./0109-subagent-dispatch-contract.md)
created: dispatched as a fresh subagent by `specbind-implement`, and invoked
directly by a user. Their semantic baselines are already embedded as the
`task-review` and `debug` protocols under
[Decision 0110](./0110-implement-skill-contract.md).

[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) fixes what each
returns: review judges one task implementation against the actual diff and the
approved inputs, and debug is "a read-only, fresh-context root-cause protocol"
that "returns a run-scoped diagnosis and next action; **a new implementer
applies any fix**."

One decision covers both because the questions left open are the same ones, and
answering them differently for the two skills would be an accident rather than a
distinction: what changes between the dispatched and the direct moment, what
neither skill may write, and how a skill defined by fresh context behaves when
it is invoked in a context that is not fresh.

## Decision

### The criteria never change; the consequence does

Each skill applies its protocol identically in both moments. What differs is
what happens to the result: a dispatched verdict drives another skill's control
flow, while a directly invoked one is reported to the user who asked.

This mirrors the arrangement Decision 0094 already established for
`design-validation`, and it exists for the same reason. A review that is more
lenient because a human will read it, or a diagnosis that is less rigorous
because nothing automatic depends on it, produces two standards for one
question.

### Neither skill changes the thing it is examining

Both are read-only with respect to the work under examination, and this is the
load-bearing rule for each.

- **Review** never fixes what it finds. A reviewer that repairs the change has
  destroyed the artifact under review and left the implementer with a verdict on
  work it did not produce. The finding is the deliverable.
- **Debug** changes nothing in the repository at all. Decision 0075 makes it
  read-only, and the reason is evidentiary: an edited repository leaves the next
  agent unable to tell which state its reasoning applies to, and destroys what a
  second diagnosis round would need. It may run commands that reproduce or
  observe the failure; it may not modify tracked files.

Neither writes lifecycle or execution state. Recording a task completed is the
implementer's judgment under [Decision 0095](./0095-task-progress-cli.md), and a
reviewer that recorded its own approval would collapse two separate judgments
into one.

### Implementation Notes belong to the implementation skill

[Decision 0026](./0026-runtime-implementation-notes.md) lists debugging and
review among the workflows that update Implementation Notes.
[Decision 0092](./0092-template-skill-authoring-boundary.md) is later and more
specific, assigning the durability judgment and the creation or update timing to
the implementation skill alone. This decision follows 0092.

The inherited tree corroborates the narrower reading. Across cc-sdd's skills,
`kiro-debug`, `kiro-review`, and `kiro-validate-impl` only **read** the
`## Implementation Notes` section, and `kiro-impl` is the only writer — including
for debug findings, which it records on the diagnosis's behalf so a later task
avoids the same issue. Decision 0026's prose conflates reading with updating;
the behavior it described already had one writer.

Both readings are reconcilable in practice, and the narrower one is correct
here. Durable knowledge discovered during review or diagnosis is returned in the
finding or the diagnosis, and the skill that applies the fix records it. A
reviewer writing to the repository would contaminate the diff it is reviewing,
and a diagnosis that writes has stopped being read-only. Both still **read**
notes when present, which is what makes a diagnosis aware of a trap an earlier
task already hit.

This places one obligation on the dispatcher, and
[Decision 0110](./0110-implement-skill-contract.md)'s skill carries it: a
diagnosis is run-scoped, so durable knowledge it surfaced is lost unless the
implementation run writes it down.

### Structured returns in both moments

Each skill returns the closed-set result its protocol defines — the three review
verdicts, and the four debug categories with a cause and a next action — in a
parseable block, whether dispatched or invoked directly.

Keeping the block in the direct case costs nothing and removes a class of
divergence: one output shape means the skill cannot drift into a chatty variant
that a dispatcher would later fail to parse. The surrounding explanation is free
prose in both cases.

### Review scopes itself to one task

Review judges one task. When the working tree contains changes that task does
not own, the reviewer does not silently review them and does not guess which
hunks belong.

The `task-review` protocol already provides the outcome: **cannot review**, on
the grounds that the change is entangled with unrelated work. That is a real
result and more useful than a verdict on an unknown subject. It is also why
Decision 0110 has the implement run stop rather than rescue a dirty worktree —
the two rules protect the same property from opposite ends.

### Debug invoked in a context that is not fresh

Fresh context is part of what Decision 0075 defines debug to be, and a user
invoking it inside the session that just failed cannot supply one.

The skill does not pretend otherwise. It states that it is reasoning inside a
context that already contains the failed attempts, names what it is deliberately
setting aside, and re-derives the cause from the evidence rather than from the
session's accumulated conclusions. Where the failure has already survived
several attempts in that session, it says that a genuinely fresh session would
produce a more trustworthy diagnosis.

The alternative — refusing to run outside a dispatch — would make the one skill
whose purpose is breaking a stuck loop unavailable exactly when a user is stuck.
The alternative in the other direction, running as though the context were
clean, would produce the confident re-derivation of a failed model that Decision
0109 introduced dispatch to prevent.

### Bounds belong to the caller

Neither skill counts rounds. Decision 0075 bounds automatic remediation and
diagnosis at two rounds, and Decision 0110 places that counting in
`specbind-implement`. A dispatched review or diagnosis performs one, returns it,
and ends.

A directly invoked run has no bound, because the user is the loop.

### Boundary

- Review authors nothing, fixes nothing, and records nothing. It reads the diff,
  the task, and the approved artifacts, and returns a verdict.
- Debug writes nothing at all, and never applies its own fix. Decision 0075
  gives the fix to a new implementer.
- Neither approves or invalidates a gate, records task progress, or updates
  Implementation Notes.
- Neither implements. A review that finds a one-line fix reports the one-line
  fix; making it is someone else's judgment to record.

## Consequences

- One standard serves both moments for each skill, so a dispatched result and a
  user-facing one are worth the same.
- The reviewed diff stays exactly what the implementer produced, which is what
  makes the verdict mean anything.
- A diagnosis leaves the failing state intact, so a second round has the same
  evidence the first had.
- Implementation Notes have one writer, so 0026's broader list stops implying
  three.
- Debug remains available in a stuck session while being honest about what that
  costs, rather than being unavailable or quietly compromised.
- Both skills stay small, because everything substantive already lives in their
  protocols.

## Implementation status

Implemented. `tools/specbind/assets/skills/specbind-review-task/SKILL.md` and
`tools/specbind/assets/skills/specbind-debug/SKILL.md` are embedded and
installed.

Their forward tests are specified as scenarios RT1, RT2, and DB1 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually. Both
skills are additionally exercised through the implementation scenarios, where
`specbind-implement` dispatches them.
