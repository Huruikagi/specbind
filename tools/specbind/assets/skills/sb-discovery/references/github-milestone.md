# GitHub Milestone Source Collection

Use this procedure only when the maintainer explicitly supplies canonical
`OWNER/REPO` and a Milestone number, or exactly
`https://github.com/OWNER/REPO/milestone/NUMBER`, as Discovery input. The URL
form is a complete explicit selector: parse its owner, repository, and decimal
Milestone number before acquisition, then use them exactly as if they had been
stated separately.

Accept no other URL shape. A different host, missing component, extra path,
query, fragment, percent-encoded path component, or non-decimal number stops
for clarification. Do not search, normalize, redirect, or infer identities from
a near match or a similarly named Milestone.

Read the shared semantic contract once before acquisition:

```sh
specbind protocol read source-material
```

## Acquire one complete, read-only snapshot

Prefer an available authenticated GitHub integration. Use it only to read the
selected repository, numbered Milestone, and every entry assigned to that
Milestone. If no such integration is available, use authenticated `gh` as the
fallback. Verify authentication first and stop on failure:

```sh
gh auth status
gh api repos/OWNER/REPO
gh api repos/OWNER/REPO/milestones/MILESTONE_NUMBER
gh api --paginate --slurp 'repos/OWNER/REPO/issues?milestone=MILESTONE_NUMBER&state=all&per_page=100'
```

Replace only the explicit identities parsed from the separate input or exact
URL; do not turn these read commands into a write. The `--paginate --slurp`
result is one logical inventory: retain every
page and stop if any page is unavailable or cannot be accounted for. Confirm
that the returned canonical repository and numbered Milestone match the request.
A missing, inaccessible, or ambiguous identity is partial acquisition, not an
empty collection.

Before reading content, inventory every returned entry. For each record:

- retain stable repository identity and URL, Milestone number, title, and URL;
- retain entry number and URL, title, state, and observed update time;
- identify an entry with `pull_request` metadata as a non-Issue item; and
- read an Issue body only when it is an actual Issue and needed for
  classification.

Do not read comments or timeline events. Labels, state, author, assignees, and
Milestone metadata are routing evidence, never automatic ownership or
specification authority. Do not issue any GitHub mutation command.

Report every known inaccessible, unsupported, duplicate, excluded, non-Issue,
or unresolved entry together. An inaccessible Issue, incomplete pagination, or
other partial acquisition stops before classification, Gate invalidation,
milestone mutation, or Brief authoring. Do not continue with the entries that
happened to work.

## Classify with complete Issue coverage

Treat each actual Issue as request context and apply the entry and ownership
rules from [ordinary change Discovery](ordinary.md). Several Issues may inform
one work item; one Issue may inform several Specs. Do not create a Spec per
Issue. A pull request or another non-Issue entry remains visible as excluded
with its reason.

The confirmation payload keeps the ordinary four fields and adds:

```text
Source coverage: <OWNER/REPO; Milestone number/title/URL; every Issue and
non-Issue entry; observed update time; included, excluded, duplicate, or
unresolved disposition; relevant work items; and one-line reason>
```

An unresolved item means the proposal is not approvable. Ask the maintainer to
settle it before changing state.

## Preserve the capture

When creating the Roadmap, fill the selected body with complete GitHub
provenance and coverage mapping as well as ordinary request, decomposition, and
dependency rationale. With an active milestone, read it with
`specbind milestone scope --include-body` and merge the mapping into complete
current prose.

For each Spec-backed Brief, name only the exact Issue URLs and observed metadata
relevant to that Spec, and state why each matters. A shared Issue appears in
every relevant Brief. Direct items still have no Brief. Do not paste a complete
Issue collection into every Brief.

The Brief is the approved remote request context for later planning. Requirements
and Design promote its captured meaning into canonical artifacts; they do not
silently re-query GitHub or let a later Issue edit reinterpret approved work.
