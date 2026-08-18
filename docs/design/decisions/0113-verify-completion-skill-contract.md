# 0113: Fix the completion verification skill contract

Status: Accepted

## Context

[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) keeps
`specbind-verify-completion` in the v1 set, and the target skill catalog states
its purpose precisely: "Apply the mandatory completion-verification protocol
**without becoming a workflow stage**."

The protocol it applies now exists, added by
[Decision 0112](./0112-validate-implementation-skill-contract.md) with this
skill as its second consumer.

What remains is the part that phrase implies but does not spell out. Every other
v1 skill takes a Spec or a Roadmap item and moves the workflow forward. This one
takes neither and moves nothing, which makes its contract almost entirely a set
of things it must not become.

## Decision

### Its subject is a claim, not a Spec

The skill is invoked with an assertion someone is about to make — a task is
done, a defect is fixed, a command passed, an implementation is complete — and
answers whether the evidence supports that assertion.

This is the one v1 skill whose argument is not a canonical identity. It follows
from what it is for: the claim is the thing at risk, and a claim can be made
about work that has no Spec, no Roadmap item, and no lifecycle state.

### It is not a stage, and the failure mode is helpfulness

It writes nothing, transitions nothing, records nothing, and completes nothing.
Not on refusal, and — this is the case worth stating — **not on success either**.

A skill that has just confirmed an implementation is complete is one short step
from recording that completion, and the step looks like helpfulness rather than
overreach. It would be neither. Completion evidence is written by
`specbind-validate-implementation` through the Decision 0086 handshake, which
independently rechecks guards this skill never evaluates: gate freshness,
contract review freshness, milestone convergence, and a clean revision. A
`VERIFIED` here means the claim is supported, not that the Spec may advance.

Keeping it out of the lifecycle is also what lets it be invoked anywhere without
consequence, including before there is anything to advance.

### Verdicts

`VERIFIED`, `NOT_VERIFIED`, and `MANUAL_VERIFY_REQUIRED`, as the inherited skill
used them.

These are deliberately not the `GO` / `NO-GO` / `MANUAL_VERIFY_REQUIRED` set
that Decision 0086 fixes for `specbind-validate-implementation`. That set is a
verdict on one Spec's readiness to advance and one of its values mutates state.
This set is a verdict on whether a statement is supported. Sharing the words
would invite the two to be read as the same judgment, when only one of them
authorizes anything.

`MANUAL_VERIFY_REQUIRED` keeps the meaning the protocol gives it: a mandatory
check could not be performed, so nothing is known either way. It is never a
softer refusal and never a route to `VERIFIED`.

### Fresh evidence means evidence this run produced

The skill runs the check itself wherever it can, rather than accepting reported
output. The protocol already rejects "the report said it succeeded" as evidence,
and this skill exists mainly because that rationalization is persuasive in the
moment.

Where it is handed output it cannot reproduce — from a subagent, from an earlier
run, from the user — it says so and treats the claim as resting on evidence it
did not observe. That is usually `MANUAL_VERIFY_REQUIRED` rather than
`VERIFIED`.

### It repairs nothing

A refused claim is returned as a refusal with what is missing. The skill does
not fix the failure, complete the missing work, or run a different check that
would pass. This is the same boundary Decisions 0111 and 0112 draw for the other
read-only skills, and it is the reason a verdict from any of them means
anything.

### Boundary

- Verify one claim and return a verdict.
- Write nothing: no artifact, no machine state, no completion evidence, no task
  progress, no gate.
- Repair nothing and complete nothing.
- Never invoke `spec completion accept` or any other mutating command.

## Consequences

- The protocol's second consumer exists, so a claim can be checked at any point
  without entering the lifecycle.
- The skill's most likely failure — recording the completion it just confirmed —
  is named rather than left to inference from "not a workflow stage".
- The two verdict vocabularies stay distinct, so a supported claim is not
  mistaken for an authorization to advance.
- Being consequence-free is what makes it cheap to invoke, which is the property
  that gets it used before a false claim rather than after.

## Implementation status

Implemented.
`tools/specbind/assets/skills/specbind-verify-completion/SKILL.md` is embedded
and installed.

Its forward tests are specified as scenarios VC1 and VC2 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually.
