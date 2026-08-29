# 0045: Use OKF for managed Markdown artifacts

Status: Accepted

Decision 0155 removes the Contract from the OKF Markdown artifact set and gives
it the fixed structured `contract.yaml` representation. Other managed Markdown
profiles continue to follow this decision.

## Context

SpecBind needs documents that remain comfortable for humans and agents to read while exposing a small amount of metadata to deterministic CLI operations. A private Markdown grammar or an ad hoc frontmatter convention would create another format for producers and consumers to learn.

The Open Knowledge Format (OKF) v0.2 already defines a minimal, vendor-neutral model for this boundary: a hierarchical bundle of UTF-8 Markdown concept documents with YAML frontmatter, a required non-empty `type`, standard Markdown links, and producer-defined extension fields.

## Decision

- The configured `{{SPEC_DIR}}` is an OKF v0.2 Knowledge Bundle.
- Every non-reserved Markdown file installed, generated, or maintained under `{{SPEC_DIR}}` is an OKF concept document:
  - the file begins with parseable YAML frontmatter delimited by `---`
  - frontmatter contains a non-empty `type`
  - the remainder is free-form Markdown
- OKF-reserved `index.md` and `log.md` retain their OKF meanings if SpecBind introduces them. Neither file is required in v1.
- SpecBind defines an artifact-specific profile on top of OKF. A profile may require an exact `type` value and additional fields, types, or invariants needed by the CLI.
- The active roadmap uses `type: SpecBind Roadmap`. Its authoritative `milestone_id`, Decision 0054 `baseline_revision`, `target_release`, and Decision 0046 work-item index live in YAML frontmatter; the body remains human- and agent-readable context and rationale.
- The v1 roadmap profile has no `schema_version` field. Its accepted `type` and field contract are sufficient while only one representation exists; a version field is introduced only alongside a future incompatible representation and an explicit migration rule.
- Every SpecBind OKF profile accepts and preserves unknown top-level frontmatter fields when round-tripping a document, as required by OKF. This common extension rule also applies to state-oriented concepts, managed templates, and profiles whose known fields are otherwise tightly constrained.
- Known profile fields remain required and type-checked according to their profile. The extension rule does not permit unknown members inside a SpecBind-owned structured value such as `heading_labels` or a roadmap work item; those nested shapes may remain closed where their defining decision says so.
- Unrecognized top-level fields carry no SpecBind semantics. Bundled CLI operations and skills must not depend on them until a later decision standardizes the field and defines any required migration. This differs intentionally from strict standalone artifacts such as `spec.yaml` and `tasks.yaml`.
- The full Markdown file, including frontmatter, remains the input to existing Markdown fingerprints after line-ending normalization.
- Optional OKF provenance, trust, and lifecycle fields do not replace SpecBind lifecycle state or gate evidence. SpecBind-owned producers do not emit or interpret those fields as approval authority unless a later decision defines an explicit mapping.
- Files outside `{{SPEC_DIR}}`, including agent packages and repository documentation, are not made part of the consumer project's OKF bundle by this decision.
- Decision 0094 exposes the immutable `okf-authoring` protocol used by agents to preserve these invariants without making the targeted OKF baseline project-customizable.

## Consequences

- The active roadmap begins in this shape before its body:

  ```markdown
  ---
  type: SpecBind Roadmap
  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
  baseline_revision: 0123456789abcdef0123456789abcdef01234567
  target_release: null
  ---
  ```

- The default `brief.md`, `requirements.md`, `design.md`, `contract.md`, and `implementation-notes.md`, plus steering documents, rules, templates, and the release adapter receive artifact-appropriate OKF frontmatter in the target model. Decision 0057 discovers spec-local concept documents by type and treats those filenames as defaults rather than identity. The per-spec `log.md` accepted by Decision 0048 is an OKF reserved file and has no frontmatter.
- Free-form documents such as `implementation-notes.md` remain free-form in their Markdown body; OKF frontmatter is the only common structural requirement.
- Project customization can add metadata without waiting for a SpecBind schema revision, while the CLI retains deterministic ownership of the fields it understands.
- Exact profiles for each artifact, including their canonical `type` values and any additional required fields, can be accepted incrementally with that artifact's schema.

## Reference

- [Open Knowledge Format v0.2 specification](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)

## Implementation status

The Rust discovery boundary now recognizes UTF-8 concept documents with first-line YAML Front Matter delimiters, requires a non-empty `type`, preserves tolerance for unknown valid OKF types and extension fields, and excludes reserved `index.md` and `log.md` files from concept routing. Front Matter uses the ordinary YAML deserialization boundary rather than inheriting the narrower parser restrictions owned by versioned `spec.yaml` and `tasks.yaml`.
