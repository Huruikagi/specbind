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
- The agent passes the target version and structured release evidence into the finalization boundary. Decision 0064 does not require a universal publication or source-revision field.
- The CLI must not accept a bare success assertion as sufficient proof. It rechecks core invariants and all evidence it can verify from repository or structured state before mutating active artifacts.
- Release finalization applies the Decision 0064 path-scoped Git safety check rather than requiring repository-wide cleanliness or equality with a previously captured `HEAD`.
- Decision 0065 accepts `specbind release finalize [--json] [--force]` as the finalization command. The evidence handoff arguments remain Draft.

## Execution sequence

1. The release skill loads the active milestone, target version, and `settings/release.md`.
2. The release skill asks the Rust CLI to run core preflight and readiness checks.
3. If preflight succeeds, the agent executes applicable adapter preparation guidance.
4. The agent executes applicable publication guidance and captures any project reference plus other evidence useful to the release judgment.
5. The agent executes applicable project verification guidance and captures fresh results.
6. The agent submits the version and required structured evidence to the Rust CLI finalization operation.
7. The CLI revalidates core invariants, checks evidence it can verify, and applies idempotent per-spec `log.md`, metadata, active-document, and roadmap-archive mutations.
8. After core finalization succeeds, the agent executes any applicable after-finalize guidance and reports its result separately.

## Failure semantics

- A preflight failure prevents adapter execution.
- A failure in applicable preparation, publication, or project-verification guidance prevents core finalization and preserves active SpecBind artifacts.
- Publication success followed by failed verification remains an active milestone until verification and finalization succeed; it is not reported as an unreleased rollback automatically.
- A core finalization failure is retryable and must not duplicate history or partially discard unrelated work.
- An After finalize failure does not roll back a verified release or completed core finalization. It is reported as follow-up work.

## Boundaries

- Adapter instructions cannot waive a CLI readiness or finalization gate.
- Unrelated dirty files are governed by project policy; the CLI refuses only dirty or conflicting paths in its resolved finalization mutation set under Decision 0064.
- Decision 0065 permits an explicitly user-confirmed `--force` retry for those forceable target-path conflicts only; archive collisions and all other core guards remain non-forceable.
- The CLI does not invent missing publication commands or credentials.
- The agent does not bypass the CLI by deleting the discovered `SpecBind Brief` artifact, `tasks.yaml`, or `roadmap.md` directly.
- The CLI does not claim semantic or external verification that it cannot actually observe.
- External writes remain subject to the user's authority and the active agent environment's permission model.

## Consequences

- Project release flexibility remains in an editable Markdown adapter.
- SpecBind lifecycle safety and idempotency remain shared across every supported agent.
- The release skill becomes orchestration around stable CLI contracts rather than a file-mutation implementation.
- CLI diagnostics and evidence inputs need stable human-readable and JSON representations.
- Tests must cover handoff and retry behavior across each failure boundary.

## Open questions

- Exact preflight command name.
- Evidence schema, provenance, freshness, and redaction rules.
- Whether a preflight result has a stable session or plan ID that finalization must reference.
