# 0091: Separate the embedded scaffold set from the installed customization surface

Status: Accepted

Decision 0152 later adds the standard installed `design/ui` candidate and its
required selection Rule. UI remains conditional per Spec rather than becoming a
mandatory live artifact.

## Context

[Decision 0059](./0059-okf-artifact-templates.md) makes a template both the materialization scaffold for an artifact and the file a project edits to customize that artifact. [Decision 0008](./0008-customization-surface.md) describes `settings/templates/` as the surface for "the structure and format of generated requirements, design, tasks, steering, and other documented artifacts".

Those two roles do not apply equally to every artifact type. A `SpecBind Brief` is free-form by [Decision 0062](./0062-minimal-active-brief-profile.md), which requires no title, heading inventory, or section order, and the inherited cc-sdd discovery skill wrote `brief.md` from a shape declared inside the skill rather than from `settings/templates/`. `SpecBind Implementation Notes` is persistent free-form memory under [Decision 0026](./0026-runtime-implementation-notes.md). `SpecBind Research` is optional investigation output that Decision 0059 materializes only when research is actually useful. A `SpecBind Contract` is the opposite case: [Decision 0056](./0056-canonical-contract-markdown.md) fixes its five sections and entry grammar, so a project cannot change its structure at all and can only reword the guidance around it.

Shipping any of those as a customization invitation offers a project little or nothing to customize, while still adding a file it must review and maintain.

Removing their templates entirely would cost something real. A template is also the deterministic carrier of machine identity: its literal `type`, conditional `artifact_id`, and derived output path exist under Decision 0059 precisely so those values never depend on AI interpretation. If the Brief had no scaffold, its OKF identity would return to agent prose.

## Decision

- The embedded default set covers **every** recognized Spec artifact type. `template list spec` and `template read spec <selector>` therefore always answer for every selector, and machine identity stays CLI-owned for all of them.
- The **installed customization surface** is narrower. `specbind install` writes only the templates whose structure and format a project has a reason to own:

| Template | Installed | Rationale |
| --- | --- | --- |
| `requirements.md` | yes | Heading labels and acceptance-criteria structure are project style |
| `design.md` | yes | Design decomposition and section inventory are project style |
| `contract.md` | no | Decision 0056 fixes the five sections and entry grammar, so only the guidance prose is variable |
| `research.md` | no | Optional investigation output that Decision 0059 materializes only when useful |
| `brief.md` | no | Free-form working input under Decision 0062 |
| `implementation-notes.md` | no | Free-form retained memory under Decision 0026 |

- The rule is that a template belongs on the customization surface when a project can actually change the artifact's structure. Everything else stays an embedded scaffold.
- A project may still override an uninstalled template by creating the file at its output path. Per-selector resolution already prefers a project-owned copy, so nothing is forbidden; the narrower installed set only stops SpecBind from inviting customization that carries no benefit.
- Starting narrow is the low-regret direction. Under Decision 0008 an install creates a default only when its target path is absent, so widening the installed set later adds files to existing projects safely, while narrowing it later would strand files SpecBind had told users to own.
- `template list spec` continues to report every resolved template with its source, so an agent and a human can see which selectors come from the binary and which the project owns.
- Decision 0008's customization surface therefore means "the artifacts whose structure a project may own", not "every artifact SpecBind can materialize".

## Consequences

- A newly installed project originally contained two Spec template files. Decision
  0152 adds the conditional UI candidate as a third project-owned Spec template.
- Brief and Implementation Notes keep deterministic OKF identity and a scaffold to materialize from, without pretending their structure is a product contract.
- The install and refresh plans gain an explicit list rather than "everything embedded", which the installer increment must honor.
- A project that genuinely wants to standardize its Brief or Contract guidance can still create the file under `settings/templates/specs/`; it is an informed choice rather than a default.

## Implementation status

Implemented. The embedded set and per-selector override resolution make
`template list spec` and `template read spec <selector>` behave as accepted.
`specbind install` creates the Requirements, main Design, and conditional UI
Design defaults when they are absent and keeps every existing project-owned
template unchanged.

Decision 0145 later widens the installed surface with a project-owned milestone
Roadmap body template. It does not change this Decision's six-template Spec
inventory or the Requirements-and-Design subset installed from that inventory.
