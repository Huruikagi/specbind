# 0132: Resolve template provenance and target paths for one Spec

Status: Accepted

Decision 0152 later changes "every Design entry" below to every entry selected
by the required project Rule; the inventory remains the complete candidate set.

## Context

[Decision 0059](./0059-okf-artifact-templates.md) makes a Spec template's path
relative to `settings/templates/specs/` its initial output path relative to a
target Spec directory. `template list spec` exposes that `output_path` and
whether the selected template is project-owned or embedded. A raw
`template read spec <selector>` deliberately exposes only the Markdown body.

An authoring workflow that calls only the raw read therefore receives neither
the template's provenance nor a complete target path. It has to combine the
configured SpecBind root, the `specs/` collection, the canonical Spec ID, and
the template-relative output itself. A missing live artifact cannot supply the
answer through `artifact list`, because that inventory describes current
authoritative state rather than future scaffold targets.

## Decision

The read-only template command family adds:

```text
specbind template resolve spec <spec> <selector>
```

V1 accepts only the `spec` scope for this operation. The command requires an
existing structurally readable canonical Spec and a template inventory without
diagnostics. It resolves one selector and reports:

- selector, source, type, and optional `artifact_id`;
- `template_path` and template-relative `output_path`;
- `project_path`, the exact project-root-relative path
  `<specDir>/specs/<spec>/<output_path>`.

The operation never creates, overwrites, or validates a completed live
artifact. Its result is authoring location information, not mutation authority.
`template read` remains the raw-content operation and gains no wrapper.

When `specbind-design` creates a new Design set, it lists the resolved Spec
templates first, applies Decision 0152's selection Rule, resolves each selected
entry, reads its body, and writes the authored result only to the reported
`project_path`. It resolves the Contract target the same way. The listing makes
project-owned versus embedded provenance visible without requiring different
read syntax for the two sources.

`artifact list` remains an inventory of artifacts that exist. It does not add
placeholder entries for absent outputs, because doing so would merge current
authority with possible future materialization.

## Consequences

- New Design and Contract creation no longer depends on a guessed filename or
  inferred directory.
- A project with a split Design template set materializes every configured
  Design target rather than silently assuming only `design/main`.
- Embedded Contract and project-owned Design templates retain one uniform raw
  read interface while their ownership remains observable.
