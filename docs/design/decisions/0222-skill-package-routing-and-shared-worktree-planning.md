# 0222: Align Skill routing and shared-worktree planning

Status: Accepted

## Context

The product-Skill audit found relative links resolved from the package root
instead of the referencing file, concurrent Plan authors sharing a worktree
despite a single-Spec dirty Design handoff, and Direct diagnosis inheriting
Spec-only inputs and an obsolete reference to Spec-backed recovery bounds.
Steering also described its edits as invalidating nothing despite the existing
completion-revision contract.

## Decision

Amend the concurrent phase-dispatch clauses retained through Decisions 0153,
0161, and 0174. Requirements and Tasks retain their phase-relative eligibility,
but Plan executes one owning authoring phase at a time in the shared worktree,
through approval and its checkpoint. Each Design author/independent-validator/
approval sequence finishes before another Spec's Design starts. Read fresh
status between handoffs. An unfinished dirty handoff prevents switching writers;
independent work may continue only when that handoff is clean. Do not introduce
implicit worktrees, merges, new dependency edges, or approval authority.

Direct recovery keeps its existing two review-repair rounds and at most two
diagnosis rounds; Decision 0218's three-cycle Task ledger remains Spec-backed
only. A Direct diagnosis reads the exact Roadmap summary, observable done
condition, applicable checks, current Steering, actual diff/failure evidence,
and applicable shared Contract. It requires no synthetic Spec artifacts. A
canonical-artifact or unsettled product/architecture need returns to Discovery.

Preserve Decision 0119: Steering is not planning-gate evidence, but its edits
are ordinary project changes that stale accepted completion evidence.

Resolve package-relative Markdown links from the referencing document in each
rendered Agent package. Check that local file targets are actually distributed,
including links inside references. Move Drive's conditional capacity recovery
and implementation validation's completion withdrawal into packaged references.
Keep their authority, retry bounds, handoff fields, and entry triggers unchanged;
keep pre-dispatch constraints visible before an owner can start.

## Verification

Mechanical tests resolve local Markdown links in every rendered package and
verify progressive resources and retained recovery contracts. Behavioral checks
exercise independent Design handoffs, Direct diagnosis without a Spec, and the
conditional recovery routes. Update B1/B2 to distinguish logical eligibility
from simultaneous writers and add B2a for independent Design dirty handoffs.

## Maintainer choices

Use serial writes rather than inventing worktree isolation and merge ownership.
Keep Direct retry limits unchanged. Do not broaden approval authority or alter
lifecycle state machines. There is no new configuration or unresolved product
choice in this change; parallel isolated planning can be designed separately.

### Separate follow-up: absent Contracts for new delivery Specs

Resolved by [Decision 0223](./0223-design-scoped-contract-validation.md).

The existing Design validator runs the complete Contract graph check, whose
`CONTRACT_GRAPH_CONTRACT_UNAVAILABLE` diagnostic includes other persistent Specs.
Its provisional-graph exception currently covers only waiting participants in
reverse establishment. A new delivery Spec without a Contract can therefore
block another participant's Design validation. B2a uses existing Contracts to
isolate the shared-worktree fix; it does not prove this new-Spec route.

Before changing this boundary, define whether and when Design-local validation
may tolerate an unfinished delivery participant, while keeping current-Spec and
consumed-seam checks strict and requiring the full graph at Contract Review.
This audit does not silently extend the reverse exception to delivery. This
follow-up needs its own scenario and accepted validation rule.
