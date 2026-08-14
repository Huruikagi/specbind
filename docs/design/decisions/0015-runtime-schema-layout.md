# 0015: Keep versioned runtime schemas with the CLI

Status: Accepted

## Context

Target SpecBind uses structured `spec.yaml` and `tasks.yaml` artifacts under Decisions 0014 and 0013. Their fixed machine contracts need concrete, versioned schema files so design decisions, runtime validation, fixtures, and editor-facing tooling do not drift into separate definitions.

The schemas are product-managed runtime contracts. They are not consumer customization templates and do not belong under `settings/` or documentation-only paths.

## Decision

- Canonical schema documents live under `tools/specbind/schemas/`.
- Each artifact owns an independently versioned JSON Schema Draft 2020-12 document:

  ```text
  tools/specbind/schemas/
  ├── spec/
  │   └── v1.schema.json
  └── tasks/
      └── v1.schema.json
  ```

- YAML artifacts are parsed into a data model and then validated against the applicable JSON Schema selected by `schema_version`.
- YAML-specific restrictions such as duplicate keys, anchors, aliases, merge keys, and custom tags are rejected during parsing before JSON Schema validation.
- Cross-artifact references, lifecycle transitions, dependency graphs, fingerprints, and other semantic invariants remain Rust validation responsibilities after structural schema validation.
- Schema files are packaged with the CLI version that supports them. The Rust implementation may embed them in the binary or include them as immutable release resources, but runtime resolution must not depend on project-owned settings.
- The current TypeScript package includes `schemas/` in its package surface so migration tooling and tests can reference the same source files.
- Schema conformance fixtures live under `tools/specbind/test/fixtures/schemas/<artifact>/<version>/` until the Rust test layout supersedes them.

## Initial scaffold

The first checked-in schemas intentionally contain only fields already accepted for the common envelope. They are not wired into current TypeScript artifact generation or validation.

New fields are added as their decisions become accepted. A schema change must update:

- the applicable JSON Schema
- valid and invalid conformance fixtures
- runtime validation tests when implemented
- relevant target design or ADR references

Schema documents remain self-contained initially. Shared `$ref` documents should be introduced only when a stable repeated contract justifies the packaging and version-coupling cost.

## Consequences

- Runtime-owned schemas stay near CLI implementation and packaging rather than appearing user-customizable.
- `spec.yaml` and `tasks.yaml` may evolve their schema versions independently.
- Design discussions can land one accepted field at a time in executable contract files.
- JSON Schema covers structural validation without pretending to replace YAML parser checks or semantic Rust validation.
- Distribution tests must ensure supported schemas are present or embedded in every CLI artifact.

## Open questions

- Whether Rust treats JSON Schema as the runtime validator, generates it from typed models, or checks typed models and JSON Schema against shared conformance fixtures.
- Whether schemas later receive stable published `$id` URLs for editor integration.
- The cutover point at which Rust-owned fixtures replace or share the current TypeScript test-fixture location.
