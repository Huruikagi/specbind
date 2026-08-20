# SpecBind

SpecBindは、AIコーディングエージェントを使った開発で、意図からリリースまでの
仕様を継続して保守するためのツールです。

Requirements、Design、Contract、Tasksをその場限りのプロンプトから切り離し、
あとの変更でも参照・更新できるプロジェクトの記録として残します。

!!! warning "Preview"

    SpecBindはまだ正式リリースしていません。現在はv1.0前のプレビュー版で、
    リリース候補版のバイナリまたはソースビルドで試せます。CLI、スキル、
    ガイドの細部は、正式リリースまでに変わる可能性があります。

## SpecBindの役割

- **Skillが判断する** — エージェントがスコープの確認、仕様の作成、レビュー、
  実装、検証を担当します。
- **CLIが不変条件を守る** — `specbind` CLIが成果物の構造、トレーサビリティ、
  承認、進捗、状態の遷移を検証して記録します。
- **Specを使い捨てない** — 同じ能力をあとから変更するときは、同じSpecの
  Requirements、Design、Contractを「現在の姿」として更新します。
- **Milestoneでリリース境界を明示する** — 複数のSpecと小さなDirect changeを、
  依存関係も含めて1回のリリース単位として追跡します。

## 最初に進む

[Getting Startedを始める](guide/ja/getting-started.md){ .md-button .md-button--primary }
[基本概念を読む](guide/ja/concepts.md){ .md-button }

Getting Startedでは、既存のGitプロジェクトにSpecBindを導入し、Codexまたは
Claude Codeを使って、最初の変更をDiscoveryから実装の検証まで進めます。

## 現在の対応範囲

- CodexとClaude Code
- プロジェクト成果物の言語は日本語または英語
- Windows x64とWSL2上のLinux x64
- リリース候補版のバイナリ、またはソースからビルドしたプレビュー版CLI

リリース候補版は自動では選ばれないため、バージョンを明示してインストーラを
実行してください。手順は[Getting Started](guide/ja/getting-started.md)にあります。

`specbind migrate cc-sdd`はまだ実装していません。移行時に守る安全境界と、
エージェントに任せる手順は
[マイグレーションガイド](guide/migration/cc-sdd.md)で先行公開しています。

## 詳細

- [ユーザーガイド](guide/ja/index.md)
- [cc-sddマイグレーションガイド](guide/migration/cc-sdd.md)
- [現在のスキル一覧](current-skill-index.md)
- [現在の成果物一覧](current-artifact-index.md)
- [GitHubリポジトリ](https://github.com/Huruikagi/specbind)
