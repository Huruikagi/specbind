# 0177: Verify materialized Steering scaffold conformance

Status: Accepted

## Context

Decision 0117 deliberately used `steering list` as the validation surface for
identity and discovery. It did not prove that a newly materialized document
preserved the durable instructions of the particular scaffold used to create
it. A project-specific comparison script consequently duplicated SpecBind's
instruction and template semantics.

## Decision

Add the read-only command:

```text
specbind steering check <artifact_id> --template <selector>
```

The document selector identifies the live Steering artifact; `--template`
names the resolved project-or-embedded scaffold. The explicit scaffold selector
is required because a live Steering document does not persist creation
provenance, and `document` deliberately accepts an author-chosen identity.

The command resolves both through the existing inventories and fails when the
fixed identity declared by a scaffold differs from the live artifact selector.
It validates that every durable `maintain` or `consume` instruction block in
the live document exactly equals the selected scaffold's complete comment set,
and reports leaked `create` instructions, unresolved named outputs, and any
scaffold placeholder retained outside an instruction comment. It is
deterministic, read-only, and does not compare or constrain authored prose.

## Consequences

`sb-steering` uses this command after materialization instead of asking each
project to reproduce the comparison. Decision 0117's list-based discovery
validation remains the authority for collection identity; this command adds the
separate proof of a selected scaffold's materialization contract.
