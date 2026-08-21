---
type: SpecBind Deferred Findings Adapter
---

# Deferred findings adapter

This file is project policy for where to keep a review finding that is real but
does not hold a gate. The section below contains a working default. Replace it
when the project already has a destination such as an issue tracker.

The destination is a place to write, not a place to read. An item does not
become work merely by being put there. It re-enters the workflow only when a
person adds it to the Roadmap, where ordinary scope classification applies.

## Where deferred findings go

Append the finding to `deferred.md` in the configured SpecBind root. With the
default `specDir`, its project-relative path is `.specbind/deferred.md`. This is
one project-wide file shared by every Spec.

When the file does not exist, create it with this content before appending the
first finding:

```markdown
---
type: Deferred Findings
---

# Deferred findings
```

Write one entry per finding, each naming the Spec it came from and what it
endangers, and add nothing else. This file is an OKF concept, but it is not a
SpecBind artifact, gate, fingerprint input, or work queue.

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
