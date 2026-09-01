# 0174: Make planning phases references of the Plan Skill

Status: Accepted

Supersedes: the independent phase-Skill names and direct-selection contract in
[Decision 0161](./0161-default-plan-and-phase-skill-namespace.md). The phase
behavior accepted by Decisions 0100, 0104, and 0105 remains authoritative.

## Context

Decision 0161 made `specbind-plan` the ordinary planning entry point but kept
`specbind-plan-requirements`, `specbind-plan-design`, and
`specbind-plan-tasks` as independently installed lower-level Skills. Their
descriptions say that Plan normally dispatches them and that direct selection
exists only for an explicit request to work on one phase.

Decision 0096 now supports progressive Skill packages with directly routed
reference files. A detailed phase procedure no longer needs an independent
discovery identity merely so the owning orchestrator or an explicit phase run
can load it. Keeping all four names makes the product catalog expose internal
composition as four competing planning entry points.

## Decision

`specbind-plan` is the only installed planning Skill. Its package contains:

```text
specbind-plan/
|-- SKILL.md
`-- references/
    |-- requirements.md
    |-- design.md
    `-- tasks.md
```

The entrypoint selects one of two modes before artifact work:

- An ordinary request to plan, continue planning, finish planning, or reach an
  approved task plan uses the complete named-Spec or explicit all-Spec route.
- A request that explicitly names one Spec and exactly one of Requirements,
  Design, or Tasks uses single-phase mode, reads only that reference, and stops
  after that phase.

Lifecycle state never implies single-phase intent. A phase-only request that
omits the Spec or the phase stops for that selection. Selecting a phase does
not authorize its Gate unless the maintainer separately supplies the approval
required by the retained phase contract.

The three references retain their existing artifact ownership, semantic
checks, approval consequences, rewind boundaries, checkpoint duties, and
result contracts. `specbind-validate-design` and
`specbind-contract-review` remain independent Skills because their fresh
judgment and milestone-wide acceptance are not authoring subprocedures.

For the complete route, every phase remains a fresh dispatch. The dispatch
brief carries the exact installed path of the applicable reference in the
selected Plan package. The receiver reads that reference completely; the
orchestrator does not depend on a host being able to invoke another Skill and
does not inline a possibly stale copy of the procedure.

Installation no longer owns or writes directories named
`specbind-plan-requirements`, `specbind-plan-design`, or
`specbind-plan-tasks`. Refresh removes those former exact product-managed
targets through the existing install replacement plan. No compatibility aliases
or stubs are retained.

## Consequences

- The installed product catalog has 16 Skills and one unambiguous planning
  entry point.
- Explicit single-phase work remains available without exposing internal phase
  procedures to automatic Skill selection.
- Plan keeps a small routing and orchestration entrypoint while phase detail is
  loaded progressively.
- Phase behavioral tests apply to the three Plan references rather than to
  separately discoverable Skills.
- Consumer instructions and examples name `specbind-plan` plus explicit phase
  intent instead of the removed names.

## Verification

Mechanical tests verify the 16-Skill catalog, all three packaged references,
per-Agent installation and removal targets, the Plan mode-selection contract,
and the retained phase invariants. Focused forward tests cover both ordinary
planning and one explicit single-phase request from a fresh installed fixture.
