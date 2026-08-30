# Documentation authoring and publication

This document defines the repository workflow for public documentation. It is
an internal contributor document and is excluded from the published MkDocs
site.

## Current authoring source

During active pre-1.0 development, Japanese is the practical source of truth
for public user documentation. Author and revise Japanese pages under
`docs/ja/` without requiring a simultaneous English edit. The Japanese tree
uses the relative paths intended for the eventual English tree:

```text
docs/ja/index.md
docs/ja/guide/getting-started.md
docs/ja/guide/start-new-project.md
docs/ja/guide/start-existing-project.md
docs/ja/guide/adopt-existing.md
docs/ja/guide/concepts.md
docs/ja/guide/customization.md
docs/ja/guide/feedback.md
docs/ja/guide/uninstall.md
docs/ja/guide/migrate-from-cc-sdd.md
```

The existing English migration guide and English reference pages remain
published during the transition, but they do not make English authoritative
for changing Japanese guide content.

Internal architecture, design, repository, and forward-test documents are not
part of the translation tree. Keep them in their existing repository-owned
locations and excluded from the public site.

## Publishing target

Once a Japanese page or documentation area is stable enough, create or refresh
the English counterpart from it. Preserve meaning and information architecture
rather than translating word for word. Equivalent pages should keep matching
relative paths:

```text
English source:  docs/en/guide/getting-started.md
Japanese source: docs/ja/guide/getting-started.md

English URL:  /guide/getting-started/
Japanese URL: /ja/guide/getting-started/
```

The new-project, existing-project, and existing-implementation routes follow
the same relative-path rule when English counterparts are added.

The final bilingual build will publish English at the site root and Japanese
under `/ja/`, with localized navigation, Material UI strings, search behavior,
and page-to-page language switching. Until the English tree is ready, the root
page is a transition entry and Japanese pages are published from `docs/ja/`.

## Verification

Run the strict documentation build from the repository root after changing a
public page, navigation, or documentation path:

```sh
python -m mkdocs build --strict
```

When a published URL is embedded in the CLI or another product contract,
update its source, focused tests, and the relevant accepted Decision together.
