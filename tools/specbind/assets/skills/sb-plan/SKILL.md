---
name: sb-plan
description: Plan one named Spec or all active Specs through Tasks approval, run an explicitly requested planning phase, or prepare scoped project shared Contracts.
argument-hint: "[<spec> | --all | --shared] [requirements|design|tasks]"
---

# Plan active Spec work

## First action: fail closed on an unspecified scope

An explicit request to plan the milestone's shared resources, `--shared`
scope, or a Drive handler with mode `shared` selects
[Shared Contract preparation](references/shared-contract.md); after applying
project language style, read that procedure completely. It is a separate
scope with no dedicated Spec and is exempt from the named/all-Spec choice below.

Classify the maintainer's request **before reading phase procedures, Spec
artifacts, templates, protocols, Steering, or implementation**. For a request
such as "plan the active work" that names neither one Spec nor all Specs, the
only workflow reads permitted are the language-style Rule below and
`specbind milestone status`. Then name every available Spec choice, also name
the all-Spec choice, ask the maintainer to select one, and stop.

An `Actionable` item, a single participating Spec, or an obvious next phase is
not scope authorization. Do not run `specbind spec status`, select a phase
reference, investigate missing artifact details, or dispatch a phase until the
maintainer chooses named or all-Spec scope. This first-action guard takes
precedence over every later routing and scheduling instruction.

## Apply project language style

Before authoring orchestration or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

Select this Skill for every planning request. It owns both the complete route
from Requirements through Tasks approval and an explicitly requested single
Requirements, Design, or Tasks phase. The phase procedures are references in
this package, not separately selectable Skills.

## Select the planning mode

A Drive recovery dispatch carrying resolved replan authority (from the request
or project defaults with an actual Drive execution request) and named
affected Specs is the recovery mode. Read [Replan during
Drive](references/replan.md) first, then [Complete planning
route](references/complete-route.md). The recovery procedure owns the supplied
scope and authority where it overrides the ordinary complete-route rules. Do
not repeat a scope or delegation question already settled by that dispatch.
Preserve both resolved settings and provenance; do not reread Drive defaults
or renew Task recovery counts during this handoff.

Use **single-phase mode** only when the maintainer explicitly asks to author,
revise, resume, or rerun exactly one of Requirements, Design, or Tasks for one
named Spec. The request must identify both the Spec and the phase. A request to
plan, continue planning, finish planning, or take work to an approved plan is
the **complete route**, even when the CLI currently reports one phase as
actionable. Never infer single-phase mode from lifecycle state.

If a single-phase request omits the Spec or names more than one phase, read
milestone status only to present the exact missing choices, then stop for the
maintainer's selection. Single-phase mode never expands to another Spec or
silently continues to a later phase.

For a valid single-phase request, also run `specbind spec status <spec>` and
reject a Direct item. Then read exactly one procedure completely:

- Requirements: [Requirements phase](references/requirements.md)
- Design and Contract: [Design phase](references/design.md)
- Tasks: [Tasks phase](references/tasks.md)

Follow that procedure with the authority the maintainer supplied and stop after
its phase result. The selected procedure owns its artifact inputs, gate,
checkpoint, rewind boundaries, and response. Do not apply the complete-route
scope question, delegation bundle, scheduler, Design validation sequence, or
Contract Review unless the selected procedure itself routes to one of those
independent owners. An explicit phase request is not gate approval unless it
also explicitly authorizes that gate after the procedure's stated consequences.

For an ordinary complete route, read [Complete planning
route](references/complete-route.md) completely. It owns scope selection,
delegation, scheduling, phase dispatch, validation, the global Contract Review
barrier, checkpoints, stopping conditions, and the final report.

## Own internal dispatch through its stopping point

Keep the owning turn active until every started internal role has returned and
the selected planning workflow has reached its stopping point. This applies to
complete planning, a single phase, shared preparation, and Drive recovery.
Prefer foreground dispatch; if the host runs a role in the background, wait for
its terminal result in this turn and resume the selected procedure. A progress
message such as "the Tasks-phase receiver is still running" is not a final
response. Do not ask the caller or user to send "continue" merely to collect a
running role's result.

A required confirmation or input, an exhausted retry bound, an explicit user
stop, or a real host/capacity failure remains a stopping point. Report the
reason, exact partial paths, and any receiver still running; preserve any
required restart capsule. Waiting for internal work grants no additional gate,
rewind, implementation, or checkpoint authority.

## Boundaries

- The entrypoint selects one mode and loads only its applicable procedures.
- Never infer all-Spec scope, a gate approval, or rewind authority from a request
  that did not supply it.
- Never implement, validate completion, or release from this Skill.
