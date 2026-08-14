# 0013: Use tasks.yaml as the canonical task artifact

Status: Accepted

## Context

The inherited cc-sdd workflow stores the active implementation plan and its progress in `tasks.md`. Although the file is Markdown, workflows already depend on an implicit machine structure:

- checkbox state
- numeric Task IDs and hierarchy
- optional and parallel markers
- Requirement ID references
- dependency and boundary annotations
- blocked reasons

Reliable traceability, dependency checks, status reporting, approval-revision checks, and guarded lifecycle operations would require SpecBind to define and maintain a custom Markdown parser for this structure. Normal progress updates also change the same document whose plan was previously approved.

Operationally, requirements and design benefit from prose review, while tasks are used more often as executable agent input and as a progress/status source. Human readability remains useful, but free-form Markdown is not required for that purpose.

## Decision

- `tasks.yaml` is the only canonical task artifact in the target SpecBind workflow.
- The tasks workflow creates `tasks.yaml`; implementation, review, validation, status, traceability, and release workflows read and update it.
- Target SpecBind workflows do not generate or maintain a parallel `tasks.md` view.
- Human-readable task and progress views are provided by the accepted `spec status`, `tasks list`, and `tasks show` CLI read model under Decision 0025. Users may also inspect the YAML directly.
- Requirements, design, contracts, briefs, per-spec release logs, and roadmaps remain Markdown unless a separate decision changes them.
- `tasks.yaml` uses a versioned, machine-validated schema with stable English field names. Human-authored task text follows the spec's configured product language.
- The schema must distinguish the approved task-plan definition from mutable execution state so normal progress updates do not inherently rewrite the approved plan.
- The tasks gate fingerprints a typed plan projection rather than the serialized YAML file. Status or checkbox-equivalent state and blocked execution details are excluded from that projection under Decision 0018.
- Task order remains a conservative implicit dependency; `parallel` records reviewed exceptions and `depends_on` adds sparse non-obvious prerequisites within the same spec under Decisions 0019 and 0027.
- Task and group IDs retain cc-sdd-style one- or two-level positional numbering under Decision 0020.
- Executable tasks may carry dedicated `completion_criteria`; the field is required only when the completed state or verification would otherwise be ambiguous under Decision 0021.
- The target schema has no optional-task category; every executable task in the active plan is required under Decision 0022.
- Group and executable-task objects use the strict sparse plan shape accepted by Decision 0023.
- Task progress uses the sparse persisted `completed | blocked` execution state accepted by Decision 0024; absence means pending and `in_progress` remains run-scoped.
- Persistent free-form implementation guidance lives in optional discovered `SpecBind Implementation Notes` artifacts outside `tasks.yaml`, under Decisions 0026 and 0057.
- Exact fields, status values, hierarchy representation, fingerprint projections, and evidence references remain a follow-up schema decision.

## Lifecycle

- `tasks.yaml` exists only for the active milestone change.
- It is created or replaced by the tasks workflow after requirements and design gates permit task generation.
- Same-milestone task-plan revisions update the active YAML and invalidate the tasks gate as defined by the spec state machine.
- Implementation updates structured progress and execution fields without changing plan approval when the plan definition is unchanged.
- Successful release finalization removes `tasks.yaml`; its pre-finalization content normally remains in ordinary Git history or an optional project-created release reference under Decision 0064.
- Released and idle specs do not require a placeholder task file.

## Human interaction

The YAML artifact is not intended to make users review every task definition manually. The primary human surfaces are concise CLI views for:

- completed, pending, and blocked counts
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
- In-progress projects must preserve completion, dependency, boundary, Requirement ID, and blocked information where it can be established. Relevant inherited implementation notes move to the default implementation-notes artifact with `artifact_id: main` under Decisions 0026 and 0057. Inherited optional markers require the explicit migration resolution defined by Decision 0022.
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

- Completion and verification evidence references.
- Completion projection fields and their canonical serialization. Decision 0028 fixes the task-plan projection and fingerprint algorithm.
- Exact `tasks.md` migration grammar and diagnostics.
- Routing diagnostics for inherited cross-spec `_Depends:_` text follow Decision 0027 and must not coerce it into a local Task ID.
