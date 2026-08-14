# 0044: Use a minimal strict spec.yaml root

Status: Accepted

## Context

The inherited `spec.json` stores `feature_name`, `created_at`, `updated_at`, `language`, phase flags, approval booleans, and readiness booleans. The target lifecycle now has precise state, gate evidence, and gate-local timestamps. Retaining the inherited summary fields would duplicate authoritative values and create contradictory combinations.

Canonical spec identity is already the spec directory key/path under the configured spec root. A stored `feature_name` would either duplicate that identity or introduce a second display-name concept with no lifecycle purpose.

## Decision

- A target `spec.yaml` root is a strict object with exactly three required fields:
  - `schema_version: 1`
  - `language: en | ja`
  - `active_change`
- `active_change` is always present. It is `null` for the released idle state or a strict active-change object.
- An active-change object requires exactly:
  - `milestone_id`: the Decision 0043 UUID v7
  - `state`: `requirements`, `design`, `tasks`, `implementation`, or `release_ready`
  - `requirement_ids`: `null` or the accepted non-empty Requirement ID list
  - optional non-empty `gate_evidence`: the Decision 0040 container
- The state-dependent `requirement_ids` and evidence combinations remain semantic lifecycle invariants under Decision 0040 rather than a JSON Schema state union.
- The root stores no `feature_name`. The CLI derives canonical spec identity from the spec's directory key/path and checks it against roadmap membership. Human-facing capability names belong in prose artifacts.
- The root stores no `created_at` or `updated_at`. Git, roadmap, changelog, and gate-local `passed_at` values own the useful history.
- The inherited `phase`, `approvals`, and `ready_for_implementation` fields are replaced by `active_change.state` and `gate_evidence` and are invalid target fields.
- `target_release` and cross-spec review data remain roadmap-owned and are not copied into `spec.yaml`.
- All root and active-change additional properties are rejected.
- CLI-generated YAML writes root keys in `schema_version`, `language`, `active_change` order and active-change keys in `milestone_id`, `state`, `requirement_ids`, `gate_evidence` order when present. Readers do not treat mapping order as semantic.

## Consequences

- Idle metadata is explicit and minimal:

  ```yaml
  schema_version: 1
  language: ja
  active_change: null
  ```

- Spec moves or renames cannot be hidden by editing a duplicate name field; they remain an explicit future migration concern under Decision 0041.
- Status derives lifecycle meaning from one state model rather than reconciling inherited summary flags.
- Per-spec language remains available to every agent workflow without relying on project-global language.
