# 0083: Keep JSON Schema authoritative over Rust artifact models

Status: Superseded by [Decision 0085](./0085-rust-wire-model-schema-generation.md)

## Context

Decision 0015 places versioned runtime JSON Schemas beside the CLI and separates YAML parser restrictions, structural validation, and semantic lifecycle validation. The Rust implementation still needs an explicit authority boundary: generating schemas from Rust types would make a public artifact contract depend on implementation details and generator behavior, while maintaining unrelated schemas and types without conformance tests would allow drift.

## Decision

- The checked-in JSON Schema for each structured artifact version is the authoritative v1 structural contract.
- The Rust CLI loads the artifact as YAML, rejects YAML-specific prohibited features, selects the schema from the explicit `schema_version`, validates the parsed value against the embedded matching JSON Schema, and only then converts it into the corresponding typed Rust model.
- Typed Rust models are internal execution models. They may make valid states convenient to handle but must not silently accept, reject, default, rename, or reinterpret fields differently from the authoritative schema.
- Rust semantic validation runs after successful structural validation. It owns cross-field lifecycle invariants, cross-artifact references, graph checks, fingerprints, freshness, and guarded transition rules that JSON Schema does not express.
- V1 does not generate checked-in JSON Schemas from Rust types. A Rust schema-generation library may be used only as a development comparison aid; its output is not publication authority and never rewrites the contract automatically.
- A structural contract change lands the accepted design, JSON Schema, conformance fixtures, Rust model, and runtime tests together. Changing only Rust types does not change the artifact format.
- Shared conformance fixtures are the drift gate:
  - every structurally valid fixture must pass parser restrictions, JSON Schema validation, and typed Rust deserialization;
  - every parser-invalid or schema-invalid fixture must fail at its declared layer;
  - structurally valid but semantically invalid fixtures remain separate and must deserialize before Rust semantic validation rejects them.
- CI runs the same fixture corpus against every supported artifact version embedded in the binary. A schema-valid value that the Rust model cannot deserialize, or a schema-invalid value that reaches semantic validation, is a conformance failure.
- The source schemas remain under `tools/specbind/schemas/` and are embedded into the matching binary. Consumer projects do not override them through settings.
- The concrete Rust crates used for YAML parsing and JSON Schema evaluation are implementation choices, provided they satisfy the fixtures and supported Draft 2020-12 behavior.

## Consequences

- Artifact compatibility remains reviewable as stable checked-in data rather than an incidental Rust code-generation result.
- Rust gains typed internal models without creating a second public structural contract.
- Parser, schema, deserialization, and semantic failures have distinct owners and diagnostics.
- Schema and model drift becomes a test failure before release.
