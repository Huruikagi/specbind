---
type: SpecBind Steering
artifact_id: tech
---

# Technology

<!-- specbind:instruction maintain
Record the technology decisions every later change inherits, and why they were
made. A constraint whose reason is lost gets followed superstitiously or thrown
out wholesale.

Write the decisions, not the dependency list. The manifest already states which
versions are installed, and it stays correct without anyone maintaining prose.

Include the settled boundaries a reader would otherwise rediscover by breaking
one — which layer owns persistence, what may call what, which parts are
generated. These are the statements that decide where new work goes.
-->

## Technology foundation

<!-- specbind:instruction maintain
Record languages, runtimes, frameworks, and execution environments that affect
later decisions, together with the role each plays. Do not duplicate dependency
or version inventories available from manifests.
-->

## Decisions and their reasons

<!-- specbind:instruction maintain
For each consequential choice that had viable alternatives, state the selected
direction, why it was selected, and what it protects. Omit choices obvious from
the current setup that do not constrain future decisions.
-->

## Constraints every change inherits

<!-- specbind:instruction maintain
Record dependency direction, generated assets, persistence, compatibility, and
supported-environment boundaries that no change may violate. Delete this
section when every such constraint is already clear beside its decision.
-->

## Standard verification

<!-- specbind:instruction maintain
Record project-wide verification layers, standard commands, and required
environments. Keep verification specific to one Spec in its Design. Delete this
section when the project has no settled shared standard yet.
-->
