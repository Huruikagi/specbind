# 0038: Fingerprint design and contract at the design gate

Status: Accepted

## Context

The design gate approves both the spec's internal technical design and its externally observable cross-spec seam. `requirements.md` and the ordered active Requirement IDs are already owned by the prerequisite requirements gate, so copying them into design evidence would duplicate freshness state.

Every active spec has a persistent `contract.md` under Decision 0011, including the canonical empty representation for a spec with no cross-spec seams. Treating that file as optional would make a missing migration artifact indistinguishable from an intentionally empty contract.

## Decision

- Design gate evidence fingerprints exactly two direct artifact inputs:
  - `design.md`
  - `contract.md`
- Both fingerprints are required. A missing `contract.md` is a migration or consistency failure and prevents design approval; it is not interpreted as no contract impact.
- Each Markdown file is normalized only for line endings and then hashed as a complete file using the Decision 0016 fingerprint representation.
- The design gate does not copy a requirements fingerprint or active Requirement ID list. It instead requires the prerequisite requirements gate to remain fresh under Decision 0032.
- Design gate evidence uses the common `passed_at`, `approval_mode`, and conditional `delegation_workflow` approval fields. Its `input_revisions` object contains only the two required artifact keys and rejects additional keys.

## Consequences

- Any substantive edit to either the technical design or contract invalidates design approval and downstream gates.
- Formatting and prose changes remain approval-relevant except for CRLF/LF-only differences.
- An intentionally empty contract remains explicit, reviewable, and distinguishable from a missing file.
- Design evidence stays local to direct gate inputs without duplicating requirements evidence.
