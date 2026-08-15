# 0021: Use explicit but conditionally optional task completion criteria

Status: Accepted

## Context

The inherited cc-sdd rules require every executable task to contain an observable-completion bullet inside its free-form details. Structured `tasks.yaml` can make that information easier for implementers and reviewers to find by giving it a dedicated `completion_criteria` field.

Requiring bespoke criteria for every task would add boilerplate when the completed state is already unambiguous from the task content or when the project's canonical validation procedure is the only necessary proof.

## Decision

- `completion_criteria`, when present, is a non-empty array of non-empty human-authored strings.
- The field belongs only to executable tasks and is part of the approved plan projection.
- Omit the field rather than writing an empty array when no explicit criteria are needed.
- Omission is allowed when either:
  - the observable completed state is unambiguous from the task title and details; or
  - the project's canonical validation commands and review procedure fully establish completion without task-specific instructions.
- Explicit criteria are required when completion would otherwise be ambiguous, when nonstandard verification is needed, or when the task has user-visible, persisted-data, cross-boundary, migration, or other outcomes that project-default checks do not fully demonstrate.
- Omitting `completion_criteria` never waives implementation verification, task review, or feature-level validation. The implementation workflow still derives a concrete completion definition from the task, requirements, design, and project validation contract before claiming success.
- Container-only groups do not carry `completion_criteria`.
- Changes to the presence, order, or content of explicit criteria change the task-plan fingerprint.

## Consequences

- Important completion expectations have a stable structured location instead of being hidden among detail bullets.
- Straightforward tasks avoid repetitive criteria such as merely restating that the normal test suite passes.
- Task-plan review remains responsible for rejecting an omitted field when the completed state is not actually self-evident.
- JSON Schema and Rust validate explicit values but do not decide whether omission is semantically justified. Task review owns that judgment under Decision 0080.

## Migration

- A clearly observable-completion bullet may be promoted from inherited task details into `completion_criteria`.
- Other detail bullets remain task details.
- When an inherited task is unambiguous under the omission rule, migration may leave `completion_criteria` absent.
- Ambiguous source text produces a migration diagnostic rather than an invented completion contract.
