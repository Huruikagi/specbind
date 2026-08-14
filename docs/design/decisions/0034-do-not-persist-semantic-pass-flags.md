# 0034: Do not persist semantic pass flags

Status: Accepted

## Context

Completion validation must assess requirements coverage, design alignment, integration of the tasks within the spec, and boundary integrity. A persisted object containing only fixed `passed` values would add no evidence beyond the accepted completion gate itself. Those values are agent judgments rather than independently replayable mechanical results, and adding or refining a validation dimension would otherwise force lifecycle-schema churn.

The detailed validation report remains useful during the run, especially for `NO-GO` remediation or `MANUAL_VERIFY_REQUIRED` guidance. That does not make every report heading part of current lifecycle metadata.

## Decision

- The integration-validation protocol must assess at least:
  - active requirements coverage
  - end-to-end alignment with the current design
  - integration of the spec's own completed tasks
  - boundary integrity
- All mandatory semantic dimensions must pass before the skill may produce a `GO` candidate for the Decision 0029 handshake.
- Accepted completion evidence stores no `semantic_checks` object, per-dimension `passed` flags, semantic summary, findings list, or duplicated `decision: GO` value.
- Presence of accepted completion evidence means that the mandatory semantic protocol passed for the recorded implementation revision.
- `NO-GO` and `MANUAL_VERIFY_REQUIRED` reports retain their semantic findings only in run-scoped output under Decision 0030. Durable implementation knowledge may be distilled into `implementation-notes.md`.
- Decision 0033 mechanical command evidence remains persisted because it records concrete invocations and results rather than repeating the meaning of gate acceptance.
- Contract impact and downstream-review data are outside this decision and are persisted once in the project-state artifact under Decisions 0050 and 0052.

## Consequences

- Completion metadata stays compact and avoids tautological success flags.
- Semantic validation can evolve without adding a new schema field for every review dimension.
- The agent protocol remains strict even though its individual successful judgments are not persisted.
- Status treats the accepted completion record itself as the semantic `GO`; it does not reconstruct a checklist from redundant fields.
