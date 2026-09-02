# Install SpecBind

This page covers installing the `specbind` CLI and installing SpecBind into a
project. Both the new-project and existing-project routes perform this step
once, before their first Discovery.

- For a new project, [commit the project baseline](./start-new-project.md)
  first, then return here.
- For an existing project, commit any pending work with your normal workflow
  first.

## 1. Install the CLI

From the project root, install SpecBind with [mise](https://mise.jdx.dev/):

```sh
mise use github:Huruikagi/specbind
mise lock
specbind --version
```

`mise use` records SpecBind in `mise.toml`; `mise lock` records the selected
version and distribution checksum in `mise.lock`. Review and commit both files
so the team uses the same version. See the repository
[README](https://github.com/Huruikagi/specbind#install-the-cli) for other
installation methods.

## 2. Install SpecBind into the project

The CLI's first job is to place the Agent Skills and configuration files
SpecBind uses. This guide uses Codex and English artifacts:

```sh
specbind install --agent codex --language en --project-instructions
```

| Coding agent | `--agent` value |
| --- | --- |
| Codex | `codex` |
| Claude Code | `claude-code` |
| Another Agent supporting Agent Skills and `AGENTS.md` | `generic` |

Repeat `--agent` to select more than one. `generic` installs shared
`.agents/skills/` Skills and the managed `AGENTS.md` block, but no subagent role
definitions.

`--language en` selects the language of managed artifacts such as
`requirements.md` and `design.md`. `--project-instructions` adds a marked
SpecBind block to `AGENTS.md` or `CLAUDE.md` without changing text outside the
marker.

## 3. Preview what will be written

Add `--dry-run` to the same command to see the `create`, `replace`, `keep`, and
retired-product-asset `remove` plan without applying it:

```sh
specbind install --dry-run --agent codex --language en --project-instructions
```

The main installed surfaces are:

```text
.specbind.json
.specbind/settings/
.agents/skills/sb-*/             # shared by Codex and generic
.codex/agents/specbind-*.toml    # Codex role configuration
.claude/skills/sb-*/             # Claude Code
.claude/agents/specbind-*.md     # Claude Code role configuration
AGENTS.md / CLAUDE.md            # when project instructions are enabled
```

Codex and Claude Code also receive default per-role models. To change them, see
"Project configuration and per-role models" in
[Customize SpecBind](./customization.md).

## 4. Commit and reopen the session

Review the installed files and commit them separately from your other changes.
The installer does not commit.

Then reopen the coding-agent session so it discovers the new Skills.

!!! info "Skill invocation syntax"
    The examples in this guide use Codex `$skill` syntax. Claude Code uses
    `/skill` with the same Skill names and arguments. A `generic` host installs
    the same `sb-*` Skills; use that host's Skill selection or automatic
    discovery mechanism.

## Next

- [Start a new project](./start-new-project.md) — no implementation yet
- [Start with an existing project](./start-existing-project.md) — code already exists
- [Update SpecBind](./update.md) — refresh the binary and product-managed files later
- [Customize SpecBind](./customization.md) — once a default proves wrong for you

---

[User guide](../index.md) | [Start a new project](./start-new-project.md) | [Start with an existing project](./start-existing-project.md)
