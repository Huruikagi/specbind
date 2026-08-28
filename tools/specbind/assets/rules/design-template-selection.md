---
type: SpecBind Rule
---

# Design template selection

This rule classifies every discovered `SpecBind Design` template. The project
owns the classifications and applicability conditions. It may add entries for
project-defined templates, change a template between the accepted modes, or
disable a template that remains available for later use.

Each `design/<artifact_id>` selector must appear exactly once as a level-two
heading. The first non-empty line below it is exactly `Mode: required`, `Mode:
conditional`, or `Mode: disabled`. A conditional entry must then state a
substantive applicability condition. The CLI validates those deterministic
parts; the Design workflow evaluates the condition against current
Requirements, the existing Design set, repository facts, and user decisions.

`required` means the Design is present for every Spec. `conditional` means it
is present only while its durable responsibility applies. `disabled` keeps the
scaffold available but excludes it from selection. Classification never waives
the product requirement for a non-empty Design set, complete active-Requirement
coverage, valid traceability, or a complete Contract.

## `design/main`

Mode: required

Use the main Design for the Spec's general architecture, boundaries, behavior,
failure handling, and verification strategy.

## `design/ui`

Mode: conditional

Select this Design when current Requirements introduce or change a
user-visible screen, navigation or interaction flow, input behavior, visual
feedback, responsive behavior, accessibility behavior, or another observable
UI state. Also select it when an internal change alters those existing
user-visible guarantees.

Do not select it merely because a UI framework exists, or for CLI, library,
batch, documentation, refactoring, and backend-only work whose user-visible UI
behavior remains unchanged. Ask the user when available evidence does not
resolve the boundary.
