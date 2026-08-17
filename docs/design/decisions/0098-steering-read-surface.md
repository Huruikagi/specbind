# 0098: Expose steering documents through the CLI

Status: Accepted

## Context

Several accepted decisions rest on "always-loaded steering context".
[Decision 0052](./0052-project-state-artifacts.md) keeps machine state out of it,
and [Decision 0054](./0054-milestone-baseline-revision.md) justifies the single
baseline scalar by contrast with it. Nothing, however, fixes what a steering
document *is* in SpecBind, how one is found, or when a skill reads one. The
`steering-principles` rule accepted by
[Decision 0093](./0093-default-shared-rule-set.md) states outright that
discovery and identification belong to a later contract.

The premise was inherited rather than decided.
[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) keeps
`specbind-steering` as the authoring side, but the consuming side is empty: no
command reads steering, so a skill that wanted guidance could only scan
`steering/` by hand. That is exactly what
[Decision 0057](./0057-type-based-artifact-discovery.md) rules out when it
requires that agent workflows never guess filenames or load every Markdown file
by default.

The gap surfaced while writing `specbind-discovery` under
[Decision 0097](./0097-discovery-routing-and-read-models.md), whose reading
discipline could not name steering at all.

This decision fixes the read surface only.

## Decision

### Two read-only commands

```text
specbind steering list
specbind steering read <selector>
```

They are the project-level counterparts of the Decision 0058 `artifact`
commands, with the same division: `list` returns a compact inventory, `read`
returns one document as raw UTF-8 Markdown with no result wrapper.

Neither command requires an active milestone. Steering outlives milestones, and
guidance that could only be read while work was in flight would be unavailable
at exactly the moment a new milestone is being scoped.

### Steering documents are an OKF collection

A steering document is an OKF concept document with `type: SpecBind Steering`
and a required stable `artifact_id`, matching the Decision 0057 collection
pattern:

| `type` | Multiplicity | Persistent identity |
| --- | --- | --- |
| `SpecBind Steering` | zero or more | collection role `<artifact_id>` |

- Discovery is recursive below `{{SPEC_DIR}}/steering/`, considers regular `.md`
  files only, and does not follow symbolic links.
- The selector is the bare `artifact_id`. Steering is already scoped by being
  project-level, so no further qualifier earns its place.
- `artifact_id` is the persistent identity. A steering document keeps it across
  renames and moves, and splitting one keeps the old ID on the continuing
  responsibility.

`steering/roadmap.md` is not steering in this sense. It declares
`type: SpecBind Roadmap`, so type matching excludes it with no special case, and
it keeps the milestone commands that own it. It shares the directory because
both are project-level, not because both are guidance.

### Read model

The commands follow Decision 0058 wherever it already answers a question, with
one stricter rule: a steering read fails when any inventory diagnostic exists.
Spec-local artifact reads may return one valid document despite an unrelated
fault, but steering consumers cannot safely act on project guidance known to be
incomplete.

**Listing.**

- Each recognized line exposes `selector`, `type`, and `path`, as Decision 0058
  requires. `artifact_id` is not shown separately, because for this profile it
  is the selector.
- Documents are ordered by `artifact_id` in Unicode code point order. One
  collection means one ordering rule.
- Zero steering documents is a valid answer:
  `OK STEERING_LISTED: Found 0 steering document(s).` An absent `steering/`
  directory is also zero, not a failure. Steering is optional and a project that
  has never run the steering skill has none, so absence is the correct state
  rather than a fault.
- An unreadable `steering/` directory returns `ERROR STEERING_LIST_FAILED`.
- Invalid Front Matter, and any other per-document fault, returns every
  unambiguously discovered document plus stable diagnostics and exits nonzero,
  per Decision 0058. Partial inventory is diagnostic information.
- A duplicate `artifact_id` is a hard discovery error under Decision 0057. Both
  documents are reported as diagnostics and neither is listed as a usable
  selector, because a selector that resolves to two documents cannot be read.

**Unknown types.** A valid OKF document of another type is not listed and not
readable, exactly as Decision 0058 fixes for spec-local artifacts. It remains
valid bundle content on disk under Decision 0057; that decision governs what the
bundle holds, this one governs what the read model returns. `SpecBind Roadmap`
is excluded on the same rule and is not reported as an anomaly, since it is
expected in this directory.

**Reading.**

- `steering read` takes exactly one selector and writes that document's original
  UTF-8 Markdown to standard output with no wrapper, outcome line, or
  normalization — the Decision 0067 raw-content exception.
- Diagnostics go to standard error, so successful standard output is solely the
  selected document.
- A missing or ambiguous selector returns `ERROR STEERING_READ_INVALID` with
  empty standard output. Ambiguity here means a duplicated `artifact_id`.
  Requested-selector resolution takes precedence over collection-wide
  diagnostics, so the caller receives this focused code when that selector
  itself cannot resolve.
- A target that is not a regular non-symlink file returns
  `ERROR STEERING_READ_TARGET_INVALID`; content that is not UTF-8 returns
  `ERROR STEERING_READ_NOT_UTF8`. These mirror the `artifact read` codes because
  they are the same faults.
- After one selector resolves uniquely, any remaining inventory diagnostic,
  including one attached to another document, returns
  `ERROR STEERING_READ_FAILED` with the partial inventory and diagnostics. This
  closes a race in which the collection becomes invalid after a successful
  `steering list` but before the caller finishes its reads.

**A result is never split across streams.** Either the command succeeded and
standard output holds the whole result with standard error empty, or it failed
and standard error holds the `ERROR` line with the partial inventory and
diagnostics as its details, with standard output empty. There is no case where
part of an answer appears on standard output while a diagnostic appears on
standard error.

Decision 0067 supplies the matching process contract: every failed command exits
nonzero and emits the same stable `ERROR` code. Skills branch on that command
result rather than attempting to recover a partial read.

### No core-versus-custom split

cc-sdd separated `steering/` from `steering-custom/`, with `product.md`,
`tech.md`, and `structure.md` privileged as core. SpecBind has one flat set.

The distinction was already vestigial upstream: the inherited steering skill
states that all steering files are treated equally, so the split survived in the
directory layout without surviving in behavior. Three fixed filenames also
contradict Decision 0057, under which identity is a declared `artifact_id` and a
path is a locator.

Projects that want the familiar three keep them as three steering documents with
those identities. Nothing privileges them.

### Steering is read on demand, not always loaded

The commands are the whole mechanism. SpecBind does not preload steering into
every skill, and no skill reads `steering/` directly.

Preloading into every skill is what the inherited workflow did. It spends
context on guidance most steps do not need and hides which workflow actually
consulted it. This does not prevent one owning workflow, such as discovery, from
reading the complete collection when its decision requires the complete project
context.

The phrase "always-loaded steering context" in Decisions 0050, 0052, and 0054 is
therefore read as *routinely available* rather than *eagerly loaded*. Those
decisions turn on keeping bulky machine state out of routinely read documents,
which this decision preserves.

### Consumption discipline

Every skill that consults steering does so through these commands. No skill
reads `steering/` directly, and none receives it preloaded.

**Discovery reads all of it.** In v1 `specbind-discovery` lists steering and
reads every document listed.

Selective reading is not implementable at this profile. The inventory carries
selector, type, and path, and `artifact_id` is a free identity — `main` is
valid — so relevance cannot be judged without the body. A skill instructed to
read selectively would either guess from a name or quietly read everything
anyway, and the second is what the rule was meant to prevent.

Reading all of it is bounded in a way preloading is not: it happens inside one
invocation of the one skill whose job is deciding boundaries, not in every skill
on every turn. V1 accepts that cost, because a boundary decided without the
project's conventions is wrong in a way that is expensive to discover later.

This rests on no assumption about how large steering is. The
`steering-principles` rule encourages small focused documents, but a project
owns that rule and may remove it, so the product contract cannot depend on it.

Making this selective requires a declared relevance field such as `summary` or
`applies_to` in the profile, which is deferred below.

Reading technology guidance does not license technical evaluation. Established
technical boundaries are boundaries: "authentication is owned by the gateway" is
an ownership fact that decides where work belongs, and it is no less so for
appearing in a document about the stack. What Decision 0097 excludes is
*choosing* — comparing options, selecting a library, picking an architecture. A
constraint the project already settled is an input to routing; a decision nobody
has made yet is not discovery's to make. The line is between existing
constraints and open choices, never between document categories.

**Other skills are not settled here.** This decision fixes the read surface and
discovery's use of it. What any other skill reads is its own skill decision to
make, for the same reason selective reading is not available generically: the
inventory carries no relevance field, so a rule stated here would either be
"read everything" for every skill — the preload this decision rejects — or a
selection nobody can implement.

One case is already safe. A skill holding an explicit selector from its own
input or authoritative workflow context reads that document directly, exactly as
Decision 0058 permits for a known artifact selector.

**Steering is not a gate input.** It is never fingerprinted, never part of gate
evidence, and never a freshness input. Editing steering invalidates no approval.

**Guidance that changed a decision lands in an artifact.** Because steering is
not fingerprinted, a conclusion that rests on it is unreproducible unless the
reasoning is written where the work lives. Otherwise discovery could route a
request into a Spec on the strength of a convention and leave no trace, and
`specbind-requirements` would inherit a boundary it cannot justify.

Where it lands follows from what the reasoning is about:

| Reasoning | Lands in |
| --- | --- |
| Why a Spec owns this responsibility | that Spec's Brief |
| Why an item is Direct, why items depend on each other, how the milestone was decomposed | the Roadmap body |
| A convention merely confirmed, changing nothing | nowhere |

Decision 0097 gives Direct items no Brief, and no single Spec's Brief can hold a
reason that is about the relationship *between* items. The Roadmap body is
already the agent-owned prose for milestone-wide intent under Decision 0046, so
it is where those two kinds of reasoning belong.

This adds one exception to discovery's rule of omitting `body` from a scope
candidate: when steering materially shaped the Direct classification, the
dependency structure, or the decomposition, discovery supplies a body carrying
that reasoning. The rule's purpose is to avoid clobbering authored prose through
a read-edit-write round trip, not to keep discovery from writing prose it owns.

The third row matters as much as the others. Recording every convention that
turned out to be consistent with the plan buries the ones that actually changed
it.

### Deferred

- A declared relevance field such as `summary` or `applies_to`. Adding one to
  the profile is what would make selective reading implementable, and it is a
  structural change to an accepted artifact profile rather than a read-surface
  detail.
- Installing steering templates. Decision 0091 narrowed the installed template
  surface deliberately, and the `template list` scope argument already
  anticipates a second scope; adding one is its own decision.
- The `specbind-steering` authoring skill, including bootstrap, synchronization,
  and drift detection against the codebase.
- Any write command. This decision adds no steering mutation surface.

## Consequences

- The premise several accepted decisions already relied on now has a mechanism,
  and the contradiction with Decision 0057 is closed.
- Steering gains stable identities, so it can be reorganized without breaking
  the skills that cite it.
- Skills consume guidance explicitly, which makes the influence on a decision
  visible in the transcript instead of implicit in a preload.
- Reasoning that rests on steering is written into the Brief or the Roadmap
  body, so a deliverable stays readable without the steering document that
  shaped it — which matters precisely because steering is not fingerprinted and
  may have changed since.
- Discovery fails closed on an incomplete steering read, so a routing decision
  is never made on a view of the project's conventions known to be partial.
- The inherited core-versus-custom split disappears, removing a distinction the
  upstream implementation had already stopped honoring.
- Routing keeps its narrow reading discipline while gaining access to the
  structural conventions that legitimately bear on boundaries.

## Implementation status

Not implemented. No `steering` command route exists, no `SpecBind Steering`
profile is registered in artifact discovery, and `specbind-discovery` names no
steering read.
