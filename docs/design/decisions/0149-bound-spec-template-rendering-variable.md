# 0149: Bind the canonical Spec identity to a template rendering variable

Status: Accepted

## Context

Spec-local Markdown is commonly read through `artifact read`, copied into a
review, or opened outside the surrounding directory tree. The official
scaffolds currently use generic titles such as `# Requirements` and `# Design`,
so that content loses its owning Spec identity when separated from its path.

Decision 0044 deliberately omits a duplicate display name from `spec.yaml` and
places human-facing capability names in prose. Decision 0059 reserved future
deterministic template rendering variables for a separate whitelist and
escaping contract. Allowing arbitrary placeholders now would make the value
source, rendering authority, escaping, and unresolved-output behavior implicit.
An instruction comment alone is also insufficient: the CLI must be able to
prove that every rendering site has explicit creation guidance.

## Decision

- V1 recognizes exactly one Spec-template rendering variable: `{{spec}}`.
- Its value is the validated canonical Spec identity supplied to the CLI. The
  agent does not choose, translate, normalize, or persist a second value.
- Rendering variables are permitted only in the Markdown body of Spec artifact
  templates. Front Matter, `type`, `artifact_id`, output paths, Steering
  templates, and milestone templates remain literal.
- Every used rendering variable has exactly one associated template-only
  instruction whose opening token sequence is
  `specbind:instruction create bind=<variable>`.
- Only `create` may bind a variable. A missing, duplicate, unknown, or unused
  binding is a template diagnostic. The v1 binding grammar binds one variable
  per instruction.
- `template read spec <selector>` remains an exact raw read, including the
  variable and all instruction comments.
- `template render spec <spec> <selector>` validates the complete resolved
  template inventory and canonical Spec identity, substitutes `{{spec}}`, and
  returns raw UTF-8 Markdown without writing a file. It preserves every
  instruction comment byte-for-byte so the authoring agent can follow the
  `create` guidance and copy durable guidance normally.
- A live artifact containing an unresolved recognized rendering variable is
  invalid. Materialization still omits `create` and preserves `maintain` and
  `consume`; the resulting artifact must pass its complete live profile.
- The canonical Spec identity is a lowercase portable token already validated
  by the artifact boundary. V1 performs exact token substitution and introduces
  no arbitrary user value or general Markdown-context escaping.
- The official Brief, Research, Requirements, Design, and Implementation Notes
  scaffolds render the canonical identity in their H1. Contract retains the
  exact machine-readable `# Contract` heading required by Decision 0056.
- Product-managed authoring skills use `template render` whenever they first
  materialize a Spec artifact. They continue using `template read` only when an
  exact unrendered template is intentionally required.

## Deferred complexity

[Issue #10](https://github.com/Huruikagi/specbind/issues/10) owns the post-v1
guided customization experience. It also tracks any later expansion to
project-defined or typed variables, user-supplied values, literal escaping,
context-sensitive rendering, previews, and optional reconciliation of existing
artifacts. Such an expansion must not silently turn a template edit into
authority over live artifact semantics or lifecycle state.

## Consequences

- Default Spec artifact titles remain attributable in raw reads and reviews
  without duplicating identity in structured state.
- Project-owned templates can use the same narrow variable while the CLI
  rejects unbound or unsupported placeholders mechanically.
- Exact raw template inspection and deterministic context rendering are separate
  commands, so callers cannot confuse one for the other.
- A future customization skill has an explicit, validated primitive to preview
  without inheriting an unbounded general-purpose template language.
