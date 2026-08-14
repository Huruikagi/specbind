# 0043: Use UUID v7 milestone IDs

Status: Accepted; supersedes Decision 0042

## Context

Decision 0042 selected a project-local natural-number sequence allocated through `.specbind.json`. That works only when milestone creation is serialized against one shared mutable counter. Git branches and worktrees do not share such a counter: a mainline milestone and a hotfix milestone can independently read the same next sequence and allocate the same ID before either branch sees the other's commit.

SpecBind must support branch-local milestone creation without a central allocation service. The identity also needs to remain stable before release binding and across roadmap archival.

## Decision

- `milestone_id` is a UUID version 7 generated locally by the Rust CLI.
- Persist the canonical lowercase hyphenated representation, for example `0198b2d1-7c4a-7e31-9f42-8e7c3a110d62`.
- Structural validation requires the version nibble `7` and RFC 4122/RFC 9562 variant bits through the pattern `^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$`.
- Generation uses a maintained UUID v7 implementation backed by operating-system randomness. SpecBind does not hand-roll timestamp, random-bit, clock-rollback, or same-millisecond sequencing logic.
- UUID v7's timestamp component may support convenient display ordering, but lifecycle correctness and release ordering never depend on lexicographic or timestamp order of milestone IDs.
- Milestone generation requires no project counter, registry, network service, branch name, or release version.
- `.specbind.json` has no `nextMilestoneSequence` field. Decision 0042's proposed project-setting mutation and sequence reservation are withdrawn before implementation.
- Independently created branch or worktree milestones receive distinct IDs. If their active roadmaps are later combined, ordinary milestone-scope reconciliation still applies; UUID uniqueness does not automatically merge two active milestones.
- Decision 0042 was an unimplemented target-design decision. Target tooling provides no compatibility alias for `m-<N>` IDs; encountering one in target metadata is a validation or explicit migration concern rather than normal v1 input.

## Consequences

- Hotfix, release, and mainline branches can allocate milestone identities independently without collision-prone shared state.
- Milestone IDs are longer and less human-memorable than a natural sequence, but users normally interact through roadmap scope and release names.
- `.specbind.json` remains ordinary persisted configuration rather than mixed configuration and lifecycle-allocation state.
- Release version remains the human-facing historical key; UUID v7 remains secondary trace identity.
