# SpecBindユーザーガイド

SpecBindは、ソフトウェア開発をAIコーディングエージェントと一緒に進めるためのツールです。
仕様や設計をその場限りのプロンプトで終わらせず、継続的にメンテナンスしていくことを
目標にしています。

SpecBindの大きな特徴は、エージェントのスキルと`specbind` CLIを組み合わせて
開発を進めることです。スコープの判断、仕様の作成、レビュー、実装はスキルが担当し、
成果物の整合性、承認、進捗、状態の遷移はCLIが管理します。

また、[gotlab/cc-sdd](https://github.com/gotalab/cc-sdd/tree/main)を起点に
設計しています。（とってもありがとうございます。）
多くの概念はcc-sdd v3から継承されており、いくつかの変更を加えて再構成しています。

## 対応環境

SpecBind v1の配布バイナリは、Windows x64、Linux x64、macOS ARM64を対象としています。
Linux x64はWSL2上、macOS ARM64はApple SiliconのCI環境で検証しています。

## ガイド

1. [Getting Started](./guide/getting-started.md) — 新規・既存プロジェクトのルートを選ぶ
   - [新規プロジェクトで始める](./guide/start-new-project.md)
   - [既存プロジェクトで始める](./guide/start-existing-project.md)
   - [リリースする](./guide/release.md)
2. [SpecBindの考え方](./guide/concepts.md) — 何を目指して、どう実現するのか
3. [カスタマイズ](./guide/customization.md) — やりたいことに合わせて
4. [バグ報告と改善提案](./guide/feedback.md) — 教えてください
5. [Agentの削除とアンインストール](./guide/uninstall.md) — やめたくなったら

## リファレンス

- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/) — インストールされる全スキル（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/) — 作成・管理されるファイル（英語）

## cc-sddを利用中の場合

- [cc-sddから移行する](./guide/migrate-from-cc-sdd.md) — 自動移行とエージェント支援移行

---

[Getting Startedへ進む](./guide/getting-started.md) | [GitHubリポジトリ](https://github.com/Huruikagi/specbind)
