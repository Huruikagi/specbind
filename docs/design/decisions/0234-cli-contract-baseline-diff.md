# 0234: CLI-owned Contract baseline comparison

Status: Accepted

## Decision

Add the read-only `contract diff --baseline <revision> [--json]` command. Resolve
the explicit revision once to a commit and compare it with the current worktree,
including uncommitted proposals. Use the currently configured `specDir` at both
ends; never infer renamed artifacts. The comparison does not require an active
Milestone, but Contract Review must supply its reported Milestone baseline.
If baseline configuration exists, its `specDir` must match the current root;
reject a moved root rather than misclassifying existing Specs as newly added.
A baseline before initial configuration is allowed.

Read the union of persistent Spec directories at both ends, with the fixed
`specs/<spec>/contract.yaml` singleton, and the optional canonical or legacy
shared Contract. A new or removed Spec directory is distinct from an existing
Spec lacking its required Contract. Invalid identities, unsafe file types,
missing required files, malformed documents, duplicate shared paths, and
unreadable revisions fail without a partial report. Read committed blobs through
Git without materializing a checkout or executing filters. The shared lifecycle
assessment reuses the same historical shared-manifest reader.

Use the existing fingerprint normalization for stable IDs, entry order and
unordered path/invariant lists. Preserve descriptions and schema versions.
The JSON report includes resolved baseline/current revisions, `currentSource`
(`worktree`), a normalized current-input fingerprint, configured root, and every
document's exact before/after paths and normalized content. Document changes
include added, changed, removed and unchanged; this preserves empty-manifest and
schema-only changes. Changed entries carry exact selectors, before/after values,
and separate sorted baseline/current consumer lists.

Consumers are declared direct managed references, including references whose
targets no longer exist. The diff returns complete graph diagnostics for each
side, but reference errors, ownership warnings and cycles do not suppress the
comparison. Exit zero means comparison succeeded, not that the graph or review
passed. Missing or malformed required manifests still prevent a complete read.
No external-consumer discovery, transitive impact closure, compatibility verdict,
scope authorization, gate approval, commit or file mutation is introduced.

Recheck HEAD, resolved configuration and normalized current inputs before
returning. This detects changes during the read; it is not a concurrency lock or
an acceptance token. A later input change requires a new comparison and review.
The command reads no Tasks, lifecycle metadata, Requirements or Design content.

Contract Review replaces its manual historical `git show` loop with this output,
still reads all current Contracts, checks the whole current graph and compares
Roadmap behavior with unchanged Contracts. The reviewer retains semantic
judgment, external consumers, deep inputs, findings and existing remediation
authority. Neither an empty diff nor successful comparison establishes approval.

## Verification

Cover new/removed Specs, missing baseline/current Contracts, malformed input,
custom roots, dirty proposals, normalization, shared legacy/canonical paths,
empty-manifest transitions, changed/removed targets with old/current consumers,
unsafe committed blobs, incomplete historical reads and deterministic output.
Behavioral X1 and X2 runs must discover the installed CLI, preserve downstream
artifacts, and retain the existing semantic findings and authority boundaries.
