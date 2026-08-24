# 0141: Guard agent removal and project uninstall behind exact plans

Status: Accepted

## Context

Decision 0077 makes installation additive and deliberately leaves agent removal
and project uninstall to later work. Removing generated integration assets is
not the inverse of installation: the configured SpecBind root contains durable
project-owned specifications, settings, steering, lifecycle state, and release
history, while root instruction files may mix a marked product block with
unrelated project guidance.

Git provides recovery, but recoverability alone does not identify ownership or
authorize a guessed deletion. A safe operation must name every product target,
distinguish durable knowledge from generated integration, and make the
destructive transition explicit.

## Decision

### Command boundary

Two top-level commands own project removal:

```text
specbind remove-agent <claude-code|codex>
specbind remove-agent <claude-code|codex> --apply

specbind uninstall --knowledge <retain|remove>
specbind uninstall --knowledge <retain|remove> --apply
```

Without `--apply`, each command is a read-only plan. The plan is the confirmation
surface for a human or agent: it reports every exact target as `remove`,
`update`, `retain`, or `absent`, its ownership category, and Git recovery. There
is no implicit TTY prompt, `--yes`, force mode, or default knowledge policy.
`--apply` recomputes the plan and applies only that current result.

These commands remove project integration. They never remove the machine-level
binary, edit PATH, uninstall a package-manager entry, or infer how the binary
was installed.

### One-agent removal

`remove-agent` removes one currently selected host while retaining the project
and every other selected host. It targets only:

- the current closed embedded Skill catalog rendered below that host's exact
  skill root;
- the five current exact product role files for that host;
- the marked Decision 0099 block in that host's root instruction file; and
- that host's `agents` entry and `agentRoles` object in `.specbind.json`.

Directories and content outside those exact catalog paths are retained. An
older or unknown asset name is not guessed from a `specbind-` prefix. The
configuration is updated last and is the completion marker. Removing the last
selected agent is rejected; complete removal uses project uninstall so the
durable-knowledge policy cannot be skipped.

### Project uninstall and durable knowledge

Project uninstall removes the same exact assets for every configured agent,
then removes `.specbind.json` last. It requires one explicit policy for the
configured exact `specDir`:

- `retain` preserves the complete directory, including project-owned settings,
  Specs, Steering, active Roadmap, review state, deferred findings, logs, and
  release history.
- `remove` removes the complete directory as one durable knowledge bundle. This
  is explicit knowledge deletion, not classification of those artifacts as
  disposable install output.

The configured path, not a default name or prefix search, identifies the
bundle. The plan reports the retained or removed bundle as a separate ownership
category.

### Root instructions

Removal recognizes only the exact Decision 0099 marker pair. It removes the
marked region and preserves every byte outside it, including surrounding
whitespace. Malformed, repeated, or reversed markers stop the plan. When the
file contains only the managed block, the complete tracked file is removed;
otherwise it is atomically replaced with the surrounding project content.

### Git and filesystem guards

Every mutation requires a repository with at least one commit. Exact file
targets must be Git-tracked, non-ignored regular files reached only through
non-link path components. Knowledge removal recursively requires every entry to
be tracked, non-ignored, and free of symlinks, junctions, reparse points, special
files, and nested path ambiguity. Targets must stay below the project root.

Unrelated staged, modified, deleted, or untracked state stops planning. A retry
may recognize only the exact worktree deletions and exact marked-block result
that an earlier apply could already have produced. `absent` is therefore a
deterministic completed action, not permission to broaden discovery. Every
remaining mutation is revalidated before writing.

Git is the recovery mechanism. The CLI creates no backup tree and never commits
the removal. The plan tells the caller that removed tracked content is
recoverable from the pre-apply revision.

## Consequences

- Agent removal cannot silently become project uninstall.
- Project uninstall cannot silently discard or silently preserve durable
  knowledge; the caller chooses either policy.
- Unknown historical or project-owned content survives because the CLI does
  not infer ownership from names.
- The config-last order distinguishes an interrupted operation from a completed
  uninstall and gives retries a deterministic boundary.
- Users who intend a complete cleanup can remove `specDir` safely without
  leaving an unexplained orphaned knowledge tree.

## Implementation status

Implemented by `tools/specbind/src/installation/removal.rs`, the
`remove-agent` and `uninstall` command surfaces, focused Git-fixture integration
tests, and the public uninstall guide.
