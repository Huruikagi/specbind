# 0048: Use the OKF log.md for per-spec release history

Status: Accepted

## Context

Decision 0004 introduced a persistent per-spec `changelog.md` as the navigable index of released changes and evidence. Decision 0045 later made the configured spec root an OKF v0.2 Knowledge Bundle.

OKF reserves `log.md` for the history of changes to the directory scope where it appears. A per-spec release history has exactly that scope. Keeping a separate `changelog.md` concept would preserve a private filename and format despite an applicable OKF primitive.

## Decision

- The persistent per-spec release-history path is `{{SPEC_DIR}}/specs/<feature>/log.md`, superseding the `changelog.md` path accepted by Decision 0004.
- `log.md` is an OKF reserved file, not a concept document. It has no YAML frontmatter and therefore no `type`.
- Its body follows the OKF v0.2 log format:
  - one document title
  - ISO 8601 `YYYY-MM-DD` date headings
  - newest date first
  - a flat prose list under each date
- Each released milestone that participated in the spec contributes one concise release entry under the applicable date.
- The release version remains the entry's primary human-facing label. The milestone ID remains secondary trace metadata.
- An entry retains useful SpecBind release context, including the delivered-scope summary, validation result, and archived roadmap. A project tag, Release URL, deployment identifier, or commit may be included when useful but is not required by SpecBind.
- Release finalization inserts an entry into newest-first date order instead of appending to the end of the file. Re-running finalization must not duplicate the same release entry.
- An abandoned unreleased change creates no `log.md` entry.
- The exact prose convention, release-date source, and evidence granularity remain part of the release-history entry profile to be accepted with the release evidence contract.

## Consequences

```markdown
# Spec Update Log

## 2026-08-14

* **Release v1.4.0**: Added authenticated checkout. Milestone `0198b2d1-7c4a-7e31-9f42-8e7c3a110d62`; see [roadmap](../../releases/v1.4.0-roadmap.md).

## 2026-07-20

* **Release v1.3.0**: Added the initial checkout flow.
```

- Multiple releases on the same date are separate list entries under one date heading.
- Existing projects migrate `changelog.md` content into the date-grouped `log.md` form rather than retaining both files.
- Decision 0004 remains authoritative for per-spec history ownership and roadmap archival, but its `changelog.md` filename and append-oriented wording are superseded by this decision.

## Reference

- [Open Knowledge Format v0.2, Log files](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md#9-log-files)
