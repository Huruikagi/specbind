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

#### `spec list`

It reports every persistent Spec in the project, whether or not it participates
in the active milestone: canonical identity, declared lifecycle state, and enough
artifact availability and health to tell a usable Spec from one needing repair.
It requires no active milestone.

- Specs are ordered by canonical identity in Unicode code point order, so the
  listing is stable across platforms and independent of directory enumeration.
- Zero Specs is a valid answer, not an absence of one:
  `OK SPEC_LISTED: Found 0 spec(s).` The `NO_CHANGE` result is reserved for a
  missing precondition, and an empty project has none missing.
- A Spec whose machine state cannot be read is **listed**, with its health naming
  the fault. A single unreadable Spec must not fail the listing, because this is
  the command an agent uses to discover that the Spec needs repair.
- Only a failure to resolve the project or to enumerate the Spec directory at all
  returns `ERROR SPEC_LIST_FAILED`.

#### `milestone scope`

It writes the active milestone's current scope to stdout as a normalized
candidate document, in exactly the shape `update-scope --scope` accepts.

- With no active milestone it returns `NO_CHANGE NO_ACTIVE_MILESTONE`, the same
  result `milestone status` already uses, and writes no document.
- An unparseable or invalid active Roadmap returns
  `ERROR MILESTONE_SCOPE_FAILED` with the parser's stable diagnostics, and writes
  no document. Emitting a partial scope would invite a replacement composed from
  a scope the parser never accepted.
- The document is the Decision 0089 version-1 camelCase form: `schemaVersion`
  first, then `workItems`, with categories present only when non-empty and item
  order preserved from the Roadmap. It is serialized with two-space indentation,
  no trailing whitespace, `\n` line endings, and exactly one trailing newline,
  independent of the platform.
- Per-item `status` is omitted. Decision 0089 forbids it in a candidate and
  preserves completed Direct status by identity across an update, so echoing it
  would produce a document the command rejects while adding nothing.

The two commands compose into a checkable invariant: feeding `milestone scope`
output straight back into `update-scope --scope -` returns
`NO_CHANGE MILESTONE_SCOPE_UNCHANGED`.

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

Classification applies to **new Roadmap work items only**. A request that refines
work already in the active scope is not reclassified: it is a refinement of the
existing item, routed back to the phase that owns the affected artifact.

This distinction is load-bearing. A request that changes only a Spec's task plan
satisfies the Direct criterion literally — no Requirements, Design, or Contract
changes — yet it is not Direct work, because the Spec already owns it. Treating
it as a new Direct item would create a second Roadmap entry for work an existing
item already covers, and would leave the stale task plan approved. It is instead
a Tasks rewind on the item that already exists.

Within that limit, discovery classifies each part of a request as exactly one of:

| Kind | Criterion |
| --- | --- |
| Direct | It belongs to no existing Spec and requires no Requirements, Design, or Contract change. |
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

When a request touches a Spec that already has approved gates — whether it is an
existing Spec entering the scope or an item already in it — discovery states
which gate the change invalidates, choosing the earliest one affected:

- Requirements change: `spec requirements invalidate`
- Requirements unchanged but Design or Contract changes: `spec design invalidate`
- Only the task plan changes: `spec tasks invalidate`
- No canonical artifact changes: nothing is invalidated. For an existing scope
  item this means the request needs no rewind; for a request that belongs to no
  Spec it means the work is Direct.

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
- Briefs are authored **after** the creating or updating command succeeds. Before
  it succeeds there is no committed scope for them to describe, and Decision 0089
  requires an untracked-free repository at `milestone create`, so a Brief written
  first would fail the very command it was meant to accompany.
- A Brief failure does not roll back machine state. Discovery neither rewinds the
  scope nor invalidates a gate to undo it: the mutation is the authoritative
  record, and reversing it to recover from a text-authoring fault would discard
  the only thing that succeeded. Discovery reports which Briefs are outstanding.
- Re-running discovery after such a failure is safe and repeats no mutation. When
  the submitted scope already matches, `update-scope` returns
  `NO_CHANGE MILESTONE_SCOPE_UNCHANGED` by construction, so discovery proceeds
  directly to completing the missing Briefs.
- Discovery does not report success until every Brief it owes has been written
  and read back. A scope whose Briefs were never authored is precisely the state
  the next skill cannot proceed from, and it is invisible to the CLI, which
  tracks machine state rather than Brief content.
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

### What discovery reads

Routing needs the project's shape, not its contents. The reads are fixed so the
skill stays small and cannot drift back into the inherited whole-repository scan:

| Read | When |
| --- | --- |
| `specbind milestone status` | always |
| `specbind spec list` | always |
| `specbind milestone scope` | only when a milestone is active |
| A Spec's Requirements and Contract | only for a Spec the request may touch |

The first two answer whether a milestone is active and which Specs exist, which
is everything the classification needs to begin. Requirements and Contract are
read to decide whether a specific Spec owns a specific request, so they are read
per candidate Spec and never swept. No other artifact is read for routing: Design
and task plans describe how accepted work is built, and consulting them would
reintroduce the technical evaluation this decision excludes.

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

The read commands are covered by CLI tests: the round-trip invariant above,
`spec list` on an empty project, on a Spec whose machine state is unreadable, and
for ordering; `milestone scope` with no active milestone, with an invalid
Roadmap, and for byte-exact serialization including the trailing newline.

Beyond the Decision 0096 conformance checks, the discovery skill's forward tests
cover at least: a new Direct item, an existing Spec update, a new Spec, mixed
work in one candidate, adding to an already active milestone, a task-plan-only
change to a Spec already in scope routed as a Tasks rewind rather than a new
Direct item, a gate rewind preceding a scope update, a refused creation on a
dirty repository, and a refused reclassification of a completed Direct item.

## Consequences

- Discovery decides scope and delegates every lifecycle and state mutation, so
  no Roadmap, `spec.yaml`, or gate evidence is written by the skill. It still
  authors the Brief, which is a managed artifact and the one thing it owns.
- Classification rests on boundary ownership, which is reviewable, instead of on
  size, which is not.
- `update-scope` becomes usable by an agent, because the current scope can be
  read in the exact shape the replacement takes.
- `spec list` gives every skill a way to see the project's Specs, not only the
  ones in the active milestone.
- Operations the CLI does not expose surface as explicit stops, so a missing
  command cannot be worked around by an agent editing state directly.

## Implementation status

Partially implemented. Both read commands exist; no discovery skill is embedded
yet.

`tools/specbind/src/spec_list.rs` lists Specs from a shared enumeration lifted
out of the Contract graph resolver into `artifacts::discover_spec_ids`, which
reports a rejected entry as a structured fault so each caller names it in its own
diagnostic vocabulary. Identities arrive already ordered, an unreadable Spec is
listed with its fault named, and only an unreadable `specs/` directory fails.

`tools/specbind/src/milestone_scope.rs` renders the active Roadmap as a
version-1 candidate. The document is serialized by hand rather than through
`serde_json`, which orders object keys alphabetically without `preserve_order`
and would emit `directChanges` before `newSpecs`, inverting the declared order.
Absence returns `NO_CHANGE NO_ACTIVE_MILESTONE` and an invalid Roadmap returns
`ERROR MILESTONE_SCOPE_FAILED` with no partial document.

The accepted checks are covered, including the byte-exact serialization, the
round trip through `update-scope` returning `NO_CHANGE`, and the omission of
completed Direct status. The skill and its forward tests remain outstanding.
