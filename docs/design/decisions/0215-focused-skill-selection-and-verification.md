# 0215: Keep Skill selection and verification focused on the requested outcome

Status: Accepted

## Context

An audit prompted by OpenAI's [Rethinking skills and prompts for GPT-6
Astra](https://developers.openai.com/blog/rethinking-skills-and-prompts-for-gpt-6-astra)
found long discovery metadata, outdated routing prose, and verification wording
that could add an unrequested workflow. The product supports several agents and
models, so this is a clarification of their shared instructions, not a model
migration.

## Decision

- Configure, Plan, and Implement descriptions name their selection boundaries.
  Their existing bodies and references retain mode-specific procedures and
  authority. Descriptions need not enumerate every configurable surface or
  implementation step.
- Status applies Decision 0187's consequence-free claim-check precedence even
  when a completed Task plan names a Spec. Only explicit authority to record
  completion selects lifecycle validation.
- Implement owns one Roadmap item and returns to its caller; Drive owns
  milestone-wide orchestration under Decision 0168. Remove the obsolete claim
  that no orchestrator exists.
- Claim verification remains the independent workflow in Decision 0113. Reporting
  a check already executed does not require invoking another Skill. Its own
  fresh-evidence requirement, no-repair boundary, and verdicts remain intact.
- Task implementation runs all applicable required checks. Add tests for changed
  behavior and meaningful regressions, not merely to mirror prose. After those
  checks pass, proceed to review. Repeat or broaden checks when later changes,
  failures, unresolved risks, or explicit project requirements justify it.
  Whole-Spec validation retains every required evidence dimension under
  Decisions 0112, 0182, and 0205.

The installed project-instruction block already separates routing, explicit
completion authority, and CLI-owned state. Keep those rules. Discovery and Plan
already load mode-specific references; keep that structure. Preserve scoped
artifact instructions, required project-policy reads, fresh role separation,
retry bounds, and confirmed recovery consequences. Their constraints protect
product authority rather than prescribe optional model technique. No model
defaults, installed user files, or repository development instructions change.

## Verification

Existing metadata and packaging checks cover all agent renderings. The Status
contract test retains both claim-check and completion-authority routes.
Behavioral checks cover Plan's missing-scope boundary, a Status-to-claim-check
conversation, and Task implementation through required verification and review.
The run archive records the tested build and any environment limitations.
