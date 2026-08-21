# 0139: Carry scoped instructions through artifact lifetime

Status: Accepted

## Context

Decision 0059 defines one unscoped `specbind:instruction` comment as
template-only guidance and rejects every such comment in a live artifact. That
works for first materialization, but it loses guidance that matters when the
artifact is revised later. Existing-spec workflows deliberately revise current
artifacts without reconciling them to the current template, so re-reading the
template would either miss a split collection artifact or let later template
edits silently change an existing artifact's maintenance contract.

Some guidance is useful only to a consumer as well. Mixing it with revision
guidance makes every agent process instructions unrelated to its task, while
putting it in ordinary prose makes agent workflow guidance part of the artifact's
human-facing specification.

## Decision

Every managed instruction comment names exactly one required scope immediately
after the `specbind:instruction` token:

```markdown
<!-- specbind:instruction create Choose the initial stable identity. -->
<!-- specbind:instruction maintain Preserve existing IDs while revising. -->
<!-- specbind:instruction consume Treat this document as context, not authority. -->
```

- `create` is template-only. The materializing workflow follows it and omits it
  from the live artifact.
- `maintain` is durable revision guidance. Materialization follows it and copies
  the complete comment into the live artifact. Every later revision reads and
  preserves it.
- `consume` is durable reader guidance. Materialization copies the complete
  comment into the live artifact. Workflows that use the artifact as input read
  it; revision workflows do not need to process it.
- A bare `specbind:instruction` and every unknown scope are invalid. There is no
  compatibility interpretation because an implicit default would hide a missed
  classification.
- A `create` instruction in a live Spec or Steering artifact is a leak and makes
  that artifact invalid. `maintain` and `consume` are valid live comments.
- Artifact-profile parsers mask every valid scoped instruction before evaluating
  semantic Markdown grammar. The persisted bytes, including durable comments,
  still participate in ordinary artifact fingerprints.

The template owns only the first materialization. Existing artifacts own their
copied durable instructions, so a later template edit does not synchronize,
replace, or reinterpret them. Changing an existing artifact's instruction is an
explicit edit to that artifact.

## Read projections

Template reads remain exact raw Markdown because materialization must see the
`create` guidance and carry both durable scopes forward.

Live reads remain exact raw Markdown when no option is supplied. Both read
families accept an explicit purpose projection:

```text
specbind artifact read <spec> <selector> --for maintain
specbind artifact read <spec> <selector> --for consume
specbind steering read <selector> --for maintain
specbind steering read <selector> --for consume
```

The CLI preserves all non-instruction bytes and the requested instruction scope
exactly, and omits comments belonging to the other durable scope. The caller
names the purpose; the CLI never infers it from workflow state. Raw reads remain
available for inspection, copying, and diagnostics.

`consume` cannot carry meaning required to understand an otherwise incomplete
artifact. Artifact semantics belong in its body, non-waivable product semantics
belong in protocols, workflow ordering belongs in Skills, and project-wide
authoring policy belongs in shared rules. The scope is only reader-directed
handling guidance local to this artifact.

## Implementation status

Implemented. The Rust document boundary parses, validates, masks, and projects
scoped instruction comments. Spec and Steering discovery reject missing or
unknown scopes and live `create` leaks. The CLI exposes explicit maintain and
consume projections while preserving the raw-read contract. Official English
and Japanese templates classify every instruction; materializing Skills carry
durable comments into live artifacts, revising Skills request maintain
projections, and consuming Skills request consume projections.

## Consequences

- Revision guidance survives the template that originally supplied it.
- Reader-only guidance no longer distracts an author revising the document.
- Live artifacts contain explicit agent guidance, but its scope is mechanically
  distinguishable from authoritative prose.
- Changing a template affects new materializations only, preserving Decision
  0059's no-synchronization boundary.
- Projects must classify every instruction explicitly.
