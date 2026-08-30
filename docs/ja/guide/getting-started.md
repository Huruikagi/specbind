# Getting Started

このページでは、既存のプロジェクトにSpecBindを導入し、コーディングエージェントと
一緒に、最初の変更を計画・実装・検証まで進めてみます。

## 1. 前提を確認する

次のものを用意してください。

- Gitで管理していて、コミットが1つ以上ある対象プロジェクト
- コーディングエージェント
- Windows x64、WSL2上のLinux x64、またはmacOS ARM64

### 1.1. 使えるエージェントについて

開発者はCodexとClaude Codeを利用しており、この２つでテストを進めています。

それ以外のコーディングエージェントでも、
Agent Skillsと`AGENTS.md`に対応していれば
（試してないけど）多分動くはずです。
具体的には

- Cursor
- GitHub Copilot
- Devin
- ほか...

うまくいかなかったら[バグ報告と改善提案](./feedback.md)から教えてください。

## 2. CLIをインストールする

対象プロジェクトのルートへ移動します。未コミットの変更が残っている場合は、
内容を確認し、いつもの手順でコミットしておいてください。

このガイドでは、[mise](https://mise.jdx.dev/)を使ってSpecBindをインストールします。

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

## 3. 対象プロジェクトへ導入する

### 3.1. インストールする

今インストールしたCLIの最初の仕事として、
SpecBindがこれから使うエージェントスキルや設定ファイルをプロジェクトに配置します。

このガイドではCodexを例に進めます。

```sh
specbind install --agent codex --language ja --project-instructions
```

#### `--agent`

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

#### `--language ja`

SpecBindが管理する成果物、具体的には `requirements.md` や
`design.md` の言語を日本語にします。

#### `--project-instructions`

`AGENTS.md`または
`CLAUDE.md`に、マーカーで囲んだSpecBindの案内ブロックを追加します。
もともと書いてある既存の文章はそのまま残ります。
普通はつけたほうがいいでしょう。

書き込まれるファイルを事前に確認したい場合は、同じコマンドへ`--dry-run`を
追加すると、変更を適用せずに`create`、`replace`、`keep`の計画を確認できます。

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

生成された内容をレビューし、いつもの手順でコミットしてください。SpecBindの
インストーラ自体はコミットを行いません。

初回のinstallが成功すると、最後に`specbind-configure`でプロジェクトに合わせた
設定レビューを行うよう案内が表示されます。coding agentのセッションを開き直した
あと、たとえば「このプロジェクト向けにSpecBindの設定を見直して、必要な変更まで
進めて」と依頼してください。スキルは次の読み取り専用コマンドから現在値を確認し、
テンプレート、ルール、adapter、Steering、Agent設定のうち関係する面だけを扱います。

```sh
specbind configuration show
```

既定値のまま使う判断も有効です。コマンドやスキルは、プロジェクト全体を単純な
「設定済み／未設定」には分類しません。

導入したら、対象プロジェクトでcoding agentのセッションを開き直してください。
そうしないと、エージェントが新しいスキルを認識できないことがあります。
以下ではCodexの`$`とClaude Codeの`/`を使って呼び出し例を示します。`generic`を
選んだ場合、`specbind-*`というスキル名は同じですが、呼び出し方はagentごとに
異なります。利用するagentのスキル選択または自動Discoveryの方法に読み替えてください。

## 4. 最初の変更を選ぶ

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

## 5. Discoveryでスコープを確認する

変更内容を添えて、discoveryスキルをエージェントに依頼します。

Codex:

```text
$specbind-discovery 一覧画面に表示している内容を、CSVファイルとして
ダウンロードできるようにしたい。
```

Claude Code:

```text
/specbind-discovery 一覧画面に表示している内容を、CSVファイルとして
ダウンロードできるようにしたい。
```

Discoveryは、プロジェクトの現在のSpec、Steering、Milestoneを読んだうえで、
変更を次のどれかに分類します。

- Direct
- 既存Specの更新
- 新規Spec

新規Specになった場合は、責務を表すkebab-caseのSpec ID、Milestoneのスコープ、
依存関係を提示します。ここでの結論が以降のワークフロー全体の前提になるので、
分類と境界を必ず読んでから承認してください。承認すると、CLIがMilestoneを作成し、
エージェントが`brief.md`を書きます。

途中で状態を確認したくなったら、次のように依頼できます。

```text
$specbind-status
```

Claude Codeでは`/specbind-status`です。このスキルは読み取り専用で、承認したり
成果物を書き換えたりはしません。

## 6. Tasks承認まで進める

Discoveryが報告したSpec IDを使い、標準の計画ワークフローで最初の1件を進めます。
ここではSpec IDが`csv-export`だったとします。

Codex:

```text
$specbind-plan csv-export
```

Claude Code:

```text
/specbind-plan csv-export
```

Planは、Requirements、Design、Design検証、Contract review、Tasksを順に実行し、
Tasksの承認まで進んだところで止まります。実行の最初に、Requirements、Design、
Tasksの各Gateをこの実行の中でまとめて承認してよいか聞かれます。

まとめて承認しても、レビューやCLIの検査は省略されません。各Gateで個別に行う
確認を、1回の実行に対する確認へまとめるだけです。1つずつ内容を見ながら進めたい
場合は、まとめての承認を断れば、各フェーズで個別に承認できます。

Planが終わった時点では、実装はまだ始まっていません。Requirements、Design、Tasksの
どれか1フェーズだけを明示的に進めたい場合は、対応する`specbind-plan-*`スキルを使います。

## 7. 実装して検証する

承認済みのTasksを実装します。

Codex:

```text
$specbind-implement csv-export
```

Claude Code:

```text
/specbind-implement csv-export
```

Implementが扱うのは1つのRoadmap itemだけで、着手できるTaskから順に実装します。
Spec-backed itemでは、Taskごとに実装、レビュー、CLIへの進捗記録を行います。
計画やDesignの問題が見つかった場合は、無理に実装を続けず、該当フェーズへ戻す
ためにいったん停止します。

全Taskが終わったら、Spec全体がRequirementsとDesignを満たしているか検証します。

Codex:

```text
$specbind-validate-implementation csv-export
```

Claude Code:

```text
/specbind-validate-implementation csv-export
```

検証結果が`GO`になり、CLIがcompletion evidenceを受理すれば、そのSpecの実装は
完了です。最後に状態を確認します。

```text
$specbind-status csv-export
```

この時点では、Milestoneはまだリリースされていません。リリースするには、
プロジェクト固有の`.specbind/settings/adapters/release.md`を用意し、対象
リリースを実際に公開・検証できる状態にしておく必要があります。最初の試用では、
実装の検証までを完了地点にすることをおすすめします。

## 8. 生成された成果物を見る

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
Tasksの計画部分は、それぞれを所有するスキル経由で保守します。

現在の状態は、CLIから直接確認することもできます。

```sh
specbind milestone status
specbind spec status csv-export
specbind tasks list csv-export
specbind artifact list csv-export
```

周辺ツールやスクリプトからMilestoneとSpecの状態を読む場合に限り、2つの
statusコマンドはコマンド固有のJSON出力も提供します。通常の利用では既定の
簡潔なテキスト出力をそのまま使います。

```sh
specbind milestone status --json
specbind spec status csv-export --json
```

## 次に読む

- [基本概念](./concepts.md)
- [現在のスキル一覧](../../current-skill-index.md)
- [現在の成果物一覧](../../current-artifact-index.md)

---

[ガイドの入口](../index.md) | [基本概念](./concepts.md)
