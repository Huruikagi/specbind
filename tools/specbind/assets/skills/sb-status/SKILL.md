---
name: sb-status
description: Report where a Spec or active milestone is in its lifecycle and what can happen next. Read-only; do not use to judge whether completed implementation is actually done.
argument-hint: "[spec]"
---

# Report current SpecBind state

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

Answer "where is this work now, and what is the next thing that can happen?"
using the CLI's derived read models. Change nothing.

This is not the answer to "is this completed implementation actually done?"
When every task is complete and the user asks whether a named Spec is done, use
`sb-validate-implementation`; that judgment may record completion evidence
on `GO`.

## Choose the scope

- A named Spec, when the user asks about one, or when only one Spec is under
  discussion.
- The active milestone otherwise, including when the user asks a general
  question such as what to do next.

When the request is ambiguous, report the milestone first: it names the
participating Specs, so the user can narrow from there.

## Follow the selected procedure

- For the active milestone, read [references/milestone.md](references/milestone.md)
  and follow it completely. Do not also load the Spec procedure unless the user
  narrows the request to one Spec.
- For one named Spec, read [references/spec.md](references/spec.md) and follow it
  completely. Do not start from the milestone procedure.

A read that fails is part of the answer. Report the diagnostic rather than
retrying it or working around it, because a command that cannot produce a
trustworthy read is telling the user their project state needs repair.

## Report

Lead with the answer, then the evidence. A user asking for status wants to know
where the work stands, not to read a transcript of commands.

Treat the CLI projection as input to judgment, not as a report template. First
interpret the complete projection and decide what changes the user's
understanding or next action. Then apply a user-facing filter:

- report the current stage or state, material progress, actual blockers or
  inconsistencies, and the next condition or action;
- summarize completed or healthy portions compactly when they help locate the
  remaining work;
- omit normal evidence that does not change the handoff, including fresh Gates
  or reviews, zero-valued blocker categories, and healthy items listed only to
  mirror the CLI; and
- include stable diagnostic codes or raw lifecycle identifiers only when the
  user asks for machine detail or the identifier materially helps remediation.

Filtering may compress facts but may not strengthen them into a remedy. For a
blocked Task, state the recorded condition that must change. Do not choose
among supplying an input, changing implementation, reordering or rewriting the
plan, or changing scope unless the CLI projection or recorded blocker names
that action. If more than one resolution could satisfy the condition, leave the
choice with the owning workflow or user.

Cover, in the project's language:

- **Where the work is.** The milestone stage or the Spec's lifecycle state, in
  plain terms rather than as an internal identifier.
- **What is not consistent**, if anything. Explain what each diagnostic means
  for the user's work rather than repeating the stable code alone.
- **What can happen next.** The CLI reports actionable items and release
  blockers; turn those into the concrete next step and say who has to take it.

`State health: consistent` means the CLI found no deterministic schema,
lifecycle, freshness, declared-coverage, or other machine-checkable diagnostic.
`Semantic alignment: not evaluated` is equally authoritative: status does not
judge whether Requirements, Design, Contract, Steering, and implementation
prose agree. Never use state health to rule out an artifact contradiction in a
review or diagnosis. These are interpretation boundaries, not routine
user-facing disclaimers. Mention consistent state health or the semantic limit
only when the user asks about consistency or correctness, when status is being
used as evidence for such a claim, or when omitting the distinction would make
the answer misleading. Always report inconsistent health and its material
diagnostics.

When named-Spec status reports `Task plan authority: not current; reconcile in
Tasks phase`, explain that the readable plan is not current implementation
authority. It may be preserved recovery input or an unapproved draft; the CLI
does not claim which. Name the Tasks phase as the owner of reconciliation, but
do not choose retain, revise-and-reset, or remove. The authority projection does
not cancel `inconsistent` health or explain away any unrelated diagnostic.

Keep it proportionate. A healthy Spec needs a few lines. A milestone with
blocked items must retain the recorded progress, Task identity, reason, and the
condition that must be resolved before its owning workflow can resume.

## Boundaries

- This skill only reads. It never approves, records progress, edits an artifact,
  or repairs state, even when the fix looks obvious. Offer the next step and let
  the owning skill perform it.
- Report what the CLI derived. Do not infer lifecycle state by reading
  `spec.yaml`, `tasks.yaml`, or the roadmap directly; those projections exist so
  every agent answers the same question the same way.
- Do not present a stale or inconsistent state as healthy. If the report is
  confusing because the project is genuinely inconsistent, say that plainly.
