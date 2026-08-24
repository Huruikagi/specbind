# 0112: Fix the implementation validation skill contract

Status: Accepted

## Context

[Decision 0086](./0086-completion-cli-handshake.md) fixes the Spec completion
handshake and assigns it: "`specbind-validate-implementation` routes Spec-backed
work through the three Spec completion commands." It also fixes the verdict
vocabulary in passing — "`NO-GO` and `MANUAL_VERIFY_REQUIRED` remain run-scoped
and do not invoke `accept`" — and the candidate document shape.
[Decision 0033](./0033-completion-mechanical-checks.md) fixes the evidence
entries and says "the validation skill derives the required set from project
automation and rules."
[Decision 0082](./0082-derived-milestone-state-machine.md) puts a milestone-wide
convergence barrier before this validation, and
[Decision 0080](./0080-v1-task-contract-and-completion-details.md) makes
completion evidence project-revision-scoped so a later commit stales it.

One thing those decisions rely on does not exist. Decision 0086 has "the
validation skill runs mechanical checks **and the mandatory semantic protocol**
against that checkout," and [Decision 0075](./0075-v1-skill-and-orchestration-scope.md)
gives `specbind-verify-completion` the job of applying "the mandatory
completion-verification protocol without becoming a workflow stage." No such
protocol is in the Decision 0094 set.

This decision adds it, and fixes the orchestration around it.

## The completion-verification protocol

`completion-verification` joins the Decision 0094 set. It owns one thing: the
relationship between a claim and the evidence for it — state the claim, identify
what would prove that claim, require fresh evidence, and reject any claim
broader than its evidence.

It has two consumers, which is why it is a protocol rather than skill-local
content. This skill applies it before returning `GO`, and
`specbind-verify-completion` applies the same gate standalone, which is exactly
what Decision 0075 means by "without becoming a workflow stage."

Its content is largely inherited rather than invented. The cc-sdd verification
skill already carried the claim-versus-evidence gate, the graded evidence
requirements per claim, and a table of rationalizations — including the one that
matters most here, that a passing test suite alone never establishes that a
whole implementation is complete.

## Decision

### The verdict set is already fixed, and only one of them mutates

`GO`, `NO-GO`, and `MANUAL_VERIFY_REQUIRED`, as Decision 0086 names them. Only
`GO` proceeds to `spec completion accept`.

`MANUAL_VERIFY_REQUIRED` is not a weaker `NO-GO` and never a route to `GO`. They
carry different information: `NO-GO` means something is known to be wrong;
`MANUAL_VERIFY_REQUIRED` means a mandatory check could not be performed, so
nothing is known either way. Collapsing them would either invent a failure or
hide a gap, and the second is how an unverified Spec reaches `release_ready`.

### The checks are run, not assembled

The skill derives the required mechanical checks from the project's own
automation and rules under Decision 0033, then **runs them** and records what
they actually returned.

It never composes a plausible check list from what a project probably has,
never reports a command it did not execute, and never submits a check whose exit
status it did not observe. Decision 0086 is explicit that the CLI cannot make
this judgment — "neither acceptance command claims that command text proves
execution; the invoking skill owns that judgment" — so the entire integrity of
the recorded evidence rests here.

A failing check is `NO-GO`. A check that cannot be identified or executed is
`MANUAL_VERIFY_REQUIRED`. Neither is repaired by choosing a different command
that passes.

The validator also snapshots Git status immediately before and after every
canonical project command, with no cleanup in between. A zero exit code that
leaves new caches, reports, or other untracked output is `NO-GO`: release and
user runs repeat the command without this validator's cleanup context. The
validator reports the paths and leaves repair to implementation rather than
deleting evidence to manufacture a clean completion.

### Independent dimensions are dispatched

The validation dimensions — full-suite results, runtime liveness, active
Requirement coverage, cross-task integration, Design alignment, blocked-task
status — are independent enough to investigate separately and voluminous enough
to crowd out the judgment that has to follow them.

Under [Decision 0109](./0109-subagent-dispatch-contract.md) they are dispatched
as fresh subagents with self-contained briefs, returning structured findings
rather than raw output. **The verdict is synthesized in the main context**, for
the reason the design phase keeps its own synthesis: the decision needs the
whole picture, and no dispatched part has it.

Dispatch is proportional. A small Spec whose checks are two commands does not
need it.

### The skill never repairs what it validates

A validator that fixes what it finds has stopped validating, and its `GO` then
attests to work it produced itself. Findings return to `specbind-implement`.

This is the same boundary [Decision 0111](./0111-review-task-and-debug-skill-contracts.md)
draws for review, one level up, and it matters more here because this verdict is
what writes durable evidence.

The skill may run commands. It may not change source, weaken a check, or edit
Requirements, Design, the Contract, or the plan.

### Convergence, and the multi-Spec acceptance sequence

Decision 0082 requires implementation to converge before final validation, and
Decision 0080 makes evidence stale when any later non-metadata commit lands. The
skill therefore validates at the final code revision, and re-validates a Spec
whose evidence went stale rather than treating an earlier `GO` as durable.

The multi-Spec case has a sequence Decision 0086 fixes and assigns to the agent.
The first acceptance at a revision requires a completely clean worktree; later
acceptances at the same revision tolerate only the other participants'
`implementation` to `release_ready` `spec.yaml` transitions, and "the agent
commits the accepted metadata set together."

That last clause is this skill's obligation and is easy to lose. Accepting
several Specs leaves several uncommitted `spec.yaml` files, and the tolerance
that permitted them is narrow: any other worktree change fails closed. The skill
commits that metadata set as one commit, containing only those files, and does
not interleave other work into it.

Committing here is not an exception to Decision 0101. Where the project's Git
adapter forbids committing, the skill stops after the acceptances and reports
that the metadata set is uncommitted, rather than leaving the milestone in a
state only it understands.

### Invalidation is explicit and confirmed

`spec completion invalidate` is used when a Spec sits at `release_ready` with
evidence that no longer holds and must be revalidated. The skill states what it
clears — completion evidence only, leaving Requirements, Design, and Tasks
intact — and runs it after confirmation.

It is not used to clear a path to a new `GO`. A refused validation is
information about the implementation.

### Direct items are not validated here

Decision 0086 gives Direct completion to the implementation skill and states that
Direct work "does not create a synthetic Spec." This skill validates Spec-backed
items only, and a request to validate a Direct item is reported as belonging to
`specbind-implement`.

### Boundary

- The skill validates one Spec and, on `GO`, performs its completion handshake.
- It authors no Requirements, Design, Contract, or plan, approves no gate, and
  records no task progress.
- It repairs nothing.
- It commits only the accepted completion metadata set, and only where the
  project's adapter permits committing.

## Consequences

- The protocol Decision 0086 already referenced exists, so "the mandatory
  semantic protocol" resolves to something a subagent can read.
- The two refusal verdicts stay distinct, so an unperformable check cannot
  quietly become a pass.
- The one judgment the CLI explicitly cannot make — that a recorded command was
  actually executed — has a stated owner and an explicit prohibition against the
  shortcut.
- The multi-Spec metadata commit has an owner, so the narrow tolerance Decision
  0086 grants is not left to chance.
- Validation stays separable from implementation, so a `GO` is an assertion by
  something that did not do the work.

## Implementation status

Implemented.
`tools/specbind/assets/skills/specbind-validate-implementation/SKILL.md` is
embedded and installed, and the `completion-verification` protocol is embedded.

`specbind-verify-completion`, the protocol's second consumer, is authored under
its own decision.

Its forward tests are specified as scenarios VI1 through VI3 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually.
