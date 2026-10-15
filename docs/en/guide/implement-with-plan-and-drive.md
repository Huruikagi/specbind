# Plan and Drive a Milestone

This guide advances an active Milestone containing several Specs or Direct
items from accepted planning through implementation validation. The normal
recommended route is to review the complete plan with `sb-plan --all`,
then let `sb-drive` advance every safely reachable action.

Drive never runs Release. When one branch needs attention, it continues an
independent branch when safe and accumulates required decisions until reachable
work is exhausted.

## 1. Confirm the prerequisites

Discovery must already have confirmed scope, Specs, Direct items, and
dependencies and created an active Milestone. Start with a clean worktree.

```text
$sb-status
```

Drive rereads current CLI state throughout the run. It does not treat an older
conversation or a previous Drive report as authoritative progress.

## 2. Plan the complete Milestone

```text
$sb-plan --all
```

Plan processes Requirements for every selected Spec, Design and independent
validation in dependency order, one Milestone-wide Contract review, and Tasks.
It stops after Tasks approval. Direct items have no Spec artifacts and are not
part of this planning scope.

At the start, Plan asks whether Requirements, Design, and Tasks Gate approvals
may be delegated to this named run. Delegation combines confirmation points;
it does not skip reviews or CLI checks.

Separating planning lets you review Milestone behavior, Design boundaries, and
execution order before implementation. Drive may also enter while planning is
incomplete, but invoking ordinary Drive grants no Gate approval. Without applicable
authority, it parks that item and looks for another reachable action.

## 3. Drive reachable work

```text
$sb-drive
```

For ordinary scheduling, Drive selects actions exposed by `specbind milestone status --json` and
delegates one at a time to the owning workflow. It does not author Requirements
or Design itself and does not batch-complete Tasks.

A Drive request includes recording completion on `GO` and its Git-adapter
checkpoint for the actionable Specs, without another completion confirmation.
An explicit instruction to stop before completion recording or make no changes
limits that authority. Planning Gate approvals and Release remain separate.

Typical ownership is:

| State | Owning workflow |
| --- | --- |
| Incomplete planning | `sb-plan` and its phase workflows |
| Contract review | `sb-contract-review` |
| Implementation | `sb-implement <item-id>` |
| Whole-Spec implementation validation | `sb-validate-implementation <spec-id>` |
| Release boundary | Report status and stop |

After every handoff, Drive rereads Git worktree state and Milestone status. The
initial implementation runs only one mutating workflow at a time.

### Delegate replanning discovered during implementation

```text
$sb-drive --replan
```

This option delegates Design, Contract, and Tasks corrections within the
approved Requirements and existing Milestone scope. The option itself grants
that authority; Drive does not ask for another delegation confirmation. You
can also explicitly ask it to delegate replanning in natural language.

Drive returns findings to the planning owner, which invalidates the necessary
gates, revises artifacts, runs independent validation and Contract Review, and
reapproves the plan. Drive then resumes implementation and validation in the
same run. It reports changes without stopping merely because the plan changed.

If this replan makes another participant's accepted completion stale, Drive
checks the recorded clean baseline and actual planning changes, then delegates
completion-only withdrawal to `sb-validate-implementation`. Each withdrawal
gets its own checkpoint. Drive resumes the remaining implementation and waits
for all Milestone work to finish before scheduling fresh validation. This
aftercare needs no additional confirmation, but pre-existing staleness, other
inconsistencies, missing history, or uncommitted target metadata stop automatic
withdrawal. Requirements, Design, Tasks, and their approvals are preserved.

Design changes can require renewed Contract Review and Tasks approval across
the Milestone. Previously completed work may need implementation and proof
again; unchanged work keeps its progress only when its evidence still applies.
Attributable partial implementation can remain through this recovery, with
planning leaving it untouched until implementation resumes. Unrelated or
conflicting changes still stop the run.

Requirements, Milestone scope or dependency changes, out-of-scope Spec changes,
and unsettled external compatibility or migration obligations still need your
decision. Reviews and their retry limits remain in force, and Release remains
separate. Omit `--replan` to keep the ordinary rewind confirmations.

## 4. Understand attention versus stopping

Once several Specs reach final validation, one validation owner assesses each
at the same committed revision. Only after every target passes does it accept
and commit their completion records together. A failed or unavailable required
check stops acceptance for the whole set; a passing Spec is not recorded early
while another still needs repair.

If HEAD changes or the worktree becomes dirty before acceptance, no completion
is recorded and Drive stops with the affected Specs, old and current revisions,
and changed paths. Resolve the intervening work, then invoke Drive again. A
fresh validation owner repeats all preflights, checks and per-Spec assessments
for the current set; earlier candidates and verdicts are not reused.

An item that cannot advance does not necessarily stop the whole run.

```text
Item A needs a decision
  -> park A and its descendants
  -> continue independent item B when safe
  -> report the decision for A after reachable work is exhausted
```

Drive separates the cause from the scheduler disposition:

| Example cause | Treatment |
| --- | --- |
| `HUMAN_DECISION` | Park until the user decides scope, meaning, authority, or an irreversible consequence |
| `BLOCKED` | Park an item its owning workflow established cannot progress |
| `WAITING` | Wait for a dependency or Milestone-wide barrier |
| `REROUTABLE` | Return to an upstream owner such as Requirements or Design |
| `EXTERNAL_BLOCK` | Park or stop because the environment cannot meet a prerequisite |

Another safe action yields `CONTINUE_ELSEWHERE`; none yields `STOP_RUN`. An
unfinished Design prevents Contract review but not another Spec's reachable
Design. An unfinished implementation prevents its descendants and Milestone
completion but not independent implementation.

An unsafe worktree is different. Partial, rejected, unrelated, or unattributed
changes make switching ownership unsafe, so Drive stops without resetting or
stashing them. The exception is the attributable implementation carried through
the same authorized replan described above; it cannot be used to switch to
independent work.

## 5. Review the Drive handoff

The final report identifies:

- the Milestone boundary reached;
- completed owning workflows and authoritative state gained;
- each attention item, its cause, and affected descendants;
- decisions or external conditions needed to resume;
- the next safe action, if any; and
- confirmation that Release did not run.

Attention is a run-local report, not a persistent queue. After resolving a
decision or external condition, invoke `$sb-drive` again. It reconstructs
reachable work from current state.
For an interrupted replan recovery, also supply the previous handoff, including
baseline evidence, completed withdrawal checkpoints, and any remaining Task
recovery budget. Current status alone cannot reconstruct those facts.

## 6. Stop before Release

When implementation validation is complete, the next boundary is release
preparation. If the target version is unbound, Drive does not choose one. To
authorize guarded binding while still stopping before Release, supply it
explicitly:

```text
$sb-drive --target-release 1.2.0
```

This still does not build, publish, verify, or finalize a release. Drive
completes at `release_ready`. Review the result, then use
[Release a milestone](./release.md) as a separate explicit workflow.

## When to use this route

- Advance a Milestone with several Specs or Direct items in dependency order.
- Continue independent work while one branch awaits attention.
- Delegate post-Tasks implementation and validation while retaining CLI checks.
- Resume from fresh state without maintaining a persistent queue.

To review every artifact and Gate separately, use
[Plan and implement one item at a time](./implement-step-by-step.md).

---

[User guide](../index.md) | [Plan and implement one item at a time](./implement-step-by-step.md) | [Release a milestone](./release.md)
