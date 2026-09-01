---
name: sb-validate-design
description: Independently judge whether a Spec's design is ready to be built on — requirement coverage, boundary, buildability, self-containment, and architectural fit.
argument-hint: "<spec>"
---

# Validate one Spec's design

An independent verdict on whether this design can be built on.

You read and judge. You author nothing, repair nothing, and rewind nothing.

**Read-only stop rule — before any command:** whatever you find, do not run a
gate invalidation command. A `NOT_READY` verdict is information for the user;
it is not authority to change lifecycle state. In particular, invalidating the
design gate also deletes the milestone's accepted contract review. Report the
finding and stop with every gate and review record exactly as you found them.

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

This is not a gate — the design phase does not wait for you. It is available
before approval as a second opinion, and after approval when someone wants the
design checked by something that did not write it.

## 1. Clear the structural checks first

```sh
specbind spec status <spec>
specbind check traceability <spec>
specbind check contracts
```

These are cheap, and a structural failure makes semantic review premature.

**Fix the review scope from the status output before reading prose.** The
`Requirement coverage: design N/N` count is over the active Requirement IDs for
this milestone. The Requirements document is a complete persistent contract and
may retain other IDs that this milestone does not deliver. Read those retained
Requirements for context, but do not report the Design incomplete, expand its
scope, or raise a finding merely because it does not realize an inactive ID.
Validate only the active set represented by the Design traceability markers and
the status coverage count.

**They are not your review.** The CLI already verifies traceability markers,
active requirement coverage, and contract structure — repeating them back is not
a finding. It matters here more than anywhere: a complete set of traceability
markers is exactly what makes an unrealized requirement look covered.

## 2. Read

```sh
specbind artifact list <spec>
specbind artifact read <spec> requirements --for consume
specbind artifact read <spec> design/main --for consume
specbind artifact read <spec> contract --for consume
```

Read every design artifact when the set is split. Read the research if the Spec
has one — you need to know what the design might be leaning on.

Then the standard:

```sh
specbind protocol read design-validation
specbind template list spec
```

Read the project's design and seam preferences through its rule surface:

```text
specbind rule read design-principles --for consume
specbind rule read contract-principles --for consume
specbind rule read design-template-selection --for consume
```

`NO_CHANGE RULE_ABSENT` for design or contract principles means that
customization is absent; the protocol still applies. Design-template selection
is required. Any `ERROR` line stops this validation. Confirm that the current
Design set contains every required template and every conditional template
whose responsibility applies to the current Requirements and repository. A
missing applicable Design is `NOT_READY`; do not create it.

When judging whether the design fits the system it enters requires real
investigation of the existing code, dispatch that as a fresh subagent with a
self-contained brief and have it return findings rather than file dumps.
Use the registered `specbind-researcher` role when available, with an ordinary
fresh subagent as the fallback.
Fallback is only for an absent role. A configured role whose model cannot start
is a configuration or environment failure, not permission to change models.
Everything else is a reading judgment — the criteria interlock, and splitting
them loses the picture.

Existing code is architectural context, not implementation evidence. This
validation normally runs before implementation, so code that does not yet
realize the proposed behavior is expected and is never by itself a finding.
Judge whether the design can realize the Requirements in that codebase; do not
judge whether the code already does.

## 3. Apply the deletion test

For anything the design points at rather than states — research, notes, a
ticket, the source itself — **remove it mentally and read the design again.** If
a requirement, constraint, interface, or rationale is now missing or ambiguous,
the design was depending on it.

**Research is the case that matters.** It is excluded from every gate
fingerprint and deleted at release, so a design that leans on it becomes
incomplete the moment the milestone closes — and nothing mechanical will ever
report that loss.

Code is the other one. Code says what the system does now; the design says what
it must do.

## 4. Return the verdict

```text
## Design validation
- VERDICT: READY | NOT_READY
- FINDINGS:
  - [BLOCKING|DEFERRED|RESOLVED] <requirement or boundary at risk> — <where in the design> — <consequence>
```

Every finding carries a disposition. A finding with none is one nobody carries
past this report.

**There is no "cannot judge" verdict, and that is deliberate.** If you cannot
judge readiness from the design as written, that is the finding — the design
does not yet stand on its own — and the verdict is `NOT_READY`. A review of an
implementation can be blocked by a missing environment; a design's inputs are
always present, so inconclusiveness is a property of the document.

`READY` asserts all of: every active requirement substantively realized, the
owned boundary explicit and inspectable, the work decomposable into bounded
tasks, the document carrying its own meaning, and proportionate complexity that
fits the architecture.

Every finding names what it endangers, points at where, and states the
consequence. "Section X is vague" cannot be acted on; "Section X does not
determine which component owns retry, so tasks cannot be bounded" can.

Rank by what would change the verdict, and say what the design does well when it
is true.

## 5. Record deferred findings

A deferred finding needs the destination this project names, or it is not
deferred — it is dropped, and the next review raises its successor as blocking
to keep that from happening again.

```sh
specbind adapter list
specbind adapter read deferred
```

The listing must report `state=active` for `deferred` before you follow it.
`state=absent` or `NO_CHANGE ADAPTER_ABSENT` means the project has no
destination. Say so in one line and record nothing. Do not invent a place to put
it. Write only what an active adapter says to write. Read the destination only
far enough to avoid recording the same finding twice; nothing in it is a source
of work for you, and no entry there becomes work until a person puts it on the
Roadmap.

This adapter write happens only after the verdict. Report the exact
project-relative destination and whether you changed it so an orchestrator can
keep that phase-owned path inside the bounded unapproved-Design handoff. Do not
write any other path.

## Boundaries

- **Never edit the design or the contract.** A validator that fixes what it
  found is judging its own work.
- **Never invalidate the design gate**, whatever the verdict. That rewind
  belongs to `sb-plan` in explicit Design-phase mode, which states its cost first — it also deletes
  the milestone's accepted contract review — and requires confirmation. Your
  verdict is information; acting on it is someone else's decision.
- Approve nothing, and record no machine state.
- Do not author or revise research. That belongs to `sb-gap-analysis`.
- Report in the project's language, with the block above intact.
