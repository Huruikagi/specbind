# 0161: Make plan the default and namespace its phase Skills

Status: Accepted

Supersedes: [Decision 0153](./0153-unified-quick-plan-orchestrator.md) and the
installed-name clauses of [Decision 0100](./0100-requirements-skill-contract.md),
[Decision 0104](./0104-design-skill-contract.md), and
[Decision 0105](./0105-tasks-skill-contract.md)

## Context

Decision 0153 unified named-Spec and all-Spec planning behind
`specbind-quick-plan`. Its workflow no longer skips an artifact, validation,
review, gate, or CLI guard. `quick` describes only the option to delegate three
gate approvals, while the same orchestrator also supports explicit approval at
each gate.

That makes the qualifier misleading. A maintainer asking to plan ordinary active
work should not have to choose between a qualified orchestrator and three phase
Skills presented at the same level. Discovery can route a general planning
request more reliably when there is one unqualified default, while maintainers
still need precise entry points for revising one phase.

Design validation remains an independently invocable second opinion, and
Contract Review is a milestone-global barrier rather than a Spec-local authoring
phase. Neither belongs under a three-phase per-Spec naming hierarchy.

SpecBind has not reached its first stable release. It does not need compatibility
aliases for the current pre-release names.

## Decision

### Plan is the default planning entry point

V1 installs `specbind-plan` as the ordinary route for a request to plan active
work. It owns the complete orchestration from Requirements through Tasks
approval and retains Decision 0153's two explicit scope modes:

- **named scope**: one named or targeted Spec-backed Roadmap item;
- **all scope**: every Spec-backed participant in the active milestone, selected
  by `--all` or equally explicit all-Spec intent.

A bare planning request still reads only milestone status, presents named and
all-Spec choices, and stops before dispatch, authoring, or approval. The
milestone scheduler, fresh phase dispatch, mandatory Design validation, global
Contract Review, clean checkpoint handoffs, retry classification, and
Tasks-approval stopping point are unchanged.

Delegated approval is an authorization mode, not a separate fast workflow.
`specbind-plan` offers one bounded delegation confirmation for the Requirements,
Design, and Tasks gates. If delegation is declined, the same orchestration
continues with explicit approval at each gate. Delegated gate evidence records
the durable workflow identity `specbind-plan`.

### Phase Skills are explicit lower-level entries

The three per-Spec authoring Skills are installed under the plan namespace:

| Phase | Installed Skill |
| --- | --- |
| Requirements | `specbind-plan-requirements` |
| Design and Contract | `specbind-plan-design` |
| Tasks | `specbind-plan-tasks` |

`specbind-plan` dispatches these Skills as its owned phases. Discovery and
installed project instructions select them directly only when the maintainer
explicitly asks to author, revise, or resume that individual phase. A general
request to plan, continue planning, or take work to an approved plan selects
`specbind-plan`, not the phase inferred from current status.

The behavioral contracts accepted by Decisions 0100, 0104, and 0105 remain
authoritative under the new installed identifiers. Artifact ownership, approval
authority, rewind boundaries, and checkpoints do not change.

`specbind-validate-design` and `specbind-contract-review` keep their independent
names and responsibilities.

### No compatibility surface

`specbind-quick-plan`, `specbind-requirements`, `specbind-design`, and
`specbind-tasks` are removed from the embedded and installed Skill catalog. No
alias, forwarding Skill, deprecation stub, or compatibility diagnostic is
installed. Tests assert that the removed identifiers do not resolve.

## Consequences

- One unqualified Skill is the predictable entry point for ordinary planning.
- The `plan-*` prefix makes individual phase Skills discoverable as deliberate
  lower-level operations rather than competing default workflows.
- Gate delegation remains explicit and visible instead of becoming an implicit
  consequence of making Plan the default.
- Design validation remains independently invocable, and Contract Review keeps
  its milestone-global identity.
- Delegated gate evidence changes to the current installed workflow identity
  without a pre-release alias.

## Implementation status

Implemented. The embedded Skill packages, registry, installed project
instructions, tests, generated indexes, maintained documentation, and forward
test contracts use the default `specbind-plan` hierarchy.
