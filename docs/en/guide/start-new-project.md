# Start a new project

This route installs SpecBind before implementation begins, divides the first
release scope into durable responsibilities, and proceeds from scope review
through implementation validation.

See [Choose a route](./getting-started.md) for supported environments and
coding agents.

!!! info "Terminology"
    Terms used below, including Spec, Steering, Milestone, and Gate, are
    explained in [Core concepts](./concepts.md). Read that page first or refer
    to it as the terms appear.

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

## 2. Install SpecBind

Installing the CLI and installing SpecBind into the project are shared by both
routes and are described in [Install SpecBind](./install.md).

```sh
mise use github:Huruikagi/specbind
mise lock
specbind install --agent codex --language en --project-instructions
```

That page covers agent selection, previewing the plan with `--dry-run`, the
installed surfaces, and reopening the agent session. Commit the installation
separately from the baseline in step 1.

Return here once the installation is committed. The examples below use Codex
`$skill` syntax; Claude Code uses `/skill`.

## 3. Keep the defaults for the first cycle

The installer suggests a configuration review with `sb-configure`.
For a new project, first complete one cycle with the defaults. After you have
real artifacts to evaluate, use [Customize SpecBind](./customization.md) to
change only the surfaces that do not fit.

## 4. Prepare the first release scope

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
dependencies between the resulting work items. Source items must be tracked in
Git, so commit this material together with the step 1 baseline.

### If you already have durable guidance

If you already have stable product, technology, structure, security, or testing
guidance that should apply beyond this Milestone, establish it separately
before Discovery:

```text
$sb-steering
```

Empty Steering is a valid state, so do not invent guidance just to start. Do
not use Steering for temporary release scope or unstable implementation notes.

## 5. Confirm scope with Discovery

Give Discovery the collection and the delivery intent in an ordinary request:

```text
$sb-discovery Use docs/product-definition/ as the source collection for
the first release, including task management and reminders.
```

Discovery reads current Spec, Steering, and Milestone state and classifies the
input. A new project has no existing Specs, so this scope becomes either
**Direct** (a small change that alters no specification) or **new Specs** (one
durable responsibility each).

!!! info "Term: Spec"
    A Spec is one durable capability boundary the project keeps, identified by
    a short kebab-case ID (see [Core concepts](./concepts.md)). If this scope
    separates task management from reminders, each becomes its own new Spec.

Review the proposal, which covers:

- **Work items** — everything included in this Milestone;
- **New Specs** — proposed durable responsibility boundaries;
- **Gate invalidations** — existing approvals that would be invalidated;
- **Dependencies** — ordering between work items (for example, reminders →
  task management); and
- **Source coverage** — every supplied source and its destination or exclusion
  reason.

Approve only after the coverage, boundaries, and dependencies are correct; that
conclusion becomes the premise for the rest of the workflow. The CLI then
creates the Milestone and Spec state. The Roadmap records collection coverage,
while each Brief points only to the source items relevant to that Spec.
Requirements and Design later promote accepted content into authoritative
artifacts; source material is not authoritative by itself.

Use `$sb-status` at any time for a read-only explanation of state and next
actions. It never approves anything or rewrites artifacts.

## 6. Choose how to plan and implement

The first release scope contains several Specs, so the normal route is:

```text
$sb-plan --all
$sb-drive
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

## 7. Inspect the artifacts

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
- [Release a milestone](./release.md) — when you actually close the Milestone
- [Customize SpecBind](./customization.md) — after one cycle shows what to adjust
- [Current generated skill index](../reference/current-skill-index.md)
- [Current generated artifact index](../reference/current-artifact-index.md)

---

[User guide](../index.md) | [Install SpecBind](./install.md) | [Start with an existing project](./start-existing-project.md)
