---
type: SpecBind Deferred Findings Adapter
---

# Deferred findings adapter

<!-- specbind:instruction
Describe where this project keeps a review finding that is real but is not the
reason a gate is held. SpecBind reads this prose and follows it; it is not a
script.

This file exists because a finding with nowhere to go is a finding that gets
raised as blocking. A reviewer who knows an observation will otherwise vanish
has one way to make it survive, and uses it. Naming a destination removes that
pressure.

The destination is written to, not read from. SpecBind never reads it to decide
what to build, and no entry becomes work by sitting here. An item re-enters the
workflow only when a person adds it to the Roadmap, where the usual scope
classification applies.

The section below is filled in with a working default. Replace it if this
project already has somewhere these belong, such as an issue tracker. Leaving
this file empty, or removing it, means SpecBind has no destination and states
deferred findings in the review report only.
-->

## Where deferred findings go

Append the finding to `deferred.md` at the root of the specification directory,
creating the file if it does not exist. Write one entry per finding, each naming
the Spec it came from and what it endangers, and add nothing else to the file.

Before appending, read the file only far enough to see whether the same finding
is already recorded, and skip the append when it is. Do not use its contents for
any other purpose.

## What belongs here

An observation that is actionable and worth keeping, and that does not change a
verdict. A finding that changes a verdict is resolved before the gate, not
recorded here.

## What does not belong here

Work that the current change must do, an unresolved question that blocks a
decision, or a note whose only reader would be the review that produced it.
