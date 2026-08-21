---
name: specbind-release
description: Close the active milestone — bind the version, run the project's release procedure, judge whether it worked, and finalize the whole milestone once it did.
argument-hint: "[version]"
---

# Release the milestone

The whole milestone or nothing. There is no partial release and no subset
option.

You orchestrate; the CLI owns every mutation of SpecBind state. **SpecBind never
verifies that a publication happened** — you and the user judge that.

## 1. Read the state and bootstrap release policy once

```sh
specbind milestone status
specbind adapter list
specbind adapter read release
```

Classify the exact read result before doing anything else:

1. If the adapter exists, has the required Front Matter, has no scaffold marker,
   and everything after Front Matter is whitespace, it is an **explicit empty
   adapter**. Do not bootstrap it. Continue to core release under the empty-body
   rule below.
2. Only an absent adapter or one carrying the exact scaffold marker is
   **unconfigured** and enters the bootstrap flow.
3. Otherwise it is an active adapter and its body is project policy.

Never infer that an adapter is unconfigured merely because it contains no
instructions. The marker, not the lack of a body, distinguishes an installed
scaffold from the explicit empty state.

An absent adapter or one carrying the exact
`<!-- specbind:adapter-scaffold -->` marker is **unconfigured**. Do not interpret
its remaining body, bind a version, run release work, or finalize.

Inspect only repository evidence that can define this project's real release
procedure: root agent instructions, package and version manifests, release
workflows, build or packaging scripts, and existing release documentation. Do
not edit any of them. Before concluding that release documentation is absent,
enumerate the repository-root files and inspect matching release documents such
as `RELEASE*`, `RELEASING*`, and `CHANGELOG*` (case-insensitively), in addition
to linked documentation. Do not rely on `README.md` being the only entry point.
Draft a complete replacement Release adapter that:

- preserves the exact `type: SpecBind Release Adapter` Front Matter;
- removes the scaffold marker;
- gives concrete Prepare, Publish, Verify, and After-finalize guidance;
- says `Nothing.` in a section that requires no action;
- names fresh success evidence, not merely a command to run; and
- never invents a version label, credential, destination, release channel, or
  external verification capability the repository does not establish.

When repository evidence cannot answer a material release question, ask the
user. Do not turn a guess into durable project policy.

Present the **entire proposed adapter** and state both boundaries before writing:

1. approval authorizes only replacing the adapter and its narrow local
   checkpoint — not binding, tagging, publishing, pushing, or finalizing; and
2. the settings write is an ordinary project change, so every participating
   Spec that already has accepted completion must run its completion handshake
   again before release preflight can pass.

After explicit approval, replace only the Release path reported by `adapter
list` below the configured SpecBind root. If the project explicitly chooses no
project-specific release work at all, preserve the Front Matter and remove the
entire body instead. Then read the result back and confirm that the scaffold
marker is absent.

Read the Git adapter and inspect `git status --short`. When it has active
guidance, follow it for one checkpoint containing only the Release adapter. The
configuration approval authorizes this narrow local checkpoint as the ordinary
final action of the bootstrap, but does not authorize push or history rewriting.
If the adapter file cannot be separated safely, leave it uncommitted and report
that fact.

**Stop after bootstrap.** Report which completion handshakes must be rerun. Do
not continue into any release step in this run, even when the user originally
asked to release the milestone.

An adapter whose body is empty after Front Matter and has no scaffold marker is
different: it explicitly means this project needs no project-specific Prepare,
Publish, Verify, or After-finalize action. Continue to core release. An active
body is project policy; follow it below.

## 2. Bind the version, and do it early

```sh
specbind release preflight
```

`RELEASE_VERSION_UNBOUND` means no version is bound.

**Ask the user for it. Never invent one.** The label is opaque and
case-sensitive — `v1.4.0` and `1.4.0` are *different releases*. Adding or
dropping a leading `v` to make it look right picks an identity the project did
not choose.

```sh
specbind milestone bind-release <version>
```

> **Binding late costs a revalidation cycle.** The binding writes
> `target_release` into the roadmap, and any non-metadata project change after a
> Spec's completion evidence stales it. So if a participating Spec is already
> `release_ready`, binding now produces:
>
> ```text
> FRESHNESS_COMPLETION_PROJECT_CHANGED: commit history since
> implementation_revision contains a non-metadata project change
> ```
>
> and the only exit is re-running the completion handshake for every affected
> Spec. Say this before binding, so the user can decide. Where the version is
> known earlier in the milestone, binding earlier avoids it entirely.

A different version already bound needs `--rebind`, which is a deliberate
replacement — confirm it explicitly first.

Then re-run `specbind release preflight` and resolve what it reports. **A
preflight failure stops the run**; no adapter work happens until it passes.

This is prose, not a script. You perform what it says; a code block in it is an
example to follow, not something that runs on its own.

**An empty adapter means releasing needs no project-specific action.** That is
an explicit statement, not a gap. Skip sections 3 through 6 and go directly to
section 7, core finalization. There is no project publication claim to verify in
this branch; the cannot-verify rule applies when active adapter guidance claims
that Publish occurred.

(Release keeps this selector-specific empty-body meaning. The installed Git
adapter is active checkpoint policy; do not transfer either adapter's default
to the other.)

## 3. Prepare

Execute any applicable Prepare guidance. Repeatable and local — no confirmation
needed. If it fails, stop here and report; nothing has left the repository.

Prepare may build ignored packages, but a version bump, generated tracked file,
or release-specific commit is an ordinary project change after accepted
completion. The Release adapter owns whether such a commit is part of this
project's procedure; it cannot keep the old completion evidence fresh. After
Prepare, rerun:

```sh
specbind release preflight
```

If the project changed and preflight no longer passes, stop before Publish and
report the affected completion handshakes. They must be rerun at the new
revision. Never treat a successful Prepare commit as permission to publish stale
completion evidence.

## 4. Publish — confirm with the user first

**This is the release-identity or outward-facing boundary in the workflow.** A
project may define Publish as a local annotated tag, or as a deployment, upload,
submission, or remote tag. A local tag has not left the repository, but it
establishes the release identity that later work may consume. External actions
also become visible to people you cannot reach.

State what the adapter will do, whether it stays local or leaves the repository,
and to which version, then get confirmation.

Do this **even if the run began with broad instructions.** Authority to prepare a
release is not authority to publish one.

## 5. Verify — it is a completion claim

```sh
specbind protocol read completion-verification
```

The claim is "the intended version really was published and is usable". Hold it
to the same standard as every other completion claim.

**Re-reading what the publish step reported is not verification.** A publish
command's success output is a claim about itself. Get fresh evidence that the
thing is actually out there and usable.

If verification cannot be performed at all — no way to reach the published
artifact, no credentials — that is *cannot verify*, and it is **not** a pass.
Report it and do not finalize.

## 6. When publish succeeded but verify did not

The milestone stays active. Every SpecBind artifact stays as it is. Then:

- **Do not roll back the publication.** SpecBind has no authority over the
  external system, an unpublish is often impossible or itself destructive, and
  that decision belongs to the user.
- **Do not retry blindly.** Read the adapter against *current* external state
  first. A publish step that partly succeeded may not be idempotent — repeating
  it can leave a second tag, a duplicate artifact, or a failure that hides the
  first success.

Report what you observed, say the milestone is still active, and work with the
user on reconciling, retrying, or abandoning it.

## 7. Write the summaries and finalize

One summary per participating Spec — exactly the participating set, no more and
no fewer.

**The summary says what was delivered, not what was asked for.** The brief is
drafting context only: it describes the start of the milestone, and the two
diverge routinely — scope gets cut, an approach changes, a requirement appears
during design. Check each summary against the final requirements, the active
requirement IDs, the design, the completed tasks, and the roadmap scope. This
text becomes the Spec's permanent history.

Keep it one line: no carriage return or line feed. Inline Markdown is fine.

Immediately before finalization, record `git status --short`. Finalization
targets must be clean, but unrelated dirty paths may exist and must remain
untouched.

```sh
specbind release finalize --log-entries -
```

```json
{
  "log_entries": [
    { "spec": "cart", "summary": "Capped held quantities at 99 per SKU." }
  ]
}
```

A Direct-only milestone omits the option entirely, or passes an explicit empty
array.

**Do not pre-edit `log.md`.** The CLI owns the whole structural update — date
headings, ordering, the canonical entry wrapper, and idempotent retry matching
by milestone ID. A failed attempt is retryable and will not duplicate history.

## 8. Checkpoint only the finalized lifecycle metadata

After successful finalization, run `git status --short` again and compare it to
the snapshot taken immediately before. The newly changed paths are the CLI's
log, archive, idle-state, Brief, Research, Tasks, review, and Roadmap lifecycle
transaction. They are post-publication metadata; the published tag or package
may correctly point to the earlier verified implementation revision.

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT`, an empty Git adapter, or one carrying the exact
`<!-- specbind:adapter-scaffold -->` marker means no adapter-directed commit.
Leave the finalized metadata uncommitted and report it.

With active guidance, follow it for one local checkpoint containing **only**
the paths newly changed by finalization. Never include a path that was already
dirty, unrelated work, or any After-finalize output. Check the staged diff and
use a concise message describing the closed release.

Publication approval does not authorize pushing this commit. Push only when the
user explicitly requested it for the current run or an applicable project
instruction independently requires it. Never amend, rewrite history, force-push,
or move the published tag to include this later metadata commit.

If the paths cannot be separated safely or the commit fails, the release is
still finalized. Report the checkpoint failure separately and never rerun
finalization merely to obtain a commit.

## 9. After finalize

Execute any applicable After-finalize guidance, and report its result
separately. **A failure here is not a failed release** — the milestone is
already closed. Never re-run finalization because of it.

### Recommend steering work when this milestone earned it

Finalization is the moment a steering edit becomes free again. Before it, a
steering change is an ordinary project change that stales every accepted
completion and forces the whole handshake to be re-run. So this belongs here,
after finalization succeeded, and nowhere earlier.

Say one sentence recommending `specbind-steering` when any of these holds:

- the milestone's scope included a **new Spec** — the project took on a durable
  responsibility it did not have before
- **Contracts changed** during the milestone, so a boundary moved
- the project has **no steering documents at all** and has now shipped a release

Otherwise say nothing. A release that changed no durable pattern does not need
the prompt, and one that appears every time is one nobody reads.

It is a recommendation. The release is already complete, nothing waits on it,
and stale steering is never a release failure.

## Boundaries

- Orchestrate only. Author no Spec artifact, edit no `log.md`, approve no gate.
- **Never claim SpecBind verified an external publication.** It reports only
  what it can check in the repository.
- Never invent a release label, and never normalize one the user gave you.
- Finalize the complete milestone or nothing.
- Report in the project's language: the version, what the adapter did, what
  verification showed, what was finalized, and anything left for the user.
