# 既存プロジェクトで始める

このページでは、既存のプロジェクトにSpecBindを導入し、コーディングエージェントと
一緒に、最初の変更を計画・実装・検証まで進めてみます。

まだ実装を始めていない場合は、[新規プロジェクトで始める](./start-new-project.md)へ
進んでください。必要な環境とエージェントについては、
[ルートを選ぶ](./getting-started.md)の「どちらのルートでも必要なもの」を
確認しておいてください。

!!! info "用語について"
    出てくる用語（Spec、Steering、Milestone、Gate など）は[基本概念](./concepts.md)
    にまとめています。先に目を通しても、出てきたときに参照してもかまいません。

## 1. SpecBindをインストールする

未コミットの変更が残っている場合は、内容を確認していつもの手順でコミットして
おいてください。そのうえで、両ルート共通の
[SpecBindをインストールする](./install.md)を実行します。

```sh
mise use github:Huruikagi/specbind
mise lock
specbind install --agent codex --language ja --project-instructions
```

エージェントの選び方、`--dry-run`での事前確認、書き込まれるファイル、コミットと
セッションの開き直しは、そのページを参照してください。

インストールが済んだら、このページへ戻ってきてください。以降のスキル呼び出しは
Codexの表記（`$`）で示します。Claude Codeでは`/`に読み替えてください。

## 2. 最初の進め方を選ぶ

既存プロジェクトには、目的の異なる2つの始め方があります。

| 目的 | 次に使うワークフロー |
| --- | --- |
| これから行う変更をSpecBindで進める | このページの続きを進める |
| すでに動いている実装から、現在の基準となるSpecを確立する | [既存実装からSpecを確立する](./adopt-existing.md) |

最初の小さな変更を進めるなら、既定値のまま通常のライフサイクルを一周する方法が
分かりやすいでしょう。実際に合わないと分かった面だけ、あとから
[カスタマイズ](./customization.md)で調整できます。そのままこのページを続けて
ください。

すでに相当量のコードがある一方で信頼できるSpecがまだなく、まず現在のプロダクトを
Specとして固定したい場合は、[既存実装からSpecを確立する](./adopt-existing.md)へ
進んでください。Steeringと共通設定を整えてから、Requirements、Design、Contract
Reviewまでを確定し、非リリースの基準履歴として閉じる経路です。

## 3. 最初の変更を選ぶ

最初は、1つの振る舞いを追加するだけの小さな変更を選んでください。複数の機能や
リリース作業をまとめて試すのは避けます。

このガイドでは、既存のアプリケーションに次の変更を加える例で説明します。

> 一覧画面に表示している内容を、CSVファイルとして
> ダウンロードできるようにしたい。

この例は、プロジェクトが持ち続ける新しい振る舞いと境界を作るため、該当する
既存Specがなければ新規Specに分類される想定です。出力するCSVの列や形式は、
あとから他の機能や利用者が依存する外部との約束になるので、Contractとしても
扱いやすい題材です。実際に試すときは、自分のプロジェクトにある同じくらいの
規模の変更に置き換えてください。

## 4. Discoveryでスコープを確認する

変更内容を添えて、discoveryスキルをエージェントに依頼します。関連するissueやメモが
あれば、その場所も伝えます。

```text
$sb-discovery 一覧画面に表示している内容を、CSVファイルとして
ダウンロードできるようにしたい。
```

細かい要件や設計はこのあとのPlanで詰めるので、ここで全部渡す必要はありません。
使う技術や実装方針の選定もDiscoveryの仕事ではありません。

Discoveryは、プロジェクトの現在の状態（Spec、Steering、Milestone）を読んだうえで、
変更を **Direct**（既存の仕様を変えずにできる小さな変更）、**既存Specの更新**
（すでにある能力の振る舞いや境界を変える）、**新規Spec**（プロジェクトに新しい
責務を1つ増やす）のいずれかに分類します。

!!! info "用語: Spec"
    Specは「プロジェクトが持ち続ける1つの能力の境界」で、責務を表す短い
    kebab-caseのIDが付きます（用語は[基本概念](./concepts.md)）。

分類の結果は、次の項目にまとめて提案されるので、確認してください。

- **作業項目（Work items）** — 今回行う作業の一覧
- **新規Spec（New Specs）** — 新しく作る責務の境界
- **Gateの無効化（Gate invalidations）** — やり直しになる既存の承認
- **依存関係（Dependencies）** — 作業どうしの依存関係

ここでの結論が以降のワークフロー全体の前提になります。分類と境界を必ず読んでから
承認してください。承認すると、CLIがMilestoneとSpecの状態を作り、エージェントが、
その変更の要点をまとめた作業メモ（`brief.md`）を書きます。

途中で状態を確認したくなったら、次のように依頼できます。

```text
$sb-status
```

このスキルは読み取り専用で、承認したり成果物を書き換えたりはしません。

## 5. 計画と実装の進め方を選ぶ

最初の`csv-export`を段階ごとに確認する場合は、`sb-plan`のフェーズを
明示して順に進めます。

```text
$sb-plan csv-export requirements
$sb-plan csv-export design
$sb-contract-review
$sb-plan csv-export tasks
$sb-implement csv-export
$sb-validate-implementation csv-export
```

各段階で確認する内容、承認、上流へ戻る場合の扱いは、
[1件ずつ計画・実装する](./implement-step-by-step.md)にまとめています。

複数のSpecやDirect項目を含むMilestoneでは、`$sb-plan --all`のあとに
`$sb-drive`を使います。保留、別項目への継続、停止条件は
[PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)を参照してください。
どちらの経路もリリース前で停止します。

## 6. 生成された成果物を見る

Specの成果物は、既定では`.specbind/specs/<spec>/`にできます。

```text
.specbind/
├─ steering/roadmap.md
└─ specs/csv-export/
   ├─ spec.yaml
   ├─ brief.md
   ├─ requirements.md
   ├─ design.md
   ├─ contract.yaml
   └─ tasks.yaml
```

`spec.yaml`、`roadmap.md`、`tasks.yaml`に入っている実行状態は、CLIの持ち物です。
状態を進める目的で手編集しないでください。Requirements、Design、Contract、
Tasksの計画部分は、`sb-plan`の対応するフェーズを通して保守します。

現在の状態は、CLIから直接確認することもできます。

```sh
specbind milestone status
specbind spec status csv-export
specbind tasks list csv-export
specbind artifact list csv-export
```

周辺ツールやスクリプトからMilestoneとSpecの状態を読む場合に限り、2つの
`status`コマンドはコマンド固有のJSON出力も提供します。通常の利用では既定の
簡潔なテキスト出力をそのまま使います。

```sh
specbind milestone status --json
specbind spec status csv-export --json
```

## 次に読む

- [基本概念](./concepts.md)
- [1件ずつ計画・実装する](./implement-step-by-step.md)
- [PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)
- [既存実装からSpecを確立する](./adopt-existing.md) — 現在のコードを基準Specにする
- [リリースする](./release.md) — Milestoneを実際に締めるとき
- [カスタマイズ](./customization.md) — 一周して調整したい点が見えてから
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）

---

[ユーザーガイド](../index.md) | [SpecBindをインストールする](./install.md) | [新規プロジェクトで始める](./start-new-project.md)
