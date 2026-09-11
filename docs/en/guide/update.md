# Update SpecBind

Updating SpecBind has two parts: updating the `specbind` executable on your
machine and refreshing the product-managed files installed in each project.
Normally you ask the `sb-configure` Skill to do both.

SpecBind does not provide a self-update command that replaces its own
executable. The installation client that placed the binary, normally mise,
updates it.

## 1. Prepare the repository

Check the repository state:

```sh
git status --short
```

The update stops before starting when any of these remain. Resolve them first,
for example by committing through your normal workflow:

- staged changes, renames, copies, or dirty submodules;
- uncommitted changes to `mise.toml` or `mise.lock`; or
- uncommitted changes to product-managed targets such as `.agents/skills/sb-*`.

Other unstaged changes and untracked files may remain. SpecBind reports those
paths and preserves them without modifying or staging them.

## 2. Ask sb-configure to update

Open a coding-agent session in the project and request the update explicitly:

```text
$sb-configure Update SpecBind to the latest version.
```

To move to a specific version, or when `mise.toml` pins an exact version,
name the target in the request, for example `Update SpecBind to 1.5.1.` An
exact pin never advances unless you name the target.

`sb-configure` then:

1. records the current version and confirms that this project's mise
   configuration owns SpecBind;
2. updates the binary with `mise upgrade`, or `mise use` for an explicit
   version, and reviews the `mise.toml` and `mise.lock` changes;
3. following the Git adapter, commits only the binary selection as the first
   checkpoint;
4. presents the migration plan and the `specbind install --dry-run` plan, then
   refreshes product-managed files;
5. rereads the newly installed `sb-configure`, verifies the refresh, and
   commits the product-managed changes as a second checkpoint; and
6. presents the remaining migration-plan entries one at a time.

The two checkpoints keep the binary selection and the project-file refresh
separately reviewable and reversible. An update request never includes a push
or any release action.

### Decide on migration-plan entries

The migration plan can include recommended adjustments to existing artifacts
for the new version. `sb-configure` previews each target and asks before
editing.

- Pending `required` entries prevent reporting migration work complete.
- You may decline presented `recommended` or `optional` entries.
- A `blocked` entry resumes after its reported input fault is resolved.

The update request itself does not authorize rewriting project-owned
artifacts. Edits happen only within the scope you confirm.

## 3. Review the result

`sb-configure` finishes by reporting:

- the old and new binary versions;
- the outcome of each checkpoint;
- retained project-owned settings and preserved unrelated changes;
- declined migration entries and remaining targets; and
- where and why the run stopped, if it did.

If the Git adapter does not authorize commits, review the reported changes and
commit them through the project's normal workflow.

Other team members who pull the updated project can run `mise install` to get
the binary pinned by the lockfile. They receive the refreshed project files
through Git and do not need to repeat the update.

!!! info "If the update is interrupted"
    The migration plan starts from the pre-update version. When resuming after
    an interrupted session, tell `sb-configure` that original version. It does
    not substitute the current version or guess.

## Version-specific migration entries {#migration-items}

### 1.5.0: add `description` metadata

This recommended entry adds `description` metadata at the 1.5.0 boundary for
Requirements, Design, Steering, and their owned templates. Each sentence names
the document's lasting responsibility. Existing major-one documents remain
valid without the field. Template changes do not rewrite live artifacts, and
metadata completion is not approval or freshness evidence.

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

### Before retiring Requirements

Existing Requirements need no ID migration. Before using
[Requirement retirement](./implement-step-by-step.md#retire-requirements),
refresh installed Skills and any customized template instructions. Older
binaries cannot interpret `_Retired_` markers correctly, so do not downgrade
afterward.

## What changes and what is retained

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

## Exception: update manually

Update manually instead of through `sb-configure` when:

- the binary was not installed with mise;
- `sb-configure` stopped because the project's mise configuration does not own
  SpecBind; or
- you want to update without an Agent.

### Update manually with mise

1. Run `specbind --version` and record the original version.
2. Update the binary. To change an exact pin, use
   `mise use github:Huruikagi/specbind@<version>` instead of the second line.

    ```sh
    specbind --version
    mise upgrade github:Huruikagi/specbind
    specbind --version
    ```

    When `mise.toml` selects `latest`, mise chooses the newest stable release
    that satisfies its `minimum_release_age` setting. See mise's
    [`upgrade`](https://mise.jdx.dev/cli/upgrade.html) and
    [`mise.lock`](https://mise.jdx.dev/dev-tools/mise-lock.html) documentation
    for the exact behavior.

3. Review `mise.toml` and `mise.lock` and commit them separately from other
   changes. Do not run the next `specbind install` before this commit.
4. Inspect the migration plan and the product-asset refresh plan:

    ```sh
    specbind migration plan --from <previous-version> --json
    specbind install --dry-run
    ```

    The target defaults to the running binary version, and planning changes no
    files. Review the `create`, `replace`, `keep`, and retired-target `remove`
    actions. If the plan reports `Unrelated changes: preserved`, confirm those
    are your existing changes and do not overlap an update target.

5. Apply the refresh, review the diff, and commit it:

    ```sh
    specbind install
    git status --short
    git diff
    ```

6. Handle remaining migration-plan entries using
   [Version-specific migration entries](#migration-items).

### If mise did not install the binary

Update the binary by rerunning the same installer used to place it. See
[Install the CLI in the README](https://github.com/Huruikagi/specbind#install-the-cli)
for the available installers and supported environments. After updating the
binary, continue from step 4 above.

---

[User guide](../index.md) | [Install SpecBind](./install.md) | [Remove an Agent or uninstall](./uninstall.md)
