# 0066: Let the agent judge release success and the CLI insert spec logs

Status: Accepted

## Context

The free-form project adapter may involve external systems, manual observations, credentials, or no project-specific action at all. The Rust CLI cannot reliably observe those outcomes. Requiring a structured publication-evidence object would add ceremony without turning an agent assertion into independently verifiable proof.

When a milestone contains Spec-backed work, release finalization still needs to add one correctly placed, idempotent `log.md` entry for every participating spec. If the agent edits those files before invoking finalization, the Decision 0064 target-path guard correctly treats them as dirty. The CLI should therefore own the structural log mutation while the agent supplies the human-authored delivered-change summary. Direct-only milestones do not create per-spec log entries.

## Decision

### Release judgment

- SpecBind defines no dedicated release-evidence object and persists no publication success fields in `spec.yaml`, `roadmap.md`, `log.md`, or another state artifact.
- The release skill executes any applicable adapter guidance. The agent and human decide whether that project work succeeded and invoke `specbind release finalize` only when they are ready to close the milestone.
- A direct human invocation carries the same caller responsibility. The command invocation is an explicit transition request, not evidence that the CLI observed an external release.
- The CLI does not receive or require a tag, Release URL, deployment ID, command list, exit-code list, verification flags, or external timestamps.
- The CLI reports only the deterministic facts it can verify: current schemas, lifecycle state, roadmap membership and release binding, gate freshness, task completion, the accepted contract review when applicable, target-path safety, archive collision rules, and finalization consistency.
- The CLI and generated output must not claim that SpecBind independently verified publication or an external service. If applicable project work fails, the skill does not invoke finalization.

### Log insertion

- Before finalization of a Spec-backed milestone, the agent prepares one delivered-change summary for every participating spec. These summaries are finalization mutation content, not release evidence.
- For Spec-backed milestones, the finalization request must cover the exact participating spec set. Missing, duplicate, or extra spec summaries are input errors. Direct-only milestones omit the log-entry input entirely.
- The CLI owns the complete structural `log.md` update:
  - select or create the applicable `## YYYY-MM-DD` heading
  - keep date headings newest first
  - insert one flat release entry per participating spec
  - add the bound release version, milestone ID, and archived-roadmap link in the canonical entry wrapper
  - preserve unrelated existing log content
  - identify an existing entry by milestone ID for idempotent retry
  - accept an identical retry without duplication and reject conflicting existing content
- The log updates occur in the same ordered, retry-safe finalization transaction as `spec.yaml` transitions, Brief, Research, and `tasks.yaml` removal and the applicable roadmap/cross-spec-review archival. The operation is not claimed to be crash-atomic across files.
- The agent does not pre-edit `log.md` as part of ordinary release orchestration. It may use the Brief as drafting context, but each summary must agree with the final requirements, active Requirement IDs, design, completed tasks, roadmap scope, and accepted completion/cross-spec records.
- Decision 0068 defines the strict JSON transport, local-date source, canonical wrapper, inline-Markdown safety check, and retry diagnostics.

## Consequences

- SpecBind does not pretend to verify external state it cannot observe.
- Projects avoid a universal release-evidence schema while retaining deterministic lifecycle checks.
- AI supplies the semantic release description without performing fragile date-heading or list insertion itself.
- The target-path guard remains meaningful because `log.md` stays clean until the CLI applies finalization.
