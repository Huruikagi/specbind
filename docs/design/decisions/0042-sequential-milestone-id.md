# 0042: Use a project-sequential milestone ID

Status: Superseded by Decision 0043

> This sequence-based design is retained as decision history. Decision 0043 replaces it with UUID v7 because Git branches and worktrees cannot safely share the proposed project-local counter.

## Context

Milestone identity must remain stable before a release version is known and after an active roadmap is archived. UUID-style identifiers avoid centralized allocation but are unnecessarily long for a project that permits only one active milestone. A positive project sequence is easier to read in status, changelogs, diagnostics, and archived roadmaps.

A sequence requires a durable allocation source. Active roadmaps are eventually renamed by release version, and abandoned roadmaps are not archived by default, so deriving the next value only from roadmap filenames or surviving roadmap content could reuse an abandoned ID.

## Decision

- A milestone ID is the string `m-<N>`, where `<N>` is an unpadded positive base-10 integer. Its schema pattern is `^m-[1-9][0-9]*$`.
- IDs are allocated monotonically within one project: `m-1`, `m-2`, and so on. Numeric ordering uses the integer suffix, not lexical string ordering.
- Project-root `.specbind.json` stores the next unallocated positive integer in the CLI-managed reserved field `nextMilestoneSequence`.
- `nextMilestoneSequence` is project lifecycle allocation state embedded in the existing settings file, not a user preference. It has no CLI flag, environment override, or config-precedence fallback.
- Milestone creation reserves the current value by advancing and durably writing the counter before publishing the new active roadmap. A failure after reservation may leave a gap; gaps are valid and reserved values are never reused.
- Successful release, abandonment, or scope removal never decrements the counter.
- The guarded milestone operation preserves unrelated `.specbind.json` fields and rejects a concurrent or inconsistent counter mutation instead of silently choosing another identity.
- For an existing project without `nextMilestoneSequence`, migration inspects parseable `m-<N>` IDs in the active and archived roadmaps. It initializes to one greater than the maximum, or `1` when none exist. Ambiguous, duplicate, or conflicting state requires explicit repair rather than guessed reuse.
- This v1 placement avoids another project artifact. A later storage migration may move the counter while preserving every allocated `milestone_id`.

Example after allocating `m-12`:

```json
{
  "specDir": ".specbind",
  "nextMilestoneSequence": 13
}
```

## Consequences

- Milestone IDs remain short and recognizable while release versions stay independently bindable.
- Abandoned or failed milestone creation can create harmless sequence gaps.
- Concurrent milestone creation is intentionally serialized through guarded project configuration mutation.
- `.specbind.json` contains one CLI-managed lifecycle field alongside ordinary persisted settings; tooling must not expose it as an overridable preference.
