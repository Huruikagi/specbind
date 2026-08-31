# PlanとDriveでMilestoneを進める

このページでは、複数のSpecやDirect itemを含むactiveなMilestoneを、計画の確定から
実装検証の完了までまとめて進めます。最初に`specbind-plan --all`で計画をレビューし、
その後`specbind-drive`で安全に到達可能な作業を進めるのが通常の推奨経路です。

DriveはReleaseを実行しません。局所的な問題があっても独立した作業が残っていれば
そちらを進め、すべての到達可能な作業が尽きた時点で、必要な判断をまとめて報告します。

## 1. 前提を確認する

先にDiscoveryでスコープ、Spec、Direct item、依存関係を確認し、activeなMilestoneを
作成しておきます。開始時には作業ツリーをcleanにしてください。

```text
$specbind-status
```

Driveは現在の状態をCLIから毎回読み直します。以前の会話や前回のDrive報告を、進捗の
正規情報として引き継ぎません。

## 2. Milestone全体を計画する

```text
$specbind-plan --all
```

Planは、対象となる全SpecのRequirements、依存順のDesignと独立検証、Milestone全体の
Contract review、Tasksを進め、Tasks承認後に停止します。Direct itemはSpec成果物を
持たないため、この計画対象には入りません。

実行の最初に、Requirements、Design、TasksのGate承認をこの名前付き実行へ委任するか
確認されます。委任は確認箇所をまとめるだけで、レビューやCLI検査を省略しません。

計画を先に分けることで、Milestone全体の振る舞い、設計境界、実行順を実装前にまとめて
レビューできます。なおDriveを計画途中から実行することもできますが、Drive自体には
Gate承認権限がありません。必要な承認がなければ、その項目を保留して別の到達可能な
作業を探します。

## 3. 到達可能な作業をDriveする

```text
$specbind-drive
```

Driveは`specbind milestone status --json`が示すactionableな操作だけを選び、1回に
1つの所有ワークフローへ委譲します。RequirementsやDesignを自身で書いたり、Taskを
まとめて完了扱いにしたりはしません。

代表的な委譲先は次のとおりです。

| 状態 | 所有するワークフロー |
| --- | --- |
| 未完了の計画 | `specbind-plan`と各計画フェーズ |
| Contract review | `specbind-contract-review` |
| 実装 | `specbind-implement <item-id>` |
| Spec全体の実装検証 | `specbind-validate-implementation <spec-id>` |
| Release前 | 状態を報告して停止 |

各委譲後に、DriveはGitの作業ツリーとMilestone statusを読み直します。最初の実装では
mutating workflowを並列実行せず、1件ずつ進めます。

## 4. 保留と停止の違いを理解する

ある項目が進められなくても、Driveが即座に実行全体を止めるとは限りません。

```text
項目Aに判断が必要
  -> Aとその依存先を保留
  -> 独立した項目Bが安全に進められるなら継続
  -> 到達可能な作業が尽きたら、Aの判断を報告
```

Driveは、問題の原因と実行全体の判断を分けます。

| 原因の例 | 扱い |
| --- | --- |
| `HUMAN_DECISION` | スコープ、意味、承認、不可逆な結果をユーザーが判断するまで保留 |
| `BLOCKED` | 所有ワークフローが進行不能と確定した項目を保留 |
| `WAITING` | 依存先やMilestone全体のbarrierが満たされるまで待機 |
| `REROUTABLE` | RequirementsやDesignなど、より上流の所有フェーズへ戻す |
| `EXTERNAL_BLOCK` | 環境や外部前提を満たせないため保留または停止 |

別の安全な操作があれば`CONTINUE_ELSEWHERE`、無ければ`STOP_RUN`になります。未完了の
DesignはContract reviewを止めますが、別SpecのDesignまでは止めません。未完了の実装は
その依存先とMilestone全体の完了を止めますが、独立した実装までは止めません。

作業ツリーに部分的、拒否済み、無関係、または所有者不明の変更が残った場合は例外です。
別項目へ安全に切り替えられないため、Driveはresetやstashをせず実行全体を止めます。

## 5. Driveの結果を確認する

Driveは最後に、次の内容をまとめて報告します。

- 到達したMilestoneの境界
- 完了した所有ワークフローと増えた正規の状態
- 保留項目、その原因、影響を受ける依存先
- 再開に必要な判断または外部条件
- 次に安全に実行できる操作
- Releaseを実行していないこと

保留一覧はその実行だけの報告で、永続queueではありません。判断や外部条件を解決したら、
もう一度`$specbind-drive`を実行します。Driveは最新状態から到達可能な作業を再構築します。

## 6. Release前で止める

すべての実装検証が完了すると、次の境界はRelease準備になります。対象バージョンが
未設定なら、Driveは勝手に選ばず人の判断として保留します。バージョンを明示して
Release前のbindingまで進める場合は、次のように依頼できます。

```text
$specbind-drive --target-release 1.2.0
```

この指定でも、Driveは配布物の作成、公開、検証、finalizeを実行しません。状態が
`release_ready`になったところが完了境界です。公開するときは、結果を確認してから
[リリースする](./release.md)を別途実行します。

## この進め方を選ぶ場面

- 複数SpecやDirect itemを含むMilestoneを依存順に進めたい
- 局所的な判断待ちがあっても独立作業を続けたい
- Tasks承認後の実装と検証を、正規状態を確認しながら任せたい
- 中断後に、永続queueを保守せず最新状態から再開したい

各成果物を段階ごとに確認したい場合は、
[1件ずつ計画・実装する](./implement-step-by-step.md)を使ってください。

---

[基本概念](./concepts.md) | [1件ずつ計画・実装する](./implement-step-by-step.md) | [リリースする](./release.md)
