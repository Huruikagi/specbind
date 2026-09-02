# 既存実装からSpecを確立する

`sb-discovery`の明示的なリバースモードは、固定した既存リビジョンがすでに表している
プロダクトについて、永続的なSpecを確立します。動くコードはあるが信頼できる仕様がない
プロジェクト向けであり、別のSDD製品からの移行でも、新しい変更の提供でもありません。

実装は証拠であって、仕様を決める権威ではありません。観察した挙動は、維持する意図、
構造上の制約、歴史的な事情、内部詳細、バグの疑い、または判断が必要な問いになり得ます。

## 前提条件

- 永続的なSpecがなく、アクティブなMilestoneもない
- Steeringがプロダクトの目的、技術制約、構造を扱っている
- Steeringを含むリポジトリがコミット済みで、作業ツリーがクリーンである
- 対象をリポジトリ全体または具体的な領域として指定する
- そのリビジョンが表す既存のプロダクトバージョンを指定する

Steeringが不足している場合は、先に`sb-configure`と`sb-steering`を使い、結果を
コミットします。設定とリバース確立は別の実行です。

## 1回の確認から最後まで進む経路

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

提案には`baseline_version`、`reverseSpecs`候補、維持する意図と根拠、依存関係、
停止が必要な不明点と後回しにできる不明点、バグの疑い、対象外が含まれます。この完全な
提案を確認するまで何も作りません。確認後は、通常のフェーズ確認では止まらず進みます。

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

有効なDeferred Findings Adapterがあれば、欠陥に見える挙動を、ソースリビジョン、証拠の
位置、主張とともに「バグの疑い」として記録できます。自動的にバグや要件にはならず、この
経路では修正しません。プロジェクト外への送信には、別の権限が必要です。

## Tasksもリリースも作らない

リバースでは、通常のRequirements、Design、Design検証、Contract Reviewの担当を
再利用します。Designを承認すると、リバースSpecは`adoption_ready`になります。
`tasks.yaml`は作らず、実装や実装検証も始めません。Release Adapter、タグ、公開、
`target_release`も作りません。

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

## 次に読む

- [基本概念](./concepts.md)
- [既存プロジェクトで始める](./start-existing-project.md)
- [プロジェクトに合わせてカスタマイズする](./customization.md)
- [現在のスキル一覧](https://huruikagi.github.io/specbind/reference/current-skill-index/)（英語）

---

[はじめに](./getting-started.md) | [既存プロジェクトで始める](./start-existing-project.md)
