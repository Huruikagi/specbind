# 0062: Keep the active brief a minimal free-form change input

Status: Accepted

## Context

The `SpecBind Brief` artifact exists only while one spec participates in an active milestone. Discovery creates it to communicate why and how the persistent spec should change, later same-milestone deltas are merged into it, and release finalization removes it.

The authoritative milestone identity and release binding already live in the roadmap and `spec.yaml`. Requirements, active Requirement IDs, design, tasks, and gate evidence record the approved and delivered result. Duplicating those state fields or imposing a parsed brief outline would turn an intentionally lightweight authoring input into another state artifact.

## Decision

- A live active brief is a singleton OKF concept with the exact known field:

  ```yaml
  ---
  type: SpecBind Brief
  ---
  ```

- The brief has no SpecBind-owned `artifact_id`, `milestone_id`, release version, timestamp, status, or Requirement ID field. Unknown top-level Front Matter extensions remain allowed under Decision 0045 but carry no SpecBind semantics.
- The body is free-form Markdown. SpecBind requires no fixed title, heading inventory, section order, user-story form, or machine-readable inline marker.
- A template and its AI instruction comments may encourage useful context such as the problem, desired outcome, scope boundaries, dependencies, or source request. These are authoring guidance rather than CLI-required fields or sections.
- Discovery creates the brief for a confirmed new or existing-spec change. Additional requests affecting the same spec in the same milestone are incorporated into the same current brief rather than creating another brief or preserving an append-only event stream.
- The CLI validates OKF syntax, the exact `type`, singleton multiplicity, and lifecycle placement. It does not parse, summarize, or semantically validate the body.

## Downstream use and lifecycle

- The requirements workflow reads the current brief as natural-language input for creating or revising the persistent requirements and active Requirement ID set.
- Under Decision 0017, the brief is not fingerprinted and does not appear in requirements-gate evidence. Editing only the brief does not invalidate an approved gate; a real scope change must update authoritative downstream artifacts and use the corresponding lifecycle event.
- Release-log authoring may use the brief as drafting context, but it must derive the delivered summary from final requirements, active Requirement IDs, completed tasks, roadmap scope, and release evidence. The brief is not authoritative release evidence.
- Successful release finalization removes the discovered brief. The immutable release reference retains its final pre-finalization content for deeper inspection.
- Confirmed abandonment may remove the brief only through the existing guarded milestone cleanup after affected persistent artifacts have been reconciled.

## Template behavior

- A managed brief template contains `type: SpecBind Brief`, optional unknown project metadata, a free-form scaffold, and optional `specbind:instruction` comments under Decision 0059.
- Template guidance is removed during materialization. The resulting live brief need only satisfy this minimal profile and the common live-artifact rules.

## Consequences

- Brief authoring remains natural and adaptable to different kinds of changes.
- Milestone and approval state stay in their authoritative machine-owned artifacts instead of being copied into prose metadata.
- The brief can evolve as discovery continues without creating automatic fingerprint invalidation unrelated to authoritative requirements.
- Released history remains concise in `log.md`, while Git retains the complete final working brief at the immutable release revision.
