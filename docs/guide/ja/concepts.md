# 基本概念

SpecBindは、エージェントにすべてを任せる仕組みでも、どんな変更にも文書作成を
求める仕組みでもありません。意味を判断するエージェントと、状態を検証して記録
するCLIを組み合わせることで、仕様と実装の関係を長く保ちます。

## SkillとCLI

SpecBindでは、責任を次のように分けています。

| 担当 | 主な責任 |
| --- | --- |
| エージェントのSkill | スコープ判断、RequirementsやDesignの作成、レビュー、実装、結果の説明 |
| `specbind` CLI | 構造検証、トレーサビリティ、承認の証拠、進捗、状態遷移、リリース前チェック |
| あなた | スコープの確定、必要な承認、プロジェクト固有の判断、公開結果の確認 |

SkillはCLIが持つ状態を直接書き換えません。逆にCLIは、Requirementsの意味が
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

Milestoneは、1回のリリースとしてまとめて届ける作業の単位です。Spec-backed item、
Direct item、項目どうしの依存関係、対象リリースをRoadmapに記録します。

同時にactiveにできるMilestoneは、プロジェクトごとに1つだけです。現在のRoadmapは
`.specbind/steering/roadmap.md`にあり、状態はCLIが管理します。リリースが完了
すると、Roadmapの内容はrelease archiveへ移ります。

いったんMilestoneに入れた作業は、単独なら通常の作業で済むような小さな変更でも、
同じリリース境界の中で追跡します。

## Spec-backed itemとDirect item

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

## 永続成果物とMilestone固有成果物

Spec-backed itemの成果物には、リリース後も残るものと、Milestoneがactiveな間だけ
存在するものがあります。

| 種類 | 代表例 | ライフサイクル |
| --- | --- | --- |
| 永続 | `spec.yaml`、`requirements.md`、`design.md`、`contract.md`、`log.md` | Specの現在の姿と履歴として残る |
| Milestone固有 | `brief.md`、`research.md`、`tasks.yaml` | 進行中の変更を進めるために使い、リリース完了時に片付ける |
| プロジェクト全体 | `steering/roadmap.md`、Steering文書、Contract review | 複数のSpecにまたがるスコープと判断を保持する |

RequirementsとDesignは差分メモではありません。どちらも、現在有効な契約の全体を
表します。以前から変えない記述も、そのまま現在の文書の中に残してください。

## Gateと承認

Requirements、Design、TasksにはそれぞれGateがあります。Gateの承認は単なる
チェック印ではなく、レビューした入力のrevisionとfingerprintに結び付いた証拠です。

そのため、上流の成果物が変わると、影響を受ける下流の承認やcompletion evidenceは
無効、または古い状態になります。エージェントが古いDesignやTasksのまま黙って
進んでしまうのを防ぐための仕組みです。

承認には2つの形があります。

- **explicit** — あなたがそのGateで内容を確認して承認する
- **delegated** — `specbind-quick`など、名前の付いた1回の実行に対して、承認を
  あらかじめ委任する

委任しても、レビューや検査は省略されません。また、承認済みGateを破棄したり、
Contract reviewを受理したりする権限までは委任されません。

## Contract review

Designには、そのSpecの内部構造だけでなく、外部へ公開する責任、依存、ファイルの
所有境界も含まれます。この部分をContractとして維持します。

Tasksを作る前に、activeなMilestoneに含まれる全SpecのContractをまとめてレビュー
します。Specが1つしかないMilestoneでも、このレビューは省略しません。所有権の
重複、循環依存、互換性の前提、統合時の抜けを、実装前に見つけるためです。

## Invalidationとrewind

承認したあとで前提が変わったときは、影響を受ける中でいちばん手前のGateを、
明示的にinvalidateします。

```text
Requirementsが変わった -> Requirements Gateからやり直す
Design/Contractが変わった -> Design Gateからやり直す
Tasksだけが変わった -> Tasks Gateからやり直す
```

invalidateすると、下流の証拠も消えます。これは失敗ではなく、変わった前提に古い
承認を使わないための、通常のrewindです。

なお、確立済みのSpecからRequirement groupやAcceptance Criterionを削除する場合、
v1では完全なretirement履歴を残せません。既存Requirementの削除が必要になった
ときは、履歴が欠けたまま進めず、その操作の手前で停止します。既存内容の更新と、
新しいRequirementの追加は問題なく行えます。

## 通常のライフサイクル

Spec-backed itemは、だいたい次の順で進みます。

```text
Discovery
  -> Requirements
  -> DesignとContract
  -> Design検証
  -> Milestone全体のContract review
  -> Tasks
  -> ImplementationとTask review
  -> Implementation validation
  -> Release
```

`specbind-quick`は、RequirementsからTasks承認までの確認回数を減らします。ただし、
使う成果物、レビュー、CLIの検査は通常のワークフローと同じです。
`specbind-implement`が実装するのは、1回につき1つのRoadmap itemだけです。v1には、
Milestone全体を自動で実装するオーケストレータはありません。

## プロジェクト固有の設定

`.specbind/settings/`以下のテンプレート、ルール、adapterは、プロジェクトの
持ち物です。初回の導入で既定値を作りますが、そのあと`specbind install`を実行
しても、プロジェクト側の設定を上書きしません。

一方、`.agents/skills/specbind-*/`と`.claude/skills/specbind-*/`はSpecBind製品
側の持ち物です。`specbind install`を再実行すると、Gitがクリーンであることを
確認したうえで、現在の埋め込み版へ更新します。Skillファイルを直接編集する
やり方は、サポートしているカスタマイズ方法ではありません。

---

[ガイドの入口](./index.md) | [Getting Started](./getting-started.md)
