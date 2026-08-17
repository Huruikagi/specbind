# 0088: Expose Spec gate approval and invalidation commands

Status: Accepted

## Context

Decisions [0012](./0012-delegated-approval.md), [0017](./0017-requirements-gate-inputs.md), [0032](./0032-gate-local-freshness-chain.md), [0036](./0036-rfc3339-gate-timestamps.md), [0038](./0038-design-gate-inputs.md), [0039](./0039-minimal-tasks-gate-evidence.md), and [0040](./0040-state-gate-evidence-invariants.md) already fix the approval modes, the per-gate fingerprint input sets, freshness chaining, and the cumulative evidence invariants. The [Spec state machine](../spec-state-machine.md) fixes the guards and effects for `REQUIREMENTS_APPROVED`, `DESIGN_APPROVED`, `TASKS_APPROVED`, and their `*_CHANGED` invalidations. The versioned wire models already carry explicit and delegated variants of all three gate evidence objects.

No public command writes any of them. [Decision 0086](./0086-completion-cli-handshake.md) exposed the completion handshake and [Decision 0087](./0087-milestone-review-cli.md) exposed the milestone-owned review, but both explicitly defer the earlier lifecycle gates. The result is a lifecycle whose middle is unreachable: a Spec can only arrive in `implementation` if `spec.yaml` gate evidence is authored by hand, which is exactly what this repository's own test fixtures do.

This decision fixes the public command surface only. It changes no accepted evidence shape, input projection, or transition guard.

## Decision

### Commands and ownership

The accepted commands are:

```text
specbind spec requirements approve <spec> --approval-mode <explicit|delegated> --requirement-ids <ids> [--delegation-workflow <workflow>]
specbind spec design approve <spec> --approval-mode <explicit|delegated> [--delegation-workflow <workflow>]
specbind spec tasks approve <spec> --approval-mode <explicit|delegated> [--delegation-workflow <workflow>]
specbind spec requirements invalidate <spec>
specbind spec design invalidate <spec>
specbind spec tasks invalidate <spec>
```

- The gate segment reuses the Decision 0040 evidence key vocabulary, completing a uniform `specbind spec <gate> <operation> <spec>` surface alongside the accepted `spec completion preflight|accept|invalidate`.
- `<spec>` is one canonical Spec identity. These commands never infer a Spec from the current directory and never operate on every Spec implicitly.
- Each command requires the Spec to be a current participant of the active Roadmap whose `active_change.milestone_id` matches that Roadmap.
- The three `invalidate` commands emit the state machine's `REQUIREMENTS_CHANGED`, `DESIGN_CHANGED`, and `TASKS_CHANGED` events. The public verb stays `invalidate` for consistency with `spec completion invalidate`; the event names are unchanged.
- V1 adds no gate-approval status command, because `spec status` already reports declared state, per-gate freshness, and coverage. It adds no bulk, all-Spec, or milestone-wide approval, and no `-y` alias under Decision 0012.

### Approval input transport

Approval inputs are command flags rather than a strict JSON candidate document.

- Decision 0086 uses `--evidence` and Decision 0087 uses `--candidate` because their payloads are genuinely document-shaped: ordered mechanical command text and free-form Markdown assessment. Every gate approval input here is instead a constrained token — canonical numeric Requirement IDs and a workflow identifier — so a document adds a file-handling and safety boundary without buying transport safety.
- `--approval-mode` is required for all three approvals and has no default. Absence of a terminal, `--non-interactive`, and script invocation never imply approval authority under Decision 0012.
- `--delegation-workflow` is required when `--approval-mode` is `delegated` and rejected when it is `explicit`, matching the conditional `delegation_workflow` field.
- `--requirement-ids` is accepted only by `requirements approve`. It is a comma-separated list of canonical Requirement IDs with no surrounding whitespace. There is no repeatable single-ID form, so one canonical invocation shape exists.
- The CLI validates that every submitted ID is canonical, unique, and present in the discovered Requirements artifact, and rejects an empty set. It stores the deterministic order it derives under Decisions 0018 and 0040; caller order carries no meaning.
- `design approve` and `tasks approve` accept no IDs, paths, or fingerprints. Their complete input sets are derived by the CLI under Decisions 0038 and 0039.
- The CLI owns `passed_at` under Decision 0036. No command accepts a timestamp, fingerprint, milestone identity, or semantic pass flag.

### Guards

Every command revalidates the following immediately before mutation and performs no partial write:

- the configured project and SpecBind roots resolve, the active Roadmap parses, and the target Spec is a current participant with matching milestone identity
- `spec.yaml` is structurally and semantically valid under Decision 0040; a contradictory file is reported for repair and never advanced or completed with manufactured evidence

Each approval additionally enforces its state machine guard:

| Command | From state | Additional guards | To state |
| --- | --- | --- | --- |
| `requirements approve` | `requirements` | valid Requirements artifact; submitted active set non-empty, unique, canonical, and existing | `design` |
| `design approve` | `design` | requirements gate fresh under Decision 0032; the singleton Contract and at least one Design artifact present; Decision 0061 traceability holds; the complete Design set covers every active Requirement ID | `tasks` |
| `tasks approve` | `tasks` | requirements and design gates fresh; valid `tasks.yaml`; every active Requirement ID mapped to an executable Task; the milestone-owned cross-spec review fresh through the existing Decision 0078 Tasks-approval boundary guard | `implementation` |

Gate approval is not revision-bound. Unlike the Decision 0029 and 0086 completion handshake it requires no clean worktree and accepts no implementation revision, because no accepted decision binds an approval to a Git commit. It applies Decision 0081 path safety to the target `spec.yaml` only.

### Results

Approval follows the Decision 0067 text contract:

```text
OK SPEC_REQUIREMENTS_APPROVED: Approved requirements for spec checkout.
  State: design
  Approval mode: explicit
  Passed at: 2026-08-16T10:00:00Z
  Approved requirement IDs: 3
```

- `design approve` and `tasks approve` return `OK SPEC_DESIGN_APPROVED` and `OK SPEC_TASKS_APPROVED` with the same `State:`, `Approval mode:`, and `Passed at:` details and no approved-ID count.
- A `delegated` result adds a `Delegation workflow:` detail.
- `NO_CHANGE SPEC_<GATE>_ALREADY_APPROVED` applies only when the Spec already holds the corresponding post-state and identical fresh evidence: same approval mode, same delegation workflow, same approved Requirement ID set where applicable, and unchanged input revisions. Stale, missing, or contradictory evidence is never treated as identical and must be invalidated before a new approval.
- Every other outcome returns `ERROR SPEC_<GATE>_APPROVE_FAILED`, exits nonzero, emits the underlying stable diagnostics, and leaves `spec.yaml` unchanged. State, participation, freshness, traceability, coverage, review, and target-path failures stay distinguishable rather than collapsing into one semantic verdict.

### Invalidation semantics

- Each `invalidate` applies its state machine effect exactly: `requirements invalidate` sets `requirement_ids` to `null` and clears requirements, design, tasks, and completion evidence; `design invalidate` clears design, tasks, and completion evidence; `tasks invalidate` clears tasks and completion evidence.
- Each rewinds the declared state to its own gate and retains downstream documents as stale repair input. No command deletes Requirements, Design, Contract, or `tasks.yaml` content.
- `requirements invalidate` and `design invalidate` additionally remove the accepted `state/cross-spec-review.md` under Decision 0078, because the milestone review is accepted between Design approval and Tasks authoring and cannot survive a rewind past it. `tasks invalidate` keeps the review, which is still required and still valid at the `tasks` state. The removal happens before the Spec is rewound, so an interruption leaves a milestone that visibly needs a new review rather than a rewound Spec behind a review still claiming to cover it.
- Invalidation permits unrelated dirty project paths because it exists to reconcile later work, but refuses a dirty or staged target `spec.yaml`, matching Decision 0086.
- A mutation returns `OK SPEC_<GATE>_INVALIDATED`. `NO_CHANGE SPEC_<GATE>_NOT_APPROVED` applies only when the Spec already sits at or before that gate with no evidence to clear. Other failures return `ERROR SPEC_<GATE>_INVALIDATE_FAILED`.

### Agent and output boundary

- The requirements, design, and tasks skills own semantic review, active-scope selection, user-facing explanation, and bounded remediation. They invoke `approve` only after reaching an approved conclusion, and supply `delegated` only from an intentional accelerated run context under Decision 0012.
- The CLI owns discovery, validation, fingerprinting, freshness and review guards, timestamping, persistence, concise English results, and process exit status.
- Delegation run context remains orchestration state and is never persisted beyond the recorded `approval_mode` and `delegation_workflow` fields.
- V1 returns no general JSON response under Decisions 0067 and 0074.
- `SPEC_CREATED` and `CHANGE_STARTED` are fixed by [Decision 0089](./0089-milestone-creation-cli.md) as effects of milestone creation and scope update. `SPEC_SCOPE_REMOVED` and `MILESTONE_ABANDONED` remain a separate lifecycle CLI decision. The standalone `check` command vocabulary remains unaccepted under Decision 0087.

## Consequences

- The complete per-Spec lifecycle from `requirements` to `release_ready` becomes reachable through public CLI surfaces, so generated skills stop needing to author `spec.yaml` evidence.
- The repository's own fixtures can construct lifecycle state through the same commands consumers use, which removes a standing source of drift between hand-written evidence and the accepted contract.
- Approval and invalidation stay symmetric across all four gates, so a stale gate is repaired by an explicit rewind rather than by overwriting evidence in place.
- Requiring `--approval-mode` makes an unattended script unable to claim explicit user approval by omission.
- The existing Decision 0078 Tasks-approval boundary guard gains its intended caller, so the cross-spec review becomes enforced at the point it was designed for rather than only at later boundaries.

## Implementation status

Implemented. `tools/specbind/src/approval.rs` owns the six guarded transitions. Approval validates the request, resolves the active Roadmap, participation, milestone identity, declared state, and prior-gate freshness, then applies each gate's own guards: the requirements gate checks the submitted selection against the discovered Requirements catalog and stores the deterministic order it derives; the design gate requires the Contract and at least one Design artifact plus clean Requirement traceability; the tasks gate requires a valid plan, complete Task coverage, and the existing Decision 0078 Tasks-approval review boundary. Every approval re-resolves its inputs immediately before mutation, owns `passed_at`, writes the cumulative sparse evidence container, clears later evidence, and revalidates the mutated document before atomically replacing `spec.yaml`. Identical fresh approvals return the no-change result. Invalidation rewinds the declared state, clears exactly the cumulative downstream keys, drops an emptied evidence container, nulls `requirement_ids` for the requirements gate, removes the accepted cross-spec review for the requirements and design gates, and refuses a dirty or staged target. Approvals are not revision-bound and require no clean worktree.
