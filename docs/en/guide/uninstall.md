# Remove an Agent or uninstall SpecBind from a project

SpecBind distinguishes removing one coding-Agent integration from removing the
whole project integration. Both commands show a plan on the first run and make
no changes. Review the exact delete, update, and retain actions, then repeat the
command with `--apply`.

These are project operations. They do not remove the machine-level `specbind`
binary, PATH entries, or package-manager configuration such as mise.

## Remove one Agent integration

For example, inspect the Codex removal plan:

```powershell
specbind remove-agent codex
```

The plan identifies the exact Codex Skills, five role definitions, marked
`AGENTS.md` block, and `.specbind.json` update. Claude Code integration,
Specs and settings below `.specbind/`, and text outside the managed marker are
retained. Shared `.agents/skills/` and `AGENTS.md` targets are retained when
another selected Agent still needs them.

Apply the reviewed plan with:

```powershell
specbind remove-agent codex --apply
```

Use `claude-code` for Claude Code and `generic` for the shared integration.
The last Agent cannot be removed with this command; use project uninstall and
explicitly choose what happens to durable knowledge.

## Uninstall the project integration

### Retain Specs and history

```powershell
specbind uninstall --knowledge retain
specbind uninstall --knowledge retain --apply
```

`retain` removes managed Agent integration, role files, root instruction
blocks, and `.specbind.json`, while retaining the configured `specDir`. Use it
when migrating workflows, planning to reinstall later, or keeping
Requirements, Design, Contract, Steering, logs, and release history as ordinary
project documentation.

### Remove Specs and history

```powershell
specbind uninstall --knowledge remove
specbind uninstall --knowledge remove --apply
```

`remove` also deletes the exact configured `specDir` as a durable-knowledge
bundle. It is allowed only when everything below that directory is tracked by
Git, the repository is clean, and it contains no ignored or untracked files,
symlinks, junctions, or reparse points. The deleted content can then be restored
from the commit before uninstall.

```powershell
git restore --source=HEAD -- .
```

!!! warning
    This example restores the whole worktree to `HEAD`. Use it only immediately
    after uninstall, before making other edits. Otherwise restore the exact
    paths listed by the plan from the appropriate earlier revision.

## If the operation stops

If the plan or apply reports dirty, untracked, ignored, link-like, or malformed
marker content, SpecBind does not guess or force deletion. Inspect the reported
path, commit or move anything you need to retain, and rerun the same plan.

Apply is retry-safe. Already removed exact targets are recognized as absent,
and `.specbind.json` is updated or removed last as the completion marker.
