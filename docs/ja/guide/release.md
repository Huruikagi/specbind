# リリースする

このページでは、実装と検証まで終えたMilestoneを、1つのリリースとして締めるまでを
説明します。SpecBindでのリリースは「Milestone全体を出すか、出さないか」で、一部の
Specだけを切り出す方法はありません。

用語（Milestone、Spec、Gate、completion evidence など）は[基本概念](./concepts.md)に
まとめています。

## いつリリースするか

次がそろっていることが前提です。

- 参加する全SpecがImplementと`specbind-validate-implementation`を終え、CLIが
  completion evidenceを受理している（`specbind milestone status`で確認できます）
- このMilestoneを実際に出すと決めている

まだ試用の段階なら、無理にリリースまで進める必要はありません。実装の検証までを
完了地点にして、次のMilestoneのDiscoveryへ進んでもかまいません。

## 1. リリース方針を用意する

準備・公開・検証・後片付けをどう行うかは、プロジェクト固有の
`.specbind/settings/adapters/release.md`（Release adapter）に自然言語で書きます。
CLIもSpecBindも、この手順を代行しません。

- **未設定のまま`specbind-release`を実行した場合** — スキルがリポジトリ内の
  リリース用workflow、version manifest、buildスクリプト、既存の
  `RELEASE`/`CHANGELOG`などを調べ、Prepare・Publish・Verify・After-finalizeの
  具体案を提示します。承認するとその案を`release.md`へ保存してローカルコミットし、
  **その回はそこで停止します**（バージョンのbindも公開もしません）。
- **設定を変更したあと** — `release.md`の保存は通常のプロジェクト変更なので、
  completion evidenceを受理済みのSpecは、それぞれcompletionのやり直しが必要に
  なります。やり直してから、改めて`specbind-release`を実行します。
- **プロジェクト固有の作業が本当に不要なら** — Front Matterを残して本文を空に
  すると、「リリースに固有の手順は不要」という明示になります。

書き方の詳細は[プロジェクトに合わせてカスタマイズする](./customization.md)の
adapterの節にあります。

## 2. specbind-release を実行する

```text
$specbind-release 1.0.0
```

リリースの操作は、基本的にこのスキル1つで進みます。バージョンのbindからfinalizeまで
スキルがオーケストレーションし、状態の変更はすべてスキル経由でCLIが行います。あなたは
要所の確認に答えます。以下は、スキルが内部で進める流れです。

### バージョンのbind

リリースのラベルは不透明で、大文字小文字も区別します（`v1.4.0`と`1.4.0`は別の
リリース）。スキルはこの値を自動で決めず、必ず尋ねます。指定するとMilestoneへ
`target_release`が書き込まれます。

あるSpecがすでに`release_ready`になってからbindすると、そのcompletion evidenceが
古くなり、対象Specのcompletionをやり直すことになります。バージョンが早い段階で
決まっているなら、リリースを待たず自分で固定しておくと、このやり直しを避けられます。

```sh
specbind milestone bind-release 1.0.0     # 任意。早めにバージョンを固定したいとき
```

### preflight / Prepare / Publish / Verify

スキルはまず前提を確認し（この検査が失敗した回は、そこで止まります）、通ったら
adapterに書いた手順を順に実行します。

- **Prepare** — 繰り返し可能でローカルに閉じた準備。失敗したらそこで報告し、
  リポジトリの外には何も出ていません。
- **Publish** — リリースの識別子を確定させる、または外部に出る境界です。ローカル
  タグでも、デプロイやアップロードでも、スキルは「何をどのバージョンに対して
  行うか」を述べて確認を取ってから実行します。広い指示で始めた場合でも、公開は
  別に確認します。
- **Verify** — 「意図したバージョンが実際に公開され、使える」ことを、公開コマンドの
  出力の読み直しではなく、新しい証拠で確かめます。確かめる手段がない場合は
  「検証できない」であって、成功ではありません。この場合はfinalizeしません。

Publishは成功したがVerifyが通らなかったときは、Milestoneはactiveのまま、SpecBindの
成果物もそのままです。公開のロールバックや盲目的な再試行はせず、現状を報告して
どう扱うかを相談します。

### finalize

Verifyまで通ったら、参加する各Specの成果（要求ではなく、実際に届けたもの）を1行で
要約し、スキルがMilestone全体のfinalizeをCLIに指示します。`log.md`はCLIが構造ごと
更新するので、手で先に編集しないでください。失敗しても再実行でき、履歴は重複しません。

## 3. finalize 後

- RoadmapはCLIによってrelease archiveへ移ります。各Specはリリース後も残り、次の
  変更の出発点として待機状態（idle）に戻ります
- `log.md`にこのMilestoneの記録が追加されます
- Milestoneはクローズされ、次の`specbind-discovery`が新しいMilestoneを開始できます
- `git.md`に方針があれば、finalizeが生成したlog・archive・cleanupは、公開対象とは
  別のローカルコミットになります

このMilestoneで新規Specを追加した、Contractが動いた、あるいはまだSteeringが1つも
ない状態でリリースした場合は、`specbind-steering`を一度回しておくとよいです。
finalize後はSteeringの編集がふたたび自由になります。

## 現在の状態を確認する

スキルを呼ぶ前に、自分でリリースの準備状況を見ておけます。どちらも読み取り専用です。

```sh
specbind milestone status          # Target release、Release blockersを表示
specbind release preflight         # 未bindや未受理のcompletionなど、残っている障害を表示
```

## 次に読む

- [基本概念](./concepts.md)
- [プロジェクトに合わせてカスタマイズする](./customization.md) — Release / Git adapterの書き方
- [現在のスキル一覧](../../current-skill-index.md)
- [現在の成果物一覧](../../current-artifact-index.md)

---

[Getting Started](./getting-started.md) | [基本概念](./concepts.md)
