# スキル一覧

このページでは、`specbind install`がプロジェクトに配置する製品管理のスキルを
まとめます。スキルが作成・管理するファイルは[成果物一覧](./current-artifact-index.md)を
参照してください。

## スキルの置き場所と呼び出し方

| エージェント | 置き場所 | 呼び出し方 |
| --- | --- | --- |
| Codex | `.agents/skills/sb-*/` | `$sb-*` |
| Claude Code | `.claude/skills/sb-*/` | `/sb-*` |
| generic | `.agents/skills/sb-*/` | ホストのエージェントに従う |

どのエージェントにも同じ15個のスキルが配置されます。Codexとgenericは
`.agents/skills/`を共有するので、両方を選んでも各スキルは1つだけ配置されます。

## 普段の流れで使うスキル

ライフサイクルの順に並べています。全体の関係は[基本概念](../guide/concepts.md)を
参照してください。

| スキル | 使う場面 |
| --- | --- |
| `sb-configure` | プロジェクトの設定を見直す・変更する、またはSpecBindをアップデートする。 |
| `sb-steering` | プロジェクトで長く維持する方針（Steering）を作成・更新する。 |
| `sb-discovery` | 変更を始める。スコープを確認し、MilestoneとSpecを作成する。 |
| `sb-plan` | 1つのSpecまたはMilestone全体（`--all`）のRequirements、Design、Tasksを計画する。共有Contractの変更（`--shared`）も準備する。 |
| `sb-contract-review` | Tasksの前に、Milestone内のすべてのContractをまとめてレビューする。 |
| `sb-implement` | 1つのRoadmap項目を、Taskごとのレビューを含めて実装する。 |
| `sb-validate-implementation` | 完了したSpecをRequirementsに照らして検証し、`GO`なら完了を記録する。 |
| `sb-release` | Milestoneをリリースする。バージョンの紐付け、公開、検証、確定処理を行う。 |
| `sb-drive` | Milestone内で安全に進められる作業をまとめて進め、リリースの手前で止まる。`--replan`を付けると範囲内の再計画も任せられる。 |
| `sb-status` | 現在の状態と、次に実行できる操作を表示する。読み取り専用。 |

## 必要なときに使うスキル

| スキル | 使う場面 |
| --- | --- |
| `sb-gap-analysis` | 計画の前に、予定している変更と現在のコードを比べ、結果をResearchとして残す。 |
| `sb-validate-design` | Designを独立して検証する。`sb-plan`はDesignの作成後に自動で実行する。 |
| `sb-review-task` | 実装済みの1つのTaskを差分からレビューする。修正はしない。 |
| `sb-debug` | 止まった実行の根本原因を調べて分類する。修正はしない。 |
| `sb-verify-completion` | 「完了した」という主張を新しい証拠で確認する。ライフサイクルの状態は変えない。 |

## 一時的なスキル

| スキル | 使う場面 |
| --- | --- |
| `sb-adopt` | 既存実装からSpecを確立する。`specbind install --with-adoption`のときだけ配置され、確立が完了すると自動で削除される。[既存実装からSpecを確立する](../guide/adopt-existing.md)を参照。 |

## Codex用のメタデータ

Codexには、追加で`.agents/skills/<skill>/agents/openai.yaml`が配置されます。
Codexの画面で使う表示名（`SpecBind Plan`など）、短い説明、依頼の例を定義する
ファイルです。スキルの呼び出し方は変わりません。
