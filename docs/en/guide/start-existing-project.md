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

## 3. Choose a first-cycle route

| Goal | Route |
| --- | --- |
| Use SpecBind for the next change | Continue on this page |
| Establish Specs from working implementation as the current baseline | [Full-support route](#full-support-route) |

For a small first change, keeping the defaults is a good way to learn the
normal lifecycle. Adjust only demonstrated mismatches afterward using
[Customize SpecBind](./customization.md). Continue with [Choose the first
change](#first-change).

## Full-support route for an existing implementation {#full-support-route}

Choose this route when the repository already has substantial code but no
trusted Specs, and you want to establish project guidance and plan the adopted
product before making a change. It ends with accepted Requirements, Design,
and Contract Review plus a non-release baseline archive. It creates no Tasks,
implementation change, or product release.

1. **Shape the project with `sb-configure`.** Start with an initial review,
   such as:

   ```text
   $sb-configure Review this project's initial configuration for adopting the
   existing implementation. Start with the Steering it needs.
   ```

   `sb-configure` first reads the mechanical configuration summary. When
   durable guidance is needed, it routes Steering bootstrap or synchronization
   to `sb-steering`. Review and commit the resulting Steering before adoption;
   Discovery pins that revision as its evidence.

2. **Run focused configuration reviews until the common project surfaces fit.**
   Ask `sb-configure` again after Steering is established to compare it and the
   repository with the Requirements and Design templates and shared Rules. For
   example:

   ```text
   $sb-configure Use the confirmed Steering and repository facts to review the
   Requirements and Design templates and shared Rules for this project.
   ```

   Make a separate follow-up request for each remaining surface—templates,
   Rules, Agents, or operational adapters. `sb-configure` rereads the summary
   after each relevant change and completes its required aftercare. A new Design
   template is appropriate only for a distinct recurring responsibility; a
   technology label alone is not enough. Existing Specs and lifecycle artifacts
   are not silently reconciled by configuration.

3. **Start existing-implementation Discovery.** With committed Steering and a
   clean worktree, ask for a bounded adoption target, for example:

   ```text
   $sb-discovery Establish Specs from the existing implementation across this
   repository as existing version v2.4.0. Investigate the current code and
   tests as evidence, and ask me to confirm the boundaries and maintained
   behavior before creating anything.
   ```

   Discovery runs its adoption preflight, pins the inspected revision, and
   presents one complete reverse proposal: the existing `baseline_version`,
   Spec boundaries, maintained intent, evidence, unknowns, suspected defects,
   dependencies, and excluded area. Current code is evidence, not automatically
   the specification.

4. **Let Discovery finish the baseline.** After you confirm that proposal,
   the same invocation creates the reverse milestone and continues through
   Requirements, Design validation, Design approval, and the milestone-wide
   Contract Review. It does not stop for routine phase confirmations and never
   creates Tasks. It stops only for a question whose answer would change the
   Spec, source drift, or a failed lifecycle check.

   Finalization records a `Baseline v2.4.0` entry in each Spec log and archives
   the Roadmap and Contract Review under `baselines/`. The established Specs
   then behave like ordinary existing Specs while retaining their source
   revision and version provenance. See [Establish Specs from an existing
   implementation](./adopt-existing.md) for the complete stopping and
   finalization rules.

## 4. Choose the first change {#first-change}

Start with one small behavior rather than several features or release work. For
example:

> Allow users to download the contents of the list screen as a CSV file.

If no existing Spec owns this behavior, it likely introduces one new durable
responsibility. CSV columns and format can also become a Contract on which
other capabilities or consumers depend.

## 5. Confirm scope with Discovery

Describe the change and point to any relevant Issue or notes:

```text
$sb-discovery Allow users to download the contents of the list screen as
a CSV file.
```

Discovery reads current Spec, Steering, and Milestone state and classifies the
work as Direct, an existing-Spec update, or a new Spec. It presents work items,
new Specs, Gate invalidations, and dependencies. Review the classification and
responsibility boundary before approval. The CLI then creates lifecycle state
and the Agent writes a concise `brief.md`.

Use `$sb-status` for a read-only explanation at any point.

## 6. Choose how to plan and implement

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

## 7. Inspect the artifacts

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
