# 0101: Group project adapters and add Git workflow guidance

Status: Accepted

## Context

SpecBind already has one project-owned operational adapter. Decisions 0002,
0010, and 0063 place free-form release instructions in
`settings/release.md`, where an agent interprets project-specific preparation,
publication, verification, and cleanup without turning Markdown into a command
language.

Git checkpoint policy has the same shape. Projects differ on whether they commit
after each approved gate, once after planning, per implementation Task, or only
on explicit request. Commit messages, branch conventions, and push policy also
belong to the project, while approval, task completion, and repository safety
remain SpecBind contracts.

Putting this in an authoring rule would cross the boundary in Decision 0093:
rules tune artifact judgment and writing, not workflow mutations. Adding more
single files directly below `settings/` would also leave no clear home for later
project-specific operational guidance.

## Decision

### Adapter directory

Project-owned operational adapters live below:

```text
{{SPEC_DIR}}/settings/adapters/
```

V1 defines two known adapters:

| Selector | Path | OKF type | Presence |
| --- | --- | --- | --- |
| `release` | `settings/adapters/release.md` | `SpecBind Release Adapter` | required |
| `git` | `settings/adapters/git.md` | `SpecBind Git Adapter` | optional at runtime |

The release adapter moves from `settings/release.md`; Decision 0063 otherwise
keeps its free-form profile and semantics unchanged. The old path is not an
alias. No Rust release skill or installed release asset has shipped yet, so this
is a pre-implementation path correction rather than a compatibility migration.
Explicit cc-sdd migration writes any converted release guidance to the new path.

The directory is organization, not an extension loader. A product-managed skill
names every adapter it consumes and reads that exact known path. It does not
scan the directory, infer behavior from filenames, or execute an unknown file.
Adding another product adapter requires an explicit selector, profile, owning
consumers, absence semantics, installation treatment, and conflict boundary.

Each adapter is a project-owned OKF concept. Its known `type` is exact, unknown
top-level Front Matter extensions are preserved under Decision 0045, and its
Markdown body is free-form agent-readable guidance. Headings, lists, and code
blocks are not machine syntax. A code block is never an automatically executable
hook.

### Installation and absence

`specbind install` embeds and creates missing scaffolds for both known adapters,
then treats the project copies as user-owned settings that are never overwritten.
This follows Decisions 0008 and 0077.

The adapters differ when a project removes or has not yet received one:

- Missing `release.md` is the Decision 0063 configuration error. An empty body
  explicitly means no project-specific release actions.
- Missing or empty `git.md` means no adapter-directed commit or push. A core
  operation may still require a clean committed revision; the owning skill then
  reports or requests that prerequisite instead of inferring Git authority.

An install refresh may offer a missing Git scaffold again as an uncommitted
project setting. Removing that addition before committing remains valid.

### Git adapter scope

The Git adapter may guide:

- whether eligible planning work is checkpointed after each gate or once at an
  orchestration boundary
- whether accepted implementation Tasks receive separate commits
- selective staging, commit grouping, and commit-message conventions
- branch conventions and whether a workflow should offer a push

It does not redefine when work becomes eligible for a checkpoint:

- discovery output is eligible only after the confirmed milestone mutation and
  all owed Briefs are complete
- Requirements, Design, and Tasks output is eligible only after the corresponding
  guarded approval succeeds
- an accelerated workflow may combine several delegated gates at its named end
- implementation output is eligible only after the Task's required review or
  verification passes and guarded progress is recorded

Unapproved drafts, rejected work, and a partially written discovery result are
never committed merely because the adapter requests frequent checkpoints.

### Policy is not authority

An adapter records project policy. It grants no commit, push, branch, publication,
credential, or approval authority by being present in the repository.

The consuming skill still applies the user's request, root agent instructions,
tool permissions, and the normal external-mutation boundary. In particular:

- delegated gate approval authorizes crossing only the named gates; it does not
  imply commit or push authority
- commit guidance does not imply push guidance
- a push instruction does not authorize force-push, history rewriting, or a
  protected-branch bypass
- the skill stages only the intended workflow paths and preserves unrelated
  user changes
- ambiguous, unsafe, or conflicting guidance stops before the Git mutation

A failed checkpoint does not roll back or falsify an approval or completed Task.
The skill reports the accepted state as uncommitted and stops when the next core
operation requires a committed revision.

### Relationship to release

Release-specific version commits, tags, publication branches, and pushes remain
the release adapter's responsibility because their timing is part of release
orchestration. The Git adapter supplies general repository conventions and
checkpoint preferences. When both apply, the release adapter is the specific
workflow instruction, but it receives no extra authority and cannot weaken Git
safety. A material conflict between the two stops for clarification.

### CLI boundary

This decision does not make gate approval revision-bound and does not add a
clean-worktree precondition to Decision 0088. Approval still fingerprints its
artifact inputs and writes lifecycle evidence first; an authorized checkpoint
is a later skill action.

No generic `adapter list` or `adapter read` command is accepted here. Known
adapter paths and profiles are bounded, project-owned settings, and their owning
skills already need the complete prose. A future shared read surface can be
added when more than two consumers justify it.

## Consequences

- Operational customization has a named home distinct from artifact rules and
  templates.
- Release and Git guidance share one free-form, agent-interpreted safety model
  without becoming arbitrary executable hooks.
- Projects can choose checkpoint granularity without changing SpecBind gate or
  Task semantics.
- Approval evidence remains valid without a Git commit, while authorized
  workflows can create durable recovery points immediately afterwards.
- New adapters cannot appear accidentally through directory scanning; every new
  operational integration remains an explicit product decision.

## Implementation status

Not implemented. The installer has no adapter assets, the release skill is not
embedded, and current phase skills do not read `settings/adapters/git.md`.
