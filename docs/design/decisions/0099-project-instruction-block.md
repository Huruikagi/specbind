# 0099: Fix the project-instruction block contract

Status: Accepted

## Context

[Decision 0077](./0077-v1-installation-distribution-and-migration.md) accepts that
enabling project instructions makes the installer maintain "only a marked
SpecBind block" in the selected agents' root `AGENTS.md` or `CLAUDE.md`. It does
not fix the marker, the content, the target for each agent, or what happens when
the setting is turned off. Nothing is implemented: the flag is accepted and
persisted to `.specbind.json`, and no file is written.

The gap has a concrete cost. An agent opening a project sees skills invoking
`specbind milestone status` with nothing establishing what that command is,
whether the binary is available, or why the machine state it reads must not be
hand-edited. The skills describe how to do the work; nothing says what system
the work belongs to.

The inherited block answered that question and much more besides — roughly sixty
lines covering artifact paths, a twenty-command workflow listing, skill
directory structure, development rules, and an instruction to load all steering
as project memory. Most of that has since moved elsewhere in SpecBind, and one
part is now the opposite of an accepted decision.

## Decision

### Scope of the block

The block establishes the system and its boundary. It contains:

- that this project uses SpecBind, and that the `specbind` CLI is the interface
- that the installed `specbind-*` skills are the entry points, naming discovery
  and status as the two starting points
- that hyphenated `specbind-*` names identify Skills rather than CLI syntax,
  without defining the agent platform's invocation mechanism
- that CLI-owned machine state is never hand-edited, stated precisely enough to
  exclude the parts that are authored

It contains nothing else. In particular it does not restate:

| Excluded | Owner |
| --- | --- |
| Artifact and directory paths | the CLI, which resolves them; skills name logical selectors under Decision 0058 |
| The workflow and its commands | the skills themselves |
| Skill directory structure and invocation | the agent platform, which discovers skills |
| Authoring conventions and development rules | Decision 0094 protocols and Decision 0093 rules |
| "Load all steering as project memory" | rejected outright by [Decision 0098](./0098-steering-read-surface.md) |

Every excluded row is a place where the block would become a second, unversioned
copy of a contract owned elsewhere. A duplicate that cannot be validated drifts,
and a drifted instruction in always-loaded context is worse than none, because
it is read with the authority of project configuration.

The last row is not merely redundant but contradictory: it would instruct every
agent to do the thing Decision 0098 removed.

### Markers

```text
<!-- specbind:block -->
...
<!-- /specbind:block -->
```

HTML comments render as nothing in Markdown, and `specbind:` already prefixes
the `specbind:instruction` comments in Decision 0059 templates, so the namespace
is consistent.

- Exactly one opening and one closing marker may appear, in that order.
- Zero markers means the block is absent and will be appended.
- Anything else — a marker without its pair, reversed order, or more than one of
  either — stops the operation with a diagnostic naming the file. The installer
  never guesses which of two blocks is authoritative, and never repairs a
  malformed one, because both would edit text the project owns.
- Markers are matched as whole lines. A marker inside a fenced code block still
  counts, which is deliberate: resolving that would require a Markdown parse
  whose result the user cannot easily predict, and a stop is recoverable.

### Content and language

The body is an embedded asset at
`tools/specbind/assets/project-instructions/block.md`, included in the binary
the same way Decision 0094 protocols, Decision 0093 rules, and Decision 0096
skills are. It is prose the product ships, so it is authored and reviewed as
prose rather than as a string literal in Rust.

This decision fixes what the body must establish, listed above. The asset holds
the wording, and revising the wording within that scope is an ordinary asset
change rather than an amendment here.

It is product-managed, not project-owned. Like a skill, and unlike a Decision
0091 template or a Decision 0093 rule, a divergent copy in a project is replaced
rather than kept: the block is a pointer to the current product surface, and a
stale pointer is the failure it exists to prevent. Local guidance belongs outside
the markers, where the installer never touches it.

The block is English, like every other product-managed agent-facing asset. Only
Decision 0059 templates are localized, because they scaffold artifacts the
project authors in its own language; this block is instruction to the agent.

The asset currently reads:

```markdown
## SpecBind

This project uses SpecBind for spec-driven development. The `specbind` CLI owns
the specification lifecycle: it validates artifacts, records approvals, and is
the only supported writer of machine state.

- Hyphenated names such as `specbind-status` identify installed Skills, not
  shell commands. Select them through the agent platform; do not translate a
  Skill name into a `specbind ...` command. CLI syntax comes from the selected
  Skill.
- Work through those installed `specbind-*` Skills. Use `specbind-discovery` to
  turn a request into scope, and `specbind-status` to see where work stands.
- When every Task for a named Spec is complete and the user asks whether that
  Spec is done, complete, or ready, use `specbind-validate-implementation`.
  Do not answer that question from status or consequence-free claim checking.
- When the user asks to review one implemented Task, use
  `specbind-review-task`; the review must judge the actual diff without fixing
  it or recording Task state.
- When the user asks why a Task failed or cannot be implemented, use
  `specbind-debug` directly. A diagnosis-only request does not start
  implementation, and its final response must preserve the exact diagnosis
  block rather than summarize a nested result.
- Use `specbind-steering` when the request creates or updates durable,
  project-wide guidance, including conventions for testing, APIs, security, or
  deployment. This route does not require a Spec or observable behavior change.
- Use `specbind-adopt-existing` only when the user explicitly wants to establish
  new Specs from an existing implementation. It requires committed Steering and
  treats code and tests as evidence rather than intended specification.
- A request enters that flow when it changes a Spec's artifacts or observable
  behavior, including a validation rule, limit, or rejected case; modifies a path
  the Spec owns; adds a durable responsibility; or belongs to a delivery the
  project is tracking. Before classifying anything as ordinary work, run
  `specbind milestone status`: a request matching a pending Spec-backed or Direct
  item is tracked delivery work and routes to `specbind-implement`. When the
  classification is genuinely unclear, enter the flow. Anything else is ordinary
  work: say in one line that it needs no Spec, and do it.
- Never hand-edit `spec.yaml`, the active roadmap, or the execution state in
  `tasks.yaml`. Those are CLI-owned, and a hand edit produces state no command
  validated. The task plan itself is authored, by the skill that owns it.
- Run `specbind --help` if the command is unfamiliar or appears unavailable.
```

The machine-state line names its targets exactly. An earlier wording grouped
`tasks.yaml` with `spec.yaml` and the roadmap, which forbade the only way a task
plan can be written: no command authors plan content, and Decisions 0024 and 0095
give the CLI only the execution state inside that file. A forward test found it —
an agent drafted a plan revision, reverted it on reading the block, and stopped
with nothing left to do. Always-loaded context reaches every agent in every
installed project, so an over-broad prohibition there is not a wording problem.

### Targets

| Agent | File |
| --- | --- |
| Claude Code | `CLAUDE.md` |
| Codex | `AGENTS.md` |

Both files are written when both agents are selected. Each agent reads only its
own file, so a shared file would leave one of them without instructions.

The file is at the repository root. A missing file is created containing the
block alone. An existing file keeps all content outside the markers exactly as
it is; only the region between them is replaced.

An absent block is appended at the end of the file, preceded by one blank line
when the file does not already end in one. Appending is the only safe insertion
point: the top of an instruction file is where a project states its own most
important guidance, and inserting there would reorder the project's content by
the installer's preference.

### Turning it off

Disabling project instructions stops maintaining the block. It does not remove
one that exists.

`specbind install` never interprets disabling as removal. Decision 0141 later
adds separate plan-by-default `remove-agent` and `uninstall` commands. Those
commands may remove only this exact valid marked region behind committed,
tracked, clean, non-link guards; they preserve every byte outside it and stop on
malformed or repeated markers.

### Guards

- Replacing an existing block is a replacement under Decision 0077 and requires
  a repository with at least one commit and a clean worktree. Creating a missing
  file or appending to one does not.
- The write revalidates the planned state and fails closed when the file changed
  after planning. Unlike every other installed asset, this one is edited in
  place, so presence proves nothing: the plan carries the exact prior content it
  read and the apply compares against those bytes.
- The plan reports the block as `create`, `replace`, or `keep` with its target
  path, so `--dry-run` shows exactly which instruction files would be touched.

## Consequences

- An agent entering an installed project learns what SpecBind is before it meets
  a skill that assumes it.
- The block stays small enough to review in one screen, and small enough that it
  does not compete with the project's own instructions.
- Contracts stay single-sourced: the block points at the CLI and the skills
  rather than restating them, so it cannot drift out of agreement with either.
- The wording lives with the other shipped prose, so it is reviewed as prose and
  can be revised without amending this decision.
- The inherited instruction to preload steering disappears rather than being
  quietly carried forward into a decision that rejects it.
- A malformed or duplicated marker is a stop rather than a repair, so the
  installer never rewrites instructions a human wrote.

## Implementation status

Implemented. `tools/specbind/src/installation/project_instructions.rs` owns the markers and
embeds `assets/project-instructions/block.md`, and `specbind install` plans and
applies one entry per selected agent under the `project-instructions` category.

Adding a block to an existing file is reported as a creation rather than a
replacement, because it removes no text and therefore does not require the
Decision 0077 committed clean repository. That distinction needed a stronger
race check than the other categories use: presence cannot detect a change to a
file the installer edits in place, so the entry carries the exact prior content
it planned from and the apply compares against those bytes.

Tests cover creating a missing file, appending behind exactly one blank line
across four trailing-whitespace shapes, replacing only the marked region while
preserving the text around it, idempotence, and a stop on duplicated, unpaired,
or reversed markers including one inside a code fence. An install-level test
confirms both agent files are written, the project's own content survives, a
second run reports `NO_CHANGE`, no entry is planned when the setting is off, and
a malformed marker fails the plan while leaving the file untouched.

[Decision 0147](./0147-generic-agent-shared-surfaces.md) later adds `generic`
as another consumer of root `AGENTS.md`. Installation and removal treat that
marked block as one shared target when Codex and generic are both selected.
