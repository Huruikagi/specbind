# 0050: Keep one global accepted contract review

Status: Accepted

Decision 0078 preserves one global accepted record but removes per-item classifications and excludes Direct-only milestones.

## Context

Decision 0035 originally made the active roadmap the canonical owner of contract review evidence and left room to associate records with individual milestone items or specs. The cross-spec workflow accepts a milestone only when the complete active scope is mutually consistent. Per-item or per-spec pass records would duplicate that conclusion and permit misleading partial states in which some participants appear accepted while the milestone as a whole is not. Decision 0052 later separates the detailed machine record from always-loaded roadmap context without changing its milestone-wide meaning.

Failed reviews and remediation attempts are useful while a workflow is running, but they are not accepted release-readiness state.

## Decision

- The active milestone has at most one accepted `cross_spec_review` record, stored in the canonical state artifact defined by Decision 0052 rather than in roadmap frontmatter.
- That record covers the complete current Spec-backed Roadmap projection, all current persistent Contracts, and every deeper Requirements or Design input materially used by the judgment. Direct items and dependencies to or from them are excluded under Decision 0078.
- The persisted record represents only the latest accepted all-consistent outcome. Failed, incomplete, and remediation-in-progress results remain in workflow run context and are not appended to the roadmap.
- No milestone item or participating spec stores its own cross-spec pass flag, review status, or copy of the accepted record.
- Every participating spec resolves the same global record through its `active_change.milestone_id` and membership in the roadmap's current `work_items`. `spec.yaml` does not add a reference to it.
- The record contains the minimal Decision 0078 Front Matter and one free-form accepted assessment. It persists no per-item classification, affected-entry list, or reviewed-consumer result.
- Any change to the Decision 0054 milestone baseline, roadmap scope, dependencies, applicable contracts, or required downstream review scope makes the complete record stale. A later accepted review replaces it atomically in the state artifact.
- Decision 0078 fixes the state-artifact fields and free-form judgment; Decision 0055, as amended, fixes the Contract-first fingerprint inputs.

## Consequences

- A fresh `cross_spec_review` certifies that the entire active milestone is cross-spec consistent.
- SpecBind cannot represent or accidentally rely on partial cross-spec acceptance.
- This decision narrows Decision 0035: association with an individual item or `(milestone_id, canonical spec identity)` pair is no longer supported. Roadmap membership is sufficient for lookup.
- Release finalization preserves the final accepted global record in the companion archive defined by Decision 0052.
