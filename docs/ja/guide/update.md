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

!!! warning "プロジェクト資産を更新する前にコミットする"
    次の`specbind install`は、既存ファイルの置換・移動・削除を含む場合、1件以上の
    コミットがあり、作業ツリーに未コミットの変更がないことを要求します。miseによる更新で
    `mise.toml`や`mise.lock`が変更された場合、その差分をコミットしてから進んで
    ください。

## 2. プロジェクト内の製品管理対象を更新する

新しいバイナリには、そのバージョンのSkillやほかの製品管理対象が埋め込まれています。
まず、適用される計画を確認します。

```sh
git status --short
specbind install --dry-run
```

`create`、`replace`、`keep`と、廃止された製品管理対象に対する`remove`を確認してから
適用します。

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
上書きせず停止します。必要な方針をプロジェクトが所有する設定へ移すか、Gitで管理対象を
元に戻してから計画をやり直してください。

## mise以外でインストールした場合

バイナリを導入した方法と同じインストーラーを再実行して更新します。利用できる
インストーラーと対応環境は
[READMEのInstall the CLI](https://github.com/Huruikagi/specbind#install-the-cli)を
参照してください。バイナリを更新したあとの`specbind install --dry-run`と
`specbind install`は、miseの場合と同じです。

---

[ユーザーガイド](../index.md) | [SpecBindをインストールする](./install.md) | [エージェントの削除とアンインストール](./uninstall.md)
