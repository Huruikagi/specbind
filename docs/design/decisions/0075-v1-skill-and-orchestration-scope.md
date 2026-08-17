# 0075: Fix the v1 skill and orchestration scope

Status: Accepted

## Context

The inherited cc-sdd skill set mixes product phases, compatibility names, initialization mechanics, and multi-spec implementation orchestration. SpecBind v1 needs a complete agent-facing workflow without committing to a milestone-wide implementation scheduler whose subagent and validation behavior has not yet been designed.

## Decision

- V1 installs these outcome-oriented skills:
  - `specbind-discovery`
  - `specbind-requirements`
  - `specbind-design`
  - `specbind-validate-design`
  - `specbind-cross-spec-review`
  - `specbind-tasks`
  - `specbind-implement`
  - `specbind-review-task`
  - `specbind-validate-implementation`
  - `specbind-verify-completion`
  - `specbind-status`
  - `specbind-quick`
  - `specbind-batch`
  - `specbind-release`
  - `specbind-gap-analysis`
  - `specbind-debug`
  - `specbind-steering`
- The inherited `spec-*` suffix prefix is removed. V1 ships no legacy skill aliases.
- `specbind-spec-init` is removed. After discovery confirms scope, a Rust CLI milestone creation operation initializes the roadmap, briefs, new specs, and active changes coherently.
- `specbind-steering-custom` is merged into `specbind-steering`; the skill bootstraps, synchronizes, or adds guidance based on intent.
- A dedicated customization skill is post-v1. Project maintainers edit the documented settings customization surface directly.
- `specbind-quick` and `specbind-batch` are thin orchestrators over the same artifacts, reviews, approvals, and CLI guards as the deliberate flow. They stop after Tasks approval and never implement code.
- Cross-spec review occurs after every participating Spec has passed its Design gate and entered the `tasks` state, but before any current `tasks.yaml` is authored. It is a contract-first milestone review, not a general Design review or release gate. Decision 0087 routes its focused read and accepted assessment through `specbind milestone review status` and `specbind milestone review accept --candidate <path|->`.
- `specbind-implement` targets exactly one roadmap item per invocation:
  - for a Spec-backed item, it executes the approved local task plan;
  - for a Direct item, it performs the scoped implementation without creating Requirements, Design, Contract, or Tasks artifacts.
- V1 has no milestone-wide implementation orchestrator. Decision 0082 and the milestone state machine define phase-relative dependency waves as a CLI read model that per-item skills can follow without adding an orchestration skill.
- `specbind-status` is one read-only explanation skill over explicit CLI entities. With no argument it calls `specbind milestone status`; with a canonical Spec identity it calls `specbind spec status <spec>`; task-level questions drill into `specbind tasks list/show`. It does not recreate the inherited behavior by scanning every persistent Spec as an implicit active scope.
- Direct implementation is valid only while the change requires no canonical Requirements, Design, or Contract change. Discovery must reroute a Direct item when that premise fails.
- The default task-review mode is `required` for Spec-backed implementation and `inline` for Direct implementation. `required`, `inline`, and `off` are run-scoped choices; `off` never disables final implementation validation or completion verification.
- `specbind-debug` is a read-only, fresh-context root-cause protocol. It returns a run-scoped diagnosis and next action; a new implementer applies any fix.
- Automatic task debug/remediation and automatic cross-spec-review remediation each stop after two rounds. Unresolved work remains blocked or returned to the appropriate earlier phase.

[Decision 0096](./0096-skill-asset-layout.md) fixes how these skills are authored, stored, and rendered per agent.

## Consequences

- Skill names describe user outcomes rather than inherited cc-sdd command families.
- Direct work has an explicit implementation owner without adding another public skill.
- Quick and batch improve throughput without defining weaker lifecycle semantics.
- A later milestone orchestrator can coordinate multiple Spec and Direct items without changing the v1 per-item implementation contract.
