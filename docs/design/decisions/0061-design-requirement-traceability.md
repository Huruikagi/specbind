# 0061: Make design Requirement traceability explicit in Front Matter and body markers

Status: Accepted

## Context

The `SpecBind Design` profile is a collection, so one spec may split its current design across several focused Markdown files. The CLI needs a deterministic mapping from each design artifact to the Requirement IDs it addresses, while reviewers and authoring agents also need to see those mappings in the document body without inspecting Front Matter separately.

The inherited cc-sdd task format already uses an italic `_Requirements: 1.1, 2.3_` marker. Reusing that narrow convention is more readable than marking each ID as inline code and avoids treating arbitrary numeric prose as traceability evidence.

## Decision

- Every live `SpecBind Design` artifact requires the three SpecBind-owned Front Matter fields `type`, `artifact_id`, and `requirement_ids`:

  ```yaml
  ---
  type: SpecBind Design
  artifact_id: persistence
  requirement_ids:
    - "1.1"
    - "2.3"
  ---
  ```

- `requirement_ids` is a non-empty YAML array of unique strings. Every value must be a canonical Requirement ID extracted under Decision 0060 and must exist in the current `SpecBind Requirements` artifact.
- Array order has no semantic meaning. Overlap between different design artifacts is allowed because one Requirement may affect several design concerns.
- Additional valid top-level Front Matter fields are allowed and preserved under the common Decision 0045 extension rule. An unrecognized field is project metadata: the CLI does not assign it lifecycle, identity, traceability, or gate semantics, and bundled skills must not depend on it as a stable SpecBind contract.
- Projects may namespace custom fields to reduce future naming collisions, but v1 does not require a particular extension prefix. A field becomes part of the stable SpecBind contract only through a later profile decision and migration definition.
- The Markdown body is otherwise template-defined free-form content. The CLI does not require a particular H1, title field, section inventory, or section order; only the explicit Requirement marker contract below is machine-recognized for this profile.
- Each live design artifact must contain one or more explicit italic Requirement markers using this canonical presentation:

  ```markdown
  _Requirements: 1.1, 2.3_
  ```

- The CLI recognizes a marker only when a Markdown emphasis node's complete extracted plain text matches this grammar:

  ```text
  Requirements: <ID>(, <ID>)*
  ```

- `Requirements:` is the exact case-sensitive ASCII label. The separator is an ASCII comma followed by one space. Each `<ID>` is in the canonical Decision 0060 form. The underscore form is the canonical authoring style; equivalent Markdown emphasis syntax has the same parsed meaning.
- A document may contain several markers and may repeat an ID where the relevant design discussion appears. The set union of IDs in all recognized body markers must exactly equal that document's Front Matter `requirement_ids` set.
- A Front Matter ID absent from the body marker set, a body-marker ID absent from Front Matter, or an ID that does not exist in the current requirements artifact is a structural validation error.
- Bare numeric prose, strong emphasis, inline code, fenced code, HTML comments, and partial emphasis nodes do not create traceability references. Ordinary italic prose remains valid because it does not match the complete marker grammar.

## Active-scope coverage

- Structural validation checks every declared or marked ID against the complete current requirements artifact, regardless of milestone scope.
- For an active change, the complete discovered `SpecBind Design` collection must cover every ID in `spec.yaml.active_change.requirement_ids` through the union of its `requirement_ids` sets.
- Requirements outside the active set may remain mapped by the persistent current design, but the current design gate does not require every inactive Requirement ID to appear. This preserves the active-scope distinction accepted by Decisions 0003 and 0038.
- The CLI proves reference presence and set consistency only. The design agent still judges whether the referenced sections substantively satisfy each Requirement.

## Template behavior

- A managed `SpecBind Design` template must omit `requirement_ids`. Presence is a template-source validation error even when the value is an empty array.
- The template may contain a `specbind:instruction` comment telling the authoring agent to add the non-empty Front Matter mapping and matching body markers.
- Materialized output must satisfy the complete live profile before it is written. Empty arrays, placeholder IDs, and leaked instruction comments are invalid.

## Fingerprints and freshness

- Design-gate evidence continues to fingerprint the complete Markdown file after line-ending normalization under Decisions 0038 and 0057.
- Consequently, `requirement_ids` order and unrecognized Front Matter fields have no additional design-profile semantics, but adding, removing, reordering, or reformatting them still changes the complete-file fingerprint and invalidates existing design approval.
- The design gate does not duplicate the active Requirement ID list or requirements fingerprint in its evidence. It requires fresh prerequisite requirements evidence, validates current traceability, and records the current contract and complete design logical-key set as already accepted by Decision 0038.

## Consequences

- Humans and agents can see Requirement coverage in the design body, while the CLI receives an inexpensive typed mapping from Front Matter.
- Splitting design across files remains supported and each file states its own scope explicitly.
- Projects can attach local metadata without forking the core profile, while the three required fields remain the only stable v1 Front Matter contract for bundled workflows.
- Exact set equality prevents Front Matter from becoming a stale index that disagrees with visible design content.
- Reusing the narrow italic marker keeps ordinary prose and examples from accidentally satisfying traceability checks.

## Implementation status

The Rust Design parser now recognizes only complete plain-text Markdown emphasis nodes matching the exact `Requirements: N.M, ...` grammar, unions repeated markers, and validates bidirectional set equality with the artifact's structurally valid Front Matter `requirement_ids`. Bare text, strong emphasis, code, comments, partial emphasis, and nested inline constructs do not create references. Discovery reports missing markers and both mismatch directions with complete document line numbers. Checking those IDs against the current Requirements artifact and checking collection-wide active-scope coverage remain cross-artifact validation work.
