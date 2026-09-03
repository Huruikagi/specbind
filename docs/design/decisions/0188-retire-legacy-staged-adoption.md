# 0188: Retire the legacy staged adoption route

Status: Accepted

Supersedes Decision 0175. Decision 0181's single confirmed reverse-establishment
orchestration remains authoritative.

## Context

Decision 0175 folded existing-implementation adoption into `sb-discovery` as a
three-invocation route: adoption start wrote a temporary record, ordinary
Discovery created the confirmed Specs and Briefs, and adoption resume completed
the evidence handoff before Requirements. Decision 0181 replaced that route for
new work with one fixed-revision reverse establishment that continues through
Requirements, Design, Contract Review, and non-release baseline finalization.

The old `adopt-start.md` and `adopt-resume.md` procedures remained installed only
to resume temporary records created by an older SpecBind version. The maintainer
accepts retiring that compatibility path during the current early adoption
phase. Fresh forward tests also showed agents reading those legacy references
during an explicitly current reverse run even though the entrypoint prohibited
that read. Keeping the files therefore adds a competing interpretation and
context cost without preserving a required workflow.

## Decision

`sb-discovery` exposes exactly one existing-implementation route:
`references/reverse.md`. An explicit request to establish Specs from current
code and tests selects that procedure before ordinary change-request routing.
There is no legacy Start, ordinary-Discovery handoff, or Resume branch.

The package no longer contains:

```text
references/adopt-start.md
references/adopt-resume.md
```

The current reverse procedure still uses
`<specDir>/adoption/reverse-discovery.yaml` as temporary run evidence after its
single proposal is confirmed. This record remains outside persistent Spec
discovery and is removed by reverse finalization. Retiring the staged route does
not remove that current evidence artifact, `specbind adoption preflight`, the
`adoption_ready` state, or the reverse finalize and abandon commands.

An orphan temporary adoption record from the retired route is unsupported. The
read-only adoption preflight rejects it before a new reverse run can overwrite
or reinterpret it. The maintainer must inspect and explicitly reconcile that
project-owned evidence; SpecBind neither resumes nor deletes it automatically.

`specbind install` treats the two former reference paths as retired exact
product-managed resources inside the still-active `sb-discovery` package. A
refresh removes those regular files under the existing committed-clean
repository guard for every selected Agent, preserves unrelated package content,
and removes an empty `references` directory only through the existing safe
cleanup behavior.

Historical forward-test run records remain unchanged. Current scenario
contracts and result projections describe only the Decision 0181 reverse route.

## Consequences

- Existing-implementation establishment has one discoverable procedure and one
  confirmation model.
- The Discovery package loses two references and its legacy state-selection
  branch.
- Old staged records are not silently migrated, resumed, overwritten, or
  deleted.
- Package refresh needs an exact retired-resource list for a Skill that remains
  active; retired whole-package handling alone is insufficient.
- Decision 0175 remains historical evidence for the removed workflow, while
  Decision 0181 and this decision define current behavior.

## Verification

Mechanical tests verify that the catalog and rendered packages omit both legacy
references, refresh removes their exact installed paths while preserving extra
content, an orphan record blocks adoption preflight, and project instructions
route explicit existing-implementation establishment only to current reverse
Discovery. A fresh A1 forward test verifies that the installed package contains
no legacy references and still runs adoption preflight before any ordinary
milestone or Spec read.
