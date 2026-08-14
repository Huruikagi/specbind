# 0050: Keep one global accepted cross-spec review

Status: Accepted

## Context

Decision 0035 originally made the active roadmap the canonical owner of cross-spec review evidence and left room to associate records with individual milestone items or specs. The cross-spec workflow accepts a milestone only when the complete active scope is mutually consistent. Per-item or per-spec pass records would duplicate that conclusion and permit misleading partial states in which some participants appear accepted while the milestone as a whole is not. Decision 0052 later separates the detailed machine record from always-loaded roadmap context without changing its milestone-wide meaning.

Failed reviews and remediation attempts are useful while a workflow is running, but they are not accepted release-readiness state.

## Decision

- The active milestone has at most one accepted `cross_spec_review` record, stored in the canonical state artifact defined by Decision 0052 rather than in roadmap frontmatter.
- That record covers the complete current `work_items` scope, its dependency graph, all applicable persistent contracts, and every required downstream review.
- The persisted record represents only the latest accepted all-consistent outcome. Failed, incomplete, and remediation-in-progress results remain in workflow run context and are not appended to the roadmap.
- No milestone item or participating spec stores its own cross-spec pass flag, review status, or copy of the accepted record.
- Every participating spec resolves the same global record through its `active_change.milestone_id` and membership in the roadmap's current `work_items`. `spec.yaml` does not add a reference to it.
- Under Decision 0053, the record retains only one compact semantic classification per roadmap item. Affected contract entries and reviewed downstream consumers are not persisted as independent facts or acceptance records.
- Any change to the Decision 0054 milestone baseline, roadmap scope, dependencies, applicable contracts, or required downstream review scope makes the complete record stale. A later accepted review replaces it atomically in the state artifact.
- Decision 0053 fixes the state-artifact fields, classification summary, and AI-authored judgment; Decision 0055 fixes the contract-first fingerprint inputs.

## Consequences

- A fresh `cross_spec_review` certifies that the entire active milestone is cross-spec consistent.
- SpecBind cannot represent or accidentally rely on partial cross-spec acceptance.
- This decision narrows Decision 0035: association with an individual item or `(milestone_id, canonical spec identity)` pair is no longer supported. Roadmap membership is sufficient for lookup.
- Release finalization preserves the final accepted global record in the companion archive defined by Decision 0052.
