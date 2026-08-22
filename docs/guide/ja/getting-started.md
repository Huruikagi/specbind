# Getting Started

このページでは、既存のGitプロジェクトにSpecBindを導入し、Codexまたは
Claude Codeを使って、最初の変更をスコープの確認から実装の検証まで進めます。

SpecBindはv1.0前のプレビュー版として配布しています。インストーラは公開済みの
最新stableリリースを選び、対応するバイナリをGitHub Releaseから取得します。

## 1. 前提を確認する

次のものを用意してください。

- Gitで管理していて、コミットが1つ以上ある対象プロジェクト
- CodexまたはClaude Code
- Windows x64、またはWSL2上のLinux x64

ソースからビルドする場合は、これに加えて[Rustup](https://rustup.rs/)で入れた
Rustツールチェーンが必要です。Windowsでソースビルドするには、Visual Studio
Build Toolsの**Desktop development with C++**ワークロードとWindows SDKも
入れてください。

対象プロジェクトに未コミットの変更が残っている場合は、導入前に内容を確認し、
いつもの手順でコミットしておいてください。SpecBindは、既存ファイルを置き換える
操作の安全境界として、Gitの履歴とクリーンな作業ツリーを利用します。

## 2. プレビュー版CLIをインストールする

miseを使っている場合は、WindowsとWSL2/Linuxのどちらでも次のコマンドで
GitHub Releaseの対応するバイナリを導入できます。

```sh
mise use github:Huruikagi/specbind
```

このコマンドは、現在のディレクトリでmiseが選んだ設定ファイルへSpecBindを記録し、
miseの管理する`PATH`から実行できるようにします。miseは`latest`に既定で最低公開
期間を設けているため、stableリリースの公開直後は、必要に応じて
`mise use github:Huruikagi/specbind@0.2.0`のようにバージョンを明示してください。

miseを使わない場合は、プラットフォーム別のインストーラを使います。

Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/Huruikagi/specbind/main/install.ps1 | iex
```

WSL2/Linux:

```sh
curl -fsSL https://raw.githubusercontent.com/Huruikagi/specbind/main/install.sh | sh
```

プラットフォーム別インストーラはGitHub Releaseのアーカイブと`SHA256SUMS`を
取得し、チェックサムが一致した場合だけバイナリを配置します。既定の配置先は
次のとおりです。

- Windows: `%LOCALAPPDATA%\SpecBind\bin\specbind.exe`
- WSL2/Linux: `$HOME/.local/bin/specbind`

インストーラは`PATH`を恒久的には変更しません。配置先が現在の`PATH`に入って
いない場合は、いま使っているシェルに追加するためのコマンドを表示します。別の
場所へ入れたい場合は、PowerShellでは`-InstallDir`、Linuxでは`--install-dir`を
指定してください。

インストールできたか確認します。

```sh
specbind --version
```

### ソースからビルドする

SpecBindリポジトリを取得し、Rustワークスペースでreleaseバイナリをビルドします。

```sh
git clone https://github.com/Huruikagi/specbind.git
cd specbind/tools/specbind
cargo build --release
```

ビルドしたバイナリは次の場所にできます。

- Windows: `target/release/specbind.exe`
- WSL2/Linux: `target/release/specbind`

このディレクトリを、いま使っているシェルの`PATH`に追加して確認します。

PowerShell:

```powershell
$env:Path = "$(Resolve-Path .\target\release);$env:Path"
specbind --version
```

WSL2/Linux:

```sh
export PATH="$(pwd)/target/release:$PATH"
specbind --version
```

この設定が効くのは、いま使っているシェルの中だけです。CodexやClaude Codeを別の
シェルから起動する場合は、そちらのプロセスからも`specbind`を実行できるように
してください。

## 3. 対象プロジェクトへ導入する

対象プロジェクトのルートへ移動します。まずはdry runで、どんなファイルが作られる
のかを確認します。

Codexだけを使う場合:

```sh
specbind install --dry-run --agent codex --language ja --project-instructions
```

Claude Codeだけを使う場合は、`codex`を`claude-code`に置き換えてください。
両方を使う場合は`--agent`を並べます。

```sh
specbind install --dry-run --agent codex --agent claude-code --language ja --project-instructions
```

`--language ja`を付けると、SpecBindが管理する成果物の言語をプロジェクト全体で
日本語にします。`--project-instructions`を付けると、ルートの`AGENTS.md`または
`CLAUDE.md`に、マーカーで囲んだSpecBindの案内ブロックを追加します。まわりに
書いてある既存の文章はそのまま残ります。

内容に問題がなければ、`--dry-run`を外して実際に適用します。

```sh
specbind install --agent codex --language ja --project-instructions
```

主に、次のファイルが追加されます。

```text
.specbind.json
.specbind/settings/
.agents/skills/specbind-*/       # Codex
.codex/agents/specbind-*.toml    # Codexの役割別モデル設定
.claude/skills/specbind-*/       # Claude Code
.claude/agents/specbind-*.md     # Claude Codeの役割別モデル設定
AGENTS.md / CLAUDE.md            # 指示の統合を有効にした場合
```

CodexとClaude Codeのどちらでも、実装・レビューなどの役割ごとにSpecBindの既定
モデルが設定されます。通常は変更不要です。プロジェクトでコストと能力の配分を
変える場合だけ、`.specbind.json`の`agentRoles`を上書きしてから、クリーンな
リポジトリで`specbind install`を再実行します。

```json
{
  "agentRoles": {
    "codex": {
      "implementer": {
        "model": "gpt-5.6-luna",
        "reasoningEffort": "medium"
      }
    },
    "claudeCode": {
      "researcher": {
        "model": "sonnet"
      }
    }
  }
}
```

指定できる役割は`planner`、`implementer`、`reviewer`、`debugger`、
`researcher`です。省略した役割と項目にはSpecBindの既定値が使われます。
Claude Codeのサブエージェント定義には推論強度の項目がないため、
`agentRoles.claudeCode`で指定できるのは`model`だけです。

生成された内容をレビューし、いつもの手順でコミットしてください。SpecBindの
インストーラ自体はコミットを行いません。

導入したら、対象プロジェクトでCodexまたはClaude Codeのセッションを開き直して
ください。そうしないと、エージェントが新しいスキルを認識できません。

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

Discoveryが報告したSpec IDを使い、最初の1件はquickワークフローで進めます。
ここではSpec IDが`csv-export`だったとします。

Codex:

```text
$specbind-quick-plan csv-export
```

Claude Code:

```text
/specbind-quick-plan csv-export
```

Quickは、Requirements、Design、Design検証、Contract review、Tasksを順に実行し、
Tasksの承認まで進んだところで止まります。実行の最初に、Requirements、Design、
Tasksの各Gateをこの実行の中でまとめて承認してよいか聞かれます。

まとめて承認しても、レビューやCLIの検査は省略されません。各Gateで個別に行う
確認を、1回の実行に対する確認へまとめるだけです。1つずつ内容を見ながら進めたい
場合は、まとめての承認を断れば、各フェーズで個別に承認できます。

Quickが終わった時点では、実装はまだ始まっていません。

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
   ├─ contract.md
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

## 次に読む

- [基本概念](./concepts.md)
- [現在のスキル一覧](../../current-skill-index.md)
- [現在の成果物一覧](../../current-artifact-index.md)

---

[ガイドの入口](./index.md) | [基本概念](./concepts.md)
