# 0087: Expose milestone-owned cross-spec review commands

Status: Accepted

## Context

Decision 0078 defines one accepted contract-first cross-spec review after every participating Design gate and before Tasks authoring. The Rust core can already resolve authoritative inputs, guard acceptance, persist the review, evaluate freshness, and enforce freshness at later lifecycle boundaries. However, the agent workflow has no stable public CLI command for submitting its semantic assessment or requesting a focused review summary.

The review is milestone-wide state rather than a per-Spec gate. A top-level `cross-spec-review` command would expose the storage name as a separate product domain, while placing acceptance under one Spec would misrepresent its complete persistent-Contract scope.

## Decision

### Commands and ownership

The accepted commands are:

```text
specbind milestone review status
specbind milestone review accept --candidate <path|->
```

- `review` is nested under `milestone` because the accepted artifact covers the active milestone and is stored once for the complete Spec-backed scope.
- Neither command accepts or infers an individual Spec identity.
- These commands do not replace `specbind milestone status`. The milestone status remains the ordinary aggregate view; `milestone review status` is the focused workflow and repair view.
- V1 adds no separate `review show`, raw state-artifact read, review invalidation, or review-history command. Existing explicit lifecycle rewinds own removal when required, and Git remains the audit history.

### Focused status

`status` is read-only. It resolves the configured project and SpecBind roots, parses the current Roadmap, and evaluates the canonical `state/cross-spec-review.md` through the same authoritative freshness read model used by later lifecycle guards.

For a valid readable project state, it returns `OK MILESTONE_REVIEW_STATUS_REPORTED`, exits zero, and renders these stable public status values:

- `not_applicable` for a Direct-only milestone with no active review artifact
- `absent` for a Spec-backed milestone with no accepted artifact
- `fresh` when milestone identity, Git baseline, required Contract set, declared deep inputs, and every persisted fingerprint match current authority
- `stale` when a structurally valid accepted artifact is no longer usable against current authority

The concise output contains the active milestone ID and status. When an accepted record is structurally available, it also contains `Passed at:` and `Inputs:` detail lines, where `Inputs:` is the persisted accepted-input count. Its shape is:

```text
OK MILESTONE_REVIEW_STATUS_REPORTED: Reported cross-spec review status for milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62.
  Status: fresh
  Passed at: 2026-08-16T10:00:00Z
  Inputs: 3
```

`not_applicable` and `absent` omit `Passed at:` and `Inputs:` because no accepted record exists. Stale diagnostics follow as indented stable-code details. The command does not print the free-form assessment body or fingerprint values in the ordinary summary.

`absent` and `stale` are successfully reported lifecycle facts, not command-execution failures. Later guarded mutations independently require the applicable fresh state. An invalid Roadmap, unreadable or structurally invalid review artifact, unsafe target type, or other failure that prevents a trustworthy read returns `ERROR MILESTONE_REVIEW_STATUS_FAILED`, exits nonzero, and includes owned diagnostics.

### Acceptance candidate

`--candidate` reads one strict UTF-8 JSON document. `-` reads standard input; a path must identify a repository-external ordinary non-symlink file. The input is transient command data and is never copied, archived, fingerprinted, or deleted by SpecBind.

The version-1 document remains the Decision 0078 shape:

```json
{
  "schemaVersion": 1,
  "assessment": "# Assessment\n\nThe current contracts are mutually compatible.",
  "deepInputs": [
    "specs/checkout#requirements",
    "specs/checkout#design/main"
  ]
}
```

- The root permits only `schemaVersion`, `assessment`, and `deepInputs`; all are required.
- `schemaVersion` is the integer `1`.
- `assessment` is non-empty Markdown containing the accepted semantic judgment.
- `deepInputs` is a JSON array of unique canonical Requirements or Design selectors. It may be empty, and array order has no semantic meaning. Contract and Roadmap selectors are never submitted because Rust always derives their complete required set.
- The candidate cannot supply paths, fingerprints, milestone identity, timestamps, classifications, or task-plan inputs.

`accept` is the sole public cross-spec review acceptance mutation. It validates the candidate, Roadmap, complete Contract graph, optional deep inputs, baseline, participating Spec states, Design freshness, and absence of current `tasks.yaml`; resolves every revision itself; repeats the authoritative guards immediately before mutation; owns `passed_at`; and atomically creates or replaces `state/cross-spec-review.md`.

A successful acceptance returns `OK MILESTONE_REVIEW_ACCEPTED`, exits zero, and reports the milestone ID, owned timestamp, and accepted input count:

```text
OK MILESTONE_REVIEW_ACCEPTED: Accepted cross-spec review for milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62.
  Passed at: 2026-08-16T10:00:00Z
  Inputs: 3
```

Every candidate, guard, race, or filesystem failure returns `ERROR MILESTONE_REVIEW_ACCEPT_FAILED`, exits nonzero, emits the underlying stable diagnostics, and leaves the previously accepted artifact unchanged. Reaccepting a currently fresh review is allowed because a revised semantic assessment or declared deep-input set may intentionally replace it.

### Agent and output boundary

- `specbind-cross-spec-review` owns semantic compatibility judgment, selection of material deep inputs, user-facing explanation, and bounded remediation. It invokes `accept` only after reaching an accepted conclusion.
- The CLI owns deterministic discovery, validation, fingerprinting, Git and lifecycle guards, timestamping, persistence, freshness evaluation, concise English results, and process exit status.
- V1 returns no general JSON response. Results follow Decisions 0067 and 0074 with stable `OK` or `ERROR` codes.
- The proposed standalone `specbind check contracts` and `specbind check traceability` vocabulary remains separate and unaccepted. This decision does not block exposing those read-only checks later.

## Consequences

- The review skill can complete its accepted workflow without editing state files or reproducing fingerprints in shell logic.
- Milestone ownership is visible in the command hierarchy and consistent with `milestone status`, Direct completion, and release readiness.
- A focused status command supports repair while aggregate milestone status remains compact.
- Missing or stale review state can be inspected without treating the read itself as a failed command, while invalid state still fails closed.
- Requirements, Design, and Tasks approval commands are fixed separately by [Decision 0088](./0088-gate-approval-cli.md).

## Implementation status

Implemented. The Rust CLI routes both milestone-owned commands without accepting a Spec identity. `status` renders the active milestone ID, the public `not_applicable`, `absent`, `fresh`, and `stale` values from the single authoritative freshness read model, and adds `Passed at:` and the persisted input count only when an accepted record is structurally available. It exits zero for those four states, prints neither the assessment body nor fingerprint values, follows stale and absent state with indented stable-code diagnostics, and fails closed with `ERROR MILESTONE_REVIEW_STATUS_FAILED` for structurally invalid state. `accept` reads one strict UTF-8 JSON candidate from standard input or a repository-external ordinary non-symlink file through the shared external-input boundary, delegates every guard and re-resolution to the core operation, reports the owned timestamp and accepted input count, and leaves any previously accepted artifact unchanged on failure. Reaccepting a fresh review remains allowed.
