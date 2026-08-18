# 0041: Do not store a separate per-spec change ID

Status: Accepted

## Context

Earlier lifecycle drafts included a generated `change_id` for each spec participating in a milestone. The current model permits at most one active change per spec, and additional deltas discovered in the same milestone merge into that active change. Therefore the pair of stable milestone identity and canonical spec identity already identifies the change unambiguously.

A separate ID would add another value to generate, expose, validate, migrate, and reconcile without distinguishing any supported v1 state. Renaming a spec could make path-based identity insufficient, but spec rename is a separate migration problem rather than a reason to burden every ordinary change.

## Decision

- Target `spec.yaml` has no `change_id` field.
- A spec-backed active change is identified by the pair `(milestone_id, canonical spec identity)`. The canonical spec identity is the spec's key/path under the configured spec root and must match its active-roadmap membership.
- Each spec has at most one active change in a milestone. Same-milestone deltas merge into that change rather than creating another identity.
- The project-state-owned global contract review resolves spec-backed changes through its milestone ID and canonical roadmap membership.
- Per-spec `log.md` and release-finalization idempotency use the release binding, milestone ID, and enclosing spec identity; optional project release references do not require a change ID.
- Direct or other non-spec roadmap items may receive roadmap-local identities if their own schema later needs them. Those identities do not become `spec.yaml.change_id`.
- Renaming a spec during an active milestone is outside the v1 lifecycle. A future explicit rename/migration workflow must reconcile the roadmap, contracts, history links, and filesystem atomically rather than silently treating a new path as the old identity.
- Migration to target `spec.yaml` does not invent a per-spec change ID. An unsupported `change_id` in target metadata is rejected as an unknown field.

## Consequences

- Ordinary active-change metadata has one fewer opaque identifier.
- Roadmap and spec lifecycle records use identities that already exist and are visible in project structure.
- Supporting spec rename later requires an explicit identity-migration design.
- Decision 0043 supplies the remaining generated lifecycle identifier as a UUID v7 `milestone_id`.
