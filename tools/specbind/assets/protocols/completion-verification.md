# Completion verification protocol

This protocol is the shared baseline for allowing a completion claim. It applies
to every supported agent and cannot be waived by a project template or shared
rule.

It owns the relationship between a claim and its evidence. Which workflow stage
invokes it, what happens to a refusal, and any CLI transition that follows
belong to the calling skill.

The question is never "does this look finished". It is:

> What exactly is being claimed, and what evidence proves that exact claim?

## The gate

1. State the claim precisely, in the narrowest form that is actually being made.
2. Identify the command, output, or check that would prove **that** claim.
3. Require fresh evidence from the current state of the code. Not from earlier
   in the run, not from before the last change.
4. Read the evidence properly: exit status, failure count, what was skipped, and
   what was never covered.
5. Reject any claim broader than its evidence.
6. When mandatory verification cannot be performed at all, say so rather than
   substituting a weaker check.
7. Only then allow the claim.

Step 5 is where most false completions are produced. Evidence for a part is
routinely accepted as evidence for the whole, and the gap is invisible unless
the claim and the evidence are stated side by side.

## Evidence rules by claim

**A task is done.** Task-local verification actually ran, no blocking review
finding is outstanding, and the evidence covers the task's own boundary rather
than something adjacent.

**A defect is fixed.** The original symptom is demonstrably gone, and the
verification scope is wide enough to show the fix introduced nothing new.

**A command passed.** Actual output and exit status from this run. Not inferred
from a different command, and not remembered from an earlier one.
When the command becomes durable evidence, preserve the exact executed command
string. A descriptive label, shortened argument, placeholder, or equivalent
command does not identify the evidence that produced the observed result.

For a canonical project command, passing also requires a repeatable clean
invocation. Capture `git status --short` immediately before and after the exact
command, without cleanup between the command and the after-snapshot. New caches,
coverage data, reports, or other untracked output make the completion claim
`Not verified` even when the exit code is zero. Do not delete those outputs in
the validator to manufacture a clean result; report the exact paths so
implementation can make the command itself clean for future release and user
runs.

**A whole implementation is complete.** This is the strictest claim and needs
all of:

- the full test suite, run to completion, at the current state
- a runtime check that the built artifact reaches its first usable state
- an assessment that every active Requirement is genuinely delivered, not merely
  referenced
- an assessment that the parts integrate, rather than each working alone
- an assessment that the result matches the Design end to end
- the status of anything left blocked

**A passing test suite alone never establishes this claim.** Tests exercise what
someone thought to test; they do not show that the feature runs, that the
requirements are covered, or that the pieces fit.

## Refuse rather than weaken

There are two distinct ways a claim fails, and collapsing them loses
information:

- **Not verified.** The check failed, the evidence is stale or partial, the
  claim exceeds the evidence, or work remains blocked or uncovered. Something is
  wrong and it is known.
- **Cannot verify.** No canonical command is known, the required environment is
  unavailable, or a mandatory manual step cannot be performed here. Nothing is
  known to be wrong, and nothing is known to be right either.

The second is not a softer version of the first, and it is never a reason to
allow the claim. It is a request for a human to complete the verification.

Never narrow a check, delete an assertion, skip a case, or substitute a cheaper
command in order to reach a passing result. That converts an unverified claim
into a verified-looking one, which is worse than an honest refusal because
everything downstream will trust it.

## Rationalizations that are not evidence

| Claim | Reality |
| --- | --- |
| "The report said it succeeded" | A report is a claim, not evidence. |
| "It passed earlier" | Fresh evidence only. Earlier was a different state. |
| "Lint passed, so the build is fine" | Lint does not compile. |
| "Tests and build passed, so it runs" | Startup, configuration, module loading, and native compatibility all fail after both. |
| "Every task is checked off" | Completed tasks say work happened, not that the whole is coherent or covered. |
| "The failure is unrelated" | Establish that. An unexplained failure is not a passing result. |
| "The canonical script is missing, but I can run its test runner directly" | That is a substitute command, not fresh evidence from the required check. |

Each of these is a way of accepting evidence for something adjacent to the
claim. That is the same error as step 5, arriving in a form that sounds like
diligence.
