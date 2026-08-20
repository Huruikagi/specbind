# 基本概念

SpecBindは、エージェントにすべてを任せる仕組みでも、すべての変更に文書作成を
要求する仕組みでもありません。意味を判断するエージェントと、状態を検証・記録する
CLIを組み合わせ、仕様と実装の関係を長期的に保ちます。

## SkillとCLI

SpecBindでは責任を次のように分けます。

| 担当 | 主な責任 |
| --- | --- |
| エージェントのSkill | スコープ判断、RequirementsやDesignの作成、レビュー、実装、結果の説明 |
| `specbind` CLI | 構造検証、トレーサビリティ、承認証拠、進捗、状態遷移、release guard |
| 利用者 | スコープの確定、必要な承認、プロジェクト固有の判断、公開結果の確認 |

SkillはCLIの状態を直接書き換えません。CLIも、Requirementsの意味が正しいか、
Designが適切かといった判断を代行しません。両方の層を通すことで、機械的に正しい
だけの仕様や、意味はもっともらしいが状態遷移を守っていない作業を防ぎます。

## Spec

Specは、プロジェクトが継続して所有する1つの能力または責任境界です。

Specは変更ごとに使い捨てる計画ではありません。後続Milestoneで同じ能力を変更する
場合は、同じSpecの完全な現在形としてRequirements、Design、Contractを更新します。
リリース後もSpec自体は残り、次の変更の出発点になります。

既定のSpecディレクトリは`.specbind/specs/<spec>/`です。`<spec>`には責任を表す
短いkebab-caseのIDを使います。

## MilestoneとRoadmap

Milestoneは、1回のリリースとして一緒に届ける作業のまとまりです。Spec-backed item、
Direct item、項目間の依存関係、対象リリースをRoadmapに保持します。

プロジェクトで同時にactiveにできるMilestoneは1つです。現在のRoadmapは
`.specbind/steering/roadmap.md`にあり、CLIが状態を管理します。リリース完了時には
release archiveへ移されます。

Milestoneに含めた作業は、単独なら通常作業になり得る小さな変更でも、同じリリース
境界の中で追跡されます。

## Spec-backed itemとDirect item

Discoveryは、ワークフローに入る作業を所有境界で分類します。

| 種類 | 選ばれる条件 | 持つもの |
| --- | --- | --- |
| 既存Specの更新 | 既存Specが所有する振る舞いや境界を変更する | 更新されたRequirements、Design、Contract、Tasks |
| 新規Spec | プロジェクトに新しい永続的な責務を追加する | 新しいRequirements、Design、Contract、Tasks |
| Direct | どのSpecにも属さず、Requirements、Design、Contractの変更を必要としない | Roadmap上の要約と完了状態 |

分類は作業量ではなく所有権で決まります。大きな変更でも既存の1つの責任内なら
既存Specの更新であり、小さな変更でも新しい責務を生むなら新規Specです。

Directとして始めた作業が、実際には仕様やContractの変更を必要とすると判明した
場合は、そのまま成果物を追加せず、Discoveryへ戻して分類し直します。

## 永続成果物とMilestone固有成果物

Spec-backed itemの成果物には、リリース後も残るものと、active Milestoneの間だけ
存在するものがあります。

| 種類 | 代表例 | ライフサイクル |
| --- | --- | --- |
| 永続 | `spec.yaml`、`requirements.md`、`design.md`、`contract.md`、`log.md` | Specの現在形と履歴として残る |
| Milestone固有 | `brief.md`、`research.md`、`tasks.yaml` | activeな変更を進め、release finalizationで片付けられる |
| プロジェクト全体 | `steering/roadmap.md`、Steering文書、Contract review | 複数Specをまたぐスコープと判断を保持する |

RequirementsとDesignは差分メモではなく、現在の有効な契約全体を表します。
以前の記述を維持する場合も、その内容が現在形の文書に残ります。

## Gateと承認

Requirements、Design、TasksにはGateがあります。Gateの承認は、単なるチェック印では
なく、レビューした入力のrevisionとfingerprintに結び付いた証拠です。

上流の成果物が変わると、影響を受ける下流の承認やcompletion evidenceは無効または
staleになります。古いDesignやTasksのままエージェントが黙って進むことを防ぐためです。

承認には2つの形があります。

- **explicit** — 利用者がそのGateで内容を確認して承認する
- **delegated** — `specbind-quick`など、名前付きの実行について事前に承認を委任する

委任はレビューや検査を省略しません。また、承認済みGateの破棄やContract reviewの
受理まで自動的に許可するものではありません。

## Contract review

Designは各Specの内部構造だけでなく、外部へ公開する責任、依存、ファイル所有境界を
Contractとして維持します。

Tasksを作る前に、active Milestone内の全SpecのContractをまとめてレビューします。
単一SpecのMilestoneでも、このbarrierは省略されません。実装前に所有権の重複、循環
依存、互換性の前提、統合上の抜けを発見するためです。

## Invalidationとrewind

承認後に前提が変わった場合、該当する最も早いGateを明示的にinvalidateします。

```text
Requirements変更 -> Requirements Gateからやり直す
Design/Contract変更 -> Design Gateからやり直す
Tasksだけの変更 -> Tasks Gateからやり直す
```

Invalidateは下流の証拠も消します。これは失敗ではなく、変更された前提に対して
古い承認を使わないための通常のrewindです。

v1では、確立済みSpecからRequirement groupまたはAcceptance Criterionを削除する
完全なretirement履歴はサポートされていません。既存Requirementの削除が必要な場合は、
不完全な履歴を作らず、その操作の前で停止します。既存内容の更新と新しいRequirementの
追加はサポートされます。

## 通常のライフサイクル

Spec-backed itemは、概ね次の順で進みます。

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

`specbind-quick`はRequirementsからTasks承認までの確認回数を減らしますが、同じ
成果物、レビュー、CLI guardを使用します。`specbind-implement`は1回に1つのRoadmap
itemだけを実装します。v1では、Milestone全体を自動実装するオーケストレータは
ありません。

## プロジェクト固有の設定

`.specbind/settings/`以下のテンプレート、ルール、adapterはプロジェクトが所有します。
初回導入では既定値が作られますが、その後の`specbind install`は既存のプロジェクト
設定を上書きしません。

一方、`.agents/skills/specbind-*/`と`.claude/skills/specbind-*/`は製品管理です。
`specbind install`を再実行すると、Gitがクリーンであることを確認したうえで、現在の
埋め込み版へ更新されます。Skillファイルの直接編集は、サポートされるカスタマイズ
方法ではありません。

---

[ガイドの入口](./index.md) | [Getting Started](./getting-started.md)
