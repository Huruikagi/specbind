# 0056: Use canonical Markdown for contract manifests

Status: Accepted

## Context

`contract.md` must remain concise and readable by humans and agents while the CLI parses it into the deterministic cross-spec graph required by Decisions 0011 and 0055. Putting the semantic contract in YAML frontmatter would split authoring between two representations and diverge from the structured-Markdown direction also needed for `requirements.md`.

File Ownership also needs a stable identity independent of its current path. Treating every repository file as a contract entry would turn the manifest into a brittle source-tree inventory and make ordinary refactoring create cross-spec noise.

## Decision

- `contract.md` is an OKF concept document with the exact required frontmatter field `type: SpecBind Contract`. Unknown OKF extension fields remain allowed and preserved under Decision 0045, but they do not contain the v1 semantic contract.
- The semantic contract is parsed from the Markdown syntax tree, not by regular expressions or presentation-dependent line scanning.
- The document contains one `# Contract` heading followed by exactly one of each required level-two section in this order:
  1. `## Owns`
  2. `## Exports`
  3. `## Consumes`
  4. `## Invariants`
  5. `## File Ownership`
- Structural headings are canonical English tokens. Entry descriptions may use either supported product language.
- Each section contains only a flat unordered list. An intentionally empty section has no list items; placeholder prose such as `None` is not used.
- Every entry begins with an inline-code stable ID matching `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$`. IDs are unique within their section and do not derive from list position.
- `Owns`, `Exports`, and `Invariants` use `- \`<id>\` — <non-empty description>`.
- `Consumes` uses `- \`<local-id>\` → \`<canonical-spec>/<target-section>/<target-id>\`` and may append ` — <description>`. The target section token is the canonical lowercase kebab-case form of a contract section.
- `File Ownership` uses `- \`<id>\` — \`<path-pattern>\``. Further inline-code path patterns may follow, separated by a comma and one space. Paths are values of the stable entry, not its identity.
- File Ownership path patterns are repository-root-relative POSIX paths. Absolute paths and `.` or `..` segments are invalid. Exact file paths and supported glob patterns are allowed; exact glob semantics are specified with CLI validation rather than inferred from a host shell.
- A stable entry ID remains unchanged across reordering, description edits, and path moves when its semantic boundary remains the same. A semantic replacement receives a new ID. When a boundary splits, the continuing meaning may retain the old ID and newly introduced meanings receive new IDs.
- File Ownership is a sparse declaration of persistent boundaries where a change or conflicting write could affect another spec's design or verification. It includes important shared files, ambiguous responsibility boundaries, public schemas or types, routing and migration boundaries, and generated-output boundaries when cross-spec coordination matters.
- File Ownership is not a complete inventory. Private implementation files, ordinary fixtures, temporary outputs, every file touched by a task, and refactoring-only paths are omitted unless they independently meet the cross-spec inclusion test.
- A path absent from File Ownership is merely outside the persistent cross-spec graph. Absence does not assert that the path is unowned or freely writable.
- Milestone-local concrete write scope remains in `tasks.yaml` task `boundaries`; it is not copied into the persistent contract merely because a task touches those files.

## Canonical example

```markdown
---
type: SpecBind Contract
---
# Contract

## Owns

- `compatibility-evaluation` — Evaluates compatibility between selected parts.

## Exports

- `compatibility-result` — Result consumed by build presentation.

## Consumes

- `part-type` → `part-catalog/exports/part-type`

## Invariants

- `no-selection-mutation` — Compatibility evaluation never mutates selected parts.

## File Ownership

- `compatibility-domain` — `src/domain/compatibility/**`
```

The canonical empty contract retains all five headings with no list items.

## Consequences

- Humans and agents edit one visible contract representation, while the CLI can build a typed graph from a deliberately narrow Markdown profile.
- Reordering and refactoring do not change entry identity by themselves.
- Contract review stays focused on cross-spec seams instead of repository coverage.
- Project-customized templates may change explanatory prose but must preserve the canonical structural tokens and entry grammar.
- Intentional shared ownership, generated-file precedence, and exact overlap resolution remain separate policy decisions; this decision only defines when such a boundary belongs in the manifest and how it is identified.
