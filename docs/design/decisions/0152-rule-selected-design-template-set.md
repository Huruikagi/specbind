# 0152: Select the Design template set through project policy

Status: Accepted

## Context

Decision 0059 treats every discovered `SpecBind Design` template as the initial
Design decomposition. `specbind-design` therefore materializes every
`design/<artifact_id>` selector when no Design exists. That works for a single
general `design/main` scaffold, but it cannot add a standard focused scaffold
such as a screen design without forcing that document into CLI, library,
backend-only, and other Specs where it has no responsibility.

Making only the standard UI scaffold conditional would hard-code one product
domain and leave project-defined templates without the same mechanism. Projects
need to add concerns such as security, migration, observability, mobile UI, or
administration surfaces and state when each belongs in the complete current
Design set.

Template-local instructions are not the right authority. Decision 0092 assigns
a template its document shape and assigns selection and materialization timing
to the owning Skill. Applicability is also cross-template project policy: the
workflow must compare every candidate against the current Requirements and
repository before it can know the complete set.

## Decision

### Candidate templates and selection policy

`template list spec` reports the complete candidate template inventory. Its
presence does not by itself require materialization.

The installed project-owned rule
`settings/rules/design-template-selection.md` classifies every discovered
`design/<artifact_id>` selector exactly once:

- `required` selects the template for every Spec;
- `conditional` selects it only when its stated durable responsibility applies;
- `disabled` keeps the scaffold available but excludes it from selection.

Each entry is a level-two heading containing the exact selector in backticks.
Its first non-empty body line is exactly `Mode: required`, `Mode: conditional`,
or `Mode: disabled`. A conditional entry must contain substantive applicability
prose after the mode.

The CLI validates the deterministic relationship when the Rule is listed or
read: missing, duplicate, and unknown selectors; unsupported or absent modes;
conditional entries without a condition; and a policy with no required Design
all fail closed. Unlike the other optional preference Rules, this Rule is a
required routing input. Its absence is `ERROR RULE_REQUIRED`, because neither
the Skill nor an embedded fallback may silently invent project selection.

The CLI does not interpret applicability prose. The Design authoring and
validation Skills read the rule through `rule read ... --for consume` and apply
it to complete current Requirements, the existing Design set, repository facts,
and explicit user decisions. An unresolved conditional boundary is a user
question, not authority to create an empty precautionary document or silently
omit a responsibility.

### Non-waivable result

Project selection customizes decomposition, not Design meaning. The selected
set must:

- contain at least one Design;
- contain every required template and every applicable conditional template;
- own every durable responsibility needed by the current change;
- cover every active Requirement through the collection's traceability union;
- remain consistent with the Spec's Contract.

`disabled` cannot waive those outcomes. If the rule disables the only scaffold
capable of expressing an in-scope responsibility, the Skill reports the policy
conflict and asks the user to change scope or policy.

Before authoring, `specbind-design` reports every candidate's mode and selected
or omitted result with its concrete reason. This is an inspectable authoring
decision, not new persisted machine state. Gate fingerprints continue to cover
the actual live Design collection.

### Existing Design sets

Templates remain first-materialization scaffolds rather than continuous
reconciliation authority. A template or rule edit alone never adds, removes,
moves, or rewrites a live Design.

When current Requirements introduce a newly applicable durable responsibility,
the Design Skill may materialize the selected missing template. When a live
Design becomes disabled or no longer applicable, the Skill explains the
identity removal and asks before deleting it. It does not silently discard
persistent design knowledge.

### Standard installation

The English and Japanese embedded and installed Spec template sets contain:

- `design/main`, classified `required` by the default rule;
- `design/ui`, classified `conditional` by the default rule.

The UI condition applies when current Requirements introduce or change a
user-visible screen, navigation or interaction flow, input behavior, visual
feedback, responsive behavior, accessibility behavior, or another observable
UI state, including an internal change that alters those guarantees. Framework
presence alone is insufficient, and CLI, library, batch, documentation,
refactoring, or backend-only work with unchanged UI behavior omits it.

The UI scaffold covers users and contexts, screen inventory, navigation and
interaction, screen states and validation feedback, responsive behavior,
accessibility, component and service boundaries, and UI verification. It is a
behavioral screen-design document, not pixel-perfect artwork.

Projects add a custom `SpecBind Design` template and a matching selection-rule
entry to extend the set. No product code recognizes `ui` as a special selector;
the standard asset exercises the same generic rule contract as project-defined
templates.

## Consequences

- Installing a standard UI scaffold no longer forces a UI artifact into every
  Spec.
- Template availability, project classification, and per-Spec applicability
  are separate, inspectable decisions.
- User-defined Design decompositions use the same mechanism as official ones.
- The selection Rule is more structured and less optional than the other shared
  Rules, but its machine-readable boundary stays deliberately small; semantic
  conditions remain customizable prose evaluated by the Skills.
- Existing live Design remains stable across settings edits, while genuine new
  responsibilities can acquire a focused document.

## Implementation status

Implemented by the embedded English and Japanese `design/ui` scaffolds, the
installed `design-template-selection` Rule, Rule/template catalog validation,
the Design authoring and validation Skills, the design-authoring protocol, CLI
and catalog tests, user documentation, and DS7/DS8 behavioral scenarios.
