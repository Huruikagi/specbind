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
- An entry retains the delivered-scope summary, archived roadmap reference, and milestone identity. A project tag, Release URL, deployment identifier, commit, or validation detail may be included in the agent-authored summary when useful but is not required by SpecBind.
- Under Decision 0066, the agent supplies each participating spec's delivered-change summary and release finalization inserts the canonical entry into newest-first date order instead of appending to the end of the file. Re-running finalization must not duplicate the same milestone entry.
- An abandoned unreleased change creates no `log.md` entry.
- Decision 0068 defines the strict JSON summary transport, canonical prose wrapper, local release-date source, inline-Markdown safety check, and idempotent milestone match.

## Consequences

```markdown
# Spec Update Log

## 2026-08-14

* **Release v1.4.0** — Added authenticated checkout. ([roadmap](../../releases/v1.4.0-roadmap.md), milestone `0198b2d1-7c4a-7e31-9f42-8e7c3a110d62`)

## 2026-07-20

* **Release v1.3.0** — Added the initial checkout flow. ([roadmap](../../releases/v1.3.0-roadmap.md), milestone `0197d07b-3510-7ec2-a43e-3176200d10e9`)
```

- Multiple releases on the same date are separate list entries under one date heading.
- Existing projects migrate `changelog.md` content into the date-grouped `log.md` form rather than retaining both files.
- Decision 0004 remains authoritative for per-spec history ownership and roadmap archival, but its `changelog.md` filename and append-oriented wording are superseded by this decision.

## Reference

- [Open Knowledge Format v0.2, Log files](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md#9-log-files)
