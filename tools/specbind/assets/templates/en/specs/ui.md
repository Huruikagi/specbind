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

<!-- specbind:instruction maintain
Identify the users, their goals, and device, location, or other contexts that
affect UI behavior. Keep only distinctions needed for this Spec's screen
decisions rather than a general persona description.
-->

## Design system application

<!-- specbind:instruction maintain
Reference the principles, tokens, and components from Steering or project
implementation assets that this Spec applies. Do not duplicate their source of
truth; record only this Spec's application decisions, additions, intentional
exceptions, and reasons. Delete this section when there is no applicable shared
foundation and no Spec-local decision. Do not leave a new principle that future
Specs must inherit only in this document.
-->

## Screen inventory

<!-- specbind:instruction maintain
List the screens this Spec introduces or changes. Give each a screen ID that is
consistent within this document, then record its name, a one-line summary of its
user purpose, primary users, and main entry point. Keep item and state details
in each screen's "Screen design" section rather than duplicating them here.
-->

| Screen ID | Screen | Purpose | Primary users | Main entry point |
| --- | --- | --- | --- | --- |

## Navigation and interaction flow

<!-- specbind:instruction maintain
Describe transitions between screens and cross-screen operations such as back,
cancel, and resume. Keep operations contained within one screen under that
screen's "Actions and outcomes" section. Use a flow diagram only when prose or
a table would leave the relationship easy to misread. Delete this section when
there is no cross-screen flow.
-->

## Screen design

<!-- specbind:instruction maintain
Repeat the following subsection set for every screen in the inventory.

- Under "Purpose and display conditions," state what the user accomplishes and
  the permissions, state, or other preconditions for availability.
- Under "Display items," record items that affect user decisions, input,
  permissions, or state display. Exclude presentational labels and pixel-level
  placement, and group repeated items that share one structure.
- Under "Actions and outcomes," state each user-initiated action's preconditions,
  successful result or destination, and failure feedback. Reference "Navigation
  and interaction flow" for a shared cross-screen flow. Delete this subsection
  for a screen with no user-initiated action.
- Under "States and input feedback," describe applicable loading, empty, error,
  and unavailable states together with input-validation timing, placement, and
  recovery. Delete this subsection for a screen with no distinct state or input
  behavior.
- Under "Layout and information hierarchy," describe the spatial relationship
  of major regions, the priority of information and actions, and composition
  changes across viewport widths. Delete this subsection when the existing
  design system applies without a screen-specific decision. Use a simple ASCII
  wireframe only when prose or a table would make the spatial relationship easy
  to misread, and never as a pixel, color, or visual-finish specification.

Do not split a Design identity merely because the Spec has several screens.
Consider a split only for a durable UI responsibility that a separate owner
maintains and that changes independently.
-->

<!-- specbind:instruction create
Replace `<screen-id> <screen-name>` with actual values from the inventory and
repeat the subsection set for every screen. Do not leave the empty example
heading in a live artifact.
-->

### `<screen-id>` `<screen-name>`

#### Purpose and display conditions

#### Display items

| Item | Content or data source | Display or edit conditions |
| --- | --- | --- |

#### Actions and outcomes

| Action | Preconditions | Result or destination | Failure feedback |
| --- | --- | --- | --- |

#### States and input feedback

#### Layout and information hierarchy

## Responsive behavior

<!-- specbind:instruction maintain
Record shared breakpoint reasoning, input-method differences, and the
information priority preserved through rearrangement rather than simple hiding.
Keep screen-specific composition changes under that screen's "Layout and
information hierarchy" section. Delete this section when the Spec adds or
changes nothing from the design system's shared responsive behavior.
-->

## Accessibility

<!-- specbind:instruction maintain
State the keyboard, focus order, assistive reading, contrast, reduced-motion, or
other usage guarantees this Spec provides. Do not duplicate common design-system
standards; describe their application and any exception. Delete this section
when the Spec adds or changes no accessibility guarantee.
-->

## Component, data, and service boundaries

<!-- specbind:instruction maintain
Define UI component responsibilities, the owners of displayed data, and service
boundaries that receive updates or side effects. Do not duplicate screen items
or the Contract's standard seam inventory. Delete this section when `design/main`
already owns every required boundary and this document has no UI-specific
decision.
-->

## UI verification strategy

<!-- specbind:instruction maintain
State which layers verify the primary flows, states, input feedback, responsive
behavior, and accessibility. Describe observable outcomes that detect UI
failure rather than listing implementation techniques.
-->
