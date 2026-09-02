# cc-sddから移行する

このページは、既存の`.kiro`プロジェクトをSpecBindへ移行するための手順です。

SpecBindが自動で変換するのは、意味を機械的に確認できる入力だけです。Milestoneの
範囲、Designのトレーサビリティ、成果物の言語など、人の判断が要る部分に行き当たった
場合、CLIは推測せずに停止します。そこから先は、エージェントに支援してもらう移行に
切り替えます。

!!! warning "プレビュー"

    旧Specなどの意味判断には、エージェント支援の経路を使います。`specbind install`は
    `.kiro`を変換しませんが、読み取り専用の計画を確認した後、合意した言語とエージェントに
    対応するSpecBindの移行先を準備するために使えます。移行完了の判定と旧資産の退役には、
    必ずこのページの移行判断の受理と`--apply`へ戻ってください。

## 安全境界

移行では、次の境界を必ず守ります。

- 最初に、Gitの状態と変更内容を確認してから始める。
- 読み取り専用の計画とエージェント支援作業の間は、移行元の`.kiro`ツリーを削除・移動・
  上書きしない。最終`--apply`だけがGit追跡を確認して退役させる。
- cc-sddの承認を、SpecBindのGateの承認証拠としてコピーしない。
- 分からないMilestone、リリース履歴、Contract、Requirementの対応関係を作り話で
  埋めない。
- 旧`kiro-*`のエージェント資産は、変換後の検証とあなたの確認が済むまで残す。
- 最終退役する`.kiro`、`.cc-sdd.json`、旧エージェント資産、移行判断の状態ファイルが
  Gitで追跡されていなければ停止する。無視対象のファイルも削除しない。
- CLIが所有する状態は、対応するSpecBindコマンドがある限り手編集しない。

## 1. 読み取り専用の計画を取得する

対象プロジェクトのルートで実行します。

```sh
specbind migrate cc-sdd
```

すべてを一意に変換できる場合、CLIは作成・変換・退役する予定の対象を表示します。
`--apply`は計画を再計算し、コミット済みで未コミットの変更がないことを確認してから、
既知の自動変換だけを適用します。退役対象がGit未追跡なら、回復できない削除を避けるため
適用を停止します。

```sh
specbind migrate cc-sdd --apply
```

`MANUAL_MIGRATION_REQUIRED`が返ったときは、`--apply`で押し切ろうとしないで
ください。CLIが表示した診断コード、対象パス、理由を控えて、次の手順へ進みます。

現在の自動適用範囲は、`.cc-sdd.json`からのSpecBindのインストールと、Codex・Claude Codeの
既知の`kiro-*`スキルおよびGit追跡済みcc-sdd移行元の最終退役です。旧Specが1件でもあれば
`MIGRATE_SPEC_CONVERSION_REQUIRED`で停止し、MilestoneやGateの承認証拠を推測しません。

## 2. エージェントに依頼する

CodexまたはClaude Codeに、CLIの出力全体とこのページのURLを渡します。

```text
次の公式ガイドを読み、このリポジトリのcc-sdd移行を進めてください。
最初にspecbind migrate cc-sddの診断と対象ファイルを確認し、ガイドの停止条件を
守ってください。証明できない承認・完了状態を作らず、最後にCLI検証へ戻って
ください。

https://huruikagi.github.io/specbind/ja/guide/migrate-from-cc-sdd/
```

エージェントには、既存の`AGENTS.md`や`CLAUDE.md`を含むリポジトリ側の指示も
そのまま適用されます。この移行ガイドが、そこに書かれた権限やGitの方針を
上書きすることはありません。

## 3. あなたが決めること

エージェントは、まずリポジトリから判断できることを自分で調べます。そのうえで、
次のように証拠から決められない事柄だけ、選択肢と影響を添えてあなたに確認します。

- 進行中の複数のSpecを、同じ進行中のMilestoneにまとめてよいか
- 言語が混在した成果物を、どちらの言語に揃えるか
- 完了しているように見える旧Specを、現在の実装済みの基準として受け入れるか
- カスタマイズされたルールのうち、どの方針を新しいプロジェクト所有のルールへ
  引き継ぐか
- 曖昧に書き換えられた旧クイックスタートを、どこまで手作業で取り除くか

確認が取れない場合、エージェントはその箇所で停止します。

## 4. 成果物を変換する

エージェントが手を付けるのは、CLIが検出した項目に対応する範囲だけです。

| cc-sdd入力 | SpecBind側 | 境界 |
| --- | --- | --- |
| `.cc-sdd.json` | `.specbind.json` | 旧`kiroDir`、言語、エージェントを検査し、SpecBindの設定として新規作成する |
| `spec.json` | `spec.yaml` | フェーズと承認の組合せを検査する。Gateの承認証拠は作り直さない |
| `requirements.md` | `SpecBind Requirements` | 既知の見出しとAcceptance CriteriaからRequirement IDを検証する |
| `design.md` | `SpecBind Design` | Front Matterと本文マーカーのRequirement対応を一致させる |
| `tasks.md` | `tasks.yaml` | 既知のタスク記法だけを変換し、証明できる進捗だけを引き継ぎ、`(P)`は保持せず移行先の順序へ直列化する |
| Implementation Notes | `implementation-notes.md` | 中身があり、今後も残す価値のあるノートだけを切り出す |
| steering | `SpecBind Steering` | 文書の責務と、安定した`artifact_id`を確認する |
| 旧ルール | 新しいプロジェクト所有のルール | 既定値との差分を方針としてレビューする。ファイルを丸ごとコピーしない |

Contractが存在しない場合、エージェントは「影響なし」と決めつけて空のContractを
作ることはしません。現在のRequirementsとDesignをもとに、通常のSpecBind Design
ワークフローでContractを作り、必要なレビューと承認をやり直します。

## 5. 移行判断をCLIへ受け渡す

まず、合意した言語とエージェントでSpecBindの移行先を準備し、変換した成果物を通常のCLIで
検証します。`.kiro`を入力にした変換処理としてではなく、SpecBind側の土台を作る
操作として`specbind install`を使います。変換結果をレビューし、コミットして
未コミットの変更をなくした状態を回復点にしてください。

次に、エージェントは現在の診断項目をすべて正確に列挙した厳密なJSON候補を、
プロジェクト外の一時ファイルまたは標準入力からCLIへ渡します。

```json
{
  "schemaVersion": 1,
  "assessment": "旧ルールを比較し、現在も必要な方針だけをSpecBind用に書き直した。",
  "target": { "language": "ja", "agents": ["codex"] },
  "resolutions": [
    {
      "code": "MIGRATE_RULE_REVIEW_REQUIRED",
      "path": ".kiro/settings/rules",
      "disposition": "converted",
      "targets": [".specbind/settings/rules/project.md"]
    }
  ]
}
```

```sh
specbind migrate cc-sdd --accept-resolution ../cc-sdd-resolution.json
```

`converted`は具体的な移行先を1件以上必要とします。意図的に移行しない診断項目は
`not_migrated`とし、`targets`を空にします。候補は現在の意味判断が必要な診断項目を
過不足なく含む必要があり、安全性に関する診断項目を判断だけで抑制することはできません。

CLIは移行元と移行先を再検証し、フィンガープリントを自分で計算して
`.specbind/state/cc-sdd-migration.yaml`へ保存します。このファイルは手編集せず、内容を
レビューしてコミットしてください。移行元または移行先が後で変われば移行判断は古くなり、
元の診断項目が再表示されます。この状態ファイルは一時的な受け渡し手段で、最終`--apply`が
cc-sddの移行元と一緒に削除します。受理内容はGit履歴に残ります。

## 6. CLI検証へ戻る

エージェントによる作業が終わったら、もう一度、読み取り専用の計画を実行します。

```sh
specbind migrate cc-sdd
```

診断項目が残っていれば、その対象だけを解消してください。変換して有効になった
SpecBind成果物を、旧入力から再生成して上書きしてはいけません。

移行を完了と見なせるのは、次の2つがそろったときだけです。エージェントによる
作業を移行実装が認識し、残りの安全な処理だけを計画していること。そして、通常の
SpecBind検証が成功すること。少なくとも、対象に応じて次を確認します。

```sh
specbind artifact list <spec>
specbind check traceability <spec>
specbind check contracts
specbind spec status <spec>
specbind milestone status
```

移行判断の記録をコミットし、未コミットの変更がない状態で、最終切り替えを適用します。
このコマンドの実行が退役の明示確認です。CLIはすべての後片付け対象を再検証し、1つでも
未追跡、無視対象、リンク、変更済みなら何も削除しません。

```sh
specbind migrate cc-sdd --apply
```

## 7. 旧ワークフローを止める

成功すると、設定されたcc-sddの移行元ルート、`.cc-sdd.json`、既知の旧`kiro-*`スキル、
移行判断の状態ファイルが削除され、SpecBindだけが稼働中のワークフローとして残ります。
再実行は`NO_CHANGE CC_SDD_MIGRATION_COMPLETE`です。

`AGENTS.md`や`CLAUDE.md`の編集済み・混在・重複した旧指示は、Gitで戻せる場合でもCLIが
範囲を推測して消しません。エージェント支援作業中に意味を確認して編集・コミットしてから
最終切り替えへ進みます。後片付け中にファイルシステムエラーが起きた場合は、
切り替え直前のコミットをGitで復元してから再実行してください。

## 診断コード

### MIGRATE_TARGET_ALREADY_EXISTS {#migrate-target-already-exists}

`.specbind.json`または`.specbind`がすでにあります。既存の有効なSpecBind成果物を
上書きせず、旧入力と現在の対象状態を照合してください。

### MIGRATE_AGENT_SELECTION_REQUIRED / MIGRATE_AGENT_UNSUPPORTED {#migrate-agent-selection-required}

CodexまたはClaude Codeという移行先を一意に決められないか、旧設定のエージェントが
SpecBind v1の対象外です。利用するエージェントを確認してください。

### MIGRATE_LANGUAGE_UNSUPPORTED {#migrate-language-unsupported}

旧設定またはSpecのメタデータの言語が、SpecBind v1の英語・日本語に含まれません。
対象言語と翻訳範囲を確認してください。

### MIGRATE_LANGUAGE_SELECTION_REQUIRED {#migrate-language-selection-required}

旧設定やSpecのメタデータから英語・日本語を決められません。自動適用前に成果物言語を
選択してください。

### MIGRATE_SPEC_CONVERSION_REQUIRED {#migrate-spec-conversion-required}

旧Specがあります。進行中のMilestone、Requirement対応、Gateの承認証拠を推測せず、
エージェント支援の手順で成果物を変換してからCLI検証へ戻ります。

### MIGRATE_ACTIVE_SCOPE_AMBIGUOUS {#migrate-active-scope-ambiguous}

進行中に見える旧Specが複数あるものの、それらを1つの進行中のMilestoneとして
扱ってよい根拠がありません。旧Roadmap、依存関係、いま進めようとしている作業を
調べたうえで、範囲をあなたに確認します。

### MIGRATE_DESIGN_TRACEABILITY_REQUIRED {#migrate-design-traceability-required}

旧Designからは、SpecBindが求めるRequirement対応を機械的に組み立てきれません。
RequirementsとDesignを読み、各Design成果物のFront Matterと本文マーカーが同じ集合を
指すように直してから、CLIで検証します。

### MIGRATE_LANGUAGE_MIXED {#migrate-language-mixed}

旧Specの成果物の言語が混在しています。SpecBindは成果物の言語をプロジェクトごとに
1つへ揃えるため、あなたが言語と翻訳範囲を決めるまで停止します。

### MIGRATE_LEGACY_INSTRUCTIONS_AMBIGUOUS {#migrate-legacy-instructions-ambiguous}

`AGENTS.md`または`CLAUDE.md`にある旧案内が、既知のブロックと完全には一致しません。
`kiro`という語が入っていることだけを根拠に削除せず、まわりのプロジェクト所有の
指示を保持したうえで、対象範囲をあなたに確認します。

### MIGRATE_RULE_REVIEW_REQUIRED / MIGRATE_TEMPLATE_REVIEW_REQUIRED {#migrate-rule-review-required}

旧ルールまたはテンプレートがあります。現在のSpecBind既定値との差分を、プロジェクト所有の
方針または上書き設定として残すべきか確認し、手順を丸ごとコピーしません。

### MIGRATE_STEERING_REVIEW_REQUIRED {#migrate-steering-review-required}

旧steering文書があります。文書の責務と安定した`artifact_id`を決め、SpecBind
Steeringとして検証してください。

### MIGRATE_SPEC_DIRECTORY_INVALID / MIGRATE_SPEC_ID_INVALID {#migrate-spec-path-invalid}

旧Specのパスが通常ディレクトリではないか、正規のkebab-case IDではありません。
リンクをたどらず、意図したSpec IDと配置を確認してください。

### MIGRATE_SPEC_METADATA_MISSING / MIGRATE_SPEC_STATE_INVALID {#migrate-spec-state-invalid}

`spec.json`がないか、`phase`・`generated`・`approved`の組合せが旧cc-sddの状態として
成立しません。成果物と履歴から状態を調べ、証明できないGateの承認証拠は作りません。

### MIGRATE_LEGACY_AGENT_ASSET_INVALID / MIGRATE_LEGACY_AGENT_ASSET_UNKNOWN / MIGRATE_LEGACY_CONTENT_UNSUPPORTED {#migrate-legacy-content-unsupported}

既知の旧エージェント資産が通常ディレクトリではない、未知の`kiro-*`資産がある、または
`.kiro`直下に未対応の内容があります。対象を個別に調べ、変換するか意図的に
移行しないかを移行判断へ記録します。最終切り替え後の移行元はGit履歴に残ります。

### MIGRATE_RESOLUTION_STALE / MIGRATE_RESOLUTION_STATE_INVALID {#migrate-resolution-stale}

受理済みの移行判断に対応する移行元、移行先、診断項目、または選択済みのインストールが
変わったか、CLI所有の状態ファイルが壊れています。状態ファイルを手編集せず、現在の診断項目を
もう一度レビューして、
新しい外部候補を`--accept-resolution`で受理し直してください。

### MIGRATION_CLEANUP_TARGET_UNTRACKED / MIGRATION_CLEANUP_TARGET_UNSAFE {#migration-cleanup-target-unsafe}

最終切り替えの対象に、Git未追跡または無視対象のファイル、リンク、再解析ポイント、
不正な対象が含まれています。必要な内容をコミットするか旧資産のルート外へ移動し、
未コミットの変更がない状態でやり直してください。CLIは回復できないファイルを削除しません。

---

[ユーザーガイド](../index.md) | [既存プロジェクトで始める](./start-existing-project.md)
