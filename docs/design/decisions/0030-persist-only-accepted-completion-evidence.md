# 0030: Persist only accepted completion evidence

Status: Accepted

## Context

Integration validation can return `GO`, `NO-GO`, or `MANUAL_VERIFY_REQUIRED`, and may require several attempts before a spec is ready. Persisting every candidate, failed command, remediation report, and manual-verification gap in `spec.yaml` would turn current lifecycle metadata into an append-only validation log and make the authoritative evidence difficult to identify.

SpecBind needs to explain why the current active change is `release_ready`, not preserve every unsuccessful attempt. Detailed command output and CI history already have more appropriate run-scoped or external storage, while durable implementation knowledge has `implementation-notes.md`.

## Decision

- `spec.yaml` stores at most the currently accepted completion-gate evidence for the active change.
- Only a `GO` candidate that passes the complete Decision 0029 CLI handshake is persisted.
- Accepted evidence is written atomically with the `IMPLEMENTATION_VALIDATED` transition and records the Decision 0036 timezone-qualified RFC 3339 `passed_at` for that accepted revision.
- `NO-GO`, `MANUAL_VERIFY_REQUIRED`, preflight failure, stale-input rejection, and malformed candidate evidence do not mutate `spec.yaml` or any lifecycle state.
- Candidate evidence remains in the validation run context until acceptance. It is not a separately persisted SpecBind artifact and does not have its own lifecycle event.
- Validation output reports failures, missing manual checks, ownership, and remediation to the caller. CI systems, agent task logs, or other project tooling may retain that output independently of SpecBind metadata.
- A durable implementation lesson may be written to `implementation-notes.md`; a task that cannot proceed uses structured `blocked` state. Neither mechanism is a validation-attempt log.
- When completion is invalidated, the accepted completion evidence is cleared. A later successful handshake writes one new current record rather than appending attempt history.
- Release finalization summarizes the accepted validation result in the per-spec `log.md` and release history as already required; it does not migrate failed attempts into release artifacts.
- The completion evidence schema contains no failure counter, attempt array, rejected candidate, remediation transcript, raw command output, or conversation transcript. Decision 0033 retains only concise successful command metadata, while Decision 0034 excludes tautological semantic pass flags.

## Consequences

- Presence of completion evidence has one meaning: the active change has a currently accepted completion gate.
- Failed validation is retryable without producing metadata churn or stale candidate records.
- `spec.yaml` remains a compact current-state artifact rather than an operational log.
- Projects that require audit retention can preserve CI or agent-run artifacts without changing SpecBind's core lifecycle schema.
