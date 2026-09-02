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
incomplete, but invoking Drive grants no Gate approval. Without applicable
authority, it parks that item and looks for another reachable action.

## 3. Drive reachable work

```text
$sb-drive
```

Drive selects only actions exposed by `specbind milestone status --json` and
delegates one at a time to the owning workflow. It does not author Requirements
or Design itself and does not batch-complete Tasks.

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

## 4. Understand attention versus stopping

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
stashing them.

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
