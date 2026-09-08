# 0205: Verification dimensions do not require invented commands

Status: Accepted

## Context

Whole-implementation validation requires full-suite evidence and a runtime
liveness assessment. A fresh forward-test driver interpreted those as requiring
distinct commands even though the fixture was a library, its canonical suite
loaded and exercised the public entry point, and no separate runtime command was
defined. It constructed an ad-hoc Python invocation to fill the perceived slot.

That behavior conflicts with the existing rule that required commands come from
project automation and conventions. It also turns a list of evidence questions
into a command-count requirement the contract never intended.

## Decision

Completion-verification dimensions are independent questions, not mandatory
one-command-per-dimension slots. One canonical project command may support more
than one dimension when its observed behavior genuinely proves each claim.

For a library-only artifact with no project-defined startup command, runtime
liveness means that the current artifact can be loaded and its public first
usable operation can execute. Existing canonical tests may prove that when the
validator inspects the relevant coverage and fresh output. The validator must
not invent an ad-hoc shell, language, or smoke command solely to create a
separate runtime entry.

If neither canonical evidence nor an applicable Validation adapter proves first
usable state, the result remains `MANUAL_VERIFY_REQUIRED`. A passing suite whose
contents do not exercise that boundary still does not prove runtime liveness.

This Decision clarifies Decision 0112 and the completion-verification protocol;
it does not weaken any required evidence dimension.

## Consequences

- Small libraries can be validated from their real project suite when it
  demonstrably loads and exercises the public API.
- Agents do not manufacture unowned commands to make a checklist look complete.
- Applications and services still need actual startup or equivalent liveness
  evidence, and missing evidence still fails closed.

## Verification

Skill contract tests require the multi-dimension rule, the library interpretation,
the no-ad-hoc-command boundary, and the fail-closed outcome. Fresh completion
forward testing confirms the driver uses only project-owned commands.
