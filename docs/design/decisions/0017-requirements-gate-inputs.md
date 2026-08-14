# 0017: Exclude the active brief from requirements gate fingerprints

Status: Accepted

## Context

The `SpecBind Brief` artifact is discovery-owned, milestone-local free-form guidance under Decision 0062 for applying a requested change to the persistent requirements. It is removed after successful release finalization. The requirements gate instead approves the resulting singleton `SpecBind Requirements` content and the active Requirement ID set selected for the change. Decision 0057 discovers these Markdown artifacts by OKF type rather than filename.

Fingerprinting the brief would make later editorial changes to that guidance look like changes to already-approved requirements even when the authoritative requirements and active set remain identical.

Release history needs a problem and delivered-scope summary, but the brief does not need to become an authoritative release input for that purpose. The final requirements, active Requirement IDs, completed tasks, roadmap, and release evidence describe what was actually delivered. The pre-finalization brief remains available through the immutable release reference.

## Decision

- Requirements gate freshness covers the singleton `SpecBind Requirements` artifact under the logical evidence key `requirements` and the ordered active Requirement ID set.
- The `SpecBind Brief` artifact is not fingerprinted and is not stored in requirements gate evidence.
- Editing only the brief after requirements approval does not by itself make the spec inconsistent or invalidate the requirements gate.
- If a brief revision represents a real scope change, the responsible workflow must revise the authoritative requirements or active Requirement ID set as needed and emit `REQUIREMENTS_CHANGED`.
- Per-spec `log.md` authoring may use the discovered brief as drafting context, but the released summary must be checked against final authoritative artifacts and release evidence. Brief content alone does not determine the release-log entry.
- Requirements gate evidence stores the ordered active Requirement ID snapshot directly as `approved_requirement_ids`; Decision 0018 defines its comparison contract.

## Consequences

- Out-of-band edits to the brief are not detected through gate-evidence fingerprint comparison.
- Requirements approval remains attached to the output of requirements work rather than its discovery prompt.
- A scope-changing brief update cannot rely on automatic fingerprint invalidation; workflows must express the semantic change explicitly.
- Release history can remain concise while the immutable release reference preserves the complete original brief for deeper inspection.
