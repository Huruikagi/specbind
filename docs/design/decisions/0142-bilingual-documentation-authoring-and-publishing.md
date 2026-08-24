# 0142: Separate Japanese-first documentation authoring from English-default publishing

Status: Accepted

This decision supersedes the Japanese guide path listed by
[Decision 0125](./0125-agent-assisted-cc-sdd-migration.md). Its neutral and
English migration entry paths remain unchanged.

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
experience. The build mechanism remains replaceable; no third-party i18n
plugin becomes a product dependency merely to establish the content model.

During Phase 1, the site root is a transition entry, the normalized Japanese
tree is already published under `/ja/`, and existing English migration and
reference pages remain available. The Japanese cc-sdd migration guide moves to:

`https://huruikagi.github.io/specbind/ja/guide/migrate-from-cc-sdd/`

The unused pre-1.0 Japanese migration URL is removed. CLI output uses the new
canonical URL.

## Consequences

- Normal feature documentation can continue in Japanese without stale English
  blocking it.
- The source hierarchy now exposes the intended one-to-one translation paths.
- English publication remains visibly transitional until the Japanese guide is
  stable enough to translate.
- Changes to embedded documentation URLs remain coordinated with CLI tests and
  accepted Decisions.
