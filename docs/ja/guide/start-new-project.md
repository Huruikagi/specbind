# 新規プロジェクトで始める

このページでは、まだ実装を始めていないプロジェクトへSpecBindを導入し、最初の
リリース範囲を複数の責務へ分けて、スコープの確認から実装の検証まで進めます。

必要な環境とエージェントについては、[はじめに](./getting-started.md)の
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

## 2. CLIをインストールする

対象プロジェクトのルートで、[mise](https://mise.jdx.dev/)を使ってSpecBindを
インストールします。

```sh
mise use github:Huruikagi/specbind
mise lock
```

`mise use`はSpecBindを`mise.toml`へ追加し、`mise lock`は選ばれたバージョンと
配布物のチェックサムを`mise.lock`へ記録します。

インストールできたか確認します。

```sh
specbind --version
```

追加または更新された`mise.toml`と`mise.lock`の内容を確認し、どちらもGitへ
コミットしてください。これにより、チームで同じバージョンのSpecBindを使えます。

miseを使わないインストール方法は、
[README](https://github.com/Huruikagi/specbind#install-the-cli)を参照してください。

## 3. SpecBindを導入する

今インストールしたCLIの最初の仕事として、
SpecBindがこれから使うエージェントスキルや設定ファイルをプロジェクトに配置します。

このガイドではCodexを例に進めます。

```sh
specbind install --agent codex --language ja --project-instructions
```

### `--agent`

使うコーディングエージェントに合わせて、`--agent`の値を選んでください。

| 使うエージェント | 指定する値 |
| --- | --- |
| Codex | `codex` |
| Claude Code | `claude-code` |
| Agent Skillsと`AGENTS.md`に対応するその他のエージェント | `generic` |

複数のエージェントを使う場合は、`--agent codex --agent claude-code`のように
`--agent`を繰り返します。

`generic`が作るのは、`.agents/skills/`のAgent Skillsとルート`AGENTS.md`の
案内ブロックだけです。サブエージェント定義は作りません。

### `--language ja`

SpecBindが管理する成果物、具体的には `requirements.md` や
`design.md` の言語を日本語にします。

### `--project-instructions`

`AGENTS.md`または
`CLAUDE.md`に、マーカーで囲んだSpecBindの案内ブロックを追加します。
もともと書いてある既存の文章はそのまま残ります。
普通はつけたほうがいいでしょう。

### 書き込まれる内容を確認する

同じコマンドへ`--dry-run`を追加すると、変更を適用せずに`create`、`replace`、
`keep`の計画を確認できます。

```sh
specbind install --dry-run --agent codex --language ja --project-instructions
```

主に、次のファイルが追加されます。

```text
.specbind.json
.specbind/settings/
.agents/skills/specbind-*/       # Codexとgenericで共有
.codex/agents/specbind-*.toml    # Codexの役割別モデル設定
.claude/skills/specbind-*/       # Claude Code
.claude/agents/specbind-*.md     # Claude Codeの役割別モデル設定
AGENTS.md / CLAUDE.md            # 指示の統合を有効にした場合
```

CodexとClaude Codeには、役割ごとに使うモデルの既定値も設定されます。
変更する場合は、
[カスタマイズガイド](./customization.md)の「プロジェクト設定と役割別モデル」を
参照してください。

生成された内容をレビューし、手順1で作った土台へのコミットとは分けてコミットして
ください。SpecBindのインストーラ自体はコミットを行いません。

### セッションを開き直す

導入したら、対象プロジェクトでコーディングエージェントのセッションを開き直してください。
そうしないと、エージェントが新しいスキルを認識できないことがあります。

以降のスキル呼び出しはCodexの表記で示します。Claude Codeでは、先頭の`$`を`/`に
読み替えてください。スキル名と引数は同じです。`generic`を選んだ場合、
`specbind-*`というスキル名は同じですが、呼び出し方はエージェントごとに異なります。
利用するエージェントのスキル選択または自動Discoveryの方法に読み替えてください。

## 4. 設定は既定のまま進める

初回のインストールが成功すると、`specbind-configure`で設定レビューを行うよう案内されます。
ただし最初は、この案内に従う前に、既定値のまま最初のリリース範囲を一周することを
おすすめします。SpecBindの既定のテンプレートや判断基準は、そのまま使えるように
設計されています。特に新規プロジェクトでは、まだ判断材料が少ないので、一周して
調整したい面が見えてから`specbind-configure`に見直しを依頼すれば十分です。方法は
[プロジェクトに合わせてカスタマイズする](./customization.md)にまとめています。

## 5. 最初のリリース範囲を用意する

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

すでに決めた長期的な方針がある場合は、Discoveryの前に`specbind-steering`へ
`bootstrap`モードでの作成を依頼しておくことができます。

```text
$specbind-steering この新規プロジェクトで、すでに決めた長期的な方針から
最初のSteeringを提案して。書く前に内容を確認したい。
```

スキルはプロジェクトの証拠を調べ、書く内容を先に提案します。まだ決めていない
技術や構造を先回りして決める必要はありません。作成した場合は内容を確認し、
コミットしてから続けます。

## 6. Discoveryでスコープを確認する

用意したディレクトリを、最初のリリース範囲としてdiscoveryスキルへ渡します。

```text
$specbind-discovery docs/product-definition/ の内容を最初のリリース範囲としたい
```

Discoveryはコレクションを全部棚卸しし、各ファイルをどの作業に使うか、今回は
使わないかを示します。細かい要件や設計は後続のPlanで詰めます。技術や構成の選定も
Discoveryの範囲外で、決まっていれば5節のSteering、まだなら後続のDesignで扱います。

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
- **入力資料の網羅状況（Source coverage）** — 棚卸しした全ファイルと、その振り分け先または不使用の理由（資料を渡したときに付く）

入力に取りこぼしがないこと、責務の境界、依存関係に納得してから承認します。ここでの
結論が以降のワークフロー全体の前提になります。

承認すると、CLIがMilestoneとSpecの状態を作ります。エージェントは、Roadmapに
コレクション全体の振り分けを、各Specの作業メモ（`brief.md`）にはそのSpecが参照
する資料だけを記録します。資料は仕様そのものではないので、後続のRequirementsと
Designが該当ファイルを読み、採用した内容を正規の成果物へ書き直します。

途中で状態を確認したくなったら、次のように依頼できます。

```text
$specbind-status
```

このスキルは読み取り専用で、承認したり成果物を書き換えたりはしません。

## 7. 計画と実装の進め方を選ぶ

最初のリリース範囲には複数のSpecがあるため、通常は次の組み合わせで進めます。

```text
$specbind-plan --all
$specbind-drive
```

PlanでMilestone全体のRequirements、Design、Contractレビュー、Tasksを確定し、Driveで
安全に到達可能な実装と検証を進めます。局所的な判断待ちは保留して独立項目を続け、
リリース前で停止します。詳しい動作と停止条件は
[PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)を参照してください。

各成果物とGateを段階ごとに確認する場合は、
[1件ずつ計画・実装する](./implement-step-by-step.md)へ進みます。どちらも同じ所有スキル、
レビュー、CLIが記録する証拠を使い、各境界を自分で選ぶ粒度だけが異なります。

## 8. 生成された成果物を見る

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
- [プロジェクトに合わせてカスタマイズする](./customization.md) — 一周して調整したい点が見えてから
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）

---

[はじめに](./getting-started.md) | [既存プロジェクトで始める](./start-existing-project.md)
