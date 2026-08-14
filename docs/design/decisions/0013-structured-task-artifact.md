# 0013: Use tasks.yaml as the canonical task artifact

Status: Accepted

## Context

The inherited cc-sdd workflow stores the active implementation plan and its progress in `tasks.md`. Although the file is Markdown, workflows already depend on an implicit machine structure:

- checkbox state
- numeric Task IDs and hierarchy
- optional and parallel markers
- Requirement ID references
- dependency and boundary annotations
- blocked reasons and implementation notes

Reliable traceability, dependency checks, status reporting, approval-revision checks, and guarded lifecycle operations would require SpecBind to define and maintain a custom Markdown parser for this structure. Normal progress updates also change the same document whose plan was previously approved.

Operationally, requirements and design benefit from prose review, while tasks are used more often as executable agent input and as a progress/status source. Human readability remains useful, but free-form Markdown is not required for that purpose.

## Decision

- `tasks.yaml` is the only canonical task artifact in the target SpecBind workflow.
- The tasks workflow creates `tasks.yaml`; implementation, review, validation, status, traceability, and release workflows read and update it.
- Target SpecBind workflows do not generate or maintain a parallel `tasks.md` view.
- Human-readable task and progress views are provided by CLI output such as status and task-display operations. Users may also inspect the YAML directly.
- Requirements, design, contracts, briefs, changelogs, and roadmaps remain Markdown unless a separate decision changes them.
- `tasks.yaml` uses a versioned, machine-validated schema with stable English field names. Human-authored task text follows the spec's configured product language.
- The schema must distinguish the approved task-plan definition from mutable execution state so normal progress updates do not inherently rewrite the approved plan.
- The tasks gate fingerprints a typed plan projection rather than the serialized YAML file. Status or checkbox-equivalent state, blocked execution details, and implementation notes are excluded from that projection under Decision 0018.
- Exact fields, status values, hierarchy representation, fingerprint projections, and evidence references remain a follow-up schema decision.

## Lifecycle

- `tasks.yaml` exists only for the active milestone change.
- It is created or replaced by the tasks workflow after requirements and design gates permit task generation.
- Same-milestone task-plan revisions update the active YAML and invalidate the tasks gate as defined by the spec state machine.
- Implementation updates structured progress and execution fields without changing plan approval when the plan definition is unchanged.
- Successful release finalization removes `tasks.yaml` after the immutable release reference has preserved its pre-finalization content.
- Released and idle specs do not require a placeholder task file.

## Human interaction

The YAML artifact is not intended to make users review every task definition manually. The primary human surfaces are concise CLI views for:

- completed, pending, blocked, and optional counts
- next actionable tasks
- dependency blockers
- active Requirement ID coverage
- plan revision or approval mismatch

Detailed YAML remains available when a user wants to inspect or edit the task plan. A direct plan edit is detected through the tasks-gate revision contract and requires appropriate review and reapproval.

## Customization boundary

- The fixed YAML keys, types, identifiers, and invariants are a SpecBind machine contract and are not project-customizable.
- Shared task-generation rules may customize decomposition principles, task sizing, wording, and project-specific expectations.
- Any installed task scaffold may customize documented content defaults only within the supported schema.
- Unsupported fields or incompatible types produce explicit diagnostics rather than agent-specific fallback parsing.

## Migration

- Existing `tasks.md` files are migration inputs, not a second supported steady-state format.
- Migration reads only the inherited syntax that SpecBind explicitly supports and reports ambiguous task structure instead of guessing.
- In-progress projects must preserve completion, optional, dependency, boundary, Requirement ID, blocked, and relevant note information where it can be established.
- Historical `tasks.md` files remain available at their existing commits or release references and are not rewritten merely to normalize history.
- Target skills and the Rust CLI use `tasks.yaml` after migration; they do not continue dual writes.

## Consequences

- Task status and dependency reporting become deterministic without reconstructing semantics from Markdown decoration.
- Plan and progress fingerprints can use schema-defined projections.
- Claude Code and Codex consume the same structured task contract.
- The inherited checkbox and annotation syntax remains relevant only to migration.
- Current task, implementation, status, batch, review, validation, and release skill templates require coordinated migration.
- The target artifact catalog and lifecycle documents use `tasks.yaml`; the current artifact index continues to describe shipped `tasks.md` behavior until implementation changes.

## Open schema details

- Top-level metadata and schema-version representation.
- Task hierarchy and grouping representation.
- Required plan fields and mutable execution fields.
- Status enum and blocked / skipped / optional semantics.
- Completion and verification evidence references.
- Exact plan and completion projection fields and their canonical serialization; Decision 0018 fixes the plan/execution boundary but not the v1 field set.
- Exact `tasks.md` migration grammar and diagnostics.
