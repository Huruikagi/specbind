# 0209: Project shared Contract without dedicated Specs

Status: Accepted

共有資源を専用Specなしで管理し、既存Contract graphとMilestone reviewに統合する。
本Decisionは0155・0156・0190のSpec限定モデルと、0102・0108のDirect条件を以下の範囲で拡張する。
既存v1成果物と共有機能未使用のworkflowは維持する。

## 解決する問題

`locales/ja.json`と`locales/en.json`には、複数の機能の文言が集まる。
ファイル全体を一つの機能Specに所有させると、他機能の文言追加までそのSpecの変更に
なりやすい。一方、翻訳ファイルを管理するだけのSpecを作ると、独立した要求がないのに
Requirements、Design、Tasks、Gateを維持することになる。

Steeringに共有ルールを書く方法は軽いが、`contract owners`から発見できず、意図した
共有と宣言漏れが区別できない。解決したいのは、この機械的に見つかる管理境界の欠落である。

## 採用する分担

プロジェクトに任意の共有Contractを一つ持たせる。共有ContractはSpecではなく、共有資源の
識別、パス、変更時の約束を保持する。独自のRequirements、Design、Tasks、完了状態は持たない。

| 軸 | 意味 | 翻訳の例 |
| --- | --- | --- |
| 永続的な管理境界 | どの宣言を読めば変更条件が分かるか | 共有Contractの`translations` |
| 変更の実施主体 | 今回の変更をどのwork itemで実施するか | 検索SpecのTask、またはDirect |
| 振る舞いの責任 | その機能の正しさを誰が定義するか | 検索Spec、言語切替Specなど |

共有Contractの資源をSpecのTaskが変更しても、ファイル全体の所有権は移転しない。
Taskは自身のRequirementsを実現するために共有資源を利用する。共有資源の宣言を
Requirement IDの代用にしたり、共有資源のためだけにダミーRequirementを作ったりしない。

「Spec管理外」は名称に使わない。SpecのTaskからも利用され、SpecBindの検査・レビュー対象
でもあるためである。Steeringは背景や全体方針を説明し、変更可否を左右する約束は共有Contractを
正本にする。同じ規則を両方に複製しない。

## 最小の成果物モデル

固定パスは`<specDir>/specs/shared-contract.yaml`。各Specのディレクトリと同じ`specs/`
名前空間に置くが、共有Contract自体はSpecディレクトリではない。`.specbind`をハードコードしない。
CLIのSpec discoveryは、この予約ファイルをSpec候補から除外する。
ファイル不在は未利用として有効、存在する不正なファイルはエラーとする。
インストール時に空ファイルを全プロジェクトへ作成しない。
旧`<specDir>/shared-contract.yaml`は1.5.0の初期実装位置として公開されたため、同一メジャーの
forward-upgrade互換性のため読み取り対象として維持する。新規作成と案内は正規パスを使う。
旧パスから正規パスへの内容を保った移動だけではContractの意味差分としない。両方に存在する
場合は正本を推測せず、曖昧な入力として停止する。

初期モデルは`shared-contract/v1`として、次の形とする。

```yaml
schema_version: 1
resources:
  - id: translations
    description: 全機能が利用する日英の翻訳カタログ。
    paths:
      - locales/ja.json
      - locales/en.json
    change_policy: >-
      各機能は自身の名前空間のキーを両言語で追加・更新できる。
      共通キーの変更・削除では既存利用箇所への影響を確認する。
    invariants:
      - 両言語のキー集合が一致する。
      - 対応するキーの補間変数が一致する。
```

`resources`は必須配列で、空配列を許容する。資源IDは既存Contractと同じcanonical ID規則で
ファイル内一意とする。各資源の`description`、`change_policy`は非空、`paths`は非空・重複なし、
`invariants`は必須配列で空を許容し、各要素は非空とする。未知フィールドは拒否する。
パスは既存File Ownershipと同じproject-root相対のexact pathまたは末尾`/**`のみを許容する。

一つのresourceを参照・変更検出の単位にする。初版ではinvariantごとのID、JSON Pointer、
言語別の特別なSchema、所有者ユーザー、書込みACL、検証コマンド文字列を追加しない。
CLIは自然言語の規則を実行しない。実際のキー一致検証などはプロジェクトの検証手段で実施する。
検証方法がまだない場合も、架空のコマンドを記載しない。

同じ意味の資源のパス移動ではIDを維持し、意味の置換では新しいIDにする。
Schemaと参照はresource全体にかかるため、規則変更時はその利用者をレビュー候補として返す。

### 共有Contractを小さく保つ制約

初版は一つのファイルと複数resourceに限定する。共有ContractからSpecへの`consumes`、
共有Contract同士の依存、一般的なサービスの`exports`は持たせない。
共有Contractをもう一つのSpec体系に成長させないためである。

言語切替、フォールバック、翻訳配信など独立した振る舞いの要求があれば、それはSpecが所有する。
共有Contractは、その振る舞いを要求・設計なしで収容する場所ではない。
初期用途は翻訳カタログ、共通スタイル資源など、パスで識別できる共同利用の資源とする。

## Specからの利用と所有権検査

Spec Contractからの参照は既存`consumes`を拡張する。例として、`contract/v2`では
既存のSpecターゲットに加え、次を許容する。

```yaml
consumes:
  - id: translation-catalog
    target: { shared: true, section: resources, id: translations }
    description: search名前空間の文言を提供・利用する。
```

`spec`ターゲットと`shared: true`ターゲットは排他的な型にする。`shared`という仮想Specを
登録したり、ユーザーが作成可能なSpec IDを予約語として転用したりしない。
`consumes`は依存・利用の宣言であり、排他的な書込み権限ではない。
継続して資源を利用するSpecは参照を宣言する。Directには参照を維持するためだけのContractを作らない。

`contract owners <path>`は同じ完全なContract集合から、次を区別して返す。

| 結果 | 意味 |
| --- | --- |
| Spec declaration | 既存SpecのFile Ownershipに一致 |
| Shared declaration | 共有Contractのresourceに一致 |
| None | どちらにも宣言なし。自由な変更を保証しない |

共有の結果には資源ID、宣言パス、読み出し先を含める。規則の全文は専用readで取得する。
複数Specが一つの共有resourceをconsumeすることは、所有権の重複ではない。
共有resourceとSpecのFile Ownershipが同じパスを宣言する場合、または二つのresourceが重なる
場合は、すべてを返して警告する。具体的なパスを優先して広い宣言を黙って無視しない。
初版では階層所有・上書き規則を追加せず、レビューで解消または許容理由を明示する。

追加する読み取り面は次のとおり。

```text
specbind contract shared read
specbind contract shared consumers [resource-id]
```

`contract graph`と`contract dependencies <spec>`にも共有ターゲットを出す。
`check contracts`は共有ファイルの構造、参照切れ、重複候補を含める。
読み取りにMilestoneは不要であり、無効な共有ファイルを無視して部分的な成功を返さない。
参照切れはエラー、重複は既存と同じく意味判断を伴う警告とする。
共有ファイル不在時の`shared read`は未利用を明示し、存在するがresource IDが不明な照会とは区別する。
グラフは宣言済みの利用者候補を返すもので、実装中の未宣言利用や外部利用を網羅したとは主張しない。

## 変更ルーティング

共有パスへの一致はDiscoveryへの入口とするが、特定Specへの帰属は決めない。
既存のpending work itemへの一致を先に確認する順序は維持する。
共有Contractそのものの作成・変更・削除も必ず入口にする。

| 依頼 | ルート | 共有Contractの更新 |
| --- | --- | --- |
| 検索機能に`search.title`を追加 | 検索SpecのTasks | 既存規則内なら不要 |
| 共有カタログの誤字修正。Specの保証は変わらない | Discoveryで確認したDirect | 不要 |
| 誤字に見えるがSpecの要求する表示内容が変わる | 既存Spec更新 | 必要性は変更内容で判断 |
| カタログの共有規則だけを変更 | 後述の共有Contract変更を伴うDirect | 必要 |
| 命名規則の変更で検索等のDesignも変わる | 影響Spec更新と必要なDirectを同じMilestoneへ | 必要 |
| 新しい言語切替機能を導入 | 既存Spec更新または新Spec | 資源側の約束が変わる場合のみ |
| 宣言のない無関係なREADME修正 | 既存の入口判定に従う | 不要 |

共有の誤字修正も、共有パスを明示的な管理境界にした以上、通常作業として黙って迂回しない。
軽量化するのは専用SpecとそのGateであり、管理された変更の追跡ではない。
Directかどうかは変更量ではなく、SpecのRequirements・Design・Contract変更の必要性で判定する。

## 共有の約束自体を変更するルート

現行DirectはContract変更を許さず、Direct-only MilestoneはContract Reviewを受け付けない。
そのままでは共有Contractを維持するために結局ダミーSpecが必要になる。

第三のwork item種別を追加せず、Directの条件を限定的に拡張する。
SpecのRequirements・Design・Contractを変えない条件は維持し、共有Contractの変更だけを
宣言したDirectを許容する。そのDirectはContract Reviewを免除されない。

RoadmapのDirect項目に任意の`shared_contract_changes`資源ID配列を追加する。
欠落・空配列は従来のDirectである。空manifestの作成・削除だけは`*`を使う。この値は資源の一括変更権限ではない。
配列は新設・変更・削除する資源のIDを指すため、
現行集合への参照だけでは検証しない。baselineとcurrentの和集合、および作成予定を考慮する。
操作の内容と必要な検証はRoadmap本文に記載し、別のBriefやTasksは作らない。

この宣言は予定であって、変更なしの証拠にはしない。共有Contractにbaselineからの意味差分が
あれば、宣言がなくてもレビュー必須とし、未スコープの変更として受入れを止める。
Spec作業に付随する共有Contract変更にも、約束を準備する担当Directを一つ割り当てる。
各Specの通常の文言追加にはこのDirectを要求しない。

完了済みDirectの共有変更義務は後から置き換えない。同じ資源の追加変更は新しいDirectに
記録できるため、資源IDの重複禁止は一つのDirect内に限定する。複数項目間の実施順序は
必要に応じて既存の`depends_on`で表す。

### 準備・レビュー・実施の順序

1. Discoveryが共有変更の範囲と担当Directを計画する。baselineは変更前に確定する。
2. Planが共有Contractの提案内容を作成する。Direct-onlyでもこの準備を可能にする。
   新しいSpec Gateは作らず、ここでは資源本体の変更やDirect完了を行わない。
3. 参加Specがあれば、そのDesignは提案された共有の約束を前提に整える。
4. 既存のMilestone Contract Reviewで、共有Contract、全Spec Contract、baseline差分、
   共有変更のRoadmapスコープを一緒に評価する。Direct-onlyでも受入れ可能にする。
5. 受入れ後、SpecのTasks作成・実施、Directの資源変更・検証を進める。
6. 完了とリリースでレビューのfreshnessおよび通常の実装検証を確認する。

Planの準備には独立した「共有Contract承認済み」状態を追加しない。受入れの正本は既存の
Milestone reviewである。Driveは共有変更がありレビュー未受入れなら、Planで提案を整え、
Reviewへ進める一つの準備経路を使う。再入時は既存案とレビューfindingから再開し、同じ案を
毎回再生成しない。レビューがfreshになれば通常の実施へ進む。
単なるファイル存在を準備完了の証拠にせず、レビューでRoadmapの変更意図との一致を確認する。

共有規則の準備をDirectの「実装完了」に依存させない。レビュー前の提案準備と、レビュー後の
実施完了を分離し、SpecのDesignとの循環を避ける。実装順が必要な場合だけ、通常のRoadmap
依存関係を使う。Contractへの参照そのものから実装順を自動生成しない。

影響先Specに変更が必要なら、レビューでそのSpecをスコープに追加して正規の変更手順を踏む。
参照しているという理由だけですべてのSpecを再計画せず、影響がない理由をレビューに残す。
反対に、Spec側変更が必要なのに共有Directで代行してはならない。

## Review、freshness、完了

レビュー要否は次の条件に一般化する。

```text
Spec-backed参加あり
OR shared_contract_changesの予定あり
OR baselineとcurrentの共有Contractに意味差分あり
```

従来のDirect-onlyかつ共有変更なしなら、従来どおりreviewはnot requiredとする。
共有ファイルが不正、baselineが読めない、変更の帰属が不明な場合をnot requiredへ落とさない。
共有Contractの新設と削除も比較対象にする。baseline不在は初回導入として明示的に扱い、
Git読み取り失敗と混同しない。

レビュー入力は既存の全Spec Contractに共有Contract全体と、共有変更を宣言したDirectの
ID・変更意図・資源ID・依存関係を追加する。共有の変更意図を記載したRoadmap本文も、その
経路では入力に含める。Directの完了statusだけでreviewをstaleにしない。
資源配列・pathsの順序、YAML整形では意味fingerprintを変えない。
説明、規則、invariantsの値、資源集合、存在・不在の変化は検出する。
翻訳JSONの通常の内容変更をContractのfingerprintに含めない。実装の正しさはTask/Directの
検証と完了revisionで扱う。

共有Contractの意味変更後は、以前の受入れを再利用しない。必要なレビューがmissing・stale・
invalidなら、影響する実施への進行、共有変更Directの完了、Milestoneの完了・releaseを止める。
完了済みDirectの早期returnも、この検査を迂回しないこと。読み取りが勝手にcompletedを
pendingへ書き戻すことはなく、追加実施が必要なら既存の再計画手順で明示的に調整する。

既存のSpec Design gateには、共有Contract全体を重ねてfingerprintしない。
既存の外部Spec Contractと同様にMilestone reviewで変更を検出し、意味的にDesignへ影響する
ときに対象Specを戻す。全SpecのGateを一律に無効化する実装を避ける。

## 導入、移管、削除

既存プロジェクトへの導入では、Steeringや実装から共有境界を提案し、明示されたスコープで
共有Contractを作る。未宣言ファイルを自動的に全部共有へ登録しない。
Spec所有から共有へ移す場合は、そのSpecのContractと利用側参照も同じ変更で調整し、
元Specの要求する振る舞いまで消さない。参照が残る削除は検査エラーとなる。

release finalizeは共有Contractを残す。変更履歴はMilestoneの記録とGitに保持し、
共有Contract専用のlogや完了状態を追加しない。reverse adoptionでも共有ファイルを
収容するだけのreverse Specは作らず、既存のbaseline確立とレビューに共有宣言を含める。
install/updateはプロジェクト所有の内容を上書きしない。uninstallのdurable knowledge分類、
Git guards、review後のcleanup対象にもこの成果物を明示的に組み込む。

## 互換性と実装範囲

`contract/v1`の意味と旧ターゲット構造を保持し、`v2`と共有artifactのwire modelを追加する。
v1のみのプロジェクトを強制変換しない。共有参照を追加するSpecだけが新表現を必要とする。
既存のカスタムテンプレートも自動上書きしない。

共有機能未使用の既存review recordは、従来の入力集合で引き続き読めるようにする。
初回導入では共有入力の追加によってstaleにし、削除時も入力集合の変化を検出する。
共有を含む新しい受入れには新selectorを保存し、共有不在へ戻った状態でもその履歴を
無視して旧recordとして受け入れない。新表現の導入と既存record互換のfixtureを用意する。

主な変更箇所は次のとおり。

| 領域 | 必要な変更 |
| --- | --- |
| Schema・domain・fingerprint | 共有artifact、Contract target、Roadmapの任意宣言、正規化 |
| Contract read model・CLI | 共有ノード、owners、consumers、read、参照・重複検査 |
| Review・完了・release | Direct-only除外の一般化、入力集合、再検査、保持・cleanup |
| Discovery・Plan・Drive・Implement | 共有境界の入口、準備経路、Taskからの利用、Direct完了条件 |
| 導入・更新・adoption・uninstall | 任意導入と永続成果物の保護 |
| テンプレート・文書・検証 | 日英の説明と両エージェントの同一契約、fixtureとforward test |

wire/read model、review/完了、Skill/導入を一体で提供する。
ownersだけが共有を認識し、Direct完了が無検査で通る中間状態を完成扱いしない。

## トレードオフ

「単一共有Contract、資源単位、共有変更を宣言するDirect、既存reviewの再利用」を採用する。
専用Spec案より軽く、Steering単独案より機械的に発見できる。ただし共有の誤字修正も
Discoveryを通すこと、Directの既存の意味を限定拡張することは明示的なプロダクト判断になる。

第三のwork item種別を作ればDirectの意味は保てるが、スケジューラ、完了、release、表示に
新しい分岐が必要になる。初版では採らない。共有変更を無条件のDirectにする案は、
利用者への破壊的変更を受入れなしで通すため採らない。

Scope入力は`sharedContractChanges`、永続Roadmapは`shared_contract_changes`を使う。
Direct-onlyを含む共有準備は`sb-plan --shared`が所有し、Driveのtyped handlerもこのSkillへ渡す。
共有reviewの入力selectorは`shared-contract`、共有変更のRoadmap本文は`roadmap#shared-body`とする。
共有Contractの新設・削除もbaselineとの比較で検出する。

## 検証シナリオ

1. 共有機能未使用のプロジェクトで既存Contract/Direct/review recordが従来どおり動く。
2. 翻訳パスのownersが共有資源を返し、宣言なし・不正ファイルと区別される。
3. 検索Specが既存規則内のキーを追加し、共有専用Specや共有変更Directを作らず完了できる。
4. 同じ翻訳ファイルを二つのSpecがconsumeしても所有権重複と判定しない。
5. Spec所有との実際の重複は隠さず、JSONキー境界をCLIが検証したと主張しない。
6. 共有規則だけのDirect-only変更が、提案準備、review、実施、完了、releaseまで進む。
7. 未宣言の共有Contract差分、削除、初回作成を検出し、Direct-onlyでもreviewを迂回できない。
8. 共有規則変更が既存Specに影響するとき、そのSpecを戻す。無影響consumerは自動で戻さない。
9. review後の共有規則変更はstaleとなり、通常の翻訳値追加ではstaleにならない。
10. Direct完了後の規則変更でもreleaseで検出し、completedの早期returnで迂回しない。
11. 共有提案準備がSpec実装完了を待つ循環を作らず、Drive再入で提案を再生成し続けない。
12. 初回導入、Specからの移管、参照を伴う削除、finalize、更新・uninstallが成果物を正しく扱う。

CLIの構造検証・lifecycle fixtureに加え、Skillを実装する段階で翻訳例のfresh-agent forward testを
行う。実装と同じbuildのfixtureで測定する。

## 現行契約との対応

- [Cross-spec contracts](../cross-spec-contracts.md): 疎なFile Ownership、Taskのwrite scope、Directの現行条件。
- [Decision 0102](./0102-workflow-entry-condition.md): workflowの入口と通常作業・Directの区別。
- [Decision 0155](./0155-versioned-yaml-contract-artifact.md): 厳密なContract wireと意味fingerprint。
- [Decision 0156](./0156-derived-contract-graph-reads.md): 完全なgraph、参照単位のprojection、意味判断の境界。
- [Decision 0190](./0190-file-ownership-path-projection.md): ownersの候補取得と宣言なしの意味。
- [Decision 0144](./0144-major-version-compatibility-and-migration.md): 既存成果物のforward-upgrade互換。

`cross_spec_review`のDirect-only受入れ制限、入力解決、freshnessと、
`completion/direct.rs`の完了ガードを一体で変更する。共有変更ありのDirect-only releaseでは
reviewをアーカイブし、通常のDirect-only releaseは従来の成果物集合を維持する。
