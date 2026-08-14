# 0052: Separate project-wide machine state from steering

Status: Accepted

## Context

The active roadmap is intentionally loaded often by humans and agents because it explains current milestone intent, scope, and dependencies. Persisting detailed fingerprints, impact records, and downstream review inputs in its frontmatter would consume routine context with data needed only by deterministic checks and specialized workflows.

A generic `evidence/` directory would also imply that spec-local gate evidence belongs there, even though `spec.yaml` already owns that state. The new location needs to distinguish current project- or milestone-wide machine state from steering prose, spec-local state, run-scoped workflow data, and released history.

## Decision

- `{{SPEC_DIR}}/state/` is the canonical location for committed, currently effective project- or milestone-wide machine state owned by the SpecBind CLI.
- The accepted global cross-spec review is stored at `{{SPEC_DIR}}/state/cross-spec-review.yaml`, not in roadmap frontmatter.
- The file is a strict standalone YAML artifact rather than an OKF concept. Its internal `milestone_id` must equal the active `steering/roadmap.md` milestone ID. The roadmap stores no pointer to the canonical path.
- Absence of the file means that the active milestone has no accepted global cross-spec review. At most one such active file exists because SpecBind permits at most one active milestone.
- Ordinary agents and always-loaded steering context do not preload `state/`. Status and ordinary workflows consume concise CLI summaries; only the cross-spec review, release, repair, or explicit diagnostic flow requests detailed state through the CLI.
- The CLI is the authoritative read and mutation boundary for supported workflows. The YAML remains inspectable for debugging and audit, but agents do not directly edit it.
- `state/` does not absorb spec-local lifecycle or gate evidence, which remains in each spec's `spec.yaml`. Workflow attempts, failed reviews, delegation authorization, and other run-scoped data are not persisted there.
- Successful release finalization moves the accepted active record to `{{SPEC_DIR}}/releases/<version>-cross-spec-review.yaml` alongside `<version>-roadmap.md`, refusing conflicting archive content. Confirmed milestone abandonment removes the active record without creating release history.
- Future files under `state/` require their own ownership, lifecycle, invalidation, and archive decisions. This directory is not a generic cache or dumping ground.

## Consequences

- `roadmap.md` remains compact and suitable for routine human and agent loading.
- Detailed fingerprints and review scope remain durable, portable, and Git-tracked without entering normal context.
- The roadmap remains the canonical scope owner; `state/cross-spec-review.yaml` becomes the canonical owner of the current accepted global review evidence.
- Released scope and review evidence use parallel flat archive files with the same version prefix.
- CLI status can expose a short freshness and impact summary while detailed structured output remains opt-in.
