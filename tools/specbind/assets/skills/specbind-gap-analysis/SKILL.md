---
name: specbind-gap-analysis
description: Compare what a change needs against what the repository already provides, and record the findings worth keeping. Informs the decision; does not make it.
argument-hint: "<spec>"
---

# Analyze the distance between the need and what exists

Establish what the repository already provides, name precisely what is missing,
and hand the next phase material it can decide from. **You inform. You do not
decide.**

Read the protocol before you start. It owns how evidence is gathered, how gaps
are named, and how uncertainty is handled:

```sh
specbind protocol read gap-analysis
```

## 1. Read

```sh
specbind spec status <spec>
specbind artifact list <spec>
specbind steering list
```

Then:

- the Spec's brief — always
- **every** steering document the listing named
- the Requirements, when the Spec has them
- the existing research, when one exists
- the contract, and the contracts across a seam this work touches, when
  boundaries are in scope

An `ERROR` line from `steering list` or `steering read` stops you. An analysis
built against a knowingly partial view of the project's constraints is worse than
none, because it is persuasive.

Read the project's own preferences through its rule surface:

```text
specbind rule read design-principles --for consume
specbind rule read contract-principles --for consume
```

The second read is required when boundaries are in scope. `NO_CHANGE
RULE_ABSENT` means no customization; any `ERROR` line stops the analysis.

**Requirements is an input, not a precondition.** This skill runs before
Requirements exist as readily as after. Discovery deliberately refuses technical
evaluation, so running here — right after routing, before the behavior contract
is written — is the point where the answer is cheapest to act on. When no
Requirements exist yet, read the milestone's complete current scope and work
from it together with the brief:

```sh
specbind milestone scope
```

## 2. Decide whether there is anything to compare

If the affected area has no meaningful existing implementation, say so in a
sentence and stop. A greenfield comparison produces an empty document and the
impression that something was checked.

## 3. Investigate

Dispatch fresh readers rather than reading everything yourself. Each starts with
no context, so give it a brief that stands alone and a question it can answer.
Use the registered `specbind-researcher` role when available, with ordinary
fresh readers as the fallback.
Fallback is only for an absent role. A configured role whose model cannot start
is a configuration or environment failure, not permission to change models.
The independent lines are:

- **What exists** in the affected area — modules, layout, reusable components
- **What constrains it** — layering, dependency direction, where tests live, the
  patterns already in force
- **What it must meet** — data models, external clients, authentication, and the
  other integration surfaces
- **What an external dependency actually offers**, when one is in question

You may investigate outside the repository. Record the sources, and never present
an external claim as an observation about this codebase — those are different
kinds of statement and the reader has to be able to tell them apart.

Read the code. An analysis built on a plausible but wrong picture of the system
is the expensive failure here.

## 4. Route what you found

Anything that changes what is **being asked for** goes back to the user before it
goes anywhere else.

| What you found | Where it goes |
| --- | --- |
| The request cannot be met, or only at a cost the requester would not accept | Back to the user. On their agreement, the brief records the revised request |
| It exists, but restricts how the work can be done | Design input. Requirements is not touched |

That second row is the one to hold. "The current code makes this awkward" is a
design constraint, and letting it reach Requirements turns accidental structure
into an obligation the project has promised.

**Only revise the brief once the user has accepted the change.** The brief holds
the requester's own words. Rewriting them on the strength of a technical finding
is exactly what this routing exists to prevent.

Before the first managed Markdown write in this run — a user-approved Brief
revision here or a Research artifact below — read the authoring protocol once:

```sh
specbind protocol read okf-authoring
```

## 5. Decide whether to write the research artifact

Research is optional and its absence is normal. Write it when the finding
outlives the analysis:

- substantial investigation that a later session would otherwise repeat
- conclusions the design phase will need and cannot reconstruct cheaply
- the user asked for it

Do not write it when the Design will absorb the conclusions in full. Routine
analysis needs no separate document.

**Say which you chose, and why.** A silent omission is indistinguishable from
forgetting.

If you write it:

```sh
specbind template render spec <spec> research
```

If this is the first managed Markdown write in the run, read `okf-authoring` as
directed above before materializing the artifact. Do not read it a second time
when a Brief revision already required it.

Follow every scoped instruction the template returns. Omit `create` comments
from the materialized artifact, copy every `maintain` and `consume` comment
unchanged, and write it at the Spec's research path.

**Replace an existing research document. Do not append to it.** Read the current
document with `artifact read <spec> research --for maintain` first and preserve
its durable scoped comments. Research states
the current view of the investigation; a document that accumulates every
superseded finding makes the next reader work out which conclusions still hold.
Git holds the earlier drafts.

### Mark where each conclusion has to land

Research is **deleted at release finalization**. A conclusion recorded only here
is one the project has decided to forget. Mark each one:

| Mark | For |
| --- | --- |
| **Brief** | It changed what is being asked for |
| **Requirements** | It changes an obligation the system must meet |
| **Design** or **Contract** | It constrains or decides how the work is built |
| **Steering** | It is durable project knowledge beyond this milestone |
| **—** | It informed the choice and needs no afterlife |

The last row matters as much as the others. Marking everything for promotion
buries the conclusions that actually need it.

## 6. Report

Lead with the answer. In the project's language:

- what exists that this work can build on
- what is **missing**, what is **unknown**, and what is **constrained** — kept
  distinct, because conflating them hides work
- the realistic options with what each costs. A preference is welcome, visibly as
  a recommendation with its reasoning; a single option presented as analysis is a
  decision in disguise
- whether you wrote research, and why
- anything you routed back to the user

## Boundaries

- **Author Research, with one narrow exception:** after the user accepts a
  request change exposed by the analysis, revise that Spec's Brief to carry the
  requester's new terms. Requirements, Design, Contract, and `tasks.yaml` belong
  to their phases. Write no machine state.
- **Inform, do not decide.** Gathering the evidence does not make the decision
  yours.
- Not a gate and not a precondition. Nothing waits on this, and Design proceeds
  whether or not it ran.
- If the analysis suggests the scope itself was wrong — work in the wrong Spec, a
  boundary in the wrong place — report it. Discovery owns that change; do not
  create Roadmap items or Specs here.
- Research binds nothing. Requirements, Design, and Contract remain the
  authoritative statements, and no later artifact may defer meaning to what you
  write here.
