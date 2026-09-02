# エージェントの削除とアンインストール

SpecBindには、1つのコーディングエージェント連携だけを外す操作と、プロジェクトから
SpecBind連携全体を外す操作があります。どちらも最初の実行では計画を表示するだけで、
ファイルを変更しません。表示された削除・更新・保持対象を確認してから
`--apply`を付けて再実行します。

これらはプロジェクト内の操作です。PCにインストールされた`specbind`バイナリ、
PATH、miseなどのパッケージマネージャー設定は削除しません。

## 1つのエージェント連携だけを外す

たとえばCodex連携の削除計画を確認します。

```sh
specbind remove-agent codex
```

計画には、Codex用の製品管理のスキル、5つの役割定義、`AGENTS.md`の
マーカーで囲まれたブロック、`.specbind.json`の更新が正確なパスで表示されます。
Claude Code連携、`.specbind/`以下のSpecsや設定、マーカー外の
`AGENTS.md`本文は保持されます。

`generic`も選ばれている場合、Codexと共有する`.agents/skills/`以下の管理対象スキルと
`AGENTS.md`の管理ブロックは`retain`として表示され、Codex固有の役割定義だけが削除
されます。逆に`generic`を外してCodexが残る場合も、共有対象は保持され、設定だけが
更新されます。削除後に残るエージェントのどれも必要としない対象だけが削除されます。

内容を確認したら適用します。

```sh
specbind remove-agent codex --apply
```

Claude Codeを外す場合は`claude-code`、共通形式の連携を外す場合は`generic`を指定
します。最後の1エージェントは、このコマンドでは削除できません。プロジェクト全体の
アンインストールを使い、永続知識を残すか削除するか明示してください。

## SpecBind連携全体を外す

アンインストールでは、設定された`specDir`をどう扱うか必ず選びます。
通常の新規プロジェクトでは`specDir`は`.specbind`です。

### Specsや履歴を残す

```sh
specbind uninstall --knowledge retain
specbind uninstall --knowledge retain --apply
```

`retain`はエージェント用スキル、役割定義、ルート指示ブロック、`.specbind.json`を
削除しますが、設定されていた`specDir`全体を残します。別のワークフローへ移行する
とき、後でSpecBindを再導入するとき、Requirements、Design、Contract、Steering、
ログ、リリース履歴を通常のプロジェクト文書として参照し続けるときに使います。

### Specsや履歴も削除する

```sh
specbind uninstall --knowledge remove
specbind uninstall --knowledge remove --apply
```

`remove`はエージェント連携に加えて、設定された`specDir`を永続知識一式として
削除します。プロジェクトが所有する設定、Specs、進行中のRoadmap、レビュー状態、
ログ、リリース履歴も含まれます。

この操作は、配下がすべてGit管理され、リポジトリに未コミットの変更がなく、Gitの
無視対象や未追跡ファイル、シンボリックリンク、ジャンクション、再解析ポイントが
ない場合だけ実行できます。削除前のコミットから復元できます。たとえば既定構成なら、
削除直後、ほかの編集を始める前に次のように連携全体を戻せます。

```sh
git restore --source=HEAD -- .
```

!!! warning
    このコマンドは作業ツリー全体をHEADへ戻すため、アンインストール後に別の編集を
    始めた場合は使わず、計画に表示された正確なパスを個別に復元してください。
    すでにアンインストールをコミットした後は、`HEAD^`のようにアンインストール前
    のリビジョンと復元するパスを明示します。

## 停止したとき

計画または適用が`dirty`、`untracked`、`ignored`、`link-like`、マーカー不正などを
報告した場合、SpecBindは対象を推測したり強制削除したりしません。診断に表示された
パスを確認し、残す内容をコミットまたは別の場所へ移してから、同じ計画を再実行して
ください。

適用が途中で停止した場合も、すでに削除済みの対象は`absent`として認識されます。
`.specbind.json`は最後に更新または削除される完了マーカーなので、同じコマンドを
再実行して残りの計画を確認できます。

---

[ユーザーガイド](../index.md) | [カスタマイズ](./customization.md) | [バグ報告と改善提案](./feedback.md)
