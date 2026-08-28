# プロジェクトに合わせてカスタマイズする

SpecBindのライフサイクルと検証はそのままに、成果物の書き方、プロジェクト固有の
判断基準、運用手順、役割ごとに使うモデルを、プロジェクトに合わせて調整できます。

このページでは、何を変えたいときにどこを編集するのかをまとめます。

## 変更したいことから選ぶ

| 変更したいこと | 編集する場所 | 主な確認方法 |
| --- | --- | --- |
| RequirementsやDesignなどの構成、見出し、例 | `.specbind/settings/templates/` | `specbind template list`、`specbind template read` |
| Requirements、Design、Contract、Tasks、Steeringの書き方や判断基準 | `.specbind/settings/rules/` | `specbind rule list`、`specbind rule read` |
| リリース、Git、保留した指摘の届け先 | `.specbind/settings/adapters/` | `specbind adapter list`、`specbind adapter read` |
| プロジェクトについてエージェントが長く参照する知識 | `.specbind/steering/` | `specbind steering list`、`specbind steering read` |
| Specの置き場所、成果物の言語、使うエージェント | `.specbind.json`と`specbind install`のオプション | `specbind install --dry-run ...` |
| 役割ごとのモデルと推論の深さ | `.specbind.json`の`agentRoles` | 設定後に`specbind install --dry-run` |

このページでは、Specの置き場所が既定値の`.specbind`である前提でパスを書きます。
`.specbind.json`の`specDir`を変えている場合は、その値に読み替えてください。

## 成果物テンプレート

テンプレートは、新しい成果物の構成と初期内容を決めるひな形です。見出し、節の
分け方、例、テンプレート内の`specbind:instruction`コメントを調整できます。

初回のinstallでプロジェクト側にコピーされるのは、構成をプロジェクトで所有する
次の4つです。

- `settings/templates/specs/requirements.md`
- `settings/templates/specs/design.md`
- `settings/templates/specs/ui.md`
- `settings/templates/roadmap.md`

`design.md` と `ui.md` はいずれもDesignの候補です。候補が存在するだけで
すべてのSpecに生成されるわけではありません。
`settings/rules/design-template-selection.md`が各`design/<artifact_id>`を
`required`、`conditional`、`disabled`のいずれかに分類します。標準設定では
`design/main`は必須、`design/ui`は画面、操作、表示状態、レスポンシブ動作、
アクセシビリティなどのユーザー可視な責任がある場合だけ選択されます。

独自のDesignテンプレートを追加する場合は、同じselectorの分類と、
`conditional`なら適用条件もこのRuleに追加してください。テンプレートとの
対応が欠落、重複、または不明な場合、Ruleの読み取りはfail closedします。

Roadmapテンプレートは、マイルストーン全体の変更要求、境界、分解判断、依存関係の
理由を書く本文だけをカスタマイズします。`milestone_id`、baseline、target release、
work itemはCLIが所有するため、このテンプレートには書けません。

Brief、Research、Contract、Implementation NotesのSpecテンプレートと、Steeringの
テンプレートもCLIに埋め込んであります。変更したいプロジェクトだけが、CLIの一覧に
出てくる`template_path`へコピーして上書きしてください。

```sh
specbind template list spec
specbind template read spec requirements
specbind template read spec requirements
specbind template list steering
specbind template read steering document
specbind template list milestone
specbind template read milestone roadmap
```

特定Specへ新しく配置するパスまで確認する場合は、次の読み取り専用コマンドを使います。

```sh
specbind template resolve spec <spec> <selector>
```

結果にはtemplateの`Source`、SpecBindルート相対の`Target path`、設定済みSpec rootを
含むプロジェクトルート相対の`Project path`が含まれます。ファイル操作では
`Project path`をそのまま使用します。

成果物を初めて作るときは、raw templateとその`create` instructionをエージェントが
読み、materializeします。Markdown本文では、プロジェクトが任意の変数を
`{{変数名}}`の形式で定義できます。変数名は空ではなく、空白と波括弧を含まない
必要がありますが、日本語を含むUnicode名を使用できます。

```sh
specbind template read spec <selector>
```

異なる変数名ごとに、対応する`create` instructionがちょうど1つ必要です。同じ変数は
本文で何度参照しても構いません。エージェントはbindingの指示を1回実行し、すべての
同名参照を同じ値で置換します。CLIは値を持たず、名前の対応だけを検証します。

```markdown
<!-- specbind:instruction create bind=今日の天気
利用可能なMCPで、作成時点の東京の天気を取得する。
-->

{{今日の天気}}の日に作成。
{{今日の天気}}に合わせた注意事項を記載する。
```

既定テンプレートの`spec`と`artifact_id`も特別な組み込み変数ではありません。それぞれの
`create bind` instructionが、現在のauthoring contextやリテラルなFront Matterから値を
取得するようエージェントへ指示します。

変数とbindingの欠落、bindingの重複や未使用、`create`以外でのbinding、Front Matterでの
使用はテンプレート診断になります。未展開の変数が残った成果物も無効です。
`template read`は変数とinstructionを含む元のテンプレートをbyte-exactに返します。

read結果は未記入のひな形であり、そのまま有効な成果物とは限りません。既定の
Requirementsは実際のRequirementとAcceptance Criterionを書くまで検証に失敗します。
Brief、Research、Implementation Notesも、見出しやコメントだけでは有効になりません。
作成指示に従って実内容を埋め、`create`コメントを除いてからlive artifactとして
検証・保存してください。

テンプレートを変えても、すでにある成果物は書き換わりません。変更後に新しく作る
成果物から、新しいテンプレートが使われます。

`specbind:instruction`コメントには、用途を必ず1つ指定します。

```markdown
<!-- specbind:instruction create 初回の識別子を決める。 -->
<!-- specbind:instruction maintain 既存IDを振り直さずに更新する。 -->
<!-- specbind:instruction consume これは補助情報であり権威ではない。 -->
```

- `create`は初回作成時だけ従い、成果物には残しません。
- `maintain`は初回作成時に成果物へコピーし、以後の更新時にも読み、残します。
- `consume`も成果物へコピーし、その成果物を入力として参照するときだけ読みます。

用途なし、または未知の用途はテンプレート診断になります。既存成果物は、作成時に
コピーされた`maintain`と`consume`を自分で所有します。テンプレート側だけを変更しても、
既存成果物の指示は変わりません。

成果物またはSteeringを用途別に読むと、CLIは反対側の永続指示だけを除いて返します。
`--for`を省略した場合は、指示コメントを含む元のMarkdownをそのまま返します。

```sh
specbind artifact read <spec> <selector> --for maintain
specbind artifact read <spec> <selector> --for consume
specbind steering read <selector> --for maintain
specbind steering read <selector> --for consume
```

!!! warning
    `type`、`artifact_id`、必須の識別子や対応関係など、CLIが読み取る構造は残して
    ください。自由に変えられるのは、この機械可読な部分を保った範囲だけです。

## 共有ルール

共有ルールは、複数のエージェントが共通で参照する、プロジェクト固有の執筆方針と
判断基準です。内容は強めても、緩めても、書き換えても、削除しても構いません。

### template instructionとruleを使い分ける

どこに指示を書くか迷ったときは、その指示が1つの成果物に閉じるか、複数の作業で
共有されるかで判断します。

| 指示や基準の性質 | 置く場所 |
| --- | --- |
| そのテンプレートから作る成果物だけの作成、更新、読み方 | テンプレート内の`specbind:instruction` |
| 複数の成果物、エージェント、作成、検証、レビューで共有するプロジェクト判断基準 | `settings/rules/` |
| プロジェクトが弱められない製品共通の意味や品質基準 | 製品protocol（カスタマイズ対象外） |
| 作業の順序、分岐、停止条件 | 製品管理のSkill（カスタマイズ対象外） |
| 必須構造、状態遷移、機械的な整合性 | CLI（カスタマイズ対象外） |

テンプレートの`maintain`と`consume`は、初回作成時に成果物へコピーされ、その後は
各成果物が所有します。テンプレートを更新しても、既存成果物の指示は変わりません。
一方、共有ルールはSkillが作成時だけでなく検証やレビュー時にも読みます。既存成果物を
含めて次の作業から同じ方針を適用したい場合や、Design作成とContract reviewのように
複数の作業で同じ判断基準を使う場合は、独立した共有ルールに置きます。

逆に、特定の見出しをどう埋めるか、その成果物のIDを更新時にどう保つかなど、1つの
成果物だけに必要な指示は共有ルールへ重複させず、テンプレートのinstructionに置きます。

| ファイル | 書けること |
| --- | --- |
| `ears-format.md` | RequirementsのEARS表現、主語の立て方、テストしやすさの好み |
| `design-principles.md` | アーキテクチャ、インターフェース、データ、エラー処理、記述の細かさ |
| `contract-principles.md` | 所有境界、外部へ公開する接点、互換性、依存の向きに関する方針 |
| `tasks-generation.md` | Taskの大きさ、分割の仕方、テスト作業の扱い |
| `steering-principles.md` | Steeringに残す知識の粒度、例の書き方、更新の方針 |

v1のSkillが読むのは、この5つのパスだけです。別の名前でルールファイルを足しても
読み込まれません。

```sh
specbind rule list
specbind rule read ears-format --for consume
specbind rule read ears-format --for maintain
```

一覧は既知の5件と、各ファイルがプロジェクトに存在するかを返します。Skillが判断基準
として使うときは`--for consume`、ルール自体を更新するときは`--for maintain`を指定します。
省略すると、指示コメントを含むMarkdownをそのまま返します。ルール内でも
`specbind:instruction maintain`と`consume`を使用できますが、初回作成専用の`create`は
使用できません。ファイルがない場合は`NO_CHANGE RULE_ABSENT`となり、製品protocolは
引き続き適用されます。

また、ルールで弱められないものがあります。成果物の必須構造、Gate、承認、状態の
遷移、Skillの必須手順、CLIの検証です。

## 運用adapter

adapterは、プロジェクトごとに違う運用のやり方を、自然言語でエージェントに伝える
場所です。本文は自由に書けます。コードブロックを書いても、自動実行されるフックには
なりません。

| ファイル | 伝える内容 |
| --- | --- |
| `release.md` | リリースの準備、公開、検証、後片付け |
| `git.md` | どの単位で区切るか、ステージング、コミットメッセージ、ブランチ、pushの方針 |
| `deferred.md` | Gateを止めるほどではない指摘を残す先（Issue tracker、wiki、ファイルなど） |

```sh
specbind adapter list
specbind adapter read git
```

一覧の`state`は、ファイルがない`absent`、まだ書き換えられていない`scaffold`、実際に
従う方針がある`active`を区別します。`deferred`には最初から動作する既定値があり、
既定の`specDir`ではOKF準拠の`.specbind/deferred.md`へ全Specの保留指摘を記録します。
この記録は作業キューではなく、人がRoadmapへ採用するまでスコープにはなりません。

未編集のひな形には、完全なHTMLコメント
`<!-- specbind:adapter-scaffold -->`が入っています。このマーカーがある間は本文全体が
方針として扱われません。内容を具体化したらマーカーを削除します。テンプレート用の
`specbind:instruction`はアダプターの状態には影響しません。

`release.md`が未設定のままリリースを始めると、Release Skillがリポジトリ内のworkflow、
version manifest、build script、既存ドキュメントを調べて具体案を提示します。承認した
実行では`release.md`だけを保存・ローカルcommitして停止し、公開は行いません。この設定
変更後はcompletionを再検証してから、改めてリリースします。プロジェクト固有の作業が
本当に不要なら、Front Matterを残して本文を空にすることで明示できます。

`git`にも動作する既定値があります。Discoveryの完了、各Gateの承認、Contract reviewの
受理、各実装Taskの完了など、Skillが定めた安全な完了単位ごとに、関係するパスだけを
ローカルコミットします。現在のブランチを使い、既定ではpushやamendなどの履歴書き換えを
行いません。初回のRelease adapter設定と、`release finalize`が生成するlog・archive・
cleanupも、それぞれ公開対象とは別のローカルcommitになります。自動コミットを望まない
場合は`git.md`の本文を空にしてください。ファイルを
削除しても実行時は同じですが、次のinstallで既定値が再作成されます。既存のプロジェクト所有
ファイルは、installを再実行しても上書きされません。

実装Taskはplanの順に1件ずつ実行します。1回の依頼で複数Taskを進める場合も、各Taskの
実装、レビュー、CLIへの完了記録が終わった直後に、そのTaskだけのチェックポイントを
作ってから次へ進みます。複数Taskの完了を最後の1commitへまとめるのは既定動作では
ありません。Spec全体のcompletion記録は、Taskの実装commitとは別です。

adapterはあくまで方針であり、広い権限を与えるものではありません。変更を伴うSkillの依頼は
既定のローカルチェックポイントまでを含みますが、`git.md`にpushの方針を書いても、それだけで
エージェントがpushできるようにはなりません。pushにはあなたの依頼と実行環境の権限が別途
必要です。

v1が読むのは上の3つだけです。`settings/adapters/`に好きなファイルを置いて種類を
増やす仕組みではありません。

## Steering

Steeringは、製品の目的、技術方針、構造、テスト方針、セキュリティの考え方など、
これから先の作業でも参照するプロジェクトの知識です。作業中だけのメモや、すぐに
変わる状態は書きません。

`specbind-steering` Skillが、現在の一覧を確認したうえで、初期作成、既存文書の同期、
1文書の追加を行います。既定の`product`、`tech`、`structure`という分け方は提案です。
名前を変えても、統合しても、分割しても、使わなくても構いません。

```sh
specbind steering list
specbind steering read <selector> --for consume
```

SteeringはGateの入力ではなく、古くなったかどうかの判定にも使いません。ただし、
completionを受理済みのMilestoneの途中で編集すると、completionの再検証が必要に
なることがあります。Milestoneを始めてから最初のcompletionまでの間か、リリースの
後片付けが終わったあとに更新すると、扱いやすくなります。

## プロジェクト設定と役割別モデル

初回のinstallでは、成果物の言語、使うエージェント、Specの置き場所、ルート指示
ファイルへの案内追加を選べます。

```sh
specbind install --dry-run --agent codex --language ja --spec-dir .specbind --project-instructions
```

`specDir`は初回のinstallで決まり、v1では導入後に変更できません。言語と、選んだ
エージェントは`.specbind.json`に保存されます。あとからエージェントを追加することは
できます。1つのAgentを外すときは`specbind remove-agent`、連携全体を外すときは
`specbind uninstall`を使います。詳しくは[Agentの削除とプロジェクトの
アンインストール](./uninstall.md)を参照してください。

`.agents/skills/`と`AGENTS.md`の共通形式だけを導入する場合は`--agent generic`を
指定します。`generic`には役割定義がないため、`agentRoles`の対象にはできません。

実装、レビュー、調査といった役割ごとに、使うモデルを変えることもできます。
`.specbind.json`の`agentRoles`で上書きしてください。Codexでは、あわせて
`reasoningEffort`も指定できます。

```json
{
  "agentRoles": {
    "codex": {
      "implementer": {
        "model": "gpt-5.6-sol",
        "reasoningEffort": "high"
      }
    },
    "claudeCode": {
      "researcher": {
        "model": "sonnet"
      }
    }
  }
}
```

役割は`planner`、`implementer`、`reviewer`、`debugger`、`researcher`の5つです。
指定しなかった役割には、SpecBindの既定のモデルが使われます。

変更したら、リポジトリがクリーンな状態でdry runの結果を確認し、installを実行し
直してください。`.codex/agents/specbind-*.toml`や`.claude/agents/specbind-*.md`は
直接編集せず、設定から作り直します。

## カスタマイズできないもの

次はSpecBindが管理している製品側の契約です。直接編集しても、サポートされた
カスタマイズにはなりません。

- `.agents/skills/specbind-*/`と`.claude/skills/specbind-*/`のSkill本体
- `.codex/agents/specbind-*.toml`と`.claude/agents/specbind-*.md`の役割定義
- CLIが埋め込むprotocolとschema
- Gate、承認、fingerprint、状態の遷移、必須のトレーサビリティ
- `spec.yaml`、`tasks.yaml`、Roadmapなど、CLIが所有する構造化された状態
- ルート指示ファイルの中の、SpecBind管理ブロック

プロジェクト固有の方針を足したいときは、Skill本体を書き換えず、目的に応じて
テンプレート、ルール、adapter、Steeringのどれかに置いてください。

## 変更するときの進め方

どの面をカスタマイズする場合も、次の順で進めると現在の状態とずれません。

1. 変えたい内容が、テンプレート、ルール、adapter、Steering、プロジェクト設定の
   どれに当たるかを決める。
2. テンプレート、adapter、Steeringは、対応するCLIの`list`と`read`で現在値を
   確認してから編集する。
3. 編集するのはプロジェクト所有のファイルだけにする。
4. 機械可読な構造と、上に挙げた製品側の契約は壊さない。
5. 変更後に、CLIの`read`、dry run、または実際のSkillの実行で結果を確認する。

インストールされるファイルの全体像は
[現在の成果物一覧](../../current-artifact-index.md)で確認できます。

---

[ガイドの入口](../index.md) | [基本概念](./concepts.md) | [現在の成果物一覧](../../current-artifact-index.md)
