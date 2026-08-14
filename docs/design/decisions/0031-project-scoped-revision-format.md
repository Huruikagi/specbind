# 0031: Interpret implementation revisions from project context

Status: Accepted

## Context

Decision 0029 binds accepted completion evidence to a full Git commit object ID. Repeating `vcs`, object format, repository path, or similar metadata in every per-spec completion record would duplicate properties of the project repository that do not normally change within a milestone.

SpecBind v1 supports Git for the completion handshake. Git object format can be detected from the repository, so adding a project setting before another revision provider exists would create configuration without a user choice.

## Decision

- Completion evidence stores `implementation_revision` as one scalar containing the full lowercase hexadecimal Git commit object ID.
- It stores no per-evidence `vcs`, `object_format`, repository path, branch, tag, author, or commit timestamp.
- The repository containing the SpecBind project is the implicit revision context.
- In v1, the CLI detects that repository's Git object format and requires the submitted full object ID to match it. JSON Schema accepts the supported 40- or 64-character lowercase hexadecimal forms; semantic validation selects the one valid for the current repository.
- A branch name, tag name, abbreviated object ID, symbolic `HEAD`, uppercase hexadecimal value, or object ID from another repository is invalid.
- The revision provider and object format cannot change as an ordinary operation within an active milestone. A detected change is a project migration that invalidates affected completion evidence and requires a fresh Decision 0029 validation handshake.
- SpecBind v1 adds no revision-provider setting because Git is the only supported provider. If another provider is introduced, provider selection belongs to project-level configuration and schema evolution rather than each completion record.

Example:

```yaml
implementation_revision: 0123456789abcdef0123456789abcdef01234567
```

## Consequences

- Completion evidence stays compact and avoids repeating stable repository metadata for every spec.
- The CLI, not the agent, determines the valid commit-object representation.
- A project-wide revision-provider migration remains explicit and cannot silently reinterpret existing evidence.
- Future provider support may change the project configuration and evidence schema without adding mixed-provider records inside one milestone.
