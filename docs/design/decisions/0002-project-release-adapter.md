# 0002: Combine core release lifecycle with a project adapter

Status: Accepted

## Context

SpecBind needs a portable release lifecycle for active specs, but actual publication differs by project. A release may require package version updates, platform-specific builds, deployment, Git tags, GitHub Releases, application-store operations, or other repository-specific steps.

Putting all publication behavior in the core skill would make it project-specific. Limiting SpecBind to post-publication cleanup would separate release verification from the evidence required to finalize active spec documents.

## Decision

`specbind-release` combines:

- a non-overridable core release contract
- project-specific release instructions read from `{{SPEC_DIR}}/settings/release.md`

The core owns lifecycle gates, sequencing, fresh verification requirements, and idempotent finalization of SpecBind artifacts. The project adapter owns instructions for preparing, publishing, and verifying the project's actual release.

The adapter is an agent-readable instruction document, not an unrestricted executable hook interface. It may instruct the agent to use existing project commands and external release systems, subject to the normal repository instructions, authorization boundaries, and tool permissions.

The execution boundary is fixed in [Decision 0010](./0010-release-execution-boundary.md): the AI agent executes adapter instructions, while the Rust CLI owns core preflight and idempotent finalization. The CLI never treats adapter Markdown as a shell script.

## Adapter phases

The initial contract uses four explicit phases:

1. `Prepare`: version synchronization, build/package preparation, and project-specific pre-publication checks.
2. `Publish`: tag, deployment, release workflow, store submission, or other publication operations.
3. `Verify`: fresh checks proving that the intended version was actually published and is usable.
4. `After finalize`: optional project cleanup that runs only after core SpecBind finalization succeeds.

The exact Markdown schema remains Draft. Explicit phases are preferred over generic before/after hooks because the core must know where publication succeeded and which evidence permits destructive finalization of active documents.

## Core invariants

The adapter cannot weaken or replace these rules:

- A concrete target release version is required before release operations begin.
- Current milestone scope, tasks, approvals, and completion evidence must pass core readiness gates.
- Publication must have fresh success evidence before active spec documents are finalized.
- The immutable release reference must retain the pre-finalization `brief.md`, `tasks.md`, and `roadmap.md`.
- Finalization must be idempotent, archive the active roadmap without overwriting history, and must not remove unrelated work.
- Failure before verified publication preserves all active documents.

## Missing or incomplete adapter

If `{{SPEC_DIR}}/settings/release.md` is missing or does not define enough information to publish and verify the project safely, `specbind-release` stops before external publication. It reports the missing phase instead of inventing release commands from incidental repository files.

## Consequences

- SpecBind installs a customizable release-adapter scaffold in project settings.
- Projects can evolve release operations without forking the core release skill.
- Claude Code and Codex consume the same adapter contract.
- Adapter updates are project configuration changes and must be preserved during SpecBind upgrades.
- The core reports adapter-phase failure separately from core-finalization failure.
- An optional After finalize failure is reported without rolling back the verified release or completed core finalization.
