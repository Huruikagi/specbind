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

## 1. Understand the request

Restate the request in your own words and check it back with the user if
anything material is unclear. A misread request produces a wrong boundary, and a
wrong boundary is expensive to undo later.

Do not begin comparing technical options. Choosing a library, an architecture, or
an implementation approach is not discovery work; it belongs to gap analysis and
design. Scope justified by an approach nobody has chosen yet is scope built on a
guess.

## 2. Read the project shape

Always:

```sh
specbind milestone status
specbind spec list
```

Then, only when a milestone is active:

```sh
specbind milestone scope
```

`NO_CHANGE NO_ACTIVE_MILESTONE` from either milestone command is an answer, not a
failure: there is no active milestone, so this request will create one.

Read a specific Spec's requirements and contract only when you need to decide
whether that Spec owns part of this request:

```sh
specbind artifact read <spec> requirements
specbind artifact read <spec> contract
```

Read them for the candidate Specs, never for all of them. Do not read designs or
task plans at all — they describe how accepted work is built, which is exactly
the technical evaluation this stage stays out of.

If `spec list` reports a Spec as unreadable, say so and stop. Routing work into a
Spec whose machine state is broken compounds the fault.

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

## 5. Confirm before changing anything

Present the whole plan and get explicit agreement:

- each work item, its kind, and one line on why it belongs where you put it
- each new Spec identity and the responsibility it owns
- every gate that will be invalidated, and what that costs in rework
- dependencies between items

Scope is the decision the rest of the workflow is built on. Confirm it once here
rather than discovering it was wrong three phases later.

## 6. Apply, rewinds first

Perform every confirmed gate invalidation **before** creating or updating scope.

The reverse order is unsafe. Between the scope change and the rewind, the
milestone already claims the new scope while a participating Spec still carries
gates approved for the old one — and those gates look genuinely fresh, because no
artifact has changed yet. Nothing would stop an approval acting on a Spec that is
about to be rewound.

Then apply the scope. With no active milestone:

```sh
specbind milestone create --scope -
```

With one already active, compose the complete replacement from the current value
rather than writing it from scratch:

```sh
specbind milestone scope
specbind milestone update-scope --scope -
```

`update-scope` takes a **complete** replacement, so start from what
`milestone scope` emitted and add to it. Omit `body` unless the user asked for
the roadmap prose to change; omitting it preserves what is already written.

The scope document is transient input. Pipe it on standard input rather than
leaving a file behind in the repository.

## 7. Write the briefs, then report

Every Spec-backed work item gets an active brief at `<specDir>/<spec>/brief.md`,
where `specDir` is the value configured in `.specbind.json`. Start from the
template:

```sh
specbind template read spec brief
```

Fill it from the request in the requester's own terms. Keep it short — the
authoritative scope lives in requirements, and this document is not
fingerprinted. When the Spec already has a brief in this milestone, **fold the
new request into it** rather than adding a second one.

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

Finally, report in the project's language: what was created or changed, what was
invalidated and why, and which skill runs next for each item.

## Boundaries

Read the OKF authoring protocol before writing any brief:

```sh
specbind protocol read okf-authoring
```

Do not author requirements here, for a new Spec or an existing one. A newly
created Spec correctly holds only machine state until `specbind-requirements`
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
