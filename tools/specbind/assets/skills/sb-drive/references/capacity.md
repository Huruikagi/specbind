# Preserve Drive dispatch-capacity handoffs

When a `handler.kind=skill` receiver cannot start because the host's finite
agent or thread capacity is full:

1. Consume completed Drive-owned receiver results first. If the host exposes a
   safe release operation, release only a completed receiver whose result was
   consumed and which this Drive run will not continue. Never release an
   unrelated receiver or invent a release operation.
2. Continue an addressable receiver only when that already-started run lacks
   its terminal status, and only for the same owning Skill, action, item, and
   handler mode. Never repurpose it as another owner or internal role.
3. If the owner never started and capacity remains unavailable, record
   `EXTERNAL_BLOCK`, reread Git and milestone state once, and do not retry that
   unchanged dispatch in this run. Do not execute owner work in the Drive
   context, change the handler target, or ask the user to manage receiver slots.

For an owner that never started, retain a restart capsule with these exact
fields:

- `Handler target and mode:`
- `Action, command operand, and item:`
- `Supplied authority:`
- `Project cwd, executable, and PATH facts:`
- `Capacity evidence:`
- `Git state:`
- `Resume owner:`

Return every label with its exact value, using `none` rather than omission. A
narrative equivalent is not a restart capsule. Invent no finding ledger or
retry budget for work that did not start. When no other safe action avoids the
unavailable capacity, return the labeled capsule immediately as the terminal
Drive result; do not leave the turn open for capacity to change.

When an owning Skill returns its own capacity restart handoff, preserve that
handoff **verbatim** as owner-owned continuation state. Append the independent
`git status --short` and milestone-status evidence; do not replace the owner's
exact paths, finding IDs and dispositions, used or remaining budgets, supplied
or omitted authority, resume role, or execution facts with a summary. A
mechanical contradiction blocks the handoff instead of authorizing repair.

If the returned handoff names unapproved or partial paths, the shared worktree
is unsafe: stop after the reread and include the complete owner handoff in the
final report. A clean pre-owner capacity block may continue only to another safe
action whose handler does not need the unavailable receiver capacity. A later
Drive run cannot infer non-durable finding, budget, or authority facts from
status; it needs the prior restart capsule when the owner requires them.
