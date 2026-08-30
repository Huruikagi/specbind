# 0164: Add local Discovery source collections with provider-neutral semantics

Status: Accepted

## Context

Discovery already accepts one request containing mixed Direct, existing-Spec,
and new-Spec work, then records one confirmed Roadmap scope. What it does not
accept as a product contract is a collection of source material. Decision 0097
fixes its reads to project state, Steering, and candidate Spec artifacts and
explicitly excludes every other read from routing.

This leaves the new-project journey narrower than the product. A greenfield
maintainer commonly arrives with a product brief, use cases, screen notes, or
other requirement material that should be split into several durable
responsibilities. Asking for one small feature avoids that input boundary, but
also avoids the Roadmap decomposition, dependency ordering, all-Spec planning,
and Contract Review that distinguish SpecBind.

The Japanese new-project guide had already suggested passing the location of
such material to Discovery. Without a Skill contract, one agent may read the
whole set, another may select promising files, and a later phase may never see
what informed its Spec. Documentation cannot promise behavior the installed
Skill does not own.

Remote providers introduce authentication, changing query results, and
provider-specific revision semantics. None of those is necessary to establish
the v1 greenfield journey. The first supported provider can remain local and
Git-backed while the semantic model stays usable by later providers.

## Decision

### One provider-neutral concept, one v1 provider

A **Source Collection** is an explicitly supplied set of request-context files
captured at the start of Discovery. Each regular file is a **Source Item**.

The shared semantics are provider-neutral and live in the immutable
`source-material` protocol. V1 implements only the **local-files** provider:

- the maintainer explicitly names one file or one directory;
- a relative locator is resolved from the project root;
- every source item is inside the project, Git-tracked at the current baseline,
  an ordinary non-symlink file, and valid UTF-8 text;
- a directory is traversed recursively, with items ordered by portable
  project-relative path;
- the Skill neither infers an unnamed source directory nor scans the repository
  for likely requirements.

A locator outside the project, an untracked or ignored item, a symlink, a
non-regular entry, unreadable content, or non-UTF-8 content makes the collection
incomplete. Discovery reports every known unsupported entry and stops before
classification or mutation. The maintainer may narrow the collection or place a
durable supported representation in the project; the Skill never copies, moves,
converts, or rewrites source material on its own.

Markdown and other UTF-8 text need no format registry. The provider guarantees
bytes and complete enumeration, not semantic understanding of every extension.
A text document whose syntax the agent cannot interpret is reported as
unsupported rather than silently treated as read.

### Capture is one-shot request context

Source material is evidence of the request, not a second specification
authority. Discovery captures the collection at the active milestone's Git
baseline and records project-relative locators. It adds no per-item digest,
remote revision abstraction, or persistent source-set artifact.

Editing a source item later does not mechanically stale a Gate. A changed source
re-enters through an explicit Discovery request, which compares the requested
meaning with current scope and uses the ordinary confirmed invalidation and
scope-update flow. No phase re-queries an implicit live collection.

The Roadmap records collection-wide provenance and routing:

- provider and collection locator;
- every source item;
- its disposition as an existing Spec update, new Spec, Direct, excluded,
  duplicate, or unresolved;
- the relevant work items and the reason for that mapping.

Each Spec-backed Brief records only the source items relevant to that Spec and
why they matter. A source item shared by several Specs appears in every relevant
Brief; the Roadmap explains the cross-Spec decomposition. Direct items still
receive no Brief.

### Complete coverage precedes scope confirmation

An ordinary conversational Discovery retains Decision 0097's four-field
confirmation payload. A Source Collection run adds a fifth field:

```text
Source coverage: <every item, disposition, relevant work items, and unresolved questions>
```

No item disappears silently. Excluded and duplicate items remain visible with a
reason. An unresolved item prevents an approvable proposal because the remaining
scope may depend on its disposition.

The Source Collection is read before classification and before any Gate
invalidation or milestone mutation. After confirmation, the same
rewind-before-scope order remains authoritative. A new Roadmap materializes the
selected template with source provenance; an existing Roadmap is read with
`milestone scope --include-body` and updated without discarding current prose.

### Later phases promote meaning, not links

`specbind-plan-requirements` and `specbind-plan-design` already read the Brief.
When it declares source items, each Skill reads `source-material` once and reads
every declared local item. A missing, inaccessible, or no-longer-supported item
stops authoring rather than producing an artifact from a partial request.

Requirements restates every source-derived behavioral obligation it accepts in
the complete current behavioral contract. Design restates every source-derived
technical conclusion it needs in the complete current Design or Contract. A
link, locator, or quoted source fragment is not a substitute for a self-contained
canonical artifact. If source material contradicts approved Requirements,
Design reports a Requirements rewind rather than choosing the source silently.

Tasks and implementation read the canonical artifacts, not the source
collection. Contract Review remains grounded in Roadmap scope, Contracts, and
canonical Requirements or Design deep inputs. Raw source items are not accepted
as `deepInputs` and are not fingerprinted into review evidence.

### Carrier allocation

- `source-material` owns complete-capture, provenance, request-context, and
  promotion semantics shared by Discovery, Requirements, and Design.
- `specbind-discovery` owns provider selection, acquisition ordering,
  confirmation, Roadmap/Brief recording, and failure stops.
- `specbind-discovery/references/local-files.md` owns the conditional local
  provider procedure.
- Requirements and Design Skills own their phase-specific reads and promotion.
- Roadmap and Brief templates may add source headings as default affordances,
  but customized templates cannot remove the Skill and protocol obligations.
- Project Rules may specialize citation or terminology but cannot weaken
  complete coverage or promotion.
- The CLI gains only the embedded protocol and packaged Skill resource. It gains
  no filesystem inventory command, provider registry, network client, OAuth,
  credentials, or source lifecycle state.

### Deferred providers and synchronization

GitHub Milestones and Issues are the first intended remote provider and remain a
v1.1 item. Google Drive, ticket systems, and other providers may reuse the Source
Collection semantics, but each needs an explicit acquisition contract only when
a demonstrated use case exists.

Live synchronization, automatic Gate invalidation from upstream changes,
two-way status updates, source-system comments or closure, a universal provider
SDK, and a structured fingerprinted source-set artifact are outside v1.
Reading a source never grants authority to mutate it.

## Consequences

- The greenfield journey can begin from a cohesive product slice and exercise
  multi-Spec decomposition without adding network or authentication risk to v1.
- Complete inventory and visible dispositions make a directory input safer than
  ad hoc agent file selection.
- Roadmap and Brief preserve provenance while Requirements, Design, and Contract
  remain the only durable specification authority.
- Git-backed project-local inputs reuse the milestone baseline instead of
  inventing a second revision system.
- Remote providers can be added without redefining promotion or authority, but
  the product does not ship a speculative provider framework.

## Verification

Mechanical tests cover the protocol registry, packaged Discovery reference,
required Skill reads and promotion language, and valid customized templates
without source headings. Behavioral forward tests cover a clean greenfield
directory split into multiple Specs and Requirements authoring from one Brief's
declared subset. The fixture state, not the driver report, proves Roadmap source
coverage, per-Spec Brief references, unchanged source files, and canonical
promotion.

## Implementation status

Implemented by the `source-material` protocol, the local-files Discovery
reference, the Discovery/Requirements/Design Skill contracts, localized default
Roadmap and Brief affordances, the greenfield guide, and focused mechanical and
behavioral tests.
