# Explicit SpecBind update

Load this procedure only when the maintainer explicitly asks to update the
SpecBind binary, change its mise-selected version, or refresh product-managed
project assets as part of that update. Never turn an ordinary configuration run
into an update check, and never perform a background network operation.

The installation client owns the binary. This procedure coordinates mise and
the existing `specbind install` refresh; it does not add or simulate a SpecBind
self-update command.

## Preflight and ownership proof

The common `sb-configure` preamble and language policy still apply. Before any
mutation, record the current binary version and inspect the complete repository
and mise state:

```sh
specbind --version
git status --short
mise config ls
mise tool github:Huruikagi/specbind --json
```

Inspect the reported `backend`, `requested_versions`, `active_versions`, and
`config_source`, then read the named active configuration file. Continue only
when the selected project configuration proves that its active SpecBind tool is
the `github:Huruikagi/specbind` backend. The executable name, its location on
`PATH`, a globally installed copy, or an unrelated parent or sibling config is
not ownership proof.

Stop before mutation when the worktree already contains any change. Report the
exact paths; never hide, stash, commit, or absorb unrelated work to manufacture
a clean update boundary. Also stop when the active config is untrusted,
ambiguous, outside the selected project's intended configuration, or does not
use the required GitHub backend. Route the maintainer to the installation client
that originally installed the binary and the public update guide instead of
guessing or silently selecting another executable.

Read the active Git adapter before changing mise state:

```sh
specbind adapter read git --for consume
```

Either `NO_CHANGE` result means there is no adapter-directed checkpoint.
It controls each narrow local checkpoint below. It does not authorize push,
branch changes, tags, release publication, deployment, destructive removal, or
history rewriting.

## Select and update the binary

Preserve the configured selector. When it is a moving selector, use:

```sh
mise upgrade github:Huruikagi/specbind
```

An exact pin does not advance without an explicit target from the maintainer.
Do not infer `latest`, select a prerelease, downgrade, or weaken mise trust,
checksum, lockfile, or `minimum_release_age` policy. Once an exact target is
explicitly authorized, use:

```sh
mise use github:Huruikagi/specbind@<version>
```

Verify the selected executable immediately:

```sh
specbind --version
mise tool github:Huruikagi/specbind --json
git status --short
git diff -- mise.toml mise.lock
```

Compare the old and new versions and confirm that the active backend and
configuration source are unchanged. Review only the applicable `mise.toml` and
`mise.lock` changes. Preserve every unrelated file and setting.

If the active Git adapter requires a local checkpoint, stage only the changed
mise selection files and create the binary-selection checkpoint. If it does not
authorize that checkpoint and those files changed, stop and report that the
project's own Git workflow must restore the clean-worktree precondition. Never
fold the later asset refresh into this checkpoint.

Confirm that the worktree is clean before continuing:

```sh
git status --short
```

## Preview and apply the project refresh

Use the newly selected binary to preview the complete product-asset plan:

```sh
specbind install --dry-run
```

Present every reported `create`, `replace`, `keep`, and `remove` action. An
explicit request for this update workflow authorizes applying that exact
guarded refresh plan, including its presented retired product-target removals,
subject to the existing install guards. It does not authorize destructive
removal outside that plan or overwriting project-owned templates, Rules,
adapters, Steering, Specs, lifecycle state, or release history. Stop if the
plan or repository guard reports a dirty managed target, unrelated state, or a
boundary outside the presented plan.

Apply the reviewed plan:

```sh
specbind install
```

## Mandatory post-replacement reload

`specbind install` may have replaced the package that is directing this run.
Immediately read these three files again from the newly installed package for
the active Agent:

- Codex or Generic: `.agents/skills/sb-configure/SKILL.md`,
  `.agents/skills/sb-configure/references/update.md`, and
  `.agents/skills/sb-configure/references/aftercare.md`;
- Claude Code: `.claude/skills/sb-configure/SKILL.md`,
  `.claude/skills/sb-configure/references/update.md`, and
  `.claude/skills/sb-configure/references/aftercare.md`.

Resume at **Post-refresh continuation** in the newly read update reference. Do
not rerun the binary mutation or finish from cached pre-update instructions. If
any required new file cannot be read, stop and report the old and new binary
versions plus the exact applied asset state; do not improvise aftercare from
the old package.

## Post-refresh continuation

When resuming immediately after the mandatory reload, do not repeat the
preflight or update operation. Verify the refreshed installation and inspect its
exact diff:

```sh
specbind configuration show
specbind install --dry-run
git status --short
git diff
```

The second dry run must report no pending product-managed update. Confirm that
project-owned settings and durable artifacts were retained. Follow the freshly
read aftercare reference, including any procedure-specific checks it requires.

Treat the project-asset refresh as a second workflow unit. When the active Git
adapter requires it, stage only the exact refresh paths and create a separate
asset-refresh checkpoint. Do not include a binary-selection file already
recorded in the first checkpoint or any unrelated path.

Report the old and new binary versions, the binary-selection and asset-refresh
checkpoint outcomes separately, the retained project-owned settings, refresh
verification, and any installation-client, Git, or external boundary where the
run stopped. Updating never authorizes a push or any release action.
