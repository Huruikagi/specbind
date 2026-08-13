# Target workflows

This document defines the intended user journeys and responsibility boundaries for the future SpecBind skill system. It stays name-neutral where naming is not yet decided; concrete names belong in the [target skill catalog](./target-skill-catalog.md).

Status: Draft

## Design goals

- Give users one clear entry point when they do not yet know the right workflow.
- Keep discovery, specification, implementation, and verification responsibilities distinguishable.
- Make every persistent write and approval boundary visible.
- Support both a deliberate phase-by-phase path and an intentional accelerated path.
- Keep equivalent Claude Code and Codex workflows aligned without hiding platform-specific capabilities.

## New work

```text
User request
  -> discovery
       -> direct implementation candidate
       -> update an existing spec
       -> create one new spec
            -> requirements
            -> design
            -> tasks
            -> implementation
            -> integration validation
       -> create several new specs
            -> roadmap and boundaries
            -> per-spec requirements, design, and tasks
            -> implementation by dependency order
            -> cross-spec validation
```

Open questions:

- Whether direct implementation remains only a recommendation or gets a dedicated skill.
- Whether multi-spec decomposition belongs to discovery or a separate planning skill.
- Whether cross-spec validation is part of batch orchestration or an explicit final skill.

## Existing-system work

```text
Existing repository
  -> project guidance bootstrap or sync
  -> discovery
       -> direct change
       -> update an existing spec
       -> create a new spec
            -> optional codebase gap analysis
            -> requirements and design
            -> tasks and implementation
            -> integration validation
```

The target contract should state when project guidance and gap analysis are optional, recommended, or required. They should not become implicit prerequisites merely because they exist as skills.

## Responsibility boundaries

| Stage | Owns | Does not own |
| --- | --- | --- |
| Discovery | Routing, scope clarification, decomposition, durable handoff context | Detailed requirements or technical design |
| Requirements | User-visible behavior, constraints, acceptance criteria | Architecture and implementation sequencing |
| Design | Architecture, interfaces, data flow, file boundaries | Task execution or unapproved scope changes |
| Tasks | Executable decomposition, dependencies, verification expectations | Implementation |
| Implementation | Code and tests for approved tasks, progress recording | Silent changes to approved requirements or design |
| Review | Independent task-level conformance review | Feature-level integration acceptance |
| Integration validation | Cross-task behavior, full verification, spec coverage | Replacing missing task-level review |
| Completion verification | Evidence for a specific success claim | Broad design or implementation work |

## Approval and automation model

The future workflow needs an explicit answer for each transition:

| Transition | Current target question |
| --- | --- |
| Discovery -> spec work | Is the selected route confirmed by the user? |
| Requirements -> design | Are requirements accepted, and what counts as acceptance? |
| Design -> tasks | Has technical design passed its review gate? |
| Tasks -> implementation | Are task boundaries and dependencies approved? |
| Implementation -> completion | Which reviews and fresh verification evidence are required? |

Accelerated and batch workflows may automate transitions, but they should reuse the same phase contracts rather than define competing document formats or success criteria.

## Topics to resolve next

1. Accept the skill naming convention and invocation syntax.
2. Review discovery's outputs and routing responsibilities.
3. Decide whether quick and batch remain first-class skills.
4. Review the separation among task review, integration validation, and completion verification.
5. Add a target artifact catalog when proposed file-path or format changes begin to accumulate.
