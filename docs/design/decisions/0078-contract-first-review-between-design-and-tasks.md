# 0078: Keep one free-form contract review between Design and Tasks

Status: Accepted

Implementation status: the Rust acceptance operation validates strict version-1 candidate JSON, non-empty assessment Markdown, canonical optional deep selectors, the complete Contract graph, and the exact current input revision map. Direct-only Roadmaps are rejected. It then verifies the full baseline commit and ancestry, matching participating-Spec `tasks` state, fresh Design gates, and absence of `tasks.yaml`; re-resolves all inputs; owns the RFC 3339 timestamp; and atomically replaces the four-field accepted artifact. The read model strictly validates the accepted profile and Markdown body, reconstructs deep selectors, re-resolves current inputs, rechecks the Git baseline, and reports `not required`, `missing`, `fresh`, `stale`, or `invalid`. A common later-boundary guard now requires a fresh review and current Roadmap participation for Tasks approval and implementation validation, requires a fresh review for Spec-backed release preflight, and accepts `not required` only for Direct-only release preflight. CLI command rendering remains a subsequent increment.

## Context

Decisions 0050 through 0055 introduced one milestone-wide cross-spec review, but Decision 0053 also required a closed classification enum for every Roadmap item and Decision 0055 allowed Tasks as review input. Contract compatibility and external-consumer impact are open-ended semantic judgments, while Direct items are defined by the absence of canonical Contract change.

## Decision

- Every milestone containing at least one `new_specs` or `spec_updates` item requires one accepted cross-spec review, even when only one Spec-backed item participates. A single changed producer may affect persistent consumers outside the milestone.
- A Direct-only milestone has no active or archived cross-spec-review artifact.
- All participating Spec-backed items must have current Design approval and be in the `tasks` state before review begins. No current `tasks.yaml` is authored until review passes.
- The CLI always validates and fingerprints every current persistent Contract. The review skill may declare selected Requirements and Design artifacts as deeper semantic inputs; Tasks are not allowed inputs.
- The normalized Roadmap review projection contains milestone identity, baseline revision, Spec-backed item identity, summary, category, and only Spec-to-Spec dependencies. Direct items, their status, and dependencies to or from Direct items are excluded.
- `state/cross-spec-review.md` is an OKF concept with exactly these SpecBind-owned fields:
  - `type: SpecBind Cross-Spec Review`
  - `milestone_id`
  - `passed_at`
  - `input_revisions`
- The OKF profile has no `schema_version` or `classifications`. Presence of a fresh artifact means the complete current semantic assessment passed; its non-empty Markdown body is the accepted free-form judgment.
- The review skill passes a strict transient JSON input from stdin or a repository-external file:

  ```json
  {
    "schemaVersion": 1,
    "assessment": "...Markdown...",
    "deepInputs": [
      "specs/checkout#requirements",
      "specs/checkout#design/main"
    ]
  }
  ```

- `deepInputs` may contain only canonical Requirements or Design selectors. The CLI resolves paths, computes hashes, sets `passed_at`, and never accepts agent-supplied paths or fingerprints as authority.
- Failed or incomplete reviews remain run-scoped. The skill automatically remediates and reruns at most twice; unresolved Specs remain in Design and no accepted artifact is written.
- Review findings may identify affected persistent Specs outside current scope. Any Spec requiring owned work is added to the Roadmap and brought through Design before acceptance. External or unmanaged consumers are handled by semantic agent judgment and user guidance; v1 defines no closed disposition enum.
- Review does not mutate Spec state by itself. The agent presents affected Specs, obtains confirmation where scope changes materially, and invokes explicit invalidation or Roadmap update operations.
- A confirmed Design rewind deletes the accepted review, preserves Requirements evidence, clears Design, Tasks, and completion evidence for affected Specs, and retains prose/task documents only as stale repair input. Git preserves the previous review.
- Out-of-band input edits leave the file present but stale; read-only checks report the mismatch until explicit invalidation or a successful replacement review.
- Cross-spec review is milestone-level state, not part of the per-Spec `release_ready` invariant. Unaffected Specs retain their local state when the global review becomes stale.
- A fresh review is required before Tasks approval, implementation validation, and release preflight. These boundaries recheck freshness without rerunning semantic review.
- File Ownership overlap and dependency-cycle detection are warnings for agent judgment. Missing Contracts, dangling references, invalid syntax, and impossible targets are mechanical errors.

## Consequences

- Decision 0053 is superseded. Decisions 0050, 0052, and 0055 remain in force only where consistent with this decision.
- The durable record preserves the exact inputs and useful reasoning without pretending that three enum values model every compatibility case.
- Design correction happens before Task authoring, reducing discarded plans.
- Per-Spec implementation completion and milestone-wide consistency remain distinct state dimensions.
