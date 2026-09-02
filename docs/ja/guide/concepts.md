# 基本概念

SpecBindは、エージェントにすべてを任せる仕組みでも、どんな変更にも文書作成を
求める仕組みでもありません。意味を判断するエージェントと、状態を検証して記録
するCLIを組み合わせることで、仕様と実装の関係を長く保ちます。

## スキルとCLI

SpecBindでは、責任を次のように分けています。

| 担当 | 主な責任 |
| --- | --- |
| エージェントのスキル | スコープ判断、RequirementsやDesignの作成、レビュー、実装、結果の説明 |
| `specbind` CLI | 構造検証、トレーサビリティ、承認の証拠、進捗、状態遷移、リリース前チェック |
| あなた | スコープの確定、必要な承認、プロジェクト固有の判断、公開結果の確認 |

スキルはCLIが持つ状態を直接書き換えません。逆にCLIは、Requirementsの意味が
正しいか、Designが妥当かといった判断をしません。両方の層を必ず通すことで、
形式だけ整った仕様や、内容はもっともらしいのに状態遷移を飛ばした作業を防ぎます。

## Spec

Specは、プロジェクトが持ち続ける1つの能力、あるいは責任の境界です。

Specは変更のたびに使い捨てる計画書ではありません。あとのMilestoneで同じ能力を
変更するときは、同じSpecのRequirements、Design、Contractを「現在の姿」として
更新します。リリース後もSpecは残り、次の変更の出発点になります。

Specの既定の置き場所は`.specbind/specs/<spec>/`です。`<spec>`には、その責任を
表す短いkebab-caseのIDを使います。

## MilestoneとRoadmap

Milestoneは、1回のリリースとしてまとめて届ける作業の単位です。Specに基づく作業項目
（Spec-backed item）、Direct項目、項目どうしの依存関係、対象リリースをRoadmapに
記録します。

同時に進行できるMilestoneは、プロジェクトごとに1つだけです。現在のRoadmapは
`.specbind/steering/roadmap.md`にあり、状態はCLIが管理します。リリースが完了
すると、Roadmapの内容はリリースアーカイブへ移ります。

いったんMilestoneに入れた作業は、単独なら通常の作業で済むような小さな変更でも、
同じリリース境界の中で追跡します。

## Spec項目とDirect項目

Discoveryは、ワークフローに入ってきた作業を「誰が所有するか」で分類します。

| 種類 | 選ばれる条件 | 持つもの |
| --- | --- | --- |
| 既存Specの更新 | 既存Specが所有する振る舞いや境界を変更する | 更新されたRequirements、Design、Contract、Tasks |
| 新規Spec | プロジェクトに新しい責務を追加し、今後も持ち続ける | 新しいRequirements、Design、Contract、Tasks |
| Direct | どのSpecにも属さず、Requirements、Design、Contractを変えない | Roadmap上の要約と完了状態 |

分類を決めるのは作業量ではなく所有権です。大きな変更でも、既存の1つの責任の中に
収まるなら既存Specの更新です。逆に小さな変更でも、新しい責務が生まれるなら
新規Specになります。

Directとして始めた作業が、実は仕様やContractの変更を必要とすると分かった場合は、
その場で成果物を足さずに、Discoveryへ戻して分類をやり直します。

## DiscoveryのSource Collection

Discoveryには、プロジェクト内のGitで追跡済みのテキストファイルやディレクトリ、
または明示したGitHubリポジトリのMilestoneを、ひとまとまりのSource Collectionとして
渡せます。Discoveryはコレクション全体を棚卸しし、RoadmapにすべてのSource Itemの
振り分けを、各BriefにそのSpecが参照する項目だけを記録します。読めないローカル項目、
アクセスできないGitHub項目、GitHubの不完全なページ送りがあれば、一部だけを使って
進めずに停止します。GitHub Milestoneは`OWNER/REPO`とMilestone番号を別々に指定するか、
厳密な正規URLの`https://github.com/OWNER/REPO/milestone/NUMBER`を指定します。別のURL
形式は受け付けません。openとclosedのIssueを対象にします。コメントとタイムライン
イベントは入力資料ではありません。

Source Collectionは正規の仕様そのものではありません。RequirementsとDesignは
Briefが指定した資料を読み、採用する振る舞いや技術上の結論を自身の成果物へ
書き直します。リモートの入力文脈はDiscovery時に確定し、後続の工程で黙って
再取得しません。元資料を更新した場合は自動同期されないため、必要な範囲を指定して
Discoveryを明示的にやり直します。

## 永続成果物とMilestone固有成果物

Spec項目の成果物には、リリース後も残るものと、Milestoneが進行中の間だけ
存在するものがあります。

| 種類 | 代表例 | ライフサイクル |
| --- | --- | --- |
| 永続 | `spec.yaml`、`requirements.md`、`design.md`、`contract.yaml`、`log.md` | Specの現在の姿と履歴として残る |
| Milestone固有 | `brief.md`、`research.md`、`tasks.yaml` | 進行中の変更を進めるために使い、リリース完了時に片付ける |
| プロジェクト全体 | `steering/roadmap.md`、Steering文書、Contractレビュー | 複数のSpecにまたがるスコープと判断を保持する |

RequirementsとDesignは差分メモではありません。どちらも、現在有効な契約の全体を
表します。以前から変えない記述も、そのまま現在の文書の中に残してください。

## Gateと承認

Requirements、Design、TasksにはそれぞれGateがあります。Gateの承認は単なる
チェック印ではなく、レビューした入力のリビジョンとフィンガープリントに結び付いた証拠です。

そのため、上流の成果物が変わると、影響を受ける下流の承認や、完了を裏付ける記録
（completion evidence）は
無効、または古い状態になります。エージェントが古いDesignやTasksのまま黙って
進んでしまうのを防ぐための仕組みです。

承認には2つの形があります。

- **明示的な承認（explicit）** — あなたがそのGateで内容を確認して承認する
- **委任による承認（delegated）** — `sb-plan`など、名前の付いた1回の実行に対して、承認を
  あらかじめ委任する

委任しても、レビューや検査は省略されません。また、承認済みGateを破棄したり、
Contractレビューを受理したりする権限までは委任されません。

## Contractレビュー

Designには、そのSpecの内部構造だけでなく、外部へ公開する責任、依存、ファイルの
所有境界も含まれます。この部分をContractとして維持します。

Tasksを作る前に、進行中のMilestoneに含まれる全SpecのContractをまとめてレビュー
します。Specが1つしかないMilestoneでも、このレビューは省略しません。所有権の
重複、循環依存、互換性の前提、統合時の抜けを、実装前に見つけるためです。

Contract同士の直接の依存関係は、元の`contract.yaml`を変更せずにCLIから確認できます。

```sh
specbind contract graph
specbind contract dependencies <spec>
specbind contract consumers <spec>
```

`graph`はプロジェクト全体の解決済み参照、`dependencies`は指定したSpecが利用する
提供側、`consumers`は指定したSpecを利用する管理対象の利用側を表示します。
いずれも直接参照の機械的な投影です。到達可能なSpecが実際に変更の影響を受けるか、
SpecBind管理外の利用側が存在するかは、Contractレビューで判断します。

## 無効化とやり直し

承認したあとで前提が変わったときは、影響を受ける中でいちばん手前のGateを、
明示的に無効化します。

```text
Requirementsが変わった -> Requirements Gateからやり直す
Design/Contractが変わった -> Design Gateからやり直す
Tasksだけが変わった -> Tasks Gateからやり直す
```

無効化すると、下流の証拠も消えます。これは失敗ではなく、変わった前提に古い
承認を使わないための、通常のやり直しです。

!!! warning "v1の制限: Requirementの削除"
    確立済みのSpecからRequirementグループやAcceptance Criterionを削除する場合、
    v1では完全な廃止履歴を残せません。既存Requirementの削除が必要になった
    ときは、履歴が欠けたまま進めず、その操作の手前で停止します。既存内容の更新と、
    新しいRequirementの追加は問題なく行えます。

## 通常のライフサイクル

Spec項目は、だいたい次の順で進みます。

```text
Discovery
  -> Requirements
  -> DesignとContract
  -> Design検証
  -> Milestone全体のContractレビュー
  -> Tasks
  -> 実装とTaskレビュー
  -> 実装検証
  -> リリース
```

`sb-plan`は、RequirementsからTasks承認までを進める標準の入口です。Specを指定
するとその1件、`--all`または全Specという明示的な依頼ではMilestone内の全Specを対象に
します。対象を付けずに呼び出すと、作業を始める前にどちらかを確認します。各Gateの承認を
この実行へ委任すれば確認回数を減らせますが、使う成果物、レビュー、CLIの検査は変わりません。
Requirements、Design、Tasksの1フェーズだけを扱う場合も、対象Specとフェーズを
明示して同じ`sb-plan`を使います。
`sb-implement`が実装するのは、1回につき1つのRoadmap項目だけです。
`sb-drive`はMilestone全体から安全に到達可能な所有ワークフローを1つずつ選び、
各委譲後にCLI状態を読み直します。局所的な判断待ちは保留して独立項目を続けますが、
リリースは実行せず、その手前で停止します。

## プロジェクト固有の設定

`.specbind/settings/`以下のテンプレート、ルール、アダプターは、プロジェクトの
持ち物です。初回の導入で既定値を作りますが、そのあと`specbind install`を実行
しても、プロジェクト側の設定を上書きしません。

一方、`.agents/skills/sb-*/`と`.claude/skills/sb-*/`はSpecBind製品
側の持ち物です。`specbind install`を再実行すると、Gitに未コミットの変更がないことを
確認したうえで、現在の埋め込み版へ更新します。スキルファイルを直接編集する
やり方は、サポートしているカスタマイズ方法ではありません。
`.agents/skills/`はCodexと`generic`エージェントで共有されます。`generic`は、共通形式の
スキルと`AGENTS.md`だけを導入し、製品固有のサブエージェント定義は作りません。

どの設定に何を書くか、変更できない製品契約との境界、変更後の確認方法は
[カスタマイズ](./customization.md)にまとめています。

## 次に読む

- [ルートを選ぶ](./getting-started.md)
- [1件ずつ計画・実装する](./implement-step-by-step.md)
- [PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)
- [リリースする](./release.md)
- [カスタマイズ](./customization.md)

---

[ユーザーガイド](../index.md) | [1件ずつ計画・実装する](./implement-step-by-step.md) | [PlanとDriveでMilestoneを進める](./implement-with-plan-and-drive.md)
