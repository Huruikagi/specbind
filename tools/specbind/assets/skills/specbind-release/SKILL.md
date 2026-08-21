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

## 1. Bind the version, and do it early

```sh
specbind milestone status
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

## 2. Read the project's release procedure

```sh
specbind adapter read release
```

This is prose, not a script. You perform what it says; a code block in it is an
example to follow, not something that runs on its own.

**An empty adapter means releasing needs no project-specific action.** That is
an explicit statement, not a gap — proceed to finalization. The same applies
when it carries the exact `<!-- specbind:adapter-scaffold -->` marker: that is
the installed scaffold, not project policy. The marker classifies the whole
document, so ignore every other body line even when it looks actionable.

(Release keeps this selector-specific absence meaning. The installed Git
adapter is active checkpoint policy; do not transfer either adapter's default
to the other.)

## 3. Prepare

Execute any applicable Prepare guidance. Repeatable and local — no confirmation
needed. If it fails, stop here and report; nothing has left the repository.

## 4. Publish — confirm with the user first

**This is the only irreversible, outward-facing action in the workflow.** A tag,
a deployment, an upload, a submission — it leaves the repository and becomes
visible to people you cannot reach.

State what the adapter will do and to which version, and get confirmation.

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

## 8. After finalize

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
