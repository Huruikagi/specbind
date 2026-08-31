# Establish Specs from an existing implementation

`specbind-adopt-existing` is an initial-adoption workflow that investigates
working code and tests and organizes the evidence into candidate SpecBind
Specs. It is for an existing project without a trusted specification, not for
migration from another SDD product such as cc-sdd.

The implementation is evidence, not the intended specification. Observed
behavior may be a requirement, bug, historical constraint, implementation
detail, or an unresolved question. Only behavior confirmed by the user is
promoted through the Brief into Requirements.

## Prerequisites

- No durable Specs exist yet and no Milestone is active.
- Steering describes product, technology, and structural direction.
- The repository, including Steering, is committed and the worktree is clean.
- The adoption target is explicitly the whole repository or a concrete area.

If Steering is absent, run `specbind-steering` in bootstrap mode first, review
the proposal, and commit it.

## Standard route

```text
Bootstrap or synchronize Steering
  -> specbind-adopt-existing
  -> confirm candidate Spec boundaries
  -> specbind-discovery
  -> create Specs and Briefs
  -> resume specbind-adopt-existing
  -> confirm observations and intent per Spec
  -> specbind-plan-requirements
  -> specbind-plan-design
  -> specbind-plan-tasks
```

The first run checks adoption prerequisites with:

```sh
specbind adoption preflight
```

The returned `source_revision` pins the Git commit used as investigation
evidence. If implementation, tests, dependencies, configuration, or Steering
change during the investigation, the workflow stops instead of silently
following the new state.

## Investigation depth and Spec boundaries

The workflow first scans the whole repository shallowly to identify public
APIs, entry points, module boundaries, tests, and dependencies. It then
investigates only the requested adoption area deeply. Specs are divided by
durable responsibility, not directory size or estimated task count.

No Spec boundary is created before user confirmation. After confirmation,
ordinary `specbind-discovery` presents the Roadmap scope again and owns the
CLI-managed Milestone and Spec changes.

## Observations and intent

Observations are temporarily recorded in
`.specbind/specs/adoption/reverse-discovery.yaml` by default. Each one points
to evidence at the pinned revision using a path and a symbol, test name, route,
schema entry, or similar locator.

| Disposition | Meaning |
| --- | --- |
| `requirement` | Intended behavior promoted through Brief to Requirements |
| `design` | Technical or structural constraint promoted through Research to Design |
| `bug` | Existing behavior that should not become the specification |
| `historical_constraint` | Temporarily preserved but not a product promise |
| `implementation_detail` | Internal detail that is not specified |
| `unknown` | Open question for Requirements or Design |

After every Spec has a complete Brief and Research handoff, the project-wide
dossier is removed from the current tree. Git retains the investigation
history, and each Spec's Research remains until normal release finalization.

## Return to the ordinary lifecycle

Adoption does not author or approve Requirements or Design. It stops after
handing confirmed intent to the Brief and implementation evidence and design
constraints to Research. Continue with the ordinary Requirements, Design, and
Tasks planning Skills; there are no reverse-specific authoring Skills.

## Next

- [Core concepts](./concepts.md)
- [Start with an existing project](./start-existing-project.md)
- [Customize SpecBind](./customization.md)
- [Current generated skill index](../reference/current-skill-index.md)
- [Current generated artifact index](../reference/current-artifact-index.md)
