# 0011: Add a contract manifest for cross-spec seams

Status: Accepted

## Context

Cross-spec review currently tends to load every participating spec's complete requirements, design, and tasks. Most of that content is internal to one spec. The review cost grows with document size even though the cross-spec questions concern a much smaller set of dependencies, ownership boundaries, exported capabilities, and shared invariants.

A separately written summary would reduce input size but drift from the authoritative specs. SpecBind instead needs a maintained, minimal source of truth for the boundary that other specs may observe or depend on.

## Decision

- Every active spec has one persistent `SpecBind Contract` artifact describing its current cross-spec seams. Decision 0057 discovers it by OKF type; `contract.md` is the conventional default path rather than the artifact identity.
- The contract is a manifest, not a summary of requirements, design, or tasks.
- Its core categories are Owns, Exports, Consumes, Invariants, and File Ownership.
- Entries have stable identifiers, and cross-spec references resolve an explicit spec, category, and entry ID.
- The design workflow maintains the contract alongside the current design and reviews whether internal changes require contract updates.
- Cross-spec review reads roadmap and contracts first, then loads full spec documents only for affected or ambiguous boundaries.
- Direct implementation candidates declare contract impact. A change that cannot justify no impact is rerouted to spec work.
- The Rust CLI validates syntax, identifiers, references, graph consistency, and other deterministic invariants.
- AI review classifies semantic impact and evaluates whether contracts and implementations are substantively compatible.
- Exact Markdown syntax and ID format are accepted by [Decision 0056](./0056-canonical-contract-markdown.md). The diagnostic schema remains Draft.

## Scope boundary

Include an item only when changing it could require another spec's design or verification result to change. Do not include internal architecture, implementation steps, prose summaries, completed spec changes, or release history.

## Compatibility classes

Agent review classifies a contract-relevant change as:

- `LOCAL_ONLY`: the current contract is unchanged.
- `CONTRACT_COMPATIBLE`: the contract changes without breaking existing consumers, such as an additive export.
- `CONTRACT_BREAKING`: an existing dependency, ownership boundary, or invariant may require downstream revision or revalidation.

The CLI may compute the structural diff and affected dependency graph, but it does not decide semantic compatibility from syntax alone.

## Migration and fallback

- Bootstrap contracts for all active existing specs by extracting only seams supported by current artifacts and repository evidence.
- Do not invent a contract or rewrite requirements and design merely to complete migration.
- Mark ambiguous ownership and dependencies for review instead of silently resolving them.
- A missing contract on an active spec is an incomplete migration or damaged state, not the normal empty-contract representation.
- When a required contract is missing, cross-spec review fails. Requirements and Design may be selected as deeper semantic inputs, but they never substitute for the required Contract.

## Consequences

- Cross-spec review cost follows boundary size more closely than total spec size.
- Contracts persist and evolve as active specification artifacts across releases.
- Contract templates and shared review rules become part of project-customizable settings, subject to the fixed machine-readable contract.
- Downstream revalidation can be targeted from changed entries and consumer edges.
- Contract maintenance and mechanical graph checks become explicit workflow gates.

## Open questions

- How an active change records changed contract entries and its compatibility classification.
- Which File Ownership overlaps are allowed and how intentional sharing is declared.
- Whether contract graph output is ephemeral CLI output or may be persisted as derived evidence.
