# 新規プロジェクトで始める

このページでは、まだ実装を始めていないプロジェクトへSpecBindを導入し、最初の
機能をスコープの確認から実装の検証まで進めます。

## 0. いつ始めるのか

ここで、まったくの手探りでこれから走り始めるプロジェクトにSpecBindを導入することは、正直お勧めしません。

SpecBind は仕様駆動の考えを取り入れていますが、仕様駆動とは、**仕様作成を駆動するものではなく、仕様を使って実装・検証を駆動するもの**だと思うからです。
その足場となるべき仕様がまったく定まっていないプロジェクトに導入しても、立ち上げの足枷になるだけかもしれません。

では新規プロジェクトに導入するべきではないかというとそんなことはなく、
いわゆる「要件定義」がおおよそ終わっていて、

- 何を作るかは決まっている
- MVPやスコープもだいたい決まっている
- 画面・機能・主要ユースケースが列挙できる
- ただし実装詳細や例外系、画面間の契約までは詰め切れていない

というような状態には相性がいいと思います。
それまでの成果物を、後述するDiscoveryのインプットにして開始してみてください。

そうでなければ、まずSpecBind無しでプロトタイプを作る探索フェーズを進め、
前述の要件定義相当にプロダクトの輪郭が出てきてから導入することをお勧めします。
→ [既存プロジェクトで始める](./start-existing-project.md) 

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
specbind --version
```

`mise use`はSpecBindを`mise.toml`へ追加し、`mise lock`は選ばれたバージョンと
配布物のチェックサムを`mise.lock`へ記録します。追加または更新された2ファイルを
確認し、Gitへコミットしてください。miseを使わないインストール方法は、
[README](https://github.com/Huruikagi/specbind#install-the-cli)を参照してください。

## 3. SpecBindを導入する

このガイドではCodexを例に進めます。

```sh
specbind install --agent codex --language ja --project-instructions
```

使うコーディングエージェントに合わせて、`--agent`を選びます。

| 使うエージェント | 指定する値 |
| --- | --- |
| Codex | `codex` |
| Claude Code | `claude-code` |
| Agent Skillsと`AGENTS.md`に対応するその他のエージェント | `generic` |

複数のエージェントを使う場合は、`--agent codex --agent claude-code`のように
`--agent`を繰り返します。`--language ja`はSpecBindが管理する成果物を日本語にし、
`--project-instructions`はルートのAgent向け指示へSpecBindの案内を統合します。

書き込まれるファイルを先に確認する場合は、`--dry-run`を追加します。

```sh
specbind install --dry-run --agent codex --language ja --project-instructions
```

生成された内容をレビューし、コミットしてください。インストーラ自体はコミットを
行いません。コミット後にコーディングエージェントのセッションを開き直します。
`generic`を選んだ場合は、以降のスキル呼び出しを利用するエージェントのスキル選択や
自動Discoveryの方法に読み替えてください。

## 4. プロジェクト設定を確認する

初回のinstallが成功すると、`specbind-configure`で設定レビューを行うよう案内されます。
セッションを開き直したあと、たとえば次のように依頼してください。

```text
このプロジェクト向けにSpecBindの設定を見直して、必要な変更まで進めて。
```

設定の機械的な現在値は次のコマンドで確認できます。

```sh
specbind configuration show
```

既定値のまま使う判断も有効です。設定を変更した場合は、エージェントが示す検証と
Gitの手順を完了してから次へ進みます。

## 5. 必要なSteeringだけを用意する

Steeringは、プロジェクト全体で長く維持する目的、技術上の制約、構造上の方針を
記録する場所です。空のSteeringも有効なので、最初の機能を始めるためだけに方針を
作り足す必要はありません。

すでに決めた長期的な方針がある場合は、`specbind-steering`へbootstrapを依頼します。

Codex:

```text
$specbind-steering この新規プロジェクトで、すでに決めた長期的な方針から
最初のSteeringを提案して。書く前に内容を確認したい。
```

Claude Code:

```text
/specbind-steering この新規プロジェクトで、すでに決めた長期的な方針から
最初のSteeringを提案して。書く前に内容を確認したい。
```

スキルはプロジェクトの証拠を調べ、書く内容を先に提案します。まだ決めていない
技術や構造をSteeringで先回りして決める必要はありません。作成した場合は内容を
確認し、コミットしてから続けます。

## 6. 最初の機能をDiscoveryする

プロジェクトが持ち続ける最初の小さな機能を選び、Discoveryを依頼します。ここでは
例として、タスクを登録して一覧表示する機能を扱います。

Codex:

```text
$specbind-discovery タスクを登録して一覧表示できるようにしたい。
```

Claude Code:

```text
/specbind-discovery タスクを登録して一覧表示できるようにしたい。
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

## 7. Tasks承認まで計画する

Discoveryが報告したSpec IDを使います。ここでは`task-management`だったとします。

Codex:

```text
$specbind-plan task-management
```

Claude Code:

```text
/specbind-plan task-management
```

PlanはRequirements、Design、Design検証、Contract review、Tasksを順に実行し、
Tasksの承認まで進んだところで止まります。各Gateをまとめて承認しても、成果物の
レビューやCLIの検査は省略されません。1フェーズずつ進める場合は、対応する
`specbind-plan-*`スキルを使います。

## 8. 実装して検証する

承認済みのTasksを実装します。

Codex:

```text
$specbind-implement task-management
```

Claude Code:

```text
/specbind-implement task-management
```

全Taskが終わったら、Spec全体がRequirementsとDesignを満たしているか検証します。

Codex:

```text
$specbind-validate-implementation task-management
```

Claude Code:

```text
/specbind-validate-implementation task-management
```

検証結果が`GO`になり、CLIがcompletion evidenceを受理すれば、そのSpecの実装は
完了です。最後に状態を確認します。

```text
$specbind-status task-management
```

Claude Codeでは`/specbind-status task-management`です。最初の試用では、リリース
まで進めず、実装の検証を完了地点にすることをおすすめします。

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
状態を進める目的で手編集しないでください。現在の状態はCLIから確認できます。

```sh
specbind milestone status
specbind spec status task-management
specbind tasks list task-management
specbind artifact list task-management
```

## 次に読む

- [基本概念](./concepts.md)
- [カスタマイズ](./customization.md)
- [現在のスキル一覧](../../current-skill-index.md)
- [現在の成果物一覧](../../current-artifact-index.md)

---

[Getting Started](./getting-started.md) | [既存プロジェクトで始める](./start-existing-project.md)
