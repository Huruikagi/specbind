# CLI commands

You normally work through Skills, which run the CLI for you. This page lists
the `specbind` commands by purpose so you can inspect state yourself or
recognize a command a Skill reports. Run `specbind <command> --help` for exact
arguments and options.

## Inspect state (read-only)

Safe to run at any time. They never change project files.

| Command | Shows |
| --- | --- |
| `specbind milestone status` | Active Milestone stage, progress, next actions, and release blockers |
| `specbind milestone replan capture` | Clean pre-replan revision and fresh accepted completions, as JSON |
| `specbind milestone replan check --baseline <commit> --recovery-revision <commit>` | Remaining completion withdrawals and verified checkpoints after a fixed replan, as JSON |
| `specbind spec list` | Every Spec in the project |
| `specbind spec status <spec>` | One Spec's state, freshness, coverage, and Task progress |
| `specbind spec completion batch preflight --spec <id> --spec <id>` | Explicit completion target set and one clean implementation revision, as JSON |
| `specbind spec completion batch verify --evidence <file\|->` | Proof that the supplied completion set has one narrow, fresh checkpoint |
| `specbind tasks list <spec>` / `tasks show <spec> <task>` | Task hierarchy and progress, or one Task's details |
| `specbind tasks revision preflight <spec>` | Source fingerprint and current plan/execution for an existing-plan revision, as JSON |
| `specbind artifact list <spec>` / `artifact read <spec> <selector>` | A Spec's artifacts, or one artifact's content |
| `specbind steering list` / `steering read <selector>` | Steering documents |
| `specbind configuration show` | Summary of every project configuration surface |
| `specbind release preflight` | What still blocks release |
| `specbind check traceability <spec>` | Requirement coverage by Design and Tasks |
| `specbind check contracts` | Project-wide Contract graph errors |
| `specbind check contracts --for-design <spec>` | One Spec's Contract and its direct consumed targets; does not evaluate the whole graph |
| `specbind contract graph` / `dependencies` / `consumers` / `owners` / `shared` | Contract relationships and file ownership |
| `specbind contract diff --baseline <revision> [--json]` | Normalized baseline/worktree Contract changes and declared consumers on both sides |
| `specbind worktree capture` | Transient JSON snapshot of HEAD, index and observed file contents |
| `specbind worktree check --snapshot <file\|-> --changed <path>` | Actual changes compared with exact declared paths, as JSON |

`milestone status` and `spec status` also accept `--json` for tools and scripts.
See [Lifecycle states](./lifecycle-states.md) for the state names they report.

Task readiness requires every earlier executable Task to be completed, including
across groups. Retained completed records do not hide an earlier pending or
blocked Task. The same rule guards `tasks complete`; proven progress is preserved.

Individual Design checks can pass while unrelated Specs are still being designed.
Required provider Contracts must already exist and their consumed entries must
resolve. The complete graph is still required at Contract Review; later Design
work or that review may require revising a previously approved Design.

## Compare changes before and after work

`worktree capture` writes JSON to stdout. Preserve it unchanged in memory or a
file outside the repository. After the work, pass it to `worktree check` using
`--snapshot <file>` or `--snapshot -` for stdin. Repeat `--changed` for every
exact project-relative changed file, with `/` separators; omit it when no change
is expected. Directories and globs do not cover child paths. Declare both paths
of a rename. The commands require a Git repository with a committed HEAD, but
do not require SpecBind configuration.

The comparison distinguishes file-content changes from index changes and detects
additional edits to already dirty files. `unexpectedPaths` reports undeclared
changes, `preexistingTouchedPaths` identifies earlier dirty files that changed
again, and `declaredUnchangedPaths` reports declarations without observed edits.
Exit zero returns `WORKTREE_CHECK_MATCHED`; HEAD movement or unexpected changes
return `WORKTREE_CHECK_MISMATCH` with exit one. Input/read errors stop without a
partial report. A match does not establish ownership, correctness or permission
to include an earlier change in a commit.

Coverage includes tracked and nonignored untracked files, plus original snapshot
paths even if they later become ignored. Ordinary ignored files, Git metadata
and empty directories are outside coverage. Links, submodules, conflicts,
intent-to-add and hidden/sparse index entries are rejected. The CLI checks for
stable repeated reads, but does not lock the tree. It never restores, removes,
stages or commits files. Read the snapshot schema with
`specbind schema read worktree-snapshot/v1`.

## Compare Contracts with a baseline

`contract diff` compares all persistent Spec Contracts and the optional shared
Contract at the supplied Git revision with the current worktree, including
uncommitted proposals. It uses the currently configured `specDir` at both ends.
Contract Review supplies the Milestone's baseline; choosing a newer baseline
would hide changes that still need review.
If baseline configuration exists with a different `specDir`, comparison stops
instead of inferring moved Spec identities.

With `--json`, `documents` contains complete normalized before/after manifests
and their exact paths, including unchanged documents and empty-manifest creation
or removal. `entries` lists added, changed and removed entries with separate
`baselineConsumers` and `currentConsumers`. These are declared direct references
from managed Specs; they do not prove the absence of external or indirect impact.
Ordering and formatting alone do not create entry changes.

`currentRevision` identifies HEAD, while `currentSource: worktree` and
`currentFingerprint` identify the actual compared inputs. Input errors stop the
comparison; missing required Contracts are never treated as new Specs. Graph
errors remain visible in `diagnostics`, so a removed target can still name its
consumers. Exit zero means the comparison succeeded, not that review passed.
Run `check contracts` for the complete current graph and assess compatibility,
external consumers and scoped behavior even when the diff is empty. Recompare
and reassess after inputs change. The command changes no files or approvals.

## Read project settings and embedded content (read-only)

| Command | Shows |
| --- | --- |
| `specbind template list` / `read` / `resolve` | Artifact templates and where a new artifact would be placed |
| `specbind rule list` / `read` | Project-owned shared rules |
| `specbind adapter list` / `read` | Project-owned operational adapters |
| `specbind protocol list` / `read` | Product protocols embedded in the binary |
| `specbind schema list` / `read` | Structured artifact schemas embedded in the binary |

`specbind schema read drive/v1` returns the schema for the `drive` member of
`.specbind.json`, not the entire root configuration. `configuration show` reports
effective Drive defaults and whether each comes from configuration or legacy fallback.

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
| `specbind tasks revision apply <spec> --input <file\|->` | Existing-plan replacement and explicit progress reconciliation (`sb-plan`; `--dry-run` checks without writing) |
| `specbind milestone direct` | Direct item completion (`sb-implement`) |
| `specbind spec completion` | Spec completion evidence (`sb-validate-implementation`) |
| `specbind spec completion batch accept --evidence <file\|->` | Completion evidence for an explicitly supplied Spec set at one revision |
| `specbind milestone bind-release <version>` | Target release binding (`sb-release`; you may also bind ahead of time) |
| `specbind release finalize` | Milestone release finalization (`sb-release`) |
| `specbind adoption preflight` / `milestone reverse` | Establishing Specs from an existing implementation (`sb-adopt`) |
| `specbind artifact check` / `steering check` | Checks on a newly created artifact during authoring |

!!! warning
    Running these commands directly bypasses the reviews the owning Skill
    performs. Use them by hand only when a guide tells you to, such as
    `specbind milestone reverse abandon` in
    [Establish Specs from an existing implementation](../guide/adopt-existing.md).

## Revise a Task plan with recorded progress

After an authorized Tasks rewind, `sb-plan` uses `tasks revision preflight`
and `schema read task-revision/v1` to prepare a replacement candidate. Both
revision operations require `tasks` state, fresh Requirements/Design approval
and fresh Contract Review. The existing plan remains available as repair input
even when its old Requirement coverage no longer fits the active change.

The strict JSON candidate contains `schemaVersion: 1`, the returned
`sourceFingerprint`, a complete `plan` in Tasks v1 format and a `progress` array.
Every recorded entry needs exactly one mapping: `preserve` or `reset` with
`from`, `to`, and `reason`, or `remove` with `from` and `reason`. With no recorded
execution, use `progress: []`. Targets must be unique executable Tasks.
The candidate must not contain `execution` or replacement blocker reasons.

The Skill judges whether obligations and evidence still apply and obtains
confirmation of the mapping unless existing Drive replan authority covers it.
The CLI copies preserved records unchanged, leaves reset/new work pending and
removes discarded records. It rejects missing/duplicate mappings, invalid
replacement coverage and source changes. A changed fingerprint requires a new
read and reassessment, not simply substituting the latest fingerprint.

Input is supplied on stdin (`--input -`) or from a temporary file outside the
worktree. `--dry-run` returns the mapping and resulting document without writing;
apply replaces only `tasks.yaml` atomically. Gate approval and the Git checkpoint
remain separate. First authoring still creates a plan without execution;
subsequent revisions use this CLI even when no progress has been recorded.

## Accept completion for several Specs together

After batch preflight, `sb-validate-implementation` performs the required checks
and a separate semantic assessment for every named Spec. Only an all-`GO` set
may be accepted. The CLI never executes those checks or grants authority.

`schema read completion-batch/v1` describes the strict transient JSON input:
`schemaVersion: 1`, one `implementationRevision`, and nonempty `targets`, each
with a unique `spec` ID and nonempty `mechanicalChecks`. Checks contain `kind`,
the exact executed `command`, `exitCode: 0`, and optional project-relative
`workingDirectory`. Supply it on stdin or from a file outside the project.

Batch acceptance validates all candidates and lifecycle conditions before the
first write, then checks the fixed HEAD and exact pending changes before each
write. Its JSON report includes `accepted`, `pending`, `paths` and `issues`.
An ordinary precondition failure reports `COMPLETION_BATCH_REJECTED` without
writes. `COMPLETION_BATCH_INTERRUPTED` preserves the confirmed accepted prefix;
the operation is atomic per file, not across the set. Stop on failure: do not
automatically roll back, retry or commit a subset. Invalid input and read-only
preflight/verification failures use the usual CLI error diagnostics. An abrupt
process failure may leave no report; inspect current state before any recovery.

After `COMPLETION_BATCH_ACCEPTED`, the Git adapter governs one commit containing
exactly the returned metadata paths. Reuse the same candidate for `batch verify`.
`COMPLETION_BATCH_VERIFIED` proves a clean single-parent checkpoint immediately
after the implementation revision, unchanged earlier gates and fresh completion
for every target. Extra files, other metadata changes and split checkpoints
fail verification. If the adapter forbids committing, report the uncommitted
set and skip verification. A HEAD or worktree change requires resolving that
work and starting a fresh validation run; changing only the candidate revision
does not renew its evidence.

## Check completion recovery after replanning

Before an authorized replan, Drive retains the full `baselineRevision` returned
by `milestone replan capture`. After Plan finishes, it fixes `recoveryRevision`
to that checkpoint and uses `milestone replan check` before each withdrawal and
after each withdrawal checkpoint. The CLI reconstructs committed state in
temporary local repositories, checks the entire affected set and recognizes
only exact, ordered completion-only checkpoints. It requires a clean worktree;
missing history, stale earlier gates, changed scope or Requirements authority,
and unrelated intervening commits stop the check.

The result contains `remaining`, `withdrawn` and `recoveryChangedPaths`.
It does not establish authorization or whether the recovery delta implements
the authorized repair. Drive and the validation owner still review that delta
and the original authority. Neither fixed revision may be advanced to hide work.
The validation owner uses the existing completion invalidation and its separate
checkpoint; the new commands never withdraw or accept completion themselves.
