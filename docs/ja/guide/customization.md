# プロジェクトに合わせてカスタマイズする

SpecBindのライフサイクルと検証はそのままに、成果物の書き方、プロジェクト固有の
判断基準、運用手順、役割ごとに使うモデルを、プロジェクトに合わせて調整できます。

このページでは、何を変えたいときにどこを編集するのかをまとめます。通常は場所を
先に調べて手作業する必要はありません。コーディングエージェントへ設定したい結果を伝えると、
`sb-configure`が現在値を調べ、必要な変更、検証、アフターケアまで完遂します。

```sh
specbind configuration show
```

このコマンドは、エージェントと役割設定、テンプレート、ルール、アダプター、Steeringの現在値を
読み取り専用で要約します。`current-default`は現在の組み込み既定値と完全一致するという
機械的な事実、`project-content`は異なるという事実だけを表し、意図的に設定済みかどうかは
断定しません。

## 変更したいことから選ぶ

| 変更したいこと | 編集する場所 | 主な確認方法 |
| --- | --- | --- |
| RequirementsやDesignなどの構成、見出し、例 | `.specbind/settings/templates/` | `specbind template list`、`specbind template read` |
| Requirements、Design、Contract、Tasks、Steeringの書き方や判断基準 | `.specbind/settings/rules/` | `specbind rule list`、`specbind rule read` |
| リリース、Git、保留した指摘の届け先、実装完了時に追加する検証手順 | `.specbind/settings/adapters/` | `specbind adapter list`、`specbind adapter read` |
| プロジェクトについてエージェントが長く参照する知識 | `.specbind/steering/` | `specbind steering list`、`specbind steering read` |
| Specの置き場所、成果物の言語、使うエージェント | `.specbind.json`と`specbind install`のオプション | `specbind install --dry-run ...` |
| 役割ごとのモデルと推論の深さ | `.specbind.json`の`agentRoles` | 設定後に`specbind install --dry-run` |

このページでは、Specの置き場所が既定値の`.specbind`である前提でパスを書きます。
`.specbind.json`の`specDir`を変えている場合は、その値に読み替えてください。

## 成果物テンプレート

テンプレートは、新しい成果物の構成と初期内容を決めるひな形です。見出し、節の
分け方、例、テンプレート内の`specbind:instruction`コメントを調整できます。

初回のインストールでプロジェクト側にコピーされるのは、構成をプロジェクトで所有する
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

独自のDesignテンプレートを追加する場合は、同じセレクターの分類と、
`conditional`なら適用条件もこのRuleに追加してください。テンプレートとの
対応が欠落、重複、または不明な場合、ルールの読み取りは安全側に停止します。

Roadmapテンプレートは、マイルストーン全体の変更要求、境界、分解判断、依存関係の
理由を書く本文だけをカスタマイズします。`milestone_id`、基準、対象リリース、
作業項目はCLIが所有するため、このテンプレートには書けません。

Brief、Research、Contract、Implementation NotesのSpecテンプレートと、Steeringの
テンプレートもCLIに埋め込んであります。変更したいプロジェクトだけが、CLIの一覧に
出てくる`template_path`へコピーして上書きしてください。

```sh
specbind template list spec
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

結果にはテンプレートの`Source`と、設定済みのSpecルートを含むプロジェクトルート相対の
`Project path`が含まれます。ファイル操作では`Project path`をそのまま使用します。

成果物を初めて作るときは、元のテンプレートとその`create`指示をエージェントが
読み、成果物として生成します。Markdown本文では、プロジェクトが任意の名前付き生成出力を
`{{名前}}`の形式で参照できます。名前は空ではなく、空白と波括弧を含まない
必要がありますが、日本語を含むUnicode名を使用できます。

```sh
specbind template read spec <selector>
```

異なる名前ごとに、対応する`create output=<名前>`指示がちょうど1つと、参照が
1つ以上必要です。エージェントは指示を1回実行し、短い文字列またはMarkdown断片
全体を生成できます。同名の参照はすべてその同じ出力で置換します。CLIはこの対応だけを
検証し、内容の生成や比較は行いません。

```markdown
<!-- specbind:instruction create output=components
新設または変更する責任境界ごとにH3小節を1つ生成する。
各小節に実際のコンポーネント名を付け、その責任を記載する。
-->

{{components}}
```

`components`の出力は、それぞれ異なる複数のH3小節を含められます。それでも生成結果全体が
1つのMarkdown断片です。既定テンプレートの`spec`と`artifact_id`も特別な組み込み出力では
ありません。それぞれの`create output`指示が、現在の作成時の文脈やリテラルな
Front Matterから内容を生成するようエージェントへ指示します。

出力宣言の欠落、重複、未使用、`create`以外での宣言、Front Matterでの参照はテンプレート
診断になります。未展開の参照が残った成果物も無効です。`template read`は出力参照と
指示を含む元のテンプレートをバイト単位でそのまま返します。

read結果は未記入のひな形であり、そのまま有効な成果物とは限りません。既定の
Requirementsは実際のRequirementとAcceptance Criterionを書くまで検証に失敗します。
Brief、Research、Implementation Notesも、見出しやコメントだけでは有効になりません。
作成指示に従って実内容を埋め、`create`コメントを除いてから有効な成果物として
検証・保存してください。

テンプレートを変えても、すでにある成果物は書き換わりません。変更後に新しく作る
成果物から、新しいテンプレートが使われます。

`sb-configure`はテンプレート変更後に、既存成果物も合わせるかを確認します。
同意した時点では候補と影響のプレビューだけを作り、`format-only`、`instruction-update`、
`structural`、`semantic`、`conflict`に分類します。実際の書き換えは別に確認し、意味を変える
変更はRequirementsやDesignなど、その成果物を所有するスキルへ引き渡します。Gate、
完了記録、リリース済みアーカイブ、CLI所有の構造化状態は、テンプレートに合わせるという理由で
直接書き換えません。

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

### テンプレート指示とルールを使い分ける

どこに指示を書くか迷ったときは、その指示が1つの成果物に閉じるか、複数の作業で
共有されるかで判断します。

| 指示や基準の性質 | 置く場所 |
| --- | --- |
| そのテンプレートから作る成果物だけの作成、更新、読み方 | テンプレート内の`specbind:instruction` |
| 複数の成果物、エージェント、作成、検証、レビューで共有するプロジェクト判断基準 | `settings/rules/` |
| プロジェクトが弱められない製品共通の意味や品質基準 | 製品protocol（カスタマイズ対象外） |
| 作業の順序、分岐、停止条件 | 製品管理のスキル（カスタマイズ対象外） |
| 必須構造、状態遷移、機械的な整合性 | CLI（カスタマイズ対象外） |

テンプレートの`maintain`と`consume`は、初回作成時に成果物へコピーされ、その後は
各成果物が所有します。テンプレートを更新しても、既存成果物の指示は変わりません。
一方、共有ルールはスキルが作成時だけでなく検証やレビュー時にも読みます。既存成果物を
含めて次の作業から同じ方針を適用したい場合や、Design作成とContract reviewのように
複数の作業で同じ判断基準を使う場合は、独立した共有ルールに置きます。

逆に、特定の見出しをどう埋めるか、その成果物のIDを更新時にどう保つかなど、1つの
成果物だけに必要な指示は共有ルールへ重複させず、テンプレートの指示に置きます。

| ファイル | 書けること |
| --- | --- |
| `ears-format.md` | RequirementsのEARS表現、主語の立て方、テストしやすさの好み |
| `design-principles.md` | アーキテクチャ、インターフェース、データ、エラー処理、記述の細かさ |
| `design-template-selection.md` | 各Designテンプレートを必須、条件付き、無効のどれにするか |
| `contract-principles.md` | 所有境界、外部へ公開する接点、互換性、依存の向きに関する方針 |
| `tasks-generation.md` | Taskの大きさ、分割の仕方、テスト作業の扱い |
| `steering-principles.md` | Steeringに残す知識の粒度、例の書き方、更新の方針 |
| `language-style.md` | 正確な識別子を保ちながら、成果物とスキルの報告を自然な日本語で書く方針 |

v1のスキルが読むのは、この7つのパスだけです。別の名前でルールファイルを足しても
読み込まれません。

初回インストールでは、どの言語でも使う6つの既定ルールに加えて、`--language ja`を
選んだ場合だけ日本語の`language-style.md`を作成します。このルールは任意で、
プロジェクトが所有します。すべての製品スキルが成果物や報告を書く前に読みますが、
ファイルがなくても設定言語で出力するという製品の契約は変わりません。後から
インストールを実行しても、既存の内容は上書きされません。

```sh
specbind rule list
specbind rule read ears-format --for consume
specbind rule read ears-format --for maintain
```

一覧は既知の7件と、各ファイルがプロジェクトに存在するかを返します。スキルが判断基準
として使うときは`--for consume`、ルール自体を更新するときは`--for maintain`を指定します。
省略すると、指示コメントを含むMarkdownをそのまま返します。ルール内でも
`specbind:instruction maintain`と`consume`を使用できますが、初回作成専用の`create`は
使用できません。ファイルがない場合は`NO_CHANGE RULE_ABSENT`となり、製品protocolは
引き続き適用されます。

また、ルールで弱められないものがあります。成果物の必須構造、Gate、承認、状態の
遷移、スキルの必須手順、CLIの検証です。

## 運用アダプター

アダプターは、プロジェクトごとに違う運用のやり方を、自然言語でエージェントに伝える
場所です。本文は自由に書けます。コードブロックを書いても、自動実行されるフックには
なりません。

| ファイル | 伝える内容 |
| --- | --- |
| `release.md` | リリースの準備、公開、検証、後片付け |
| `git.md` | どの単位で区切るか、ステージング、コミットメッセージ、ブランチ、pushの方針 |
| `deferred.md` | Gateを止めるほどではない指摘を残す先（Issue tracker、wiki、ファイルなど） |
| `validation.md` | Spec全体の実装を最終検証するとき、プロジェクト固有で追加して行う手順 |

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

`release.md`が未設定のままリリースを始めると、Releaseスキルがリポジトリ内のワークフロー、
バージョンマニフェスト、ビルドスクリプト、既存ドキュメントを調べて具体案を提示します。
承認した実行では`release.md`だけを保存・ローカルコミットして停止し、公開は行いません。
この設定変更後は完了記録を再検証してから、改めてリリースします。プロジェクト固有の作業が
本当に不要なら、Front Matterを残して本文を空にすることで明示できます。

`git`にも動作する既定値があります。Discoveryの完了、各Gateの承認、Contractレビューの
受理、各実装Taskの完了、`sb-configure`による1つの設定変更など、スキルが定めた
安全な完了単位ごとに、関係するパスだけを
ローカルコミットします。現在のブランチを使い、既定では`push`や`amend`などの履歴書き換えを
行いません。初回のリリースアダプター設定と、`release finalize`が生成するログ・アーカイブ・
後片付けも、それぞれ公開対象とは別のローカルコミットになります。自動コミットを望まない
場合は`git.md`の本文を空にしてください。ファイルを
削除しても実行時は同じですが、次のインストールで既定値が再作成されます。既存のプロジェクト所有
ファイルは、インストールを再実行しても上書きされません。

実装Taskは計画の順に1件ずつ実行します。1回の依頼で複数Taskを進める場合も、各Taskの
実装、レビュー、CLIへの完了記録が終わった直後に、そのTaskだけのチェックポイントを
作ってから次へ進みます。複数Taskの完了を最後の1コミットへまとめるのは既定動作では
ありません。Spec全体の完了記録は、Taskの実装コミットとは別です。

アダプターはあくまで方針であり、広い権限を与えるものではありません。変更を伴うスキルの依頼は
既定のローカルチェックポイントまでを含みますが、`git.md`に`push`の方針を書いても、それだけで
エージェントが`push`できるようにはなりません。`push`にはあなたの依頼と実行環境の権限が別途
必要です。

`validation.md`は、プロジェクトが追加の最終検証手順を定めるまでは未設定のひな形です。
`sb-configure`は既存のスクリプト、CI、実行手順、fixture、ブラウザや実機の設定、接続済み
ツールとの連携を調べ、ひな形を置き換える全文または現在の方針への変更案を提示できます。
有効な手順は、必須のcompletion-verification protocolとリポジトリの標準チェックに追加
されます。置き換えたり弱めたりすることはできません。不一致が確認できた場合は`NO-GO`、
必要な環境、認証、実機、人による確認、ツールなどが利用できず必須手順を実行できない場合は
`MANUAL_VERIFY_REQUIRED`となります。

本文には、コマンド、ブラウザや実機の操作、MCPサーバーなどの接続済みツール、人による
目視確認、準備、合否を判断できる結果、後片付けを記載できます。アダプターだけで認証情報の
利用、外部変更、ソース編集、指摘の修正を行う権限は得られません。本文を空にした場合は、
プロジェクト固有の追加手順なしという意味です。完了記録を受理したあとに変更すると、通常の
プロジェクトrevisionの規則によって以前の完了記録は古くなり、再検証が必要になります。

v1が読むのは上の4つだけです。`settings/adapters/`に好きなファイルを置いて種類を
増やす仕組みではありません。

## Steering

Steeringは、製品の目的、技術方針、構造、テスト方針、セキュリティの考え方など、
これから先の作業でも参照するプロジェクトの知識です。作業中だけのメモや、すぐに
変わる状態は書きません。

`sb-steering`スキルが、現在の一覧を確認したうえで、初期作成、既存文書の同期、
1文書の追加を行います。既定の`product`、`tech`、`structure`という分け方は提案です。
名前を変えても、統合しても、分割しても、使わなくても構いません。

```sh
specbind steering list
specbind steering read <selector> --for consume
```

SteeringはGateの入力ではなく、古くなったかどうかの判定にも使いません。ただし、
完了記録を受理済みのMilestoneの途中で編集すると、完了記録の再検証が必要に
なることがあります。Milestoneを始めてから最初の完了記録までの間か、リリースの
後片付けが終わったあとに更新すると、扱いやすくなります。

## プロジェクトの形を整えるときの推奨順

初回インストール後の見直しや、テンプレートをどう分けるかがプロジェクト全体の
前提に依存する場合は、まずSteeringの初期作成または同期を提案します。次に、確定した
継続的な方針とリポジトリの事実を、現在のRequirements・Designテンプレートと共有Ruleに
照らし合わせます。その責任が既存のテンプレートやRuleで共通して扱えるなら更新し、
複数のSpecで独立した設計判断とトレーサビリティを継続して必要とする場合だけ、Design
テンプレートを追加します。

Steeringが空であること自体は有効な状態です。空だからといって作成せず、欠けている
プロジェクト知識を必要としない明示的で狭いテンプレート変更も止めません。Steeringは
プロジェクトに長く残る事実や方針を記録し、各Specにどの候補を適用するかは引き続き
`design-template-selection` Ruleが決めます。

Web、モバイル、API、インフラといった技術ラベルだけでテンプレートを増やしません。
ユーザーに見えるWeb・モバイルの変更は通常、既存のUI候補で扱えます。APIの互換性や
インフラの方針は、まずSteeringとDesignまたはContractのRuleに置きます。`design/api`や
`design/infrastructure`のような条件付き候補は、その責任に独立した設計の扱いが繰り返し
必要になるときだけ追加し、適用条件にはフレームワークの有無ではなく責任を記述します。

将来のAPIやインフラに触れる依頼でも、現在のSteeringとリポジトリの事実から独立した
継続的責任が確認できない場合、`sb-configure`は既存の面を更新する案と条件付き候補を
追加する案を示して、どちらを意図するか確認します。将来使う技術の名前だけから、新しい
候補を推定して追加しません。

## 1回限りのDesign補足

Designの作成中に、あるSpecだけが独立した所有境界や検証上の関心を持つ継続的な責任を
含み、既存の選択済みDesignでは明確に表せないことがあります。その場合、エージェントは
artifact ID、対応するRequirements、配置先、既存Designへ統合する案を記録したSpecローカルの
補足Designを、現在のDesignドラフトとして作成します。これだけのために確認で止まることはなく、
通常のDesign Gateがレビュー境界のままです。

この1回限りの文書は、設定済みSpecBindルート配下の
`specs/<spec>/design/<artifact_id>.md`に置きます。通常の`SpecBind Design`なので、
トレーサビリティ、検証、フィンガープリント、Design Gateの対象です。プロジェクト全体の
テンプレートや`design-template-selection`は変更しません。同じ責任が別のSpecでも独立して
必要になったときにだけ、エージェントが条件付き候補テンプレートへの昇格を提案します。

## プロジェクト設定と役割別モデル

初回のインストールでは、成果物の言語、使うエージェント、Specの置き場所、ルート指示
ファイルへの案内追加を選べます。

```sh
specbind install --dry-run --agent codex --language ja --spec-dir .specbind --project-instructions
```

`specDir`は初回のインストールで決まり、v1では導入後に変更できません。言語と、選んだ
エージェントは`.specbind.json`に保存されます。あとからエージェントを追加することは
できます。1つのエージェントを外すときは`specbind remove-agent`、連携全体を外すときは
`specbind uninstall`を使います。詳しくは[エージェントの削除とプロジェクトの
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

変更したら、リポジトリに未コミットの変更がない状態でドライランの結果を確認し、インストールを実行し
直してください。`.codex/agents/specbind-*.toml`や`.claude/agents/specbind-*.md`は
直接編集せず、設定から作り直します。

## カスタマイズできないもの

次はSpecBindが管理している製品側の契約です。直接編集しても、サポートされた
カスタマイズにはなりません。

- `.agents/skills/specbind-*/`と`.claude/skills/specbind-*/`のスキル本体
- `.codex/agents/specbind-*.toml`と`.claude/agents/specbind-*.md`の役割定義
- CLIが埋め込むプロトコルとスキーマ
- Gate、承認、フィンガープリント、状態の遷移、必須のトレーサビリティ
- `spec.yaml`、`tasks.yaml`、Roadmapなど、CLIが所有する構造化された状態
- ルート指示ファイルの中の、SpecBind管理ブロック

プロジェクト固有の方針を足したいときは、スキル本体を書き換えず、目的に応じて
テンプレート、ルール、アダプター、Steeringのどれかに置いてください。

## 変更するときの進め方

通常はコーディングエージェントへ目的を伝え、`sb-configure`に次の一連の作業を任せます。

1. `configuration show`と関係する`list`、`read`で現在値を確認する。
2. 目的をテンプレート、ルール、アダプター、Steering、インストール設定の所有面へ分類する。
3. 変更案と影響を示し、必要な確認を得てプロジェクト所有面だけを変更する。
4. インストールの再実行や専用スキルへの委譲を含め、所有する経路で反映する。
5. 機械的な検証を再実行し、必須（`required`）、推奨（`recommended`）、任意（`optional`）に分けてアフターケアを
   完了または明示的に見送る。

Steeringの執筆は`sb-steering`、成果物の意味変更は各成果物のスキルが所有します。
`sb-configure`はそれらへ委譲しても、依頼された設定変更全体の完了確認と報告を
引き続き担当します。有効なGitアダプターが定める狭いローカルチェックポイントは通常の完了手順に
含まれますが、削除、`push`、ブランチ変更、タグ、履歴操作、外部操作、ライフサイクル変更は
別の確認境界です。

インストールされるファイルの全体像は
[現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）で確認できます。

---

[ガイドの入口](../index.md) | [基本概念](./concepts.md) | [現在の成果物一覧](https://huruikagi.github.io/specbind/reference/current-artifact-index/)（英語）
