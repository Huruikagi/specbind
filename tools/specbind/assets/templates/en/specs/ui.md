---
type: SpecBind Design
artifact_id: ui
---

<!-- specbind:instruction create bind=spec
Resolve `spec` to the canonical Spec identity in the current authoring context.
Replace every `{{spec}}` reference with that exact value and keep it in the
title so the artifact remains identifiable when read outside its directory.
-->

<!-- specbind:instruction create bind=artifact_id
Resolve `artifact_id` to the literal collection identity in this template's
Front Matter. Replace every `{{artifact_id}}` reference with that same value.
-->

# `{{spec}}` Design — `{{artifact_id}}`

<!-- specbind:instruction maintain
Keep this document only while the project selection rule classifies it as
applicable to the Spec's current responsibilities. Add a Front Matter
`requirement_ids` array for every Requirement this document covers and repeat
the same IDs in exact `_Requirements: 1.1, 1.2_` body markers beside the
decisions that realize them.

Describe user-visible behavior and states, not pixel-perfect artwork. Delete
sections that do not apply. Split another Design identity only for a durable UI
responsibility that readers and maintainers follow independently.
-->

## Users and contexts

## Screen inventory

## Navigation and interaction flow

## Screen behavior

### Primary content and actions

### Loading, empty, error, and unavailable states

### Input and validation feedback

## Responsive behavior

## Accessibility

## Component, data, and service boundaries

## UI verification strategy
