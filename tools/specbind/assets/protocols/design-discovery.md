# Design discovery protocol

This protocol is the shared baseline for the repository and dependency
investigation that must precede authoring a Design. It applies to every
supported agent and cannot be waived by a project template or shared rule.

It owns what must be established and when investigation is insufficient. It does
not own tool selection, mode switching, prompt wording, or how many passes to
run; the design skill owns that control flow.

## Purpose

Discovery exists so the Design can be authored as a self-contained document
about a system that actually exists.

A Design written from assumption produces two failures that surface late: a
Contract that describes a seam the code does not have, and tasks that cannot be
executed as written. Discovery is finished when the author can state the
approach, its boundary, and its realization without needing the reader to
consult the source to understand what is being changed.

## What must be established

Investigation is proportional to the change, but these must be settled before
authoring, at whatever depth the change warrants:

- **Requirement realization.** Which active Requirement IDs this Design must
  satisfy, and what each one actually demands technically.
- **Current reality.** How the affected area works today: structure, the
  patterns already in use, the data and control flow being changed.
- **Reuse and extension points.** What already exists that should be extended or
  reused rather than reimplemented, and the existing conventions to follow.
- **Boundary and seam.** What this Spec owns, what it consumes from adjacent
  Specs, and which existing Contract entries the change touches, adds, or
  invalidates. A Design that changes a seam without recognizing it produces a
  Contract that later fails cross-spec review.
- **External dependencies.** For each new or upgraded dependency: the actual
  current API, version compatibility with the existing stack, and any
  constraint, limit, or licensing consideration that affects the approach.
- **Risk.** Where the change can break existing behavior, and which risks are
  material enough to shape the design rather than be handled during
  implementation.

## Verify rather than recall

Claims about the current repository and about external interfaces are facts to
check, not memories to reproduce.

- Read the code that will be changed. Do not infer its structure from names,
  documentation, or an earlier session.
- Confirm an external API, version, or behavior against its current
  authoritative source when the design depends on it. A remembered signature is
  a likely defect, and dependency surfaces change between releases.
- When a fact cannot be verified, record it as an open question rather than
  adopting the most convenient assumption.

## When investigation must go deeper

Shallow investigation is appropriate for a change that follows an established
pattern inside a well-understood area. Investigation must deepen when any of the
following appears, even if the change looked small at the outset:

- the approach requires an architectural change rather than an extension
- an external service or unfamiliar dependency is being integrated
- the change is security- or privacy-sensitive
- the change is on a performance- or reliability-critical path
- the affected area is poorly documented, inconsistent, or not understood
- the change touches a seam other Specs consume
- the evidence contradicts steering, an existing Contract, or a prior decision

Recognizing one of these mid-investigation is a normal outcome, not a failure of
planning. Continuing at the original depth after seeing one is the defect.

## Unresolved gaps are carried, not dropped

Discovery does not have to resolve everything. It has to be honest about what it
did not resolve.

- A question that blocks choosing the approach must be resolved or escalated
  before authoring; it cannot be deferred into the Design as an open item.
- A question that does not block the approach but will affect implementation is
  carried forward explicitly so the task plan can account for it.
- Investigation conclusions that will outlive this change belong in the
  authoritative artifacts or, when they need their own record, in Research under
  the gap-analysis protocol. They do not live only in the run's context.

## What discovery does not do

- It does not choose the design. Synthesis, simplification, and the realization
  argument belong to the design-authoring protocol.
- It does not write or approve authoritative artifacts.
- It does not substitute for cross-spec review. Recognizing a seam is discovery;
  judging compatibility across the milestone is a separate accepted review.
