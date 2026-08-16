# 0055: Keep cross-spec review inputs contract-first

Status: Accepted

Decision 0078 narrows the Roadmap projection to Spec-backed scope, removes Tasks from optional deep inputs, and removes classification input.

## Context

Global cross-spec review must discover consumers that are not already named in the active roadmap. Reading only changed producers cannot find a new or previously unrelated `Consumes` edge, so the deterministic contract graph must cover every current persistent spec.

At the same time, always fingerprinting every requirement, design, and task plan would undermine the purpose of concise contracts and make unrelated spec-local edits invalidate global review. Deep documents should become review inputs only when the final semantic judgment materially depends on them.

## Decision

- `state/cross-spec-review.md` frontmatter `input_revisions` is a non-empty flat mapping from canonical input key to a Decision 0016 fingerprint.
- Every accepted record contains the key `steering/roadmap.md#cross-spec-scope`.
- The cross-spec scope projection contains exactly:
  - `milestone_id`
  - `baseline_revision`
- Spec-backed `work_items`, including category, Spec identity, summary, and Spec-to-Spec dependencies
- Direct items, Direct status, and dependencies to or from Direct items are excluded.
- The projection excludes `type`, `target_release`, the roadmap Markdown body, unknown OKF extension fields, and the review artifact itself.
- Because Decision 0046 makes item and dependency list order non-semantic, normalization sorts spec items by canonical spec identity, direct items by roadmap-local ID, and each typed dependency list by kind then identity. It then serializes the typed projection through RFC 8785 JCS and computes SHA-256.
- Every current persistent spec contributes exactly one required `specs/<canonical-spec>#contract` logical key, resolved by the Decision 0057 OKF inventory, including specs outside active roadmap scope and canonical empty contracts. The key set itself is part of freshness, so adding or removing a contract makes an accepted review stale.
- A missing current contract prevents acceptance. Missing-contract fallback may provide safe migration diagnostics and review context, but it does not produce normal accepted v1 evidence.
- Contract fingerprints cover the complete OKF Markdown file after line-ending normalization, consistent with Decisions 0038 and 0045.
- Contract-first review adds no other input when the contract diff and complete current graph are sufficient for the final judgment.
- When the final judgment materially relies on deeper content, `input_revisions` may additionally contain only:
  - `specs/<canonical-spec>#requirements`
  - `specs/<canonical-spec>#design/<artifact_id>`
- Requirements and design use complete-file Markdown fingerprints after line-ending normalization. Task plans use the Decision 0028 normalized typed plan projection and JCS fingerprint; mutable execution state is excluded.
- A file that was merely opened or consulted incidentally is not an input. The review workflow declares every deeper artifact whose content materially supports the accepted conclusion, and its Markdown assessment explains why deep review was necessary.
- The agent submits canonical optional deep selectors and the candidate assessment to the guarded CLI operation. The CLI resolves current paths through type-based discovery, validates the allowed and required key set, reads the files, computes every fingerprint itself, and writes the accepted artifact. Agent-supplied paths and hash values are not accepted as authority.
- Logical selectors use the Decision 0057 forms. Fixed machine projections remain SpecBind-root-relative POSIX paths with no `.` or `..` segments. CLI-generated YAML writes the roadmap projection first, required contracts in canonical spec identity order, then optional deep inputs ordered by spec identity, artifact kind, and collection ID. Mapping order is presentation only.
- Any current fingerprint mismatch, required-key-set change, missing declared deep input, stale applicable active-spec prerequisite gate, or invalid Decision 0054 baseline makes the global review unusable until the responsible state is repaired and a new review passes.

## Consequences

```yaml
input_revisions:
  steering/roadmap.md#cross-spec-scope: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
  specs/account-auth#contract: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
  specs/checkout#contract: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
  specs/checkout#requirements: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
  specs/checkout#design/persistence: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

- The CLI always reads all contracts, but ordinary agent context can begin with the compact graph and changed entries rather than every raw file.
- Most accepted reviews contain only the roadmap projection and contract set.
- Ambiguous cases remain reproducible because the particular deep documents used by the AI judgment become freshness inputs.
- Spec-local gate evidence remains the authoritative owner of ordinary requirements, design, and task approval; cross-spec review duplicates a revision only when its own semantic conclusion directly depends on that content.

## Implementation status

The Rust Contract graph resolver now enumerates every immediate persistent Spec directory and requires one valid discovered singleton Contract from each. It retains each per-Spec partial inventory, resolves valid Consumes targets against the complete typed Contract set, and reports missing Specs, unavailable Contracts, and missing target entries mechanically. The review-input resolver fingerprints the normalized Roadmap scope and complete Markdown of every valid Contract, accepts only canonical Requirements or Design deep selectors, resolves their current paths through the inventories, and computes every revision itself. Baseline Git ancestry, participating-Spec Design freshness, re-resolution at mutation time, and the guarded accepted-review write remain the persistence increment.
