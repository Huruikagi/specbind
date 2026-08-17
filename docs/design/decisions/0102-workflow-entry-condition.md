# 0102: Fix when a request enters the workflow

Status: Accepted

## Context

[Decision 0097](./0097-discovery-routing-and-read-models.md) classifies a new
Roadmap work item as Direct, an existing Spec update, or a new Spec. It assumes
the request is already known to be milestone work. Nothing decides that.

The gap surfaced in the first forward test. Asked to fix a typo in a README
title, an agent read the project-instruction block, read the discovery skill,
and then made the edit directly, reasoning that routing a one-line typo through
milestone scoping would itself exceed the request. Nothing in the accepted
decisions contradicted it, and nothing supported it either.

Both possible defaults are bad if adopted silently. If every change must become
a Roadmap item, a typo fix costs a milestone, and users learn to work around
SpecBind rather than through it. If any change may be judged too small, the
judgment has no floor and real specification work can leave through it.

[Decision 0047](./0047-sparse-direct-change-status.md) supplies the missing
anchor. A Direct item exists so that release preflight has "one durable
indication that each direct item was performed" — that is, so the release record
is complete. The question is therefore not how large a change is, but whether it
belongs to what the milestone delivers.

The inherited system already worked this way, which is why nothing felt wrong
until a test looked. `kiro-discovery` recommended "direct implementation without
creating a spec" for a standalone small request and filed nothing at all; it
wrote direct work into `roadmap.md` only when the request also produced new
Specs, and explicitly declined to create roadmap entries for an existing-Spec
update plus direct work. So a standalone direct change was never recorded
anywhere. SpecBind did not decide against that; it decided the classification
and left entry unstated, and the behavior was lost by omission.

This decision restores the boundary and moves its axis. cc-sdd filed direct work
when it needed coordinating with other items. SpecBind files it when it belongs
to the delivery being released, because Decision 0047 gave the Direct item a
release-record purpose the inherited checkbox did not have.

## Decision

### SpecBind is not a gate on every change

A repository containing SpecBind does not become a repository where every edit
requires a milestone. Work enters the workflow when it belongs to a tracked
delivery or touches what a Spec owns, and not otherwise.

### Entry is mandatory when any of these holds

No judgment is available here, and no size exemption applies:

- The request changes a Spec's Requirements, Design, or Contract.
- The request changes behavior an existing Spec owns, whether or not the
  artifacts have caught up.
- The request modifies a path declared in some Spec's Contract File Ownership.
  Decision 0056 declares exactly the boundaries where a change could affect
  another Spec's design or verification, so a change there is Spec work by the
  project's own declaration.
- The user framed the request as part of the active milestone, or as work whose
  completion the release should record.

The third rule is what keeps this from becoming a size heuristic in disguise. It
is a property of the repository the project itself declared, not an estimate of
effort, and a one-line change to an owned path enters exactly as a large one
does.

### Otherwise the work is done directly, and said so

A request that satisfies none of those is performed as ordinary work, outside
the workflow. No milestone, no Roadmap item, no Brief.

The agent states that it is doing so and why, in one sentence. That sentence is
the whole safeguard: the user can answer "actually, track that," and a decision
made silently is one they never get to correct. Announcing it also keeps the
exemption from being a place where work quietly disappears.

This is not the same as a Direct item. A Direct item is milestone work that
happens to need no Spec; out-of-workflow work is not milestone work at all.
Decision 0097's classification applies only after entry, and its "no size
heuristic" rule is unchanged: it governs Spec-versus-Direct, never entry.

### Ambiguity enters

When it is genuinely unclear whether a request touches something a Spec owns,
the request enters the workflow.

The two errors are not symmetric. Conscripting a typo into a milestone wastes
ceremony and is visible immediately. Letting real Spec work out means behavior
changed with no requirement, no coverage, and no record — the failure SpecBind
exists to prevent, and one that surfaces only when something later depends on
the specification being true.

Asking the user is always available and is better than either guess.

### Out-of-workflow work still shares the repository

Work done outside the workflow lands in the same worktree that milestone
operations read. Decision 0054 requires a clean repository to create a
milestone, and completion and Direct-completion commands resolve a revision from
the current state.

An agent that makes an out-of-workflow edit while a milestone is active leaves
the worktree dirty for whatever runs next. It says so, and does not commit on
the user's behalf to tidy up: [Decision 0101](./0101-project-adapter-directory-and-git-workflow.md)
gives the Git adapter authority over checkpoints, and this work is not one of the
eligible checkpoints that decision lists.

### Where this is stated to the agent

The Decision 0099 project-instruction block names the entry condition, because
it is the one place an agent reads before it has chosen a skill. A skill cannot
state when it should have been invoked; by the time it is read, it has been.

The block gains one sentence naming what enters. It does not gain the rules
above: this decision is their home, and duplicating them into always-loaded
context is the drift Decision 0099 refuses.

`specbind-discovery` states the classification for work that has entered, and
says plainly when a request it was handed does not need the workflow at all
rather than scoping it anyway.

## Consequences

- The most common interaction with a SpecBind project — a small change to
  something no Spec owns — stops being a reason to route around the product.
- The exemption has a floor that is a declared property of the repository rather
  than a judgment about effort, so it cannot widen under pressure.
- Contract File Ownership acquires a second, load-bearing use, which raises the
  cost of leaving it stale. That is a fair trade: a Spec that has not declared
  its boundaries was already unable to protect them.
- An out-of-workflow decision is visible to the user in the moment it is made.
- Decision 0097 keeps a clean scope: it classifies, and no longer implies that
  everything must be classified.
- An inherited behavior that was dropped by omission rather than by decision is
  restored deliberately, with a stated reason for the axis it now uses.

## Implementation status

Implemented. The Decision 0099 project-instruction block names the entry
condition in one bullet, and `specbind-discovery` gains an entry check before it
reads the project shape: the four mandatory rules, the File Ownership rule stated
as something to check rather than judge, the one-sentence hand-back, the
enter-when-unclear bias, and the note that handing work back while a milestone is
active leaves the worktree dirty.

The forward-test scenario that produced this decision is corrected in
[Skill forward tests](../../skill-forward-tests.md). D1 now expects the README
typo to be fixed with no milestone at all, and to be announced rather than done
silently; a new D2 covers a Direct item that does enter because the user framed
it as release work.
