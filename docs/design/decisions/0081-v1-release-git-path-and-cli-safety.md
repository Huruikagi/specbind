# 0081: Tighten v1 release, Git, path, and CLI safety

Status: Accepted

## Context

Earlier decisions allow release finalization to overwrite dirty target paths with `--force`, describe multi-file finalization as atomic, and require an empty log-entry JSON document for Direct-only milestones. Git is now mandatory and is the recovery mechanism, so destructive bypasses and bespoke backups are unnecessary. The text-only CLI also needs explicit warning, stream, and terminal-safety rules.

## Decision

### Release finalization

- V1 removes `specbind release finalize --force`. Every path that finalization will create, update, move, or delete must be Git-clean; unrelated dirty source paths remain allowed.
- Archive collisions are always errors unless an idempotent retry proves the already-finalized identity and complete content.
- `--log-entries <path|->` is required when at least one Spec-backed item participates. It is optional for Direct-only milestones; if provided there, only an explicit empty array is valid.
- Direct-only release archives only the final Roadmap. A Spec-backed release additionally archives the accepted cross-spec review.
- Project publication and verification occur before core finalization. Git tags or packages may therefore point to the verified implementation revision and exclude the later SpecBind log, archive, idle-state, Brief, Research, and Tasks cleanup commit. These are post-publication lifecycle metadata in v1.
- Multi-file finalization is a validated, ordered, idempotent logical transaction, not a promise of crash-atomic filesystem replacement:
  1. validate all guards and render all outputs before mutation;
  2. update per-Spec logs and lifecycle artifacts;
  3. archive the cross-spec review when present;
  4. archive the active Roadmap last as the completion marker.
- A retry while the active Roadmap remains finishes idempotently. When no active Roadmap exists, an exact archive and final-state match returns `NO_CHANGE RELEASE_ALREADY_FINALIZED`; inconsistent partial state stops for Git-assisted recovery.
- CLI-authored release-log titles and wrapper prose use `.specbind.json.language`; machine IDs and references remain fixed tokens. Existing log titles are preserved.

### Git and managed paths

- SpecBind lifecycle requires Git. Project installation requires being inside a Git repository but may precede the first commit; milestone creation requires at least one commit and a fully clean repository, including untracked files and dirty submodules.
- SpecBind performs no independent backup, reset, stash, automatic unrelated commit, or `.git/info/exclude` mutation.
- Every managed artifact must be tracked or visible to Git as untracked. Installation rejects new managed targets hidden by repository or global ignore rules; tracked files remain valid even when a later ignore pattern matches them.
- `specDir` is a portable project-root-relative child directory such as `.specbind` or `docs/specs`. Absolute paths, escape through `..`, and `.` itself are invalid.
- A parent project cannot place `specDir` inside a nested submodule. A submodule root may independently be a SpecBind project. Implementation Tasks may modify source submodules.
- SpecBind-managed files and every intermediate mutation directory must be real files/directories. Artifact discovery does not traverse symlink or Windows junction directories; managed symlink artifacts and mutation paths are errors.
- Managed relative paths reject absolute roots, `.` and `..` segments, control characters, Windows-reserved characters and device names, trailing spaces or periods, and ASCII case-insensitive collisions. Titles and body prose remain Unicode.
- Front Matter round trips preserve unknown top-level keys and semantic values, not comments, whitespace, quoting style, anchors, aliases, custom tags, or original key order. Mutations emit canonical YAML and preserve an otherwise untouched Markdown body.

### CLI process contract

- Lifecycle commands are non-interactive. Skills gather confirmation and pass explicit arguments. Initial project installation may prompt for missing agent, language, root, and project-instruction choices in a TTY; non-TTY use requires explicit values.
- Process exit `0` covers `OK` and `NO_CHANGE`; process exit `1` covers every v1 `ERROR`. Stable text codes carry detailed classification.
- `specbind check` without a focused subcheck aggregates independent diagnostics rather than failing fast. Warnings do not create a fourth top-level outcome:

  ```text
  OK CHECK_COMPLETED_WITH_WARNINGS: Check completed with 2 warnings.
    WARNING FILE_OWNERSHIP_OVERLAP: ...
  ```

- Warning-only checks exit zero. Any error produces `ERROR CHECK_FAILED`, includes all safely collectable diagnostics, and exits one. Mutation commands stop at guard failure.
- Success, no-change, read-model data, help, and version use stdout. Errors, details belonging to errors, hints, usage errors, and any progress use stderr. A failed raw read leaves stdout empty.
- Non-raw output escapes newlines, control characters, and ANSI escape sequences from user-authored values. Skills branch on the first outcome token and stable code. Raw single-artifact reads remain byte-preserving UTF-8 content on stdout.

## Consequences

- Decision 0065 is superseded. Decisions 0064, 0067, and 0068 remain only where consistent with this decision and Decision 0076.
- Git-clean targets and deterministic retry replace destructive force and backup machinery.
- The release contract is implementable on ordinary filesystems without making an untrue atomicity claim.
- Text-only CLI output remains compact while supporting warnings, safe piping, and stable agent branching.

