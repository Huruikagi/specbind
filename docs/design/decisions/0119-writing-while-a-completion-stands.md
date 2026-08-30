# 0119: State the cost of writing while a completion stands

Status: Accepted

[Decision 0165](./0165-release-binding-preserves-completion.md) adds one exact
exception to the general rule below: a CLI-shaped active-Roadmap
`target_release` bind or rebind preserves completion freshness. Ordinary
managed-Markdown authoring remains completion-invalidating.

## Context

Three accepted contracts state a version of one caution, each having found it
from its own direction. [Decision 0115](./0115-release-skill-contract.md) found
it in release binding, where a one-line Roadmap edit invalidates every accepted
completion. [Decision 0117](./0117-steering-authoring-contract.md) found it in
steering, and used it to fix when the steering recommendation belongs.
[Decision 0118](./0118-gap-analysis-skill-contract.md) found it in writing
Research, and deferred the general statement on the grounds that three instances
show the pattern and a fourth removes the need to guess its shape.

`specbind-quick-plan` and `specbind-batch-plan` are the fourth. They author Requirements,
Design, and task plans across a milestone that may already contain a completed
Spec, and would otherwise restate the same derivation a fourth time.

The rule is not a property of Roadmaps, steering, Research, or task plans. It is
a property of how completion evidence is scoped, and it reaches every skill that
writes.

## The rule

Under [Decision 0080](./0080-v1-task-contract-and-completion-details.md)
completion evidence is project-revision-scoped, and the only project change it
tolerates is a Spec's own `implementation` to `release_ready` transition in
`spec.yaml`.

**Once any participating Spec holds accepted completion evidence, every other
change to the project stales it**, and that Spec's completion handshake must be
re-run before the milestone can be released. This holds regardless of what was
written, where, or how obviously unrelated it looks — Decision 0080 declines to
infer semantic non-impact from path boundaries, deliberately.

The condition is observable without a new command. A Spec holds accepted
completion exactly when its state is `release_ready`, which `specbind milestone
status` already reports in its `Spec states` line.

## What a writing skill owes

- **Check before writing, not after.** The cost is identical either way, but a
  skill that reports it afterwards has spent the user's work rather than offered
  them the choice.
- **Name it concretely.** Which Specs lose their evidence, and that each needs
  its completion handshake re-run. "This may affect completion" is not usable.
- **Do not refuse.** The write stays available and the decision is the user's. A
  milestone in which nothing can be corrected after its first completion is
  worse than one where corrections are known to cost something.

In the ordinary ordering the question never arises, because authoring precedes
implementation. It arises in the milestone that has partly finished, which is
exactly where an unannounced re-validation cycle is most expensive.

## Where the instruction lives

The `okf-authoring` protocol carries it. Its declared consumers under
[Decision 0094](./0094-embedded-product-protocols.md) are "every skill that
creates or rewrites managed Markdown", which is the population this rule reaches,
and it already carries a comparable non-structural caution about `log.md`
insertion. One paragraph there reaches every authoring skill with no per-skill
duplication, which is the outcome this decision exists to produce.

Skills therefore state nothing about it. A skill contract that restates the
derivation is the defect this decision closes, not the way it is implemented.

## What was considered and not done

**A new `milestone status` field.** `release_ready` in the existing `Spec states`
line already carries the condition, and a second derived line would restate data
the command already prints, against the concise result contract of
[Decision 0067](./0067-text-first-english-cli-results.md). If agents demonstrably
fail to make the one inference, adding the field is a small, separable change.

**A guard that refuses the write.** Fail-closed is right for reads of state known
to be incomplete, and wrong here: it would make the CLI refuse ordinary authoring
to protect evidence the user may be willing to spend.

**Persisting anything.** The condition is derived from state that already exists.
A stored flag would be a second authority over a fact `spec.yaml` already owns.

## Consequences

- The caution is stated once, and the next authoring skill inherits it instead of
  rediscovering it.
- Every skill that writes managed Markdown receives it through a protocol it
  already reads, so the reach is wider than the three contracts that stated it
  individually — discovery, requirements, design, and tasks were never covered.
- Decisions 0115, 0117, and 0118 keep their own statements. Each uses the rule to
  fix something specific to its skill: when the release recommendation belongs,
  which window makes steering free, when Research may be written. Those remain
  correct and are not edits waiting to happen.

## Implementation status

Implemented. The `okf-authoring` protocol states the rule, the condition to check,
and what the skill owes the user before writing.

`specbind-steering` and `specbind-gap-analysis` now read `okf-authoring`. Both
author managed Markdown and were consumers under Decision 0094's mapping without
naming the selector, which this decision makes load-bearing rather than merely
inconsistent. `specbind-gap-analysis` keeps its Research-specific note and drops
the derivation the protocol now owns.
