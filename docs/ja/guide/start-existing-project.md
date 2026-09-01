# 既存プロジェクトで始める

このページでは、既存のプロジェクトにSpecBindを導入し、コーディングエージェントと
一緒に、最初の変更を計画・実装・検証まで進めてみます。

まだ実装を始めていない場合は、[新規プロジェクトで始める](./start-new-project.md)へ
進んでください。必要な環境とエージェントについては、
[はじめに](./getting-started.md)の「どちらのルートでも必要なもの」を
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
`keep`と、廃止された製品管理対象に対する`remove`の計画を確認できます。

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

導入したら、対象プロジェクトでコーディングエージェントのセッションを開き直してください。
そうしないと、エージェントが新しいスキルを認識できないことがあります。

以降のスキル呼び出しはCodexの表記で示します。Claude Codeでは、先頭の`$`を`/`に
読み替えてください。スキル名と引数は同じです。`generic`を選んだ場合、
`specbind-*`というスキル名は同じですが、呼び出し方はエージェントごとに異なります。
利用するエージェントのスキル選択または自動Discoveryの方法に読み替えてください。

## 3. 最初の進め方を選ぶ

既存プロジェクトには、目的の異なる2つの始め方があります。

| 目的 | 次に使うワークフロー |
| --- | --- |
| これから行う変更をSpecBindで進める | このページの続きを進める |
| すでに動いている実装からSpecを確立し、計画まで整える | [フルサポート経路](#full-support-route) |

最初の小さな変更を進めるなら、既定値のまま通常のライフサイクルを一周する方法が
分かりやすいでしょう。実際に合わないと分かった面だけ、あとから
[プロジェクトに合わせてカスタマイズする](./customization.md)で調整できます。
[最初の変更を選ぶ](#first-change)へ進んでください。

## 既存実装を対象にフルサポートで進める経路 {#full-support-route}

既に相当量のコードがある一方で、信頼できるSpecがまだないプロジェクトでは、
Steeringと共通設定を整えてから既存実装を採用し、実装に入る前に計画を完成させる
この経路を選べます。この節はTasksの承認までで止まり、実装やリリースは始めません。

1. **`sb-configure`でプロジェクトの形を整えます。** まず、初回レビューを依頼します。

   ```text
   $sb-configure 既存実装を採用するための初期設定を、このプロジェクトについて
   見直してください。必要なSteeringの作成から始めてください。
   ```

   `sb-configure`は最初に機械的に確認できる設定の要約を読みます。継続的に使う方針が
   必要なら、Steeringの初期作成または同期を`sb-steering`へ引き継ぎます。提案された
   Steeringを確認してコミットしてから採用を始めてください。Discoveryはそのリビジョンを
   調査の証拠として固定します。

2. **共通の面が合うまで、対象を絞って設定レビューを繰り返します。** Steeringができたら、
   もう一度`sb-configure`にSteeringとリポジトリの事実をRequirements・Designテンプレート、
   共有Ruleと照らし合わせるよう依頼します。たとえば次のようにします。

   ```text
   $sb-configure 確定したSteeringとリポジトリの事実を使い、このプロジェクト向けに
   Requirements・Designテンプレートと共有Ruleを見直してください。
   ```

   テンプレート、Rule、エージェント、運用アダプターなど、残る面はそれぞれ別の依頼で
   見直します。`sb-configure`は関係する変更ごとに要約を読み直し、必要なaftercareまで
   完了します。Designテンプレートを増やすのは、独立した責任を繰り返し扱う必要がある
   場合だけです。技術名だけでは理由になりません。設定変更によって既存のSpecや
   ライフサイクル成果物が暗黙に調整されることもありません。

3. **既存実装を対象にDiscoveryを始めます。** Steeringをコミットし、作業ツリーを
   クリーンにしてから、採用する範囲を明示して依頼します。たとえばリポジトリ全体なら
   次のようになります。

   ```text
   $sb-discovery このリポジトリ全体の既存実装からSpecを確立してください。現在の
   コードとテストを証拠として調査し、何かを作る前にSpec境界と維持する意図を
   私に確認してください。
   ```

   Discoveryは採用用の事前検査を行い、調査したリビジョンを固定します。Spec境界候補、
   通常のMilestoneスコープ、各Specについて観察した挙動と意図は、それぞれ確認します。
   既存コードは証拠であって、自動的に仕様になるものではありません。調査と確認の詳細は
   [既存実装からSpecを確立する](./adopt-existing.md)を参照してください。

4. **採用したすべてのSpecをPlanします。** DiscoveryがSpec、Brief、Researchへの
   引き継ぎを作成したら、次を実行します。

   ```text
   $sb-plan --all
   ```

   Planは、選択したすべてのSpecについてRequirements、Design、TasksのGateを一度に
   承認してよいかを確認します。委譲しない場合は、各Gateで明示的に確認を求めます。
   Tasksの前にはDesignの検証とMilestone全体のContract Reviewを実行します。Tasksが
   承認されたら計画をレビューし、実装を始めるかは別の依頼として判断してください。
   この経路はここで終了します。

## 4. 最初の変更を選ぶ {#first-change}

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
$sb-status
```

このスキルは読み取り専用で、承認したり成果物を書き換えたりはしません。

## 6. 計画と実装の進め方を選ぶ

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

## 7. 生成された成果物を見る

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
- [リリースする](./release.md) — Milestoneを実際に締めるとき
- [プロジェクトに合わせてカスタマイズする](./customization.md) — 一周して調整したい点が見えてから
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）

---

[はじめに](./getting-started.md) | [新規プロジェクトで始める](./start-new-project.md)
