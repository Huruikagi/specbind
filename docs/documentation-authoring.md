# Documentation authoring and publication

This document defines the repository workflow for public documentation. It is
an internal contributor document and is excluded from the published MkDocs
site.

## Current authoring source

During active v1 development, Japanese is the practical source for drafting
new public documentation and settling product explanations. Once a page has a
counterpart, however, an edit in either language also reviews and aligns the
other language in the same change unless synchronization is explicitly
deferred. The Japanese tree uses the same relative paths as the English tree:

```text
docs/ja/index.md
docs/ja/guide/getting-started.md
docs/ja/guide/install.md
docs/ja/guide/start-new-project.md
docs/ja/guide/start-existing-project.md
docs/ja/guide/concepts.md
docs/ja/guide/implement-step-by-step.md
docs/ja/guide/implement-with-plan-and-drive.md
docs/ja/guide/adopt-existing.md
docs/ja/guide/release.md
docs/ja/guide/customization.md
docs/ja/guide/update.md
docs/ja/guide/uninstall.md
docs/ja/guide/migrate-from-cc-sdd.md
docs/ja/guide/feedback.md
```

The English tree mirrors the same user-guide paths under `docs/en/`. English
reference pages live under `docs/en/reference/`; they are public but are not
part of the Japanese-first translation contract.

Internal architecture, design, repository, and forward-test documents are not
part of the translation tree. Keep them in their existing repository-owned
locations and excluded from the public site.

## Publishing target

When a page in either language changes, review and refresh its counterpart in
the same documentation change unless synchronization is intentionally deferred.
Preserve meaning and information architecture rather than translating word for
word. If synchronization is deferred, state that explicitly in the change or
tracking Issue instead of allowing silent drift. The repository-local
`sb-dev-sync-docs` Skill defines the repeatable check and verification flow.
Equivalent pages keep matching relative paths:

```text
English source: docs/en/guide/getting-started.md
Japanese source: docs/ja/guide/getting-started.md

English URL:  /guide/getting-started/
Japanese URL: /ja/guide/getting-started/
```

The new-project, existing-project, and existing-implementation routes follow
the same relative-path rule when English counterparts are added.

The bilingual build publishes English at the site root and Japanese under
`/ja/`. `mkdocs-static-i18n` uses the folder structure to select corresponding
pages and reconfigures navigation, Material UI strings, search behavior, and
page-to-page language switching. The dependency is pinned in
`requirements-docs.txt`; Markdown content remains ordinary files so the build
mechanism stays replaceable.

## Verification

Run the strict bilingual build from the repository root after changing a
public page, navigation, translation, or documentation path:

```sh
python -m mkdocs build --strict
```

When a published URL is embedded in the CLI or another product contract,
update its source, focused tests, and the relevant accepted Decision together.
