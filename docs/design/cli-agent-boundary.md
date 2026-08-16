# CLI and agent responsibility boundary

This document defines the working boundary between deterministic SpecBind CLI behavior and AI-agent skills. It adapts the proposal from [pc-build-planner Issue #49](https://github.com/Huruikagi/pc-build-planner/issues/49) to SpecBind as an independent product.

Status: Draft

## Direction

SpecBind should ship both:

- agent-facing skills and templates for interpretation, authoring, review, and orchestration
- a deterministic CLI for parsing, invariant checks, and safe lifecycle state changes

The CLI is part of the SpecBind repository and distribution. The Issue #49 plan to publish a separate Rust `spec-lint` repository is therefore no longer the target integration model. Under [Decision 0006](./decisions/0006-rust-cli.md), the existing SpecBind installer and future deterministic operations will be implemented together as the Rust `specbind` CLI.

All command families resolve the same configurable specification root. New projects default to `.specbind`; see [Decision 0007](./decisions/0007-spec-root.md).

Versioned structural contracts for `spec.yaml` and `tasks.yaml` live under `tools/specbind/schemas/` and are packaged with the CLI under [Decision 0015](./decisions/0015-runtime-schema-layout.md). YAML parsing, JSON Schema validation, and semantic invariant checks remain distinct validation layers.

## Why this boundary exists

Repeated grep, PowerShell, or shell-specific inspection consumes agent context and produces inconsistent diagnostics for questions that have deterministic answers. Moving those operations into the CLI provides:

- stable parsing and validation rules
- short human-readable output
- stable result codes and exit behavior for agents and CI
- stable exit semantics
- one implementation shared by every supported agent
- version alignment between installed templates and the rules they invoke

Under [Decision 0067](./decisions/0067-text-first-english-cli-results.md), non-raw commands return an explicit concise English `OK`, `NO_CHANGE`, or `ERROR` outcome with a stable code. Decision 0074 makes text the sole non-raw v1 result surface; agent skills consume it directly and translate or explain results for the user when needed.

The goal is not to replace agent judgment. It is to remove mechanical work from prompts so the agent can focus on meaning and decisions.

## Responsibility model

| Layer | Owns | Does not own |
| --- | --- | --- |
| Agent skills | Interpret user intent, choose a workflow, author prose, review meaning, explain failures, and obtain approval where needed. | Reimplement deterministic parsers or infer lifecycle state from ad hoc searches. |
| SpecBind CLI | Parse owned formats, check identifiers and references, enforce lifecycle invariants, and perform explicit idempotent state mutations. | Decide whether requirements or design are substantively correct, or silently choose product scope. |
| Project release adapter | Supply project-specific Prepare, Publish, Verify, and optional After finalize instructions. | Weaken SpecBind core gates or redefine artifact lifecycle. |

A skill may orchestrate a CLI operation, but the operation's contract belongs to the CLI rather than being duplicated in each agent template.

Approval authority is also distinct from process interaction. Under [Decision 0012](./decisions/0012-delegated-approval.md), `--non-interactive` suppresses prompts but does not approve a gate. Explicit and delegated approvals both pass through the same guarded CLI event and carry revision-bound evidence.

For release, the agent executes the adapter's natural-language project instructions and judges their result with the human. It supplies per-spec log summaries only when the milestone contains Spec-backed work; Direct-only milestones have no per-spec log update. The CLI derives core readiness from existing lifecycle artifacts, owns preflight and finalization, and never executes adapter Markdown as an unrestricted hook; see [Decisions 0010](./decisions/0010-release-execution-boundary.md), [0070](./decisions/0070-derived-release-readiness.md), and [0081](./decisions/0081-v1-release-git-path-and-cli-safety.md).

## First deterministic check: requirement traceability

Issue #49 proposed checking mappings across the inherited fixed files. Under Decision 0057, the equivalent logical artifact flow is:

```text
SpecBind Requirements singleton
  -> SpecBind Design collection
  -> tasks.yaml
```

That proposal predates the accepted active-requirement-set model in [Decision 0003](./decisions/0003-active-requirement-set.md). The SpecBind version should therefore distinguish the complete requirement catalog from the active milestone scope.

The first check should mechanically verify the Decision 0060 requirements grammar and then:

- canonical Requirement IDs can be extracted from the discovered requirements artifact
- Requirement IDs use the supported format and are unique
- every ID in `spec.yaml.active_change.requirement_ids` exists in the requirements artifact
- the active Requirement ID set is established before downstream coverage is claimed
- every design artifact's non-empty Front Matter `requirement_ids` set exactly matches the union of its Decision 0061 italic `_Requirements: ..._` body markers
- every design mapping references current canonical Requirement IDs, and the complete discovered design set traces every active Requirement ID
- `tasks.yaml` maps every active Requirement ID through its schema-defined requirement references
- design and task mappings do not reference unknown Requirement IDs
- task requirement mappings use only the supported canonical syntax

Requirements outside the active set remain valid current requirements, but they do not need to appear in the current milestone's `tasks.yaml`. This differs intentionally from Issue #49's original all-requirements task-coverage rule.

The CLI recognizes only the exact Decision 0061 emphasis-marker grammar rather than scanning arbitrary numeric prose. It verifies that an ID is present in the required mapping. An agent still reviews whether the mapped design and tasks actually satisfy the requirement.

## Cross-spec contract checks

Under [Decisions 0011](./decisions/0011-cross-spec-contract.md), [0055](./decisions/0055-cross-spec-review-inputs.md), [0057](./decisions/0057-type-based-artifact-discovery.md), and [0078](./decisions/0078-contract-first-review-between-design-and-tasks.md), the CLI discovers every current persistent contract by OKF type, validates deterministic structure and the complete dependency graph, and fingerprints the accepted input set. It reports duplicate IDs, unresolved references, missing manifests, and dangling references as errors. Ownership overlaps and dependency cycles are review warnings rather than unconditional structural failures.

The agent remains responsible for deciding whether the manifest describes the real seam, whether a change is semantically compatible, and which downstream specs require deeper review. A CLI graph is evidence and routing input, not a semantic compatibility verdict. The review starts from all current Contracts after every participating Design is approved and before current `tasks.yaml` authoring. When deeper Requirements or Design content materially supports the verdict, the agent declares its logical selector and the CLI resolves and fingerprints it. Task plans and Direct roadmap items are not review inputs.

## Working command shape

The exact command vocabulary is not yet accepted. The initial shape should remain within the existing `specbind` executable, for example:

```sh
specbind check traceability <spec-path>
specbind check contracts [<scope>]
```

Human-readable success output should stay compact:

```text
PASS requirements=24 active=6 design=6 tasks=6
```

Failure output is English-only and contains stable diagnostic codes, affected IDs, and source locations where available:

```text
FAIL
ACTIVE_UNKNOWN: 9.9 at spec.yaml
DESIGN_MISSING: 3.2
TASKS_MISSING: 4.1, 4.2
INVALID_TASK_MAPPING: "Requirement 2.1" at tasks.yaml:18
```

Stable result codes and exit behavior are part of the v1 contract. A JSON response schema is explicitly post-v1 under Decision 0074.

The Rust read model now implements the complete Requirements-to-Design-to-Tasks existence and active-coverage calculation behind this proposed command. It treats absent `tasks.yaml` as normal before the `tasks` state, requires it from `tasks` onward, and reports unknown Task mappings whenever a valid plan exists. The command vocabulary, concise text rendering, and exit-code exposure remain unimplemented.

## Lifecycle automation candidates

The same boundary can prevent `specbind-discovery` from becoming a general-purpose state manager. Candidate CLI command families include:

- capture a clean Decision 0054 Git baseline, generate a branch-safe UUID v7, and create an active roadmap with both its stable milestone ID and baseline revision
- apply an explicitly confirmed roadmap scope update
- mark or reopen a direct roadmap change through its sparse completed-state mutation
- bind the target release through `specbind milestone bind-release <version>`, or replace a non-null binding only through its explicitly confirmed `--rebind` form
- check milestone and per-spec lifecycle consistency
- perform the deterministic portion of confirmed abandonment cleanup
- run the stateless `specbind release preflight` readiness check and idempotent finalization mutations

These are accepted CLI responsibilities under [Decision 0009](./decisions/0009-milestone-cli-boundary.md). Their exact command names remain Draft except for `specbind milestone bind-release`, accepted by Decision 0072, `specbind release preflight`, accepted by Decision 0069, and `specbind release finalize`. Decision 0081 removes the former finalization `--force` bypass. Discovery remains the user-facing entry point for understanding and routing a request, while CLI commands own the resulting mechanical writes. SpecBind does not expose a separate `specbind-milestone` agent skill.

The draft event names, expected states, guards, invalidation effects, and consistency-health model for per-spec mutations are defined in [Spec state machine](./spec-state-machine.md). The aggregate read model and phase-relative dependency waves are defined in [Milestone state machine](./milestone-state-machine.md). Stable CLI commands may rename those events, but must preserve the accepted transition semantics once finalized.

## Integration with skills

Generated SpecBind skills should call the bundled CLI at the phase where its invariant becomes relevant:

```text
CLI mechanical check
  -> agent semantic review
  -> implementation or lifecycle transition
  -> fresh completion evidence
```

For the traceability check, requirements, design, tasks, validation, and release-readiness workflows should consume the same CLI contract instead of embedding agent-specific grep instructions. A standalone validation skill is unnecessary when its only purpose would be to expose one deterministic CLI command.

The stable project-customization surface is shared `{{SPEC_DIR}}/settings/templates/` and `{{SPEC_DIR}}/settings/rules/`; see [Decision 0008](./decisions/0008-customization-surface.md). Generated skills and agent metadata are product-managed resources. The installer replaces clean product-managed assets, never overwrites an existing user-owned settings file, and creates newly introduced defaults when their target is absent. Direct skill modification is not the cross-agent customization contract.

The CLI and skills must respect supported settings customization while still enforcing documented machine-readable structure. A mechanical check reports an incompatible customized format explicitly rather than silently falling back to agent-specific searches.

## Task read model

Decision 0025 accepts `specbind spec status`, `specbind tasks list`, and `specbind tasks show` as read-only CLI projections over `spec.yaml` and `tasks.yaml`. The CLI owns schema validation, consistency health, sparse-status expansion, effective dependency calculation, group rollups, Requirement ID coverage, approval freshness, and concise text rendering. Agent skills own when to request a view, how to explain it in workflow context, and any semantic recommendation that cannot be derived mechanically.

These commands replace routine raw-YAML interpretation but do not create a generated Markdown artifact. V1 exposes one concise text projection; Decision 0074 defers alternate JSON rendering.

## Artifact inventory and content read model

[Decision 0058](./decisions/0058-artifact-inventory-read-model.md) accepts `specbind artifact list <spec>` and `specbind artifact read <spec> <selector>` as the read-only boundary over Decision 0057 type-based discovery. The list command returns a compact deterministic text inventory without bodies or hashes. The read command resolves one logical selector rather than an agent-supplied path and returns untouched Markdown; workflows issue separate reads for multiple bodies in v1.

Agent skills directly read a known singleton or authoritative collection selector. They list first only when they need collection membership, optional-artifact discovery, selector choice, or structural diagnostics. They do not reproduce recursive searches or bind workflow behavior to default filenames. Gate and review mutations independently rediscover and fingerprint current inputs, so list and read outputs never become mutation authority.

## Template read and validation model

[Decision 0059](./decisions/0059-okf-artifact-templates.md) defines a separate `specbind template list/read` family for project-owned scaffolds. Template inventory mirrors logical artifact selectors but reports both source template and derived output paths. Raw reads retain `specbind:instruction` comments for the authoring agent, while materialization removes those nodes and rejects leaks in live artifacts.

The CLI owns template discovery, identity and path validation, collision checks, instruction-node recognition, and non-writing output-tree previews. In v1, maintainers follow customization guidance and edit project-owned settings directly; a dedicated customization skill is post-v1. Ordinary artifact workflows own materialization through their guarded lifecycle operation; template read commands never create or overwrite live artifacts.

## Completion validation handshake

Decision 0029 assigns completion preflight and guarded acceptance to the CLI. Preflight returns only a clean full Git `HEAD`; it does not round-trip authoritative fingerprints through the agent. The validation skill owns execution of project checks and semantic `GO | NO-GO | MANUAL_VERIFY_REQUIRED` synthesis. Only a `GO` candidate is submitted, and the CLI independently recomputes current inputs and rejects it unless the same clean revision, approvals, fresh cross-spec review, and all-completed task state still hold immediately before the `IMPLEMENTATION_VALIDATED` mutation.

The CLI detects the repository's Git object format and validates the scalar full `implementation_revision` under Decision 0031. Generated skills neither submit per-evidence VCS metadata nor infer acceptable hash length themselves.

Decision 0032 makes freshness a gate-local chain. The CLI compares each gate's current direct input projection with that gate's accepted evidence, then derives downstream staleness from prerequisite freshness. Agent workflows may read broader context, but they do not duplicate upstream fingerprints when submitting later-gate evidence.

Under Decision 0030, only successfully accepted evidence is persisted. `NO-GO`, `MANUAL_VERIFY_REQUIRED`, preflight failures, and rejected candidates return diagnostics without a `spec.yaml` mutation or a separate evidence-recording event.

The exact command names remain a follow-up detail; Decision 0037 fixes the accepted structured completion evidence shape. A generated skill must not replace either CLI call with its own `git rev-parse`, status interpretation, or direct `spec.yaml` edit.

For accepted mechanical evidence, Decision 0033 requires an ordered list of categorized, display-safe commands with successful exit codes. The agent discovers and executes the project-appropriate set; the CLI validates its strict shape and rejects non-success entries without storing raw output.

Decision 0034 keeps requirements coverage, design alignment, spec-local task integration, and boundary integrity as mandatory agent judgments but omits their fixed `passed` flags from persisted evidence. The CLI accepts only the final guarded `GO` candidate; it does not mistake stored booleans for replayable semantic proof.

Decisions 0050, 0052, 0053, and 0055 are refined by Decision 0078. The singleton `state/cross-spec-review.md` contains free-form accepted judgment and only `type`, `milestone_id`, `passed_at`, and `input_revisions` in Front Matter. During Tasks approval, completion acceptance, and release readiness, the CLI resolves this global record through the matching milestone ID and current Spec-backed roadmap membership. The agent consumes a CLI summary and does not copy that record into per-spec candidate evidence.

## Initial implementation boundary

The first increment should remain narrow:

- read-only traceability validation for one spec directory
- concise default output
- stable non-zero failure exit behavior
- stable text result codes for agents and CI
- fixtures covering valid mappings, missing coverage, unknown references, duplicates, and invalid syntax
- integration into at least the design and tasks review paths

It should not initially validate task hierarchy, task dependencies, approval semantics beyond the active-set prerequisite, project-specific rules, or the substantive quality of requirements, design, and tasks.

## Open questions

- Final command names for remaining checks and whether `check` becomes their common read-only validation namespace; Decision 0025 fixes the task read-model command names.
- Exact command contracts for the accepted milestone operations and any additional lifecycle candidates.
