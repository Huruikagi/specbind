# SpecBindをアップデートする

SpecBindのアップデートには、PCで実行する`specbind`バイナリの更新と、
各プロジェクトに配置された製品管理対象ファイルの更新があります。この2つは
別々の操作です。

このガイドでは、バイナリをmiseで管理している場合を基本に説明します。
`specbind`自身がバイナリを置き換える自己更新コマンドはありません。

## 1. miseでバイナリを更新する

更新前に`specbind --version`を実行し、元のバージョンを記録してください。
中断して再開する場合も、この値を移行計画の起点として使います。

対象プロジェクトのルートで、設定されているバージョン範囲内の最新版へ更新します。

```sh
mise upgrade github:Huruikagi/specbind
specbind --version
```

`mise.toml`で`latest`を選んでいる場合、miseの`minimum_release_age`を満たす最新の
安定版が選ばれます。完全に固定されたバージョンは通常の`mise upgrade`では進みません。
特定のバージョンへ変更する場合は、明示的に選び直します。

```sh
mise use github:Huruikagi/specbind@<version>
specbind --version
```

`mise.toml`と`mise.lock`に差分がある場合は、内容を確認して、プロジェクトの通常の
手順でコミットしてください。`mise.lock`は、チームやCIが同じバージョンと配布物を
使うためのロックファイルです。詳しい挙動はmiseの
[`upgrade`](https://mise.jdx.dev/cli/upgrade.html)と
[`mise.lock`](https://mise.jdx.dev/dev-tools/mise-lock.html)の説明を参照してください。

!!! warning "バイナリ選択の変更をコミットしてからプロジェクト資産を更新する"
    次の`specbind install`が既存ファイルを置換・移動・削除する場合、リポジトリに
    1件以上のコミットがあり、変更対象のパスがGit上でクリーンである必要があります。
    miseによる更新で`mise.toml`や`mise.lock`が変更された場合は、その差分を
    コミットしてから進んでください。更新対象外にある未ステージまたは未追跡の変更は
    残して構いません。SpecBindはそのパスを表示し、変更せずに保持します。更新前からある
    ステージ済みの変更、名前変更、コピー、変更を含むサブモジュールについては、2つの
    チェックポイントを安全に分離できないため、従来どおり更新を停止します。

## 2. プロジェクト内の製品管理対象を更新する

新しいCLIで、元のバージョンからのプロジェクト移行計画を確認します。

```sh
specbind migration plan --from <更新前のバージョン> --json
```

移行先は実行中のCLIのバージョンです。`--to`で明示することもできますが、
カタログが把握する範囲を超えた移行先やダウングレードは受け付けません。
計画の取得はファイルを変更しません。同じコマンドを再実行すると、現在の
ファイルから残作業を判定し直します。計画の項目は、次の手順3で進めます。

新しいバイナリには、そのバージョンのスキルやほかの製品管理対象が埋め込まれています。
まず、適用される計画を確認します。

```sh
git status --short
specbind install --dry-run
```

`create`、`replace`、`keep`と、廃止された製品管理対象に対する`remove`を確認します。
`Unrelated changes: preserved`が表示された場合は、自分の既存作業であり、更新対象と
重なっていないことも確認してから適用します。

```sh
specbind install
git status --short
git diff
```

差分を確認し、プロジェクトの通常の手順でコミットしてください。更新済みの
プロジェクトを取得したほかのメンバーは、`mise install`でロックファイルに固定された
バイナリを導入できます。プロジェクト内の更新済みファイルはGitから取得されるため、
全員が`specbind install`を再実行する必要はありません。

## 3. 移行計画の項目を進める

エージェントで更新する場合は、製品管理対象の更新後に新しい`sb-configure`を
読み直してから、計画にある手順を確認します。`required`の未完了項目があれば
移行作業全体の完了とは扱いません。`recommended`と`optional`は提案を確認して
見送れます。適用判定が`blocked`の場合は、表示された入力の問題を解消します。

### 1.5.0: `description`の追加

最初の移行項目は、1.5.0境界のRequirements・Design・Steeringと関連テンプレートの
`description`追加です。文書が継続して引き受ける責務を一文で記録します。
既存1.x文書での欠落は有効なままで、この追加は推奨作業です。エージェントが
対象ごとの差分を提示し、別途確認された範囲だけを編集します。テンプレート更新は
既存文書を書き換えず、説明の追加も承認や鮮度の証拠にはなりません。

`spec list`はRequirements、`artifact list <spec>`はDesign、`steering list`は
Steeringのdescriptionを表示します。`missing`は項目の欠落、`invalid`は不正な
説明またはRequirementsの検証エラー、`unavailable`はRequirementsを一意に
取得できない状態です。一覧の説明は現在の文書内容であり、承認状態とは独立しています。

Designの新規作成時には、`artifact check <spec> <selector> --template <template>`で
選択したテンプレートの作成条件とdescriptionの完全な継承を確認できます。
責務を評価済みのSpec固有one-off文書には`--template design/main`を指定し、
その文書独自の責務を記します。この検査は既存文書の由来を証明せず、承認状態も変更しません。

### 要件の退役を使う前に

既存RequirementsのID移行は不要です。[要件の退役](./implement-step-by-step.md#retire-requirements)
を使う前に、インストール済みのスキルと、変更している場合はテンプレートの指示を
更新してください。旧バイナリは`_Retired_`マーカーを正しく解釈できないため、利用を
始めたあとは旧バージョンに戻さないでください。

## 更新されるものと保持されるもの

| 対象 | 所有者 | アップデート時の扱い |
| --- | --- | --- |
| `specbind`バイナリ | mise | `mise upgrade`または明示的な`mise use`で更新 |
| `.agents/skills/sb-*`、`.claude/skills/sb-*`などの製品管理対象 | SpecBind | `specbind install`で現在の埋め込み版へ置換。廃止された対象は計画に表示して削除 |
| `AGENTS.md`または`CLAUDE.md`のSpecBind管理ブロック | SpecBind | マーカー内だけを更新し、周囲の文章を保持 |
| `.specbind/settings/`以下のテンプレート、ルール、アダプター | プロジェクト | 既存ファイルを上書きしない。新しく追加された既定ファイルがなければ作成 |
| Specs、Roadmap、Gate、リリース履歴 | プロジェクト | `specbind install`では変更しない |

製品管理対象のスキルを直接編集する方法は、サポートされているカスタマイズでは
ありません。製品管理対象に未コミットの変更がある場合、SpecBindはその内容を推測して
上書きせず、競合するパスを表示して停止します。そのパスをコミット、スタッシュ、または
別の方法で解消してから計画をやり直してください。SpecBindが代わりにスタッシュを作る
ことはありません。継続して必要な方針は、製品管理対象のスキル内ではなく、プロジェクトが
所有する設定へ移してください。

## mise以外でインストールした場合

バイナリを導入した方法と同じインストーラーを再実行して更新します。利用できる
インストーラーと対応環境は
[READMEのInstall the CLI](https://github.com/Huruikagi/specbind#install-the-cli)を
参照してください。バイナリを更新したあとの`specbind install --dry-run`と
`specbind install`は、miseの場合と同じです。

---

[ユーザーガイド](../index.md) | [SpecBindをインストールする](./install.md) | [エージェントの削除とアンインストール](./uninstall.md)
