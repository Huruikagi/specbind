# 0050: Keep one global accepted cross-spec review

Status: Accepted

## Context

Decision 0035 made the active roadmap the canonical owner of cross-spec review evidence, but left room to associate records with individual milestone items or specs. The cross-spec workflow accepts a milestone only when the complete active scope is mutually consistent. Per-item or per-spec pass records would duplicate that conclusion and permit misleading partial states in which some participants appear accepted while the milestone as a whole is not.

Failed reviews and remediation attempts are useful while a workflow is running, but they are not accepted release evidence.

## Decision

- The active roadmap contains at most one top-level `cross_spec_review` record.
- That record covers the complete current `work_items` scope, its dependency graph, all applicable persistent contracts, and every required downstream review.
- The persisted record represents only the latest accepted all-consistent outcome. Failed, incomplete, and remediation-in-progress results remain in workflow run context and are not appended to the roadmap.
- No milestone item or participating spec stores its own cross-spec pass flag, review status, or copy of the accepted record.
- Every participating spec resolves the same roadmap-level record through its `active_change.milestone_id` and membership in the roadmap's current `work_items`. `spec.yaml` does not add a reference to it.
- The record may retain facts such as affected contract entries and reviewed downstream consumers, but these facts do not become independent per-spec acceptance records.
- Any change to milestone scope, dependencies, applicable contracts, or required downstream review scope makes the complete record stale. A later accepted review replaces it atomically.
- The exact `cross_spec_review` fields and fingerprint inputs remain a follow-up schema decision.

## Consequences

- A fresh `cross_spec_review` certifies that the entire active milestone is cross-spec consistent.
- SpecBind cannot represent or accidentally rely on partial cross-spec acceptance.
- This decision narrows Decision 0035: association with an individual item or `(milestone_id, canonical spec identity)` pair is no longer supported. Roadmap membership is sufficient for lookup.
- The released roadmap archive preserves the final accepted global record.
