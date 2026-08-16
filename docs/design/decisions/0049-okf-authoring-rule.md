# 0049: Distribute a concise OKF authoring rule

Status: Superseded by 0094

## Context

Decision 0045 makes the configured spec root an OKF v0.2 Knowledge Bundle, and Decision 0048 adopts the reserved `log.md` format. Every supported agent that creates or edits managed Markdown needs the same authoring constraints.

Copying the complete upstream OKF specification into each consumer project would add substantial unrelated context, duplicate an external specification, and create an update burden. Most SpecBind workflows need only the small structural subset that applies to managed artifacts.

## Decision

- SpecBind distributes `{{SPEC_DIR}}/settings/rules/okf-artifacts.md` as the shared, agent-readable OKF authoring rule.
- The rule is itself an OKF concept document with `type: SpecBind Rule`.
- It identifies the targeted OKF version and links to the canonical upstream specification, but does not copy the complete specification.
- The concise rule covers at least:
  - normal concept documents require parseable YAML frontmatter and a non-empty `type`
  - unknown frontmatter keys must be preserved on round-trip
  - `index.md` and `log.md` are reserved and follow their OKF-specific forms
  - `log.md` has no frontmatter and uses newest-first ISO `YYYY-MM-DD` date sections
  - internal relationships use standard Markdown links
  - optional OKF lifecycle, trust, provenance, and attestation fields do not replace SpecBind lifecycle state or gate evidence
  - artifact-specific SpecBind profiles add requirements on top of the common OKF rule
- Agent workflows that create or rewrite managed Markdown load this shared rule, directly or through the common rules-loading contract.
- The rule is authoring guidance, not an executable schema or an authority that can weaken core invariants. The CLI validates all deterministic OKF and SpecBind profile requirements independently.
- Project maintainers may extend the rule with local guidance under Decision 0008, but local edits cannot make CLI-invalid artifacts valid.
- Updating the targeted OKF version is an explicit SpecBind compatibility change. It is not silently inherited from changes at the canonical URL.
- A bundled offline copy of the complete OKF specification is outside v1. It may be added later under a references surface if an offline use case justifies its context and maintenance cost.

## Consequences

- All supported agents receive one concise rule instead of agent-specific OKF instructions.
- Templates provide the artifact's concrete initial frontmatter, while the shared rule explains the cross-artifact invariants that must survive edits.
- Consumer projects retain a visible canonical link for deeper or newly introduced OKF behavior without loading the complete upstream specification during normal work.

## Reference

- [Open Knowledge Format v0.2 specification](https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md)
