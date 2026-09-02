# SpecBindユーザーガイド

SpecBindは、ソフトウェア開発をAIコーディングエージェントと一緒に進めるためのツールです。
仕様や設計をその場限りのプロンプトで終わらせず、継続的にメンテナンスしていくことを
目標にしています。

SpecBindの大きな特徴は、エージェントのスキルと`specbind` CLIを組み合わせて
開発を進めることです。スコープの判断、仕様の作成、レビュー、実装はスキルが担当し、
成果物の整合性、承認、進捗、状態の遷移はCLIが管理します。

また、[gotlab/cc-sdd](https://github.com/gotalab/cc-sdd/tree/main)を起点に
設計しています。cc-sddはKiroから継承されたものであり、Kiro、cc-sdd、そして
そのコントリビューターの皆さんが築いてきた知見と取り組みに、深く感謝します。多くの概念はcc-sdd v3から
継承されており、いくつかの変更を加えて再構成しています。

## 対応環境

SpecBind v1の配布バイナリは、Windows x64、Linux x64、macOS ARM64を対象としています。
Linux x64はWSL2上、macOS ARM64はApple SiliconのCI環境で検証しています。

## はじめに

まずここから。ルートを選び、SpecBindを導入します。

- [ルートを選ぶ](./guide/getting-started.md) — 新規・既存プロジェクトのどちらか
- [SpecBindをインストールする](./guide/install.md) — 両ルート共通の導入手順
- [新規プロジェクトで始める](./guide/start-new-project.md)
- [既存プロジェクトで始める](./guide/start-existing-project.md)

## 開発を進める

導入後の日常的なワークフローです。

- [基本概念](./guide/concepts.md) — Spec、Milestone、Gateなど、以降の前提になる用語
- [1件ずつ計画・実装する](./guide/implement-step-by-step.md) — 成果物とGateを段階ごとに確認する
- [PlanとDriveでMilestoneを進める](./guide/implement-with-plan-and-drive.md) — 到達可能な作業をまとめて進める
- [既存実装からSpecを確立する](./guide/adopt-existing.md) — 現在のコードを基準Specにする
- [リリースする](./guide/release.md) — Milestoneを1つのリリースとして締める

## 設定と運用

- [カスタマイズ](./guide/customization.md) — テンプレート、ルール、アダプター、Steering、役割別モデル
- [SpecBindをアップデートする](./guide/update.md) — バイナリとプロジェクト内の製品管理対象を更新する
- [エージェントの削除とアンインストール](./guide/uninstall.md) — やめたくなったら
- [cc-sddから移行する](./guide/migrate-from-cc-sdd.md) — 自動移行とエージェント支援移行
- [バグ報告と改善提案](./guide/feedback.md) — 教えてください

## リファレンス

- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/) — インストールされる全スキル（英語）
- [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/) — 作成・管理されるファイル（英語）

---

[ルートを選ぶ](./guide/getting-started.md) | [GitHubリポジトリ](https://github.com/Huruikagi/specbind)
