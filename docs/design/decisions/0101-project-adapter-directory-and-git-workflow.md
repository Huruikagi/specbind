# 0101: Group project adapters and add Git workflow guidance

Status: Accepted

[Decision 0137](./0137-active-default-git-checkpoints.md) supersedes the
inactive installed Git scaffold and the claim that an ordinary phase request
does not authorize its narrow local checkpoint. Missing, empty, and legacy
scaffold adapters still mean no adapter-directed commit.

## Context

SpecBind already has one project-owned operational adapter. Decisions 0002,
0010, and 0063 put free-form release instructions in a single settings file,
where an agent interprets project-specific preparation, publication,
verification, and cleanup without turning Markdown into a command language. This
decision moves that file, and those decisions now name its new path.

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
names every adapter it consumes by selector and reads it through the command
surface below. It does not scan the directory, infer behavior from filenames, or
execute an unknown file.
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

Scaffolds are localized for both configured languages, like Decision 0059
templates and unlike Decision 0093 rules. The distinction is what the document
becomes: a rule states the product's judgment until a project overrides it, so
it speaks in the product's voice, while an adapter scaffold is an empty vessel
for the project's own operational procedure. A Japanese project describing its
release steps under English headings would be as wrong here as it would be in a
Requirements scaffold.

The `type` literal stays English in both languages. It is machine identity under
Decision 0045, not prose.

The adapters differ when a project removes or has not yet received one:

- Missing `release.md` is the Decision 0063 configuration error. An empty body
  explicitly means no project-specific release actions.
- Missing or empty `git.md` means no adapter-directed commit or push. A core
  operation may still require a clean committed revision; the owning skill then
  reports or requests that prerequisite instead of inferring Git authority.
- An adapter that still carries its `specbind:instruction` comments has not been
  filled in and counts the same as an absent one. Decision 0059 makes those
  comments guidance to the author that is omitted from what the author writes,
  so their presence is a reliable signal that the project has stated no policy
  yet. Without this rule the installed scaffold reads as guidance too vague to
  act on, and every freshly installed project stops at its first checkpoint to
  ask about a file it has not written — the ceremony
  [Decision 0102](./0102-workflow-entry-condition.md) exists to prevent.

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

Two read-only commands are accepted:

```text
specbind adapter list
specbind adapter read <selector>
```

They match the `artifact`, `template`, `protocol`, and `steering` pattern:
`list` returns a compact inventory, `read` returns one document as raw UTF-8
Markdown with no result wrapper.

`list` enumerates the **known selectors**, not the directory. It reports each
accepted selector with its type, path, and whether the project has it. This is
what keeps the read surface from becoming the extension loader this decision
rejects: an unknown file below `settings/adapters/` is never listed, never
readable, and never acquires meaning by existing.

Absence is reported, not judged. `read` returns `NO_CHANGE` for an adapter the
project does not have, and the consuming skill applies its own presence
semantics — a missing release adapter is the Decision 0063 configuration error,
a missing Git adapter is simply no guidance. The CLI states what is there.

Reading through the CLI rather than by path keeps
[Decision 0098](./0098-steering-read-surface.md)'s rule general: no skill reads
project settings directly. It also removes the need for a skill to resolve
`{{SPEC_DIR}}` from `.specbind.json` before it can find the file, which is
configuration handling that belongs in the CLI and nowhere else.

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

Partially implemented. `tools/specbind/src/catalog/adapter.rs` holds the closed selector
set and the localized scaffolds, `specbind adapter list/read` expose them, and
`specbind install` plans and creates missing scaffolds as project-owned settings
that an existing copy keeps.

The listing enumerates the accepted selectors and reports project presence per
selector, so an unrecognized file below the adapters root is neither listed nor
readable; a test writes one and confirms the read is refused. Absence returns
`NO_CHANGE ADAPTER_ABSENT` rather than a fault, leaving presence semantics to
the consuming skill. Scaffolds are verified to differ per language while both
open with the same English `type` literal.

Both embedded phase skills carry a checkpoint step. `specbind-discovery`
reaches it only after the milestone mutation succeeded and every owed Brief is
written; `specbind-requirements` only after the approval succeeded, and not at
all when it stopped short of approving. Each reads the Git adapter, treats
`NO_CHANGE ADAPTER_ABSENT` as the instruction to commit nothing, and carries the
policy-is-not-authority limits: no authority from presence, commit guidance is
not push guidance, no force-push or history rewrite, staging confined to the
paths that run produced, and a stop before the Git operation on ambiguous or
conflicting guidance. A failed checkpoint leaves the milestone, Briefs, and
approval intact and is reported as uncommitted.

The conformance check covers the adapter invocation; it was confirmed to reject
a renamed `adapter` route.

A forward test confirms the consuming side: with a Git adapter carrying real
policy, a run committed after the approval with the required message prefix,
staged only the paths it produced, and neither pushed nor changed branch. With
the adapter left as its installed scaffold, other runs committed nothing and did
not stop to ask about it.

The release skill is not embedded, so the release adapter has an installer and a
read surface but no consumer yet. Later skills carry the checkpoint step from the
start. An
install that creates a Git adapter no skill reads leaves a project that has
stated its checkpoint policy and is silently ignored, which is worse than having
no adapter: the setting exists, so its absence of effect reads as SpecBind
disagreeing rather than as SpecBind not listening. The two embedded phase skills
are `specbind-discovery` and `specbind-requirements`; later skills carry the
checkpoint step from the start.
