# Templates and existing-artifact reconciliation

Use this procedure for Spec, Steering, or Roadmap templates, agent-bound
variables, Design-template selection, and alignment of existing artifacts.

## Inspect and edit

List the applicable scope and read the selected raw template before editing:

```sh
specbind template list <scope>
specbind template read <scope> <selector>
```

For a Spec target that already exists, resolve its exact path:

```sh
specbind template resolve spec <spec> <selector>
```

Write only to the reported project-owned template path. Preserve the OKF
profile, selector, output path, machine-owned Front Matter boundary, and every
complete scoped-instruction block.

Every `{{name}}` body variable needs exactly one `create bind=name`
instruction. Resolve no value while editing the template. Variables are
forbidden in Front Matter. Do not restore a removed CLI renderer or implement a
second template language.

## Design template set

Treat every discovered `design/<artifact_id>` template and
`design-template-selection` as one configuration transaction. Classify every
candidate exactly once as `required`, `conditional`, or `disabled`; give every
conditional entry an applicability condition. Re-run both the template list and
Rule read before completion.

## Default effect

A template edit affects future materialization only. State explicitly that no
existing artifact changed, then offer reconciliation. Declining completes the
template change without touching live artifacts.

## Reconciliation preview

When accepted for preview:

1. Preserve the pre-change template from Git and compare it with the new one.
2. Enumerate Specs through `specbind spec list`; use `template resolve` for the
   exact target of each applicable selector. Use the corresponding Steering or
   Roadmap inventory for those scopes.
3. Read each live managed Markdown artifact through its owning CLI read surface.
4. Classify the proposal as `format-only`, `instruction-update`, `structural`,
   `semantic`, or `conflict`.
5. Present a per-artifact diff or precise change summary, preserved identities,
   and lifecycle effect without writing.

Template provenance is not stored in live artifacts. A matching selector and
path makes an artifact a candidate, not proof that it was materialized from the
previous template.

## Apply only a separately confirmed reconciliation

Preserve authored semantic content, Requirement and Contract identities,
artifact IDs, machine-owned fields, and durable instruction blocks unless the
preview explicitly covers an instruction update. Route semantic changes through
the ordinary Requirements, Design, Steering, or other owning Skill.

Never directly rewrite `spec.yaml`, `tasks.yaml`, Gate or completion evidence,
released archives or logs, or CLI-owned Roadmap Front Matter.

Artifact reconciliation and any resulting review, approval, or completion rerun
are separate confirmations. Byte changes can stale fingerprints even when the
agent classifies the edit as format-only.

## Verify

Re-list and re-read the template. For reconciled artifacts, run their parser,
traceability, status, and lifecycle checks. Fix structural invalidity; do not
invent semantic approval to make a check pass.
