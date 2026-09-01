# SpecBindユーザーガイド

SpecBindは、ソフトウェア開発をAIコーディングエージェントと一緒に進めるためのツールです。
仕様や設計をその場限りのプロンプトで終わらせず、継続的にメンテナンスしていくことを
目標にしています。

SpecBindの大きな特徴は、エージェントのスキルと`specbind` CLIを組み合わせて
開発を進めることです。スコープの判断、仕様の作成、レビュー、実装はスキルが担当し、
成果物の整合性、承認、進捗、状態の遷移はCLIが管理します。

また、[gotlab/cc-sdd](https://github.com/gotalab/cc-sdd/tree/main)を起点に
設計しています。cc-sddはKiroから継承されたものであり、Kiro、cc-sdd、そして
そのコントリビューターの皆さんに、この土台への感謝を表します。多くの概念はcc-sdd v3から
継承されており、いくつかの変更を加えて再構成しています。

## 対応環境

SpecBind v1の配布バイナリは、Windows x64、Linux x64、macOS ARM64を対象としています。
Linux x64はWSL2上、macOS ARM64はApple SiliconのCI環境で検証しています。

## ガイド

1. [はじめに](./guide/getting-started.md) — 新規・既存プロジェクトのルートを選ぶ
   - [新規プロジェクトで始める](./guide/start-new-project.md)
   - [既存プロジェクトで始める](./guide/start-existing-project.md)
2. 実装を進める — 確認の粒度とMilestoneの規模に合わせて選ぶ
   - [1件ずつ計画・実装する](./guide/implement-step-by-step.md)
   - [PlanとDriveでMilestoneを進める](./guide/implement-with-plan-and-drive.md)
   - [リリースする](./guide/release.md)
3. [SpecBindをアップデートする](./guide/update.md) — バイナリとプロジェクト内の製品管理対象を更新する
4. [SpecBindの考え方](./guide/concepts.md) — 何を目指して、どう実現するのか
5. [カスタマイズ](./guide/customization.md) — やりたいことに合わせて
6. [バグ報告と改善提案](./guide/feedback.md) — 教えてください
7. [エージェントの削除とアンインストール](./guide/uninstall.md) — やめたくなったら

## リファレンス

- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/) — インストールされる全スキル（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/) — 作成・管理されるファイル（英語）

## cc-sddを利用中の場合

- [cc-sddから移行する](./guide/migrate-from-cc-sdd.md) — 自動移行とエージェント支援移行

---

[はじめに進む](./guide/getting-started.md) | [GitHubリポジトリ](https://github.com/Huruikagi/specbind)
