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

- YAML artifacts are parsed into a neutral value and then validated against the applicable JSON Schema selected by `schema_version`.
- YAML-specific restrictions such as duplicate keys, anchors, aliases, merge keys, and custom tags are rejected during parsing before JSON Schema validation.
- Under Decision 0085, a dedicated versioned Rust wire model is the authoritative structural contract. Schemars generates the checked-in JSON Schema from that model; after schema validation passes, the CLI deserializes the value into the same wire model, converts it to domain types, and applies semantic validation.
- Cross-artifact references, lifecycle transitions, dependency graphs, fingerprints, and other semantic invariants remain Rust validation responsibilities after structural schema validation and typed deserialization.
- Schema files are embedded in the CLI version that supports them. Runtime resolution does not depend on project-owned settings.
- The current TypeScript package includes `schemas/` in its package surface so migration tooling and tests can reference the same source files.
- Schema conformance fixtures live under `tools/specbind/test/fixtures/schemas/<artifact>/<version>/` until the Rust test layout supersedes them.

## Initial scaffold

The first checked-in schemas intentionally contain only fields already accepted for the common envelope. They are not wired into current TypeScript artifact generation or validation.

New fields are added as their decisions become accepted. A structural change must update:

- the applicable versioned Rust wire model
- the generated JSON Schema
- valid and invalid conformance fixtures
- runtime validation tests when implemented
- relevant target design or ADR references

Schema documents remain self-contained initially. Shared `$ref` documents should be introduced only when a stable repeated contract justifies the packaging and version-coupling cost.

## Consequences

- Runtime-owned schemas stay near CLI implementation and packaging rather than appearing user-customizable.
- `spec.yaml` and `tasks.yaml` may evolve their schema versions independently.
- Design discussions can land one accepted field at a time in versioned wire models and their generated contract files.
- JSON Schema covers structural validation without pretending to replace YAML parser checks or semantic Rust validation.
- Shared conformance fixtures require schema-valid values to deserialize into the matching Rust wire models and keep layer ownership aligned.
- Distribution tests must ensure supported schemas are present or embedded in every CLI artifact.

## Open questions

- Whether schemas later receive stable published `$id` URLs for editor integration.
- The cutover point at which Rust-owned fixtures replace or share the current TypeScript test-fixture location.
