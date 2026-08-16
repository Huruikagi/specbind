# 0038: Fingerprint design and contract at the design gate

Status: Accepted

## Context

The design gate approves both the spec's internal technical design set and its externally observable cross-spec seam. The singleton requirements artifact and ordered active Requirement IDs are already owned by the prerequisite requirements gate, so copying them into design evidence would duplicate freshness state.

Every active spec has one persistent `SpecBind Contract` artifact under Decision 0011, including the canonical empty representation for a spec with no cross-spec seams. Decision 0057 discovers it and the one-or-more `SpecBind Design` artifacts by OKF type rather than filename.

## Decision

- Design gate evidence fingerprints the complete current direct artifact set:
  - the singleton contract under logical key `contract`
  - every design document under logical key `design/<artifact_id>`
- The contract and at least one design artifact are required. A missing contract is a migration or consistency failure and prevents design approval; it is not interpreted as no contract impact.
- Before approval, every design artifact must satisfy the Decision 0061 Front Matter/body traceability contract, and the union of the complete design set must cover every active Requirement ID.
- Each Markdown file is normalized only for line endings and then hashed as a complete file using the Decision 0016 fingerprint representation.
- The design gate does not copy a requirements fingerprint or active Requirement ID list. It instead requires the prerequisite requirements gate to remain fresh under Decision 0032.
- Design gate evidence uses the common `passed_at`, `approval_mode`, and conditional `delegation_workflow` approval fields. Its `input_revisions` object contains exactly the currently discovered contract and design logical keys and rejects any other key.

## Consequences

- Any substantive edit to the technical design set or contract invalidates design approval and downstream gates. Adding or removing a design identity also invalidates approval.
- Formatting and prose changes remain approval-relevant except for CRLF/LF-only differences.
- Renaming or moving a file alone does not invalidate approval because the stable logical identity and file content remain unchanged.
- An intentionally empty contract remains explicit, reviewable, and distinguishable from a missing file.
- Design evidence stays local to direct gate inputs without duplicating requirements evidence.
- Traceability failures block approval without adding another evidence field.

## Implementation status

The generated Rust-owned v1 schema now requires logical key `contract`, at least one `design/<artifact_id>` key, and no unrelated input key. This replaces the earlier scaffold's fixed default-filename keys and supports the complete discovered design set.
