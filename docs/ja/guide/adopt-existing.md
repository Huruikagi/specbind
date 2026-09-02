# 既存実装からSpecを確立する

`sb-discovery`の明示的なリバースモードは、固定した既存リビジョンがすでに表している
プロダクトについて、永続的なSpecを確立します。動くコードはあるが信頼できる仕様がない
プロジェクト向けであり、別のSDD製品からの移行でも、新しい変更の提供でもありません。

実装は証拠であって、仕様を決める権威ではありません。観察した挙動は、維持する意図、
構造上の制約、歴史的な事情、内部詳細、バグの疑い、または判断が必要な問いになり得ます。

この経路が作るのは、Requirements、Design、Contract Reviewまでを確定した非リリースの
基準履歴です。Tasks、実装変更、プロダクトリリースは作りません。次の変更から
SpecBindを使いたいだけなら、[既存プロジェクトで始める](./start-existing-project.md)
の通常の手順へ進んでください。

## 前提条件

- SpecBindを[インストール](./install.md)済みである
- 永続的なSpecがなく、アクティブなMilestoneもない
- Steeringがプロダクトの目的、技術制約、構造を扱っている
- Steeringを含むリポジトリがコミット済みで、作業ツリーがクリーンである
- 対象をリポジトリ全体または具体的な領域として指定する
- そのリビジョンが表す既存のプロダクトバージョンを指定する

## 全体の流れ

```text
Steeringを設定してコミット
  -> 対象領域と既存バージョンを指定してsb-discovery
  -> source_revisionを固定
  -> コードとテストを調査
  -> 完全なリバース提案を1回確認
  -> リバースRoadmap、Spec、Brief、Researchを作成
  -> Requirements
  -> Designと独立したDesign検証
  -> Contract Review
  -> adoption finalize
```

確認が必要なのは、原則としてリバース提案の1回だけです。設定とリバース確立は別の
実行なので、まずSteeringと共通設定を整えてから、Discoveryを1回だけ回します。

## 1. Steeringを整える

Steeringが不足している場合は、先に`sb-configure`へ初回レビューを依頼します。

```text
$sb-configure 既存実装を採用するための初期設定を、このプロジェクトについて
見直してください。必要なSteeringの作成から始めてください。
```

`sb-configure`は最初に機械的に確認できる設定の要約を読みます。継続的に使う方針が
必要なら、Steeringの初期作成または同期を`sb-steering`へ引き継ぎます。提案された
Steeringを確認してコミットしてから採用を始めてください。Discoveryはそのリビジョンを
調査の証拠として固定します。

## 2. 共通設定を対象を絞って見直す

Steeringができたら、もう一度`sb-configure`に、Steeringとリポジトリの事実を
Requirements・Designテンプレート、共有Ruleと照らし合わせるよう依頼します。

```text
$sb-configure 確定したSteeringとリポジトリの事実を使い、このプロジェクト向けに
Requirements・Designテンプレートと共有Ruleを見直してください。
```

テンプレート、Rule、エージェント、運用アダプターなど、残る面はそれぞれ別の依頼で
見直します。`sb-configure`は関係する変更ごとに要約を読み直し、必要なaftercareまで
完了します。Designテンプレートを増やすのは、独立した責任を繰り返し扱う必要がある
場合だけです。技術名だけでは理由になりません。設定変更によって既存のSpecや
ライフサイクル成果物が暗黙に調整されることもありません。

詳しくは[カスタマイズ](./customization.md)を参照してください。

## 3. リバースDiscoveryを開始する

Steeringをコミットし、作業ツリーをクリーンにしてから、採用する範囲と既存の
プロダクトバージョンを明示して依頼します。たとえばリポジトリ全体なら次のように
なります。

```text
$sb-discovery このリポジトリ全体の既存実装を、既存バージョンv2.4.0のSpecとして
確立してください。現在のコードとテストを証拠として調査し、何かを作る前に
Spec境界と維持する意図を私に確認してください。
```

Discoveryは採用用の事前検査を行い、調査したリビジョンを固定します。そのうえで、
`baseline_version`、`reverseSpecs`候補、維持する意図と根拠、依存関係、停止が必要な
不明点と後回しにできる不明点、バグの疑い、対象外を、1つの完全なリバース提案として
示します。この提案を確認するまで何も作りません。

## 4. 確認後は基準確立まで進む

提案を確認すると、同じ実行がリバースMilestoneを作成し、Requirements、Designの検証と
承認、Milestone全体のContract Reviewまで続行します。通常のフェーズ確認では止まらず、
Tasksは作りません。Specの意味が変わる問い、ソースの変更、ライフサイクル検査の失敗が
ある場合だけ停止します。

Roadmapは`newSpecs`や`specUpdates`ではなく`reverseSpecs`を使い、`target_release`を
持ちません。作成した各Specには、次の来歴が残ります。

```yaml
establishment:
  kind: reverse
  source_revision: <固定したGitリビジョン>
  baseline_version: <既存のプロダクトバージョン>
  milestone_id: <リバースMilestone>
```

## リビジョン固定と不明点

`specbind adoption preflight`が返す`source_revision`を固定します。完了するまで、実装、
テスト、依存関係、設定、Steeringを変更できません。リバースのスコープ更新やrebaselineも
できません。ソースが変わった場合は停止し、新しいクリーンなリビジョンからやり直します。

維持する挙動を意味のある形で書くために回答が必要なら、その問いは該当Specを停止します。
独立した別のSpecは進められますが、Contract Reviewとファイナライズは待ちます。どの回答に
なっても現在のSpecの意味が変わらない問いだけ、後回しにできます。

Discoveryは`specbind adapter list`で有効なDeferred Findings Adapterを探し、種別名から
コマンドを推測せず、一覧に示されたselectorを読みます。欠陥に見える挙動は、ソース
リビジョン、証拠の位置、主張とともに「バグの疑い」として提案できます。クリーンな固定
リビジョンを保つため、確認済みのローカル保存先へ記録するのは、リバースMilestoneを
作成した後です。自動的にバグや要件にはならず、この経路では修正しません。プロジェクト
外への送信には、別の権限が必要です。

## Tasksもリリースも作らない

リバースでは、通常のRequirements、Design、Design検証、Contract Reviewの担当を
再利用します。Designを承認すると、リバースSpecは`adoption_ready`になります。
`tasks.yaml`は作らず、実装や実装検証も始めません。Release Adapter、タグ、公開、
`target_release`も作りません。

依存関係があるリバースSpecのDesignは、その順序に従って進めます。後続のリバースSpecが
先行Designの完了を待っていて、まだContractを持たない間に限り、Designの検査はContract
グラフ全体を暫定状態として扱えます。それ以外のグラフエラーは見逃しません。Milestone
全体のContract Reviewは、すべてのDesignとContractがそろってから開始し、通常どおり
完全なグラフを必須とします。

途中で通常の変更依頼が来た場合は、先にリバースを完了し、その後に新しい通常Milestoneを
作ります。緊急時は`specbind milestone reverse abandon --milestone-id <id>`で明示的に
リバースを中断し、通常Discoveryで変更を進めたあと、新しいリビジョンからやり直します。
ライフサイクル状態を手作業で削除しないでください。

## ファイナライズと履歴

すべてのSpecが`adoption_ready`になり、Contract Reviewがfreshになると、Discoveryは
次を実行します。

```sh
specbind milestone reverse finalize --log-entries <path-or->
```

ファイナライズは、確立の来歴を残したままactive changeを閉じ、一時的なBriefとResearchを
削除し、各Specの`log.md`へ`ベースライン <version>`を記録します。RoadmapとContract
Reviewは`baselines/`へ履歴化され、アクティブなMilestoneが閉じます。これらは採用の記録で
あり、プロダクトリリースの記録ではありません。

確立したSpecは、元のリビジョンとバージョンの来歴を保持したまま、以後は通常の既存Specと
して扱われます。次の変更からは、
[既存プロジェクトで始める](./start-existing-project.md)の通常の手順に合流します。

## 次に読む

- [基本概念](./concepts.md)
- [既存プロジェクトで始める](./start-existing-project.md)
- [カスタマイズ](./customization.md)
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）

---

[ユーザーガイド](../index.md) | [既存プロジェクトで始める](./start-existing-project.md) | [基本概念](./concepts.md)
