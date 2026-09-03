# 0191: Resume reverse establishment from verified durable state

Status: Accepted

## Context

Decision 0181 defines reverse establishment as one confirmed orchestration from
a fixed implementation revision through Requirements, Design, Contract Review,
and non-release finalization. After scope creation the workflow checkpoints an
active reverse Roadmap, establishment provenance in each Spec, Briefs, Research,
and a temporary adoption record. It did not define re-entry after the driving
conversation or process ended.

Requiring one uninterrupted Agent run is weaker than the durable lifecycle the
workflow creates. Ordinary Discovery must not reinterpret the temporary record,
but an explicitly requested reverse continuation can be verified from the
active Roadmap, Spec provenance, record, Git history, and current status.

## Decision

`specbind adoption preflight` remains the first command for both a new reverse
establishment and an explicit request to resume one. It returns exactly one
ready result:

- `ADOPTION_PREFLIGHT_READY` when the initial clean, committed, Steering-backed
  project has no persistent Specs, active Milestone, or temporary record;
- `ADOPTION_RESUME_READY` when an active unbound reverse Milestone and its
  temporary record agree on the fixed source revision, all current lifecycle
  state is consistent, the checkout is clean, and changes since the fixed
  revision are confined to the reverse workflow's accepted paths.

The resume result reports milestone identity, source revision, baseline
version, current stage, actionable work, and Steering count. A missing,
non-regular, unreadable, malformed, or revision-mismatched record fails. An
ordinary delivery Milestone, inconsistent status, dirty checkout, or source
drift also fails. Preflight never repairs or mutates state.

Only an explicit maintainer request to resume the named reverse establishment
authorizes its remaining orchestration under delegated workflow
`sb-discovery`. The existence of an active reverse Milestone, a generic request
to drive it, or a nested dispatch without relayed authority does not grant Gate
approval. The resume route never repeats the boundary proposal, changes scope,
or repeats an already approved phase.

`sb-discovery` is the reverse continuation entrypoint. It reads fresh Milestone
status, follows only the phase-relative actions exposed for the active reverse
Milestone, and retains the original no-Tasks, no-implementation, no-release,
fixed-source boundary. Ordinary Discovery never reads the temporary record.

## Consequences

- Reverse establishment no longer depends on one uninterrupted Agent context.
- The same preflight command distinguishes start from resume without a second
  overlapping state probe.
- Durable workflow state is continuation evidence, not persisted user
  authority.
- Partial, dirty, mismatched, or source-stale reverse state remains a visible
  stop rather than being silently repaired.

## Verification

CLI tests cover clean resume, orphan and missing records, revision mismatch,
ordinary active Milestones, dirty checkpoints, and source drift. Skill tests
cover start-versus-resume selection, explicit continuation authority, and the
prohibition on repeating scope or completed phases. A fresh forward test starts
from a checkpointed active reverse Milestone in a new Agent context and verifies
that continuation uses the existing scope and changes no implementation source.
