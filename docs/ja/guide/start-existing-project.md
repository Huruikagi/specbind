# 既存プロジェクトで始める

このページでは、既存のプロジェクトにSpecBindを導入し、コーディングエージェントと
一緒に、最初の変更を計画・実装・検証まで進めてみます。

まだ実装を始めていない場合は、[新規プロジェクトで始める](./start-new-project.md)へ
進んでください。必要な環境とエージェントについては、
[Getting Started](./getting-started.md)の「どちらのルートでも必要なもの」を
確認しておいてください。

!!! info "用語について"
    出てくる用語（Spec、Steering、Milestone、Gate など）は[基本概念](./concepts.md)
    にまとめています。先に目を通しても、出てきたときに参照してもかまいません。

## 1. CLIをインストールする

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

## 2. SpecBindを導入する

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

生成された内容をレビューし、いつもの手順でコミットしてください。SpecBindの
インストーラ自体はコミットを行いません。

### セッションを開き直す

導入したら、対象プロジェクトでcoding agentのセッションを開き直してください。
そうしないと、エージェントが新しいスキルを認識できないことがあります。

以降のスキル呼び出しはCodexの表記で示します。Claude Codeでは、先頭の`$`を`/`に
読み替えてください。スキル名と引数は同じです。`generic`を選んだ場合、
`specbind-*`というスキル名は同じですが、呼び出し方はagentごとに異なります。
利用するagentのスキル選択または自動Discoveryの方法に読み替えてください。

## 3. 設定は既定のまま進める

初回のinstallが成功すると、最後に`specbind-configure`でプロジェクトに合わせた
設定レビューを行うよう案内が表示されます。ただし最初は、この案内に従って設定を
見直す前に、既定値のまま最初の変更を一周することをおすすめします。SpecBindの
既定のテンプレートや判断基準は、そのまま使えるように設計されています。一周した
うえで、成果物の様式や粒度が自分のプロジェクトに合わないと感じた面だけ、あとから
`specbind-configure`に見直しを依頼すれば十分です。具体的なカスタマイズ方法は
[プロジェクトに合わせてカスタマイズする](./customization.md)にまとめています。

## 4. 導入方法を選ぶ

既存プロジェクトには、目的の異なる2つの始め方があります。

| 目的 | 次に使うワークフロー |
| --- | --- |
| これから行う変更をSpecBindで進める | このページの続きを進める |
| すでに動いている実装からSpecを確立する | [既存実装からSpecを確立する](./adopt-existing.md) |

`specbind-adopt-existing`は、既存コードをそのまま正しい仕様とみなす機能では
ありません。コードとテストを証拠として調査し、維持したい意図を確認してから
Specへ引き継ぐ専用ワークフローです。今後の変更からSpecBindを使い始めるだけなら、
採用ワークフローは不要です。

このワークフローは、プロダクト・技術・構造の全体方針を説明するSteeringが
用意されていることを前提にします。まだなければ、先に`specbind-steering`を
bootstrapモードで実行します。詳しい前提条件と流れは
[既存実装からSpecを確立する](./adopt-existing.md)にまとめています。

以下では、これから行う最初の変更を通常のライフサイクルで進めます。

## 5. 最初の変更を選ぶ

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

## 6. Discoveryでスコープを確認する

変更内容を添えて、discoveryスキルをエージェントに依頼します。関連するissueやメモが
あれば、その場所も伝えます。

```text
$specbind-discovery 一覧画面に表示している内容を、CSVファイルとして
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

スキルは提案を次の4項目で示します。

- **Work items** — 今回行う作業の一覧
- **New Specs** — 新しく作る責務の境界
- **Gate invalidations** — やり直しになる既存の承認
- **Dependencies** — 作業どうしの依存関係

ここでの結論が以降のワークフロー全体の前提になります。分類と境界を必ず読んでから
承認してください。承認すると、CLIがMilestoneとSpecの状態を作り、エージェントが、
その変更の要点をまとめた作業メモ（`brief.md`）を書きます。

途中で状態を確認したくなったら、次のように依頼できます。

```text
$specbind-status
```

このスキルは読み取り専用で、承認したり成果物を書き換えたりはしません。

## 7. 計画と実装の進め方を選ぶ

最初の`csv-export`を段階ごとに確認する場合は、次の所有スキルを順に使います。

```text
$specbind-plan-requirements csv-export
$specbind-plan-design csv-export
$specbind-contract-review
$specbind-plan-tasks csv-export
$specbind-implement csv-export
$specbind-validate-implementation csv-export
```

各段階で確認する内容、承認、上流へ戻る場合の扱いは、
[1件ずつ計画・実装する](./implement-step-by-step.md)にまとめています。

複数のSpecやDirect itemを含むMilestoneでは、`$specbind-plan --all`のあとに
`$specbind-drive`を使います。保留、別項目への継続、停止条件は
[PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)を参照してください。
どちらの経路もRelease前で停止します。

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
- [1件ずつ計画・実装する](./implement-step-by-step.md)
- [PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)
- [リリースする](./release.md) — Milestoneを実際に締めるとき
- [プロジェクトに合わせてカスタマイズする](./customization.md) — 一周して調整したい点が見えてから
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）

---

[Getting Started](./getting-started.md) | [新規プロジェクトで始める](./start-new-project.md)
