# 0007: Default the configurable spec root to `.specbind`

Status: Accepted

Decisions 0077 and 0081 complete the v1 root and migration contract. V1 accepts only `specDir`, performs legacy handling through explicit `specbind migrate cc-sdd`, and applies the project-child, submodule, and link-traversal rules in Decision 0081.

## Context

The current TypeScript CLI allows the project specification root to be changed with `--kiro-dir` or `.specbind.json`'s `kiroDir`. Its precedence is command-line value, persisted configuration, then the `.kiro` default.

SpecBind is now an independent product and its target artifacts already use the neutral `{{SPEC_DIR}}` placeholder. Keeping `.kiro` and Kiro-specific option names as the default interface would preserve inherited branding in every consumer project.

## Decision

- Keep the specification root configurable.
- Change the default root for new installations to `.specbind`.
- Use the product-neutral names `--spec-dir`, `specDir`, and `{{SPEC_DIR}}` in the Rust CLI, configuration, and templates.
- Preserve the precedence model: explicit command-line value, persisted configuration, then the `.specbind` default.
- Treat the current `--kiro-dir`, `kiroDir`, `{{KIRO_DIR}}`, and `.kiro` names only as inputs to explicit cc-sdd migration. They are not target aliases.
- Do not detect or implicitly adopt a legacy root during ordinary install or lifecycle commands.
- Require `specDir` to be a portable repository-relative child directory subject to Decision 0081. It may not be the project root, escape it, traverse a managed symbolic link or junction, or point inside a nested submodule.

## Migration expectations

- `specbind migrate cc-sdd` produces a read-only plan by default and mutates only with `--apply`.
- Migration handles only known, unambiguous cc-sdd artifacts and stops on ambiguous content instead of inventing aliases or precedence.
- Migration from `.kiro` to `.specbind` is deliberate and guarded, updates known generated references consistently, and never overwrites an existing target tree.
- Projects may continue to choose another repository-relative root such as `docs/specs`; `.specbind` is a default, not a mandatory directory name.

## Consequences

- New projects use SpecBind terminology and filesystem layout by default.
- Existing customized roots remain supported.
- Templates require a coordinated placeholder rename during implementation.
- Installation, checking, milestone, and release commands can resolve one shared `specDir` configuration.
- Explicit migration needs collision detection and clear diagnostics for repositories containing `.kiro`, `.specbind`, or both.
