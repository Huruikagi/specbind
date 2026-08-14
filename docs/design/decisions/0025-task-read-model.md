# 0025: Make CLI projections the primary task-reading surface

Status: Accepted

## Context

`tasks.yaml` improves deterministic validation and updates but is less convenient than the inherited Markdown checklist for routine human review. Agent workflows also should not repeatedly reconstruct group progress, implicit dependencies, pending state, blockers, and gate freshness by interpreting raw YAML.

Maintaining a generated `tasks.md` view would reintroduce dual-write and drift problems rejected by Decision 0013. The bundled CLI can instead compute read-only projections from the canonical structured artifacts.

## Decision

### Command surface

The target CLI provides these read-only commands:

- `specbind spec status <spec>` — concise lifecycle, consistency, gate, coverage, and task-progress summary.
- `specbind tasks list <spec>` — ordered group/task view with derived progress and actionable work.
- `specbind tasks show <spec> <task-id>` — one task's full plan content, derived status, dependencies, blocker, and completion criteria.

The exact spec locator grammar and optional filtering flags remain follow-up CLI details; these command responsibilities and names are accepted.

### Human output

- Default output is concise, readable terminal text rather than serialized YAML.
- `spec status` includes declared lifecycle state, derived consistency health, completed/pending/blocked counts, next actionable task or tasks, blockers, Requirement ID coverage, and task-plan approval freshness.
- `tasks list` preserves plan order and hierarchy, derives group progress, and distinguishes pending, completed, and blocked tasks.
- `tasks show` displays title, details, Requirement IDs, boundaries, contracts, explicit and effective prerequisites, blocker state, and explicit completion criteria when present.
- Derived display labels such as partial group progress do not create new persisted task statuses.

### Structured output

- Each command supports `--json` with the same underlying semantics as the default concise English text output under Decision 0067.
- JSON keys are stable English machine keys, contain no ANSI formatting, and are intended for structured agent processing and CI rather than routine skill calls.
- The JSON response schema and diagnostic/exit-code contract must be versioned before implementation is considered complete.
- Generated skills use the structured CLI output when they need computed state; raw YAML remains available for authoring and narrowly scoped edits.

### Validation and ownership

- Read commands never mutate project artifacts.
- They validate the relevant schemas and semantic invariants before presenting derived conclusions.
- `spec status` may report declared state plus `inconsistent` health and diagnostics; it must not silently repair contradictions.
- Task list/show operations fail clearly when corruption prevents a trustworthy projection.
- The CLI derives pending tasks from sparse execution state, spec-local effective dependencies from Decisions 0019 and 0027, group progress, current Requirement ID coverage, and approval freshness.
- No command generates or maintains a parallel `tasks.md` artifact.
- Task commands do not parse or embed the free-form implementation-notes artifacts accepted by Decisions 0026 and 0057; semantic agent workflows ask the CLI to discover and read them when needed.

## Consequences

- Humans regain a checklist-like view without sacrificing a single structured source of truth.
- Agents and CI share deterministic calculations instead of embedding agent-specific YAML interpretation.
- `specbind-spec-status` becomes orchestration and explanation over the CLI read model rather than a competing parser.
- CLI compatibility now includes human rendering and versioned JSON projections as well as artifact schemas.

## Open questions

- Versioned JSON response schemas and stable exit-code categories.
- Exact command-specific success and no-change codes within the Decision 0067 result contract.
- Filters such as blocked-only, actionable-only, or group selection.
