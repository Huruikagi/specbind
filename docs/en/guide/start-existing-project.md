# Start with an existing project

This route installs SpecBind into a repository that already contains code or
tests and takes a first change through planning, implementation, and validation.
If implementation has not begun, use [Start a new project](./start-new-project.md).

See [Choose a route](./getting-started.md) for supported environments and
coding agents.

!!! info "Terminology"
    Terms used below, including Spec, Steering, Milestone, and Gate, are
    explained in [Core concepts](./concepts.md). Read that page first or refer
    to it as the terms appear.

## 1. Install SpecBind

Commit any pending work first, then follow [Install SpecBind](./install.md),
which both routes share.

```sh
mise use github:Huruikagi/specbind
mise lock
specbind install --agent codex --language en --project-instructions
```

That page covers agent selection, previewing the plan with `--dry-run`, the
installed surfaces, and reopening the agent session.

Return here once the installation is committed. The examples below use Codex
`$skill` syntax; Claude Code uses `/skill`.

## 2. Choose a first-cycle route

An existing project has two starting points with different goals.

| Goal | Route |
| --- | --- |
| Use SpecBind for the next change | Continue on this page |
| Establish Specs from working implementation as the current baseline | [Establish Specs from an existing implementation](./adopt-existing.md) |

For a small first change, keeping the defaults is a good way to learn the
normal lifecycle. Adjust only demonstrated mismatches afterward using
[Customize SpecBind](./customization.md). Continue on this page.

If the repository already has substantial code but no trusted Specs, and you
want to fix the current product as a specification first, use
[Establish Specs from an existing implementation](./adopt-existing.md). That
route shapes Steering and the shared configuration surfaces, then ends with
accepted Requirements, Design, and Contract Review as a non-release baseline.

## 3. Choose the first change

Start with one small behavior rather than several features or release work. For
example:

> Allow users to download the contents of the list screen as a CSV file.

If no existing Spec owns this behavior, it likely introduces one new durable
responsibility. CSV columns and format can also become a Contract on which
other capabilities or consumers depend. When trying this on your own project,
substitute a change of comparable size.

## 4. Confirm scope with Discovery

Describe the change and point to any relevant Issue or notes:

```text
$sb-discovery Allow users to download the contents of the list screen as
a CSV file.
```

Detailed requirements and design come later in Plan, so you do not need to
supply everything here. Choosing technologies or an implementation approach is
also not Discovery's job.

Discovery reads current Spec, Steering, and Milestone state and classifies the
work as **Direct** (a small change that alters no specification), an
**existing-Spec update** (changing the behavior or boundary of a capability the
project already has), or a **new Spec** (one additional durable responsibility).

!!! info "Term: Spec"
    A Spec is one durable capability boundary the project keeps, identified by
    a short kebab-case ID (see [Core concepts](./concepts.md)).

Review the proposal, which covers:

- **Work items** — everything included in this Milestone;
- **New Specs** — proposed durable responsibility boundaries;
- **Gate invalidations** — existing approvals that would be invalidated; and
- **Dependencies** — ordering between work items.

That conclusion becomes the premise for the rest of the workflow, so read the
classification and boundary before approving. The CLI then creates lifecycle
state and the Agent writes a concise `brief.md`.

Use `$sb-status` for a read-only explanation at any point. It never approves
anything or rewrites artifacts.

## 5. Choose how to plan and implement

For the first `csv-export` change, you can inspect each boundary explicitly:

```text
$sb-plan csv-export requirements
$sb-plan csv-export design
$sb-contract-review
$sb-plan csv-export tasks
$sb-implement csv-export
$sb-validate-implementation csv-export
```

[Plan and implement one item at a time](./implement-step-by-step.md) explains
what to review and approve at every step, including upstream rewinds.

For a Milestone with several Specs or Direct items, use
`$sb-plan --all` followed by `$sb-drive`. See
[Plan and Drive a Milestone](./implement-with-plan-and-drive.md) for attention,
continuation, and stopping behavior. Both routes stop before Release.

## 6. Inspect the artifacts

By default, artifacts live below `.specbind/specs/<spec>/`:

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
Do not hand-edit them to advance state. Maintain the planning content of
Requirements, Design, Contract, and Tasks through the matching `sb-plan` phase.
Read current state directly with:

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
- [Establish Specs from an existing implementation](./adopt-existing.md) — make current code the baseline
- [Release a milestone](./release.md) — when you actually close the Milestone
- [Customize SpecBind](./customization.md) — after one cycle shows what to adjust
- [Current generated skill index](../reference/current-skill-index.md)
- [Current generated artifact index](../reference/current-artifact-index.md)

---

[User guide](../index.md) | [Install SpecBind](./install.md) | [Start a new project](./start-new-project.md)
