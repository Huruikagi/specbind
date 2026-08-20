# 0122: Give every review finding a disposition and a destination

Status: Accepted

## Context

[Decision 0121](./0121-requirements-coverage-is-not-slots.md) bounded what a
Requirements document may claim. It did not touch the other direction scope
grows from: review.

Field observation from cc-sdd is that reviewers raise findings as critical
because a finding is otherwise volatile. Nothing downstream carries a
non-blocking observation, so blocking is the only state in which it survives.
Severity inflation is a rational adaptation to a missing destination rather than
reviewer error, and a severity floor imposed on its own does not remove the
incentive: either reviewers keep inflating, or true observations are lost. Both
are worse than the current behavior.

Half of the floor already exists. `task-review` states that naming, formatting,
and unstated preferences are not rejecting on their own and must be said to be
non-rejecting. `design-validation` requires ranking by what would change the
decision. Neither says where the surviving observation goes, and no SpecBind
artifact holds it: the Roadmap is milestone-scoped and archived at release,
Research and `tasks.yaml` are milestone-local and deleted at release, `log.md`
is history, and steering carries durable convention rather than pending work.

## Decision

### Disposition, not severity

`task-review`, `design-validation`, and `requirements-review` each gain *Every
finding gets a disposition*. A finding ends in exactly one of three states, named
by the reviewer:

- **Blocking.** It changes the verdict and is resolved before the gate.
- **Resolved in place.** It was examined, needs no work, and the reason is
  stated.
- **Deferred.** It is real and actionable, does not change the verdict, and is
  written to the destination the project names.

The fourth state, mentioned in the report and carried nowhere, is what the
protocols now remove. A severity scale is deliberately not introduced: a scale
invites argument about levels, while a disposition asks only what happens to the
finding.

Deferring is stated as not being a way to pass a review that should not pass, and
a project with no destination states the deferred finding in the report and says
it was not recorded, rather than promoting or silently discarding it.

The reviewing skills carry the disposition in the report shape itself, as
`[BLOCKING|DEFERRED|RESOLVED]` on each findings line, so it cannot be omitted by
inattention.

### Scope of the deferred lane

`contract-review` and `completion-verification` are deliberately excluded.

`contract-review` states that scope expansion is surfaced, not absorbed: a Spec
that requires owned work must enter the milestone and cannot be left behind a
passing review as follow-up. A deferred lane there would be an exit from that
prohibition. `completion-verification` is an evidence gate rather than a quality
review, and its baseline is to refuse rather than weaken.

### Destination

A new project adapter under [Decision 0101](./0101-project-adapter-directory-and-git-workflow.md):

| Selector | Path | OKF type | Presence |
| --- | --- | --- | --- |
| `deferred` | `settings/adapters/deferred.md` | `SpecBind Deferred Findings Adapter` | optional at runtime |

Owning consumers are `specbind-review-task`, `specbind-validate-design`,
`specbind-design`, and `specbind-requirements`, each naming the selector and
reading it through `specbind adapter read deferred`. Absence means the project
has no destination, which is reported rather than worked around.

The adapter is the destination's name, not the destination. A project routes
findings to whatever it already uses, an issue tracker or a wiki or a file, and
SpecBind does not grow a tracker of its own. No status, triage, or lifecycle
field is defined, because defining one is how it would become a tracker.

### The installed scaffold carries a working default

Decision 0101 describes an adapter scaffold as an empty vessel, and the skills
treat a scaffold that still carries its `specbind:instruction` comments as no
guidance. The `deferred` adapter departs from this: its installed body names a
default destination, an appended `deferred.md` at the specification-directory
root, and the consuming skills follow it as written.

The reason is the failure mode itself. For release and Git, no default is safe,
and absence correctly means that nothing is done. Here absence restores exactly
the incentive this decision removes, so a project that has configured nothing
would still push its reviewers toward blocking. The default is ordinary project
policy once installed: it is never overwritten, and emptying or removing the file
returns the project to having no destination.

### One-way by construction

An authoring agent that reads deferred findings for work would reopen from the
back door what Decision 0121 closed at the front, so the adapter states, and the
skills repeat, that the destination is written to and not read as a source of
scope. Reading it far enough to avoid recording the same finding twice is
permitted and is the only permitted read. An entry re-enters the workflow when a
person puts it on the Roadmap, where ordinary discovery classification applies.

### Not decided here

A cap on findings per round or on review rounds is not adopted. A count limit
can hide a genuine blocker, and Decision 0094 places review-loop limits in the
owning skill rather than in a protocol. No skill defines one today; whether one
should is separate work.

A review may not introduce a new Requirement, but this needs no new text:
`task-review` already states that work belonging to another task is out of scope,
and `design-validation` already requires returning to Requirements rather than
inventing design detail that hides a gap. The disposition sections give those
statements the destination they were missing.

## Consequences

- A true observation that does not hold a gate now has somewhere to go, so
  raising it as blocking stops being the only way to preserve it.
- `specbind install` plans one additional adapter. Existing projects receive it
  as an uncommitted addition on the next refresh, per Decision 0077.
- The deferred destination is outside the artifact system. It has no OKF profile,
  no discovery entry, no fingerprint, no gate, and no release-finalization
  handling, and nothing in SpecBind validates its contents.
- Reviews that defer nothing are unaffected, and no existing verdict changes
  meaning.

## Alternatives considered

- **A managed `deferred.md` artifact owned by SpecBind.** Rejected: it would
  require an OKF profile, template, discovery, install, release-finalization
  exclusion, and documentation, to hold a file whose contents the product never
  reads.
- **An adapter with no default destination.** Rejected: a project that configures
  nothing keeps the original incentive, which is the failure this decision
  exists to remove.
- **A severity scale with a blocking floor.** Rejected: it makes the level the
  subject of the argument, and without a destination it is the inflation
  mechanism rather than its cure.
