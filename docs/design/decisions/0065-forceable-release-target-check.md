# 0065: Expose guarded release finalization with a narrow force override

Status: Accepted

## Context

Decision 0064 limits Git cleanliness checks to the paths the Rust CLI will mutate during release finalization. A path may still be intentionally dirty in a project-specific workflow. The CLI can diagnose that deterministic conflict, but it cannot decide whether discarding, merging, or superseding the local state is acceptable.

The final choice belongs to the human working with the agent. SpecBind therefore needs a normal guarded command that explains the conflict and an explicit override that skips only this one protective check without weakening lifecycle or history invariants.

## Decision

- The accepted mutation command is:

  ```text
  specbind release finalize [--json] [--force]
  ```

- Without `--force`, the command resolves the complete finalization mutation set, runs every core guard, and performs the Decision 0064 target-path Git check before writing.
- If a forceable target-path conflict exists, the command exits nonzero and performs no mutation. Human output lists each affected path, its planned operation, and its Git state. JSON output returns the same facts through stable diagnostics.
- The initial forceable diagnostic is `FINALIZE_TARGET_DIRTY`. Its structured fields are:
  - `code: "FINALIZE_TARGET_DIRTY"`
  - `severity: "error"`
  - display-safe `message`
  - SpecBind-root-relative POSIX `path`
  - `operation`: `create`, `modify`, `delete`, `move_source`, or `move_destination`
  - `git_status`: a non-empty set drawn from `staged`, `unstaged`, and `untracked`
  - `forceable: true`
- Multiple conflicts are reported together when they can be resolved safely without mutation. The human and JSON renderings come from the same structured result.

## Force boundary

- `--force` skips only forceable `FINALIZE_TARGET_DIRTY` checks for the resolved mutation set. It does not disable or downgrade:
  - schema and OKF validation
  - lifecycle-state and roadmap-membership guards
  - release-version binding
  - gate freshness and task completion
  - accepted cross-spec review requirements
  - path containment and symbolic-link safety
  - required-source existence
  - archive identity, content, and overwrite protection
  - atomicity and idempotency checks
- A pre-existing roadmap or cross-spec-review archive collision is never forceable. `--force` cannot overwrite released history.
- `--force` does not reset, stash, stage, commit, back up, or otherwise preserve affected working-tree content. It authorizes the normal finalization mutation even though the reported target paths contain uncommitted state.
- Direct human use of `--force` is explicit override authority. An agent must first present the forceable diagnostics and obtain explicit user confirmation for the affected paths; `--non-interactive`, delegated gate approval, or adapter prose does not authorize force automatically.
- A rejected normal invocation and a rejected or successful forced invocation do not add a separate force field to `spec.yaml` or `log.md`. The command result reports `forced: true` when the override was actually used.

## CLI and workflow behavior

- `specbind-release` invokes `specbind release finalize` normally after applicable project release work is judged successful and the Decision 0066 per-spec log summaries are prepared.
- On `FINALIZE_TARGET_DIRTY`, the skill reports the complete affected-path summary. It may help the user inspect and resolve those paths, or—with explicit confirmation—retry the same current finalization using `--force`.
- The CLI independently rediscovers targets and reruns every non-forceable guard on the forced retry. The earlier diagnostic output is not mutation authority and no stale path list is trusted.
- Exact log-summary arguments and the broader diagnostics envelope remain separate CLI-contract work; this decision fixes the command name, override semantics, and forceable diagnostic payload.

## Consequences

- The safe default remains no mutation when SpecBind-owned finalization targets contain uncommitted work.
- Unusual project workflows have an intentional escape hatch without a universal clean-worktree policy.
- Humans and agents receive enough structured information to judge the actual risk before overriding it.
- `--force` cannot become a general bypass for release gates or archive protection.
