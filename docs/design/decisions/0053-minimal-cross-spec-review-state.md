# 0053: Pair structured classifications with an AI-authored review

Status: Accepted

## Context

The global cross-spec review must preserve the semantic judgment that cannot be reproduced deterministically from contract diffs and file hashes alone. A classification map is useful as a compact, machine-readable index for coverage checks and status summaries, but it is too lossy to explain why an apparently additive change is compatible, why a breaking change is adequately remediated, or why a direct change is truly local.

Detailed changed-entry and consumer lists should not be duplicated as rigid persistent fields when the CLI can derive them from contract diffs, the contract graph, and roadmap scope. The durable payload should instead be the accepted AI assessment itself, bound to exact input revisions and accompanied by a small structured summary.

## Decision

- The accepted artifact is the OKF concept `{{SPEC_DIR}}/state/cross-spec-review.md`, replacing the standalone YAML filename proposed by Decision 0052.
- Its YAML frontmatter requires these SpecBind fields:
  - `type: SpecBind Cross-Spec Review`
  - `schema_version: 1`
  - `milestone_id`
  - `passed_at`
  - `classifications`
  - `input_revisions`
- `milestone_id` is the Decision 0043 UUID v7 and must exactly match the active roadmap.
- `passed_at` is the Decision 0036 timezone-qualified RFC 3339 timestamp at which the complete milestone review passed.
- `classifications` is the structured intermediate judgment produced from the contract diff. It becomes an input to the AI-authored final assessment and remains its machine-readable summary. It is a non-empty sparse mapping with these optional groups:
  - `specs`: canonical spec identity to `LOCAL_ONLY`, `CONTRACT_COMPATIBLE`, or `CONTRACT_BREAKING`
  - `direct_changes`: roadmap-local direct-change ID to `LOCAL_ONLY`
- A classification group is present only when it contains at least one entry. At least one group is required.
- Every current roadmap work item appears exactly once in `classifications`: both `new_specs` and `spec_updates` resolve into `specs`, while `direct_changes` resolves into `direct_changes`.
- A direct change with non-local contract impact is invalid and must be rerouted to a new-spec or spec-update item before the global review can pass.
- `input_revisions` is the non-empty contract-first mapping accepted by Decision 0055. It identifies the current source artifact revisions from which classifications and the final judgment were produced. The Decision 0054 baseline supplies the immutable before-state and is included in the normalized roadmap scope projection.
- The non-empty Markdown body is the accepted AI-authored final judgment. The review workflow must address every structured classification there and explain semantic compatibility or breakage, downstream conclusions, and any reasoning needed to understand why the complete milestone is cross-spec consistent.
- The CLI does not parse required Markdown headings or turn prose claims into independent booleans. Templates may recommend Conclusion, Assessment, and Downstream Compatibility sections.
- The artifact contains no duplicated `status`, approval mode, workflow name, reviewer identity, rigid affected-entry list, rigid downstream-spec list, failed findings, or attempt history. Presence of a fresh accepted artifact means the global review passed.
- Failed and incomplete assessments remain run-scoped. The review skill submits only a successful candidate assessment to the guarded CLI mutation that writes or replaces the accepted artifact.

## Consequences

```markdown
---
type: SpecBind Cross-Spec Review
schema_version: 1
milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
passed_at: 2026-08-14T17:30:00+09:00
classifications:
  specs:
    account-auth: CONTRACT_COMPATIBLE
    checkout: LOCAL_ONLY
  direct_changes:
    update-ci: LOCAL_ONLY
input_revisions:
  steering/roadmap.md#cross-spec-scope: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
  specs/account-auth#contract: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
---

# Cross-Spec Review

## Conclusion

The milestone is cross-spec consistent.

## Assessment

The authentication export is additive. Checkout consumes it without changing existing public behavior. The CI item does not alter a published contract.

## Downstream Compatibility

Checkout requirements and design remain valid. No downstream spec revision is required.
```

- CLI status can summarize freshness and classification counts without loading the Markdown body into ordinary agent context.
- Detailed review and release-history flows can request the AI-authored judgment explicitly.
- Release finalization archives the accepted artifact as `releases/<version>-cross-spec-review.md`.
