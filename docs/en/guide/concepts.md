# Core concepts

SpecBind neither delegates everything to an Agent nor requires documents for
every change. It combines Agents that make semantic judgments with a CLI that
validates and records state, preserving the relationship between specification
and implementation over time.

## Skills and the CLI

| Owner | Main responsibilities |
| --- | --- |
| Agent Skills | Scope judgment, Requirements and Design authoring, review, implementation, and explanation |
| `specbind` CLI | Structural validation, traceability, approval evidence, progress, lifecycle transitions, and release preflight |
| You | Scope confirmation, required approvals, project-specific choices, and confirmation of published results |

Skills do not edit CLI-owned state directly. The CLI does not judge whether a
Requirement is correct or a Design is sound. Passing through both layers avoids
formally valid but meaningless artifacts and plausible prose that bypasses the
lifecycle.

## Spec

A Spec is one durable capability or responsibility boundary. It is not a
disposable change plan. Later Milestones update the same Spec's Requirements,
Design, and Contract as the complete current truth. Specs remain after release
and become the starting point for the next change.

The default path is `.specbind/specs/<spec>/`, where `<spec>` is a short
kebab-case responsibility ID.

## Milestone and Roadmap

A Milestone groups work delivered as one release. Its Roadmap records
Spec-backed items, Direct items, dependencies, and the target release. Only one
Milestone may be active in a project. The current Roadmap normally lives at
`.specbind/steering/roadmap.md`; after release, the CLI moves it to the release
archive.

Once work enters a Milestone, it remains tracked inside that release boundary
even if it could otherwise be a small standalone change.

## Spec-backed and Direct items

Discovery classifies work by ownership, not size.

| Type | When selected | Artifacts |
| --- | --- | --- |
| Existing-Spec update | Changes behavior or boundaries owned by an existing Spec | Updated Requirements, Design, Contract, and Tasks |
| New Spec | Adds a durable project responsibility | New Requirements, Design, Contract, and Tasks |
| Direct | Belongs to no Spec and changes no Requirements, Design, or Contract | Roadmap summary and completion state |

A large change may remain one existing-Spec update; a small change may require a
new Spec. If Direct implementation reveals a specification or Contract change,
stop and return to Discovery instead of adding artifacts ad hoc.

## Discovery source collections

Discovery can accept a tracked project file or directory as one Source
Collection. It inventories every text file, records each Source Item's Roadmap
destination or exclusion reason, and lists only a Spec's relevant items in its
Brief. An unreadable or untracked item stops the collection rather than allowing
partial coverage.

Source material is input, not authoritative specification. Requirements and
Design read the Brief-declared sources and promote accepted behavior and
technical conclusions into their own artifacts. Updating a source does not
automatically synchronize downstream artifacts; rerun Discovery explicitly for
the intended scope.

## Durable and Milestone-local artifacts

| Kind | Examples | Lifecycle |
| --- | --- | --- |
| Durable | `spec.yaml`, `requirements.md`, `design.md`, `contract.yaml`, `log.md` | Retained as current Spec truth and history |
| Milestone-local | `brief.md`, `research.md`, `tasks.yaml` | Used for the active change and cleaned up at release |
| Project-wide | `steering/roadmap.md`, Steering documents, Contract review | Holds cross-Spec scope and decisions |

Requirements and Design are complete current contracts, not change-only notes.
Statements that remain true stay in the current document.

## Gates and approval

Requirements, Design, and Tasks each have a Gate. Approval is evidence tied to
the reviewed input revision and fingerprint, not an unqualified checkbox.
Changing upstream artifacts therefore invalidates or stales affected downstream
approval and completion evidence.

Approval may be:

- **explicit** — you review and approve at that Gate; or
- **delegated** — you authorize a named run such as `specbind-plan` to approve
  specified Gates after their normal reviews and checks pass.

Delegation does not skip review and does not authorize invalidating an existing
Gate or accepting Contract review.

## Contract review

Design includes outward responsibilities, dependencies, and file ownership in
a structured Contract. Before Tasks are authored, all active-Milestone
Contracts are reviewed together, even when there is only one Spec. This exposes
ownership overlap, cycles, compatibility assumptions, and integration gaps
before implementation.

Read the resolved graph without changing source Contracts:

```sh
specbind contract graph
specbind contract dependencies <spec>
specbind contract consumers <spec>
```

## Invalidation and rewind

When implementation reveals an upstream problem, do not patch Requirements,
Design, Contract, or Tasks from the implementation workflow. Stop, diagnose the
owner, explicitly invalidate the affected Gate, revise through that phase's
Skill, and then rebuild downstream approvals.

```text
Implementation observation
  -> fresh diagnosis
  -> explicit Gate invalidation
  -> owning planning phase
  -> downstream review and approval
  -> implementation resumes
```

!!! warning "v1 limitation: removing Requirements"
    Removing an active Requirement may require abandoning and re-establishing
    the Spec rather than silently deleting its identity. Follow the CLI's
    reported route.

## Ordinary lifecycle

```text
Discovery
  -> Requirements
  -> Design and independent validation
  -> Milestone-wide Contract review
  -> Tasks
  -> implementation and per-Task review
  -> Spec completion validation
  -> release and finalization
```

`specbind-plan` is the default entry from Requirements through Tasks approval.
Use a named Spec or `--all`; an invocation without scope first asks which scope
you intend. An explicit request for one named Spec and one Requirements,
Design, or Tasks phase uses that phase's procedure from the same Plan Skill.
`specbind-implement` handles exactly one Roadmap item at a time.
`specbind-drive` selects safely reachable owning workflows across the Milestone
one at a time and rereads CLI state after every handoff. It parks branch-local
attention and continues independent work, but never executes Release.

## Project-owned configuration

Templates, Rules, and adapters below `.specbind/settings/`, plus Steering below
`.specbind/steering/`, are project-owned. Product-managed Skills, protocols,
schemas, and CLI state transitions are not. Use `specbind-configure` to route a
change to the correct owner and complete its verification and aftercare.

Codex and generic integrations share `.agents/skills/`; Claude Code uses
`.claude/skills/`. Agent-specific role files adapt planner, implementer,
reviewer, debugger, and researcher capabilities without changing the shared
Skill contract.

## Next

- [Getting Started](./getting-started.md)
- [Plan and implement one item at a time](./implement-step-by-step.md)
- [Plan and Drive a Milestone](./implement-with-plan-and-drive.md)
- [Customize SpecBind](./customization.md)
- [Release a milestone](./release.md)
