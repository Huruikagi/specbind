# Update SpecBind

Updating SpecBind has two separate parts: updating the `specbind` executable on
your machine and refreshing the product-managed files installed in each
project.

This guide uses mise as the primary binary manager. SpecBind does not provide a
self-update command that replaces its own executable.

## 1. Update the binary with mise

Run `specbind --version` before updating and retain the original version. Use
that same value as the migration starting point after an interrupted update.

From the project root, upgrade to the newest version allowed by the configured
version selector:

```sh
mise upgrade github:Huruikagi/specbind
specbind --version
```

When `mise.toml` selects `latest`, mise chooses the newest stable release that
satisfies its `minimum_release_age` setting. An exactly pinned version does not
advance through an ordinary `mise upgrade`. Select an exact target explicitly
when you intend to change that pin:

```sh
mise use github:Huruikagi/specbind@<version>
specbind --version
```

If `mise.toml` or `mise.lock` changed, review both files and commit them through
the project's normal workflow. `mise.lock` keeps the selected version and
distribution inputs reproducible for the team and CI.
See mise's [`upgrade`](https://mise.jdx.dev/cli/upgrade.html) and
[`mise.lock`](https://mise.jdx.dev/dev-tools/mise-lock.html) documentation for
the installation client's exact behavior.

!!! warning "Commit the binary selection before refreshing project assets"
    When the next `specbind install` plan replaces, moves, or removes existing
    files, SpecBind requires a repository with at least one commit and
    Git-clean mutation targets. If mise changed `mise.toml` or `mise.lock`,
    commit that change before continuing. Unstaged or untracked work outside
    the refresh targets may remain; SpecBind reports and preserves those paths.
    Pre-existing staged changes, renames, copies, and dirty submodules still
    stop the coordinated update because its two checkpoints cannot be isolated.

## 2. Refresh product-managed project files

Use the new CLI to inspect project migration work from the original version:

```sh
specbind migration plan --from <previous-version> --json
```

The target defaults to the running binary version. An explicit `--to` is also
accepted within catalog coverage; future unknown targets and downgrades fail.
Planning changes no files. Repeating the same command re-evaluates remaining
work against the current files.

The new binary embeds the Skills and other product-managed assets for that
version. Inspect the project refresh plan first:

```sh
git status --short
specbind install --dry-run
```

Review the reported `create`, `replace`, and `keep` actions, plus any `remove`
actions for retired product-managed targets. If the plan reports `Unrelated
changes: preserved`, confirm those are your existing changes and do not overlap
an update target. Then apply the plan:

```sh
specbind install
git status --short
git diff
```

Review and commit the resulting project changes through the normal workflow.
Other team members who pull the updated project can run `mise install` to get
the binary pinned by the lockfile. They receive the refreshed project files
through Git and do not all need to rerun `specbind install`.

## What changes and what is retained

For an agent-driven update, reload the newly installed `sb-configure` package
after asset refresh before reading the plan's procedures. Pending `required`
entries prevent reporting migration work complete. You may decline presented
`recommended` or `optional` work. A `blocked` probe requires resolution of its
reported input fault.

The first entries introduce recommended `description` reconciliation at the
1.5.0 boundary for Requirements, Design, Steering, and their owned templates.
Each sentence names the document's lasting responsibility. Existing major-one
documents remain valid without the field. The agent previews each target and
edits only separately confirmed reconciliation scope. Template changes do not
rewrite live artifacts, and metadata completion is not approval or freshness
evidence.

`spec list` shows Requirements descriptions, `artifact list <spec>` shows Design
descriptions, and `steering list` shows Steering descriptions. `missing` means
the field is absent; `invalid` means invalid metadata or Requirements validation
errors; `unavailable` means Requirements could not be resolved unambiguously.
Descriptions reflect current authored documents independently of approval state.

When creating a Design, `artifact check <spec> <selector> --template <template>`
checks the chosen scaffold's creation obligations, including literal description
inheritance. An assessed Spec-local one-off uses `--template design/main` and
provides its own responsibility sentence. This check does not establish the
provenance of an existing artifact or change its approval state.

| Target | Owner | Update behavior |
| --- | --- | --- |
| `specbind` executable | mise | Updated by `mise upgrade` or an explicit `mise use` |
| Product-managed targets such as `.agents/skills/sb-*` and `.claude/skills/sb-*` | SpecBind | Replaced with the current embedded versions by `specbind install`; retired targets are shown and removed through the plan |
| SpecBind-managed block in `AGENTS.md` or `CLAUDE.md` | SpecBind | Only the marked block is maintained; surrounding text is preserved |
| Templates, Rules, and Adapters below `.specbind/settings/` | Project | Existing files are never overwritten; newly introduced missing defaults may be created |
| Specs, Roadmap, Gates, and release history | Project | Not changed by `specbind install` |

Direct edits to product-managed Skills are not a supported customization
surface. If a refresh finds dirty managed targets, SpecBind stops instead of
guessing or overwriting them and reports the conflicting paths. Commit, stash,
or otherwise resolve those paths before planning again. SpecBind never creates
a stash for you. Move durable policy to a project-owned surface rather than
keeping it in a managed Skill.

## If mise did not install the binary

Update the binary by rerunning the same installer used to place it. See
[Install the CLI in the README](https://github.com/Huruikagi/specbind#install-the-cli)
for the available installers and supported environments. After updating the
binary, the `specbind install --dry-run` and `specbind install` project-refresh
steps are the same as the mise flow.

---

[User guide](../index.md) | [Install SpecBind](./install.md) | [Remove an Agent or uninstall](./uninstall.md)
