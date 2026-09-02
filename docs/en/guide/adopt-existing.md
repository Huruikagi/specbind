# Establish Specs from an existing implementation

The explicit reverse mode of `sb-discovery` establishes durable Specs for the
product a fixed existing revision already represents. It is for a project with
working code but no trusted specification, not migration from another SDD
product and not delivery of a new change.

Implementation is evidence, not specification authority. An observed behavior
may be maintained intent, a structural constraint, a historical accident, an
internal detail, a suspected defect, or a question that needs your decision.

## Prerequisites

- No durable Specs exist and no Milestone is active.
- Steering covers product purpose, technology constraints, and structure.
- The repository, including Steering, is committed and clean.
- You name the whole repository or a concrete area.
- You provide the existing product version represented by that revision.

If Steering is incomplete, use `sb-configure` and `sb-steering` first, then
commit the result. Configuration and reverse establishment are separate runs.

## One confirmed, continuous route

```text
Configure and commit Steering
  -> sb-discovery with selected area and existing version
  -> fix source_revision
  -> inspect code and tests
  -> confirm one complete reverse proposal
  -> create reverse Roadmap, Specs, Briefs, and Research
  -> Requirements
  -> Design and independent Design validation
  -> Contract Review
  -> adoption finalize
```

The proposal includes `baseline_version`, the candidate `reverseSpecs`, their
maintained intent and evidence, dependencies, blocking and deferred unknowns,
suspected defects, and excluded areas. Nothing is created before you confirm
that complete proposal. After confirmation, the run continues without routine
phase pauses.

The Roadmap uses `reverseSpecs`, not `newSpecs` or `specUpdates`, and has no
`target_release`. Every created Spec retains this provenance:

```yaml
establishment:
  kind: reverse
  source_revision: <fixed Git revision>
  baseline_version: <existing product version>
  milestone_id: <reverse milestone>
```

## Fixed-revision and unknown rules

`specbind adoption preflight` returns the `source_revision`. Implementation,
tests, dependencies, configuration, and Steering must not change until the run
finishes. Reverse scope cannot be updated or rebaselined. Source drift stops the
run and requires restarting from the new clean revision.

A question blocks its Spec when an answer is needed to state meaningful
maintained behavior. Other independent Specs can continue, but Contract Review
and finalization wait. A question may be deferred only when every later answer
would leave current Spec meaning unchanged.

An active Deferred Findings Adapter may record behavior that looks defective as
a suspected defect with the source revision, evidence locator, and claim. It is
not automatically a bug or requirement, and reverse establishment does not fix
it. Sending anything outside the project still needs separate authority.

## No Tasks and no release

Reverse reuses the ordinary Requirements, Design, Design validation, and
Contract Review owners. Design approval moves a reverse Spec to
`adoption_ready`. No `tasks.yaml` is authored, no implementation or validation
work begins, and no Release Adapter, tag, publication, or `target_release` is
created.

If an ordinary change request arrives, finish reverse first and create a new
ordinary Milestone afterward. An emergency requires explicit abandonment with
`specbind milestone reverse abandon --milestone-id <id>`; the urgent change
then uses ordinary Discovery, and reverse restarts later from the new revision.
Do not manually delete lifecycle state.

## Finalization and history

When every Spec is `adoption_ready` and Contract Review is fresh, Discovery
runs:

```sh
specbind milestone reverse finalize --log-entries <path-or->
```

Finalization clears active change state while retaining establishment
provenance, removes temporary Brief and Research evidence, and writes a
`Baseline <version>` entry to each Spec `log.md`. It archives the Roadmap and
Contract Review under `baselines/` and closes the active milestone. These are
adoption records, not product-release records.

## Next

- [Core concepts](./concepts.md)
- [Start with an existing project](./start-existing-project.md)
- [Customize SpecBind](./customization.md)
- [Current generated skill index](../reference/current-skill-index.md)

---

[Getting started](./getting-started.md) | [Start with an existing project](./start-existing-project.md)
