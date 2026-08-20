# SpecBindユーザーガイド Preview

SpecBindは、AIコーディングエージェントと一緒に進めるソフトウェア変更で、
Requirements、Design、Contract、Tasksを一度きりのプロンプトではなく、継続して
保守する仕様として残すためのツールです。

エージェントのスキルがスコープの判断、仕様の作成、レビュー、実装を担当し、
`specbind` CLIが成果物の整合性、承認、進捗、ライフサイクル遷移を管理します。

## このガイドの対象

このPreviewガイドは、次の利用者を対象にしています。

- Gitで管理された既存プロジェクトがある
- CodexまたはClaude Codeを利用している
- 1つの振る舞いの変更を、スコープ確認から実装検証まで進めたい
- SpecBindをソースからビルドして試せる

最初に[Getting Started](./getting-started.md)を進めてください。作業中に登場する
用語や責任分担は[基本概念](./concepts.md)で確認できます。

## 現在の公開状態

SpecBindはまだ正式リリースされていません。このガイドは、現在の実装を試すための
Previewです。

- GitHub Releaseバイナリとダウンロード用インストーラはまだ提供していません。
- 現在はWindows x64またはWSL2上のLinux x64で、Rustからビルドして試します。
- `specbind migrate cc-sdd`はまだ実装されていません。
- エージェントの削除とプロジェクトからのアンインストールはv1の対象外です。
- CLI、スキル、ガイドの細部は正式リリースまで変更される可能性があります。

既存の`.kiro`プロジェクトを通常の`specbind install`で自動変換することは
ありません。移行機能が提供されるまでは、同じプロジェクト上での試用を避けて
ください。

## 最初に読むページ

1. [Getting Started](./getting-started.md) — 導入から最初の変更の実装検証まで
2. [基本概念](./concepts.md) — Spec、Milestone、Direct、Gate、成果物
3. [現在のスキル一覧](../../current-skill-index.md) — インストールされる全スキル
4. [現在の成果物一覧](../../current-artifact-index.md) — 作成・管理されるファイル

## SpecBindを使う変更

すべての編集にSpecBindが必要なわけではありません。次のいずれかに当てはまる
変更は、SpecBindのワークフローに入ります。

- 既存SpecのRequirements、Design、Contractを変更する
- 既存Specが所有する振る舞いやファイル境界を変更する
- プロジェクトが今後も所有する新しい責務を追加する
- 明示的にMilestoneやリリースへ含める

それ以外の小さな保守作業は、通常の開発作業として進められます。分類に迷う場合は
`specbind-discovery`スキルに依頼してください。

---

[Getting Startedへ進む](./getting-started.md) | [リポジトリREADME](../../../README.md)
