# Start a new project

This route installs SpecBind before implementation begins, divides the first
release scope into durable responsibilities, and proceeds from scope review
through implementation validation.

See [Getting Started](./getting-started.md) for supported environments and
coding agents.

## Before you begin

SpecBind is best at turning a reasonably clear product direction into durable
specifications that drive planning, implementation, and verification. It is
not intended to discover a product from nothing. Start here when the product,
MVP, screens, capabilities, or major use cases are identifiable, even if
details, edge cases, and contracts still need refinement.

If those boundaries are still changing rapidly, prototype first and return via
the [existing-project route](./start-existing-project.md). You do not need a
complete requirements specification before installing SpecBind; existing
product notes can become Discovery input.

## 1. Establish the project baseline

Initialize the Git repository and add a short README plus any already chosen
license, language, framework, or project configuration. Discovery needs a
committed baseline for the first Milestone.

```sh
git init
git status --short
git add .
git commit -m "Initialize project"
```

Review `git status --short` before staging and follow the project's commit
policy.

## 2. Install the CLI

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

## 3. Install SpecBind into the project

This guide uses Codex and English artifacts:

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
definitions. `--language en` selects English managed artifacts.
`--project-instructions` adds a marked SpecBind block to `AGENTS.md` or
`CLAUDE.md` without changing text outside the marker.

Preview the exact `create`, `replace`, and `keep` actions first when desired:

```sh
specbind install --dry-run --agent codex --language en --project-instructions
```

The main installed surfaces are:

```text
.specbind.json
.specbind/settings/
.agents/skills/specbind-*/       # shared by Codex and generic
.codex/agents/specbind-*.toml    # Codex role configuration
.claude/skills/specbind-*/       # Claude Code
.claude/agents/specbind-*.md     # Claude Code role configuration
AGENTS.md / CLAUDE.md            # when project instructions are enabled
```

Review and commit the installation separately from the initial baseline. The
installer does not commit. Then reopen the coding-agent session so it discovers
the new Skills. The examples below use Codex `$skill` syntax; use `/skill` in
Claude Code and the equivalent invocation mechanism in a generic host.

## 4. Keep the defaults for the first cycle

The installer suggests a configuration review with `specbind-configure`.
For a new project, first complete one cycle with the defaults. After you have
real artifacts to evaluate, use [Customize SpecBind](./customization.md) to
change only the surfaces that do not fit.

## 5. Prepare the first release scope

The first scope may contain several capabilities that should ship together.
Collect the tracked text files that describe the intended product into a
project directory, for example:

```text
docs/product-definition/
├─ task-management.md
└─ reminders.md
```

Discovery inventories the whole collection, assigns every source item to a
durable Spec responsibility or records why it is not used, and determines
dependencies between the resulting work items.

If you already have stable product, technology, structure, security, or testing
guidance that should apply beyond this Milestone, establish it separately with:

```text
$specbind-steering
```

Do not use Steering for temporary release scope or unstable implementation
notes.

## 6. Confirm scope with Discovery

Give Discovery the collection and the delivery intent in an ordinary request:

```text
$specbind-discovery Use docs/product-definition/ as the source collection for
the first release, including task management and reminders.
```

Discovery classifies work as an existing-Spec update, a new Spec, or Direct.
For a new project, durable capabilities normally become new Specs. Review:

- **Work items** — everything included in this Milestone;
- **New Specs** — proposed durable responsibility boundaries;
- **Gate invalidations** — existing approvals that would be invalidated;
- **Dependencies** — ordering between work items; and
- **Source coverage** — every supplied source and its destination or exclusion
  reason.

Approve only after the coverage, boundaries, and dependencies are correct. The
CLI then creates the Milestone and Spec state. The Roadmap records collection
coverage, while each Brief points only to the source items relevant to that
Spec. Requirements and Design later promote accepted content into authoritative
artifacts; source material is not authoritative by itself.

Use `$specbind-status` at any time for a read-only explanation of state and next
actions.

## 7. Choose how to plan and implement

The first release scope contains several Specs, so the normal route is:

```text
$specbind-plan --all
$specbind-drive
```

Plan establishes Requirements, Design, Contract review, and Tasks for the
Milestone. Drive then advances every safely reachable implementation and
validation action, parks branch-local attention, and stops before Release. See
[Plan and Drive a Milestone](./implement-with-plan-and-drive.md) for the full
workflow and stopping conditions.

To inspect every artifact and Gate separately, instead follow
[Plan and implement one item at a time](./implement-step-by-step.md). Both
routes use the same owning Skills, reviews, and CLI evidence; the difference is
how explicitly you choose each boundary.

## 8. Inspect the artifacts

By default, artifacts live below `.specbind/specs/<spec>/`:

```text
.specbind/
├─ steering/roadmap.md
└─ specs/
   ├─ task-management/
   │  ├─ spec.yaml
   │  ├─ brief.md
   │  ├─ requirements.md
   │  ├─ design.md
   │  ├─ contract.yaml
   │  └─ tasks.yaml
   └─ reminders/
      └─ ...
```

Do not hand-edit lifecycle state in `spec.yaml`, `roadmap.md`, or `tasks.yaml`.
Maintain semantic artifacts through their owning Skills. Direct CLI reads are
available when needed:

```sh
specbind milestone status
specbind spec status <spec-id>
specbind tasks list <spec-id>
specbind artifact list <spec-id>
```

The two status commands also provide `--json` for tools and scripts.

## Next

- [Core concepts](./concepts.md)
- [Plan and implement one item at a time](./implement-step-by-step.md)
- [Plan and Drive a Milestone](./implement-with-plan-and-drive.md)
- [Release a milestone](./release.md)
- [Customize SpecBind](./customization.md)
- [Current generated skill index](../reference/current-skill-index.md)
- [Current generated artifact index](../reference/current-artifact-index.md)
