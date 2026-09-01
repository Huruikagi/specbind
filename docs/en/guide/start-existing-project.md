# Start with an existing project

This route installs SpecBind into a repository that already contains code or
tests and takes a first change through planning, implementation, and validation.
If implementation has not begun, use [Start a new project](./start-new-project.md).

## 1. Install the CLI

Commit existing work, then install from the project root:

```sh
mise use github:Huruikagi/specbind
mise lock
specbind --version
```

Review and commit `mise.toml` and `mise.lock` so the selected version and
checksum are shared. See the repository
[README](https://github.com/Huruikagi/specbind#install-the-cli) for other
installation methods.

## 2. Install SpecBind into the project

This guide uses Codex and English artifacts:

```sh
specbind install --agent codex --language en --project-instructions
```

| Coding agent | `--agent` value |
| --- | --- |
| Codex | `codex` |
| Claude Code | `claude-code` |
| Another Agent supporting Agent Skills and `AGENTS.md` | `generic` |

Repeat `--agent` for multiple Agents. `generic` installs shared Agent Skills
and the managed `AGENTS.md` block but no role definitions. The language option
selects the language of managed artifacts such as Requirements and Design.
Project instructions are inserted only inside a managed marker.

Preview the exact plan if desired:

```sh
specbind install --dry-run --agent codex --language en --project-instructions
```

Review and commit the installed files; the installer does not commit. Reopen
the coding-agent session so it discovers the Skills. Examples below use Codex
`$skill` syntax; Claude Code uses `/skill`.

## 3. Keep the defaults for the first cycle

Although installation suggests `specbind-configure`, first complete one real
change with the defaults. Adjust only demonstrated mismatches afterward using
[Customize SpecBind](./customization.md).

## 4. Choose the adoption route

| Goal | Route |
| --- | --- |
| Use SpecBind for the next change | Continue on this page |
| Establish Specs from working implementation | [Establish Specs from an existing implementation](./adopt-existing.md) |

The existing-implementation route of `specbind-discovery` does not treat current
code as automatically correct. It investigates code and tests as evidence,
confirms the intent to preserve, and then hands it to the ordinary lifecycle.
Adoption requires durable Steering; bootstrap it first with
`specbind-steering` when absent.

## 5. Choose the first change

Start with one small behavior rather than several features or release work. For
example:

> Allow users to download the contents of the list screen as a CSV file.

If no existing Spec owns this behavior, it likely introduces one new durable
responsibility. CSV columns and format can also become a Contract on which
other capabilities or consumers depend.

## 6. Confirm scope with Discovery

Describe the change and point to any relevant Issue or notes:

```text
$specbind-discovery Allow users to download the contents of the list screen as
a CSV file.
```

Discovery reads current Spec, Steering, and Milestone state and classifies the
work as Direct, an existing-Spec update, or a new Spec. It presents work items,
new Specs, Gate invalidations, and dependencies. Review the classification and
responsibility boundary before approval. The CLI then creates lifecycle state
and the Agent writes a concise `brief.md`.

Use `$specbind-status` for a read-only explanation at any point.

## 7. Choose how to plan and implement

For the first `csv-export` change, you can inspect each boundary explicitly:

```text
$specbind-plan csv-export requirements
$specbind-plan csv-export design
$specbind-contract-review
$specbind-plan csv-export tasks
$specbind-implement csv-export
$specbind-validate-implementation csv-export
```

[Plan and implement one item at a time](./implement-step-by-step.md) explains
what to review and approve at every step, including upstream rewinds.

For a Milestone with several Specs or Direct items, use
`$specbind-plan --all` followed by `$specbind-drive`. See
[Plan and Drive a Milestone](./implement-with-plan-and-drive.md) for attention,
continuation, and stopping behavior. Both routes stop before Release.

## 8. Inspect the artifacts

```text
.specbind/
├─ steering/roadmap.md
└─ specs/csv-export/
   ├─ spec.yaml
   ├─ brief.md
   ├─ requirements.md
   ├─ design.md
   ├─ contract.yaml
   └─ tasks.yaml
```

The CLI owns lifecycle state in `spec.yaml`, `roadmap.md`, and `tasks.yaml`.
Do not hand-edit them to advance state. Read current state directly with:

```sh
specbind milestone status
specbind spec status csv-export
specbind tasks list csv-export
specbind artifact list csv-export
```

The two status commands also support `--json` for integrations.

## Next

- [Core concepts](./concepts.md)
- [Plan and implement one item at a time](./implement-step-by-step.md)
- [Plan and Drive a Milestone](./implement-with-plan-and-drive.md)
- [Release a milestone](./release.md)
- [Customize SpecBind](./customization.md)
- [Current generated skill index](../reference/current-skill-index.md)
- [Current generated artifact index](../reference/current-artifact-index.md)
