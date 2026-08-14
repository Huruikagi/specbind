# 0032: Derive freshness through gate-local input ownership

Status: Accepted

## Context

Later workflows read requirements, design, contracts, and tasks together, but that does not mean every downstream gate should duplicate fingerprints for every upstream artifact. Repeating the same revisions in completion evidence would enlarge `spec.yaml`, create multiple stored copies of one approval fact, and require rules for resolving disagreement between them.

Freshness is a comparison of the current gate-owned input projection with the projection that passed, plus the freshness of every prerequisite gate. It describes current content identity, not whether a file has ever been edited since approval.

## Decision

- Each gate persists only its direct input revision data. Reading an upstream artifact for semantic review does not transfer fingerprint ownership to the downstream gate.
- A gate is fresh only when its own current input projection equals its accepted projection, every prerequisite gate is fresh, and the surrounding lifecycle invariants still hold.
- Freshness cascades in this order: requirements -> design -> tasks -> completion.
- Requirements freshness compares the normalized `requirements.md` fingerprint and the ordered active Requirement ID array accepted by Decisions 0017 and 0018.
- Design freshness requires a fresh requirements gate and compares the normalized `design.md` and `contract.md` fingerprints owned by the design gate.
- Tasks freshness requires a fresh design gate and compares the normalized typed `plan` fingerprint accepted by Decision 0028.
- Completion freshness requires a fresh tasks gate, a valid Decision 0029 implementation-revision relationship, current all-completed and unblocked task state, and currently accepted completion evidence under Decision 0030.
- Completion evidence does not repeat requirements, design, contract, active Requirement ID, or task-plan revisions. Their authoritative snapshots remain in their owning gates and are checked through the freshness chain.
- While accepted evidence still exists, an out-of-band content change that is fully reverted to the accepted projection restores equality and is fresh; SpecBind does not treat edit history alone as invalidation evidence. Reverting content does not recreate evidence already cleared by an explicit lifecycle invalidation event.
- Gate-specific normalization still applies: Markdown uses line-ending normalization, active Requirement IDs use exact ordered-array comparison, and task plans use typed projection normalization and JCS canonicalization.
- A stale upstream gate makes every downstream gate stale even when downstream-owned bytes or values have not changed.

## Consequences

- Every stored revision has one authoritative owner.
- Completion evidence stays focused on implementation validation rather than duplicating earlier approval records.
- Status can explain the earliest stale gate and derive downstream staleness without comparing redundant snapshots.
- Reverting an out-of-band edit exactly can restore freshness while evidence remains; semantic workflow changes use the normal invalidation event and require new approval.
