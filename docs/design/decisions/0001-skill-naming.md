# 0001: Replace inherited skill naming

Status: Superseded by Decision 0075

## Context

The current generated skills use the inherited `kiro-` prefix. SpecBind is becoming an independent command system, so its public skill names should not imply that the inherited interface is the permanent product interface.

Skill names also appear in:

- Claude Code and Codex skill directories
- Skill-to-skill instructions and suggested next commands
- Generated `CLAUDE.md` and `AGENTS.md` guidance
- Tests, examples, and package documentation

The naming decision therefore needs an explicit migration policy, not only a directory rename.

## Direction

Replace the `kiro-` prefix with `specbind-` across the generated skill set. Final suffix vocabulary remains subject to the individual skill design review.

The [target skill catalog](../target-skill-catalog.md) uses `specbind-` for all working target names.

## Before and after pattern

| Current | Target working name |
| --- | --- |
| `kiro-discovery` | `specbind-discovery` |
| `kiro-spec-init` | `specbind-spec-init` |
| `kiro-impl` | `specbind-impl` |

The final mapping will be recorded in the catalog rather than duplicated here.

## Questions to decide

- Should the internal `spec-` grouping remain in user-facing names?
- Should Claude Code slash invocation and Codex skill invocation expose exactly the same base names?
- Are old names removed in one release, retained as aliases for a transition, or handled by a migration command?
- How should existing consumer repositories detect and remove obsolete generated skill directories?

## Consequences to account for

- Update both agent template trees together.
- Update all skill-to-skill references and generated guidance.
- Add tests that reject stale inherited names in maintained outputs.
- Document upgrade behavior for already initialized repositories.
- Keep the current indexes unchanged until implementation actually changes the CLI output.
