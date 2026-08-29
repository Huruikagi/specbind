---
name: specbind-discovery
description: Turn a change request into confirmed milestone scope. Decides whether work is Direct, an update to an existing Spec, or a new Spec, then delegates every state change to the CLI and writes the briefs.
argument-hint: "<what you want to change>"
---

# Decide what the work is

Take a change request and answer one question: **which durable boundaries does
this work belong to?** Everything else follows from that answer.

You decide and confirm. The CLI performs every lifecycle and state change. The
one thing you author is the brief for each Spec-backed item.

## Non-negotiable first boundary

The request to run this skill is **not** confirmation of the scope you have not
presented yet. This remains true when the request says "ship", "go ahead", or
"take it through release", supplies a precise change and version, or otherwise
authorizes the desired outcome broadly.

Your first response before any product or lifecycle mutation presents the
four-field scope proposal from step 5 and asks the user to confirm it. Only a
later user reply that refers to that visible proposal authorizes Discovery to
apply it. Do not infer this separate confirmation from the invocation.

## 1. Understand the request

Restate the request in your own words and check it back with the user if
anything material is unclear. A misread request produces a wrong boundary, and a
wrong boundary is expensive to undo later.

Do not begin comparing technical options. Choosing a library, an architecture, or
an implementation approach is not discovery work; it belongs to gap analysis and
design. Scope justified by an approach nobody has chosen yet is scope built on a
guess.

### Does this need the workflow at all?

SpecBind is not a gate on every change. Say so and stop when a request does not
need it — scoping work that needs no Spec is not thoroughness, it is ceremony
that teaches people to route around the product.

A request **enters** when any of these holds, regardless of how small it is:

- it changes a Spec's requirements, design, or contract
- it changes behavior an existing Spec owns, even if the artifacts have not
  caught up
- it modifies a path some Spec's contract declares under File Ownership
- it adds a durable responsibility the project will own, rather than adjusting
  something that already exists
- the user framed it as part of the active milestone, or as work the release
  should record

The first three rules and the last are checks, not judgments. The
new-responsibility rule is the only one that asks you to think, and it is about
what the project will own afterwards, not about how large the work looks: a small
first version of a capability nobody owns yet still creates the boundary that
will hold it.

The File Ownership rule is the one to check rather than judge. It is the
project's own declaration of which boundaries matter, so a one-line change to an
owned path enters exactly as a large one does. Run `specbind spec list` and read
the contracts of the Specs that could plausibly own what the request touches.

The last rule is about the **request**, not one item in it. When the user presents
several pieces as one delivery, every piece enters, including the ones that would
not on their own. Do not split a stated delivery across the boundary: the user
said what belonged in the release, and part of it silently would not be there.

Otherwise say in one sentence that the work needs no Spec, and hand it back to be
done as ordinary work. Do not create a milestone, a Roadmap item, or a brief for
it. That one sentence matters: it lets the user answer "actually, track that,"
which they cannot do if you decided silently.

**When it is genuinely unclear, it enters.** Conscripting a small change into a
milestone wastes ceremony and is obvious immediately. Letting real Spec work out
means behavior changed with no requirement, no coverage, and no record, and that
surfaces only much later, when something depends on the specification being
true. Asking the user beats both guesses.

If a milestone is active and you hand work back, say that doing it will leave
the worktree dirty for whatever runs next. Do not commit on the user's behalf to
tidy up.

## 2. Read the project shape

Always:

```sh
specbind milestone status
specbind spec list
specbind steering list
```

Then read **every** steering document the listing named:

```sh
specbind steering read <selector> --for consume
```

Read all of them, not a promising-looking subset. The listing carries only a
selector, a type, and a path, so there is nothing in it from which relevance
could honestly be judged — a document called `main` may be the one that decides
this boundary. This is the one set discovery reads whole, and it happens here,
in the skill whose job is deciding boundaries, rather than being loaded into
every skill.

Then, only when a milestone is active:

```sh
specbind milestone scope
```

`NO_CHANGE NO_ACTIVE_MILESTONE` from either milestone command is an answer, not a
failure: there is no active milestone, so this request will create one.

Read a specific Spec's requirements and contract only when you need to decide
whether that Spec owns part of this request:

```sh
specbind artifact read <spec> requirements --for consume
specbind artifact read <spec> contract --for consume
```

Read them for the candidate Specs, never for all of them. Do not read designs or
task plans at all — they describe how accepted work is built, which is exactly
the technical evaluation this stage stays out of.

### When a read fails

Stop before classifying and before changing anything if:

- `spec list` reports a Spec as unreadable. Routing work into a Spec whose
  machine state is broken compounds the fault.
- `steering list` or `steering read` printed an `ERROR` line. The document you
  did not get may be the one that decided the boundary, and routing on a
  knowingly partial view of the project's conventions is a guess presented as a
  decision.

`OK STEERING_LISTED: Found 0 steering document(s).` is not this case. It is a
complete answer — the project has no steering — and you continue normally.

## 3. Classify each part of the request

Classification applies to **new work items only**.

If the request refines something already in the active scope, it is not a new
item. Route it back to the phase that owns the affected artifact and say which
one. In particular, changing only the task plan of a Spec already in scope is a
tasks rewind on the existing item — never a new Direct item, even though it
changes no requirements, design, or contract.

For genuinely new work, each part is exactly one of:

| Kind | It is this when |
| --- | --- |
| **Direct** | It belongs to no existing Spec and needs no requirements, design, or contract change. |
| **Existing Spec update** | It changes behavior or a boundary an existing Spec already owns. |
| **New Spec** | It needs a new durable responsibility and contract boundary. |

Decide by ownership, not by size. A large change inside one boundary is still one
Spec; a small change that creates a new seam is a new Spec. Task counts and
effort estimates say nothing about where a boundary belongs.

Steering informs this. A constraint the project already settled is an input:
"authentication is owned by the gateway" decides where work goes, and it is no
less a boundary for appearing in a document about the stack. What you must not
do is *choose* — compare options, pick a library, select an architecture. The
line is between constraints that exist and choices nobody has made, never
between kinds of document.

A request spanning several kinds is normal and is not a fourth case. It becomes
one scope candidate holding several work items.

Name each new Spec with a short lowercase kebab-case identity that describes the
responsibility it owns, not the change being made.

## 4. Name the gates the work invalidates

For each Spec the work touches that already holds approved gates, name the
earliest gate affected:

- requirements change → `specbind spec requirements invalidate <spec>`
- requirements unchanged, design or contract changes → `specbind spec design invalidate <spec>`
- only the task plan changes → `specbind spec tasks invalidate <spec>`
- no canonical artifact changes → nothing to invalidate

Each rewind clears the downstream evidence too, so name only the earliest one.

Use the gate states from `specbind spec status <spec>`. A gate reported as
`not_reached` is not approved and must not be named or run as an invalidation.
For an idle established Spec, the confirmed milestone mutation starts the new
active change at Requirements; there is no prior gate to rewind first. An
existing Requirements artifact does not by itself mean the Requirements gate
is approved.

## 5. Confirm before changing anything

Present the whole plan and get explicit agreement:

- each work item, its kind, and one line on why it belongs where you put it
- each new Spec identity and the responsibility it owns
- every gate that will be invalidated, and what that costs in rework
- dependencies between items

Stopping for confirmation means presenting that complete confirmation payload
in the same response. Render it with these four explicit fields, writing `None`
when a field has no entries:

```text
Work items: <identity, kind, and reason for each>
New Specs: <identity and owned responsibility for each, or None>
Gate invalidations: <earliest gate and rework cost for each, or None>
Dependencies: <edges only between the work-item identities above, or None>
```

A release version, publication, phase, or gate is not a work item and never
appears as a dependency endpoint. One work item therefore always reports
`Dependencies: None`.

Then ask for confirmation of that proposal. Never return only a statement that
confirmation is required, a list of reads or commands, or a no-change summary.
Until all four fields are visible to the user, there is nothing they can safely
approve. Do not run an invalidation, `milestone create`, `milestone
update-scope`, or any artifact write before sending this payload and receiving
the user's later reply.

Scope is the decision the rest of the workflow is built on. Confirm it once here
rather than discovering it was wrong three phases later.

## 6. Apply, rewinds first

Perform every confirmed gate invalidation **before** creating or updating scope.

Run only invalidations that the pre-confirmation status showed as approved and
the user confirmed. `SPEC_*_STATE_INVALID` is not a harmless way to discover
that no rewind was needed: stop on it rather than treating the rejected command
as a completed invalidation.

The reverse order is unsafe. Between the scope change and the rewind, the
milestone already claims the new scope while a participating Spec still carries
gates approved for the old one — and those gates look genuinely fresh, because no
artifact has changed yet. Nothing would stop an approval acting on a Spec that is
about to be rewound.

Then apply the scope. With no active milestone:

```sh
specbind template read milestone roadmap
specbind protocol read okf-authoring
specbind schema read scope/v1
specbind milestone create --scope -
```

Author the candidate against the schema you just read. Materialize the Roadmap
template's Markdown body into the candidate's `body`: resolve every
`create bind=<name>` once, replace every reference to that name with the same
value, apply its remaining `create` guidance without copying that instruction,
preserve `maintain` and `consume` instructions, and fill the scaffold with the
confirmed milestone-wide request, boundaries, decomposition reasoning, and
dependency rationale. Front Matter from the template never enters `body`; the
milestone command owns the live Front Matter. `--help` describes the transport,
not the strict document shape; do not probe a mutating command with guessed JSON
to discover its fields.

With one already active, compose the complete replacement from the current value
rather than writing it from scratch:

```sh
specbind milestone scope
specbind milestone update-scope --scope -
```

`update-scope` takes a **complete** replacement, so start from what
`milestone scope` emitted and add to it.

Do not reapply the Roadmap template to an active milestone. The existing body is
current authority; a later project template edit affects the next Roadmap, not
one already in progress.

The default read carries no `body`, and an omitted body preserves the roadmap
prose already written. Use the complete form only when you intend to change that
prose — including when steering shaped a decision you must record there, per
step 7:

```sh
specbind milestone scope --include-body
```

Edit the `body` you were given and submit it whole. Never hand-write a partial
body into an otherwise default candidate: a complete replacement containing a
fragment silently discards the rest of the roadmap prose.

The scope document is transient input. Pipe it on standard input rather than
leaving a file behind in the repository.

## 7. Write the briefs, then report

Every Spec-backed work item gets an active brief at `<specDir>/<spec>/brief.md`,
where `specDir` is the value configured in `.specbind.json`. Start from the
template, and read the authoring protocol before you write:

```sh
specbind template read spec brief
specbind protocol read okf-authoring
specbind milestone status
```

The final status read is the protocol's check immediately before the first Brief
write. If any participating Spec is `release_ready`, stop at the protocol's
confirmation boundary before authoring. An earlier status read does not replace
this check because applying the scope changed the milestone.

Fill it from the request in the requester's own terms. Keep it short — the
authoritative scope lives in requirements, and this document is not
fingerprinted. When the Spec already has a brief in this milestone, **fold the
new request into it** rather than adding a second one.

The template title and instruction comments are not a valid Brief by
themselves. Include substantive request content before the first write.

On first materialization, follow every `create bind=<name>` instruction once,
replace every reference to that name with the same resolved value, and omit the
`create` instruction. Copy `maintain` and `consume` instructions unchanged. When
folding into an existing brief, read it with
`artifact read <spec> brief --for maintain` and preserve those durable comments.

### Record what steering decided

Steering is not fingerprinted, so a conclusion resting on it is unreproducible
unless you write the reasoning where the work lives. Otherwise the next skill
inherits a boundary it cannot justify, and nobody can tell later whether the
guidance still says what it said.

| Reasoning | Write it in |
| --- | --- |
| Why a Spec owns this responsibility | that Spec's brief |
| Why an item is Direct, why items depend on each other, how the milestone was decomposed | the roadmap body |
| A convention you merely confirmed, changing nothing | nowhere |

Direct items get no brief, and no single Spec's brief can hold a reason that is
about the relationship *between* items — that is why those land in the roadmap
body instead.

The third row matters as much as the others. Recording every convention that
turned out to be consistent with the plan buries the ones that actually changed
it.

Write briefs only after the CLI command succeeded. Before it succeeds there is no
committed scope for them to describe, and `milestone create` refuses to run with
untracked files present, so a brief written first would break the command it was
meant to accompany.

If writing a brief fails, do not undo the scope change to recover. The mutation
is the authoritative record; reversing it would discard the only part that
worked. Report which briefs are outstanding and finish them.

Re-running this skill after such a failure is safe. An unchanged scope returns
`NO_CHANGE MILESTONE_SCOPE_UNCHANGED` and nothing is mutated twice, so you can go
straight to completing the missing briefs.

Do not report success until every brief you owe has been written **and read
back**. The CLI tracks machine state, not brief content, so a scope whose briefs
were never authored looks healthy to every command while being exactly the state
the next skill cannot start from.

## 8. Checkpoint

Only now is this work eligible to commit: the milestone mutation succeeded and
every brief you owed is written. A partially written discovery result is never
committed, however often the project wants checkpoints.

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT` means there is no adapter-directed commit. Stop
there — that is an answer, not a missing file to work around.

An adapter carrying the exact `<!-- specbind:adapter-scaffold -->` marker is an
inactive scaffold, not project policy. Treat it as no guidance, say so in one
line, and commit nothing. The marker classifies the whole document: ignore every
other body line even when it looks actionable. Do not stop to ask about a file
nobody has filled in.

When the adapter has guidance, follow it. The request to perform this mutating
phase authorizes the adapter's narrow local checkpoint as its ordinary final
step. It does not authorize anything broader:

- An explicit user or root instruction that forbids commits wins, and tool
  permissions still apply.
- Commit guidance is not push guidance. Push only where the adapter says to, and
  never force-push, rewrite history, or bypass a protected branch.
- Stage only the paths this run produced. Unrelated work already in the worktree
  is left exactly as it is.
- Stop before the Git operation if the guidance is ambiguous, unsafe, or
  conflicts with something else you were told.

A failed checkpoint changes nothing that already succeeded. The milestone and
the briefs remain valid; report them as uncommitted and continue.

## 9. Report

In the project's language: what was created or changed, what was invalidated and
why, whether the work was committed, and which skill runs next for each item.

## Boundaries

Do not author requirements here, for a new Spec or an existing one. A newly
created Spec correctly holds only machine state until `specbind-plan-requirements`
runs; an empty scaffold placed now would put an artifact before the skill that
owns it. Requirements are written once, in full, from the brief.

Do not write `roadmap.md`, any `spec.yaml`, or any gate evidence directly. Those
are CLI-owned, and hand-editing them produces state no command validated.

## Stop and explain

Some operations the CLI deliberately does not expose. When the plan needs one,
stop and tell the user what is needed, rather than improvising an equivalent:

- removing an active Spec from the milestone scope
- abandoning the milestone
- reclassifying a completed Direct item as Spec-backed work
- committing or stashing to satisfy the clean-repository requirement of
  `milestone create`

The last one matters most. That requirement exists so the milestone baseline is a
real commit later contract diffs can be read against. Moving the user's
uncommitted work to satisfy it would defeat the guarantee and touch work they
never offered you. Ask them to commit or stash it themselves.
