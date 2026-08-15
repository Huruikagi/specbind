# 0010: Let the agent execute release instructions and the CLI guard finalization

Status: Accepted

## Context

Project release procedures differ and are documented as agent-readable instructions in `{{SPEC_DIR}}/settings/release.md`. They may involve repository commands, authenticated external services, application stores, manual checks, or project-specific judgment.

The Rust CLI can enforce SpecBind schemas and lifecycle invariants, but it cannot safely or portably execute arbitrary natural-language instructions. Conversely, an agent should not directly implement destructive SpecBind finalization through ad hoc file edits.

## Decision

- Keep `specbind-release` as the agent-facing orchestration skill.
- The Rust CLI owns core release preflight, deterministic state checks, evidence validation where mechanically possible, and idempotent finalization mutations.
- The AI agent reads the complete free-form `settings/release.md` under Decision 0063 and executes any applicable Prepare, Publish, Verify, and After finalize guidance under normal repository, authorization, and tool-permission boundaries.
- The CLI does not interpret Markdown code blocks as executable hooks and does not run arbitrary adapter commands.
- Under Decision 0066, the agent and human judge applicable project release work and, for Spec-backed milestones, pass the per-spec log summaries needed for mutation, not a structured external release-evidence object.
- Under Decision 0070, the CLI derives release readiness from existing authoritative artifacts and does not persist a separate aggregate readiness record.
- The CLI does not claim to verify external release success. It rechecks every deterministic core invariant it can observe before mutating active artifacts.
- Release finalization applies the Decision 0064 path-scoped Git safety check rather than requiring repository-wide cleanliness or equality with a previously captured `HEAD`.
- Decision 0069 accepts `specbind release preflight` as a stateless read-only readiness check whose result is not passed to finalization.
- Decisions 0068 and 0081 accept `specbind release finalize [--log-entries <path|->]` as the finalization command and conditional summary transport.

## Execution sequence

1. The release skill loads the active milestone, target version, and `settings/release.md`.
2. The release skill asks the Rust CLI to run core preflight and readiness checks.
3. If preflight succeeds, the agent executes applicable adapter preparation guidance.
4. The agent executes applicable publication guidance and uses any project references or results in its release judgment.
5. The agent executes applicable project verification guidance and decides with the human whether the project work succeeded.
6. The agent submits the per-spec delivered-change summaries to the Rust CLI finalization operation.
7. The CLI revalidates core invariants, checks evidence it can verify, and applies idempotent per-spec `log.md`, metadata, active-document, and roadmap-archive mutations.
8. After core finalization succeeds, the agent executes any applicable after-finalize guidance and reports its result separately.

## Failure semantics

- A preflight failure prevents adapter execution.
- A failure in applicable preparation, publication, or project-verification guidance prevents core finalization and preserves active SpecBind artifacts.
- Publication success followed by failed verification remains an active milestone until verification and finalization succeed; under Decision 0071 it is reported as external partial success, not as a completed SpecBind release or automatic rollback.
- A core finalization failure is retryable and must not duplicate history, partially finalize the milestone, or discard unrelated work.
- An After finalize failure does not roll back or reopen a verified release or completed core finalization. It is reported as follow-up work under Decision 0071.

## Boundaries

- Adapter instructions cannot waive a CLI readiness or finalization gate.
- Unrelated dirty files are governed by project policy; the CLI refuses only dirty or conflicting paths in its resolved finalization mutation set under Decision 0064.
- Dirty finalization targets and archive collisions are non-forceable. The user or agent must make target paths Git-clean before retrying.
- The CLI does not invent missing publication commands or credentials.
- The agent does not bypass the CLI by deleting the discovered `SpecBind Brief` artifact, `tasks.yaml`, or `roadmap.md` directly.
- The CLI does not claim semantic or external verification that it cannot actually observe.
- External writes remain subject to the user's authority and the active agent environment's permission model.

## Consequences

- Project release flexibility remains in an editable Markdown adapter.
- SpecBind lifecycle safety and idempotency remain shared across every supported agent.
- The release skill becomes orchestration around stable CLI contracts rather than a file-mutation implementation.
- CLI diagnostics use the accepted Decision 0067 text result in v1; Decision 0074 defers a command-wide JSON envelope.
- Tests must cover handoff and retry behavior across each failure boundary.

## Open questions

- None currently recorded for this boundary.
