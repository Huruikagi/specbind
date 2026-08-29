# 0155: Replace the Contract Markdown DSL with a versioned YAML artifact

Status: Accepted

## Context

[Decision 0056](./0056-canonical-contract-markdown.md) made the persistent
Contract an OKF Markdown concept whose body is a deliberately strict DSL: five
fixed headings, one flat list per section, stable entry IDs, and exact inline
punctuation for descriptions, targets, and paths. The Rust parser already turns
that syntax into a closed typed model before any graph operation can use it.

[Issue #3](https://github.com/Huruikagi/specbind/issues/3) asked whether
non-structural prose should be allowed in that body. Later review contracts make
that relaxation undesirable. A current promise belongs in an entry description
or Invariant, rationale that is not contract meaning belongs in Design, and an
unresolved Contract-review finding cannot be parked behind a passing review.
Allowing ignored prose would therefore add a second, non-authoritative lane
inside the graph source without giving it a valid lifecycle role.

Once arbitrary body prose is excluded, Markdown no longer provides a useful
authoring advantage. It instead requires a custom syntax tree parser, makes
punctuation part of the structural contract, and makes harmless presentation
changes stale because Decision 0038 fingerprints the whole file. SpecBind now
has an established structured-artifact boundary: restricted YAML, a versioned
Rust wire model, generated JSON Schema, artifact-local domain validation, and
normalized typed fingerprints.

## Decision

### Identity and loading

- Every persistent Spec has exactly one structured Contract at the fixed path
  `<specDir>/specs/<canonical-spec>/contract.yaml`.
- The Contract is not an OKF Markdown concept, has no Front Matter `type`, and
  is not discovered by scanning or by a project-selected filename. Its logical
  selector remains `contract`.
- The file is loaded through the same restricted-YAML, generated-schema, wire,
  and domain-validation layers as other structured artifacts. Anchors, aliases,
  tags, merge keys, duplicate keys, multiple documents, and non-JSON values are
  rejected at the shared YAML boundary.
- `schema_version: 1` selects the public `contract/v1` Schema. The root and
  every entry object are closed; unknown fields are rejected.
- `contract.md` is no longer a Contract locator and is not accepted as a
  compatibility representation. Existing projects convert it before using the
  new executable. Installation refresh does not rewrite persistent Spec
  knowledge.

### Version 1 shape

The root contains exactly these required fields:

```yaml
schema_version: 1
owns: []
exports: []
consumes: []
invariants: []
file_ownership: []
```

- `owns`, `exports`, and `invariants` contain `{ id, description }` objects.
- `consumes` contains `{ id, target, description? }` objects. `target` contains
  exactly `{ spec, section, id }`; `section` is `owns`, `exports`, `invariants`,
  or `file-ownership`. A Consumes entry is never a target.
- `file_ownership` contains `{ id, paths }` objects with one or more paths.
- Entry IDs retain Decision 0056's lowercase kebab-case grammar and remain
  unique within their array. Stable-ID continuation, split, and replacement
  rules remain unchanged.
- Descriptions are semantic Contract text. They are non-empty and may use the
  project artifact language. A current constraint another Spec may rely on is
  expressed there or as an Invariant, not as adjacent free-form prose.
- File Ownership retains Decision 0056's exact-path and terminal-`/**` grammar,
  project-root-relative POSIX interpretation, ASCII-case-insensitive duplicate
  detection, sparse inclusion test, and separation from Task boundaries.
- YAML comments have no Contract semantics and are excluded from the typed
  projection. They must not carry a promise, rationale required to understand a
  seam, or future work. Non-contract design rationale belongs in Design.

### Fingerprint and comparison

- Design evidence and Contract-review inputs fingerprint the complete validated
  typed Contract projection using JCS and the existing tagged SHA-256 format.
- Root mapping order, YAML presentation, comments, entry-array order, and path
  order do not change the fingerprint. Entry arrays are normalized by ID and
  File Ownership paths are normalized by value before JCS serialization.
- IDs, descriptions, targets, paths, field presence, and `schema_version` remain
  fingerprinted meaning. A description edit is a seam edit even when graph
  topology is unchanged.
- Contract review reads the historical before-state from the same fixed
  `<specDir>/specs/<spec>/contract.yaml` path at the milestone baseline. A
  missing historical path means the Spec was new or its required Contract was
  absent; the skill does not search for a renamed Markdown concept.

### Authoring surface

- `specbind schema read contract/v1` exposes the exact authoring contract.
- The Spec template selector remains `contract`. Its embedded default and an
  optional project override materialize as `contract.yaml` and contain the
  explicit empty v1 shape.
- The Contract template remains outside the installed project-owned default
  template set because its structure is fixed. A project override may choose
  initial semantic entries but cannot extend the wire shape.
- `artifact list`, `artifact read <spec> contract`, `check contracts`, Design
  approval, and Contract review retain their existing logical selectors and
  graph responsibilities while operating on the structured artifact.

## Consequences

- Decision 0056 is superseded. Its semantic inclusion, stable-ID, target, and
  path rules continue only where restated here.
- Decision 0057 no longer applies type-based Markdown discovery to Contracts.
  Requirements, Design, Brief, Research, and Implementation Notes remain
  type-discovered OKF concepts.
- Decision 0038's complete raw-Markdown Contract fingerprint and Decision
  0055's complete-Markdown review fingerprint are replaced by the normalized
  typed Contract fingerprint. Their logical input keys and freshness chains do
  not change.
- Decision 0091's `contract.md` embedded scaffold becomes `contract.yaml`; its
  exclusion from installed project-owned defaults remains unchanged.
- Agents author a public data contract rather than reproducing punctuation
  conventions, and the CLI rejects structural drift through a generated Schema.
- Formatting-only YAML edits no longer invalidate Design or Contract review;
  semantic edits still do.

## Implementation status

Implemented. `schema::contract::v1` is the structural source of truth,
`schemas/contract/v1.schema.json` is generated from it, `domain::contract`
owns artifact-local semantics, fixed-path discovery loads `contract.yaml`, and
the Contract graph and lifecycle fingerprints consume the validated domain
model. Embedded templates, product-managed Skills, forward-test fixtures, and
Rust conformance and behavior tests use the YAML contract.
