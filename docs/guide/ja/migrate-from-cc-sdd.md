# cc-sddから移行する

このガイドは、既存の`.kiro`プロジェクトをSpecBindへ移行するためのものです。
SpecBindは、機械的に意味を証明できる入力だけを自動変換します。Milestoneの範囲、
Designのトレーサビリティ、成果物の言語などに意味判断が必要な場合は、安全のため
停止し、エージェント支援マイグレーションへ切り替えます。

!!! warning "Preview"

    `specbind migrate cc-sdd`は現在のPreview CLIにはまだ実装されていません。
    このページは、コマンドの公開に先立って確定した移行手順を公開しています。
    通常の`specbind install`で`.kiro`を移行しないでください。

## 安全境界

- 最初にGitの状態と変更内容を確認します。
- 移行元の`.kiro`ツリーは削除・移動・上書きしません。
- cc-sddのapprovalをSpecBindのgate evidenceとしてコピーしません。
- 不明なMilestone、リリース履歴、Contract、Requirement対応を捏造しません。
- 旧`kiro-*`エージェント資産は、変換後の検証と利用者確認が終わるまで残します。
- CLIが所有する状態は、対応するSpecBindコマンドがある場合は手編集しません。

## 1. 読み取り専用の計画を取得する

対象プロジェクトのルートで実行します。

```sh
specbind migrate cc-sdd
```

すべてを一意に変換できる場合は、CLIが作成、変換、保持、除去予定の対象を表示
します。内容をレビューした後でのみ適用します。

```sh
specbind migrate cc-sdd --apply
```

`MANUAL_MIGRATION_REQUIRED`が返った場合、`--apply`を試して回避しないでください。
CLIが表示したfinding code、対象パス、理由を保存して、次の手順へ進みます。

## 2. エージェントに依頼する

CodexまたはClaude Codeへ、CLIの完全な出力とこのページのURLを渡します。

```text
次の公式ガイドを読み、このリポジトリのcc-sdd移行を進めてください。
最初にspecbind migrate cc-sddの診断と対象ファイルを確認し、ガイドの停止条件を
守ってください。証明できない承認・完了状態を作らず、最後にCLI検証へ戻って
ください。

https://huruikagi.github.io/specbind/guide/ja/migrate-from-cc-sdd/
```

エージェントには、既存の`AGENTS.md`や`CLAUDE.md`を含むリポジトリの指示も
適用されます。移行ガイドはそれらの権限やGit方針を上書きしません。

## 3. 利用者が決める事項

エージェントはリポジトリから判断できることを先に調べます。次のような意味判断を
証明できない場合だけ、選択肢と影響を示して利用者へ確認します。

- 複数の進行中Specを同じactive milestoneへ含めるか
- mixed languageの成果物をどの言語へ正規化するか
- 完了済みに見えるlegacy Specを現在の実装済みbaselineとして受け入れるか
- customized ruleのどの方針を新しいproject-owned ruleへ移すか
- 曖昧に編集されたlegacy quickstartをどこまで手動で除去するか

確認が得られない場合、エージェントは該当箇所で停止します。

## 4. 成果物を変換する

エージェントはCLIのfindingに対応する範囲だけを扱います。

| cc-sdd入力 | SpecBind側 | 境界 |
| --- | --- | --- |
| `spec.json` | `.specbind.json`、`spec.yaml` | phaseとapprovalの組合せを検査する。gate evidenceは再生成しない |
| `requirements.md` | `SpecBind Requirements` | 既知の見出しとAcceptance CriteriaからRequirement IDを検証する |
| `design.md` | `SpecBind Design` | Front Matterと本文markerのRequirement対応を一致させる |
| `tasks.md` | `tasks.yaml` | 対応する既知のtask文法だけを変換し、証明できる進捗だけを保持する |
| Implementation Notes | `implementation-notes.md` | durableな非空ノートだけを分離する |
| steering | `SpecBind Steering` | 文書の責務と安定した`artifact_id`を確認する |
| legacy rules | 新しいproject-owned rules | 既定値との差分を方針としてレビューし、ファイルを丸ごとコピーしない |

Contractが存在しない場合、エージェントは「影響なし」と推測して空のContractを
作りません。現在のRequirementsとDesignから通常のSpecBind Designワークフローで
Contractを作成し、必要なレビューと承認をやり直します。

## 5. CLI検証へ戻る

guided workの後、再び読み取り専用計画を実行します。

```sh
specbind migrate cc-sdd
```

残っているfindingがあれば、その対象だけを解消します。変換済みの有効なSpecBind
成果物を、legacy入力から再生成して上書きしてはいけません。

移行実装がguided workを認識して安全な残処理だけを計画し、通常のSpecBind検証が
成功するまでは、移行完了を宣言しません。少なくとも対象に応じて次を確認します。

```sh
specbind artifact list <spec>
specbind check traceability <spec>
specbind check contracts
specbind spec status <spec>
specbind milestone status
```

## 6. 旧ワークフローを停止する

変換後の状態が有効であることを確認し、利用者がcutoverを承認した後に限り、CLIが
計画した正確に既知の`kiro-*`エージェント資産とlegacy quickstart blockを除去
します。編集済み、混在、重複した指示は推測削除せず、個別に確認します。

元の`.kiro`ツリーは移行後も残ります。復旧が必要な場合はGitでSpecBind側の変更を
確認・復元できますが、旧スキルと新スキルを同時に使って状態を更新しないでください。

## Finding code

### MIGRATE_ACTIVE_SCOPE_AMBIGUOUS {#migrate-active-scope-ambiguous}

複数のlegacy Specが進行中に見えますが、一つのactive milestoneとして扱える証拠が
ありません。legacy roadmap、依存関係、現在の作業意図を調べ、利用者に範囲を確認
します。

### MIGRATE_DESIGN_TRACEABILITY_REQUIRED {#migrate-design-traceability-required}

legacy Designから、SpecBindが要求する完全なRequirement対応を機械的に構築できません。
RequirementsとDesignを読み、各Design artifactのFront Matterと本文markerを同じ集合へ
修正した後、CLIで検証します。

### MIGRATE_LANGUAGE_MIXED {#migrate-language-mixed}

legacy Specの成果物言語が混在しています。SpecBindはプロジェクト全体で一つの成果物
言語を使用するため、利用者が言語と翻訳範囲を決めるまで停止します。

### MIGRATE_LEGACY_INSTRUCTIONS_AMBIGUOUS {#migrate-legacy-instructions-ambiguous}

`AGENTS.md`または`CLAUDE.md`のlegacy案内が、既知の完全一致blockではありません。
`kiro`という語だけを根拠に削除せず、周囲のproject-owned instructionsを保持して
利用者と対象範囲を確認します。

---

[移行ガイド入口](../migration/cc-sdd.md) | [English](../en/migrate-from-cc-sdd.md)
