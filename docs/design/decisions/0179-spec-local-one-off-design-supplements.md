# 0179: Assess and materialize Spec-local one-off Design supplements

Status: Accepted

## Context

Decision 0152 makes project-owned Design templates and their selection Rule the
right mechanism for a durable responsibility that recurs across Specs. It is
not the right place to persist a template and a Rule entry solely because one
current Spec needs a focused design document. Still, Decision 0061 already
allows a Spec's complete Design to be a collection of focused artifacts.

The Design workflow needs to surface and author a one-off split when its
investigation shows that an independent responsibility would be obscured in
`design/main`. Requiring an additional confirmation solely for that
evidence-backed decomposition interrupts a delegated or `--all` Plan run even
though the normal Design gate remains the review boundary.

## Decision

- During Design authoring, the Skill materializes a Spec-local supplement when all
  of these hold: the current change has a durable responsibility; it has an
  independent ownership boundary, failure or verification concern, or both; an
  existing focused Design does not communicate it clearly; and no project-owned
  candidate other than `design/main` is already applicable. The default main
  Design is the alternative to assess, not evidence that an independently
  reviewable responsibility needs no supplement.
- The assessment records a lowercase-kebab `artifact_id`, the responsibility,
  covered Requirement IDs, the alternative of extending an existing Design,
  and the intended path `specs/<spec>/design/<artifact_id>.md` below the
  configured SpecBind root in the authored Design set. It does not require a
  separate user confirmation. The Skill never creates a precautionary
  supplement.
- The Skill materializes the one-off document from the current `design/main`
  scaffold as an authoring shape. It replaces the identity with the assessed
  `artifact_id`, follows applicable creation guidance, preserves durable
  instructions, supplies valid traceability, and writes only the stated
  Spec-local path. The resulting artifact is an ordinary
  `SpecBind Design`: discovery, traceability, fingerprints, validation, and
  the Design gate cover it without a new schema or lifecycle state.
- A one-off supplement does not add a project template or change
  `design-template-selection`. If the same responsibility is independently
  selected in later Specs, the Design Skill recommends that `sb-configure`
  evaluate promotion to a project-owned conditional candidate; it does not
  perform that configuration change itself.

## Consequences

- Project settings remain a reusable policy surface rather than a history of
  one-off Specs.
- Design splitting remains evidence-backed and reviewable at the ordinary
  Design-gate boundary without inserting a decomposition-only pause.
- Existing Design collection and traceability contracts remain unchanged.

## Implementation status

Implemented by the `sb-plan` Design procedure, Design-skill regression checks,
the DS9 forward-test scenario, and the public customization guides. The CLI
already discovers and validates arbitrary `SpecBind Design` collection entries
under a Spec directory.
