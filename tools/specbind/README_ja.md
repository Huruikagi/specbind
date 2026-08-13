# SpecBind: AIコーディングエージェント向け長時間の仕様駆動実装

[![license: MIT](https://img.shields.io/badge/license-MIT-green.svg)](../../LICENSE)

<div align="center" style="margin-bottom: 1rem; font-size: 1.1rem;"><sub>
<a href="./README.md">English</a> | 日本語
</sub></div>

**承認済みの仕様を、長時間でも壊れない自律実装ワークフローに変える。** ワンコマンドで agentic SDLC ワークフローを Agent Skills として導入する: discovery, requirements, design, tasks, そしてタスクごとの independent review 付きの自律実装。Claude Code と Codex に同じ 17-skill セットを提供する。

👻 **Kiro スタイル。** Kiro IDE の spec-driven / agentic SDLC スタイル。既存の Kiro 仕様書もそのまま使える。

## v3.0 の新機能

specbind v3.0 は Agent Skills と長時間自律実装を軸にした再構築である。

- **`/kiro-discovery` が新しいエントリポイント。** discovery が新規依頼を「既存 spec を拡張 / spec 不要で直接実装 / 1 つの新規 spec / 複数 spec に分解 / mixed decomposition」に振り分ける。`brief.md` と必要に応じて `roadmap.md` を書き出すので、セッションを再開しても scope を説明し直さずに続けられる。
- **`/kiro-impl` による長時間自律実装。** 各タスクに対し fresh implementer が feature flag 越しに TDD (RED → GREEN) で実装、独立した reviewer が機械的検証、失敗時は auto-debug pass が新しいコンテキストで根本原因を調査する。タスク間の知見は `tasks.md` の `## Implementation Notes` で次の implementer に引き継がれる。1 iteration = 1 task、中断後の再実行も安全。
- **境界中心の spec discipline。** `design.md` に File Structure Plan が入り、タスク境界の根拠になる。タスクには `_Boundary:_` / `_Depends:_` アノテーションが付く。review と validation はスタイルではなく境界違反を見る。
- **`/kiro-spec-batch` で複数 spec の並列作成。** roadmap から複数 spec を並列生成し、cross-spec review で矛盾・責務重複・インターフェースミスマッチを検出する。
- **Claude Code と Codex 向けの Agent Skills。** 17 skills を on-demand ロード (progressive disclosure)。両方の統合を実機で保守・検証する。外部依存なし、subagent は各プラットフォーム標準の spawn で立ち上がる。

Skills モードのワークフローと `/kiro-impl` 内部の詳細は [スキルリファレンス](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/skill-reference.md) を参照。

v1.x / v2.x からの移行は [Migration Guide](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/migration-guide.md#5-v2x--v30) を参照。

## なぜ specbind なのか

specbind は spec を、システムの各部分の間の契約として扱う。エージェントに手渡す「命令書」ではない。コードは依然として source of truth であり、spec はコード各部分の間の契約を明示化するために使う。そうすることで、人間とエージェントが常に同期を取らなくても並列に動けるようになる。

賭けはこうだ。適切な粒度で明示化された契約は、チーム規模の AI 駆動開発を速くする。遅くしない。エージェントが spec を書き、人間は phase gate で契約を承認し、出荷されるのはコードだ。

境界はオーバーヘッドではない。境界があるから自由に動ける。

設計の根拠、トレードオフ、向いている場面と向いていない場面の詳細は [specbind という賭け (philosophy note)](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/why-specbind.md)。

## クイックスタート

```bash
cd your-project
npx specbind@latest
```

デフォルトでは **Claude Code Skills** と英語ドキュメントがインストールされる。Codex または日本語を指定する場合:

```bash
npx specbind@latest --codex-skills --lang ja      # Codex、日本語
```

SpecBind の安定化期間中、公式対応言語は英語 (`en`) と日本語 (`ja`) のみ。追加言語はプロジェクトの安定後に再検討する。エージェントの詳細は [対応エージェント](#対応エージェント) を参照。

その後、エージェント上で:

```bash
/kiro-discovery <やりたいこと>
```

どこから始めれば良いか分からない場合は、まず `kiro-discovery` を実行する。依頼を整理して、次に叩くコマンドを教えてくれる。

### よくあるワークフロー

| やりたいこと | Skills モード |
|---|---|
| 新しい機能やプロダクトサイズの構想を始める | `kiro-discovery` → `kiro-spec-init` → `kiro-spec-requirements` → `kiro-spec-design` → `kiro-spec-tasks` → `kiro-impl` |
| 既存のシステムを拡張する | `kiro-steering` → `kiro-discovery` または `kiro-spec-init` → 任意で `kiro-validate-gap` → `kiro-spec-design` → `kiro-spec-tasks` → `kiro-impl` |
| 大きい initiative を分解する | `kiro-discovery` → `kiro-spec-batch` |
| spec 不要の小変更を入れる | `kiro-discovery` → 直接実装 |

規模の大きい承認済み task set に対しては、`kiro-impl` を走らせるとタスクごとの subagent spawn、independent review、失敗時の auto-debug 付きで自律実装が始まる。

## 実際の動き

例: 新規の Photo Albums 機能を作る。

```bash
/kiro-discovery Photo albums with upload, tagging, and sharing
# discovery が brief.md（マルチスペックなら roadmap.md も）を書いて、次のコマンドを提案する
/kiro-spec-init photo-albums
/kiro-spec-requirements photo-albums
/kiro-spec-design photo-albums
/kiro-spec-tasks photo-albums
/kiro-impl photo-albums
# 自律実行: タスクごとに fresh implementer, independent reviewer, auto-debug
```

spec フェーズの典型的な出力（10 分以内）:

- `requirements.md`: EARS 形式の要件と受け入れ基準。
- `design.md`: Mermaid 図と File Structure Plan 付きアーキテクチャ。
- `tasks.md`: 境界と依存関係のアノテーション付き実装タスク。

その後 `/kiro-impl` が feature flag 越しの TDD (RED → GREEN), 独立した reviewer pass, 失敗時の auto-debug と共にタスクを自律実行する。

## 対応エージェント

対応する2環境には同じ17-skillセットを配信し、両方を実機で検証する。

| エージェント | Skills モード | 状態 |
|---|---|---|
| **Claude Code** | `--claude-skills` | 対応 |
| **Codex** | `--codex-skills` | 対応 |

## インストール詳細

### 言語

```bash
npx specbind@latest --lang ja    # 日本語
# 対応言語: en, ja
```

### 高度なオプション

```bash
# 変更内容を先にプレビュー
npx specbind@latest --dry-run --backup

# カスタム specs ディレクトリ
npx specbind@latest --kiro-dir docs
```

## カスタマイズ

`{{KIRO_DIR}}/settings/` 以下のテンプレートとルールを編集して、チームのワークフローに合わせる。

- `templates/`: requirements, design, tasks のドキュメント構造。
- `rules/`: AI の生成原則と判断基準。

よくあるユースケース: PRD スタイルの要件、API とデータベーススキーマ、承認ゲート、JIRA 連携、ドメイン固有のスタンダード。

実践例とコピペ可能なスニペットは [カスタマイズガイド](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/customization-guide.md)。

## プロジェクト構造

インストール後、プロジェクトに以下が追加される:

```
project/
# いずれか1つがインストールされる
├── .claude/skills/           # 17 skills（Claude Code Skills、デフォルト）
├── .agents/skills/           # 17 skills（Codex Skills）
# プロジェクトメモリと spec 状態（共通）
├── .kiro/settings/templates/ # 共通テンプレート（{{KIRO_DIR}} を展開）
├── .kiro/settings/rules/     # 共通ルール
├── .kiro/specs/              # 機能仕様書
├── .kiro/steering/           # AI 指導ドキュメント
└── CLAUDE.md / AGENTS.md     # プロジェクト設定（エージェントごと）
```

実際に作成されるのは、選択したエージェントに対応するディレクトリのみ。

## ドキュメント

- スキルリファレンス: [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/skill-reference.md) | [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/skill-reference.md)
- コマンドリファレンス: [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/command-reference.md) | [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/command-reference.md)
- カスタマイズガイド: [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/customization-guide.md) | [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/customization-guide.md)
- 仕様駆動開発ガイド: [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/spec-driven.md) | [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/spec-driven.md)
- specbind という賭け: [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/why-specbind.md) | [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/why-specbind.md)
- Claude Subagents ガイド: [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/claude-subagents.md) | [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/claude-subagents.md)
- マイグレーションガイド: [日本語](https://github.com/Huruikagi/specbind/blob/main/docs/guides/ja/migration-guide.md) | [English](https://github.com/Huruikagi/specbind/blob/main/docs/guides/migration-guide.md)
- [Issues & サポート](https://github.com/Huruikagi/specbind/issues) - バグ報告と質問
- [Kiro IDE](https://kiro.dev)

---

**安定版リリース v3.0.0** 本番環境対応。[問題を報告](https://github.com/Huruikagi/specbind/issues) | MIT License

### プラットフォーム対応

- 対応 OS: macOS, Linux, Windows（自動検出）。
- すべての OS で統一コマンドテンプレートを提供。`--os` 指定は後方互換用の任意オプション。
