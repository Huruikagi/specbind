# Task review protocol

This protocol is the shared semantic baseline for reviewing one implemented
task. It applies to every supported agent and cannot be waived by a project
template or shared rule.

One standard serves two moments: the review dispatched during implementation and
an independently invoked review of the same work. The authority differs; the
criteria do not.

This protocol owns the verdict and what supports it. Review mode, retry rounds,
who is dispatched, and what happens after a rejection belong to the owning
skills. The CLI owns the plan, the requirement mapping, and the execution state.

## Read the change, not the account of it

The implementer's report says what it believes it did. The diff says what
happened. Review the diff.

- Read the actual change with the repository's own tools before forming any
  view.
- Read the Requirements the task carries and the Design sections that govern it,
  from the artifacts rather than from the implementer's summary of them.
- Treat the report as a claim to verify. A report and a diff that disagree is
  itself a finding, and the diff is what is true.

A review that only restates the report has reviewed nothing, and it is worse
than no review because it produces a verdict that later work will trust.

## The verdict is about this task

The question is whether **this task**, as specified, is now done correctly.

- Does the change satisfy every Requirement ID the task carries, in behavior
  rather than in intent?
- Does it satisfy the task's explicit completion criteria, or the observable
  condition standing in for them?
- Does it realize the Design, including the parts the Design fixes that the
  change was free to ignore?
- Is it complete, or does it leave a path unhandled that the requirement covers?

Work that belongs to a different task is out of scope. Noting it is useful;
rejecting this task for it is not.

## Correctness before style

Rank findings by what would change the verdict.

- **Rejecting**: the behavior is wrong, a requirement is unmet, a case the
  requirement covers is unhandled, verification was weakened to pass, or the
  change breaks something that worked.
- **Rejecting**: the change contradicts the Design or the architecture it enters
  in a way the change does not resolve.
- **Not rejecting on its own**: naming, formatting, or a preference the project
  does not state. Say it, and say that it is not the reason for the verdict.

Weakened verification deserves its own attention, because it converts a failing
task into a passing one without changing behavior. A deleted assertion, a
loosened tolerance, a skipped test, or a check narrowed to avoid a failure is a
rejection unless the change makes the check genuinely obsolete and says so.

## Findings must be actionable

A finding names the requirement or behavior it endangers, points at the specific
place in the change, and states the consequence. "This is fragile" cannot be
acted on or disputed; "this drops the second SKU when the cart already holds it,
so 1.2 is unmet" can.

Recognize what is right when it is true. A reviewer that only accumulates
objections gives the implementer no signal about what to keep, and the next
attempt rewrites work that was already correct.

## Every finding gets a disposition

A finding raised in this review ends in exactly one of three states, and the
reviewer names which one. There is no fourth state in which a finding is
mentioned in passing and then carried nowhere.

- **Blocking.** It changes the verdict. It is resolved before the gate is
  crossed.
- **Resolved in place.** It was examined and needs no work, and the reason is
  stated. A judgment made and explained is not an outstanding finding.
- **Deferred.** It is real and actionable, it does not change the verdict, and
  it is written to the destination this project names for deferred findings.

A finding stated in the report and given no disposition is volatile: nothing
downstream carries it, and a reviewer who knows this raises the next one as
blocking to make it survive. Severity inflation is the predictable result of
having nowhere to put a true observation, so the disposition is not optional
bookkeeping.

Deferring is not a way to pass a review that should not pass. A finding that
changes the verdict is blocking whether or not it is convenient, and moving it
to the destination does not settle it.

A project that names no destination has none. State the deferred finding in the
report and say that it is not recorded anywhere, rather than promoting it or
discarding it silently.

## The verdict is one of three

- **Approved.** The task is done correctly. Nothing outstanding blocks it.
- **Rejected.** Something specific and stated must change. Every rejection names
  what would make it approvable.
- **Cannot review.** The change cannot be judged as presented — it is entangled
  with unrelated work, the artifacts contradict each other, or the task's
  meaning is undetermined. This is a real outcome and not a soft rejection;
  reporting it is more useful than guessing.

Uncertainty is never an approval. An approved verdict asserts that the work was
checked and found correct, and everything downstream relies on that assertion
having been made deliberately.
