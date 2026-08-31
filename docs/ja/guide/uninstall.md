# Agentの削除とプロジェクトのアンインストール

SpecBindには、1つのcoding agent連携だけを外す操作と、プロジェクトから
SpecBind連携全体を外す操作があります。どちらも最初の実行はplan表示だけで、
ファイルを変更しません。表示された削除・更新・保持対象を確認してから
`--apply`を付けて再実行します。

これらはプロジェクト内の操作です。PCにインストールされた`specbind` binary、
PATH、miseなどのpackage manager設定は削除しません。

## 1つのAgent連携だけを外す

たとえばCodex連携のplanを確認します。

```powershell
specbind remove-agent codex
```

planには、Codex用の製品管理のスキル、5つのrole定義、`AGENTS.md`
のmarked block、`.specbind.json`の更新がexact pathで表示されます。
Claude Code連携、`.specbind/`以下のSpecsやsettings、marker外の
`AGENTS.md`本文は保持されます。

`generic`も選ばれている場合、Codexと共有する`.agents/skills/`以下の管理対象スキルと
`AGENTS.md`のmarked blockは`retain`として表示され、Codex固有の
role定義だけが削除されます。
逆に`generic`を外してCodexが残る場合も、共有targetは保持され、設定だけが更新
されます。削除後に残るAgentのどれも必要としないexact targetだけが削除されます。

内容を確認したら適用します。

```powershell
specbind remove-agent codex --apply
```

Claude Codeを外す場合は`claude-code`、共通形式の連携を外す場合は`generic`を
指定します。最後の1 agentはこの
コマンドでは削除できません。プロジェクト全体のアンインストールを使い、
durable knowledgeを残すか削除するか明示してください。

## SpecBind連携全体を外す

アンインストールでは、設定された`specDir`をどう扱うか必ず選びます。
通常の新規projectでは`specDir`は`.specbind`です。

### Specsや履歴を残す

```powershell
specbind uninstall --knowledge retain
specbind uninstall --knowledge retain --apply
```

`retain`はagent用スキル、role定義、root instruction block、
`.specbind.json`を削除しますが、設定されていた`specDir`全体を残します。
別のworkflowへ移行するとき、後でSpecBindを再導入するとき、Requirements、
Design、Contract、Steering、log、release historyを通常のproject文書として
参照し続けるときに使います。

### Specsや履歴も削除する

```powershell
specbind uninstall --knowledge remove
specbind uninstall --knowledge remove --apply
```

`remove`はagent連携に加えて、設定されたexact `specDir`をdurable knowledge
bundleとして削除します。project-owned settings、Specs、active Roadmap、
review state、logs、release historyもbundleに含まれます。

この操作は、配下がすべてGit管理され、repositoryがcleanで、ignoredや
untrackedのファイル、symlink、junction、reparse pointがない場合だけ実行
できます。削除前のcommitから復元できます。たとえばdefault構成なら、
削除直後、ほかの編集を始める前なら次のようにintegration全体を戻せます。

```powershell
git restore --source=HEAD -- .
```

!!! warning
    このコマンドはworktree全体をHEADへ戻すため、アンインストール後に別の編集を
    始めた場合は使わず、planに表示されたexact pathを個別にrestoreしてください。
    すでにアンインストールをcommitした後は、`HEAD^`のようにアンインストール前
    のrevisionと復元するexact pathを明示します。

## 停止したとき

planまたはapplyがdirty、untracked、ignored、link-like、marker不正などを
報告した場合、SpecBindは対象を推測したりforce削除したりしません。
診断に表示されたpathを確認し、残す内容をcommitまたは別の場所へ移してから
同じplanを再実行してください。

applyが途中で停止した場合も、すでに削除済みのexact targetは`absent`として
認識されます。`.specbind.json`は最後に更新または削除されるcompletion marker
なので、同じコマンドを再実行して残りのplanを確認できます。
