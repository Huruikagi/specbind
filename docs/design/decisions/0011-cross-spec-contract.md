# 0011: Add a contract manifest for cross-spec seams

Status: Accepted

Decision 0155 replaces the Contract's type-discovered Markdown representation
with the fixed strict `contract.yaml` artifact. The persistent seam-manifest
direction and Contract-first workflow remain unchanged.

Implementation status: `tools/specbind/src/documents/contract.rs` and `contract_graph.rs` now implement artifact-local parsing and the complete current persistent-Spec graph read model. The resolver distinguishes mechanical missing or dangling-reference errors from File Ownership and dependency-cycle warnings. Semantic compatibility judgment and accepted review persistence remain agent and guarded-workflow responsibilities.

## Context

Contract review currently tends to load every participating spec's complete requirements, design, and tasks. Most of that content is internal to one spec. The review cost grows with document size even though the cross-spec questions concern a much smaller set of dependencies, ownership boundaries, exported capabilities, and shared invariants.

A separately written summary would reduce input size but drift from the authoritative specs. SpecBind instead needs a maintained, minimal source of truth for the boundary that other specs may observe or depend on.

## Decision

- Every active spec has one persistent `SpecBind Contract` artifact describing its current cross-spec seams. Decision 0057 discovers it by OKF type; `contract.md` is the conventional default path rather than the artifact identity.
- The contract is a manifest, not a summary of requirements, design, or tasks.
- Its core categories are Owns, Exports, Consumes, Invariants, and File Ownership.
- Entries have stable identifiers, and cross-spec references resolve an explicit spec, category, and entry ID.
- The design workflow maintains the contract alongside the current design and reviews whether internal changes require contract updates.
- Contract review reads roadmap and contracts first, then loads full spec documents only for affected or ambiguous boundaries.
- Direct implementation has no persisted Contract-impact field. Requiring no canonical Requirements, Design, or Contract change is the route precondition; a change that cannot preserve it is rerouted to Spec work.
- The Rust CLI validates syntax, identifiers, references, graph consistency, and other deterministic invariants.
- AI review evaluates semantic compatibility and downstream impact in a free-form accepted assessment under Decision 0078.
- Exact Markdown syntax and ID format are accepted by [Decision 0056](./0056-canonical-contract-markdown.md). The diagnostic schema remains Draft.

## Scope boundary

Include an item only when changing it could require another spec's design or verification result to change. Do not include internal architecture, implementation steps, prose summaries, completed spec changes, or release history.

## Semantic review

The CLI computes structural differences and the current dependency graph but does not classify semantic compatibility. The review agent records one accepted free-form judgment bound to exact Contract-first inputs. Graph output remains an ephemeral read model; the accepted review persists its input revisions and judgment, not a duplicate derived graph or closed compatibility enum.

## Migration and fallback

- Bootstrap contracts for all active existing specs by extracting only seams supported by current artifacts and repository evidence.
- Do not invent a contract or rewrite requirements and design merely to complete migration.
- Mark ambiguous ownership and dependencies for review instead of silently resolving them.
- A missing contract on an active spec is an incomplete migration or damaged state, not the normal empty-contract representation.
- When a required contract is missing, contract review fails. Requirements and Design may be selected as deeper semantic inputs, but they never substitute for the required Contract.

## Consequences

- Contract review cost follows boundary size more closely than total spec size.
- Contracts persist and evolve as active specification artifacts across releases.
- Contract templates and shared review rules become part of project-customizable settings, subject to the fixed machine-readable contract.
- Downstream revalidation can be targeted from changed entries and consumer edges.
- Contract maintenance and mechanical graph checks become explicit workflow gates.

File Ownership overlaps remain warnings for semantic review. V1 does not add a machine-readable shared-ownership declaration or reject every overlap automatically.
