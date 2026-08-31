---
type: SpecBind Steering
---

# Title

<!-- specbind:instruction create
This is the scaffold for a steering document whose subject the author chooses.
Unlike every other SpecBind template it declares no artifact_id, because a
steering identity is the author's to pick: supply one before writing, as a
lowercase kebab-case token that no existing steering document already uses, and
materialize this file only at the project_path reported by `template list
steering` after replacing <artifact_id> with that token.

Replace these headings with the ones the subject needs.
-->

<!-- specbind:instruction maintain
Steering carries what outlives any single change. A fact belongs here when a
competent newcomer would otherwise have to discover it by reading widely, or by
getting it wrong once. Anything narrower belongs to the artifacts of the change
that needs it.

Subjects that commonly earn their own document: API conventions, testing
approach, security posture, data and migrations, error handling, deployment.
None of these is required, and one the project does not actually have settled
conventions for is better left unwritten than guessed at.

Keep the document focused enough that it never wants its own table of contents.
-->

## What this covers

<!-- specbind:instruction maintain
State which project-wide subject this document owns, which adjacent subjects it
does not own, and when a later change should consult it.
-->

## Decisions and constraints

<!-- specbind:instruction maintain
Record settled decisions, prohibitions, and selection criteria inherited by
later changes. Exclude unsettled proposals and decisions needed by only one
Spec. Give each decision or constraint a descriptive H3 heading and keep its
reason, applicability, and exceptions beside it. Do not create this document
when there is no settled content.
-->

<!-- specbind:instruction create
Replace `<decision-or-constraint>` with a name for the actual judgment and
repeat the H3 subsection for each independent judgment. Do not leave the empty
example heading in a live artifact.
-->

### `<decision-or-constraint>`

## Examples

<!-- specbind:instruction maintain
Give one representative positive or boundary example when the rule alone is
easy to misapply. Do not build an exhaustive catalog. Delete this section when
the decisions can be applied without an example.
-->
