---
type: SpecBind Release Adapter
---

# Release adapter

<!-- specbind:adapter-scaffold -->

Describe this project's release procedure in your own words. SpecBind reads this
prose and performs what it says; it is not a script, and a code block here is an
example to follow rather than something SpecBind runs on its own.

Replace this scaffold before the first release. The Release workflow can inspect
the repository, propose the complete replacement, and checkpoint it after your
approval; that setup run stops without publishing. Write `Nothing.` in a section
that needs no action. A body emptied to Front Matter only explicitly means the
whole release needs no project-specific action.

Say what "done" looks like, not only what to run. A step whose success cannot be
checked cannot be verified.

## Prepare

Version synchronization, build and packaging, and any pre-publication checks
this project requires.

## Publish

Tagging, deployment, release workflows, store submission, or whatever else
publishes this project.

## Verify

Fresh checks proving the intended version really was published and is usable.
Re-reading what the publish step reported is not verification.

## After finalize

Optional cleanup that runs only after SpecBind finalization succeeds. A failure
here is reported and does not undo the release.
