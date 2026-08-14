# 0004: Keep per-spec changelogs and archive released roadmaps

Status: Accepted

Superseded in part by [Decision 0048](./0048-okf-spec-log.md): per-spec history now uses the OKF reserved `log.md`, with date-grouped newest-first insertion instead of `changelog.md` append semantics. Decision 0052 adds a companion global-review archive while retaining this decision's flat version-prefixed release layout.

## Context

Active specs need local history that explains how each capability changed across releases. The active `roadmap.md` contains milestone-wide scope and dependency ordering, while Decision 0052's project-state artifact contains detailed cross-spec evidence that cannot be reconstructed conveniently from independent spec logs.

Keeping completed roadmaps under `steering/` would mix active project guidance with release history. Deleting them would leave ordinary Git history or optional project release references as the only complete milestone-wide record.

## Decision

- Each spec keeps its own `{{SPEC_DIR}}/specs/<feature>/changelog.md`.
- Released spec entries use the release version as their human-facing key.
- `{{SPEC_DIR}}/steering/roadmap.md` represents only the active milestone.
- Successful release finalization moves the roadmap to a new version-prefixed file at `{{SPEC_DIR}}/releases/<version>-roadmap.md`.
- Successful release finalization moves the accepted global review state to the companion `{{SPEC_DIR}}/releases/<version>-cross-spec-review.md`.
- The archived roadmap retains its machine-generated milestone ID and release-version binding.
- Under Decision 0073, `<version>` is the exact opaque portable label stored in `target_release`; archive filenames never add, remove, or normalize a leading `v`.

Example after releasing `v1.4.0`:

```text
{{SPEC_DIR}}/
├── releases/
│   ├── v1.3.0-roadmap.md
│   ├── v1.3.0-cross-spec-review.md
│   ├── v1.4.0-roadmap.md
│   └── v1.4.0-cross-spec-review.md
├── specs/
│   └── <feature>/
│       └── changelog.md
└── steering/
    └── ...
```

## Finalization rules

- Archive only after applicable project release actions and required core release verification succeed.
- Require the active roadmap's bound release version to match the archive filename.
- Refuse to overwrite an existing archive path unless an idempotent retry proves identical milestone identity and content.
- Move the roadmap and accepted global review state as part of the same coherent finalization change that updates spec logs and lifecycle state.
- Verify that `steering/roadmap.md` and `state/cross-spec-review.md` are absent and both companion archives are present after finalization.
- Spec log entries point to the archived roadmap and may include project release references where useful.

## Consequences

- `steering/` contains only active guidance and the active milestone.
- `releases/` becomes a flat, append-only project-level release-history area where each release adds a file instead of updating a shared history document.
- Spec-local history and milestone-wide history remain separately navigable.
- A new discovery run can create a fresh active roadmap without overwriting prior milestones.
- Release finalization removes the active roadmap path but does not delete the roadmap history.

## Open questions

- Whether other project-level release summaries or state records will use the same `<version>-<artifact>` naming convention.
- Whether projects need an opt-in audit artifact for cancelled, never-released milestones; by default they are not release-archived under [Decision 0005](./0005-active-change-abandonment.md).
