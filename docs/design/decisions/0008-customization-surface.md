# 0008: Keep project customization in shared settings

Status: Accepted

## Context

cc-sdd defines two project-wide customization points under its settings root: templates control generated document structure and rules control AI judgment and generation principles. The current SpecBind snapshot instead installs shared templates under the spec root while copying referenced rules into agent-specific skill directories.

SpecBind supports multiple coding agents. Treating skill-local copies as the primary customization surface would duplicate project policy, allow Claude Code and Codex behavior to drift, and make generated skill updates collide with user changes.

## Decision

- Preserve the cc-sdd customization model with shared `{{SPEC_DIR}}/settings/templates/` and `{{SPEC_DIR}}/settings/rules/` directories.
- `settings/templates/` is the supported customization surface for the structure and format of generated requirements, design, tasks, steering, and other documented artifacts.
- `settings/rules/` is the supported customization surface for project-wide AI judgment criteria and generation principles.
- Installed project copies are user-owned and may be edited and version-controlled after installation.
- Supported agent skills consume the same shared settings so project policy does not fork by agent.
- Skills, agent metadata, and installation manifests remain SpecBind-managed resources. Direct local edits may be preserved for safety, but they are not the stable project-customization API.
- Project-owned settings with dedicated contracts, such as `settings/release.md`, remain customizable alongside templates and rules.
- Whether official defaults are embedded in the Rust binary or distributed beside it is a packaging decision and does not change the installed customization contract.

## Update behavior

- Installation and update operations must not silently overwrite customized settings.
- The CLI should distinguish untouched defaults from project-modified settings where practical and present an explicit merge, keep, or replace decision.
- New default files may be offered without rewriting unrelated customized files.
- Agent-specific generated resources may be updated independently from shared settings, subject to the normal conflict and backup policy.
- A custom installation manifest is an advanced development or integration input, not a required end-user customization mechanism.

## Structural contract

Customization remains subject to the machine-readable contracts required by SpecBind workflows. Templates and rules may change prose, sections, examples, and guidance, but required identifiers, mappings, or state fields must remain parseable according to the documented schema.

The bundled CLI should report incompatible customizations with focused diagnostics. It must not silently reinterpret an unsupported structure or use agent-specific ad hoc parsing as a fallback.

[Decision 0059](./0059-okf-artifact-templates.md) defines spec Markdown templates as final-form OKF artifact prototypes: relative template paths determine initial output paths, literal frontmatter determines machine identity, and explicit `specbind:instruction` HTML comments carry template-only AI guidance. Existing artifacts are never silently reconciled to later template edits.

## Consequences

- Project customization survives changes of coding agent.
- Generated skills can become thinner and reference shared rules instead of carrying editable duplicated copies.
- The target artifact catalog must treat templates and rules as user-owned settings after installation.
- The Rust migration needs upgrade fixtures for untouched, customized, and newly introduced settings files.
- Documentation must distinguish product-managed embedded defaults from user-owned installed copies.

## Open questions

- The exact merge and provenance mechanism for detecting untouched versus customized settings.
- Which non-spec template families should adopt the Decision 0059 prototype model and which require a different structural contract.
- Whether projects can layer organization-wide settings above repository-local settings in a later release.
