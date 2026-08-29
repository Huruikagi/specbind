---
type: SpecBind Rule
---

# Contract principles

This rule is the project's policy for persistent cross-spec seams. It is a
`SpecBind Rule`: your project owns this file and may strengthen, relax, replace,
or remove it. Removing it leaves Contract validation and contract review
intact and only removes this project's seam policy.

The canonical Contract structure and entry grammar are fixed by the CLI
contract. Graph validity and the review lifecycle are owned by the CLI and the
`contract-review` protocol. This file is about which seams this project
chooses to declare and how strictly it treats them.

## Declare a seam when it is real

A Contract entry is a durable promise other Specs may depend on. Declare an
entry when another Spec legitimately needs it, not to document everything a Spec
happens to contain.

An over-declared Contract makes every internal change look like a seam change
and trains reviewers to skim. An under-declared one lets a real dependency form
without review.

## Ownership

- One owner per behavior and per piece of data. When two Specs appear to co-own
  something, decide which one owns it rather than describing the overlap.
- File Ownership entries exist so two Specs do not silently edit the same place.
  Claim the area a Spec is responsible for, not everything it currently touches.
- Generated or vendored areas usually belong to whichever Spec owns the
  generator or the update procedure. Say which, so nobody owns them by accident.

## Compatibility posture

This project's default posture is conservative:

- A removal or narrowing is accepted only when every managed consumer changes
  in the same milestone or no longer consumes the seam.
- An additive change is still reviewed for ownership, dependency direction, and
  unmanaged consumer impact; “additive” is not an automatic compatibility pass.
- A seam used outside this repository is changed only after the affected
  consumer and the user's intended compatibility disposition are explicit.

Replace this section when the project deliberately chooses a different
compatibility policy. Until then, apply this default rather than deciding case
by case during each review.

## Dependency direction

Say which direction dependencies may run between areas of this project.
Contract review reports dependency cycles as warnings because they are
occasionally deliberate; a stated direction turns that warning into a decision
you can make quickly rather than an argument you rehear each milestone.

## When a warning deserves more than a note

The CLI reports ownership overlap and dependency cycles as warnings for human
judgment. Name here which of them this project treats as blocking, for example
overlap on generated output, or any cycle involving a published interface.

Where a warning is routinely acceptable in this project, say that too, so
reviewers do not relitigate it every time.

## Review questions

- Would another Spec break if this entry changed, and does that Spec know?
- Is this entry a promise, or just a description of the current implementation?
- Does anything outside this repository depend on this seam?
- If two Specs touch the same files, is that stated deliberately?
