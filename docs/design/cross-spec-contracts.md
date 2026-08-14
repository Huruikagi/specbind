# Cross-spec contracts

This document adapts [pc-build-planner Issue #48](https://github.com/Huruikagi/pc-build-planner/issues/48) into the portable SpecBind workflow defined by [Decision 0011](./decisions/0011-cross-spec-contract.md).

Status: Draft

## Purpose

`contract.md` is the current manifest of a spec's externally observable seams. It lets cross-spec review begin with a small dependency graph rather than loading every participating `requirements.md`, `design.md`, and `tasks.yaml`.

It answers:

- What boundary does this spec own?
- What may another spec depend on?
- Which other contracts does this spec consume?
- Which cross-spec invariants must remain true?
- Which public file boundaries must not have ambiguous ownership?

It does not explain the internal implementation of those boundaries.

Cross-spec dependencies never resolve through another spec's milestone-local Task IDs. Active implementation order is carried by `roadmap.md`, while persistent observable dependencies resolve through this contract manifest; see [Decision 0027](./decisions/0027-spec-local-task-dependencies.md).

## Artifact lifecycle

`{{SPEC_DIR}}/specs/<feature>/contract.md` persists with `requirements.md` and `design.md` as part of the active specification.

| State | Contract behavior |
| --- | --- |
| New spec | Design establishes the initial contract, including an explicit empty contract when no cross-spec seam exists. |
| Active change | Design updates affected entries while preserving stable IDs for unchanged meanings. |
| Released and idle | The current released contract remains present and available to consumers. |
| Released change | The per-spec `log.md` records the contract impact classification and changed entry references where useful. |
| Retired capability | Retirement rules must resolve or migrate incoming consumer references before removal. |

The contract is not milestone-local and is never deleted merely because `brief.md` and `tasks.yaml` are finalized.

The design gate always fingerprints both `design.md` and `contract.md` under [Decision 0038](./decisions/0038-design-gate-inputs.md). A missing contract therefore prevents approval rather than silently taking the empty-contract path.

## Content boundary

The initial conceptual sections are:

| Section | Contains | Excludes |
| --- | --- | --- |
| Owns | Cross-spec responsibilities and ownership boundaries. | General feature description. |
| Exports | Types, interfaces, services, events, artifacts, or behavior other specs may depend on. | Every internal function or module. |
| Consumes | Explicit references to entries owned by another spec. | Informal mentions without a resolvable target. |
| Invariants | Minimal rules whose change could alter another spec's design or verification. | Internal business rules with no cross-spec effect. |
| File Ownership | Public or shared file boundaries where overlapping writes could conflict. | Complete internal source-tree inventory. |

The inclusion test is:

> If this item changes, could another spec's design or verification result need to change?

If the answer is no, the information belongs in requirements, design, tasks, or implementation evidence instead.

## Working representation

The exact Markdown schema is not accepted yet. It should remain concise and structurally parseable, for example:

```markdown
# Contract

## Owns
1. part-compatibility-evaluation

## Exports
1. compatibility-result
2. compatibility-rule-provider

## Consumes
1. `part-catalog/exports/2` — part-type
2. `build-state/exports/1` — selected-parts

## Invariants
1. compatibility evaluation must not mutate selected parts

## File Ownership
1. `src/domain/compatibility/**`
```

Stable entry IDs must not be renumbered merely to close gaps. A changed label does not create a new identity when the meaning is unchanged; a breaking semantic replacement must remain distinguishable in history and review.

## Review flow

Cross-spec review proceeds contract-first:

1. Read the active roadmap and participating contracts.
2. Ask the CLI to validate contract structure and construct the dependency graph.
3. Review ownership overlap, dependency direction, invariants, and File Ownership conflicts.
4. Compare changed entries with the prior released reference.
5. Classify the change as `LOCAL_ONLY`, `CONTRACT_COMPATIBLE`, or `CONTRACT_BREAKING`.
6. Traverse affected consumers and load full requirements, design, and tasks only where the contract change or ambiguity requires it.
7. Record the classification, affected entries, downstream review scope, and accepted all-consistent outcome once in `state/cross-spec-review.yaml` under [Decisions 0050](./decisions/0050-global-cross-spec-review.md) and [0052](./decisions/0052-project-state-artifacts.md).

`LOCAL_ONLY` still requires the spec-local review appropriate to the change. `CONTRACT_COMPATIBLE` does not mean no review; it narrows review to relevant consumers. `CONTRACT_BREAKING` requires downstream revision or explicit revalidation before release readiness.

## CLI and agent boundary

The Rust CLI can deterministically check:

- required sections and supported syntax
- unique and stable-form entry IDs
- resolvable spec/category/entry references
- missing targets and dangling consumer edges
- duplicate ownership declarations
- File Ownership overlap candidates
- dependency cycles where the configured rules prohibit them
- structural differences between current and released contracts
- presence of a contract for every active spec in the review scope

The agent reviews:

- whether the contract contains the real cross-spec seam and nothing more
- whether ownership and invariant meanings are correct
- whether an additive-looking change is semantically compatible
- whether a design still implements its declared contract
- whether a direct change truly has no contract impact
- which downstream specs need deeper review or revalidation

Mechanical findings may identify candidates and affected graph nodes; they do not replace semantic compatibility judgment.

## Direct implementation candidates

Every direct item in the active roadmap declares contract impact, conceptually:

```markdown
- Contract Impact: none
```

If the agent cannot justify `none` against current contracts and the intended file/behavior changes, discovery stops the direct route and reclassifies the work as an existing-spec update or new spec. When impact exists, the roadmap should reference the affected contract entries rather than only free-form names.

The CLI can validate that the field is present and references resolve. The agent validates the truth of the declaration.

## Existing-spec bootstrap

Initial adoption is a migration workflow rather than ordinary design authoring:

1. Enumerate active specs.
2. Extract observable seams from current requirements, design, tasks when present, steering, and repository evidence.
3. Do not invent unproven exports, dependencies, ownership, or invariants.
4. Mark ambiguity for human or agent review.
5. Assign stable entry IDs and connect explicit cross-spec references.
6. Run CLI graph validation across all active contracts.
7. Perform a contract-first semantic review of duplicates, direction, invariants, ambiguity, and dangling references.
8. Commit the bootstrap as an explicit migration baseline without redesigning source specs.

An active spec with no cross-spec seams still gets the canonical empty representation. A missing file therefore remains distinguishable from an intentionally empty contract.

## Missing-contract fallback

When an active or referenced spec lacks `contract.md`:

- report the migration or consistency failure
- read the relevant requirements, design, and tasks for safe review
- do not treat absence as proof of no cross-spec impact
- do not silently generate and accept inferred entries during unrelated work

Fallback preserves safety but is not a supported steady state.

## Customization boundary

The default `settings/templates/specs/contract.md` and related shared rules are project-customizable under [Decision 0008](./decisions/0008-customization-surface.md). Customization may adjust prose and presentation but must preserve the accepted machine-readable identity and reference contract. The CLI reports incompatible customization explicitly.

## Open design questions

- Exact Markdown grammar and localized headings.
- Entry ID and cross-reference normalization.
- Exact schema and fingerprint inputs for the global `state/cross-spec-review.yaml` record accepted by Decisions 0050 and 0052.
- Rules for shared File Ownership and generated files.
- Approval invalidation when a contract classification changes.
- Release-readiness evidence required for affected consumers.
