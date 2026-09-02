# SpecBindをインストールする

このページでは、`specbind` CLIの導入と、プロジェクトへのSpecBind一式の配置を
まとめて説明します。新規プロジェクト・既存プロジェクトのどちらのルートでも、
最初に一度だけこの手順を行います。

- 新規プロジェクトでは、[プロジェクトの土台をコミット](./start-new-project.md)
  してからこのページへ進みます。
- 既存プロジェクトでは、未コミットの変更を先にいつもの手順でコミットしておきます。

## 1. CLIをインストールする

対象プロジェクトのルートで、[mise](https://mise.jdx.dev/)を使ってSpecBindを
インストールします。

```sh
mise use github:Huruikagi/specbind
mise lock
```

`mise use`はSpecBindを`mise.toml`へ追加し、`mise lock`は選ばれたバージョンと
配布物のチェックサムを`mise.lock`へ記録します。

インストールできたか確認します。

```sh
specbind --version
```

追加または更新された`mise.toml`と`mise.lock`の内容を確認し、どちらもGitへ
コミットしてください。これにより、チームで同じバージョンのSpecBindを使えます。

miseを使わないインストール方法は、
[README](https://github.com/Huruikagi/specbind#install-the-cli)を参照してください。

## 2. プロジェクトへSpecBindを導入する

今インストールしたCLIの最初の仕事として、SpecBindがこれから使うエージェント
スキルや設定ファイルをプロジェクトに配置します。

このガイドではCodexを例に進めます。

```sh
specbind install --agent codex --language ja --project-instructions
```

### `--agent`

使うコーディングエージェントに合わせて、`--agent`の値を選んでください。

| 使うエージェント | 指定する値 |
| --- | --- |
| Codex | `codex` |
| Claude Code | `claude-code` |
| Agent Skillsと`AGENTS.md`に対応するその他のエージェント | `generic` |

複数のエージェントを使う場合は、`--agent codex --agent claude-code`のように
`--agent`を繰り返します。

`generic`が作るのは、`.agents/skills/`のAgent Skillsとルート`AGENTS.md`の
案内ブロックだけです。サブエージェント定義は作りません。

### `--language ja`

SpecBindが管理する成果物、具体的には`requirements.md`や`design.md`の言語を
日本語にします。

### `--project-instructions`

`AGENTS.md`または`CLAUDE.md`に、マーカーで囲んだSpecBindの案内ブロックを
追加します。もともと書いてある既存の文章はそのまま残ります。普通はつけたほうが
いいでしょう。

## 3. 書き込まれる内容を確認する

同じコマンドへ`--dry-run`を追加すると、変更を適用せずに`create`、`replace`、
`keep`と、廃止された製品管理対象に対する`remove`の計画を確認できます。

```sh
specbind install --dry-run --agent codex --language ja --project-instructions
```

主に、次のファイルが追加されます。

```text
.specbind.json
.specbind/settings/
.agents/skills/sb-*/             # Codexとgenericで共有
.codex/agents/specbind-*.toml    # Codexの役割別モデル設定
.claude/skills/sb-*/             # Claude Code
.claude/agents/specbind-*.md     # Claude Codeの役割別モデル設定
AGENTS.md / CLAUDE.md            # 指示の統合を有効にした場合
```

CodexとClaude Codeには、役割ごとに使うモデルの既定値も設定されます。変更する
場合は、[カスタマイズ](./customization.md)の「プロジェクト設定と役割別モデル」を
参照してください。

## 4. コミットしてセッションを開き直す

生成された内容をレビューし、ほかの変更とは分けてコミットしてください。SpecBindの
インストーラ自体はコミットを行いません。

そのあと、対象プロジェクトでコーディングエージェントのセッションを開き直して
ください。そうしないと、エージェントが新しいスキルを認識できないことがあります。

!!! info "スキルの呼び出し表記"
    このガイドのスキル呼び出しはCodexの表記で示します。Claude Codeでは、先頭の
    `$`を`/`に読み替えてください。スキル名と引数は同じです。`generic`を選んだ
    場合も`sb-*`というスキル名は同じですが、呼び出し方はエージェントごとに
    異なります。利用するエージェントのスキル選択または自動Discoveryの方法に
    読み替えてください。

## 次に読む

- [新規プロジェクトで始める](./start-new-project.md) — まだ実装がない場合
- [既存プロジェクトで始める](./start-existing-project.md) — すでにコードがある場合
- [SpecBindをアップデートする](./update.md) — 導入後にバイナリと製品管理対象を更新する
- [カスタマイズ](./customization.md) — 既定値が合わないと分かってから

---

[ユーザーガイド](../index.md) | [新規プロジェクトで始める](./start-new-project.md) | [既存プロジェクトで始める](./start-existing-project.md)
