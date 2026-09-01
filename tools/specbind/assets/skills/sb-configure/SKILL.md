---
name: sb-configure
description: Configure SpecBind for a project and complete the resulting aftercare. Use for initial post-install review or later changes to Agents, role models, artifact language, project instructions, templates, shared Rules, operational adapters, Steering, or existing artifacts affected by configuration changes.
argument-hint: "[what should change, or review the current configuration]"
---

# Configure SpecBind for this project

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

Own the whole configuration run. Translate the maintainer's outcome into the
supported configuration surface, use the owning CLI command or product Skill,
validate the result, and finish authorized aftercare. Delegating one step does
not delegate this responsibility.

## 1. Read the mechanical summary

Start every run with:

```sh
specbind configuration show
```

Any `ERROR` stops configuration until its named surface is repaired. The
summary reports mechanically provable state, not whether the project's choices
are good. Inspect only the repository evidence needed to make that judgment.

## 2. Classify the request and load only its procedure

Read every directly applicable reference completely before proposing or making
that part of the change. Do not load unrelated procedures.

- Agent selection, language, project instructions, or role capability:
  [installation and Agents](references/installation-and-agents.md)
- Artifact, Steering, or Roadmap template; Design-template selection; or
  existing-artifact alignment:
  [templates and reconciliation](references/templates-and-reconciliation.md)
- Shared authoring or judgment policy: [Rules](references/rules.md)
- Release, Git, or deferred-finding policy: [adapters](references/adapters.md)
- Durable project knowledge: [Steering](references/steering.md)

Read [aftercare](references/aftercare.md) after every mutation and before
reporting completion.

When one request spans several surfaces, state the dependency order and complete
one coherent surface at a time. Re-read `configuration show` after each change
that affects the inputs of the next.

## 3. Preserve ownership and authority

- Edit only project-owned configuration and durable knowledge through their
  accepted paths and owning workflows.
- Never edit installed product-managed Skills, generated Agent roles,
  protocols, schemas, root managed blocks, or CLI-owned lifecycle state by
  hand.
- A mutating configuration request follows the active Git adapter for its
  narrow eligible local checkpoint. The adapter alone grants no push, branch,
  tag, publication, removal, or history-rewrite authority.
- Configuration, existing-artifact reconciliation, lifecycle reapproval, Git
  operations beyond that eligible checkpoint, and external publication are
  separate authorization boundaries.
- Unsupported `specDir` movement, product-policy changes, semantic artifact
  changes, and lifecycle mutations route to their ordinary owning workflow.
  Never approximate them through file edits.

## 4. Propose and perform the change

Show the current state, intended state, exact owned surfaces, validation, and
expected aftercare. Ask only when the request leaves a material choice or a
separate authority boundary unresolved. A direct request to make an ordinary,
reversible project-owned configuration edit authorizes that edit and its
mechanical validation.

Use the applicable reference. When it routes to another product Skill, follow
that Skill's current contract rather than copying or weakening it here. Resume
this workflow after it finishes, inspect what actually changed, and continue
through aftercare.

## 5. Verify and report

Run `specbind configuration show` again. Run every surface-specific read or
check named by the applicable reference. Do not call a change complete because
the file exists or a delegated Skill said it succeeded.

Report in the configured project language:

- what configuration changed;
- what verification passed;
- required aftercare completed;
- recommended or optional aftercare completed or declined; and
- remaining lifecycle or external-action boundaries.

Keep it concise. The files and CLI state are the evidence.
