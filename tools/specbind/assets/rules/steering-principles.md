---
type: SpecBind Rule
---

# Steering principles

This rule is the project's preferred style for durable project guidance. It is a
`SpecBind Rule`: your project owns this file and may strengthen, relax, replace,
or remove it. Removing it leaves the steering workflow intact and only removes
these conventions.

How steering documents are discovered, identified, installed, and synchronized
is owned by the CLI contract and the steering skill. This file is about what
belongs inside them.

## What steering is for

Steering carries knowledge that outlives any single change: how this project is
built, what it values, and the constraints every change inherits.

A fact belongs in steering when a competent newcomer would otherwise have to
discover it by reading widely, or by getting it wrong once. Anything narrower
belongs to the artifacts of the change that needs it.

## Keep it durable

- Prefer statements that will still be true in six months.
- Record the constraint and its reason. A rule whose reason is lost is followed
  superstitiously or abandoned wholesale.
- Leave out what a reader sees immediately from the repository. A list of
  directories that the directory listing already shows is maintenance with no
  benefit.
- Leave out the transient: current scope, in-flight migrations, and the status
  of work under way. Those belong to the milestone that owns them.

## Granularity

Prefer several focused documents over one long one. A document covering a single
concern is easier to find, easier to trust, and easier to retire.

Split when a document starts needing its own table of contents. Merge when two
documents cannot be read independently.

## Examples earn their place

A short concrete example is usually worth more than another paragraph of
description, especially for conventions that are easy to describe and easy to
get subtly wrong.

Keep examples current. A stale example is read as authoritative and quietly
propagates the thing it demonstrates.

## Preservation

Steering is edited, not accumulated. When guidance changes, revise it in place
so the document keeps describing the project as it is now.

When history matters, Git holds it. A steering document that also records what
the project used to do makes readers guess which parts are still in force.

## Review questions

- Would a newcomer need this before their first change?
- Is this still true, or was it true when it was written?
- Does this say why, or only what?
- Is this the durable version of a decision, or the notes from the day it was
  made?
