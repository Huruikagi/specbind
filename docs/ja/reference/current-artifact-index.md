# 成果物一覧

このページでは、現在の`specbind` CLIがプロジェクトに配置するファイルと、CLIや
スキルが管理する成果物をまとめます。これらを扱うワークフローは
[スキル一覧](./current-skill-index.md)を参照してください。

以下のパスは、既定のSpecルート`.specbind/`で書いています。`.specbind.json`で
別の`specDir`を設定している場合は、`.specbind/`をそのディレクトリに読み替えて
ください。

## 各ファイルの所有者

| 所有者 | 意味 |
| --- | --- |
| **Project** | 自由に編集してよいファイルです。`specbind install`で上書きされません。 |
| **Skill** | プロジェクトの内容ですが、手で編集せず、担当スキルを通して更新します。 |
| **CLI** | CLIの操作だけが書き込む構造化された状態です。手で編集しないでください。 |
| **Product** | 製品管理対象です。`specbind install`で埋め込み版に置き換わるので、編集しないでください。 |

## 全体像

```text
.specbind.json                          Project   プロジェクト設定
.specbind/
├─ index.md                             Project   成果物全体の入口（マーカー内はProduct）
├─ settings/
│  ├─ templates/                        Project   成果物のひな形
│  ├─ rules/                            Project   共有ルール（固定の7ファイル）
│  └─ adapters/                         Project   運用アダプター（固定の4ファイル）
├─ steering/
│  ├─ roadmap.md                        CLI       進行中のMilestone
│  └─ <path>.md                         Skill     Steering文書
├─ specs/
│  ├─ shared-contract.yaml              Skill     共有Contract（任意）
│  └─ <spec>/                                     Specごとの成果物（後述）
├─ state/                               CLI       受理済みのContractレビュー
├─ releases/                            CLI       リリースアーカイブ
├─ baselines/                           CLI       Spec確立の記録
├─ deferred.md                          Skill     保留した指摘（任意）
└─ adoption/                            Skill     Specの確立中だけ存在
.agents/skills/sb-*/                    Product   Codexとgeneric用のスキル
.claude/skills/sb-*/                    Product   Claude Code用のスキル
.codex/agents/specbind-*.toml           Product   Codexの役割定義
.claude/agents/specbind-*.md            Product   Claude Codeの役割定義
AGENTS.md / CLAUDE.md（マーカー内）       Product   プロジェクト指示（任意）
```

## プロジェクト設定

| ファイル | 所有者 | 内容 |
| --- | --- | --- |
| `.specbind.json` | Project | Specルート、成果物の言語、使うエージェント、プロジェクト指示の統合、任意の`agentRoles`上書き。`specDir`はインストール時に固定されます。エージェントと言語は`specbind install`で変更します。 |
| `.specbind/index.md` | Project | 成果物全体の入口です。SpecBindが更新するのはマーカー内の案内ブロックだけなので、プロジェクト固有のリンクはマーカーの外に書きます。 |

## 設定: `.specbind/settings/`

初回のインストールで作成され、その後は上書きされません。編集の方法は
[カスタマイズ](../guide/customization.md)を参照してください。

### `templates/`

| ファイル | 内容 |
| --- | --- |
| `specs/requirements.md` | Requirementsの構成と作成用ひな形 |
| `specs/design.md` | 主Designのひな形。すべてのSpecで必須 |
| `specs/ui.md` | 画面設計の条件付きひな形 |
| `roadmap.md` | Milestone Roadmapの本文。Front MatterはCLIが所有 |

ほかのひな形はバイナリに埋め込まれており、`specbind template list`で一覧できます。
上書きしたい場合だけ、表示された`template_path`へコピーします。

### `rules/`

| ファイル | 内容 |
| --- | --- |
| `ears-format.md` | Requirementsの書き方 |
| `design-principles.md` | Designの方針 |
| `design-template-selection.md` | 各Designテンプレートを必須・条件付き・無効のどれにするか |
| `contract-principles.md` | 所有境界、接点、互換性の方針 |
| `tasks-generation.md` | Taskの分け方 |
| `steering-principles.md` | Steeringの書き方 |
| `language-style.md` | 文章のスタイル。日本語の場合だけ配置 |

### `adapters/`

| ファイル | 内容 |
| --- | --- |
| `release.md` | リリースの準備、公開、検証、後片付け |
| `git.md` | コミットの方針。既定では作業単位ごとにローカルコミットし、pushしない |
| `deferred.md` | Gateを止めない指摘の記録先 |
| `validation.md` | 最終の実装検証に追加する、プロジェクト固有の確認手順 |

## Milestoneとプロジェクトの状態

| ファイル | 所有者 | 存在する期間と内容 |
| --- | --- | --- |
| `steering/roadmap.md` | CLI | 進行中のMilestoneのスコープ、依存関係、対象リリース、Direct項目の状態。本文はDiscoveryが書きます。リリース時に`releases/`へ移ります。 |
| `steering/<path>.md` | Skill | 長く維持するSteering文書。`sb-steering`が保守します。 |
| `state/contract-review.md` | CLI | 受理済みの、Milestone全体の最新のContractレビュー。`sb-contract-review`が作成します。 |
| `state/cc-sdd-migration.yaml` | CLI | cc-sddからの移行中だけ存在します。 |
| `releases/<version>-roadmap.md` | CLI | リリースしたMilestoneのRoadmapのアーカイブ。 |
| `releases/<version>-contract-review.md` | CLI | リリースしたMilestoneのContractレビューのアーカイブ。 |
| `baselines/<version>-roadmap.md` / `baselines/<version>-contract-review.md` | CLI | `sb-adopt`でSpecを確立したときのRoadmapとContractレビューの記録。リリースの記録ではありません。 |
| `specs/shared-contract.yaml` | Skill | 複数の機能が使う資源のための、任意の共有Contract。リリース後も残ります。[カスタマイズ](../guide/customization.md#shared-contract)を参照してください。 |
| `deferred.md` | Skill | 既定のdeferredアダプターが、最初の保留指摘を記録するときに作成します。作業キューではありません。 |
| `adoption/reverse-discovery.yaml` | Skill | `sb-adopt`の一時的な調査記録。Specの確立が完了すると削除されます。 |

## Specごとの成果物: `.specbind/specs/<spec>/`

| ファイル | 所有者 | 存在する期間 | 内容 |
| --- | --- | --- | --- |
| `spec.yaml` | CLI | 永続 | ライフサイクル、Gate、完了の状態 |
| `requirements.md` | Skill | 永続 | 現在のRequirementsの全体（`sb-plan`） |
| `design.md`とほかのDesign文書 | Skill | 永続 | Designの一式（`sb-plan`） |
| `contract.yaml` | Skill | 永続 | Contract。Milestone全体でレビューします |
| `log.md` | CLI | 永続 | 新しい順のリリース履歴 |
| `implementation-notes.md` | Skill | 永続・任意 | 実装の記録 |
| `brief.md` | Skill | 進行中のMilestone | Discoveryの要約と関連資料 |
| `research.md` | Skill | 進行中のMilestone・任意 | ギャップ分析の結果（`sb-gap-analysis`） |
| `tasks.yaml` | SkillとCLI | 進行中のMilestone | Taskの計画（`sb-plan`）と実行の進捗（CLI） |

進行中のMilestoneだけの成果物は、リリースの確定処理で削除されます。Markdownの成果物は
種類と`artifact_id`で識別されるので、上のファイル名は既定値であり、必須の名前では
ありません。

## バイナリに埋め込まれているもの

次のものはCLIから読み取るだけで、プロジェクトのファイルとしては配置されません。

- Spec、Steering、Roadmapのテンプレート: `specbind template list` / `template read`
- 製品プロトコル: `specbind protocol list` / `protocol read`
- 構造化成果物とコマンド入力のスキーマ: `specbind schema list` / `schema read`

テンプレートの指示と上書きの方法は[カスタマイズ](../guide/customization.md)を
参照してください。
