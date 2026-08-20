# Getting Started

このガイドでは、既存のGitプロジェクトにSpecBindを導入し、Codexまたは
Claude Codeを使って、最初の変更をスコープ確認から実装検証まで進めます。

SpecBindはpre-1.0 Previewとして配布します。公開済みのrelease candidateを試す
場合はバージョンを明示してインストーラを実行します。対応するGitHub Releaseが
まだ公開されていない場合は、後述のソースビルドを使用します。

## 1. 前提を確認する

必要なものは次のとおりです。

- Gitで管理された、少なくとも1つのコミットがある対象プロジェクト
- CodexまたはClaude Code
- Windows x64、またはWSL2上のLinux x64

ソースからビルドする場合だけ、[Rustup](https://rustup.rs/)で導入したRust
ツールチェーンが必要です。Windowsでのソースビルドには、Visual Studio Build
Toolsの**Desktop development with C++**ワークロードとWindows SDKも必要です。

対象プロジェクトに未コミットの変更がある場合は、導入前に内容を確認し、通常の
プロジェクト手順でコミットしてください。SpecBindはGitの履歴とクリーンな作業
ツリーを、既存ファイルを置き換える操作の安全境界として利用します。

## 2. Preview CLIをインストールする

release candidateは最新版として暗黙選択されないため、バージョンを明示します。

Windows PowerShell:

```powershell
Invoke-WebRequest https://raw.githubusercontent.com/Huruikagi/specbind/main/install.ps1 `
  -OutFile install.ps1
.\install.ps1 -Version 0.1.0-rc.1
```

WSL2/Linux:

```sh
curl -fsSLO https://raw.githubusercontent.com/Huruikagi/specbind/main/install.sh
sh install.sh --version 0.1.0-rc.1
```

インストーラはGitHub Releaseのarchiveと`SHA256SUMS`を取得し、checksumが一致
した場合だけバイナリを配置します。既定の配置先は次のとおりです。

- Windows: `%LOCALAPPDATA%\SpecBind\bin\specbind.exe`
- WSL2/Linux: `$HOME/.local/bin/specbind`

永続的な`PATH`は変更しません。配置先が現在の`PATH`に含まれない場合は、現在の
シェルに追加するためのコマンドを表示します。別の配置先を使う場合は、PowerShell
では`-InstallDir`、Linuxでは`--install-dir`を指定します。

インストール後に確認します。

```sh
specbind --version
```

### GitHub Release公開前にソースからビルドする

SpecBindリポジトリを取得し、Rustワークスペースでreleaseバイナリをビルドします。

```sh
git clone https://github.com/Huruikagi/specbind.git
cd specbind/tools/specbind
cargo build --release
```

生成されるバイナリは次の場所にあります。

- Windows: `target/release/specbind.exe`
- WSL2/Linux: `target/release/specbind`

ソースビルドしたバイナリのディレクトリを現在のシェルの`PATH`へ追加し、確認します。

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

この設定は現在のシェルだけに適用されます。CodexまたはClaude Codeを別のシェル
から起動する場合も、そのプロセスから`specbind`を実行できるようにしてください。

## 3. 対象プロジェクトへ導入する

対象プロジェクトのルートへ移動します。最初にdry runで、作成されるファイルを
確認します。

Codexだけを使う場合:

```sh
specbind install --dry-run --agent codex --language ja --project-instructions
```

Claude Codeだけを使う場合は、`codex`を`claude-code`へ置き換えます。両方を
使う場合は`--agent`を繰り返します。

```sh
specbind install --dry-run --agent codex --agent claude-code --language ja --project-instructions
```

`--language ja`は、SpecBindが管理する成果物のプロジェクト共通言語を日本語に
設定します。`--project-instructions`は、ルートの`AGENTS.md`または`CLAUDE.md`に
マーカー付きのSpecBind案内ブロックを追加します。既存の周囲の文章は保持されます。

計画に問題がなければ、`--dry-run`を外して適用します。

```sh
specbind install --agent codex --language ja --project-instructions
```

主に次のファイルが追加されます。

```text
.specbind.json
.specbind/settings/
.agents/skills/specbind-*/       # Codex
.claude/skills/specbind-*/       # Claude Code
AGENTS.md / CLAUDE.md            # 指示統合を有効にした場合
```

生成内容をレビューし、対象プロジェクトの通常の手順でコミットしてください。
SpecBindのインストーラ自体はコミットしません。

エージェントが新しく導入したスキルを認識できるよう、導入後に対象プロジェクトで
新しいCodexまたはClaude Codeのセッションを開始します。

## 4. 最初の変更を選ぶ

最初は、1つの明確な振る舞いを追加する、小さな変更を選びます。複数の機能や
リリース作業をまとめて試すのは避けてください。

このガイドでは、既存のCLIアプリケーションに次の変更を加える例を使います。

> 設定ファイルを検証し、問題のある項目と理由を表示する
> `validate-config`コマンドを追加したい。

この例は、プロジェクトが継続して所有する新しい振る舞いと境界を作るため、既存の
Specがなければ新規Specとして分類される想定です。実際には、自分のプロジェクトに
合う同程度の変更へ置き換えてください。

## 5. Discoveryでスコープを確認する

エージェントに、変更内容とともにdiscoveryスキルを依頼します。

Codex:

```text
$specbind-discovery 設定ファイルを検証し、問題のある項目と理由を表示する
validate-configコマンドを追加したい。
```

Claude Code:

```text
/specbind-discovery 設定ファイルを検証し、問題のある項目と理由を表示する
validate-configコマンドを追加したい。
```

Discoveryは、プロジェクトの現在のSpec、Steering、Milestoneを読み、変更を次の
いずれかへ分類します。

- Direct
- 既存Specの更新
- 新規Spec

新規Specの場合は、責務を表すkebab-caseのSpec ID、Milestoneのスコープ、依存関係を
提示します。ここは残りのワークフローの前提になるため、分類と境界を読んでから
明示的に承認してください。承認後、CLIがMilestoneを作成し、エージェントが
`brief.md`を作成します。

途中で状態を確認したい場合は、次のように依頼できます。

```text
$specbind-status
```

Claude Codeでは`/specbind-status`です。状態確認スキルは読み取り専用で、承認や
成果物の修正は行いません。

## 6. Tasks承認まで進める

Discoveryが報告したSpec IDを使い、最初の1件はquickワークフローで進めます。
以下ではSpec IDを`config-validation`と仮定します。

Codex:

```text
$specbind-quick config-validation
```

Claude Code:

```text
/specbind-quick config-validation
```

Quickは、Requirements、Design、Design検証、Contract review、Tasksを順に実行し、
Tasks承認後に停止します。開始時に、Requirements、Design、Tasksの各Gateを同じ
実行内で委任承認してよいか確認されます。

委任を承認しても、レビューやCLIの検査は省略されません。各Gateでの追加確認を
1回の実行単位の確認へまとめるだけです。個別に内容を確認しながら進めたい場合は、
委任を断り、各フェーズで明示的に承認できます。

Quickが完了した時点では、実装はまだ始まっていません。

## 7. 実装して検証する

承認済みTasksを実装します。

Codex:

```text
$specbind-implement config-validation
```

Claude Code:

```text
/specbind-implement config-validation
```

Implementは1つのRoadmap itemだけを対象にし、実行可能なTaskを順に実装します。
Spec-backed itemでは、Taskごとに実装、レビュー、CLIへの進捗記録を行います。
計画やDesignに問題が見つかった場合は、無理に実装を続けず、該当フェーズへ戻す
ために停止します。

全Taskの完了後、Spec全体がRequirementsとDesignを満たしているか検証します。

Codex:

```text
$specbind-validate-implementation config-validation
```

Claude Code:

```text
/specbind-validate-implementation config-validation
```

検証が`GO`になり、CLIがcompletion evidenceを受理すると、そのSpecの実装は完了です。
最後に状態を確認します。

```text
$specbind-status config-validation
```

この時点ではMilestoneはまだリリースされていません。リリースにはプロジェクト固有の
`.specbind/settings/adapters/release.md`を準備し、対象リリースを公開・検証できる
状態にする必要があります。最初の試用では、実装検証までを完了地点にすることを
推奨します。

## 8. 生成された成果物を見る

既定では、Specの成果物は`.specbind/specs/<spec>/`に作られます。

```text
.specbind/
├─ steering/roadmap.md
└─ specs/config-validation/
   ├─ spec.yaml
   ├─ brief.md
   ├─ requirements.md
   ├─ design.md
   ├─ contract.md
   └─ tasks.yaml
```

`spec.yaml`、`roadmap.md`、`tasks.yaml`内の実行状態はCLIが所有します。状態を進める
目的で手編集しないでください。Requirements、Design、Contract、Tasksの計画部分は、
それぞれを所有するスキルを通して保守します。

CLIから現在の状態を直接確認することもできます。

```sh
specbind milestone status
specbind spec status config-validation
specbind tasks list config-validation
specbind artifact list config-validation
```

## 次に読む

- [基本概念](./concepts.md)
- [現在のスキル一覧](../../current-skill-index.md)
- [現在の成果物一覧](../../current-artifact-index.md)

---

[ガイドの入口](./index.md) | [基本概念](./concepts.md)
