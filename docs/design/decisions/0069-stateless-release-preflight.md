# 0069: Make release preflight a stateless readiness check

Status: Accepted

Implementation status: the Rust CLI exposes the read-only command, aggregates lifecycle, scope, review, gate, task, archive, managed-path, and target-only Git diagnostics, and shares the resulting readiness judgment with `milestone status`. No readiness evidence or mutation authority is persisted.

## Context

Release orchestration needs a deterministic check before an agent begins project-specific preparation, publication, or verification work. That check should catch lifecycle and target-path problems early, but project release work may legitimately advance `HEAD`, alter unrelated files, or take enough time that any earlier snapshot becomes stale. Finalization must therefore re-evaluate current state rather than trust a preflight token.

## Decision

### Command and checks

- The accepted read-only command is:

  ```text
  specbind release preflight
  ```

- Preflight discovers the active milestone and validates every deterministic release prerequisite available before project-specific release work, including:
  - a valid active roadmap, milestone identity, and concrete target release version
  - the complete participating-spec set and completed direct-change items
  - valid spec lifecycle state, required artifacts, completed tasks, approvals, fresh gate evidence, and accepted completion evidence
  - accepted and fresh roadmap-owned contract review state when Spec-backed work exists
  - archive destination collision rules
  - the currently resolved finalization mutation set and its Decision 0064 target-path Git safety
- Preflight does not require Decision 0068 log-entry input. The agent authors those summaries after it has judged the delivered release work.
- Preflight never creates, edits, deletes, moves, stages, commits, or archives a file.
- A successful default result is concise English under Decision 0067:

  ```text
  OK RELEASE_READY: Release v1.4.0 is ready for project release work across 3 specs.
  ```

- After the outcome line, the concise text result may list the milestone ID, release version, ordered participating-spec identities, direct-change count, and resolved mutation targets when those details help the agent begin release work.
- Any failed check returns a stable English diagnostic, performs no mutation, and prevents the release skill from starting applicable adapter work.

### No preflight authority

- Preflight does not persist a result, issue a session or plan ID, create a fingerprint, or produce evidence for `spec.yaml`, the roadmap, or release history.
- `specbind release finalize` accepts no preflight token or preflight-result file.
- Finalization independently rediscovers the active milestone, resolves its mutation targets, and reruns every applicable core invariant against current artifacts and Git state before mutating anything.
- A successful preflight is therefore an early readiness diagnosis, not authorization to finalize. Changes after preflight may cause finalization to fail and be retried after reconciliation.
- V1 has no finalization force bypass. Preflight remains read-only and never becomes mutation authority.

## Consequences

- Agents can detect core blockers before performing external release work without introducing durable run state.
- Long-running or repository-mutating project release procedures do not create false stale-token guarantees.
- Finalization remains the sole authoritative mutation boundary and stays safe when conditions change after preflight.
- The release skill consumes the concise text result and stable code in v1.
