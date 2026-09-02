# 0185: Record reverse deferred findings after milestone creation

Status: Accepted

## Context

Decision [0181](./0181-reverse-spec-establishment.md) lets reverse Discovery
record a suspected defect through an active Deferred Findings Adapter. The
reverse procedure currently performs that write while collecting evidence,
before the maintainer confirms the complete proposal.

That order conflicts with the clean repository baseline from Decisions
[0054](./0054-milestone-baseline-revision.md) and
[0089](./0089-milestone-creation-cli.md). A tracked local adapter destination
makes the worktree dirty, so `milestone create` cannot capture the fixed
evidence revision. Committing the finding first changes that revision, while
stashing it introduces an unowned operation outside the documented workflow.

## Decision

- During evidence collection, reverse Discovery discovers the Deferred
  Findings Adapter through `adapter list`, verifies the exact active selector
  and local destination through its read surface, and prepares each suspected
  defect with its fixed source revision, locator, and claim. It does not write
  the destination yet.
- The complete reverse proposal identifies those findings as pending adapter
  records. They remain observations rather than scope, Requirements, or
  confirmed bugs.
- After confirmation, `milestone create` remains the first repository mutation.
  Its baseline revision must equal the fixed preflight source revision.
- Only after successful milestone creation and provenance verification may
  reverse Discovery write the pending findings to the same verified local
  destination. It rechecks that the reported selector is still active and
  deduplicates by source revision, locator, and claim before writing.
- The finding destination joins the reverse Discovery checkpoint with the
  Roadmap, Spec state, Briefs, Research, and temporary adoption record. It is
  not a general dirty-worktree exception and does not authorize external
  transmission or push.

## Consequences

- The load-bearing clean baseline remains intact while a required local finding
  can survive the same reverse run.
- The proposal remains honest about suspected defects without treating an
  adapter write as pre-confirmation scope mutation.
- A failed milestone creation leaves no new deferred-finding record to unwind.

## Verification

Focused Skill tests require selector discovery, a pre-creation no-write
boundary, a post-creation adapter recheck, and one reverse Discovery checkpoint.
A fresh reverse fixture with an active local Deferred Findings Adapter verifies
that the proposal leaves the repository clean and the confirmed continuation
records the finding only after the milestone exists.
