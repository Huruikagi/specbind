# 0159: Close forward-test usability boundaries without moving semantic authority

Status: Accepted

## Context

The Claude Code forward-test batch against `1736d0c` passed X1, I3, RT1, and
DB1, but the paths to those results exposed several places where two valid
product surfaces admit different actions. The findings are recorded in
`docs/skill-forward-tests/results.md` and were reproduced against the installed
assets and CLI rather than accepted from driver narration.

Four findings can lead to a wrong mutation or owner: task review is both
read-only and a writer of deferred findings; contract review does not say who
confirms a Design rewind; milestone status displays a typed Direct item key
while Direct mutations accept the local identity; and status calls structurally
consistent state simply `Health: consistent` even though prose-level semantic
alignment is not machine-evaluated.

Four more findings make recovery or subject selection depend on inference: an
unrecognized nested command can receive an unrelated similarity suggestion;
directly invoked review and debug skills do not resolve omitted subjects;
Direct completion can leave its CLI-owned Roadmap mutation outside checkpoint
policy without explaining the resulting revision state; and the default
`contract-principles` Rule contains an authoring prompt where a consumer expects
current policy.

Two observations do not justify new product behavior. Translating the installed
Skill name `specbind-status` into `specbind status` occurred in an Agent-tool
environment that did not register fixture Skills; the installed instructions
and Skill commands already distinguish those names. The deferred adapter's
admission clauses are also complementary: work required for the current verdict
is blocking, while real actionable work that does not change the verdict may be
deferred.

## Decision

### Review remains evidentiary until its verdict

`specbind-review-task` performs all observation and verdict formation without
changing the repository. Its before/after worktree comparison happens before
any deferred record is written. After the verdict is fixed, a `DEFERRED` finding
may produce exactly the adapter-directed destination mutation accepted by
Decision 0122. That mutation is not part of the implementation diff under
review, may not change the verdict, and is reported separately.

This narrow post-verdict exception supersedes Decision 0111 only where its
boundary says review “authors nothing” or “records nothing.” Review still never
fixes implementation, writes lifecycle or execution state, updates
Implementation Notes, or leaves generated probe output.

### Every gate rewind has explicit human authority

Contract review may identify a missing owned seam without changing milestone
scope. If remediation requires any Requirements, Design, or Tasks gate
invalidation, it presents the target Spec and the complete downstream loss,
then obtains explicit user confirmation before invoking the invalidation.
Whether milestone scope changes materially does not weaken this rule.

Scope updates retain Decision 0108's existing confirmation boundary. Review
acceptance remains the reviewer's judgment and still needs no approval mode;
authority for a destructive rewind is separate from authority to record a
passing assessment.

### Status exposes operands and the limit of health

Spec-backed and Direct milestone item actions retain their typed item key, such as
`direct:contributing-guide`, and additionally expose a `command_operand` that is
accepted verbatim by the owning mutation command. Spec actions use the canonical
Spec identity; Direct actions use the Roadmap-local Direct identity. The text and
JSON projections expose the same field. Milestone-wide actions that require no
item identity omit it.

Spec and milestone text status label their deterministic result `State health`
rather than unqualified `Health`. Both text and JSON also expose
`Semantic alignment: not evaluated` / `semanticAlignment: "not_evaluated"`.
This is not a new semantic checker. CLI health continues to cover schemas,
lifecycle evidence, freshness, declared traceability, and other mechanically
decidable diagnostics. Requirements, Design, Contract, Steering, and
implementation prose alignment remains agent judgment under Decision 0094.

### Recovery and omitted subjects fail closed

The CLI disables similarity tips that compare only one command token and can
point at an unrelated top-level command instead of a valid nested route. The
ordinary command help and usage remain available.

Direct invocation of `specbind-review-task` or `specbind-debug` uses an explicit
Spec and Task when supplied. When omitted, the skill reads the active milestone
and task projections. It may select a subject only when exactly one candidate
satisfies the skill's stated phase and failure/review shape; otherwise it
presents the candidates and asks the user. Repository path or “only item I
noticed” inference is not identity authority.

### Direct metadata and Rules state their current policy

A successful Direct handshake always changes the CLI-owned Roadmap after the
clean implementation revision was established. The implementation skill
re-reads the Git adapter and creates a separate metadata checkpoint when its
active guidance covers completion metadata or each eligible workflow unit. An
absent, scaffolded, or narrower adapter authorizes no such checkpoint; the skill
then reports the Roadmap path, dirty revision consequence, and the policy reason
instead of implying completion left a clean checkpoint.

The embedded `contract-principles` Rule is live default project policy, not a
template and not an inactive scaffold. Its compatibility section therefore
states an actual conservative default. Every other section also states current
policy rather than asking a consumer to author missing project policy. The
default declares no project-specific dependency direction and forbids an agent
from inventing one. Ownership overlap must be resolved to one owner unless the
project explicitly records an exception; a dependency cycle is judged and
explained case by case rather than failing automatically. Projects may replace
or relax these defaults through the existing Rule customization surface; Rules
gain no scaffold marker or new state.

## Consequences

- Task review has one explicit, late, adapter-bounded write without contaminating
  the evidence or implementation under review.
- A review cannot silently discard accepted Design or downstream state merely
  because the milestone membership is unchanged.
- Direct commands no longer require callers to reverse-engineer a displayed
  typed item key.
- Status remains mechanically authoritative without claiming that free-form
  artifacts agree semantically.
- Bare review and diagnosis requests stop rather than selecting among several
  plausible Tasks.
- Default projects receive an actionable seam policy and a clean Direct metadata
  checkpoint; customized adapters retain their narrower authority.
- Agent-tool Skill-registration limitations and the logically complementary
  deferred criteria remain measurement context, not new compatibility aliases or
  lifecycle behavior.

## Implementation status

Implemented by the product-managed review, contract-review, debug, implement,
and status Skills; the milestone and Spec status projections; the default Git
and Contract Rule interpretation; CLI parser feature selection; and focused
contract and CLI tests.
