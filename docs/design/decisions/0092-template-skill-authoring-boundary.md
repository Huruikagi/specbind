# 0092: Separate artifact scaffolds from authoring workflow policy

Status: Accepted

## Context

Decision 0008 makes both `settings/templates/` and `settings/rules/` project-owned customization surfaces. Decision 0059 then allows a template to carry `specbind:instruction` comments that an authoring agent reads before removing them from the materialized artifact. This is useful for explaining a scaffold, but it also makes it easy to place workflow policy in the template merely because the template is nearby.

That placement is unsafe as a product contract. A project may replace a template, remove every instruction comment, or reorganize the document while retaining a valid artifact profile. An embedded template may also remain hidden behind a project override. Core workflow behavior, lifecycle semantics, and semantic quality requirements must not disappear or change merely because the project customizes document structure.

Moving all such guidance to `settings/rules/` would not solve the problem. Shared rules are also user-owned. They are the correct place for project-wide authoring preferences, but not the sole authority for non-overridable SpecBind workflow behavior.

SpecBind therefore needs an explicit boundary between templates, shared rules, product-managed skills, immutable product protocols, and the deterministic CLI. Decision 0094 adds the protocol carrier for shared non-customizable semantic baselines.

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

Decision 0093 fixes the narrow v1 project-owned rule set and its explicit skill consumers. Decision 0094 separately owns immutable product protocols.

Shared rules own user-customizable authoring policy that applies across templates, artifacts, or supported agents:

- project terminology, tone, and level-of-detail preferences
- preferred requirement-writing patterns such as EARS
- diagram, table, citation, naming, and review conventions
- project-specific judgment criteria that strengthen or specialize the product baseline

A rule should not prescribe one template's exact section inventory; the template owns that format. A rule also cannot waive a CLI contract, product protocol, or product-managed skill obligation. Removing every project rule must leave the core SpecBind workflow safe and semantically defined, although the resulting artifacts may lose project-specific style and policy.

### Product-protocol responsibility

The read-only protocols accepted by Decision 0094 own substantial semantic criteria that must remain consistent across supported agents and cannot be weakened by project customization. They are embedded in the binary and exposed through `specbind protocol read <selector>` rather than installed into `settings/rules/`.

This includes complete-current Requirements quality, self-contained Design and Contract realization, sparse cross-spec boundary judgment, promotion of lasting Research conclusions, and semantic review quality beyond structural reference presence.

Protocols do not own command ordering, retries, user authority, or mutation calls, and reading them does not prove compliance. Skills orchestrate them; the CLI independently enforces every deterministic invariant.

### Skill responsibility

Product-managed skills own workflow orchestration and skill-local semantic obligations that must survive project customization but do not warrant a shared protocol:

- whether and when an optional artifact is created, revised, split, omitted, or removed
- which current artifacts, templates, rules, repository facts, and user decisions must be read
- whether information is durable enough for Implementation Notes
- approval, delegated-approval, invalidation, rewind, retry, and user-confirmation behavior

Skills read the required product protocols, resolved template, and applicable shared rules. They treat templates and rules as user-owned structure and policy layered onto the product workflow. A customized template or rule can strengthen, specialize, or reformat the result; it cannot erase protocol or skill obligations.

### CLI responsibility

The CLI owns deterministic facts and guarded state transitions:

- template discovery, source precedence, selector identity, output-path safety, and instruction-node stripping
- OKF and artifact-profile validation
- exact machine-readable syntax, IDs, references, traceability sets, and fingerprints
- lifecycle prerequisites, gate freshness, state mutation, and path or Git safety
- focused diagnostics when a customized template or materialized artifact is incompatible

Neither a protocol, skill, shared rule, nor template may reinterpret CLI-invalid content as valid. Conversely, CLI structural success does not attest to semantic design or requirements quality unless a decision explicitly gives the CLI such a check.

### Allocation test

New guidance is assigned with these questions, in order:

1. Can it be decided completely and reliably from files or explicit inputs? If yes, it belongs in the CLI contract and may be repeated elsewhere only as usability guidance.
2. Must it remain true when a project replaces every template and shared rule? If yes, substantial semantic content shared across agents or skills belongs in a product protocol; orchestration and short skill-local content belong in the product-managed skill contract.
3. Does it define when, in what order, under whose authority, or with how many retries work occurs? If yes, it belongs in the skill.
4. Is it a project-wide authoring or review preference independent of one scaffold? If yes, it belongs in a shared rule.
5. Does it explain the chosen scaffold, its sections, or exact syntax encountered while filling it? If yes, it belongs in the template.

Duplicating a short machine-syntax reminder in a template is allowed. Duplicating workflow or semantic policy across templates is not the default because those copies drift and disappear under customization.

### Settings conflicts

CLI contracts, embedded product protocols, and product-managed skill obligations are non-waivable. Templates and shared rules are orthogonal user-owned settings: templates control artifact structure and placement, while rules control cross-artifact authoring policy.

When a general rule can be satisfied within a customized structure, the skill adapts placement to that structure rather than restoring an official section inventory. If user-owned settings are materially contradictory and cannot both be honored without weakening a core obligation, the skill reports the conflict and requests clarification. It does not silently prefer an embedded default, overwrite project settings, or invent a third policy.

Project adapters accepted later by Decision 0101 sit outside this authoring
model. They customize project-specific operational actions such as release work
and Git checkpoints, not artifact structure or semantic quality. Their owning
skills still control eligibility, ordering, safety, and authority; adapter prose
cannot waive any obligation allocated here.

## Artifact-specific allocation

| Artifact | Template owns | Project rule may tune | Product protocol and skill own |
| --- | --- | --- | --- |
| Brief | Default headings and concise capture format. | None by default. | The discovery skill owns creation, merge timing, faithful capture, and milestone-local lifecycle. |
| Research | Optional investigation scaffold and local section prompts. | Applicable design and contract preferences. | `gap-analysis` owns the semantic baseline; the gap-analysis skill owns materialization, current-state replacement, and promotion of lasting conclusions. |
| Requirements | Literal `heading_labels`, section inventory, Objective placement, and exact Requirement/Acceptance Criteria grammar reminders. | `ears-format.md` owns project writing preferences. | `requirements-review` owns complete-current-contract and substantive quality; the requirements skill owns active selection, approval, and invalidation. |
| Design | Default decomposition, section inventory, presentation aids, and the exact `_Requirements: ..._` reminder. | `design-principles.md` and `contract-principles.md` tune project preferences. | Design protocols own discovery, authoring, and validation baselines; the design skills own selection, orchestration, approval, and rewind. |
| Contract | Canonical headings and exact entry-shape reminders fixed by Decision 0056. | `contract-principles.md` tunes project seam policy. | Design and cross-spec protocols own semantic seam and compatibility baselines; skills own update timing and review orchestration. |
| Implementation Notes | Optional organization and note format. | None by default. | The implementation skill owns the durability judgment and creation or update timing. |

This allocation applies equally to embedded defaults and project overrides. Decision 0091's narrower installed set changes which templates SpecBind invites projects to customize; it does not change the responsibility boundary.

## Migration

The current embedded templates contain some transitional workflow and semantic guidance. That guidance is not removed until its authoritative destination exists.

Implementation proceeds in this order:

1. Inventory every existing `specbind:instruction` statement and classify it with this decision's allocation test.
2. Add shared non-waivable semantic baselines to the Decision 0094 product protocols.
3. Add orchestration and skill-local obligations to the owning product-managed skills.
4. Add genuinely customizable cross-artifact conventions to the appropriate shared default rules.
5. Thin template instructions to scaffold-local guidance and machine-syntax reminders.
6. Test core authoring workflows with valid project templates whose instruction comments are removed or substantially rewritten, proving that customization cannot erase the workflow contract.
7. Test that template-specific headings and placement are still honored and that incompatible customization receives focused CLI or skill diagnostics.

Installed project templates and rules remain user-owned and are never overwritten to perform this migration. Updated embedded defaults affect only later resolutions that are not shadowed by project copies. Existing live artifacts are not reconciled to revised scaffolds.

## Consequences

- Projects can customize document structure and writing policy without accidentally customizing SpecBind lifecycle semantics.
- Skills remain meaningful with minimal valid templates and no project rules.
- Templates stay useful at the point of authoring without becoming hidden workflow specifications.
- Shared rules remain the cross-agent project policy surface instead of a second location for non-overridable product logic.
- CLI validation, immutable protocols, skill orchestration, project policy, and user-owned presentation have explicit and testable boundaries.
- Some guidance may appear briefly in both a skill and template when the template repeats an exact machine contract for usability, but the authoritative owner remains unambiguous.

## Implementation status

This decision defines the target boundary together with Decision 0094. The embedded templates added before this decision still contain mixed scaffold, workflow, and semantic guidance. The migration above remains a separate implementation increment so guidance is not lost before the v1 protocols, skills, and shared rules provide its authoritative destination.
