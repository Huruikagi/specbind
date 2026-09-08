# Isolated Spec implementation handoff

This optional route owns one implementation batch and stops at a branch handoff.
It does not integrate results or complete the Milestone. Ordinary sequential
Drive remains the fallback when isolation is unavailable before work starts.

## 1. Check whether a batch can run

If previous work for the selected Specs is known to be retained, stop and report
its branches/paths before any new dispatch, including sequential fallback.
This route does not automatically recover interrupted batches. The maintainer
must identify and handle prior results before requesting a new batch; unclear
ownership is a stop condition, not permission to start duplicate work.

Use the current checkout's `specbind milestone status --json`. Select up to the
requested limit of already-actionable, independent Spec-backed `sb-implement`
items in status order. Confirm independence from their approved inputs and
verification prerequisites, not paths alone. Other actions, or fewer than two
eligible Specs, use the ordinary sequential workflow.

Use only verified host tools that can start complete owning workflows in
separate worktrees at the same clean committed base, retrieve their results,
and retain named branches and dirty work after return or interruption. Check
project-local instructions, executable/PATH and test environment in each worker,
and read `specbind adapter read git --for consume` for Task checkpoint policy.
No compatible checkpoint policy or unverified isolation means continue
sequentially before dispatch; generic Skill support alone is insufficient.
Do not install tools or change permissions to enable this route.

Lack of isolated worker APIs does not disable ordinary fresh-role dispatch.
Fallback executes the same owning Skills and their ordinary review procedures;
genuine no-subagent hosts keep the established disclosed compatibility path.
A registered role's start failure remains an environment failure.

## 2. Run one isolated batch

Pin the current checkout's full clean HEAD and give each worker its exact Spec,
base, retained branch/worktree, project instructions, executable and authority.
Verify each worker is clean at that base before it mutates anything. Delegate
the complete `sb-implement` Skill: its sequential Task cycle owns implementation,
fresh review, CLI Task progress, tests and per-Task checkpoints. Internal roles
work inside their owner's checkout. An internal review-ready return is not done.

Workers only change their Spec's implementation scope. They do not replan,
change approved inputs, validate whole-Spec completion, or write to the original
checkout. While workers run, dispatch no other mutating work. With `--replan`,
ordinary recovery may run before a batch; findings from the batch are reported
for subsequent planning after this handoff, not repaired within this run.
If dispatch partly fails, collect or report the already-started workers and
stop; do not switch their items to shared-checkout sequential execution.

## 3. Retain results and stop

Check each returned branch's base/result commits, changed paths, Task state and
verification evidence against its handoff. Report successes and blockers with
the retained branch/worktree and any dirty partial work. A blocked worker's
result remains useful work to retain, not permission to discard or invent a WIP
commit. Preserve the owner's ordinary retry limits.

Do not merge, cherry-pick, rebase, create an integration candidate, or delete
worker worktrees/branches in this route. Do not copy worker Task state or
completion evidence into the original checkout. Branch-local completed Tasks
are not Milestone completion or permission to start dependent Specs.

Stop after this single batch, including when every worker succeeds. Report that
integration requires a separate maintainer-directed operation, and do not start
another batch, downstream implementation, final validation or Release. After
integration is handled separately, a new ordinary Drive run uses fresh CLI state
and the existing validation rules. No merge queue or new progress ledger exists.
