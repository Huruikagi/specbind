# 0004: Keep per-spec changelogs and archive released roadmaps

Status: Accepted

## Context

Active specs need local history that explains how each capability changed across releases. The active `roadmap.md` also contains milestone-wide scope, dependency ordering, and cross-spec evidence that cannot be reconstructed conveniently from independent spec changelogs.

Keeping completed roadmaps under `steering/` would mix active project guidance with release history. Deleting them would make the immutable release reference the only complete milestone-wide record.

## Decision

- Each spec keeps its own `{{SPEC_DIR}}/specs/<feature>/changelog.md`.
- Released spec entries use the release version as their human-facing key.
- `{{SPEC_DIR}}/steering/roadmap.md` represents only the active milestone.
- Successful release finalization moves the roadmap to a new version-prefixed file at `{{SPEC_DIR}}/releases/<version>-roadmap.md`.
- The archived roadmap retains its machine-generated milestone ID and release-version binding.

Example after releasing `v1.4.0`:

```text
{{SPEC_DIR}}/
├── releases/
│   ├── v1.3.0-roadmap.md
│   └── v1.4.0-roadmap.md
├── specs/
│   └── <feature>/
│       └── changelog.md
└── steering/
    └── ...
```

## Finalization rules

- Archive only after publication and immutable release-reference verification succeed.
- Require the active roadmap's bound release version to match the archive filename.
- Refuse to overwrite an existing archive path unless an idempotent retry proves identical milestone identity and content.
- Move the roadmap as part of the same coherent finalization change that updates spec changelogs and lifecycle state.
- Verify that `steering/roadmap.md` is absent and the archived roadmap is present after finalization.
- Spec changelog entries point to the archived roadmap and immutable release reference where useful.

## Consequences

- `steering/` contains only active guidance and the active milestone.
- `releases/` becomes a flat, append-only project-level release-history area where each release adds a file instead of updating a shared history document.
- Spec-local history and milestone-wide history remain separately navigable.
- A new discovery run can create a fresh active roadmap without overwriting prior milestones.
- Release finalization removes the active roadmap path but does not delete the roadmap history.

## Open questions

- Whether other project-level release summaries or evidence files will use the same `<version>-<artifact>` naming convention.
- How cancelled, never-released milestones are archived and named.
- Whether archived roadmap filenames normalize every release version to a leading `v`.
