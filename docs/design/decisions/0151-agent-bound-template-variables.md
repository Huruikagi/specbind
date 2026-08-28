# 0151: Resolve template variables through bound agent instructions

Status: Accepted

Supersedes: [Decision 0149](./0149-bound-spec-template-rendering-variable.md)
and the built-in-variable clauses of
[Decision 0150](./0150-attributable-fail-closed-default-scaffolds.md)

## Context

Decision 0149 introduced `{{spec}}` as a CLI-rendered built-in, and Decision
0150 added `{{artifact_id}}`. Requiring an explicit
`specbind:instruction create bind=<variable>` beside each variable already
provides the missing value-source and authoring authority that motivated the
CLI whitelist. Keeping a second CLI-owned value system limits project templates
to identities the executable knows and prevents useful creation-time values,
such as repository observations, user answers, or data obtained through an
available external tool.

The authoring agent already has the target Spec, selected template, project
instructions, and user request in context. Template variables should connect
that context to explicit project-owned creation guidance rather than grow a
general-purpose CLI renderer.

## Decision

- Managed Markdown template bodies may contain project-defined variables in the
  exact form `{{name}}`. A name is non-empty and contains no whitespace or brace
  characters; it may otherwise use Unicode, so names such as `今日の天気` are
  valid.
- Every distinct variable name has exactly one template-only instruction whose
  opening token sequence is
  `specbind:instruction create bind=<name>`. One binding may serve any positive
  number of references to that name.
- The binding instruction is the value-resolution recipe and authority. The
  authoring agent follows it once, obtains one value, and replaces every
  reference to that name with the same value. Different values require
  different names.
- SpecBind validates missing, duplicate, and unused bindings mechanically. Only
  `create` instructions may bind variables. Variable references inside managed
  instruction comments are explanatory text and do not count as uses.
- SpecBind does not whitelist names, supply values, invoke tools, or substitute
  variables. There are no built-in variables. The existing `spec` and
  `artifact_id` bindings in official templates are ordinary bindings whose
  instructions tell the agent where to obtain their values.
- Template variables remain forbidden in Front Matter and other machine-owned
  identity or path fields. They are authoring primitives for Markdown bodies,
  not a way to defer structural identity.
- `template read` validates the resolved template inventory and returns the raw
  UTF-8 template, including every variable and instruction. The CLI removes
  `template render`; product-managed authoring skills use `template read` and
  perform materialization.
- Materialization follows the scoped-instruction lifecycle: the agent resolves
  every binding, replaces every reference, omits `create`, preserves
  `maintain` and `consume`, and validates the completed live artifact before
  treating it as written successfully. A recognized unresolved variable or a
  leaked `create` instruction remains a live-artifact diagnostic.
- A binding cannot grant tool access or additional authority. If its recipe
  needs unavailable data, permissions, or a user decision, the agent follows
  the normal tool and interaction boundaries and stops rather than inventing a
  value.

Decision 0150's fail-closed minimum-content checks remain accepted. Only its
built-in `artifact_id` rendering behavior is replaced here.

## Compatibility

This change occurs on the pre-`1.0.0` stabilization line governed by Decision
0144. Existing project templates using the former built-ins already carry the
required bindings, so updated product skills materialize them without a
template rewrite. Direct callers of `template render` move to `template read`
and apply the returned binding instructions. No durable live artifact or
structured lifecycle state is migrated.

## Consequences

- Projects can define creation-time values without adding executable-specific
  resolvers or waiting for a new CLI release.
- The CLI owns syntax and correspondence; the agent owns semantic resolution
  and substitution.
- Repeated references are consistent because one binding produces one value for
  the whole materialization.
- Identity attribution remains present in the official scaffolds, but its
  correctness is an authoring responsibility rather than a CLI substitution
  guarantee.
- Removing the misleading render command leaves one raw template read surface
  and one materialization owner.
