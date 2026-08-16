# 0086: Define the completion CLI handshake

Status: Accepted

## Context

Decision 0029 accepts a two-call completion-validation handshake but leaves its public command names open. Decision 0080 applies the same clean-revision principle to milestone-owned Direct items, while Decision 0082 requires all implementation work to converge before final Spec validation begins.

The CLI surface must keep semantic validation in the agent skill, keep mutation authority in Rust, work reliably from PowerShell and POSIX shells, and support revalidation after previously accepted completion evidence becomes stale. Repeated shell arguments are a poor transport for ordered commands, working directories, and multilingual text.

## Decision

### Spec completion commands

The accepted commands for a Spec-backed item are:

```text
specbind spec completion preflight <spec>
specbind spec completion accept <spec> --evidence <path|->
specbind spec completion invalidate <spec>
```

- `<spec>` is one canonical Spec identity. Completion commands never infer a Spec from the current directory and never operate on every Spec implicitly.
- `preflight` is read-only. It requires the Spec to be in `implementation`, every participating milestone item to be implementation-complete, the project worktree to be clean at a committed `HEAD`, the Spec task plan to be complete and unblocked, all prior gates to be fresh, and the applicable milestone-owned cross-spec review to be fresh.
- A successful preflight returns `OK SPEC_COMPLETION_PREFLIGHT_READY` and the full Git commit object ID on one `Implementation revision:` detail line. It returns no task-plan or artifact fingerprints.
- The validation skill runs mechanical checks and the mandatory semantic protocol against that checkout. `NO-GO` and `MANUAL_VERIFY_REQUIRED` remain run-scoped and do not invoke `accept`.
- `accept` is the sole `IMPLEMENTATION_VALIDATED` mutation. It independently recomputes every preflight guard, requires the submitted full revision to equal current `HEAD`, atomically records completion evidence, and transitions the Spec from `implementation` to `release_ready`.
- The first acceptance at a revision requires a completely clean worktree. To make the Decision 0082 multi-Spec convergence barrier achievable without a batch evidence artifact, later acceptance calls may tolerate only uncommitted `spec.yaml` transitions for other participating Specs when Rust proves that each change is exactly `implementation` to `release_ready`, contains completion evidence bound to the same revision, and changes no earlier lifecycle input. The target Spec itself must still be unchanged before its mutation. The agent commits the accepted metadata set together; any other worktree change fails closed.
- A successful new acceptance returns `OK SPEC_COMPLETION_ACCEPTED`. An identical already-accepted current result returns `NO_CHANGE SPEC_COMPLETION_ALREADY_ACCEPTED`. A stale or contradictory `release_ready` Spec is not treated as identical and must be invalidated before a new handshake.
- `invalidate` is the explicit `COMPLETION_INVALIDATED` transition. It changes `release_ready` to `implementation`, removes only completion evidence, and preserves current Requirements, Design, Tasks, and their gate evidence. It requires those earlier gates to remain semantically valid; broader staleness must use the applicable earlier invalidation transition instead.
- Invalidation permits unrelated dirty project paths because it is used to reconcile later implementation work, but it refuses to overwrite a dirty or staged target `spec.yaml`. A `release_ready` Spec is rewound even when its completion evidence is missing or stale; `NO_CHANGE SPEC_COMPLETION_NOT_ACCEPTED` applies only when the Spec is already in `implementation` with no completion evidence. A mutation returns `OK SPEC_COMPLETION_INVALIDATED`.

### Completion candidate input

`--evidence` reads one strict UTF-8 JSON document. `-` reads standard input; a path must identify a repository-external ordinary file. The input is transient command data and is never copied, archived, fingerprinted, or deleted by SpecBind.

The version-1 document contains exactly:

```json
{
  "schemaVersion": 1,
  "implementationRevision": "0123456789abcdef0123456789abcdef01234567",
  "mechanicalChecks": [
    {
      "kind": "test",
      "command": "cargo test --workspace --all-features",
      "exitCode": 0,
      "workingDirectory": "tools/specbind"
    }
  ]
}
```

- The root permits only `schemaVersion`, `implementationRevision`, and `mechanicalChecks`; all are required.
- `schemaVersion` is the integer `1`.
- `implementationRevision` is the full preflight revision and is validated against the Git repository's object format. It is not accepted as an abbreviated hash, branch, tag, or timestamp.
- `mechanicalChecks` is the non-empty ordered Decision 0033 list. Each item permits only `kind`, `command`, `exitCode`, and optional `workingDirectory`, using the existing completion-evidence value rules. `exitCode` must be `0`.
- The CLI owns `passed_at`. The candidate cannot supply a timestamp, semantic pass flags, cross-spec-review data, artifact fingerprints, raw output, environment values, or retry history.
- The persisted `spec.yaml` evidence uses the canonical snake_case wire fields. The CLI decodes the transient camelCase input and writes only the accepted three-field Decision 0037 completion evidence.
- Invalid JSON, unknown fields, unsupported versions, unsafe command text, invalid working directories, and malformed revisions return `ERROR COMPLETION_EVIDENCE_INVALID` without mutation.

### Direct completion commands

Direct items remain owned by the active Roadmap. Their accepted commands are:

```text
specbind milestone direct preflight <direct>
specbind milestone direct complete <direct> --implementation-revision <revision>
```

- `<direct>` is one canonical Direct identity in the active Roadmap. The command rejects a Spec identity or an item from another or archived milestone.
- Direct preflight is read-only and requires the item to be pending, all implementation-phase predecessors to be complete, and the project to have a clean committed `HEAD`. It returns `OK DIRECT_COMPLETION_PREFLIGHT_READY` plus the full `Implementation revision:` detail.
- The implementation skill performs its run-scoped checks and semantic judgment. The CLI does not persist Direct mechanical evidence under Decision 0080.
- `complete` independently requires the same revision, cleanliness, active milestone identity, pending item, and dependency readiness, then atomically replaces only that Direct item's status with `completed`.
- A new mutation returns `OK DIRECT_COMPLETION_RECORDED`; an already-completed matching active item returns `NO_CHANGE DIRECT_COMPLETION_ALREADY_RECORDED`.
- A Direct item is reset to pending only by an explicit Roadmap scope/edit operation. Spec completion invalidation never changes Direct status.

### Failure and orchestration boundary

- Every guard failure follows the Decision 0067 English text result contract, exits nonzero, and performs no partial mutation. Specific diagnostics identify state, milestone membership, dependency, review, task, Git, evidence, or target-path failures; the top-level failure does not collapse those categories into semantic agent judgment.
- Neither preflight command runs project tests, accepts `GO`, or creates durable candidate state. Neither acceptance command claims that command text proves execution; the invoking skill owns that judgment.
- `specbind-validate-implementation` routes Spec-backed work through the three Spec completion commands. Direct implementation uses the milestone Direct handshake and does not create a synthetic Spec.
- Decision 0087 fixes the stable public cross-spec review acceptance and status commands, and [Decision 0088](./0088-gate-approval-cli.md) fixes the Requirements, Design, and Tasks gate approval and invalidation commands, so the complete generated validation workflow can rely only on public Rust CLI surfaces.

## Consequences

- The milestone `validation` stage now has an exact mutation path to fresh `release_ready` Specs.
- Revalidation is explicit and cannot silently overwrite stale accepted evidence.
- Structured candidate data remains shell-safe without introducing general JSON output.
- Spec and Direct completion share the same clean-revision guarantee while retaining their intentionally different durable evidence models.

## Implementation status

The Rust CLI implements all five accepted commands. Spec acceptance validates strict transient candidate JSON, converged Roadmap participation, completed Tasks, fresh prior gates and cross-spec review, exact Git revision, and the narrowly recognized same-revision multi-Spec metadata exception before atomically updating `spec.yaml`. Completion invalidation preserves fresh earlier gates and refuses a dirty target. Direct completion rechecks active Roadmap identity, implementation dependencies, clean revision, and atomically replaces only the selected sparse status. The completion freshness evaluator recognizes committed or pending completion-only transitions for multiple participating Specs bound to one implementation revision while rejecting any other project change.
