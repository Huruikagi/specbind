# 0154: Guide project configuration through one completing workflow

Status: Accepted

## Context

SpecBind originally left customization to maintainers who knew the stable
project-owned settings boundary. That boundary now includes several coordinated
surfaces: install configuration and Agent capabilities, artifact and Roadmap
templates, agent-bound variables, shared Rules, operational adapters, and
Steering. The most valuable time to review them is immediately after install,
but install currently ends after materializing valid defaults.

The configuration work is semantic and project-specific, so a deterministic
CLI cannot choose good policy. Conversely, an Agent should not rediscover
configuration structure, ownership, and validity through filesystem searches.
Template changes also create a recurring aftercare question: the template owns
future materialization, while existing live artifacts own their current content
and durable instructions.

Decision 0096 currently embeds and installs only one `SKILL.md` per product
Skill. A configuration workflow covering every surface in one document would
load substantial irrelevant procedure on every invocation and make its routing
harder to maintain.

## Decision

### One completing configuration workflow

`specbind-configure` is the product-managed entry point for initial
post-install review and later supported configuration changes. It interprets
intent, classifies ownership, proposes and performs the change, invokes the
owning CLI or product Skill, revalidates the result, and completes authorized
aftercare. Delegating Steering or another content workflow does not end its
responsibility.

The workflow covers:

- selected Agents, artifact language, project instructions, and role capability
  overrides;
- project-owned Spec, Steering, and Roadmap templates;
- agent-bound template variables and Design-template selection;
- the closed shared-Rule and operational-adapter catalogs; and
- durable Steering through `specbind-steering`.

It never directly edits product-managed Skills, generated Agent roles,
protocols, schemas, managed root-instruction blocks, or CLI-owned lifecycle
state. Unsupported root movement, semantic artifact revision, lifecycle
mutation, removal, Git operations beyond the active adapter-directed
checkpoint, and external action retain their owning workflows and
authorization boundaries.

### Progressive product-Skill packages

Decision 0096's one neutral authored source expands from one document to one
package. Every product Skill still has exactly one agent-neutral `SKILL.md`.
It may additionally carry product-managed files below `references/`, each
installed byte-for-byte beside the rendered entrypoint. `SKILL.md` directly
names every conditional reference and when to read it; references do not form a
second nested routing tree.

Install refresh replaces every known package file under the same clean Git
guard as `SKILL.md`. Agent removal and project uninstall enumerate those exact
known files and preserve shared Codex/generic targets through the remaining-
Agent union. Unknown files are never inferred from a directory prefix.

The configuration package keeps common orchestration and authority boundaries
in `SKILL.md`, with separate direct references for installation and Agents,
templates and reconciliation, Rules, adapters, Steering, and common aftercare.

### Mechanical configuration summary

The read-only command is:

```text
specbind configuration show
```

It validates and reports:

- configured Spec directory, language, Agents, and project-instruction state;
- effective default or overridden Agent role capability;
- Spec, Steering, and Roadmap template inventory and source state;
- Rule presence, current-default equality, and Design-selection validity;
- adapter state and current-default equality; and
- Steering document count.

The command reports only provable facts. `current-default` means exact equality
with the current embedded asset; `project-content` means only that the project
copy differs. It never claims that content was intentionally customized.
Embedded template fallback, absence, scaffold, active state, and invalid
diagnostics retain their existing catalog meanings.

There is no project-wide configured or ready flag. Defaults may be a deliberate
valid choice, Steering is optional, and later releases may add configuration
surfaces. The Skill combines this mechanical summary with repository evidence
and maintainer intent.

### Post-install entry

A successful initial install apply adds a next-action line asking the maintainer
to have the installed Agent review the project through `specbind-configure`.
The line does not appear in dry-run output, ordinary refresh output, or
`INSTALL_UP_TO_DATE`, and does not describe valid defaults as an error.

### Aftercare

Every configuration mutation ends with one impact pass:

- required work restores validity or regenerates a derived surface;
- recommended work improves consistency without being required for a valid
  configuration; and
- optional work crosses into additional content, lifecycle, destructive, Git
  operations beyond the active adapter-directed checkpoint, or external state
  and retains a separate choice.

After required and authorized aftercare, the configuration change is one
eligible workflow unit under Decision 0137. The Skill follows the active Git
adapter and stages only that unit. Push, branch changes, tags, publication, and
history rewriting remain separate authority boundaries.

Declining optional aftercare completes the requested configuration change and
leaves an explicit report of the remaining effect.

After a template change, the Skill states that future materialization changed
and offers existing-artifact reconciliation. Acceptance first authorizes only a
preview. The preview enumerates candidates, lifecycle state, preserved
identities, and classifies each proposal as format-only, instruction-update,
structural, semantic, or conflict. A separate confirmation authorizes eligible
writes. Semantic changes route through the artifact's owning Skill, and any
resulting review, approval, or completion rerun is another boundary.

Reconciliation never directly rewrites `spec.yaml`, `tasks.yaml`, Gate or
completion evidence, released archives or logs, CLI-owned Roadmap Front Matter,
or durable artifact identities. A matching selector and target path identify a
candidate, not stored proof of which historical template created it.

## Consequences

- A maintainer can request an outcome without first knowing SpecBind's file and
  workflow ownership map.
- Initial installation leads directly to the highest-value project review while
  keeping valid defaults optional.
- The CLI supplies deterministic inventory and validity; the Agent supplies
  project-specific judgment and content.
- Product Skills can use progressive disclosure without per-Agent duplication
  or unmanaged installed resources.
- Template maintenance and existing-artifact mutation remain visibly separate,
  including their lifecycle cost.

## Implementation status

Implemented by the `specbind-configure` package, generalized product-Skill
package installation and removal, the configuration read model and CLI command,
the initial-install next action, focused conformance and CLI tests, public
Japanese guidance, and behavioral forward-test scenarios.
