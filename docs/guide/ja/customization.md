# プロジェクトに合わせてカスタマイズする

SpecBindのライフサイクルと検証はそのままに、成果物の書き方、プロジェクト固有の
判断基準、運用手順、役割ごとに使うモデルを、プロジェクトに合わせて調整できます。

このページでは、何を変えたいときにどこを編集するのかをまとめます。

## 変更したいことから選ぶ

| 変更したいこと | 編集する場所 | 主な確認方法 |
| --- | --- | --- |
| RequirementsやDesignなどの構成、見出し、例 | `.specbind/settings/templates/` | `specbind template list`、`specbind template read` |
| Requirements、Design、Contract、Tasks、Steeringの書き方や判断基準 | `.specbind/settings/rules/` | 対応するSkillで成果物を作成・レビューする |
| リリース、Git、保留した指摘の届け先 | `.specbind/settings/adapters/` | `specbind adapter list`、`specbind adapter read` |
| プロジェクトについてエージェントが長く参照する知識 | `.specbind/steering/` | `specbind steering list`、`specbind steering read` |
| Specの置き場所、成果物の言語、使うエージェント | `.specbind.json`と`specbind install`のオプション | `specbind install --dry-run ...` |
| 役割ごとのモデルと推論の深さ | `.specbind.json`の`agentRoles` | 設定後に`specbind install --dry-run` |

このページでは、Specの置き場所が既定値の`.specbind`である前提でパスを書きます。
`.specbind.json`の`specDir`を変えている場合は、その値に読み替えてください。

## 成果物テンプレート

テンプレートは、新しい成果物の構成と初期内容を決めるひな形です。見出し、節の
分け方、例、テンプレート内の`specbind:instruction`コメントを調整できます。

初回のinstallでプロジェクト側にコピーされるのは、構成を変えたくなることが多い
次の2つだけです。

- `settings/templates/specs/requirements.md`
- `settings/templates/specs/design.md`

Brief、Research、Contract、Implementation NotesのSpecテンプレートと、Steeringの
テンプレートもCLIに埋め込んであります。変更したいプロジェクトだけが、CLIの一覧に
出てくる`template_path`へコピーして上書きしてください。

```sh
specbind template list spec
specbind template read spec requirements
specbind template list steering
specbind template read steering document
```

テンプレートを変えても、すでにある成果物は書き換わりません。変更後に新しく作る
成果物から、新しいテンプレートが使われます。

!!! warning
    `type`、`artifact_id`、必須の識別子や対応関係など、CLIが読み取る構造は残して
    ください。自由に変えられるのは、この機械可読な部分を保った範囲だけです。

## 共有ルール

共有ルールは、複数のエージェントが共通で参照する、プロジェクト固有の執筆方針と
判断基準です。内容は強めても、緩めても、書き換えても、削除しても構いません。

| ファイル | 書けること |
| --- | --- |
| `ears-format.md` | RequirementsのEARS表現、主語の立て方、テストしやすさの好み |
| `design-principles.md` | アーキテクチャ、インターフェース、データ、エラー処理、記述の細かさ |
| `contract-principles.md` | 所有境界、外部へ公開する接点、互換性、依存の向きに関する方針 |
| `tasks-generation.md` | Taskの大きさ、分割の仕方、テスト作業の扱い、並列化の好み |
| `steering-principles.md` | Steeringに残す知識の粒度、例の書き方、更新の方針 |

v1のSkillが読むのは、この5つのパスだけです。別の名前でルールファイルを足しても
読み込まれません。

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

adapterはあくまで方針であり、権限ではありません。たとえば`git.md`にpushの方針を
書いても、それだけでエージェントがpushできるようになるわけではなく、あなたの依頼と
実行環境の権限が別途必要です。

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
specbind steering read <selector>
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
できますが、削除とアンインストールはv1では対応しません。

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

[ガイドの入口](./index.md) | [基本概念](./concepts.md) | [現在の成果物一覧](../../current-artifact-index.md)
