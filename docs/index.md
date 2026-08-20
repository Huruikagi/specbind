# SpecBind

SpecBindは、AIコーディングエージェントによる開発で、意図からリリースまでの
仕様を継続して保守するためのツールです。

Requirements、Design、Contract、Tasksを一度きりのプロンプトから切り離し、
後続の変更でも参照・更新できるプロジェクトの記録として残します。

!!! warning "Preview"

    SpecBindはまだ正式リリースされていません。現在はソースからCLIをビルドして
    試すPreview段階です。CLI、スキル、ガイドの細部は正式リリースまで変更される
    可能性があります。

## SpecBindの役割

- **Skillが判断する** — エージェントがスコープ確認、仕様作成、レビュー、実装、
  検証を担当します。
- **CLIが不変条件を守る** — `specbind` CLIが成果物の構造、トレーサビリティ、
  承認、進捗、ライフサイクル遷移を検証・記録します。
- **Specを使い捨てない** — 同じ能力への後続変更は、同じSpecのRequirements、
  Design、Contractを現在形として更新します。
- **Milestoneでリリース境界を明示する** — 複数Specと小さなDirect changeを、
  依存関係を含む1つのdeliveryとして追跡します。

## 最初に進む

[Getting Startedを始める](guide/ja/getting-started.md){ .md-button .md-button--primary }
[基本概念を読む](guide/ja/concepts.md){ .md-button }

Getting Startedでは、既存のGitプロジェクトへSpecBindを導入し、Codexまたは
Claude Codeを使って、最初の変更をDiscoveryから実装検証まで進めます。

## 現在の対応範囲

- CodexとClaude Code
- 日本語または英語のプロジェクト成果物
- Windows x64とWSL2上のLinux x64
- ソースからビルドするPreview CLI

GitHub Releaseバイナリ、ダウンロード用インストーラ、cc-sdd migrationは準備中です。

## 詳細

- [ユーザーガイド](guide/ja/index.md)
- [現在のスキル一覧](current-skill-index.md)
- [現在の成果物一覧](current-artifact-index.md)
- [GitHubリポジトリ](https://github.com/Huruikagi/specbind)
