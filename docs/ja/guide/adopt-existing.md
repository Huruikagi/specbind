# 既存実装からSpecを確立する

`specbind-adopt-existing`は、すでに動いているコードとテストを調査し、
新しいSpecBind Specの候補へ整理するための初期導入ワークフローです。
cc-sddなど別のSDD製品からの移行ではなく、信頼できる仕様がまだない
既存プロジェクトを対象にします。

既存実装は証拠であって、意図した仕様そのものではありません。現在の挙動は、
正式な要件、バグ、歴史的制約、内部実装、または判断が必要な不明点になり得ます。
ユーザーが確認した挙動だけがBriefを経てRequirementsへ進みます。

!!! info "用語について"
    用語（Spec、Steering、Milestone、Brief、Research など）は[基本概念](./concepts.md)、
    既存プロジェクトの通常の流れは
    [既存プロジェクトで始める](./start-existing-project.md)にまとめています。

## 前提条件

初回の導入では、次が必要です。

- 永続的なSpecがまだ存在しない
- アクティブなMilestoneがない
- Steeringがプロダクト、技術、構造の全体方針を説明している
- Steeringを含むリポジトリがコミット済みで、作業ツリーに未コミットの変更がない
- 採用対象が「リポジトリ全体」または具体的な領域として指定されている

Steeringがまだなければ、先に`specbind-steering`を`bootstrap`モードで実行し、
提案された方針を確認してコミットします。

## 標準ルート

```text
Steeringの作成または同期
  -> specbind-adopt-existing
  -> Spec境界候補の確認
  -> specbind-discovery
  -> SpecとBriefの作成
  -> specbind-adopt-existingを再開
  -> Specごとの観察と意図の確認
  -> specbind-plan
```

最初の実行では、CLIが事前検査（preflight）を行い、前提条件がそろって
いるかを確認します。

```sh
specbind adoption preflight
```

成功結果の`source_revision`は、調査の基準として固定するGitコミットです。調査中に
実装、テスト、依存関係、設定、またはSteeringが変わった場合、結果を暗黙に
追従させず停止します。

## 調査の深さ

まずリポジトリ全体を浅く読み、公開API、主要エントリポイント、モジュール境界、
テスト群、依存関係を把握します。その後、ユーザーが指定した採用領域だけを深く
調査します。ディレクトリの大きさや想定タスク数ではなく、長く残る責任の境界で
Specを分割します。

Spec境界は、ユーザーが確認するまで作成されません。確認後も、通常の
`specbind-discovery`がRoadmapのスコープをもう一度提示し、MilestoneとSpecについて
CLIが所有する変更を担当します。

## 観察結果と意図

調査で見つかった挙動（Observation、以降は「観察結果」）は、一時ファイル
`.specbind/specs/adoption/reverse-discovery.yaml`（既定。場所は`.specbind.json`の
`specDir`設定で変わります）にいったん記録されます。各観察結果は、固定した
リビジョン上のパスと、シンボル、テスト名、ルート、スキーマ項目などの位置を持ちます。

各観察結果は、意図した仕様なのかどうかで、次のいずれかへ振り分けます。

| 扱い | 意味 |
| --- | --- |
| requirement | 意図した挙動としてBriefからRequirementsへ進める |
| design | 技術・構造上の制約としてResearchからDesignへ進める |
| bug | 現在の挙動を仕様にせず、必要なら通常の修正作業へ入れる |
| historical_constraint | 当面維持するが、プロダクトの約束にはしない |
| implementation_detail | 仕様化しない内部詳細 |
| unknown | RequirementsまたはDesignで判断する未確定事項 |

すべてのSpecについてBriefとResearchへの引き継ぎが完了すると、このプロジェクト単位の
調査記録は現在のファイル群から削除されます。Git履歴には調査経緯が残り、Specごとの
Researchは通常のリリース確定処理まで
保持されます。

## 通常ライフサイクルへの復帰

採用スキルはRequirementsやDesignを直接作成・承認しません。確認済みの意図を
Briefへ、実装証拠とDesign向け制約をResearchへ渡したところで停止します。
以後は`specbind-plan`で通常のRequirements、Design、Tasksフェーズを進めます。
既存実装からの採用専用となるRequirementsやDesignスキルはありません。

## 次に読む

- [基本概念](./concepts.md)
- [既存プロジェクトで始める](./start-existing-project.md) — 通常のライフサイクルを一周する
- [プロジェクトに合わせてカスタマイズする](./customization.md) — 一周して調整したい点が見えてから
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）

---

[はじめに](./getting-started.md) | [既存プロジェクトで始める](./start-existing-project.md)
