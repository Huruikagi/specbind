# 0070: Derive release readiness without a new evidence artifact

Status: Accepted

## Context

Release preflight needs a complete readiness judgment, but SpecBind already persists the lifecycle facts that make that judgment mechanically possible. Adding a milestone-level `release_readiness` object would duplicate per-spec gate evidence, completion evidence, task state, direct-change status, and roadmap-owned contract review state. Project-specific publication and verification results are also too varied for a useful universal evidence schema and are judged by the agent and human under Decision 0066.

## Decision

- SpecBind defines no separate persisted release-readiness object, aggregate evidence record, release candidate file, or external release-result schema in v1.
- The CLI derives core release readiness from the current authoritative artifacts and project state, including:
  - per-spec lifecycle state, gate evidence, accepted completion evidence, active Requirement IDs, and task completion
  - roadmap membership, release binding, milestone identity, and completed direct-change items
  - the accepted roadmap-owned contract review when the milestone contains Spec-backed work
  - required artifact validity, archive collision rules, and finalization target-path safety
- `specbind release preflight` reports that derived judgment without persisting it under Decision 0069. It does not consolidate the source facts into another artifact.
- Project-specific Prepare, Publish, and Verify results remain in the active agent run context and the human-agent release judgment. They are not submitted to or archived by the CLI as universal structured evidence.
- Invoking `specbind release finalize` is an explicit request to perform the release lifecycle transition after the caller has judged applicable external work complete. The invocation is not proof that the CLI observed that work.
- Decision 0068 per-spec summaries are mutation content for `log.md`, not readiness or publication evidence.
- Project adapters may create project-owned tags, release records, deployment identifiers, or audit artifacts. SpecBind core neither requires nor interprets them unless a later project-integration contract explicitly adopts them.

## Consequences

- There is one authoritative copy of each mechanically verifiable readiness fact.
- Preflight and finalization can recompute current readiness instead of reconciling a stale aggregate record.
- Projects retain freedom to use their own publication evidence without forcing unrelated projects into the same schema.
- Run-scoped external results disappear with the run unless the project adapter deliberately records them in a project-owned system.
