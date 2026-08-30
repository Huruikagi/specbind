# 新規プロジェクトで始める

このページでは、まだ実装を始めていないプロジェクトへSpecBindを導入し、最初の
機能をスコープの確認から実装の検証まで進めます。

必要な環境とエージェントについては、[Getting Started](./getting-started.md)の
「どちらのルートでも必要なもの」を確認しておいてください。

## 0. いつ始めるのか

ここで、まったくの手探りでこれから走り始めるプロジェクトにSpecBindを
導入することは、正直お勧めしません。

SpecBindが得意なのは、何を作るかをゼロから探索することではありません。
ある程度見えてきたプロダクトの輪郭を仕様として定着させ、
その仕様を使って計画・実装・検証を駆動することです。
MVPや主要ユースケースが頻繁に入れ替わる段階では、
SpecBindの成果物が探索の足枷になるかもしれません。

かといって、導入前に要件定義をすべて終える必要もありません。
ある程度話が進んでいて、たとえば次のような状態のとき。

- 何を作るかは決まっている
- MVPやスコープもだいたい決まっている
- 画面・機能・主要ユースケースが列挙できる
- ただし実装詳細や例外系、画面間の契約までは詰め切れていない

この段階なら、まだ詰め切れていない内容をRequirementsやDesignで具体化できます。
それまでの成果物を、後述するDiscoveryのインプットにして開始してみてください。

そうでなければ、まずSpecBind無しでプロトタイプを作る探索フェーズを進め、
維持するプロダクト意図と最初の責務が見えてきてから導入することをお勧めします。
そうしてから[既存プロジェクトで始める](./start-existing-project.md)を参照してください。

## 1. プロジェクトの土台を作る

対象プロジェクトをGitリポジトリとして初期化します。少なくともプロジェクトの目的を
短く書いたREADMEを用意し、ライセンス、言語やフレームワークの設定など、すでに
決めた土台があれば一緒に追加してください。利用した雛形生成ツールがGitも初期化した
場合は、`git init`を省略できます。

最初のMilestoneはGit commitを基準にするため、SpecBindでDiscoveryを始める前に、
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

導入したら、対象プロジェクトでcoding agentのセッションを開き直してください。
そうしないと、エージェントが新しいスキルを認識できないことがあります。

以降のスキル呼び出しはCodexの表記で示します。Claude Codeでは、先頭の`$`を`/`に
読み替えてください。スキル名と引数は同じです。`generic`を選んだ場合、
`specbind-*`というスキル名は同じですが、呼び出し方はagentごとに異なります。
利用するagentのスキル選択または自動Discoveryの方法に読み替えてください。

## 4. 設定は既定のまま進める

初回のinstallが成功すると、`specbind-configure`で設定レビューを行うよう案内されます。
ただし最初は、この案内に従う前に、既定値のまま最初の機能を一周することを
おすすめします。SpecBindの既定のテンプレートや判断基準は、そのまま使えるように
設計されています。特に新規プロジェクトでは、まだ判断材料が少ないので、一周して
調整したい面が見えてから`specbind-configure`に見直しを依頼すれば十分です。方法は
[プロジェクトに合わせてカスタマイズする](./customization.md)にまとめています。

## 5. 必要なSteeringだけを用意する

Steeringは、プロジェクト全体で長く維持する目的、技術上の制約、構造上の方針を
記録する場所です。空のSteeringも有効なので、最初の機能を始めるためだけに方針を
作り足す必要はありません。

すでに決めた長期的な方針がある場合は、`specbind-steering`へbootstrapを依頼します。

```text
$specbind-steering この新規プロジェクトで、すでに決めた長期的な方針から
最初のSteeringを提案して。書く前に内容を確認したい。
```

スキルはプロジェクトの証拠を調べ、書く内容を先に提案します。まだ決めていない
技術や構造をSteeringで先回りして決める必要はありません。作成した場合は内容を
確認し、コミットしてから続けます。

## 6. 最初の機能をDiscoveryする

プロジェクトが持ち続ける最初の小さな機能を選び、Discoveryを依頼します。ここでは
例として、タスクを登録して一覧表示する機能を扱います。

```text
$specbind-discovery タスクを登録して一覧表示できるようにしたい。
```

Discoveryは、現在のSpec、Steering、Milestoneを読み、Direct、既存Specの更新、
新規Specのどれに当たるかを提示します。新規プロジェクトの最初の持続的な機能は、
通常は新規Specの候補です。

スキルが示す次の4項目を読み、境界と依存関係に納得してから承認してください。

- Work items
- New Specs
- Gate invalidations
- Dependencies

承認後、CLIがMilestoneとSpecの状態を作り、エージェントが`brief.md`を書きます。

途中で状態を確認したくなったら、次のように依頼できます。

```text
$specbind-status
```

このスキルは読み取り専用で、承認したり成果物を書き換えたりはしません。

## 7. Tasks承認まで計画する

Discoveryが報告したSpec IDを使います。ここでは`task-management`だったとします。

```text
$specbind-plan task-management
```

PlanはRequirements、Design、Design検証、Contract review、Tasksを順に実行し、
Tasksの承認まで進んだところで止まります。実行の最初に、Requirements、Design、
Tasksの各Gateをこの実行の中でまとめて承認してよいか聞かれます。

まとめて承認しても、レビューやCLIの検査は省略されません。各Gateで個別に行う
確認を、1回の実行に対する確認へまとめるだけです。1つずつ内容を見ながら進めたい
場合は、まとめての承認を断れば、各フェーズで個別に承認できます。

Planが終わった時点では、実装はまだ始まっていません。Requirements、Design、Tasksの
どれか1フェーズだけを明示的に進めたい場合は、対応する`specbind-plan-*`スキルを使います。

## 8. 実装して検証する

承認済みのTasksを実装します。

```text
$specbind-implement task-management
```

Implementが扱うのは1つのRoadmap itemだけで、着手できるTaskから順に実装します。
Spec-backed itemでは、Taskごとに実装、レビュー、CLIへの進捗記録を行います。
計画やDesignの問題が見つかった場合は、無理に実装を続けず、該当フェーズへ戻す
ためにいったん停止します。

全Taskが終わったら、Spec全体がRequirementsとDesignを満たしているか検証します。

```text
$specbind-validate-implementation task-management
```

検証結果が`GO`になり、CLIがcompletion evidenceを受理すれば、そのSpecの実装は
完了です。最後に状態を確認します。

```text
$specbind-status task-management
```

この時点では、Milestoneはまだリリースされていません。リリースするには、
プロジェクト固有の`.specbind/settings/adapters/release.md`を用意し、対象
リリースを実際に公開・検証できる状態にしておく必要があります。最初の試用では、
実装の検証までを完了地点にすることをおすすめします。

## 9. 生成された成果物を見る

Specの成果物は、既定では`.specbind/specs/<spec>/`にできます。

```text
.specbind/
├─ steering/
│  └─ roadmap.md
└─ specs/task-management/
   ├─ spec.yaml
   ├─ brief.md
   ├─ requirements.md
   ├─ design.md
   ├─ contract.yaml
   └─ tasks.yaml
```

`spec.yaml`、`roadmap.md`、`tasks.yaml`に入っている実行状態はCLIの持ち物です。
状態を進める目的で手編集しないでください。Requirements、Design、Contract、
Tasksの計画部分は、それぞれを所有するスキル経由で保守します。

現在の状態は、CLIから直接確認することもできます。

```sh
specbind milestone status
specbind spec status task-management
specbind tasks list task-management
specbind artifact list task-management
```

## 次に読む

- [基本概念](./concepts.md)
- [プロジェクトに合わせてカスタマイズする](./customization.md) — 一周して調整したい点が見えてから
- [現在のスキル一覧](../../current-skill-index.md)
- [現在の成果物一覧](../../current-artifact-index.md)

---

[Getting Started](./getting-started.md) | [既存プロジェクトで始める](./start-existing-project.md)
