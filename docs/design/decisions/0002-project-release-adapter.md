# 0002: Combine core release lifecycle with a project adapter

Status: Accepted

## Context

SpecBind needs a portable release lifecycle for active specs, but actual publication differs by project. A release may require package version updates, platform-specific builds, deployment, Git tags, GitHub Releases, application-store operations, or other repository-specific steps.

Putting all publication behavior in the core skill would make it project-specific. Limiting SpecBind to post-publication cleanup would separate release verification from the evidence required to finalize active spec documents.

## Decision

`specbind-release` combines:

- a non-overridable core release contract
- project-specific release instructions read from `{{SPEC_DIR}}/settings/adapters/release.md`

The core owns lifecycle gates, sequencing, fresh verification requirements, and idempotent finalization of SpecBind artifacts. The project adapter owns instructions for preparing, publishing, and verifying the project's actual release.

The adapter is an agent-readable instruction document, not an unrestricted executable hook interface. It may instruct the agent to use existing project commands and external release systems, subject to the normal repository instructions, authorization boundaries, and tool permissions.

The execution boundary is fixed in [Decision 0010](./0010-release-execution-boundary.md): the AI agent executes adapter instructions, while the Rust CLI owns core preflight and idempotent finalization. The CLI never treats adapter Markdown as a shell script.

## Adapter orchestration phases

The release skill uses four semantic project-action phases within the core sequence:

1. `Prepare`: version synchronization, build/package preparation, and project-specific pre-publication checks.
2. `Publish`: tag, deployment, release workflow, store submission, or other publication operations.
3. `Verify`: fresh checks proving that the intended version was actually published and is usable.
4. `After finalize`: optional project cleanup that runs only after core SpecBind finalization succeeds.

Under Decision 0063, these phases do not require literal Markdown headings or a parsed section order. The agent interprets the complete free-form adapter and applies relevant instructions at the appropriate point. An empty body explicitly means no adapter-specific actions, while the core still requires its own readiness and finalization evidence.

## Core invariants

The adapter cannot weaken or replace these rules:

- A concrete target release version is required before release operations begin.
- Current milestone scope, tasks, approvals, and completion evidence must pass core readiness gates.
- Applicable project release actions must be judged successful by the agent and human before they request finalization; every deterministic core guard must pass independently.
- Finalization must be idempotent, archive the active roadmap without overwriting history, and must not remove unrelated work.
- Failure before required release verification preserves all active documents.

## Missing or unclear adapter

If `{{SPEC_DIR}}/settings/adapters/release.md` is missing, `specbind-release` stops because required project configuration is absent. A present empty adapter is valid. If non-empty guidance appears to require a project action but is ambiguous or unsafe, the agent stops before that action instead of inventing release commands from incidental repository files.

## Consequences

- SpecBind installs a customizable release-adapter scaffold in project settings.
- Projects can evolve release operations without forking the core release skill.
- Claude Code and Codex consume the same adapter contract.
- Adapter updates are project configuration changes and must be preserved during SpecBind upgrades.
- The core reports adapter-phase failure separately from core-finalization failure.
- An optional After finalize failure is reported without rolling back the verified release or completed core finalization.
