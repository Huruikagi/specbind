# CLI and agent responsibility boundary

This document defines the working boundary between deterministic SpecBind CLI behavior and AI-agent skills. It adapts the proposal from [pc-build-planner Issue #49](https://github.com/Huruikagi/pc-build-planner/issues/49) to SpecBind as an independent product.

Status: Draft

## Direction

SpecBind should ship both:

- agent-facing skills and templates for interpretation, authoring, review, and orchestration
- a deterministic CLI for parsing, invariant checks, and safe lifecycle state changes

The CLI is part of the SpecBind repository and distribution. The Issue #49 plan to publish a separate Rust `spec-lint` repository is therefore no longer the target integration model. Under [Decision 0006](./decisions/0006-rust-cli.md), the existing SpecBind installer and future deterministic operations will be implemented together as the Rust `specbind` CLI.

All command families resolve the same configurable specification root. New projects default to `.specbind`; see [Decision 0007](./decisions/0007-spec-root.md).

## Why this boundary exists

Repeated grep, PowerShell, or shell-specific inspection consumes agent context and produces inconsistent diagnostics for questions that have deterministic answers. Moving those operations into the CLI provides:

- stable parsing and validation rules
- short human-readable output
- machine-readable JSON for agents and CI
- stable exit semantics
- one implementation shared by every supported agent
- version alignment between installed templates and the rules they invoke

The goal is not to replace agent judgment. It is to remove mechanical work from prompts so the agent can focus on meaning and decisions.

## Responsibility model

| Layer | Owns | Does not own |
| --- | --- | --- |
| Agent skills | Interpret user intent, choose a workflow, author prose, review meaning, explain failures, and obtain approval where needed. | Reimplement deterministic parsers or infer lifecycle state from ad hoc searches. |
| SpecBind CLI | Parse owned formats, check identifiers and references, enforce lifecycle invariants, and perform explicit idempotent state mutations. | Decide whether requirements or design are substantively correct, or silently choose product scope. |
| Project release adapter | Supply project-specific Prepare, Publish, Verify, and optional After finalize instructions. | Weaken SpecBind core gates or redefine artifact lifecycle. |

A skill may orchestrate a CLI operation, but the operation's contract belongs to the CLI rather than being duplicated in each agent template.

For release, the agent executes the adapter's natural-language project instructions and supplies structured evidence to the CLI. The CLI owns preflight and finalization and never executes adapter Markdown as an unrestricted hook; see [Decision 0010](./decisions/0010-release-execution-boundary.md).

## First deterministic check: requirement traceability

Issue #49 proposed checking mappings across:

```text
requirements.md
  -> design.md
  -> tasks.md
```

That proposal predates the accepted active-requirement-set model in [Decision 0003](./decisions/0003-active-requirement-set.md). The SpecBind version should therefore distinguish the complete requirement catalog from the active milestone scope.

The first check should mechanically verify:

- canonical Requirement IDs can be extracted from `requirements.md`
- Requirement IDs use the supported format and are unique
- every ID in `spec.json.active_change.requirement_ids` exists in `requirements.md`
- the active Requirement ID set is established before downstream coverage is claimed
- `design.md` traces every active Requirement ID
- `tasks.md` maps every active Requirement ID through its machine-readable requirement references
- design and task mappings do not reference unknown Requirement IDs
- task requirement mappings use only the supported canonical syntax

Requirements outside the active set remain valid current requirements, but they do not need to appear in the current milestone's `tasks.md`. This differs intentionally from Issue #49's original all-requirements task-coverage rule.

The CLI verifies that an ID is present in the required mapping. An agent still reviews whether the mapped design and tasks actually satisfy the requirement.

## Cross-spec contract checks

Under [Decision 0011](./decisions/0011-cross-spec-contract.md), the CLI also validates the deterministic structure of `contract.md` files and their dependency graph. It can report duplicate IDs, unresolved references, ownership overlap candidates, prohibited cycles, missing manifests, and structural diffs against a released reference.

The agent remains responsible for deciding whether the manifest describes the real seam, whether a change is semantically compatible, and which downstream specs require deeper review. A CLI graph is evidence and routing input, not a semantic compatibility verdict.

## Working command shape

The exact command vocabulary is not yet accepted. The initial shape should remain within the existing `specbind` executable, for example:

```sh
specbind check traceability <spec-path>
specbind check traceability <spec-path> --json
specbind check contracts [<scope>] [--json]
```

Human-readable success output should stay compact:

```text
PASS requirements=24 active=6 design=6 tasks=6
```

Failure output should contain stable diagnostic codes, affected IDs, and source locations where available:

```text
FAIL
ACTIVE_UNKNOWN: 9.9 at spec.json
DESIGN_MISSING: 3.2
TASKS_MISSING: 4.1, 4.2
INVALID_TASK_MAPPING: "Requirement 2.1" at tasks.md:42
```

The JSON schema and exit-code table must be versioned contracts before implementation is considered complete.

## Lifecycle automation candidates

The same boundary can prevent `specbind-discovery` from becoming a general-purpose state manager. Candidate CLI command families include:

- create an active roadmap and generate its stable milestone ID
- apply an explicitly confirmed roadmap scope update
- bind or rebind the target release version
- check milestone and per-spec lifecycle consistency
- perform the deterministic portion of confirmed abandonment cleanup
- run release preflight checks and idempotent finalization mutations

These are accepted CLI responsibilities under [Decision 0009](./decisions/0009-milestone-cli-boundary.md), but their exact command names remain Draft. Discovery remains the user-facing entry point for understanding and routing a request, while CLI commands own the resulting mechanical writes. SpecBind does not expose a separate `specbind-milestone` agent skill.

## Integration with skills

Generated SpecBind skills should call the bundled CLI at the phase where its invariant becomes relevant:

```text
CLI mechanical check
  -> agent semantic review
  -> implementation or lifecycle transition
  -> fresh completion evidence
```

For the traceability check, requirements, design, tasks, validation, and release-readiness workflows should consume the same CLI contract instead of embedding agent-specific grep instructions. A standalone validation skill is unnecessary when its only purpose would be to expose one deterministic CLI command.

The stable project-customization surface is shared `{{SPEC_DIR}}/settings/templates/` and `{{SPEC_DIR}}/settings/rules/`; see [Decision 0008](./decisions/0008-customization-surface.md). Generated skills, agent metadata, and manifests are product-managed resources. The installer must preserve conflicting local edits safely, but direct skill modification is not the cross-agent customization contract.

The CLI and skills must respect supported settings customization while still enforcing documented machine-readable structure. A mechanical check reports an incompatible customized format explicitly rather than silently falling back to agent-specific searches.

## Initial implementation boundary

The first increment should remain narrow:

- read-only traceability validation for one spec directory
- concise default output
- stable non-zero failure exit behavior
- JSON output for agents and CI
- fixtures covering valid mappings, missing coverage, unknown references, duplicates, and invalid syntax
- integration into at least the design and tasks review paths

It should not initially validate task hierarchy, task dependencies, approval semantics beyond the active-set prerequisite, project-specific rules, or the substantive quality of requirements, design, and tasks.

## Open questions

- Final command names and whether `check` becomes the common read-only validation namespace.
- How the accepted Rust migration packages templates and preserves the current installation contract.
- The canonical Requirement ID syntax and parsing rules for each supported language.
- The JSON diagnostic schema and stable exit-code categories.
- Exact command contracts for the accepted milestone operations and any additional lifecycle candidates.
