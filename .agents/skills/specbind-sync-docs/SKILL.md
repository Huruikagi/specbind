---
name: specbind-sync-docs
description: Keep SpecBind's English and Japanese public documentation aligned and the Japanese copy natural and readable. Use for changes or language-quality reviews under docs/en or docs/ja, bilingual navigation, or public documentation URLs; exclude English-only generated reference pages and internal repository documentation.
---

# Keep bilingual documentation aligned

Public user guides are paired by relative path:

```text
docs/en/<relative-path>
docs/ja/<relative-path>
```

English is published at `/`; Japanese is published at `/ja/`. Japanese remains
the practical authoring source for new product explanations, but an edit in
either language must not silently leave its counterpart with different
behavior, commands, ownership boundaries, warnings, or navigation.

## Scope

Apply this workflow to `index.md` and `guide/**/*.md` below both language roots.
`docs/en/reference/` is intentionally English-only. Internal architecture,
Decision, repository, contributor, and forward-test documents outside the two
language roots are not translation pairs.

## Workflow

1. Inspect the requested change and the current Git diff. Identify every
   affected public page and its same-relative-path counterpart.
2. Read both complete pages. When Japanese content is created, edited, or
   reviewed for readability, also read
   [the Japanese style reference](references/japanese-style.md). When the edit
   changes product behavior, commands, paths, or ownership, verify the current
   source or accepted Decision instead of translating an assumption.
3. Preserve meaning, information architecture, examples, warnings, and link
   destinations, but write natural documentation for each audience rather than
   translating sentence by sentence. Edit both counterparts when meaning or
   information structure changes. For a Japanese-only wording improvement,
   inspect the English counterpart but do not manufacture an English edit when
   its meaning and structure already agree.
4. If one counterpart does not exist, create it. If a page is intentionally
   language-specific, confirm that it is outside the paired scope or obtain an
   explicit decision to defer it; report the deferral and its tracking location.
   Do not call the bilingual change complete while drift is merely unmentioned.
5. For a move or removal, apply the same relative-path change to both trees and
   update `mkdocs.yml`, inbound links, contributor guidance, and any public URL
   embedded in code or tests.

## Verification

Run from the repository root:

```sh
python -m mkdocs build --strict
git diff --check
```

Confirm the generated English and Japanese pages exist at corresponding paths
and that their language selector links point to each other. When a public URL
is a CLI contract, also update its source, focused tests, and accepted Decision,
then run the focused test and `python scripts/check_decisions.py`.

Report the page pairs inspected and changed, any counterpart intentionally left
unchanged after semantic comparison, any other exception, and the verification
results.
