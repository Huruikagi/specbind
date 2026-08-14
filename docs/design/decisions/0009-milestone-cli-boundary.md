# 0009: Keep milestone state operations in the CLI, not a separate skill

Status: Accepted

## Context

Every change-bearing milestone needs a roadmap with a stable identity, confirmed scope, an optional release-version binding, and explicit lifecycle transitions. Putting analysis, decomposition, roadmap persistence, version binding, and abandonment into `specbind-discovery` would make that skill too broad.

A proposed `specbind-milestone` skill would separate the name but still express deterministic state operations as another agent prompt. SpecBind now has an accepted Rust CLI boundary for parsing, invariants, and guarded mutations.

## Decision

- Do not publish a separate `specbind-milestone` agent skill.
- Keep `specbind-discovery` as the user-facing entry point for understanding a request, choosing its route, clarifying scope, creating active briefs, and obtaining required confirmation.
- Put deterministic milestone state operations in the Rust CLI.
- Discovery invokes the CLI with explicit confirmed inputs rather than reimplementing file and state mutations in its instructions.
- Other workflows, including release, may call the same CLI contracts when they need milestone state.
- Exact command names remain Draft; `specbind milestone ...` is a working namespace rather than an accepted interface.

## CLI-owned milestone operations

- Require the Decision 0054 clean Git baseline, capture the full current `HEAD`, generate a branch-safe Decision 0043 UUID v7, and create `steering/roadmap.md` with both the stable milestone ID and immutable baseline revision.
- Apply an explicitly confirmed scope and ordering update.
- Apply an explicitly confirmed Decision 0054 rebaseline and invalidate the accepted global cross-spec review; never infer one as repair.
- Bind or explicitly rebind a target release version through `specbind milestone bind-release <version> [--rebind]` under Decision 0072.
- Check consistency among the roadmap and participating specs' active-change metadata.
- Apply the deterministic portion of explicitly confirmed abandonment cleanup after content reconciliation.

## Boundaries

- The CLI does not decide whether work belongs in an existing spec, a new spec, or a direct-change route.
- The CLI does not invent scope, requirements, design, dependencies, or release versions.
- The CLI does not automatically revert code, Git history, requirements, or design.
- Discovery does not duplicate the CLI's parsing, ID generation, consistency checks, or guarded write algorithms.
- Release finalization and project adapter orchestration remain governed by their own release contract.

## Consequences

- Discovery remains a focused semantic workflow rather than a general state manager.
- Milestone invariants are identical across supported coding agents.
- The target skill catalog no longer contains `specbind-milestone`.
- Generated discovery instructions can remain thin by calling stable CLI contracts.
- CLI operations require structured inputs and diagnostics suitable for both agents and direct human use.

## Open questions

- Final command and argument names.
- Which scope updates require user confirmation versus prior roadmap approval.
