# 0059: Use final-form OKF documents as artifact templates

Status: Accepted

Decision 0152 supersedes this Decision's rule that every discovered Design
template is materialized as the initial decomposition. The inventory is now the
candidate set; the project-owned selection Rule chooses required and applicable
conditional entries.

## Context

Decision 0008 makes shared templates and rules the project-owned customization surface. Decisions 0057 and 0058 then decouple live artifact identity from filenames and give agents logical inventory and read operations. The inherited cc-sdd templates still mix final Markdown structure, placeholder examples, and natural-language generation instructions without a machine-recognizable boundary.

SpecBind needs customizable template sets that can create several design artifacts, choose their initial paths, and remain readable by AI without introducing a second manifest or letting template-only instructions leak into authoritative artifacts.

## Decision

- A managed Markdown file below `{{SPEC_DIR}}/settings/templates/specs/` is a final-form OKF artifact template. Its Front Matter uses the intended artifact's exact OKF identity fields; there is no separate `SpecBind Template` wrapper type. Its body is a final-form scaffold plus the template-only instruction extension defined below, not an already complete approved artifact.
- The template's path relative to `settings/templates/specs/` is its output path relative to the target spec directory. V1 has no separate `output_path` field.
- Relative output paths must remain inside the target spec directory after normalization. Absolute paths, empty segments, `.` or `..` segments, symbolic-link escape, and collisions between normalized outputs are invalid.
- Template Front Matter contains literal machine identity:
  - singleton templates contain their exact `type` and omit `artifact_id`
  - collection templates contain their exact `type` and literal stable `artifact_id`
- `type`, `artifact_id`, and output paths contain no AI instructions or template variables. AI does not choose or rewrite these values during ordinary materialization.
- The template set obeys the Decision 0057 profile multiplicity and ID rules. In particular, duplicate singleton types and duplicate collection `artifact_id` values are invalid.
- The optional `SpecBind Research` singleton has a conventional `research.md` template. `gap-analysis` materializes it only when research is useful; ordinary authoring workflows do not create it as mandatory ceremony.
- Under Decision 0061, a `SpecBind Design` template omits the live-only `requirement_ids` field; its presence is invalid even as an empty array. An instruction comment may direct the authoring agent to add the non-empty mapping and matching italic body markers before materialization. Other valid project metadata may appear in template Front Matter and is copied into the materialized artifact without acquiring SpecBind semantics.
- Template-relative placement is user customization. For example, `settings/templates/specs/technical-design/persistence.md` initially materializes as `specs/<spec>/technical-design/persistence.md`.

## AI instruction comments

- [Decision 0139](./0139-scoped-artifact-instructions.md) refines the original
  template-only directive into required `create`, `maintain`, and `consume`
  scopes. The rules below establish the original syntax-tree and authority
  boundary; Decision 0139 governs persistence and read projection.
- A Markdown HTML comment whose trimmed content begins with the exact token `specbind:instruction` is scoped natural-language guidance for the authoring or consuming agent.
- Both single-line and multiline forms are allowed:

  ```markdown
  <!-- specbind:instruction maintain Summarize the responsibility in one paragraph. -->

  <!-- specbind:instruction create
  Describe only decisions owned by this design artifact.
  Remove this section when the concern does not apply.
  -->
  ```

- The directive is recognized from the Markdown syntax tree as a complete HTML comment node. Prefix-like text inside code fences, inline code, ordinary prose, or a different HTML comment is not an instruction.
- A template read includes every scope. Materialization omits `create` and carries
  `maintain` and `consume` into the live artifact under Decision 0139. The
  `okf-authoring` protocol accepted by [Decision 0094](./0094-embedded-product-protocols.md) states this before the first write.
- Template-source validation checks OKF structure, literal identity, path safety, multiplicity, directive syntax and scope, and every target-profile invariant that can hold before authoring. The authoring operation removes `create`, preserves durable instruction nodes, and validates the completed output against the full live-artifact profile before writing it.
- Live-artifact validation reports a leak only for `create`; valid `maintain` and
  `consume` comments are durable guidance. Ordinary HTML comments remain ordinary content and are not stripped or rejected by this rule.
- The CLI identifies, exposes, projects, masks for semantic parsing, and diagnoses instruction nodes but does not interpret whether their natural-language guidance is substantively correct.
- Required headings, IDs, mappings, and other machine contracts are never defined only through an instruction comment. CLI validation and artifact profiles remain authoritative.

## Template and rule boundary

Decision 0092 refines this boundary by separating user-owned scaffold guidance and shared policy from non-waivable product-managed skill obligations and deterministic CLI contracts.

- A template contains:
  - final OKF Front Matter
  - final-form Markdown headings and durable scaffold content
  - artifact-specific `specbind:instruction` comments
- Shared rules contain cross-template authoring principles, review criteria, and naming policy for additional artifacts not present in the initial template set.
- Skills own workflow ordering and semantic authoring. They do not copy inherited prose placeholders such as `[Describe ...]` or `2-3 paragraphs max` into output; those instructions migrate to explicit directive comments or shared rules.
- Deterministic CLI rendering variables, if introduced later, require a separate whitelist and escaping contract. V1 does not overload cc-sdd-style `{{...}}` text as both an AI placeholder and a renderer variable.

## Lifecycle

- Templates scaffold an artifact only when the owning workflow first creates that artifact or explicitly performs a user-confirmed scaffold/migration operation.
- Existing live artifacts are not continuously synchronized with template edits. Adding, removing, renaming, or editing a template affects new materialization but does not silently add, delete, move, or overwrite an existing spec artifact.
- New-spec design creation may materialize several `SpecBind Design` templates in one guarded operation. The resulting complete design set is subsequently maintained through live artifact discovery rather than template reconciliation.
- Installed templates remain user-owned and update-safe under Decision 0008. Product updates never silently replace project modifications.

## CLI and agent access

- [Decision 0091](./0091-installed-template-surface.md) narrows which of these templates `specbind install` writes into the project customization surface; every type stays available as an embedded scaffold.
- Templates use a separate read-only command family so agents cannot confuse a scaffold with current authoritative state:

  ```text
  specbind template list spec
  specbind template read spec <selector>
  ```

- Template selectors use the same singleton and collection forms as their intended live artifacts. Template inventory includes `selector`, `type`, conditional `artifact_id`, `template_path`, and derived `output_path`, but no body or fingerprint.
- Direct reads of known selectors and list-then-read collection discovery follow the Decision 0058 behavior. A raw read includes instruction comments and accepts one selector; workflows issue separate reads for multiple templates in v1.
- Artifact authoring and lifecycle operations independently validate the current template set and materialize output. Read commands never write live artifacts, and template-read results are not mutation authority.

## Implementation status

The Rust CLI exposes the accepted read-only `template list spec` and `template read spec <selector>` commands over the project-owned `settings/templates/specs/` tree, with the official defaults embedded in the binary answering every selector the project does not override. Discovery recognizes templates by OKF type, derives the live-artifact selector form, reports `selector`, `type`, conditional `artifact_id`, `template_path`, and derived `output_path` without a body or fingerprint, and enforces the template-only profile rules including the forbidden `requirement_ids` mapping on a Design template. Output paths that would escape the target Spec directory, duplicate selectors, symbolic links, and unreadable or non-UTF-8 templates are reported as diagnostics. A raw read returns the template unchanged with its `specbind:instruction` comments intact. An absent template tree is normal because the embedded defaults still resolve; each inventory entry reports whether it came from the project or the binary. The binary embeds one template per artifact type for both supported languages, and every materialized default is a recognized live artifact apart from the Design `requirement_ids` mapping that the authoring agent adds.

## Consequences

- A project can customize filenames, directories, document structure, design decomposition, and artifact-specific AI guidance using one visible template tree.
- Template sources remain valid OKF concept documents while template-only comments have an explicit removal and leak-detection contract.
- Machine identity and output paths remain deterministic instead of depending on AI interpretation.
- Existing specs remain stable when project defaults evolve.
- V1 can document manual customization and provide deterministic CLI validation. An interactive customization skill remains an optional post-v1 convenience rather than part of this decision's delivery scope.
