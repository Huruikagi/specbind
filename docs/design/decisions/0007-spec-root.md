# 0007: Default the configurable spec root to `.specbind`

Status: Accepted

Decisions 0077 and 0081 supersede the compatibility-alias details below. V1 accepts only `specDir`, performs legacy handling through explicit `specbind migrate cc-sdd`, and applies the stricter project-child, submodule, and link-traversal rules in Decision 0081.

## Context

The current TypeScript CLI allows the project specification root to be changed with `--kiro-dir` or `.specbind.json`'s `kiroDir`. Its precedence is command-line value, persisted configuration, then the `.kiro` default.

SpecBind is now an independent product and its target artifacts already use the neutral `{{SPEC_DIR}}` placeholder. Keeping `.kiro` and Kiro-specific option names as the default interface would preserve inherited branding in every consumer project.

## Decision

- Keep the specification root configurable.
- Change the default root for new installations to `.specbind`.
- Use the product-neutral names `--spec-dir`, `specDir`, and `{{SPEC_DIR}}` in the Rust CLI, configuration, manifests, and templates.
- Preserve the precedence model: explicit command-line value, persisted configuration, then the `.specbind` default.
- Treat the current `--kiro-dir`, `kiroDir`, and `{{KIRO_DIR}}` names as migration inputs rather than the target interface.
- Honor an explicitly configured legacy path, including `.kiro`, during the compatibility period.
- Do not silently create a parallel `.specbind` tree when an existing project appears to use an implicit `.kiro` root. The CLI must detect the situation and require or guide an explicit migration decision.

## Migration expectations

- The Rust CLI reads the legacy `kiroDir` setting as a deprecated alias when `specDir` is absent.
- Supplying both old and new settings with different values is an error rather than an implicit precedence rule.
- A legacy `--kiro-dir` CLI alias may be supported temporarily, but new help and generated documentation advertise `--spec-dir` only.
- Migration from `.kiro` to `.specbind` must be a deliberate, guarded operation that updates generated references consistently and does not overwrite an existing target tree.
- Projects may continue to choose another repository-relative root such as `docs/specs`; `.specbind` is a default, not a mandatory directory name.

## Consequences

- New projects use SpecBind terminology and filesystem layout by default.
- Existing customized roots remain supported.
- Templates and manifests require a coordinated placeholder rename during implementation.
- Installation, checking, milestone, and release commands can resolve one shared `specDir` configuration.
- The migration needs collision detection and clear diagnostics for repositories containing `.kiro`, `.specbind`, or both.

## Open questions

- Whether the legacy CLI and configuration aliases last for one major release or longer.
- Whether the Rust CLI provides an explicit `specbind migrate root` command or performs the migration through `specbind install` with confirmation.
- How project-local customized skill content is distinguished from safely regenerable content during a directory migration.
