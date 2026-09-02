# 新規プロジェクトで始める

このページでは、まだ実装を始めていないプロジェクトへSpecBindを導入し、最初の
リリース範囲を複数の責務へ分けて、スコープの確認から実装の検証まで進めます。

必要な環境とエージェントについては、[ルートを選ぶ](./getting-started.md)の
「どちらのルートでも必要なもの」を確認しておいてください。

!!! info "用語について"
    出てくる用語（Spec、Steering、Milestone、Gate など）は[基本概念](./concepts.md)
    にまとめています。先に目を通しても、出てきたときに参照してもかまいません。

## 始める前に

SpecBindが得意なのは、何を作るかをゼロから探索することではありません。ある程度
見えてきたプロダクトの輪郭を仕様として定着させ、その仕様で計画・実装・検証を
駆動することです。MVPや主要ユースケースが頻繁に入れ替わる段階では、SpecBindの
成果物が探索の足枷になることがあります。

次のような状態になっていれば、このルートで始められます。

- 何を作るかは決まっている
- MVPやスコープもだいたい決まっている
- 画面・機能・主要ユースケースが列挙できる
- ただし実装詳細や例外系、画面間の契約までは詰め切れていない

まだこの状態でなければ、先にSpecBindを使わずにプロトタイプを作る探索フェーズを進め、
維持したいプロダクト意図と最初の責務が見えてから導入することをおすすめします。
その場合は[既存プロジェクトで始める](./start-existing-project.md)へ進んでください。

なお、導入前に要件定義をすべて終える必要はありません。詰め切れていない内容は
RequirementsやDesignで具体化できます。それまでの成果物は、後述するDiscoveryの
インプットにできます。

## 1. プロジェクトの土台を作る

対象プロジェクトをGitリポジトリとして初期化します。少なくともプロジェクトの目的を
短く書いたREADMEを用意し、ライセンス、言語やフレームワークの設定など、すでに
決めた土台があれば一緒に追加してください。利用した雛形生成ツールがGitも初期化した
場合は、`git init`を省略できます。

最初のMilestoneはGitコミットを基準にするため、SpecBindでDiscoveryを始める前に、
この土台をコミットしておく必要があります。

```sh
git init
git status --short
git add .
git commit -m "Initialize project"
```

`git status --short`で追加対象を確認してからステージしてください。コミットする内容や
メッセージは、プロジェクトの方針に合わせます。

## 2. SpecBindをインストールする

CLIの導入とプロジェクトへの配置は、両ルート共通の
[SpecBindをインストールする](./install.md)にまとめています。

```sh
mise use github:Huruikagi/specbind
mise lock
specbind install --agent codex --language ja --project-instructions
```

エージェントの選び方、`--dry-run`での事前確認、書き込まれるファイル、コミットと
セッションの開き直しは、そのページを参照してください。手順1で作った土台とは
分けてコミットします。

インストールが済んだら、このページへ戻ってきてください。以降のスキル呼び出しは
Codexの表記（`$`）で示します。Claude Codeでは`/`に読み替えてください。

## 3. 設定は既定のまま進める

初回のインストールが成功すると、`sb-configure`で設定レビューを行うよう案内されます。
ただし最初は、この案内に従う前に、既定値のまま最初のリリース範囲を一周することを
おすすめします。SpecBindの既定のテンプレートや判断基準は、そのまま使えるように
設計されています。特に新規プロジェクトでは、まだ判断材料が少ないので、一周して
調整したい面が見えてから`sb-configure`に見直しを依頼すれば十分です。方法は
[カスタマイズ](./customization.md)にまとめています。

## 4. 最初のリリース範囲を用意する

新規プロジェクトでは、1つの小さな機能に絞る必要はありません。最初のリリースで
一緒に届けたいプロダクト範囲をまとめて用意します。Discoveryが入力全体を読み、
長く維持する責務の境界、作業順序、Spec間の依存へ振り分けます。

このガイドでは、タスクの登録・一覧・完了と、期限前のリマインダーを最初のリリース
範囲とします。すでにある要件定義の成果物を、プロジェクト内のディレクトリへ
置いたとします。

```text
docs/product-definition/
├─ task-management.md
└─ reminders.md
```

Discoveryへ渡せるのは、プロジェクト内にあってGitで追跡済みのテキストファイル
またはディレクトリです（Source Collection）。ディレクトリは配下を再帰的に
棚卸しします。未追跡・シンボリックリンク・読めない形式が混ざっていると一部だけで
進めずに停止するので、渡す前に資料を確認し、手順1の土台と一緒にコミットして
おいてください。

### すでに決めた長期的な方針がある場合

Steeringは、プロジェクト全体で長く維持する目的、技術上の制約、構造上の方針を
記録する場所です。空のSteeringも有効なので、最初の範囲を始めるためだけに方針を
作り足す必要はありません。

すでに決めた長期的な方針がある場合は、Discoveryの前に`sb-steering`へ
`bootstrap`モードでの作成を依頼しておくことができます。

```text
$sb-steering この新規プロジェクトで、すでに決めた長期的な方針から
最初のSteeringを提案して。書く前に内容を確認したい。
```

スキルはプロジェクトの証拠を調べ、書く内容を先に提案します。まだ決めていない
技術や構造を先回りして決める必要はありません。作成した場合は内容を確認し、
コミットしてから続けます。

## 5. Discoveryでスコープを確認する

用意したディレクトリを、最初のリリース範囲としてdiscoveryスキルへ渡します。

```text
$sb-discovery docs/product-definition/ の内容を最初のリリース範囲としたい
```

Discoveryはコレクションを全部棚卸しし、各ファイルをどの作業に使うか、今回は
使わないかを示します。細かい要件や設計は後続のPlanで詰めます。技術や構成の選定も
Discoveryの範囲外で、決まっていれば手順4のSteering、まだなら後続のDesignで扱います。

Discoveryは、プロジェクトの現在の状態（Spec、Steering、Milestone）を読んだうえで、
入力を分類します。新規プロジェクトではまだ既存のSpecが無いので、今回の範囲は
**Direct**（既存の仕様を変えずにできる小さな変更）か **新規Spec**（プロジェクトに
新しい責務を1つ増やす）のいずれかになります。

!!! info "用語: Spec"
    Specは「プロジェクトが持ち続ける1つの能力の境界」で、責務を表す短い
    kebab-caseのIDが付きます（用語は[基本概念](./concepts.md)）。今回の範囲が
    タスク管理とリマインダーの2つの責務に分かれるなら、それぞれ別の新規Spec
    になります。

分類の結果は、次の項目にまとめて提案されるので、確認してください。

- **作業項目（Work items）** — 今回行う作業の一覧
- **新規Spec（New Specs）** — 新しく作る責務の境界
- **Gateの無効化（Gate invalidations）** — やり直しになる既存の承認
- **依存関係（Dependencies）** — 作業どうしの依存関係（例: リマインダー → タスク管理）
- **入力資料の網羅状況（Source coverage）** — 棚卸しした全ファイルと、その振り分け先
  または不使用の理由（資料を渡したときだけ付きます）

入力に取りこぼしがないこと、責務の境界、依存関係に納得してから承認します。ここでの
結論が以降のワークフロー全体の前提になります。

承認すると、CLIがMilestoneとSpecの状態を作ります。エージェントは、Roadmapに
コレクション全体の振り分けを、各Specの作業メモ（`brief.md`）にはそのSpecが参照
する資料だけを記録します。資料は仕様そのものではないので、後続のRequirementsと
Designが該当ファイルを読み、採用した内容を正規の成果物へ書き直します。

途中で状態を確認したくなったら、次のように依頼できます。

```text
$sb-status
```

このスキルは読み取り専用で、承認したり成果物を書き換えたりはしません。

## 6. 計画と実装の進め方を選ぶ

最初のリリース範囲には複数のSpecがあるため、通常は次の組み合わせで進めます。

```text
$sb-plan --all
$sb-drive
```

PlanでMilestone全体のRequirements、Design、Contractレビュー、Tasksを確定し、Driveで
安全に到達可能な実装と検証を進めます。局所的な判断待ちは保留して独立項目を続け、
リリース前で停止します。詳しい動作と停止条件は
[PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)を参照してください。

各成果物とGateを段階ごとに確認する場合は、
[1件ずつ計画・実装する](./implement-step-by-step.md)へ進みます。どちらも同じ所有スキル、
レビュー、CLIが記録する証拠を使い、各境界を自分で選ぶ粒度だけが異なります。

## 7. 生成された成果物を見る

Specの成果物は、既定では`.specbind/specs/<spec>/`にできます。

```text
.specbind/
├─ steering/roadmap.md
└─ specs/
   ├─ task-management/
   │  ├─ spec.yaml
   │  ├─ brief.md
   │  ├─ requirements.md
   │  ├─ design.md
   │  ├─ contract.yaml
   │  └─ tasks.yaml
   └─ reminders/
      └─ ...
```

`spec.yaml`、`roadmap.md`、`tasks.yaml`に入っている実行状態はCLIの持ち物です。
状態を進める目的で手編集しないでください。Requirements、Design、Contract、
Tasksの計画部分は、それぞれを所有するスキル経由で保守します。

現在の状態は、CLIから直接確認することもできます。

```sh
specbind milestone status
specbind spec status <spec-id>
specbind tasks list <spec-id>
specbind artifact list <spec-id>
```

周辺ツールやスクリプトからMilestoneとSpecの状態を読む場合に限り、2つの
`status`コマンドはコマンド固有のJSON出力も提供します。通常の利用では既定の
簡潔なテキスト出力をそのまま使います。

```sh
specbind milestone status --json
specbind spec status <spec-id> --json
```

## 次に読む

- [基本概念](./concepts.md)
- [1件ずつ計画・実装する](./implement-step-by-step.md)
- [PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)
- [リリースする](./release.md) — Milestoneを実際に締めるとき
- [カスタマイズ](./customization.md) — 一周して調整したい点が見えてから
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）

---

[ユーザーガイド](../index.md) | [SpecBindをインストールする](./install.md) | [既存プロジェクトで始める](./start-existing-project.md)
