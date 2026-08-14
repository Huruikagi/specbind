# 0039: Use minimal tasks gate evidence for the approved plan

Status: Accepted

## Context

The tasks workflow performs substantive plan review before approval: active Requirement ID coverage, design and contract coverage, task executability, observable completion, dependency and ordering sanity, boundary ownership, and consistency with requirements and design. Persisting a boolean for every review dimension would repeat the meaning of accepted gate evidence without making the semantic judgments replayable.

`tasks.yaml` also contains mutable execution state. Decision 0028 therefore fingerprints only its normalized typed `plan` projection rather than the complete artifact.

## Decision

- Tasks gate evidence is a strict approval-evidence object containing exactly:
  - required `passed_at`
  - required `approval_mode`
  - conditional `delegation_workflow`
  - required `input_revisions`
- `input_revisions` contains exactly one required key, `tasks.yaml#plan`, whose value is the Decision 0016 fingerprint of the Decision 0028 normalized task-plan projection. Additional input keys are rejected.
- `explicit` omits `delegation_workflow`; `delegated` requires it under Decision 0012.
- Presence of current accepted tasks gate evidence means the required task-plan review and sanity review passed. The object contains no per-dimension pass flags, review summary, findings, task counts, or copied Requirement ID coverage list.
- Mutable `execution` state, blocked reasons, runtime implementation notes, upstream artifact fingerprints, and roadmap-owned cross-spec review data are excluded.
- Tasks approval requires the prerequisite design gate to remain fresh under Decision 0032.

## Consequences

- Ordinary implementation progress does not invalidate task-plan approval.
- Any semantic change to the typed `plan` projection invalidates the tasks gate and completion evidence.
- Task review can evolve without adding tautological fields to `spec.yaml`.
- The `tasks.yaml#plan` key makes the projection boundary visible instead of implying that the complete YAML file was hashed.
