# 1件ずつ計画・実装する

このページでは、1つのSpec項目（Spec-backed item）をRequirementsから実装検証まで、成果物と
Gateを段階ごとに確認しながら進めます。SpecBindを初めて使うとき、影響の大きい変更を
慎重に進めたいとき、または特定のフェーズだけをやり直すときに向いています。

Milestoneと対象Specは、先にDiscoveryで作成しておきます。まだ進行中のMilestoneが
なければ、[新規プロジェクト](./start-new-project.md)または
[既存プロジェクト](./start-existing-project.md)の導入手順から始めてください。

以下では対象Specを`csv-export`とします。Codexでは`$`、Claude Codeでは`/`で
スキルを呼び出します。

## 1. 現在地と対象を確認する

```text
$sb-status
```

Roadmapの分類、依存関係、現在有効なGate、次に進められる操作を確認します。この
ページの手順はSpec項目向けです。Direct項目にはRequirements、Design、
Contract、Tasksを作らず、`sb-implement <item-id>`へ直接進みます。

## 2. Requirementsを作成する

```text
$sb-plan csv-export requirements
```

Requirementsは今回の差分だけではなく、そのSpecが現在保証する振る舞い全体です。
BriefがSource Itemsを指定している場合は、採用する内容が正規のRequirementと
Acceptance Criteriaへ書き直されていることを確認します。

内容をレビューし、Requirements Gateを承認します。判断が必要な点が残っている場合は、
先へ進めるために曖昧な文言へ丸めず、ここで解決します。

## 3. DesignとContractを作成・検証する

```text
$sb-plan csv-export design
```

DesignはRequirementsをどう実現するかを定め、Contractは外部責任、依存関係、所有する
ファイル境界を構造化します。Design作成後には、独立した検証がRequirementsの網羅、
責任境界、検証可能性を確認します。検証が`READY`になってからDesign Gateを承認します。

検証で問題が見つかった場合は、具体的な指摘をDesignへ反映して再検証します。
Requirements自体の問題なら、Designで補わずRequirementsへ戻します。

## 4. Milestone全体のContractをレビューする

```text
$sb-contract-review
```

Contractレビューは1つのSpecだけを見る処理ではありません。進行中のMilestoneに含まれる
全SpecのDesignが準備できてから、所有権の重複、循環依存、互換性、統合時の抜けを
まとめて確認します。

ほかのSpecのDesignが未完了なら、この時点で待つ必要があります。そのSpecも個別に
手順3まで進めるか、複数Specをまとめて計画する場合は
[PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)を使います。

## 5. Tasksを作成する

```text
$sb-plan csv-export tasks
```

TasksはRequirementsとDesignを実行可能な順序へ分解します。各Taskの対象、完了条件、
検証方法、Requirementへの対応を確認してからTasks Gateを承認します。

Tasks承認までは計画です。この時点では実装は始まっていません。

## 6. 1つのRoadmap項目を実装する

```text
$sb-implement csv-export
```

Implementは1回につき1つのRoadmap項目を担当します。Spec項目ではTaskを順番に
処理し、各Taskについて実装、レビュー、検証、CLIへの進捗記録、プロジェクトの
アダプターが定めたチェックポイントを完了してから次へ進みます。

実装中にRequirements、Design、Contract、Tasksの問題が見つかった場合、Implementは
上流成果物をその場で書き換えません。新しい診断で所有フェーズを特定し、必要なGateを
明示的に無効化して、そのフェーズのスキルからやり直します。

## 7. Spec全体を検証する

全Taskが完了したら、個々のTaskではなくSpec全体を検証します。

```text
$sb-validate-implementation csv-export
$sb-status csv-export
```

検証は現在のRequirementsとDesignに対して実装を評価します。結果が`GO`となり、CLIが
完了を裏付ける記録（completion evidence）を受理すると、そのSpec項目は完了です。依存していた
別のRoadmap項目が新たに着手可能になることもあります。

## 8. 次の境界を選ぶ

Milestoneに未完了項目があれば、次の項目で同じ手順を繰り返します。ここからまとめて
進めたい場合は`sb-drive`へ切り替えてもかまいません。Driveは現在のCLI状態から
再開するため、このページで進めた内容をやり直しません。

すべての実装検証が終わっても、リリースはまだ実行されていません。公開とMilestoneの
確定処理は、別途[リリースする](./release.md)から明示的に進めます。

## この進め方を選ぶ場面

- 最初の1件で各成果物とGateの役割を理解したい
- Requirementや設計判断を段階ごとに自分で確認したい
- 影響の大きいSpecをほかの項目と切り離して進めたい
- 無効化後に、所有フェーズから限定的にやり直したい

Milestoneに複数の独立項目があり、安全に進められる範囲をまとめて進めたい場合は、
[PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)へ進んでください。

---

[ユーザーガイド](../index.md) | [PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md) | [リリースする](./release.md)
