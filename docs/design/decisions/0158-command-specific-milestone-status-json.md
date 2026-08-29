# 0158: Add command-specific JSON output for Milestone status

Status: Accepted

## Context

[Decision 0157](./0157-command-specific-spec-status-json.md) establishes a
small typed integration foothold through `spec status --json` while retaining
[Decision 0074](./0074-defer-json-cli-output.md)'s deferral of a general JSON
protocol. A surrounding tool also needs one project-wide entry point before it
can decide which Spec status to inspect.

`milestone status` already composes the active Roadmap, participating Specs,
Contract Review, Direct progress, Git state, actionable work, and release
blockers through one authoritative read model. Adding its existing model as the
second command-specific projection completes a useful project-to-Spec status
pair without extending JSON across unrelated command families.

## Decision

V1 adds:

```text
specbind milestone status --json
```

- `--json` applies only to `milestone status`. It is not a global option.
- Omitting it preserves the existing text output byte-for-byte.
- Text and JSON resolve the same `MilestoneStatusModel`.
- Success uses the Decision 0157 minimal envelope with `status: "ok"`, stable
  code `MILESTONE_STATUS_REPORTED`, and typed `data`.
- `data` reports the Milestone identity, target release, stage, health,
  Contract Review, Spec state counts, Direct progress, current and baseline
  revisions, ordered items and dependency waits, actionable work, current and
  release blockers, and diagnostics.
- `releaseReadinessEvaluated` distinguishes an empty evaluated blocker set from
  a stage where release readiness is not yet evaluated. `releaseBlockers` is
  `null` in the latter case.
- No active Milestone is a successful no-change result:

  ```json
  {"status":"no_change","code":"NO_ACTIVE_MILESTONE","data":null}
  ```

- Command failure uses the Decision 0157 error shape on stdout, keeps stderr
  empty, and preserves the nonzero exit status.

The response follows Decision 0157's compatibility boundary: executable
versioning, no independent response version or published schema, additive
fields allowed within a major, and consumers ignoring unknown fields.

Decision 0074 continues to defer JSON for all other commands and any common
cross-command response infrastructure. The two status commands may share
small serialization helpers inside their existing CLI module; they do not
establish a public generic response type.

## Consequences

- A local integration can begin with the whole Milestone and drill into a
  selected Spec without parsing text or artifacts.
- V1 still supports only two demonstrated command-specific JSON projections.
- Absence, success, and failure are mechanically distinguishable without a
  broader CLI protocol.

## Implementation status

Implemented. `milestone status --json` serializes the existing read model and
has coverage for active, absent, and invalid Milestone results while the
default text tests remain unchanged.
