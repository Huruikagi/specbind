---
name: sb-plan
description: Plan active Spec work through Requirements, Design, Contract Review, and Tasks, or run one explicitly requested planning phase for one named Spec. Ordinary planning uses one named Spec or explicit all-Spec scope.
argument-hint: "[<spec> | --all] [requirements|design|tasks]"
---

# Plan active Spec work

## First action: fail closed on an unspecified scope

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

A Drive recovery dispatch carrying explicit `--replan` authority and named
affected Specs is the recovery mode. Read [Replan during
Drive](references/replan.md) first, then [Complete planning
route](references/complete-route.md). The recovery procedure owns the supplied
scope and authority where it overrides the ordinary complete-route rules. Do
not repeat a scope or delegation question already settled by that dispatch.

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

## Boundaries

- The entrypoint selects one mode and loads only its applicable procedures.
- Never infer all-Spec scope, a gate approval, or rewind authority from a request
  that did not supply it.
- Never implement, validate completion, or release from this Skill.
