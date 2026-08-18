# 0109: Make fresh-context subagent dispatch a product contract

Status: Accepted

## Context

The inherited cc-sdd skills use subagents structurally rather than
decoratively. `kiro-impl` dispatches a fresh implementer per task, an
independent reviewer after it, and a debug agent on failure, with bounded retry
rounds between them. `kiro-spec-design` dispatches parallel research and
synthesizes the summaries. `kiro-discovery` and `kiro-spec-requirements` do the
same at smaller scale.

SpecBind has re-authored four of those skills and dropped subagents from every
one, without a decision recording the choice. No accepted decision mentions
dispatch at all, and no embedded skill contains the word.

That is not a neutral omission.
[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) already depends on
the mechanism it never names: it defines `specbind-debug` as a "read-only,
**fresh-context** root-cause protocol" and bounds automatic remediation at two
rounds. Fresh context is not a style preference — it is what stops a failed
attempt's reasoning from being carried into the retry that is supposed to
correct it.

### The platform difference is three lines

The reason to avoid dispatch would be that it cannot be expressed neutrally.
Measuring the inherited tree says otherwise. Comparing cc-sdd's two renderings
of the same skills:

| Skill | Claude Code | Codex | Differing lines |
| --- | --- | --- | --- |
| `kiro-impl` | 272 | 269 | 97 |
| `kiro-spec-design` | 202 | 207 | 81 |
| `kiro-discovery` | 262 | 262 | 80 |

Almost all of it is incidental: Front Matter that
[Decision 0096](./0096-skill-asset-layout.md)'s renderer already owns, heading
style (`## Role` versus `<background_information>`), the spelling `subagent`
versus `sub-agent`, and `first argument` versus `$1`. The only genuinely
platform-specific content is the dispatch instruction itself, three occurrences
in `kiro-impl`:

```text
Claude Code:  Dispatch via **Agent tool** as a fresh subagent
Codex:        Spawn a fresh sub-agent with this prompt
```

And the Codex rendering **names no mechanism at all**. Its design skill likewise
says only "dispatched as sub-agents". Both were reported to work well in
practice.

So the mechanism name is decoration. A neutral body that says to dispatch a
fresh subagent is understood by both platforms without naming a tool, and
splitting bodies per agent would reintroduce exactly the drift Decision 0096
exists to prevent — 97 lines of divergence surface bought for three lines of
real difference.

## Decision

### Dispatch is a product obligation where fresh context is the point

A skill dispatches a fresh subagent when the work must not inherit the
dispatcher's context:

- **Implementation of one task**, so the implementer builds its understanding
  from the approved artifacts rather than from the conversation that planned
  them.
- **Independent review of that implementation**, so the reviewer forms a verdict
  without having watched the work be justified.
- **Root-cause diagnosis after failure**, which receives the error and not the
  failed attempt history. This is where the obligation is sharpest: a retry that
  inherits the reasoning that just failed reliably reproduces it, and Decision
  0075's two-round bound assumes each round actually starts over.
- **Independent investigation that would otherwise flood the dispatcher**, such
  as parallel repository and dependency research before authoring a Design.
  Each returns a findings summary, not raw material.

Dispatch is not a general performance technique, and a skill does not fan out
work merely because it can. Where the dispatcher needs the whole picture,
synthesis stays in the main context — the inherited design skill is explicit
that its own synthesis step must not be delegated, and that remains correct.

### The neutral body names no mechanism

A skill body says to dispatch a fresh subagent and what to give it. It never
names a tool, an invocation syntax, an agent type, or a platform. Decision 0096
already requires the body to name another skill without an invocation prefix,
for the same reason; this extends that rule to dispatch.

The evidence that this is sufficient rather than merely tidy is that cc-sdd's
Codex bodies already name no mechanism and work.

### A dispatch carries a brief and a protocol selector

What the subagent is given is fixed, because this is the boundary where context
is deliberately withheld and an under-specified brief becomes an immediate
failure:

- a **self-contained brief**: the work to do, the exact artifact paths and
  identifiers it must read, the acceptance conditions, and the verification
  commands it may run;
- a **protocol selector** it reads through `specbind protocol read <selector>`,
  which carries the semantic baseline for the role it is performing.

The protocol is the payload rather than a skill name. A subagent can always read
a protocol, because the CLI is the one substrate both platforms share, whereas
whether a dispatched agent can load a named skill is a platform capability that
Decision 0096 deliberately declines to claim. Where a role needs a shared
semantic baseline that no protocol yet carries, the owning skill's decision adds
one under Decision 0094 rather than inlining it into a prompt.

The brief never assumes the subagent saw anything the dispatcher saw.

### The return is structured, and prose is not parsed

A dispatched subagent returns a result block the dispatcher can parse: an
explicit status drawn from a closed set the dispatching skill defines, plus
whatever that status requires.

When the block is missing, ambiguous, or replaced with prose, the dispatcher
re-dispatches **once**, asking only for the block. It never infers a verdict
from narrative, and never proceeds as though the work succeeded because nothing
said otherwise.

This is the same principle the CLI applies at every other boundary: a machine
decision is made from a machine-readable value. A dispatcher that reads intent
out of a paragraph has the same failure mode as an agent that decides a gate
passed because the diagnostics looked mild.

### When the host cannot dispatch

The obligation survives the mechanism. Where a host cannot spawn a subagent, the
skill performs the work in the main context and still honors what dispatch was
for: it reads the same protocol, works from the same brief, and — for the debug
case — explicitly discards the failed attempt's reasoning rather than carrying
it forward.

This fallback is a compatibility path, not the expected one. Both supported
agents dispatch today.

### Per-agent role registration stays deferred

Decision 0096 reserved "platform-specific subagent or skill-invocation adapters"
and `.codex/agents/` as separate installation surfaces it did not define. This
decision does not define them either, and does not need to: a dispatch that
carries its own brief and protocol selector requires no pre-registered role.

Registration buys pinning a model or reasoning effort to a role, and a durable
role description. Those are optimizations. They become worth an installation
surface when a role's instructions grow beyond what a brief should carry, or
when a project needs to pin capability per role; either is a later decision with
its own naming, refresh, and customization questions.

### Retrofit

The obligation applies to every skill whose work meets the conditions above, not
only to skills authored after this decision. `specbind-design` regains parallel
investigation under [Decision 0104](./0104-design-skill-contract.md)'s
discovery step. `specbind-implement`, `specbind-review-task`, and
`specbind-debug` are authored under this contract from the start.

`specbind-discovery` and `specbind-requirements` are deliberately left alone.
The inherited versions dispatched research there, but SpecBind's routing reads a
bounded set of small documents that Decision 0097 already fixes, and its
requirements phase reads steering whole under Decision 0100. Neither floods a
context, so dispatch would add coordination for no benefit.

## Consequences

- The mechanism Decision 0075 already assumed is stated, so `specbind-debug`'s
  fresh context and its two-round bound rest on something written down.
- One neutral body continues to serve both agents, so Decision 0096's guarantee
  is preserved rather than traded away for dispatch.
- The parent-child boundary is machine-readable, so a dispatching skill's
  control flow is decidable instead of inferred from prose.
- Briefs must be genuinely self-contained, which is a real authoring cost and
  the point: it is the same discipline that makes a Design readable after the
  milestone closes.
- Skills gain a failure mode they did not have — a subagent that returns
  nothing usable — which the single re-dispatch and the closed status set bound.
- No new installation surface is added, so `.codex/agents/` remains the one
  install-side question Decision 0096 left open.

## Implementation status

Accepted with `specbind-design` retrofitted: its investigation step dispatches
parallel research returning summaries, and keeps synthesis in the main context.

`specbind-implement`, `specbind-review-task`, and `specbind-debug` are authored
under this contract as they are embedded.
