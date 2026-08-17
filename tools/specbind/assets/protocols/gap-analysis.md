# Gap analysis protocol

This protocol is the shared baseline for analyzing the distance between what a
change requires and what the repository already provides. It applies to every
supported agent and cannot be waived by a project template or shared rule.

It owns what the analysis must establish and how uncertainty and conclusions are
handled. Whether a Research artifact is created, when it is revised or replaced,
and the shape of the report belong to the gap-analysis skill.

## Inform, do not decide

Gap analysis produces the material a design decision is made from. It does not
make the decision.

- Present the realistic options with what each one costs, not a single
  recommendation dressed as analysis. One option is a decision.
- A preference may be stated, and usually should be, as long as it is visibly a
  recommendation with its reasoning rather than a conclusion the reader is
  expected to inherit.
- Where the evidence genuinely determines the answer, say so plainly. Manufactured
  alternatives are as misleading as a hidden decision.

## Evidence is gathered, not recalled

Every statement about the current system is a claim that can be checked.

- Establish what exists in the affected area: the modules and layout involved,
  the components and utilities that could be reused, and the patterns already in
  force.
- Extract the conventions the change will have to live with, including layering,
  dependency direction, and where tests belong.
- Identify the integration surfaces the change meets: data models, external
  clients, authentication, and anything else it must interoperate with.
- Read the code rather than inferring it from naming or from an earlier session.
  An analysis built on a plausible but wrong picture of the system is worse than
  no analysis, because it is persuasive.

## Name the gaps precisely

For each technical need the requirements imply, state which of these it is:

- **Missing.** The capability does not exist and must be built.
- **Unknown.** Whether it exists, or whether it fits, has not been determined.
- **Constraint.** It exists but restricts the approach in a way the design must
  respect.

Conflating these hides work. An "unknown" recorded as "missing" invents scope; a
"constraint" recorded as "missing" invites a redundant parallel implementation.

## Uncertainty is classified by what it blocks

- An unknown that prevents choosing between the options must be resolved now, or
  escalated. It cannot be handed to Design as an open item.
- An unknown that does not affect the choice is carried forward explicitly so
  the later phase knows to resolve it.
- Deep investigation of a question that only matters after the approach is
  chosen belongs to that later phase. Recording it concisely here is enough.

State assumptions explicitly wherever the analysis rests on one. An unstated
assumption becomes an invisible premise of every decision that follows.

## Effort and risk are estimates with reasons

Where the analysis characterizes effort or risk, each label carries a one-line
justification naming what drives it: unfamiliar technology, breadth of impact,
integration complexity, or an unresolved unknown.

An unjustified label is not usable. The project may define its own scale; the
requirement is that the reader can see why the label was chosen and disagree
with it.

## Research is milestone-local and is not authority

When conclusions are recorded in a Research artifact, that artifact exists for
the current milestone only.

- Research is investigation, not contract. Requirements, Design, and Contract
  remain the authoritative statements of what the system must do, and a claim
  that lives only in Research binds nothing.
- Research does not survive release finalization. Any conclusion that will still
  matter afterwards must be promoted into the authoritative artifacts or into
  steering before the milestone closes. A conclusion left only in Research is a
  conclusion the project has decided to forget.
- Research reflects the current state of the investigation. Superseded findings
  are replaced rather than accumulated into a log of everything ever considered.
- Create it only when the investigation is worth retaining. Routine analysis that
  the Design absorbs needs no separate document.
