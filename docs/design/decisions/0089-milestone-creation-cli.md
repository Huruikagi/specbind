# 0089: Expose milestone creation, scope, and rebaseline commands

Status: Accepted

## Context

Decisions [0043](./0043-uuidv7-milestone-id.md), [0046](./0046-roadmap-work-items.md), [0051](./0051-current-state-roadmap.md), and [0054](./0054-milestone-baseline-revision.md) fix the active Roadmap's identity, work-item grammar, current-state-only content, and baseline contract. The [Milestone state machine](../milestone-state-machine.md) fixes the `MILESTONE_CREATED`, `MILESTONE_SCOPE_UPDATED`, and `MILESTONE_REBASELINED` guards and effects, and the [Spec state machine](../spec-state-machine.md) fixes `SPEC_CREATED` and `CHANGE_STARTED`.

No public command creates any of them. Decisions [0086](./0086-completion-cli-handshake.md), [0087](./0087-milestone-review-cli.md), and [0088](./0088-gate-approval-cli.md) made the lifecycle reachable from `requirements` through release, but the head of the chain is still unreachable: an active Roadmap and its participating `active_change` entries can only be authored by hand. Decision 0054 also explicitly leaves the rebaseline command syntax to a later CLI-surface decision.

This decision fixes that public surface. It changes no accepted artifact shape, work-item grammar, or transition guard.

### One conflict this decision resolves

Decision 0054 requires a completely clean repository before the milestone-creation mutation, including no untracked files. The Spec state machine's `SPEC_CREATED` guard, written from the per-Spec perspective, expects "an active brief and initial requirements scaffold are available."

Both cannot hold at creation time: authoring a Brief before `milestone create` leaves an untracked file and fails the clean-repository guard. This decision resolves the conflict in favor of Decision 0054, whose clean-baseline rule is load-bearing for every later Contract diff.

## Decision

### Commands and ownership

The accepted commands are:

```text
specbind milestone create --scope <path|->
specbind milestone update-scope --scope <path|->
specbind milestone rebaseline --revision <revision>
```

- These are milestone-owned operations. None accepts a Spec identity; participating Spec initialization is derived from the submitted scope.
- `create` is the sole `MILESTONE_CREATED` mutation, `update-scope` the sole `MILESTONE_SCOPE_UPDATED` mutation, and `rebaseline` the sole `MILESTONE_REBASELINED` mutation.
- V1 adds no milestone deletion, abandonment, or per-Spec scope-removal command. `MILESTONE_ABANDONED` and `SPEC_SCOPE_REMOVED` reconcile repository and Spec content under [Decision 0005](./0005-active-change-abandonment.md) and remain a separate CLI-surface decision.
- V1 adds no milestone read command beyond the existing `milestone status`.

### Scope candidate input

`--scope` reads one strict UTF-8 JSON document. `-` reads standard input; a path must identify a repository-external ordinary non-symlink file. The input is transient command data and is never copied, archived, fingerprinted, or deleted by SpecBind.

Unlike the Decision 0088 gate flags, the scope is genuinely document-shaped: a nested set of categories, per-item summaries, typed dependency references, and free-form Markdown. It therefore uses the same candidate transport as Decisions 0086 and 0087.

The version-1 document contains exactly:

```json
{
  "schemaVersion": 1,
  "workItems": {
    "newSpecs": [
      { "spec": "checkout", "summary": "Add checkout", "dependsOn": [{ "spec": "cart" }] }
    ],
    "specUpdates": [{ "spec": "cart", "summary": "Update cart" }],
    "directChanges": [{ "id": "docs", "summary": "Update docs" }]
  },
  "body": "# Roadmap\n\n## Overview\n\nDeliver checkout.\n"
}
```

- The root permits only `schemaVersion`, `workItems`, and `body`; `schemaVersion` and `workItems` are required and `body` is optional.
- `schemaVersion` is the integer `1`.
- `workItems` uses the exact Decision 0046 grammar in transient camelCase form. Every accepted rule applies unchanged: a category appears only when non-empty, at least one category is present, identities are unique across `newSpecs` and `specUpdates`, dependency targets exist, and self-references and cycles are invalid.
- `body` is the free-form Markdown Roadmap body. The agent owns that prose under Decision 0046, so the candidate carries it exactly as Decision 0087 carries the review assessment. When `body` is omitted, creation writes a minimal deterministic body and a scope update preserves the current body rather than discarding authored prose.
- The candidate cannot supply `milestoneId`, `baselineRevision`, `targetRelease`, per-item `status`, or any timestamp. The CLI derives identity and baseline itself, and the target release is bound only through the Decision 0072 command.
- Invalid JSON, unknown fields, unsupported versions, and grammar violations return the command's failure code without mutation.

### Creation

`create` requires:

- a Git repository with at least one commit, and a completely clean repository state under Decision 0054, including no untracked files or dirty submodules
- no existing active Roadmap
- a valid non-empty scope whose dependency graph is a DAG
- for every `newSpecs` item, no conflicting existing Spec directory
- for every `specUpdates` item, an existing persistent Spec that is idle, with the Contract that Decision 0054 requires as its before-state

It then generates a Decision 0043 UUID v7, captures the full `HEAD` commit object ID as `baseline_revision`, sets `target_release: null`, writes `steering/roadmap.md`, and initializes every participating Spec's `active_change` with the milestone ID, `state: requirements`, `requirement_ids: null`, and no gate evidence. A `newSpecs` item's Spec directory and `spec.yaml` are created; a `specUpdates` item's existing `spec.yaml` gains its `active_change`.

The CLI creates machine state only. It materializes no Brief, Requirements, Contract, Design, or Research Markdown. Those artifacts are authored afterwards by the responsible skill from the Decision 0059 templates, which exist precisely so an agent reads their `specbind:instruction` guidance. A newly created milestone is therefore expected to report missing prose artifacts until that authoring completes, and `spec status` names the outstanding artifact rather than treating it as corruption.

A successful creation returns `OK MILESTONE_CREATED`, exits zero, and reports the milestone ID, baseline revision, and participating counts:

```text
OK MILESTONE_CREATED: Created milestone 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62.
  Baseline revision: 0123456789abcdef0123456789abcdef01234567
  New specs: 1
  Spec updates: 1
  Direct changes: 1
```

### Scope update

`update-scope` replaces the current `work_items`, and replaces the Markdown body only when the candidate supplies one. It requires an existing active Roadmap, the same Decision 0046 grammar, and a valid DAG. It never changes `milestone_id`, `baseline_revision`, or `target_release`.

- Items added by the update are initialized exactly as creation initializes them.
- Completed Direct status accepted under Decision 0047 is preserved for any Direct item retained by identity, so a scope edit cannot silently reopen finished work.
- The command refuses to drop a participating Spec that currently holds an active change, and refuses to drop a completed Direct item. Those removals require the deferred Decision 0005 reconciliation surface.
- Unlike creation, `update-scope` does not require a clean repository, because ordinary milestone work is in progress. It applies Decision 0081 path safety and refuses a dirty or staged `steering/roadmap.md` or any `spec.yaml` it would initialize.
- When the normalized Decision 0055 Spec-backed scope projection changes, the command removes the accepted `state/cross-spec-review.md`; a Direct-only projection change does not.
- An identical submitted scope and body return `NO_CHANGE MILESTONE_SCOPE_UNCHANGED`. A successful mutation returns `OK MILESTONE_SCOPE_UPDATED` and reports the same counts as creation plus whether the accepted review was removed.

### Rebaseline

`rebaseline` implements the explicit, user-confirmed operation that Decision 0054 requires and never infers. It requires an existing active Roadmap, a completely clean repository, and one explicit full lowercase commit object ID that exists in this repository and is an ancestor of current `HEAD`. Abbreviated hashes, branch names, tags, and symbolic revisions are rejected.

It replaces `baseline_revision`, changes nothing else in the Roadmap, and removes the accepted `state/cross-spec-review.md`. An identical current baseline returns `NO_CHANGE MILESTONE_BASELINE_UNCHANGED`; a successful mutation returns `OK MILESTONE_REBASELINED`.

### Failure and orchestration boundary

- Each command has one top-level failure code — `ERROR MILESTONE_CREATE_FAILED`, `ERROR MILESTONE_SCOPE_UPDATE_FAILED`, or `ERROR MILESTONE_REBASELINE_FAILED` — emits the underlying stable diagnostics as details, exits nonzero, and performs no partial mutation. Repository, scope-grammar, dependency, Spec-state, target-path, and Git diagnostics stay distinguishable.
- Every mutation that touches more than one file is ordered and retry-safe in the same sense as release finalization: the Roadmap is written last, so an interrupted run leaves no active milestone claiming participants that were never initialized.
- `specbind-discovery` owns understanding the request, choosing Spec-backed versus Direct routing, naming Specs, and confirming scope with the user. It invokes these commands only after that confirmation, and authors Brief and initial artifact content afterwards.
- The CLI owns identity generation, baseline capture, grammar and DAG validation, lifecycle guards, persistence, concise English results, and process exit status.
- V1 returns no general JSON response under Decisions 0067 and 0074.

## Consequences

- A project can reach every lifecycle state through public commands, closing the last structural gap in the v1 CLI surface.
- Milestone identity and baseline stay CLI-owned, so no caller can weaken the Contract-diff anchor by supplying its own values.
- The clean-repository rule keeps its meaning, because creation writes only machine state and never competes with agent-authored prose.
- Scope edits become safe by construction: additions initialize correctly, completed Direct work survives, and removals that need content reconciliation are refused rather than silently applied.
- Rebaseline gains the explicit command Decision 0054 assumed, with its review-invalidating effect visible in one place.
- The Spec state machine's `SPEC_CREATED` and `CHANGE_STARTED` guard wording now reads Brief availability as an agent-workflow obligation after the guarded mutation rather than a CLI precondition.

## Implementation status

Implemented. `tools/specbind/src/milestone/` owns the three transitions. The scope candidate loader decodes the strict version-1 camelCase document, and every work-item rule is enforced by rendering the Roadmap and validating it through the authoritative parser, so a written Roadmap is exactly what the parser accepts. Creation requires a clean committed repository and no active Roadmap, generates the UUID v7, captures `HEAD` as the baseline, initializes each participating `spec.yaml` in `requirements` state, and writes the Roadmap last. Scope update preserves the body when the candidate omits it, carries completed Direct status across retained identities, refuses to drop an active Spec or a completed Direct item, initializes added participants, and removes the accepted review only when the Spec-backed projection changes. Rebaseline validates an explicit full ancestor revision against a clean repository, preserves the body and scope, and always removes the accepted review. No command materializes Brief, Requirements, Contract, Design, or Research Markdown.
