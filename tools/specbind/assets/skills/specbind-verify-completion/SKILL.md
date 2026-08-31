---
name: specbind-verify-completion
description: Consequence-free check of an explicit completion or success claim against fresh evidence. Changes nothing; do not use to advance a named Spec whose implementation may be accepted.
argument-hint: "<claim>"
---

# Verify one claim

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

Use this **before** saying a task is done, a defect is fixed, a command passed,
or an implementation is complete — including before trusting a subagent's report
that any of those is true.

You answer one question and change nothing.

This is not the Spec completion gate. When the user asks whether a named Spec's
completed implementation is done and a `GO` should record completion evidence,
use `specbind-validate-implementation` instead. Use this skill when the subject
is the claim itself and the result must remain consequence-free.

```sh
specbind protocol read completion-verification
```

## What you are judging

A **claim**, not a Spec. Something someone is about to assert. State it back
precisely in the narrowest form actually being made, because the most common
false completion is not a lie — it is evidence for a part being accepted for the
whole.

Then find what would prove *that* claim, and require evidence from the current
state of the code.

## Run the check yourself

Wherever you can reproduce it, run it. A report is a claim, not evidence, and
this skill exists mainly because "the subagent said it succeeded" is persuasive
in the moment.

When you are handed output you cannot reproduce — from another run, from an
earlier state, from the user — **say so**, and treat the claim as resting on
evidence you did not observe. That is usually `MANUAL_VERIFY_REQUIRED`, not
`VERIFIED`.

## Return the verdict

```text
## Verification
- VERDICT: VERIFIED | NOT_VERIFIED | MANUAL_VERIFY_REQUIRED
- CLAIM: <the claim, as you understood it>
- EVIDENCE: <what you ran or read, and what it returned>
- GAP: <where the claim exceeds the evidence, or none>
```

- **`VERIFIED`** — the evidence covers exactly this claim.
- **`NOT_VERIFIED`** — the check failed, the evidence is stale or partial, the
  claim is broader than what was shown, or work remains blocked or uncovered.
- **`MANUAL_VERIFY_REQUIRED`** — a mandatory check could not be performed here.
  Nothing is known to be wrong, and nothing is known to be right.

**The third is not a softer second, and never a route to the first.** Turning
"could not check" into "checked" is the failure this skill exists to prevent.

Never narrow a check, skip a case, or substitute a cheaper command to reach
`VERIFIED`.

## You are not a workflow stage

**Change nothing — including when the verdict is `VERIFIED`.**

Confirming that an implementation is complete puts you one step from recording
that completion, and the step looks like helpfulness. It is not. Completion
evidence is written by `specbind-validate-implementation` through a handshake
that rechecks things you never looked at: gate freshness, contract review
freshness, milestone convergence, and a clean revision.

`VERIFIED` means the claim is supported. It does not mean the Spec may advance.

## Boundaries

- Verify one claim. Return a verdict.
- Write nothing: no artifact, no machine state, no completion evidence, no task
  progress, no gate. Run no mutating command.
- Repair nothing and complete nothing. A refused claim comes back as a refusal
  with what is missing.
- Report in the project's language, with the block above intact.
