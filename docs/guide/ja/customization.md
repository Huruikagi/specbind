# プロジェクトに合わせてカスタマイズする

SpecBindのライフサイクルや検証を変えずに、成果物の書き方、プロジェクト固有の
判断基準、運用手順、エージェントの能力配分をプロジェクトに合わせられます。

このページは、SpecBindがサポートするカスタマイズ面の入口です。将来の
カスタマイズ用Skillから案内する場合も、まずこの分類を使い、実際の対象と内容は
そのプロジェクトで`specbind` CLIから読み取ります。

## 変更したいことから選ぶ

| 変更したいこと | 編集する場所 | 主な確認方法 |
| --- | --- | --- |
| RequirementsやDesignなどの構成、見出し、例 | `{{SPEC_DIR}}/settings/templates/` | `specbind template list`、`specbind template read` |
| Requirements、Design、Contract、Tasks、Steeringの書き方や判断基準 | `{{SPEC_DIR}}/settings/rules/` | 適用するSkillで成果物を作成・レビューする |
| リリース、Git、保留した指摘の届け先 | `{{SPEC_DIR}}/settings/adapters/` | `specbind adapter list`、`specbind adapter read` |
| プロジェクトについてエージェントが長く参照する知識 | `{{SPEC_DIR}}/steering/` | `specbind steering list`、`specbind steering read` |
| Specの置き場所、成果物の言語、利用するエージェント | `.specbind.json`と`specbind install`のオプション | `specbind install --dry-run ...` |
| 役割ごとのモデルと推論量 | `.specbind.json`の`agentRoles` | 設定後に`specbind install --dry-run` |

`{{SPEC_DIR}}`は`.specbind.json`の`specDir`です。新規プロジェクトの既定値は
`.specbind`です。

## 成果物テンプレート

テンプレートは、新しい成果物の構成と初期内容を決めるscaffoldです。見出し、節の
分け方、例、テンプレート内の`specbind:instruction`コメントを調整できます。

初回installでは、構成を変える理由が多い次の2つだけをプロジェクト設定として
作成します。

- `settings/templates/specs/requirements.md`
- `settings/templates/specs/design.md`

Brief、Research、Contract、Implementation NotesのSpecテンプレートと、Steeringの
テンプレートもバイナリに埋め込まれています。必要なプロジェクトだけが、CLIの一覧に
表示される`template_path`へコピーして上書きできます。

```sh
specbind template list spec
specbind template read spec requirements
specbind template list steering
specbind template read steering document
```

テンプレートを変えても、すでに存在する成果物は自動では書き換わりません。変更後に
作る成果物から新しいテンプレートが使われます。

!!! warning
    `type`、`artifact_id`、必須の識別子や対応関係など、CLIが読む構造は残して
    ください。自由に変えられるのは、機械可読な契約を保った範囲です。

## 共有ルール

共有ルールは、複数のエージェントで共通に使う、プロジェクト固有の執筆方針と判断
基準です。強める、緩める、置き換える、削除することができます。

| ファイル | カスタマイズする内容 |
| --- | --- |
| `ears-format.md` | RequirementsのEARS表現、主語、テスト可能性の好み |
| `design-principles.md` | アーキテクチャ、インターフェース、データ、エラー処理、文書の詳しさ |
| `contract-principles.md` | 所有境界、公開seam、互換性、依存方向のプロジェクト方針 |
| `tasks-generation.md` | Taskの大きさ、分割、テスト作業、並列化の好み |
| `steering-principles.md` | Steeringに残す知識の粒度、例、更新方針 |

v1のSkillが読むのは、この5つの既知のパスだけです。別名のルールファイルを追加しても
自動では読み込まれません。ルールを変更しても、成果物の必須構造、Gate、承認、状態
遷移、Skillの必須手順、CLIの検証を弱めることはできません。

## 運用adapter

adapterは、プロジェクトごとに異なる運用を自然言語でエージェントへ伝える場所です。
Markdownの本文は自由形式で、コードブロックを書いても自動実行されるhookには
なりません。

| ファイル | 伝える内容 |
| --- | --- |
| `release.md` | リリース準備、公開、検証、後片付け |
| `git.md` | checkpointの単位、stage、commit message、branch、pushの方針 |
| `deferred.md` | Gateを止めない実在の指摘を残すIssue tracker、wiki、ファイルなどの届け先 |

```sh
specbind adapter list
specbind adapter read git
```

adapterは方針であり、権限ではありません。たとえば`git.md`にpush方針があっても、
ユーザーの依頼や実行環境の権限なしにpushできるようにはなりません。v1は既知の3つの
selectorだけを読み、`settings/adapters/`へ任意のファイルを置いて拡張する仕組みでは
ありません。

## Steering

Steeringは、製品の目的、技術方針、構造、テスト方針、セキュリティ姿勢など、今後の
作業でも参照するプロジェクトの知識です。作業中だけのメモや、すぐ変わる状態は置き
ません。

`specbind-steering` Skillは、現在の一覧を確認したうえで、初期作成、既存文書の同期、
1文書の追加を行います。既定の`product`、`tech`、`structure`は提案であり、名前の
変更、統合、分割、採用しない選択ができます。

```sh
specbind steering list
specbind steering read <selector>
```

SteeringはGateやfreshnessの入力ではありません。ただし、accepted completionがある
Milestoneの途中で編集するとcompletionの再検証が必要になることがあります。通常は
Milestone開始から最初のcompletionまで、またはrelease finalize後に更新するのが
扱いやすいタイミングです。

## プロジェクト設定と役割別モデル

初回installでは、成果物の言語、利用するエージェント、Specの置き場所、ルート指示
ファイルへの案内追加を選べます。

```sh
specbind install --dry-run --agent codex --language ja --spec-dir .specbind --project-instructions
```

`specDir`は初回install時に決め、v1では導入後に変更できません。言語と選択した
エージェントはinstall時の指定として`.specbind.json`に保存されます。導入後に
エージェントを追加することはできますが、削除とアンインストールはv1の対象外です。

実装、レビュー、調査などの役割ごとのコストと能力を変えたい場合は、
`.specbind.json`の`agentRoles`でmodelを上書きします。Codexだけは
`reasoningEffort`も指定できます。

```json
{
  "agentRoles": {
    "codex": {
      "implementer": {
        "model": "gpt-5.6-luna",
        "reasoningEffort": "medium"
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

変更後は、クリーンでcommit済みのリポジトリでdry runを確認してからinstallを再実行
します。`.codex/agents/specbind-*.toml`や`.claude/agents/specbind-*.md`を直接編集する
のではなく、設定から再生成してください。

## カスタマイズできないもの

次はSpecBindが管理する製品契約です。直接編集しても、サポートされたカスタマイズには
なりません。

- `.agents/skills/specbind-*/`と`.claude/skills/specbind-*/`のSkill本体
- `.codex/agents/specbind-*.toml`と`.claude/agents/specbind-*.md`の役割定義
- CLIが埋め込むprotocolとschema
- Gate、承認、fingerprint、状態遷移、必須のトレーサビリティ
- `spec.yaml`、`tasks.yaml`、Roadmapなど、CLIが所有する構造化状態
- ルート指示ファイル内の`SpecBind`管理ブロック

プロジェクト固有の方針を追加したいときは、Skill本体をforkせず、目的に応じて
template、rule、adapter、Steeringのいずれかへ置きます。

## カスタマイズSkillから参照する場合

将来のカスタマイズ用Skillは、このページをユーザー向けの選択肢一覧として参照
できます。実行時には、次の順序を守るとプロジェクトの現在の状態とずれません。

1. 変更したい意図を確認し、template、rule、adapter、Steering、project configの
   どれに属するかを決める。
2. template、adapter、Steeringは対応するCLIの`list`と`read`で現在値を取得する。
3. プロジェクト所有のファイルだけを編集し、製品管理ファイルは編集しない。
4. 機械可読な構造と、カスタマイズできない製品契約を保つ。
5. diffを提示し、対応するCLIの再読込、dry run、または実際のSkill workflowで
   結果を確認する。

現在インストールされる全ファイルは
[現在の成果物一覧](../../current-artifact-index.md)で確認できます。

---

[ガイドの入口](./index.md) | [基本概念](./concepts.md) | [現在の成果物一覧](../../current-artifact-index.md)
