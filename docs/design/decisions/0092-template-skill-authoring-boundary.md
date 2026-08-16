# 0092: Separate artifact scaffolds from authoring workflow policy

Status: Accepted

## Context

Decision 0008 makes both `settings/templates/` and `settings/rules/` project-owned customization surfaces. Decision 0059 then allows a template to carry `specbind:instruction` comments that an authoring agent reads before removing them from the materialized artifact. This is useful for explaining a scaffold, but it also makes it easy to place workflow policy in the template merely because the template is nearby.

That placement is unsafe as a product contract. A project may replace a template, remove every instruction comment, or reorganize the document while retaining a valid artifact profile. An embedded template may also remain hidden behind a project override. Core workflow behavior, lifecycle semantics, and semantic quality requirements must not disappear or change merely because the project customizes document structure.

Moving all such guidance to `settings/rules/` would not solve the problem. Shared rules are also user-owned. They are the correct place for project-wide authoring preferences, but not the sole authority for non-overridable SpecBind workflow behavior.

SpecBind therefore needs an explicit boundary between four different carriers: templates, shared rules, product-managed skills, and the deterministic CLI.

## Decision

### Template responsibility

A template owns the customizable shape and local affordances of one materialized artifact:

- literal OKF identity and any template-valid artifact metadata
- relative output path and, for collection profiles, the default decomposition represented by the template set
- default headings, section order, durable scaffold text, examples, and presentation format
- concise `specbind:instruction` guidance that explains how to fill, repeat, rename, or remove the scaffold immediately around it
- reminders of exact machine-readable syntax that is easy to violate while filling that scaffold, such as Requirement headings, Design Requirement markers, and Contract entry shapes

Template instructions are advisory input tied to the selected scaffold. They are removed during materialization, do not become live-artifact authority, and may not be the only place that states a SpecBind lifecycle rule, semantic minimum, approval obligation, or cross-artifact invariant.

A project may customize template structure and template-local guidance wherever the target artifact profile permits it. Such customization does not redefine what an artifact means, when it participates in the lifecycle, or what a product-managed workflow must establish.

### Shared-rule responsibility

Shared rules own user-customizable authoring policy that applies across templates, artifacts, or supported agents:

- project terminology, tone, and level-of-detail preferences
- preferred requirement-writing patterns such as EARS
- diagram, table, citation, naming, and review conventions
- project-specific judgment criteria that strengthen or specialize the product baseline
- common OKF authoring guidance under Decision 0049

A rule should not prescribe one template's exact section inventory; the template owns that format. A rule also cannot waive a CLI contract or a product-managed skill obligation. Removing every project rule must leave the core SpecBind workflow safe and semantically defined, although the resulting artifacts may lose project-specific style and policy.

### Skill responsibility

Product-managed skills own workflow orchestration and every semantic obligation that must survive project customization but cannot be established mechanically by the CLI:

- whether and when an optional artifact is created, revised, split, omitted, or removed
- which current artifacts, templates, rules, repository facts, and user decisions must be read
- whether Requirements express the complete current behavioral contract and whether their scope is substantively adequate
- whether Design is self-contained, realizes its Requirements, and remains consistent with the persistent Contract
- whether a boundary belongs in the sparse Contract and whether Contract changes require wider review
- whether Research findings must be promoted into Requirements, Design, or Contract before Research can remain non-authoritative
- whether information is durable enough for Implementation Notes
- approval, delegated-approval, invalidation, rewind, retry, and user-confirmation behavior
- semantic review quality beyond structural reference presence

Skills read the resolved template and applicable shared rules, but treat them as user-owned structure and policy layered onto the product workflow. A customized template or rule can strengthen, specialize, or reformat the result; it cannot erase the skill's core obligations.

### CLI responsibility

The CLI owns deterministic facts and guarded state transitions:

- template discovery, source precedence, selector identity, output-path safety, and instruction-node stripping
- OKF and artifact-profile validation
- exact machine-readable syntax, IDs, references, traceability sets, and fingerprints
- lifecycle prerequisites, gate freshness, state mutation, and path or Git safety
- focused diagnostics when a customized template or materialized artifact is incompatible

Neither a skill, shared rule, nor template may reinterpret CLI-invalid content as valid. Conversely, CLI structural success does not attest to semantic design or requirements quality unless a decision explicitly gives the CLI such a check.

### Allocation test

New guidance is assigned with these questions, in order:

1. Can it be decided completely and reliably from files or explicit inputs? If yes, it belongs in the CLI contract and may be repeated elsewhere only as usability guidance.
2. Must it remain true when a project replaces every template and shared rule? If yes, it belongs in the product-managed skill contract.
3. Is it a project-wide authoring or review preference independent of one scaffold? If yes, it belongs in a shared rule.
4. Does it explain the chosen scaffold, its sections, or exact syntax encountered while filling it? If yes, it belongs in the template.

Duplicating a short machine-syntax reminder in a template is allowed. Duplicating workflow or semantic policy across templates is not the default because those copies drift and disappear under customization.

### Settings conflicts

CLI contracts and product-managed skill obligations are non-waivable. Templates and shared rules are orthogonal user-owned settings: templates control artifact structure and placement, while rules control cross-artifact authoring policy.

When a general rule can be satisfied within a customized structure, the skill adapts placement to that structure rather than restoring an official section inventory. If user-owned settings are materially contradictory and cannot both be honored without weakening a core obligation, the skill reports the conflict and requests clarification. It does not silently prefer an embedded default, overwrite project settings, or invent a third policy.

## Artifact-specific allocation

| Artifact | Template owns | Skill owns |
| --- | --- | --- |
| Brief | Default headings and concise capture format. | Creation and merge timing, faithful capture of the confirmed request, and milestone-local lifecycle. |
| Research | Optional investigation scaffold and local section prompts. | Whether durable Research is useful, current-state replacement rather than append-only logging, and promotion of every lasting conclusion into an authoritative artifact. |
| Requirements | Literal `heading_labels`, section inventory, Objective placement, and exact Requirement/Acceptance Criteria grammar reminders. | Complete-current-contract semantics, substantive scope quality, active Requirement selection, approval, and downstream invalidation. Project-wide EARS preferences belong in a shared rule. |
| Design | Default decomposition, section inventory, presentation aids, and the exact `_Requirements: ..._` reminder. | Selection or extension of the design set, self-contained decisions, Requirement realization, Contract consistency, and design approval or rewind. |
| Contract | Canonical headings and exact entry-shape reminders fixed by Decision 0056. | Semantic seam selection, stable-boundary judgment, sparse File Ownership, and coordination with cross-spec review. |
| Implementation Notes | Optional organization and note format. | Whether knowledge is non-obvious and durable enough to persist, and when to create or update an artifact. |

This allocation applies equally to embedded defaults and project overrides. Decision 0091's narrower installed set changes which templates SpecBind invites projects to customize; it does not change the responsibility boundary.

## Migration

The current embedded templates contain some transitional workflow and semantic guidance. That guidance is not removed until its authoritative destination exists.

Implementation proceeds in this order:

1. Inventory every existing `specbind:instruction` statement and classify it with this decision's allocation test.
2. Add non-waivable orchestration and semantic obligations to the owning product-managed skills.
3. Add genuinely customizable cross-artifact conventions to the appropriate shared default rules.
4. Thin template instructions to scaffold-local guidance and machine-syntax reminders.
5. Test core authoring workflows with valid project templates whose instruction comments are removed or substantially rewritten, proving that customization cannot erase the workflow contract.
6. Test that template-specific headings and placement are still honored and that incompatible customization receives focused CLI or skill diagnostics.

Installed project templates and rules remain user-owned and are never overwritten to perform this migration. Updated embedded defaults affect only later resolutions that are not shadowed by project copies. Existing live artifacts are not reconciled to revised scaffolds.

## Consequences

- Projects can customize document structure and writing policy without accidentally customizing SpecBind lifecycle semantics.
- Skills remain meaningful with minimal valid templates and no project rules.
- Templates stay useful at the point of authoring without becoming hidden workflow specifications.
- Shared rules remain the cross-agent project policy surface instead of a second location for non-overridable product logic.
- CLI validation, skill semantics, and user-owned presentation have explicit and testable boundaries.
- Some guidance may appear briefly in both a skill and template when the template repeats an exact machine contract for usability, but the authoritative owner remains unambiguous.

## Implementation status

This decision defines the target boundary. The embedded templates added before this decision still contain mixed scaffold, workflow, and semantic guidance. The migration above remains a separate implementation increment so guidance is not lost before the v1 SpecBind skills and shared rules provide its authoritative destination.
