# 0079: Keep optional research as a milestone-local singleton

Status: Accepted

## Context

Brownfield gap analysis may require enough investigation that a run-scoped report is lost across agent sessions. The inherited cc-sdd workflow persisted `research.md`, but appended attempts indefinitely and allowed later Design work to depend on a file that remained outside the authoritative specification.

## Decision

- An active Spec may contain at most one optional OKF concept with `type: SpecBind Research`; `research.md` is its default filename and type-based discovery is authoritative.
- `specbind-gap-analysis` creates or revises the artifact only when durable brownfield findings are useful. Greenfield work normally omits it; quick omits it unless requested; batch applies the same per-Spec judgment.
- Research is current-state input for the active milestone, not an append-only attempt log. Git preserves earlier drafts.
- Its Markdown body is non-empty and free-form. The CLI parses no headings and the OKF profile has no `schema_version`.
- Requirements and Design workflows may read Research. Every accepted requirement, decision, constraint, and rationale needed after release must be incorporated into the authoritative Requirements, Design, or Contract.
- Persistent specification artifacts must not depend on or defer normative meaning to Research. `specbind-validate-design` checks this semantically; Research is excluded from gate fingerprints.
- Editing Research does not mechanically invalidate approved Design. The responsible skill decides whether a new finding requires an explicit Requirements or Design rewind.
- Release finalization, scope removal, and milestone abandonment delete Research with the active Brief and Tasks. An idle Spec containing Research is inconsistent.
- Failed debug attempts and ordinary implementation diagnostics do not use this artifact. `specbind-debug` remains run-scoped, while durable implementation knowledge belongs in Implementation Notes.

## Consequences

- Substantial gap analysis survives session boundaries without becoming released source of truth.
- Design remains self-contained after milestone-local artifacts are removed.
- Optionality is represented by file absence, and Git provides attempt history.

