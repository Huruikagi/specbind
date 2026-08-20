---
name: specbind-status
description: Report where a Spec or active milestone is in its lifecycle and what can happen next. Read-only; do not use to judge whether completed implementation is actually done.
argument-hint: "[spec]"
---

# Report current SpecBind state

Answer "where is this work now, and what is the next thing that can happen?"
using the CLI's derived read models. Change nothing.

## Choose the scope

- A named Spec, when the user asks about one, or when only one Spec is under
  discussion.
- The active milestone otherwise, including when the user asks a general
  question such as what to do next.

When the request is ambiguous, report the milestone first: it names the
participating Specs, so the user can narrow from there.

## Gather

For the active milestone:

```sh
specbind milestone status
```

For one Spec:

```sh
specbind spec status <spec>
```

Add these only when the answer needs them:

- `specbind tasks list <spec>` when the Spec is implementing and the user needs
  to see individual tasks, their progress, or which are blocked.
- `specbind milestone review status` when the milestone report shows a
  contract review that is absent, stale, or invalid, and the user needs to
  know what to do about it.
- `specbind check traceability <spec>` or `specbind check contracts` when a
  reported inconsistency needs to be attributed to a specific artifact.

A read that fails is part of the answer. Report the diagnostic rather than
retrying it or working around it, because a command that cannot produce a
trustworthy read is telling the user their project state needs repair.

`NO_CHANGE NO_ACTIVE_MILESTONE` is not a failure. It means no milestone is
active, which is the correct answer to "what is happening right now".

## Report

Lead with the answer, then the evidence. A user asking for status wants to know
where the work stands, not to read a transcript of commands.

Cover, in the project's language:

- **Where the work is.** The milestone stage or the Spec's lifecycle state, in
  plain terms rather than as an internal identifier.
- **What is not consistent**, if anything. Explain what each diagnostic means
  for the user's work rather than repeating the stable code alone.
- **What can happen next.** The CLI reports actionable items and release
  blockers; turn those into the concrete next step and say who has to take it.

Keep it proportionate. A healthy Spec needs a few lines. A milestone with
several blocked items needs the detail that explains the blockage.

## Boundaries

- This skill only reads. It never approves, records progress, edits an artifact,
  or repairs state, even when the fix looks obvious. Offer the next step and let
  the owning skill perform it.
- Report what the CLI derived. Do not infer lifecycle state by reading
  `spec.yaml`, `tasks.yaml`, or the roadmap directly; those projections exist so
  every agent answers the same question the same way.
- Do not present a stale or inconsistent state as healthy. If the report is
  confusing because the project is genuinely inconsistent, say that plainly.
