# Isolated Spec implementation

This procedure belongs to `sb-drive`. It changes dispatch and integration only;
`sb-implement` still owns each Spec's sequential Task cycle. No new progress
format, Task dependency rule, gate authority or release authority is created.

## 1. Check execution capabilities before selecting a batch

Use only tools actually available in the current host. Confirm all of:

- Independent concurrent owning-workflow contexts can be started in distinct
  Git worktrees at an explicit committed base, with project-local instructions,
  executable/PATH and environment setup verified inside each checkout.
- Each context can execute the complete `sb-implement` Skill, including its
  existing fresh implementation, review and diagnostic dispatches or established
  capability fallback. Fresh review must not inherit implementation reasoning.
- Worker status and terminal results can be retrieved, and partial work and
  completed commits can be retained across interruption. Use a named branch or
  a verified host retention guarantee before accepting a disposable checkout.
- The project Git adapter allows per-Task checkpoints and serial local
  integration preserving those units. Explicit user restrictions take priority.
  Read `specbind adapter read git --for consume`; absent/inactive policy or an
  incompatible commit policy means use the ordinary sequential cycle.
- A clean integration checkout and a separate candidate checkout are available;
  integration can be coordinated without another writer changing that checkout
  during the final verification and fast-forward. Shared test services, ports,
  databases and setup hooks are isolated or explicitly coordinated.

If any capability is absent or cannot be verified, explain the limitation once
and continue sequentially. This includes generic Skill-only hosts: understanding
Skills or having a shell with `git worktree` is not proof of isolated agent
execution. Do not fail the whole delivery request, install a host integration,
change permission settings, or launch shared-worktree workers as a substitute.
Sequential fallback preserves all existing reviews, guards and approval rules;
it does not bypass a real environment or worktree blocker. `--parallel 1` takes
this same ordinary path without creating workers or candidates.

Check for retained workers from an interrupted run before dispatching even on
fallback. Use actual host sessions and Git refs/diffs, not branch names alone,
to identify their item and starting state. Reuse only attributable current work;
park ambiguous or inaccessible retained results for user attention instead of
starting duplicate work for that item. Do not require a persistent run registry.

### Host-specific checks

For Claude Code, verify the available Agent call can isolate the outer owning
workflow in a worktree. Explicit call-level isolation avoids name-based routing
into a shared-directory Agent Teams teammate. Teams are not required. Check
nested Agent capability and depth for owner -> implementer/reviewer. Internal
roles use the owner's checkout, not fresh worktrees of their own. Do not assume
the host's default base is current HEAD, and do not change global settings to
make it so. Prepare or select the exact base through supported tools, then check
it inside the worker. Hook paths and dependency setup must target that checkout.
Permission requests may need user attention; never suppress them for throughput.

For Codex, verify the actual runtime's isolated dispatch and result-control
capabilities. App worktree chats are not proof of CLI subagent support. A fork
may copy uncommitted changes and may start detached: verify clean status and
full base revision, and establish retention. Handoff between checkouts is not
integration acceptance. Preserve existing fresh-role capability selection.

Do not assume any host/version supports this procedure solely from these names.

## 2. Select a bounded implementation batch

Read `specbind milestone status --json` in the designated integration checkout.
Use only its current ordered actionable entries and typed handler operands.
When the first safe action is not Spec-backed implementation, follow the ordinary
single-owner path. Otherwise select up to the requested limit of independent,
already-actionable Spec-backed `sb-implement` entries in status order; skip
parked items and do not bypass dependency waits. With fewer than two eligible
items, use ordinary sequential execution.

Confirm semantic independence using the approved Contracts, Design and Task
verification prerequisites where needed. File ownership alone is insufficient.
Unavailable verification input, shared mutable services, overlapping generated
outputs or uncertain interaction prevents batching those items. Do not add
product APIs or planning metadata merely to make work independently finishable.

Pin the integration checkout's full clean HEAD, milestone identity, membership,
approved inputs and selected item identities in the run's dispatch briefs.
Before any worker mutation, verify its HEAD equals that base and its working
tree is clean. A host-created checkout carrying dirty files or the wrong base
is an environment failure; retain it and return to safe sequential selection
only after proving there is no duplicate/partial work. Never reset it to fit.

Do not dispatch a second batch until all workers in this batch have returned or
been safely parked with no outstanding mutations that could affect acceptance.
No planning, replanning, Direct work, release binding or final validation runs
alongside a batch. Even with `--replan`, workers return upstream findings; they
do not change approved inputs. Replanning happens after the batch is quiescent,
and invalidates reuse of every retained result whose prerequisites changed.

## 3. Delegate the complete owner

Send the exact `handler.target` Skill, item/operand, full base revision,
worktree, project-local executable/PATH, instruction paths, verification
environment and authority. Require it to read the installed Skill and finish
its owning workflow, including each Task's review, progress and checkpoint.
An internal `READY_FOR_REVIEW` is not a terminal worker result.

Confine all worker and internal-role mutations to that worker checkout. Run no
Spec completion handshake and do not update the integration checkout. Require
the terminal handoff to identify item, base/result commits, retained branch and
worktree, completed Tasks, changed paths, checks and blockers. Verify these
against Git and CLI state; narration is not evidence.

A blocked worker keeps its committed checkpoints and dirty partial changes.
Park it without adopting its partial result or manufacturing a WIP commit.
Another independent successful worker may still be accepted. Preserve retry
budgets across all dispatches; worktree isolation does not reset them.

## 4. Accept successful results serially

Process successful workers in dispatch order, skipping parked failures. For
each result:

1. Verify the base/result ancestry and complete commit range. Its diff may
   contain only the target Task implementation/tests, its CLI-written execution
   transitions and permitted Implementation Notes. Reject foreign progress,
   Requirements/Design/Contract/plan edits, completion or release metadata.
   Inspect Task checkpoint units; an uncommitted or unattributable result is
   retained attention, not an eligible integration candidate.
2. Reread integration status and Git state. Verify milestone membership,
   dependencies, approved inputs and all changes since the worker's base.
   Changed scope or stale approval routes to its owner; no automatic adoption.
   Even non-overlapping paths need judgment about changed assumptions.
3. In a separate candidate worktree based on current integration HEAD, replay
   whole Task commits using the Git adapter's allowed method, preserving
   checkpoint units. Do not mutate the integration checkout while assembling
   or checking the candidate. Retain failures; do not reset or discard work.
4. Delegate fresh interaction review and affected Task/project checks on the
   combined candidate to the owning `sb-implement` workflow. Its sequential
   Task cycle and review policy still apply. For completed Tasks whose proof
   must be renewed, explicitly request reopening with the integration finding;
   use `specbind tasks reopen <spec> <task-id>` in plan order, then the ordinary
   implement/review/complete/checkpoint cycle. Do not silently mark invalid
   behavior complete or invent a new Task. An unresolved replay conflict is
   parked with its exact conflicting paths; do not tell an owner to bypass its
   progress/cleanliness guards to finish the merge. Gates need their own owner.
5. Require the candidate to be clean and all required checks/review to pass.
   Recheck integration HEAD equals the expected base and its checkout remains
   clean immediately before the Git adapter's permitted fast-forward. Never
   force an update or rewrite existing integration history. If another writer
   intervenes, park and rebuild/recheck against the new revision; do not rely
   on a stale comparison. Without exclusive write coordination, stop acceptance.
6. After the verified fast-forward, reread `specbind milestone status --json`
   and Git status from the integration checkout. Only this state unlocks
   descendants. Worker-local `completed` never does so on its own.

Do not silently squash multiple Task checkpoints or copy completion evidence.
The CLI's existing Task progress interpretation stays unchanged; the candidate
review checks whether imported branch-local proof still applies. Final Spec
validation occurs only after implementation converges, through the existing
common clean revision preflight/accept handshake.

## 5. Retain, resume and report

A failed candidate or blocked worker stays separate from authoritative progress.
On restart, integrated progress comes from fresh CLI state. Identify retained
workers/candidates through actual base, item, approved inputs and attributable
diff before resuming. If the result is already integrated, do not replay or
implement it again. Ambiguity becomes attention, never a duplicate dispatch.

An unsafe integration checkout stops acceptance and new batches. It does not
justify deleting worker work. Return each parked path/ref and the exact owner
or decision needed; do not call an unaccepted result globally completed.

Delete a worker/candidate only after verifying its useful changes are accepted
and no remaining dirty or unaccepted work would be lost, under existing Git
policy and tool permissions. Otherwise retain it. Do not depend on automatic
host cleanup to preserve work. Create no persistent queue, progress ledger or
new authority artifact.

Report requested and actual parallelism, any sequential fallback reason,
accepted items, retained results and blockers. Then continue ordinary Drive
from integration state, or start the next eligible batch. Release remains a
separate explicit workflow.
