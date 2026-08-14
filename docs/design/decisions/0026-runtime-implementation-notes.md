# 0026: Keep implementation notes as persistent free-form Markdown

Status: Accepted

## Context

The inherited `## Implementation Notes` section is a collection of cross-task learnings discovered during implementation. Its primary consumer is a later AI implementation or validation run, not SpecBind's lifecycle engine.

Some learnings remain useful after the active milestone ends. Storing them in `tasks.yaml` would couple persistent implementation knowledge to an artifact that is intentionally replaced between milestones and removed by release finalization. Adding structured Task ID links, authors, timestamps, or note categories would also constrain useful prose without improving the core purpose: preventing later agents from repeating mistakes.

## Decision

- A spec may contain zero or more `SpecBind Implementation Notes` artifacts alongside its persistent requirements, design, and contract artifacts. Decision 0057 discovers them by OKF type; each has a stable `artifact_id`.
- Notes are optional and are created only when useful implementation knowledge needs to be retained. Multiple artifacts may separate materially different knowledge areas without forcing one growing file.
- Each implementation-notes body is free-form Markdown. Apart from OKF profile metadata, SpecBind defines no required headings, entry schema, Task ID links, authors, timestamps, categories, or stable note IDs.
- Agents may include task references, code examples, commands, failure details, or any other useful context directly in the Markdown. SpecBind does not parse, resolve, validate, or rewrite those contents.
- The file is spec-scoped persistent implementation memory rather than a child of a task or active milestone.
- Task generation, implementation, debugging, review, and implementation-validation workflows read it when present and update it when durable knowledge is discovered.
- Because this is maintained guidance rather than an audit log, stale or incorrect content may be edited or removed.
- Information that applies across specs or defines a project-wide convention should be promoted to the appropriate `steering/` document instead of being duplicated across spec notes.
- Implementation notes do not affect the task-plan fingerprint, task status, gate approval, or lifecycle state.
- A note cannot substitute for `blocked`, a plan revision, a requirements/design rewind, or completion evidence when one of those state changes is required.
- Successful release finalization preserves every discovered implementation-notes artifact unchanged while removing the milestone-local `tasks.yaml`.
- Task CLI projections do not copy or serialize this free-form artifact; agent workflows read it directly when semantic context is needed.

## Migration

- Non-empty content under an inherited `tasks.md` `## Implementation Notes` heading moves to the default `SpecBind Implementation Notes` artifact with `artifact_id: main`, preserving its useful Markdown structure and prose.
- Unstructured Task ID text remains text and receives no generated relationship metadata.
- An empty inherited heading does not create the file.
- After migration, `tasks.yaml` contains no implementation-note field.

## Consequences

- Useful implementation knowledge survives milestone refresh and release finalization.
- Agents can record the context in its natural form without a schema migration for new note shapes.
- Plan renumbering does not attempt unsafe rewrites inside free-form notes.
- Consumers that need reliable dependencies, blockers, or evidence must use the corresponding structured artifacts instead of interpreting note prose.
