---
name: specbind-steering
description: Maintain durable project guidance — bootstrap it from the codebase, bring it back in line after the code moved, or add one document on a new subject.
argument-hint: "[what changed, or the subject to document]"
---

# Maintain durable project guidance

Steering carries what outlives any single change: how this project is built,
what it values, and the constraints every change inherits. You author it. The
CLI owns discovery and identity.

Nothing else in SpecBind depends on this running, and nothing invalidates when
it does — steering is never gate evidence and editing it approves nothing. That
cuts the other way too: **`specbind-discovery` reads the whole collection and
routes work on it**, so a document that has quietly gone out of date misroutes
real work. Guidance you are not confident is still true is worth removing.

## 1. Confirm what you are doing

Three things can be asked for, and they are not interchangeable:

| Intent | What it means |
| --- | --- |
| **Bootstrap** | The project has no steering, and wants a first set |
| **Synchronize** | Steering exists and the codebase has moved past it |
| **Add** | One new subject deserves its own document |

Ask when the request does not say. The current inventory is input to that
conversation, not the answer to it: **an empty `steering/` is a valid steady
state**, not a prompt to bootstrap. A project that decided it does not want
steering should not get it because a skill assumed.

## 2. Read what exists

```sh
specbind steering list
```

Then read every document it lists:

```sh
specbind steering read <artifact_id>
```

Never read `steering/` directly and never glob it. The selector is the
`artifact_id`, and the listing is what tells you which ones exist.

**When the listing reports a diagnostic**, `steering read` will refuse every
document, including the healthy ones — a consumer must not act on guidance known
to be incomplete. You are the exception, and only for repair:

- The diagnostic names the faulty path. Read **that file** directly, fix what the
  diagnostic reports, and re-run `specbind steering list`.
- Repair first, then continue. Do not work around a broken collection by
  authoring alongside it.
- Every other read still goes through the commands above.

Read the project's `settings/rules/steering-principles.md` once. It is user-owned
policy about what belongs inside a steering document — granularity, examples,
what to leave out. If it is absent, no customization applies and you proceed on
the contract here.

## 3. Understand the codebase before writing about it

Dispatch fresh readers rather than reading everything yourself. Each one starts
with no context, so give it a brief that stands alone: what to look at, what
question to answer, and that you want the *pattern*, not an inventory.

For bootstrap, three independent readers cover the ground:

- **Product** — README, package and project metadata, user-facing documentation.
  What is this for, who uses it, what has it deliberately refused to do?
- **Technology** — build configuration, dependencies, test setup. What decisions
  is every change inheriting, and what reasons are recoverable?
- **Structure** — the tree, naming, import and dependency direction. What rule
  decides where a new file goes?

For synchronize, ask each reader to compare the codebase against the steering
text you already read, and to report specifics: what steering claims that the
code no longer does, and what durable pattern the code has established that
steering does not mention.

Extract patterns, not catalogs. **If new code that follows the existing patterns
would require a steering update, the document is written at the wrong level.**

## 4. Author

Materialize from the scaffolds rather than inventing structure:

```sh
specbind template list steering
specbind template read steering <selector>
```

`product`, `tech`, and `structure` are the bootstrap defaults and carry their own
identity. `document` is the scaffold for any other subject and deliberately
declares none.

**Remove every `specbind:instruction` comment** from what you write. Those are
addressed to you and never appear in a materialized artifact.

### Bootstrap

Propose `product`, `tech`, and `structure`, and say what each would contain
before writing. Nothing privileges these three — the user may rename them, merge
them, split them, or decline any of them. Write what the project actually has;
a section you would have to invent content for is a section to delete.

### Synchronize

**Revise in place. Do not accumulate.** A steering document states the project as
it is now, and guidance that keeps its own history makes readers guess which
version is in force. Git holds what it used to say.

That is not a licence to rewrite:

- Revise what the codebase demonstrably contradicts.
- Leave alone what is merely not how you would have written it. Restructuring a
  sound document is churn.
- When you cannot tell whether something is stale or just unfamiliar, **propose
  it and let the user decide**. Unclear is not the same as false.

Report drift you are not fixing: patterns the code has established that nobody
has decided to make policy are the user's call, not yours.

### Add

Read `template read steering document`, choose the identity, and write it at
`steering/<artifact_id>.md`.

The identity is yours to choose here — it is the one place SpecBind asks an agent
to pick an `artifact_id`. Get it right:

- lowercase kebab-case, describing the subject
- **not** already listed by `specbind steering list`. A duplicate identity is a
  hard discovery error and drops *both* documents from the collection
- stable across later renames and moves, because it is the identity, not the file
  name

## 5. Verify what you wrote

```sh
specbind steering list
```

Every document you touched must appear, with the selector you intended. **A
document that does not appear was authored wrong** — bad Front Matter, wrong
type, or a colliding identity. Fix it and list again. Do not report success on a
document the CLI cannot see.

## 6. Report

In the project's language: what you created or revised, what drift you found and
did not act on, and anything you deliberately left out. Keep it short — the
documents are the deliverable.

## Boundaries

- **Never write secrets.** No credentials, keys, tokens, connection strings, or
  anything that would be a leak if the repository were public.
- **Do not document SpecBind's own `settings/` tree or agent directories** such
  as `.claude/` and `.agents/`. That is project metadata, not project knowledge,
  and it ages against a tree the project does not maintain.
- **Do not write transient content.** Current scope, in-flight migrations, and
  the status of work under way belong to the milestone that owns them. Do not
  read Spec or milestone state to write steering — if you need it, it does not
  belong in the document.
- Steering is not a gate. This skill approves nothing, records no evidence, and
  is never required before other work proceeds.
- Reasoning that changed a routing or scoping decision belongs in that Spec's
  brief or the Roadmap body, written by the skill that made the decision. Do not
  relocate it here.
