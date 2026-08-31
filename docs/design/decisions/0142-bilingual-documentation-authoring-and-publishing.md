# 0142: Separate Japanese-first documentation authoring from English-default publishing

Status: Accepted

This decision supersedes the Japanese guide path listed by
[Decision 0125](./0125-agent-assisted-cc-sdd-migration.md). Its neutral and
English migration entry paths remain unchanged.

The CLI-specific canonical URL and neutral-entry statements below are
superseded by
[Decision 0166](./0166-single-english-cc-sdd-migration-guide-url.md). The
bilingual authoring and publication hierarchy remains accepted.

## Context

The public documentation grew with Japanese user guides under `guide/ja/`, an
English migration guide under `guide/en/`, and Japanese content at the site
root. That layout couples the source tree to an interim Japanese-default
publication structure and gives translated pages no predictable relationship.

During active development, the user guide is authored and revised primarily in
Japanese. Requiring English to change in lockstep would slow normal product
work and could make incomplete English text appear authoritative. The final
public site nevertheless needs English at its root for an OSS audience and
Japanese under a stable `/ja/` namespace.

## Decision

Documentation authoring and publication hierarchy are separate concerns.

- Japanese is the practical source of truth during active pre-1.0 development.
- Japanese public pages live under `docs/ja/` with a coherent relative path
  hierarchy. A Japanese change does not require a simultaneous English edit.
- English is created or refreshed from sufficiently stable Japanese content.
  Translation preserves meaning and information architecture, but need not be
  literal.
- Equivalent English and Japanese pages use matching relative paths where
  practical.
- The final public English tree is served at `/`, and the Japanese tree is
  served under `/ja/`.
- The final bilingual build has localized navigation, Material UI language,
  search behavior, and a switch between corresponding pages. A language root
  is an acceptable fallback when no counterpart exists.
- Internal architecture, design, repository, and forward-test documentation is
  excluded from this translation structure.

The migration is phased. Phase 1 normalizes and continues Japanese authoring.
Phase 2 creates the mirrored English content after the Japanese information
architecture stabilizes. Phase 3 enables the complete bilingual publication
experience. The build mechanism remains replaceable; a pinned build-time i18n
plugin may be used when it materially reduces navigation, search, and language
switching maintenance while leaving content as ordinary Markdown.

During Phase 1, the site root was a transition entry and the normalized Japanese
tree was published under `/ja/`. The Japanese cc-sdd migration guide moved to:

`https://huruikagi.github.io/specbind/ja/guide/migrate-from-cc-sdd/`

The unused pre-1.0 Japanese migration URL was removed. The completed bilingual
site uses `docs/en/` as the English source published at `/`, `docs/ja/` as the
Japanese source published at `/ja/`, and `mkdocs-static-i18n` for one strict
build with localized navigation, UI, search, and corresponding-page switching.

## Consequences

- New feature documentation can be drafted and stabilized in Japanese; once a
  counterpart exists, later changes align it or explicitly record a deferral.
- The source hierarchy now exposes the intended one-to-one translation paths.
- English and Japanese guide updates must check the corresponding page and
  either update it or record an intentional translation deferral.
- Changes to embedded documentation URLs remain coordinated with CLI tests and
  accepted Decisions.
