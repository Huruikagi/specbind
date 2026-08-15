# 0046: Use grouped roadmap work items

Status: Accepted

Decision 0082 fixes how the single Roadmap DAG is projected into Design and Implementation waves. Wave numbers and aggregate progress remain derived.

## Context

The inherited roadmap mixed free-form milestone context with a Markdown checklist that grouped new specs, existing-spec updates, and direct implementation candidates. SpecBind needs the same scope categories and dependency information for deterministic membership checks, dependency-wave calculation, status rendering, and release preflight, but it should not require the CLI to parse or partially rewrite prose.

Flattening every item into one list with a repeated `kind` field would also obscure the three user-facing routes that discovery already distinguishes.

## Decision

- A `SpecBind Roadmap` frontmatter requires a non-empty `work_items` mapping.
- `work_items` may contain these category keys:
  - `new_specs`
  - `spec_updates`
  - `direct_changes`
- A category is present only when it contains at least one item. Empty category arrays are invalid, and at least one category must be present.
- A `new_specs` or `spec_updates` item requires exactly:
  - `spec`: the canonical spec identity under Decision 0041
  - `summary`: a non-empty one-line description
  - optional non-empty `depends_on`
- A `direct_changes` item requires exactly:
  - `id`: a non-empty roadmap-local identity
  - `summary`: a non-empty one-line description
  - optional non-empty `depends_on`
  - optional `status: completed` under Decision 0047
- A dependency is a typed single-key reference to exactly one roadmap item:
  - `{ spec: <canonical spec identity> }` refers to either a new-spec or spec-update item
  - `{ direct: <roadmap-local identity> }` refers to a direct-change item
- Every dependency target must exist in the same roadmap. Duplicate references, self-dependencies, and dependency cycles are invalid.
- A canonical spec identity appears at most once across `new_specs` and `spec_updates`. Direct-change IDs are unique within `direct_changes`.
- Dependency relationships, not YAML list order, determine execution waves. List and category order are presentation order only.
- The frontmatter stores no generic completion checkbox or roadmap-level status. Spec-backed progress is derived from the spec lifecycle and task execution artifacts. Direct changes use only the sparse completed state accepted by Decision 0047.
- The Markdown body has no CLI-parsed schema and is never partially mutated by the CLI. Templates may recommend Overview, Approach Decision, Scope, Constraints, and Boundary Strategy sections for human and agent context.
- The roadmap owns the review scope but stores no global review evidence in frontmatter. Decisions 0050 and 0052 place the single accepted record in a dedicated project-state artifact.

## Consequences

```markdown
---
type: SpecBind Roadmap
milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
baseline_revision: 0123456789abcdef0123456789abcdef01234567
target_release: null
work_items:
  new_specs:
    - spec: account-auth
      summary: Add the authentication foundation
  spec_updates:
    - spec: checkout
      summary: Require authenticated checkout
      depends_on:
        - spec: account-auth
  direct_changes:
    - id: update-ci
      summary: Add the new validation command to CI
      depends_on:
        - spec: checkout
      status: completed
---

# Roadmap

## Overview

The milestone-wide intent and chosen approach.
```

- The category itself carries the route, so items need no repeated `kind` field.
- Typed dependency references avoid collisions between canonical spec identities and roadmap-local direct IDs.
- The CLI can flatten the three arrays internally for graph operations while preserving the roadmap's user-facing hierarchy.
- The roadmap body can evolve with project needs without changing the CLI parser.
