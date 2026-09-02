# Establish Specs from an existing implementation

The explicit reverse mode of `sb-discovery` establishes durable Specs for the
product a fixed existing revision already represents. It is for a project with
working code but no trusted specification, not migration from another SDD
product and not delivery of a new change.

Implementation is evidence, not specification authority. An observed behavior
may be maintained intent, a structural constraint, a historical accident, an
internal detail, a suspected defect, or a question that needs your decision.

This route produces a non-release baseline: accepted Requirements, Design, and
Contract Review. It creates no Tasks, implementation change, or product release.
If you only want to use SpecBind for your next change, follow the ordinary steps
in [Start with an existing project](./start-existing-project.md) instead.

## Prerequisites

- SpecBind is [installed](./install.md) in the project.
- No durable Specs exist and no Milestone is active.
- Steering covers product purpose, technology constraints, and structure.
- The repository, including Steering, is committed and clean.
- You name the whole repository or a concrete area.
- You provide the existing product version represented by that revision.

## The whole route

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

You confirm once, at the reverse proposal. Configuration and reverse
establishment are separate runs, so shape Steering and the shared surfaces
first, then run Discovery once.

## 1. Shape Steering

If Steering is incomplete, start with an initial `sb-configure` review:

```text
$sb-configure Review this project's initial configuration for adopting the
existing implementation. Start with the Steering it needs.
```

`sb-configure` first reads the mechanical configuration summary. When durable
guidance is needed, it routes Steering bootstrap or synchronization to
`sb-steering`. Review and commit the resulting Steering before adoption;
Discovery pins that revision as its evidence.

## 2. Run focused configuration reviews

Once Steering is established, ask `sb-configure` again to compare it and the
repository with the Requirements and Design templates and shared Rules:

```text
$sb-configure Use the confirmed Steering and repository facts to review the
Requirements and Design templates and shared Rules for this project.
```

Make a separate follow-up request for each remaining surface—templates, Rules,
Agents, or operational adapters. `sb-configure` rereads the summary after each
relevant change and completes its required aftercare. A new Design template is
appropriate only for a distinct recurring responsibility; a technology label
alone is not enough. Existing Specs and lifecycle artifacts are not silently
reconciled by configuration.

See [Customize SpecBind](./customization.md) for the surfaces themselves.

## 3. Start reverse Discovery

With committed Steering and a clean worktree, ask for a bounded adoption target
and the existing product version, for example:

```text
$sb-discovery Establish Specs from the existing implementation across this
repository as existing version v2.4.0. Investigate the current code and
tests as evidence, and ask me to confirm the boundaries and maintained
behavior before creating anything.
```

Discovery runs its adoption preflight, pins the inspected revision, and presents
one complete reverse proposal: the existing `baseline_version`, the candidate
`reverseSpecs`, their maintained intent and evidence, dependencies, blocking and
deferred unknowns, suspected defects, and excluded areas. Nothing is created
before you confirm that complete proposal.

## 4. Let Discovery finish the baseline

After you confirm the proposal, the same invocation creates the reverse
milestone and continues through Requirements, Design validation, Design
approval, and the milestone-wide Contract Review. It does not stop for routine
phase confirmations and never creates Tasks. It stops only for a question whose
answer would change the Spec, source drift, or a failed lifecycle check.

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

The established Specs then behave like ordinary existing Specs while retaining
their source revision and version provenance. From the next change onward, they
rejoin the ordinary steps in
[Start with an existing project](./start-existing-project.md).

## Next

- [Core concepts](./concepts.md)
- [Start with an existing project](./start-existing-project.md)
- [Customize SpecBind](./customization.md)
- [Current generated skill index](../reference/current-skill-index.md)

---

[User guide](../index.md) | [Start with an existing project](./start-existing-project.md) | [Core concepts](./concepts.md)
