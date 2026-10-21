# 0235: CLI-owned worktree observations and change checks

Status: Accepted

## Decision

Add read-only `worktree capture` and `worktree check --snapshot <file|->`
commands, independent of project configuration. Capture emits transient JSON
using the generated `worktree-snapshot/v1` schema. The caller preserves exact
output in memory or outside the worktree; no persistent product ledger or
snapshot writer is introduced. Input files must be ordinary repository-external
files. The snapshot is not authenticated and must not be edited or fabricated.

Bind observations to the canonical worktree root and Git directory, and record
the committed HEAD. Observe the union of HEAD paths, index paths and nonignored
untracked paths. Check also retains every snapshot path, including untracked
files subsequently ignored. Compare raw SHA-256 content and executable state
separately from index mode and object ID. Existing dirty paths are recorded so
additional edits to them are visible even when Git status has not changed.
Renames appear as deletion plus addition. File/directory replacements retain
the previous path and observe new nonignored children.

Check accepts repeated `--changed` exact project-relative paths. Reject duplicate
or unsafe paths; do not expand directories or glob syntax. Report before/after
states, separate index/worktree changes, unexpected paths, touched previously
dirty paths, declared unchanged paths and HEAD movement. Exit zero with
`WORKTREE_CHECK_MATCHED` means no unexpected observed paths and unchanged HEAD;
exit one with `WORKTREE_CHECK_MISMATCH` returns a complete JSON mismatch report.
Read/input errors return diagnostics without partial JSON. Declared unchanged
paths are visible but not a mismatch. Neither result proves semantic correctness,
authorship, preservation within a declared file or authority to stage/commit.

Read Git with optional locks and fsmonitor disabled. Never refresh the index,
checkout, stash, reset, delete, write a snapshot or run external diff commands.
Observe twice and require equal results to detect changing inputs; this is not
an atomic filesystem snapshot or concurrency lock. A later change needs another
check. Ordinary ignored files, Git metadata and empty untracked directories are
outside coverage. Reject unborn HEAD, unsupported paths, links/reparse points,
nonregular files, submodules, nested untracked repositories, unresolved stages,
intent-to-add, sparse/skip-worktree and assume-unchanged entries rather than
presenting incomplete observations as a match. Raw bytes are not Git's filtered
blob contents. Executable worktree bits are measured on Unix; Windows reports
false and still compares the Git index mode.

Initially integrate only the Spec-backed implementer's return boundary. Capture
immediately before each implementation dispatch, retain the output, and check
the structured `CHANGED` paths before accepting `READY_FOR_REVIEW`. A mismatch
or error prevents review and Task completion. The owner retains diagnosis,
Task recovery and ownership judgment. Existing recovery rules govern removal of
proven generated outputs. A new authorized recovery dispatch may capture again,
but cannot clear prior findings or partial paths. Touched earlier work requires
inspection even after a matched result. This does not change Direct, validation,
checkpoint or approval authority, or implement a recovery-ledger command.

## Verification

Exercise existing staged/unstaged edits plus same-path changes, binary and Unicode
paths, staging-only changes, renames, addition/deletion, file/directory replacement,
new ignore rules, HEAD movement, exact declarations, strict snapshot input,
repository identity, unsupported Git states, and unchanged index bytes.
Generate and conform the schema. Behavioral I1 and I4 must discover the installed
commands, complete the task through existing review/progress rules, and preserve
unrelated dirty work. Record actual driver coverage and environment limitations.
