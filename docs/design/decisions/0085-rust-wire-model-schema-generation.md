# 0085: Generate JSON Schema from versioned Rust wire models

Status: Accepted

## Context

Decision 0083 made checked-in JSON Schema the structural authority and treated Rust artifact models as a separate execution representation. That protects the public format from incidental implementation changes, but it also creates two manually maintained descriptions of the same accepted structure. The project wants distributable schemas for runtime validation, fixtures, and later editor integration without paying an ongoing schema-to-model synchronization cost.

The current `spec.yaml` and `tasks.yaml` contracts contain patterns, tagged alternatives, conditional field requirements, and closed objects. These constraints can be represented through dedicated Rust wire types, newtypes, Serde representation attributes, and explicit Schemars customization. Cross-artifact references and lifecycle meaning remain outside structural generation.

## Decision

- Decision 0083 is superseded. For each supported structured-artifact version, a dedicated versioned Rust wire model is the authoritative structural contract.
- Wire models are public-format DTOs, conceptually organized by artifact and schema version such as `schema::spec::v1::SpecDocument` and `schema::tasks::v1::TasksDocument`. They are separate from lifecycle and domain models so an internal refactor does not silently change the artifact contract.
- Wire models derive or implement Serde serialization, Serde deserialization, and Schemars `JsonSchema` from one field and representation definition. Newtypes, enum variants, custom `JsonSchema` implementations, or schema transforms express accepted patterns and conditional structures that a simple derive cannot represent precisely.
- Schema generation explicitly selects JSON Schema Draft 2020-12 through pinned generator settings. It must not rely on Schemars' current default draft.
- Generated schema documents are deterministic checked-in distribution artifacts under `tools/specbind/schemas/<artifact>/v<version>.schema.json`. They are embedded in the matching binary and remain available for fixtures and future editor integration.
- CI regenerates every supported schema and fails on a working-tree difference. A dependency update or generator-setting change that alters generated output is reviewed explicitly; generated differences are never accepted as an unattended formatting update.
- Runtime validation remains layered:
  1. parse YAML into a neutral value while rejecting prohibited YAML features;
  2. select and evaluate the matching generated embedded schema;
  3. deserialize into the matching versioned wire model;
  4. convert to SpecBind domain types and apply semantic validation.
- Shared conformance fixtures remain mandatory:
  - every structurally valid fixture passes parser restrictions, generated-schema validation, and wire-model deserialization;
  - every parser-invalid or schema-invalid fixture fails at its declared layer;
  - structurally valid but semantically invalid fixtures deserialize before domain validation rejects them.
- A structural contract change lands the accepted design, versioned wire-model change, regenerated checked-in schema, conformance fixtures, and runtime tests together. A breaking representation change requires the applicable artifact schema-version decision rather than an unversioned Rust type edit.
- The existing hand-authored schema files are migration inputs for the first schema implementation increment after workspace bootstrap. The initial wire models must preserve their intended accepted and rejected fixture sets; byte-for-byte reproduction of incidental hand-authored layout is not required.
- Consumer projects cannot override runtime schemas through settings. Schema generation is a maintainer build operation, not an end-user CLI command in v1.

## Implementation status

The first Rust schema increment implements the v1 `spec.yaml` and `tasks.yaml` wire models, explicit Draft 2020-12 generation, embedded checked-in schemas, deterministic regeneration checks, and Rust-owned conformance fixtures. YAML feature restrictions, wire-to-domain conversion, and semantic lifecycle validation remain subsequent runtime-validation increments.

## Consequences

- Rust wire types and published schemas no longer drift through independent manual edits.
- Checked-in JSON Schema remains reviewable and distributable, but it is generated evidence rather than the source of truth.
- Schemars becomes a build and test dependency whose output stability is guarded by pinned versions, explicit settings, generated diffs, and conformance fixtures.
- The codebase carries a deliberate wire-to-domain conversion boundary, avoiding accidental coupling between public artifacts and convenient internal state representations.
