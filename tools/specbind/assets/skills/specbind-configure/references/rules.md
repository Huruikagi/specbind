# Shared Rules

Use Rules for project-owned authoring or judgment policy shared across several
artifacts, Agents, phases, or reviews. Keep one-artifact creation, maintenance,
or consumption guidance in that template's scoped instructions instead.

## Inspect and edit

```sh
specbind rule list
specbind rule read <selector> --for maintain
specbind rule read <selector> --for consume
```

The accepted selector set is closed. Do not add meaning by dropping an unknown
file below `settings/rules/`. An absent ordinary preference Rule means no
project customization; product protocols and Skill obligations still apply.

`language-style` is the cross-artifact prose preference consumed by every
product Skill. Installation offers its Japanese default only when the
configured language is `ja`; absence in any language is valid. Keep exact
commands, paths, fields, states, diagnostics, structured output, and quoted
output outside this prose policy.

Rules may contain `maintain` and `consume` instructions, never `create`.
Preserve non-waivable CLI structure, lifecycle, protocol, and Skill contracts.

`design-template-selection` is different from optional preference Rules: it is
required routing input and must classify the complete current Design candidate
set exactly once. Coordinate its change with the template procedure.

## Aftercare and verification

Read both projections again. Explain which future authoring, validation, or
review workflows will consume the new policy. Offer review of affected existing
artifacts, but do not rewrite them merely because a Rule changed. Semantic
revision belongs to each artifact's owning workflow.
