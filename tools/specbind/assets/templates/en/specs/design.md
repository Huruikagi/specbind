---
type: SpecBind Design
artifact_id: main
---

<!-- specbind:instruction create bind=spec
Resolve `spec` to the canonical Spec identity in the current authoring context.
Replace every `{{spec}}` reference with that exact value and keep it in the
title so the artifact remains identifiable when read outside its directory.
-->

<!-- specbind:instruction create bind=artifact_id
Resolve `artifact_id` to the literal collection identity in this template's
Front Matter. Replace every `{{artifact_id}}` reference with that exact value
and keep it in the title so split Design documents remain distinguishable.
-->

# `{{spec}}` Design — `{{artifact_id}}`

<!-- specbind:instruction maintain
Add a Front Matter `requirement_ids` array listing every Requirement ID this
document covers, and repeat each one in an italic body marker of the exact form
`_Requirements: 1.1, 1.2_` next to the section that satisfies it. The Front Matter
set and the union of the body markers must match exactly.

Split a large change into several design documents by giving each its own
`artifact_id` and file. Describe only decisions this document owns. Keep the
design self-contained: Research may support a decision, but authoritative
decisions and rationale belong here. Remove sections that do not apply, and use
diagrams or tables only when they make a non-trivial relationship clearer.

Describe internal architecture and how this design realizes the persistent
Contract, but do not duplicate the Contract's canonical seam inventory. Record
enough concrete file boundaries, interfaces, failure behavior, and verification
strategy that different implementers should reach compatible results.
-->

## Overview

<!-- specbind:instruction maintain
Summarize the decisions this document owns and the central approach by which it
realizes Requirements. Do not repeat details from the sections below.
-->

### Goals

<!-- specbind:instruction maintain
State the technical results this design establishes. Do not paraphrase
Requirements; explain what the realization approach achieves.
-->

### Non-goals

<!-- specbind:instruction maintain
Record technical responsibilities that appear related but are deliberately not
addressed by this design. Delete this subsection when there is no meaningful
non-goal.
-->

## Architecture and boundaries

<!-- specbind:instruction maintain
Describe the resulting responsibility split, dependency direction, ownership
boundaries, and why this shape was selected. Show only the differences needed
for this change rather than inventorying existing architecture.
-->

## System flows

<!-- specbind:instruction maintain
Describe operations that cross several boundaries, state transitions, or
asynchronous interactions so their order and failure paths are clear. Delete
this section when all relevant behavior is local and self-evident.
-->

## Components and interfaces

<!-- specbind:instruction maintain
For each new or changed component, state its responsibility, inputs, outputs,
guarantees, and allowed dependencies. Include language-specific signatures only
when their exact shape is itself a compatibility decision.
-->

## Data models

<!-- specbind:instruction maintain
Describe new or changed concepts, persistence shapes, consistency boundaries,
and evolution strategy. Delete this section when data shape and ownership do not
change.
-->

## Error handling

<!-- specbind:instruction maintain
Describe failures at each relevant boundary, caller or user outcomes, retries,
degraded operation, and observability. Delete this section when existing policy
applies unchanged and no additional decision is needed.
-->

## Verification strategy

<!-- specbind:instruction maintain
For each important guarantee and failure path, state where it is verified and
what observable result proves it. Do not give generic testing advice or repeat
the project's standard command list.
-->

## Migration and rollout

<!-- specbind:instruction maintain
Describe the order, compatibility period, and rollback conditions for safely
moving existing data, users, or callers to this design. Delete this section when
one safe replacement step needs no migration decision.
-->

## Risks and alternatives

<!-- specbind:instruction maintain
Record concrete residual risks and why viable alternatives were not selected.
Delete this section when no risk or alternative materially affects the decision.
-->
