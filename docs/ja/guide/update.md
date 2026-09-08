# SpecBindをアップデートする

SpecBindのアップデートには、PCで実行する`specbind`バイナリの更新と、
各プロジェクトに配置された製品管理対象ファイルの更新があります。この2つは
別々の操作です。

このガイドでは、バイナリをmiseで管理している場合を基本に説明します。
`specbind`自身がバイナリを置き換える自己更新コマンドはありません。

## 1. miseでバイナリを更新する

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

新しいバイナリには、そのバージョンのSkillやほかの製品管理対象が埋め込まれています。
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

## 更新されるものと保持されるもの

| 対象 | 所有者 | アップデート時の扱い |
| --- | --- | --- |
| `specbind`バイナリ | mise | `mise upgrade`または明示的な`mise use`で更新 |
| `.agents/skills/sb-*`、`.claude/skills/sb-*`などの製品管理対象 | SpecBind | `specbind install`で現在の埋め込み版へ置換。廃止された対象は計画に表示して削除 |
| `AGENTS.md`または`CLAUDE.md`のSpecBind管理ブロック | SpecBind | マーカー内だけを更新し、周囲の文章を保持 |
| `.specbind/settings/`以下のテンプレート、Rule、Adapter | プロジェクト | 既存ファイルを上書きしない。新しく追加された既定ファイルがなければ作成 |
| Specs、Roadmap、Gate、リリース履歴 | プロジェクト | `specbind install`では変更しない |

製品管理対象のSkillを直接編集する方法は、サポートされているカスタマイズでは
ありません。製品管理対象に未コミットの変更がある場合、SpecBindはその内容を推測して
上書きせず、競合するパスを表示して停止します。そのパスをコミット、スタッシュ、または
別の方法で解消してから計画をやり直してください。SpecBindが代わりにスタッシュを作る
ことはありません。継続して必要な方針は、製品管理対象のSkill内ではなく、プロジェクトが
所有する設定へ移してください。

## mise以外でインストールした場合

バイナリを導入した方法と同じインストーラーを再実行して更新します。利用できる
インストーラーと対応環境は
[READMEのInstall the CLI](https://github.com/Huruikagi/specbind#install-the-cli)を
参照してください。バイナリを更新したあとの`specbind install --dry-run`と
`specbind install`は、miseの場合と同じです。

---

[ユーザーガイド](../index.md) | [SpecBindをインストールする](./install.md) | [エージェントの削除とアンインストール](./uninstall.md)
