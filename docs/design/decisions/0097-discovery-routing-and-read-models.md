# 0097: Fix the discovery routing contract and the read models it requires

Status: Accepted

## Context

[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) removed
`specbind-spec-init`, and [Decision 0089](./0089-milestone-creation-cli.md) made
milestone creation and scope replacement guarded CLI operations. Discovery is
therefore no longer a skill that writes roadmap and spec files itself; it is the
skill that decides *what the scope is* and hands a confirmed candidate to the
CLI.

The inherited `kiro-discovery` cannot be ported into that shape. It routes
through five lettered paths, decides Spec boundaries partly by task-count
heuristics, and writes `brief.md` and `roadmap.md` directly. It also carries
technology comparison and library investigation that SpecBind assigns elsewhere.

Two read gaps block the SpecBind version outright:

- There is no project-wide listing of persistent Specs. Deciding whether a
  request belongs to an existing Spec requires knowing which Specs exist and
  what state they are in, and no command answers that.
- `milestone status` cannot reconstruct the current scope. Its item view carries
  identity, summary, status, and unsatisfied dependencies only: the
  `new_specs`/`spec_updates` distinction is collapsed when the two categories
  are chained, and satisfied dependencies are absent. Because
  `milestone update-scope` takes a **complete** replacement candidate, composing
  one from that projection would silently drop category and dependency
  information.

## Decision

### Two read-only commands

```text
specbind spec list
specbind milestone scope
```

`spec list` reports every persistent Spec in the project, whether or not it
participates in the active milestone: canonical identity, declared lifecycle
state, and enough artifact availability and health to tell a usable Spec from
one needing repair. It requires no active milestone.

`milestone scope` writes the active milestone's current scope to stdout as a
normalized candidate document, in exactly the shape `update-scope --scope`
accepts. With no active milestone it reports no change.

This is a raw-content read, in the same family as `artifact read`,
`template read`, and `protocol read`: the document is written to stdout with no
result wrapper. It is **not** the general JSON result envelope that
[Decision 0074](./0074-defer-json-cli-output.md) defers. That decision defers
turning command *results* into JSON; this command echoes transient command
*input* so a replacement can be composed from the current value. Ordinary
`OK`, `NO_CHANGE`, and `ERROR` reporting is unchanged for every other command.

The emitted candidate omits `body`. Decision 0089 makes an omitted body preserve
the current Roadmap prose, so a read-edit-write round trip cannot accidentally
rewrite it. A caller that intends to change the body supplies one deliberately.

### Three kinds of work

Discovery classifies each part of a request as exactly one of:

| Kind | Criterion |
| --- | --- |
| Direct | It requires no Requirements, Design, or Contract change. |
| Existing Spec update | It changes behavior or a boundary an existing Spec owns. |
| New Spec | It requires a new durable responsibility and Contract boundary. |

Mixed work is not a fourth kind. A request spanning several kinds produces one
scope candidate containing several work items, which is what the Roadmap
represents anyway.

There is no size heuristic. A task count does not determine whether something
deserves its own Spec; ownership of a durable boundary does. A large change
inside one boundary stays one Spec, and a small change that creates a new seam
is a new Spec.

### Name the earliest affected gate

When a request touches a Spec that already has approved gates, discovery states
which gate the change invalidates, choosing the earliest one affected:

- Requirements change: `spec requirements invalidate`
- Requirements unchanged but Design or Contract changes: `spec design invalidate`
- Only the task plan changes: `spec tasks invalidate`
- No canonical artifact changes: the work is Direct, not a Spec update

### Rewind before changing scope

Discovery performs confirmed gate invalidations first, then creates or updates
the scope.

The reverse order leaves a window in which the milestone already claims the new
scope while a participating Spec still carries approved gates for the old one.
Those gates are genuinely fresh in the CLI's terms, because no artifact has
changed yet, so nothing would stop a concurrent approval from acting on a Spec
that is about to be rewound.

### Artifact authoring boundary

- Every Spec-backed work item gets an active Brief. When the same Spec already
  has a Brief in the same milestone, the new request is folded into it rather
  than creating a second one, per Decision 0062.
- Discovery does not author Requirements, for a new Spec or an existing one.
  That authoring belongs to `specbind-requirements`, and Decision 0089 already
  accepts that a newly created Spec holds only machine state until its owning
  skill runs. Creating an empty scaffold here would place an artifact before the
  skill that owns it has been invoked.

  The inherited two-stage shape is not evidence against this. cc-sdd initialized
  a spec with a `requirements-init` stub whose body was the raw project
  description followed by a placeholder comment, and a later phase generated the
  requirements into the same file. That staging existed because cc-sdd had no
  Brief: `requirements.md` was the only place the original request could live.
  Decision 0062 gives the request its own artifact, so the stub has no remaining
  purpose and Requirements are authored once, in full, from the Brief.
- Direct items get no Brief. They own no canonical artifacts.
- Discovery reads the `okf-authoring` protocol because it writes managed
  Markdown. V1 adds no discovery-specific shared rule; boundary judgment is the
  skill's own contract.

### What discovery does not do

Technology option comparison, library viability investigation, architecture
selection, and exploratory subagent research are not discovery work. They belong
to `gap-analysis` and the design phase, which own their protocols.

Discovery compares exactly one thing: Spec boundaries and scope decomposition.
Pulling technical evaluation forward produces a scope justified by an
implementation approach that Design has not yet chosen.

### Stop conditions

Several operations discovery might reasonably want are not exposed by the CLI.
Discovery stops and explains rather than improvising an equivalent:

- removing an active Spec from the milestone scope
- abandoning the milestone
- reclassifying a completed Direct item as Spec-backed work
- committing or stashing to satisfy the clean-repository guard that
  `milestone create` requires

The last one matters most. The guard exists so the milestone baseline is a real
commit that later Contract diffs can be read against; satisfying it by moving
the user's uncommitted work would defeat the guarantee and touch work the user
never offered.

### Verification

Beyond the Decision 0096 conformance checks, the discovery skill's forward tests
cover at least: a new Direct item, an existing Spec update, a new Spec, mixed
work in one candidate, adding to an already active milestone, a gate rewind
preceding a scope update, a refused creation on a dirty repository, and a
refused reclassification of a completed Direct item.

## Consequences

- Discovery decides scope and delegates every mutation, so its output is a
  confirmed candidate rather than a set of files it wrote.
- Classification rests on boundary ownership, which is reviewable, instead of on
  size, which is not.
- `update-scope` becomes usable by an agent, because the current scope can be
  read in the exact shape the replacement takes.
- `spec list` gives every skill a way to see the project's Specs, not only the
  ones in the active milestone.
- Operations the CLI does not expose surface as explicit stops, so a missing
  command cannot be worked around by an agent editing state directly.

## Implementation status

Not implemented. `spec list` and `milestone scope` do not exist, and no
discovery skill is embedded. The read models they need are already present:
Spec enumeration exists inside the Contract graph resolver, and the Roadmap
parser holds the categories and dependencies the scope document requires.
Implementation proceeds as the two read commands first, then the skill.
