# 0175: Make existing-implementation adoption references of Discovery

Status: Accepted

Partially superseded by [Decision 0181](./0181-reverse-spec-establishment.md),
which replaces the stop-before-Requirements handoff with one confirmed reverse
orchestration through Design and Contract Review.

Supersedes: the independent `specbind-adopt-existing` Skill identity and
cross-Skill routing accepted by [Decision 0143](./0143-existing-implementation-adoption.md).
Its evidence, confirmation, dossier, and phase-ownership boundaries remain
authoritative.

## Context

Decision 0143 correctly defined adoption as reverse discovery around the
ordinary lifecycle, but exposed that composition as a separate installed Skill.
The workflow then asks a maintainer to move from Adoption to Discovery and back
to Adoption even though all three invocations decide or prepare the same durable
Spec boundaries and Brief intent.

Decision 0096 now supports progressive Skill packages with conditional
references. A procedure no longer needs a separate discovery identity merely
because it is expensive and used only for brownfield bootstrap.

## Decision

`specbind-discovery` is the only installed discovery and adoption Skill. Its
package contains:

```text
specbind-discovery/
|-- SKILL.md
`-- references/
    |-- adopt-start.md
    |-- adopt-resume.md
    `-- local-files.md
```

The entrypoint selects adoption only for an explicit request to establish Specs
from existing implementation or resume the recorded adoption dossier. Ordinary
change discovery never scans implementation merely because a repository is
brownfield.

Adoption remains a two-depth, resumable procedure around ordinary Discovery:

1. `adopt-start.md` checks the committed Steering baseline, maps the selected
   area, confirms candidate boundaries, writes and checkpoints the dossier, and
   stops.
2. A later invocation follows ordinary Discovery for exactly the confirmed
   candidate set. It still presents the complete four-field scope and receives
   separate confirmation before the CLI creates the Milestone, Specs, and
   Briefs.
3. Once those exact Specs and Briefs exist, `adopt-resume.md` verifies the
   baseline, reconciles observations with the maintainer, revises Brief and
   Research handoffs, retires the dossier, and stops before Requirements.

Dossier presence alone does not select Resume. A confirmed dossier with no
created candidate Specs selects ordinary Discovery; exact active candidates and
Briefs select Resume. Partial or mismatched state stops without changing scope
or inventing another dossier.

The adoption references retain Decision 0143's substantive boundaries. Code and
tests are evidence, Steering is mandatory, the evidence revision stays fixed,
and only user-confirmed intent enters Briefs. Discovery owns no Requirements,
Design, Contract, Tasks, implementation, validation, or Gate approval through
this route.

Installation no longer owns the `specbind-adopt-existing` package. Refresh
removes its exact former `SKILL.md`, `references/start.md`, and
`references/resume.md` files under the existing clean committed repository
guard. Extra project files in that directory are preserved; no alias or stub is
installed.

## Consequences

- The installed product catalog has 15 Skills and one boundary-discovery entry
  point.
- Maintainers ask Discovery to establish Specs from an existing implementation
  and resume Discovery after its explicit stops.
- Adoption detail remains progressively loaded and ordinary Discovery does not
  pay its context cost.
- The separate adoption-boundary and lifecycle-scope confirmations remain
  visible even though one Skill owns both procedures.
- Existing dossiers remain resumable because their path and format do not
  change.

## Verification

Mechanical tests verify the 15-Skill catalog, all three Discovery references,
old-package removal with preservation of extra content, and the retained
evidence and phase-ownership boundaries. Fresh forward tests cover initial
adoption routing and the stop before ordinary Discovery mutation.
