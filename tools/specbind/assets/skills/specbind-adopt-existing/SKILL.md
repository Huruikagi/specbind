---
name: specbind-adopt-existing
description: Adopt a selected part of an existing implementation into new Specs by establishing boundaries, retaining implementation evidence, and reconciling observed behavior into confirmed Brief intent before the normal lifecycle begins.
argument-hint: "<area to adopt, or entire repository>"
---

# Adopt an existing implementation

Turn an explicitly selected part of a brownfield repository into new SpecBind
Specs. Existing code and tests are **evidence**, never automatic authority for
what the product ought to promise.

This workflow has two invocations separated by ordinary Discovery:

1. establish candidate Spec boundaries and checkpoint the adoption dossier;
2. after Discovery creates the confirmed Specs and Briefs, investigate each
   Spec deeply, reconcile intent with the user, update its Brief and Research,
   and retire the project-level dossier.

Requirements, Design, Tasks, implementation, and approval remain owned by their
normal skills. Do not author or approve those artifacts here.

## 1. Determine whether this is a new run or a resume

Resolve the configured `specDir` from `.specbind.json`. The one project-level
dossier path is:

```text
<specDir>/adoption/reverse-discovery.yaml
```

If it does not exist, begin at **Start**. If it exists, begin at **Resume**.
Never create a second dossier and never infer a dossier from an ordinary
Research artifact.

## 2. Load only this invocation's procedure

Read the directly applicable reference completely before continuing:

- When the dossier does not exist, read [Start](references/start.md). It owns
  boundary investigation, the dossier, and the handoff to Discovery. Stop when
  that reference hands off; do not load the resume procedure in this invocation.
- When the dossier exists, read [Resume](references/resume.md). It owns baseline
  verification, per-Spec reverse discovery, intent reconciliation, and dossier
  retirement. Do not repeat the start procedure.

The references are two halves of this workflow separated by ordinary Discovery,
not optional interpretations of the same state. Follow the selected procedure
exactly and retain the common boundaries below.

## Boundaries

- Initial adoption supports a project with no persistent Specs and no active
  milestone. Later incremental reverse adoption is future work.
- Steering is mandatory for this workflow even though it remains optional for
  ordinary SpecBind work.
- Existing implementation and tests are evidence, not intended specification.
- This workflow owns the temporary dossier, per-Spec adoption Research, and the
  narrow user-confirmed Brief revision. It owns no lifecycle state or gate.
- Discovery owns Spec creation and initial Briefs. Requirements, Design, Tasks,
  implementation, and validation use their ordinary skills without a brownfield
  branch.
- Do not change implementation, tests, dependencies, configuration, or Steering
  while establishing the adoption baseline. Findings become later lifecycle
  work; they are not repaired during reverse discovery.
