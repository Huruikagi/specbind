# 0052: Separate project-wide machine state from steering

Status: Accepted

Implementation status: the Rust Contract review acceptance operation creates `state/` only as a regular directory, rejects symlink or non-file targets, renders the four-field accepted OKF profile, flushes a temporary file in the target directory, and atomically replaces `state/contract-review.md`. Its read model strictly validates the same profile and body and distinguishes Direct-only absence, Spec-backed absence, freshness, staleness, and invalid persisted state. Replacement retains no failed attempt state or backup file; Git remains recovery history.

Decision 0078 simplifies the stored review to input revisions plus a free-form accepted assessment and defines absence for Direct-only milestones.

## Context

The active roadmap is intentionally loaded often by humans and agents because it explains current milestone intent, scope, and dependencies. Persisting detailed fingerprints, impact records, and downstream review inputs in its frontmatter would consume routine context with data needed only by deterministic checks and specialized workflows.

A generic `evidence/` directory would also imply that spec-local gate evidence belongs there, even though `spec.yaml` already owns that state. The new location needs to distinguish current project- or milestone-wide machine state from steering prose, spec-local state, run-scoped workflow data, and released history.

## Decision

- `{{SPEC_DIR}}/state/` is the canonical location for committed, currently effective project- or milestone-wide machine state owned by the SpecBind CLI.
- The accepted global contract review is stored as the OKF concept `{{SPEC_DIR}}/state/contract-review.md`, not in roadmap frontmatter.
- Its structured frontmatter and AI-authored Markdown body form one CLI-managed state artifact. Its internal `milestone_id` must equal the active `steering/roadmap.md` milestone ID. The roadmap stores no pointer to the canonical path.
- Absence of the file means that the active milestone has no accepted global contract review. At most one such active file exists because SpecBind permits at most one active milestone.
- Ordinary agents and always-loaded steering context do not preload `state/`. Status and ordinary workflows consume concise CLI summaries; Decision 0087 exposes the focused review summary through `specbind milestone review status`, while only the contract review, release, repair, or explicit diagnostic flow requests any more detailed state through the CLI.
- The CLI is the authoritative read and mutation boundary for supported workflows. The YAML remains inspectable for debugging and audit, but agents do not directly edit it.
- `state/` does not absorb spec-local lifecycle or gate evidence, which remains in each spec's `spec.yaml`. Workflow attempts, failed reviews, delegation authorization, and other run-scoped data are not persisted there.
- Successful release finalization moves the accepted active record to `{{SPEC_DIR}}/releases/<version>-contract-review.md` alongside `<version>-roadmap.md`, refusing conflicting archive content. Confirmed milestone abandonment removes the active record without creating release history.
- Future files under `state/` require their own ownership, lifecycle, invalidation, and archive decisions. This directory is not a generic cache or dumping ground.

## Consequences

- `roadmap.md` remains compact and suitable for routine human and agent loading.
- Detailed fingerprints and review scope remain durable, portable, and Git-tracked without entering normal context.
- The roadmap remains the canonical scope owner; `state/contract-review.md` becomes the canonical owner of the current accepted global review evidence.
- Released scope and review evidence use parallel flat archive files with the same version prefix.
- CLI status can expose a short freshness and impact summary while detailed structured output remains opt-in.
