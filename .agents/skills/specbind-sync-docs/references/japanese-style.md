# Write natural Japanese public documentation

Use this reference for Japanese public pages under `docs/ja/`. The goal is not
to remove English mechanically. Preserve product vocabulary that helps readers
map prose to Skills, artifacts, CLI state, and diagnostics, while expressing
ordinary explanations as natural Japanese.

## Decide by responsibility, not spelling

Apply this order:

1. Preserve exact commands, Skill names, paths, schema fields, state values,
   diagnostic codes, and quoted output. Format them as code where appropriate.
2. Retain established SpecBind concept and artifact names when they identify a
   specific product responsibility.
3. Translate generic English prose that does not need exact identity.

Do not translate a term merely because it is English, and do not retain an
English word merely because it appears in the English counterpart.

## Retain canonical product vocabulary

The following names may remain in English when they refer to the corresponding
SpecBind concept or artifact:

- `Spec`, `Requirements`, `Design`, `Contract`, `Tasks`
- `Milestone`, `Roadmap`, `Gate`, `Steering`
- `Direct`, `Brief`, `Research`
- named workflow entry points such as Plan, Drive, and Release when the prose
  clearly refers to their Skill responsibility
- `Source Collection` and `Source Item` when referring to the Discovery model

At the first meaningful use on a directly reachable procedure page, add a
short Japanese explanation or link to the concepts guide when a reader may not
know the term. Do not repeat the same parenthetical gloss throughout the page.

Use Japanese particles and surrounding prose naturally: for example,
`Requirementsを作成する`, `Designの検証`, and `Milestone内の依存関係`.
When a formal compound is hard to read, introduce it once and use a shorter
Japanese form afterward:

- `Spec-backed item（Specに基づく作業項目）` -> later `Spec項目`
- `Direct item（Spec成果物を変更しない直接作業）` -> later `Direct項目`
- `Roadmap item` -> `Roadmap項目`

## Translate generic prose

Prefer Japanese for words that describe ordinary state or action rather than a
named product surface. Choose the translation that fits the sentence; this is
not a fixed word-substitution table.

| Avoid in ordinary prose | Prefer |
| --- | --- |
| `active` | `進行中の`、`有効な` |
| `item` | `項目`、`作業項目` |
| `actionable` | `着手可能な`、`次に実行できる` |
| `workflow` | `ワークフロー`、文脈によっては`手順` |
| `evidence` | `証拠`、`根拠`、`記録` |
| `finalize` / `finalization` | `確定する`、`完了処理` |
| `provider` / `consumer` | `提供側` / `利用側` |
| `project-owned` | `プロジェクトが所有する` |
| `exact path` | `正確なパス` |
| `marked block` | `マーカーで囲まれたブロック` |
| `bundle` | `一式` |
| `bind` | `紐付ける`、`固定する` |

Some English phrases are exact CLI concepts. On first use, keep the exact term
and explain it, then use readable Japanese when exact identity is not needed.
For example, introduce `completion evidence（完了を裏付ける記録）` and later
say `完了記録`; retain `completion evidence` when referring to the literal CLI
field or diagnostic.

Use Japanese technical loanwords when they are clearer than raw English:
`adapter` becomes `アダプター`, `checkpoint` becomes `チェックポイント`, and
generic `workflow` becomes `ワークフロー`.

## Review sentences, not isolated tokens

Rewrite the sentence when English terms accumulate. Avoid inserting Japanese
particles between a sequence of untranslated English nouns.

| Hard to read | Prefer |
| --- | --- |
| `activeなMilestoneのactionable item` | `進行中のMilestoneで次に着手できる項目` |
| `project-owned settingsをreviewする` | `プロジェクトが所有する設定を確認する` |
| `release finalizationまでretainする` | `リリースの確定処理まで保持する` |
| `mutating workflowをdispatchする` | `状態を変更するワークフローへ処理を委譲する` |

After editing, read the complete Japanese paragraph aloud in ordinary sentence
order. Confirm that:

- exact terms still map to the correct product surface;
- adjacent English words do not make the reader reconstruct the grammar;
- the same concept is named consistently within the page;
- headings and link labels are understandable without the English counterpart;
- shortening or translation did not weaken a safety boundary; and
- the English counterpart still carries the same behavior and information
  structure.

Japanese-only copyediting does not require a no-op English rewrite. Record that
the counterpart was inspected and left unchanged because meaning and structure
already matched.
