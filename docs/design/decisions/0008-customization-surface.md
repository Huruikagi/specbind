# 0008: Keep project customization in shared settings

Status: Accepted

Decision 0077 removes public installation manifests and backup/conflict modes. Product-managed agent assets are replaced only from a clean committed state; existing user-owned settings files are never overwritten, while absent new defaults may be created.

## Context

cc-sdd defines two project-wide customization points under its settings root: templates control generated document structure and rules control AI judgment and generation principles. The current SpecBind snapshot instead installs shared templates under the spec root while copying referenced rules into agent-specific skill directories.

SpecBind supports multiple coding agents. Treating skill-local copies as the primary customization surface would duplicate project policy, allow Claude Code and Codex behavior to drift, and make generated skill updates collide with user changes.

## Decision

- Preserve the cc-sdd customization model with shared `{{SPEC_DIR}}/settings/templates/` and `{{SPEC_DIR}}/settings/rules/` directories.
- `settings/templates/` is the supported customization surface for the structure and format of generated requirements, design, tasks, steering, and other documented artifacts.
- `settings/rules/` is the supported customization surface for project-wide AI judgment criteria and generation principles.
- Installed project copies are user-owned and may be edited and version-controlled after installation.
- Supported agent skills consume the same shared settings so project policy does not fork by agent.
- Skills and agent metadata remain SpecBind-managed resources. Direct local edits are not the stable project-customization API.
- Project-owned operational guidance lives below `settings/adapters/` under dedicated contracts, alongside the template and rule customization surfaces; see Decision 0101.
- Official defaults are embedded in the Rust binary under Decision 0077; installed settings remain user-owned copies. [Decision 0091](./0091-installed-template-surface.md) limits the installed artifact-template set to those whose structure a project can actually change.

## Update behavior

- Initial install and later `specbind install` asset refreshes must not overwrite customized settings.
- Existing settings files are never overwritten or merged by install. Newly introduced defaults are created only when their target path is absent.
- Product-managed agent resources may be replaced only when repository guards permit the requested install refresh. SpecBind performs no backup or interactive overwrite policy.

## Structural contract

Customization remains subject to the machine-readable contracts required by SpecBind workflows. Templates and rules may change prose, sections, examples, and guidance, but required identifiers, mappings, or state fields must remain parseable according to the documented schema.

The bundled CLI should report incompatible customizations with focused diagnostics. It must not silently reinterpret an unsupported structure or use agent-specific ad hoc parsing as a fallback.

[Decision 0059](./0059-okf-artifact-templates.md) defines spec Markdown templates as final-form OKF artifact prototypes: relative template paths determine initial output paths, literal frontmatter determines machine identity, and explicit `specbind:instruction` HTML comments carry template-only AI guidance. Existing artifacts are never silently reconciled to later template edits.

[Decision 0092](./0092-template-skill-authoring-boundary.md) ensures that user-owned templates and rules can customize structure and authoring policy without becoming the sole authority for non-waivable skill behavior or deterministic CLI contracts.

[Decision 0093](./0093-default-shared-rule-set.md) narrows the installed shared-rule set to five explicit files and defines their skill-loading and cc-sdd migration boundaries.

[Decision 0094](./0094-embedded-product-protocols.md) keeps non-customizable shared semantic baselines out of project settings and exposes them as read-only CLI protocols.

## Consequences

- Project customization survives changes of coding agent.
- Generated skills can become thinner and reference shared rules instead of carrying editable duplicated copies.
- The target artifact catalog must treat templates and rules as user-owned settings after installation.
- The Rust migration needs upgrade fixtures for existing customized settings and newly introduced absent defaults.
- Documentation must distinguish product-managed embedded defaults from user-owned installed copies.

Organization-wide settings layers and additional template-family contracts remain possible post-v1 extensions, not unresolved parts of the repository-local v1 customization boundary.
