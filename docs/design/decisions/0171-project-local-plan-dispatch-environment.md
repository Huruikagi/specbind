# 0171: Carry the project-local execution environment into Plan dispatches

Status: Accepted

## Context

`specbind-plan` deliberately gives each planning phase to a fresh subagent.
The dispatch already carries the Spec, phase, workflow, and approval authority,
but it does not identify the project working directory or the `specbind`
executable that the orchestrator successfully used there.

In a host with another `specbind` on its default `PATH`, a fresh phase can
therefore run the wrong version. The phase then reports missing commands even
though the project-local CLI supports them. Retrying the same underspecified
dispatch cannot distinguish a product failure from an environment mismatch.

## Decision

Before its first phase dispatch, `specbind-plan` establishes the exact project
working directory and confirms the `specbind` version from that directory. It
also records the executable resolution, including any project-local `PATH`
entry or equivalent environment fact needed to reproduce that same resolution.

Every fresh planning-phase, Design-validation, and Contract Review dispatch
carries those facts explicitly. The receiver starts in the named directory,
honors the project-local instruction files that apply there, and invokes the
same confirmed `specbind` executable. This operating context is part of the
dispatch payload, not approval authority and not permission to widen scope.

If a fresh receiver cannot reproduce the confirmed executable and version, it
returns an environment failure before reading or writing artifacts. It never
falls back to another binary, silently changes `PATH`, installs a replacement,
or interprets a command mismatch as an artifact or workflow defect.

The orchestrator may retry only after correcting the dispatch payload or the
host environment. Phase workflow, gate, checkpoint, and retry ownership remain
unchanged.

## Consequences

- Fresh phases execute against the same project and SpecBind version that
  produced the authoritative schedule.
- A host-global older binary cannot masquerade as a missing product command.
- Execution context remains separate from workflow and approval authority.
- Environment mismatches fail before phase-owned artifacts are changed.

## Verification

Focused Skill tests require the project-local working directory, instruction,
executable, version, and no-fallback clauses in the Plan dispatch contract. HP1
exercises the complete composed planning path with a fixture-local CLI.
