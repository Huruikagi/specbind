# 0090: Expose standalone traceability and contract check commands

Status: Accepted

## Context

[Decision 0011](./0011-cross-spec-contract.md) and the [CLI and agent boundary](../cli-agent-boundary.md) proposed a deterministic `specbind check` family as the first mechanical replacement for ad hoc grep and shell inspection. The Rust read models are complete: the traceability resolver checks Requirement existence plus active Design and Task coverage, and the Contract graph resolves the project-wide dependency set while separating structural errors from review warnings.

[Decision 0087](./0087-milestone-review-cli.md) deliberately left that vocabulary unaccepted so the milestone review surface could land first. It is now the last read-only gap: both read models are reachable only indirectly through `spec status` and the guarded review operation, neither of which is a focused pass or fail gate.

The boundary document also sketched a `PASS`/`FAIL` output shape that predates [Decision 0067](./0067-text-first-english-cli-results.md), and a `<spec-path>` argument that predates the canonical Spec identity model. This decision fixes the accepted surface and supersedes both sketches.

## Decision

### Commands and ownership

The accepted commands are:

```text
specbind check traceability <spec>
specbind check contracts
```

- Both are read-only. They create, edit, and delete nothing, and they are never mutation authority.
- `<spec>` is one canonical Spec identity, consistent with every other Spec-scoped command. A filesystem path is not accepted.
- `check contracts` evaluates the complete current persistent Contract set. V1 accepts no scope filter; a narrower selection would need its own vocabulary and adds nothing while the graph read model is already project-wide.
- Neither command requires an active milestone. `check traceability` is valid in any lifecycle state, and an absent `tasks.yaml` before the `tasks` state is normal rather than a failure.
- These commands do not replace `spec status`. Status composes lifecycle, freshness, coverage, and progress and reports a readable Spec successfully even when it is inconsistent. `check` is the focused gate whose exit status encodes the verdict.

### Traceability result

A passing check returns `OK TRACEABILITY_VERIFIED`, exits zero, and reports the counts that make the verdict auditable:

```text
OK TRACEABILITY_VERIFIED: Verified traceability for spec checkout.
  Requirements: 24
  Active requirement IDs: 6
  Design coverage: 6/6
  Task coverage: 6/6 (required)
```

- `Requirements` is the complete current catalog; `Active requirement IDs` is the Decision 0003 active scope. An idle Spec reports `none` for the active scope and omits coverage ratios.
- `Task coverage` states whether coverage is currently required, so a Spec before the `tasks` state is not mistaken for missing coverage.
- Any unknown Design or Task reference, missing active-scope coverage, or unavailable required artifact returns `ERROR TRACEABILITY_FAILED`, exits nonzero, and emits the existing stable `TRACEABILITY_*` and discovery diagnostics as details.

### Contract result

A passing check returns `OK CONTRACTS_VERIFIED`, exits zero, and reports the resolved graph:

```text
OK CONTRACTS_VERIFIED: Verified 4 contract(s) and 7 dependency reference(s).
  Ownership findings: 1
  Dependency cycles: 0
  Warnings:
    - CONTRACT_GRAPH_FILE_OWNERSHIP_OVERLAP specs/cart/contract.md: ...
```

- Structural failures — an unavailable or invalid Contract, a duplicate entry ID, a missing manifest, or a dangling reference — return `ERROR CONTRACTS_FAILED`, exit nonzero, and emit their stable diagnostics.
- Ownership overlaps and dependency cycles remain review warnings under Decision 0011. They are reported in the successful result and do not change the exit status, because the accepted decision makes them agent judgment rather than unconditional structural failure. A future decision may add an escalation flag; v1 does not invent one.
- The command reports the graph it resolved rather than asserting semantic compatibility, which stays with the contract review skill.

### Exit and output contract

- Stable result codes and exit status are part of the v1 contract, which is what makes these commands usable as a continuous check outside an agent session.
- Zero exit means the deterministic check passed; nonzero means it failed. Warnings never change the exit status.
- Results follow Decision 0067. V1 returns no JSON response under Decision 0074, so a consumer reads the stable code and exit status.
- Diagnostics keep their existing codes and source locations. These commands add no new diagnostic vocabulary of their own beyond the two outcome codes per command.

## Consequences

- The complete Requirements-to-Design-to-Tasks and Contract-graph read models become directly usable, closing the last read-only gap in the v1 CLI surface.
- A project can run SpecBind's deterministic checks on every push without an agent session, which is the prerequisite for publishing a reusable CI action.
- The `PASS`/`FAIL` sketch and `<spec-path>` argument in the boundary document are superseded by the Decision 0067 contract and canonical Spec identity.
- Keeping warnings non-fatal preserves Decision 0011's distinction between mechanical failure and review judgment, at the cost of requiring a later decision if a project wants them to block.

## Implementation status

Implemented. `check traceability <spec>` resolves the discovery inventory and traceability report, fails closed with `ERROR TRACEABILITY_FAILED` on any inventory or traceability diagnostic, and otherwise reports the catalog size, active scope, and Design and Task coverage ratios with the current requirement state. An idle Spec reports `none` for the active scope and omits the ratios. `check contracts` resolves the project-wide graph, treats project, per-Spec inventory, and error-severity graph diagnostics as `ERROR CONTRACTS_FAILED`, and otherwise reports the contract and dependency counts with ownership findings, dependency cycles, and warning details at zero exit.
