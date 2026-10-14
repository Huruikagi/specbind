# 0218: Bound Task recovery by progress and three cycles

Status: Accepted

## Context

Issue #76 exposed independent diagnosis and repair budgets stopping a Task even
after planning had resolved an earlier finding. The maintainer chose a fixed
three-cycle limit without configuration, with continuation only when prior
findings are resolved and the remaining findings are materially new.

This amends the Spec-backed failure bounds in Decisions 0075, 0110, and 0195
and the recovery continuity in Decision 0199. Direct-item bounds and the
independent Design and Contract Review budgets are unchanged.

## Decision

### One Task recovery budget

The initial implementation and review do not consume recovery. A Task has at
most three recovery cycles in the same continuing work, across implementation,
Drive, and authorized replanning. Each cycle comprises diagnosis where needed,
one corrective action by its owner, and independent reassessment. Corrective
actions include implementation repair, authorized plan repair, and a corrected
review after a diagnosed review-scope defect. There is no separate Task
diagnosis or review-repair allowance and no configurable limit.

Reserve the next cycle immediately before the corrective owner starts. A
receiver that never starts consumes no cycle; an owner that starts and fails
during preparation consumes it. One initial assessment and one reassessment
after each cycle bound diagnostic dispatches to at most four. Do not perform
another diagnosis at unchanged evidence. The final reassessment may explain a
failure but cannot authorize a fourth corrective cycle.

Nested planning retains its own stricter phase budgets; it counts as one Task
recovery cycle, not one per internal phase dispatch. Other Tasks have their own
budgets. A renamed, split, or remapped Task must carry the original recovery
identity and shared remaining allowance for the same unresolved obligation.

### Progress is independently established

Keep a run-local finding ledger with stable IDs, affected obligations, evidence,
and BLOCKING or RESOLVED dispositions. A fresh debugger or reviewer owns
semantic finding identity and resolution, reading current artifacts, changes,
and verification evidence. Pass factual finding history, not failed attempt
reasoning or the implementer's desired verdict. Renaming or moving a symptom
does not make a new finding. Missing or ambiguous mappings stop recovery.

The first blocking assessment may authorize cycle one. After a cycle:

- no blockers permits the ordinary required verification/review and completion;
- any previously blocking finding still BLOCKING, or a reopened resolved
  finding, stops the Task even with cycles remaining;
- only all prior findings RESOLVED plus materially new blocking IDs permits
  another cycle, and only when fewer than three cycles were used;
- after cycle three, any remaining blocker stops the Task.

Independent reassessment is required for recovery even with inline or off
Task review. It does not replace the selected Task review or authorize completion
from a diagnosis. A review-scope diagnosis can lead only to fresh review.

Preparation errors use this same rule. There is no free preparation retry. If
preparation failed before the previous defect could be repaired, that defect
is still unresolved and recovery stops. A new preparation finding can continue
only with evidence that all prior findings were resolved. Credentials, denied
permissions, unavailable external input, scope changes, and Requirements
changes keep their existing boundaries regardless of remaining cycles.

### Continuation carries facts, not new allowances

Implementation owns the Task ledger. Drive transports it, and Plan returns its
Task identity mapping and repair evidence without declaring implementation
findings resolved. The resumed implementation owner obtains independent
reassessment before another corrective cycle. A passing plan approval alone is
not proof that every Task blocker is resolved.

Every recovery handoff reports the recovery identity and current Task mapping,
cycles used and remaining, all finding IDs/dispositions/evidence, current
assessment, exact partial paths, supplied authority, and next owner. These are
conversation handoff data, not new project files or CLI state. Missing prior
history blocks continuation; never infer zero from Git or Task status. A generic
request to continue, a new session, or replan cannot reset an exhausted budget.
After exhaustion, report the remaining findings and the specific evidence or
maintainer decision needed; never silently renew the allowance.

Task progress still uses the existing CLI. On a stop, record the Task blocked
when that mutation is valid, preserve partial work, and return the complete
handoff. No partial implementation commit or worktree rescue is authorized.

## Verification

Embedded-package conformance checks cover the fixed bound, finding mapping,
replan continuity, Direct isolation, and unchanged terminal protocol blocks.
Fresh forward tests exercise new findings, repeated findings, exhaustion,
ambiguous history, preparation failures, and corrected review continuation.
