# 0170: Carry phase-owned deferred findings through Design validation

Status: Accepted

## Context

Decision 0120 permits an unapproved Design and its Contract to remain dirty
while an independent validator reads them. It excludes every other path so the
orchestrator cannot hide unrelated work inside the one pre-approval handoff.

The Design and Design-validation Skills also follow the project's active
deferred-findings adapter after a non-blocking finding receives a `DEFERRED`
disposition. That write is a legitimate output of the same Design phase, but
the existing dirty-set rule classifies it as unrelated and stops before the
validator runs. The phase therefore cannot both preserve a real deferred
finding and satisfy its orchestration contract.

## Decision

The unapproved-Design handoff may additionally contain the exact
project-relative destination named by the active deferred-findings adapter,
but only when the Design author or independent validator actually recorded a
`DEFERRED` finding from this Design phase.

The Design author reports the finding, the adapter selector, and the exact path
written. The orchestrator verifies the adapter is active and names that same
destination before admitting it to the dirty set. It never infers a destination
from a conventional filename, admits another adapter output, or treats existing
unrelated dirt at the destination as phase-owned.

The validator remains read-only with respect to Design, Contract, lifecycle
state, and every other project path. After returning its verdict, it may append
its own deferred finding only to the same verified adapter destination and
reports whether it wrote that path. A blocking or resolved finding creates no
deferred write.

After a `READY` verdict, the delegated Design approval dispatch owns one phase
checkpoint containing the Design set, Contract, Design-gate state, and the
verified deferred destination when present. The normal clean handoff remains
mandatory before Contract Review. The orchestrator never creates that
checkpoint itself.

This is not a general dirty-worktree exception. Any unreported path, inactive
or mismatched adapter destination, earlier-phase artifact, generated output,
other Spec, or pre-existing unrelated change still stops orchestration.

## Consequences

- A real non-blocking Design finding survives without deadlocking mandatory
  independent validation.
- The adapter remains project-owned closed-world authority for the destination.
- One Design approval checkpoint closes the complete phase-owned write set.
- Unrelated work retains the same fail-closed handling as Decision 0120.

## Verification

Focused Skill contract tests require the exact adapter-derived exception and
the approval-dispatch checkpoint ownership. HP1 exercises the composed path
from planning through Design validation, Contract Review, and Tasks approval.
