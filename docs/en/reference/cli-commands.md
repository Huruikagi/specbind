# CLI commands

You normally work through Skills, which run the CLI for you. This page lists
the `specbind` commands by purpose so you can inspect state yourself or
recognize a command a Skill reports. Run `specbind <command> --help` for exact
arguments and options.

## Inspect state (read-only)

Safe to run at any time. They never change files.

| Command | Shows |
| --- | --- |
| `specbind milestone status` | Active Milestone stage, progress, next actions, and release blockers |
| `specbind spec list` | Every Spec in the project |
| `specbind spec status <spec>` | One Spec's state, freshness, coverage, and Task progress |
| `specbind tasks list <spec>` / `tasks show <spec> <task>` | Task hierarchy and progress, or one Task's details |
| `specbind artifact list <spec>` / `artifact read <spec> <selector>` | A Spec's artifacts, or one artifact's content |
| `specbind steering list` / `steering read <selector>` | Steering documents |
| `specbind configuration show` | Summary of every project configuration surface |
| `specbind release preflight` | What still blocks release |
| `specbind check traceability <spec>` | Requirement coverage by Design and Tasks |
| `specbind check contracts` | Project-wide Contract graph errors |
| `specbind contract graph` / `dependencies` / `consumers` / `owners` / `shared` | Contract relationships and file ownership |

`milestone status` and `spec status` also accept `--json` for tools and scripts.
See [Lifecycle states](./lifecycle-states.md) for the state names they report.

## Read project settings and embedded content (read-only)

| Command | Shows |
| --- | --- |
| `specbind template list` / `read` / `resolve` | Artifact templates and where a new artifact would be placed |
| `specbind rule list` / `read` | Project-owned shared rules |
| `specbind adapter list` / `read` | Project-owned operational adapters |
| `specbind protocol list` / `read` | Product protocols embedded in the binary |
| `specbind schema list` / `read` | Structured artifact schemas embedded in the binary |

See [Customize SpecBind](../guide/customization.md) for how these are used.

## Install, update, and remove

You run these yourself. Each one previews its plan before changing files.

| Command | Purpose | Guide |
| --- | --- | --- |
| `specbind install` | Install or refresh SpecBind in a project (`--dry-run` to preview) | [Install](../guide/install.md), [Update](../guide/update.md) |
| `specbind migration plan --from <version>` | Show project migration work after a binary update | [Update](../guide/update.md) |
| `specbind remove-agent <agent>` | Remove one Agent integration (`--apply` to apply) | [Remove or uninstall](../guide/uninstall.md) |
| `specbind uninstall --knowledge retain\|remove` | Remove the project integration (`--apply` to apply) | [Remove or uninstall](../guide/uninstall.md) |
| `specbind migrate cc-sdd` | Plan or apply a migration from cc-sdd | [Migrate from cc-sdd](../guide/migrate-from-cc-sdd.md) |
| `specbind feedback` | Show where to report bugs and suggest improvements | [Feedback](../guide/feedback.md) |

## Change lifecycle state (used by Skills)

These commands record approvals, progress, and releases. Skills run them after
the required reviews and checks, so you normally do not run them by hand.

| Command | Records |
| --- | --- |
| `specbind milestone create` / `update-scope` / `scope` / `rebaseline` | Milestone creation and scope changes (`sb-discovery`) |
| `specbind spec requirements` / `design` / `tasks` | Gate approval or invalidation (`sb-plan`) |
| `specbind milestone review` | Milestone-wide Contract review (`sb-contract-review`) |
| `specbind tasks complete` / `block` / `reopen` | Task progress (`sb-implement`) |
| `specbind milestone direct` | Direct item completion (`sb-implement`) |
| `specbind spec completion` | Spec completion evidence (`sb-validate-implementation`) |
| `specbind milestone bind-release <version>` | Target release binding (`sb-release`; you may also bind ahead of time) |
| `specbind release finalize` | Milestone release finalization (`sb-release`) |
| `specbind adoption preflight` / `milestone reverse` | Establishing Specs from an existing implementation (`sb-adopt`) |
| `specbind artifact check` / `steering check` | Checks on a newly created artifact during authoring |

!!! warning
    Running these commands directly bypasses the reviews the owning Skill
    performs. Use them by hand only when a guide tells you to, such as
    `specbind milestone reverse abandon` in
    [Establish Specs from an existing implementation](../guide/adopt-existing.md).
