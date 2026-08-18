# 0051: Keep the active roadmap current-state only

Status: Accepted

## Context

The active roadmap is an editable manifest of the milestone currently being delivered. Adding an embedded change history, actor log, update timestamps, or review-attempt list would mix current planning state with audit history and introduce another history model that must be reconciled with Git.

SpecBind already has distinct history surfaces: Git records active-artifact edits, the released roadmap archive preserves the final milestone snapshot, and each spec's `log.md` summarizes released results.

## Decision

- `steering/roadmap.md` represents only the current active milestone state.
- Its frontmatter and Markdown body contain no required history collection, `updated_at`, actor log, scope-change log, or contract review-attempt list.
- Confirmed scope and dependency changes update the current `work_items` through guarded CLI operations. The Decision 0054 baseline remains fixed except through an explicit user-confirmed rebaseline.
- A change that invalidates the accepted global contract review removes or supersedes the current Decision 0052 state artifact; the roadmap does not embed the previous accepted record or failed attempts.
- Git history remains the source for active roadmap edits and authorship.
- Successful release archives the final roadmap snapshot under `releases/`.
- Per-spec `log.md` records the released result rather than the active planning sequence.
- A separate project steering log may be considered later for audit-heavy projects, but it is not part of the v1 roadmap contract.

## Consequences

- Agents and humans read one unambiguous representation of current milestone intent.
- CLI mutations do not need to maintain a second event log inside the roadmap.
- Historical reconstruction before release depends on the repository's Git practices.
- Release evidence remains durable in its companion archive without carrying transient review failures or superseded scope inside the roadmap snapshot.
