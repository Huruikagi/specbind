# cc-sddから移行する

このページは、既存の`.kiro`プロジェクトをSpecBindへ移行するための手順です。

SpecBindが自動で変換するのは、意味を機械的に確認できる入力だけです。Milestoneの
範囲、Designのトレーサビリティ、成果物の言語など、人の判断が要る部分に行き当たった
場合、CLIは推測せずに停止します。そこから先は、エージェントに支援してもらう移行に
切り替えます。

!!! warning "Preview"

    旧Specなどの意味判断はagent-assisted経路です。`specbind install`は
    `.kiro`を変換しませんが、読み取り専用planの後で、合意した言語とagentの
    SpecBind targetを準備するために使えます。移行完了の判定と旧資産の退役には、
    必ずこのページのresolution受理と`--apply`へ戻ってください。

## 安全境界

移行では、次の境界を必ず守ります。

- 最初に、Gitの状態と変更内容を確認してから始める。
- 読み取り専用planとagent-assisted作業の間は、移行元の`.kiro`ツリーを削除・移動・
  上書きしない。最終`--apply`だけがGit追跡を確認して退役させる。
- cc-sddの承認を、SpecBindのGateの承認証拠としてコピーしない。
- 分からないMilestone、リリース履歴、Contract、Requirementの対応関係を作り話で
  埋めない。
- 旧`kiro-*`のエージェント資産は、変換後の検証とあなたの確認が済むまで残す。
- 最終退役する`.kiro`、`.cc-sdd.json`、旧agent資産、resolution stateの全ファイルが
  Gitで追跡されていなければ停止する。ignored fileも削除しない。
- CLIが所有する状態は、対応するSpecBindコマンドがある限り手編集しない。

## 1. 読み取り専用の計画を取得する

対象プロジェクトのルートで実行します。

```sh
specbind migrate cc-sdd
```

すべてを一意に変換できる場合、CLIは作成・変換・退役する予定の対象を
表示します。`--apply`は計画を再計算し、commit済みでcleanなGit状態を確認してから、
既知の自動変換だけを適用します。退役対象がGit未追跡なら、回復できない削除を
避けるため適用を停止します。

```sh
specbind migrate cc-sdd --apply
```

`MANUAL_MIGRATION_REQUIRED`が返ったときは、`--apply`で押し切ろうとしないで
ください。CLIが表示したfinding code、対象パス、理由を控えて、次の手順へ進みます。

現在の自動適用範囲は、`.cc-sdd.json`からのSpecBind installと、Codex・Claude Codeの
既知の`kiro-*` skillおよびGit追跡済みcc-sdd sourceの最終退役です。旧Specが1件でもあれば
`MIGRATE_SPEC_CONVERSION_REQUIRED`で停止し、MilestoneやGate evidenceを推測しません。

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

- 進行中の複数のSpecを、同じactive milestoneにまとめてよいか
- 言語が混在した成果物を、どちらの言語に揃えるか
- 完了しているように見える旧Specを、現在の実装済みbaselineとして受け入れるか
- カスタマイズされたルールのうち、どの方針を新しいプロジェクト所有のルールへ
  引き継ぐか
- 曖昧に書き換えられた旧quickstartを、どこまで手作業で取り除くか

確認が取れない場合、エージェントはその箇所で停止します。

## 4. 成果物を変換する

エージェントが手を付けるのは、CLIが検出した項目に対応する範囲だけです。

| cc-sdd入力 | SpecBind側 | 境界 |
| --- | --- | --- |
| `.cc-sdd.json` | `.specbind.json` | 旧`kiroDir`、言語、エージェントを検査し、SpecBindの設定として新規作成する |
| `spec.json` | `spec.yaml` | phaseとapprovalの組合せを検査する。Gateの承認証拠は作り直さない |
| `requirements.md` | `SpecBind Requirements` | 既知の見出しとAcceptance CriteriaからRequirement IDを検証する |
| `design.md` | `SpecBind Design` | Front Matterと本文markerのRequirement対応を一致させる |
| `tasks.md` | `tasks.yaml` | 既知のtask記法だけを変換し、証明できる進捗だけを引き継ぎ、`(P)`は保持せずtargetの順序へ直列化する |
| Implementation Notes | `implementation-notes.md` | 中身があり、今後も残す価値のあるノートだけを切り出す |
| steering | `SpecBind Steering` | 文書の責務と、安定した`artifact_id`を確認する |
| 旧ルール | 新しいプロジェクト所有のルール | 既定値との差分を方針としてレビューする。ファイルを丸ごとコピーしない |

Contractが存在しない場合、エージェントは「影響なし」と決めつけて空のContractを
作ることはしません。現在のRequirementsとDesignをもとに、通常のSpecBind Design
ワークフローでContractを作り、必要なレビューと承認をやり直します。

## 5. 移行判断をCLIへ受け渡す

まず、合意した言語とagentでSpecBindのtargetを準備し、変換した成果物を通常のCLIで
検証します。`.kiro`を入力にした変換処理としてではなく、SpecBind側の土台を作る
操作として`specbind install`を使います。変換結果をレビューし、cleanなcommitを
回復点にしてください。

次に、エージェントは現在のfindingをすべて正確に列挙したstrict JSON候補を、
プロジェクト外の一時ファイルまたはstdinからCLIへ渡します。

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

`converted`は具体的なtargetを1件以上必要とします。意図的に移行しないfindingは
`not_migrated`とし、`targets`を空にします。候補は現在のsemantic findingを過不足なく
含む必要があり、安全性findingを判断だけで抑制することはできません。

CLIはsourceとtargetを再検証し、fingerprintを自分で計算して
`.specbind/state/cc-sdd-migration.yaml`へ保存します。このファイルは手編集せず、内容を
レビューしてcommitしてください。sourceまたはtargetが後で変わればresolutionはstaleに
なり、元のfindingが再表示されます。このstateは一時的なhandshakeで、最終`--apply`が
cc-sdd sourceと一緒に削除します。受理内容はGit履歴に残ります。

## 6. CLI検証へ戻る

エージェントによる作業が終わったら、もう一度、読み取り専用の計画を実行します。

```sh
specbind migrate cc-sdd
```

findingが残っていれば、その対象だけを解消してください。変換して有効になった
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

resolution recordをcommitしたcleanな状態で、最終cutoverを適用します。このコマンドの
実行が退役の明示確認です。CLIは全cleanup targetを再検証し、1つでもuntracked、ignored、
link、変更済みなら何も削除しません。

```sh
specbind migrate cc-sdd --apply
```

## 7. 旧ワークフローを止める

成功すると、設定されたcc-sdd source root、`.cc-sdd.json`、既知の旧`kiro-*` skill、
resolution stateが削除され、SpecBindだけがactive workflowとして残ります。再実行は
`NO_CHANGE CC_SDD_MIGRATION_COMPLETE`です。

`AGENTS.md`や`CLAUDE.md`の編集済み・混在・重複した旧指示は、Gitで戻せる場合でもCLIが
範囲を推測して消しません。agent-assisted作業中に意味を確認して編集・commitしてから
最終cutoverへ進みます。cleanup中にfilesystem errorが起きた場合は、cutover直前のcommitを
Gitで復元してから再実行してください。

## Finding code

### MIGRATE_TARGET_ALREADY_EXISTS {#migrate-target-already-exists}

`.specbind.json`または`.specbind`がすでにあります。既存の有効なSpecBind成果物を
上書きせず、旧入力と現在の対象状態を照合してください。

### MIGRATE_AGENT_SELECTION_REQUIRED / MIGRATE_AGENT_UNSUPPORTED {#migrate-agent-selection-required}

CodexまたはClaude Codeという移行先を一意に決められないか、旧設定のエージェントが
SpecBind v1の対象外です。利用するエージェントを確認してください。

### MIGRATE_LANGUAGE_UNSUPPORTED {#migrate-language-unsupported}

旧設定またはSpec metadataの言語が、SpecBind v1の英語・日本語に含まれません。
対象言語と翻訳範囲を確認してください。

### MIGRATE_LANGUAGE_SELECTION_REQUIRED {#migrate-language-selection-required}

旧設定やSpec metadataから英語・日本語を決められません。自動適用前に成果物言語を
選択してください。

### MIGRATE_SPEC_CONVERSION_REQUIRED {#migrate-spec-conversion-required}

旧Specがあります。active Milestone、Requirement対応、Gate evidenceを推測せず、
agent-assisted手順で成果物を変換してからCLI検証へ戻ります。

### MIGRATE_ACTIVE_SCOPE_AMBIGUOUS {#migrate-active-scope-ambiguous}

進行中に見える旧Specが複数あるものの、それらを1つのactive milestoneとして
扱ってよい根拠がありません。旧roadmap、依存関係、いま進めようとしている作業を
調べたうえで、範囲をあなたに確認します。

### MIGRATE_DESIGN_TRACEABILITY_REQUIRED {#migrate-design-traceability-required}

旧Designからは、SpecBindが求めるRequirement対応を機械的に組み立てきれません。
RequirementsとDesignを読み、各Design成果物のFront Matterと本文markerが同じ集合を
指すように直してから、CLIで検証します。

### MIGRATE_LANGUAGE_MIXED {#migrate-language-mixed}

旧Specの成果物の言語が混在しています。SpecBindは成果物の言語をプロジェクトごとに
1つへ揃えるため、あなたが言語と翻訳範囲を決めるまで停止します。

### MIGRATE_LEGACY_INSTRUCTIONS_AMBIGUOUS {#migrate-legacy-instructions-ambiguous}

`AGENTS.md`または`CLAUDE.md`にある旧案内が、既知のブロックと完全には一致しません。
`kiro`という語が入っていることだけを根拠に削除せず、まわりのプロジェクト所有の
指示を保持したうえで、対象範囲をあなたに確認します。

### MIGRATE_RULE_REVIEW_REQUIRED / MIGRATE_TEMPLATE_REVIEW_REQUIRED {#migrate-rule-review-required}

旧ruleまたはtemplateがあります。現在のSpecBind既定値との差分を、プロジェクト所有の
方針またはoverrideとして残すべきか確認し、手順を丸ごとコピーしません。

### MIGRATE_STEERING_REVIEW_REQUIRED {#migrate-steering-review-required}

旧steering文書があります。文書の責務と安定した`artifact_id`を決め、SpecBind
Steeringとして検証してください。

### MIGRATE_SPEC_DIRECTORY_INVALID / MIGRATE_SPEC_ID_INVALID {#migrate-spec-path-invalid}

旧Specのパスが通常ディレクトリではないか、canonical kebab-case IDではありません。
リンクをたどらず、意図したSpec IDと配置を確認してください。

### MIGRATE_SPEC_METADATA_MISSING / MIGRATE_SPEC_STATE_INVALID {#migrate-spec-state-invalid}

`spec.json`がないか、phase・generated・approvedの組合せが旧cc-sddの状態として
成立しません。成果物と履歴から状態を調べ、証明できないGate evidenceは作りません。

### MIGRATE_LEGACY_AGENT_ASSET_INVALID / MIGRATE_LEGACY_AGENT_ASSET_UNKNOWN / MIGRATE_LEGACY_CONTENT_UNSUPPORTED {#migrate-legacy-content-unsupported}

既知の旧agent資産が通常ディレクトリではない、未知の`kiro-*`資産がある、または
`.kiro`直下に未対応の内容があります。対象を個別に調べ、変換するか意図的に
移行しないかをresolutionへ記録します。最終cutover後のsourceはGit履歴に残ります。

### MIGRATE_RESOLUTION_STALE / MIGRATE_RESOLUTION_STATE_INVALID {#migrate-resolution-stale}

受理済みの移行判断に対応するsource、target、finding、または選択済みinstallが変わったか、
CLI所有のstateが壊れています。stateを手編集せず、現在のfindingをもう一度レビューして、
新しい外部候補を`--accept-resolution`で受理し直してください。

### MIGRATION_CLEANUP_TARGET_UNTRACKED / MIGRATION_CLEANUP_TARGET_UNSAFE {#migration-cleanup-target-unsafe}

最終cutoverの対象にGit未追跡またはignored file、link、reparse point、不正な対象が
含まれています。必要な内容をcommitまたはlegacy root外へ移動し、cleanな状態で
やり直してください。CLIは回復できないファイルを削除しません。

---

[移行ガイド入口](../../guide/migration/cc-sdd.md) | [English](../../guide/en/migrate-from-cc-sdd.md)
