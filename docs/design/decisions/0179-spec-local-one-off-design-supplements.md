# 0179: Propose and materialize Spec-local one-off Design supplements

Status: Accepted

## Context

Decision 0152 makes project-owned Design templates and their selection Rule the
right mechanism for a durable responsibility that recurs across Specs. It is
not the right place to persist a template and a Rule entry solely because one
current Spec needs a focused design document. Still, Decision 0061 already
allows a Spec's complete Design to be a collection of focused artifacts.

The Design workflow needs to surface a one-off split when its investigation
shows that an independent responsibility would be obscured in `design/main`.
The maintainer should confirm a concrete, evidence-backed proposal rather than
having to anticipate the document before the investigation.

## Decision

- During Design authoring, the Skill proposes a Spec-local supplement when all
  of these hold: the current change has a durable responsibility; it has an
  independent ownership boundary, failure or verification concern, or both; an
  existing selected Design does not communicate it clearly; and no
  project-owned candidate is already applicable.
- The proposal names a lowercase-kebab `artifact_id`, the responsibility,
  covered Requirement IDs, the alternative of extending an existing Design,
  and the intended path `specs/<spec>/design/<artifact_id>.md` below the
  configured SpecBind root. It is a user decision. The Skill never creates a
  precautionary supplement.
- After explicit confirmation, the Skill may materialize the one-off document
  from the current `design/main` scaffold as an authoring shape. It replaces
  the identity with the proposed `artifact_id`, follows applicable creation
  guidance, preserves durable instructions, supplies valid traceability, and
  writes only the stated Spec-local path. The resulting artifact is an ordinary
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
- Design splitting remains explicit and reviewable, while users see the option
  only after the agent has evidence for it.
- Existing Design collection and traceability contracts remain unchanged.

## Implementation status

Implemented by the `sb-plan` Design procedure, Design-skill regression checks,
the DS9 forward-test scenario, and the public customization guides. The CLI
already discovers and validates arbitrary `SpecBind Design` collection entries
under a Spec directory.
