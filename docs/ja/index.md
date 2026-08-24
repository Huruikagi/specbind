# SpecBindユーザーガイド(プレビュー版)

SpecBindは、AIコーディングエージェントと一緒に進める開発のためのツールです。
Requirements、Design、Contract、Tasksをその場限りのプロンプトで終わらせず、
あとから読み返して更新できる仕様として残します。

役割ははっきり分かれています。スコープの判断、仕様の作成、レビュー、実装は
エージェントのスキルが担当します。成果物の整合性、承認、進捗、状態の遷移は
`specbind` CLIが管理します。

## このガイドの対象

次のような場合に読んでください。

- Gitで管理している既存プロジェクトがある
- CodexまたはClaude Codeを使っている
- 1つの振る舞いの変更を、スコープの確認から実装の検証まで通しで試したい
- プレビュー版のバイナリ、またはソースビルドを試せる

まず[Getting Started](./guide/getting-started.md)を進めてください。途中で出てくる
用語や役割分担は[基本概念](./guide/concepts.md)にまとめています。

## 現在の公開状態

SpecBindはv1.0前のプレビュー版です。このガイドは、リリース候補版または最新の
ソースを使って、実際に動かしながら試すためのものです。

- 配布しているバイナリの対象は、Windows x64とWSL2上のLinux x64です。
- リリース候補版は自動では選ばれません。バージョンを明示してインストールして
  ください。対応するリリースがまだない場合は、ソースからビルドします。
- `specbind migrate cc-sdd`は、読み取り専用計画、agent-assisted resolutionの受理、
  guarded `--apply`まで利用できます。
- エージェントの削除と、durable knowledgeの保持または削除を明示する
  プロジェクトアンインストールを利用できます。
- CLI、スキル、ガイドの細部は、正式リリースまでに変わる可能性があります。

既存の`.kiro`プロジェクトは、通常の`specbind install`では変換されません。
移行機能を提供するまでは、同じプロジェクト上で試さないでください。移行時に
守る安全境界と、エージェントに任せる手順は
[cc-sddマイグレーションガイド](./guide/migrate-from-cc-sdd.md)で先行公開しています。

## 最初に読むページ

1. [Getting Started](./guide/getting-started.md) — 導入から最初の変更を検証するまで
2. [基本概念](./guide/concepts.md) — Spec、Milestone、Direct、Gate、成果物
3. [カスタマイズ](./guide/customization.md) — テンプレート、ルール、adapter、Steering、モデル設定
4. [Agentの削除とアンインストール](./guide/uninstall.md) — exact plan、knowledge保持・削除、復元
5. [現在のスキル一覧](../current-skill-index.md) — インストールされる全スキル
6. [現在の成果物一覧](../current-artifact-index.md) — 作成・管理されるファイル
7. [cc-sddから移行する](./guide/migrate-from-cc-sdd.md) — 自動移行とエージェント支援移行

## SpecBindを使う変更

すべての編集にSpecBindが必要なわけではありません。次のどれかに当てはまる変更を、
SpecBindのワークフローで進めます。

- 既存SpecのRequirements、Design、Contractを変更する
- 既存Specが所有する振る舞いやファイル境界を変更する
- プロジェクトが今後も持ち続ける責務を新しく追加する
- Milestoneやリリースに含めると決めている

それ以外の小さな保守作業は、これまでどおりの進め方で構いません。どちらか迷う
場合は、`specbind-discovery`スキルに判断を任せてください。

---

[Getting Startedへ進む](./guide/getting-started.md) | [GitHubリポジトリ](https://github.com/Huruikagi/specbind)
