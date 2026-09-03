# 0192: Project typed handlers for Milestone actions

Status: Accepted

## Context

`milestone status` already derives the authoritative ordered actionable set and
exact command operands. `sb-drive` then maps free-form action strings to owning
Skills, guarded CLI work, or its release stopping boundary in a separate prose
table. The read model now also exposes reverse `adoption_finalize`, which was
not represented in that table.

The mapping is deterministic product routing. Leaving it open-coded in an Agent
document allows a new action or renamed Skill to produce an incomplete route
even when lifecycle state itself is correct.

## Decision

The Rust read model represents Milestone actions with a closed
`MilestoneActionKind` rather than `&'static str`. Existing text and JSON action
names remain unchanged.

`milestone status` adds `Milestone kind: delivery|reverse` to text and
`milestoneKind` to JSON. Every actionable entry also receives one handler:

- `kind=skill` names an installed Skill target and an optional mode;
- `kind=guarded_cli` names a guarded CLI operation whose existing authority
  checks still apply;
- `kind=boundary` names the explicit workflow beyond Drive's stopping point.

For delivery Milestones, Requirements, Design, and Tasks use `sb-plan` in
`all_spec` mode; Contract Review, implementation, and validation use their
existing owning Skills; release binding is guarded CLI work; and release
preflight is the boundary before explicit `sb-release` execution.

For reverse Milestones, every currently actionable phase uses `sb-discovery` in
`reverse_resume` mode. That handler does not grant continuation authority:
Decision 0191 still requires an explicit maintainer request before the reverse
orchestrator can approve remaining Gates. It identifies the continuation
orchestrator rather than replacing a nested phase owner: at `contract_review`,
Discovery dispatches `sb-contract-review` under the relayed reverse authority
and resumes afterward.

The lifecycle read model does not depend on the Skill catalog or CLI rendering.
The CLI projection layer derives handlers exhaustively from the typed action and
Milestone kind. Mechanical tests prove every skill target exists in the embedded
catalog and every action variant has a handler.

`sb-drive` consumes the returned handler and contains no competing
action-to-owner table. It dispatches `skill`, applies the existing authority
boundary to `guarded_cli`, stops on `boundary`, and fails closed on an unknown
kind, target, or mode.

The JSON fields are additive under Decision 0158. Consumers continue to ignore
unknown fields within the executable major.

## Consequences

- Status and Drive cannot disagree about the owner of a known action.
- Reverse finalization and future actions must receive an explicit handler.
- Skill identities stay in the transport/integration projection rather than
  becoming dependencies of the lifecycle read model.
- Handler metadata communicates routing, not permission or successful work.

## Verification

Read-model tests cover all typed action variants. CLI text and JSON tests cover
delivery skill, guarded CLI, release boundary, and reverse-resume handlers.
Catalog conformance rejects an unknown Skill target. Drive Skill tests reject a
local owner table and require handler-based routing. Fresh forward tests cover
one delivery action and one checkpointed reverse continuation.
