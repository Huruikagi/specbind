---
type: SpecBind Design
artifact_id: main
---

<!-- specbind:instruction create bind=spec
The CLI renders `spec` as the canonical Spec identity. Keep it in the title so
the artifact remains identifiable when read outside its directory.
-->

<!-- specbind:instruction create bind=artifact_id
The CLI renders `artifact_id` from the literal collection identity in Front
Matter. Keep it in the title so split Design documents remain distinguishable.
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

### Goals

### Non-goals

## Architecture and boundaries

## System flows

## Components and interfaces

## Data models

## Error handling

## Testing strategy

## Migration and rollout

## Risks and alternatives
