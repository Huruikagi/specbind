# Current generated artifact index

This page is a lightweight index of the files installed or maintained by the current SpecBind CLI and skill set. It records the inherited file surface before SpecBind-specific commands and workflows are redesigned; it is not a commitment to preserve these paths or formats.

`{{KIRO_DIR}}` defaults to `.kiro`. For the commands that manage these files, see the [current generated skill index](./current-skill-index.md).

## Settings templates

The CLI installs these files under `{{KIRO_DIR}}/settings/templates/` for both supported agents.

| Path under `settings/templates/` | Current role |
| --- | --- |
| `specs/init.json` | Initial `spec.json` structure. |
| `specs/requirements-init.md` | Initial requirements document created with a spec. |
| `specs/requirements.md` | Full requirements document structure. |
| `specs/design.md` | Technical design document structure. |
| `specs/research.md` | Research and gap-analysis log structure. |
| `specs/tasks.md` | Implementation task document structure. |
| `steering/product.md` | Core product steering structure. |
| `steering/tech.md` | Core technology steering structure. |
| `steering/structure.md` | Core repository-structure steering format. |
| `steering-custom/api-standards.md` | Optional API standards steering template. |
| `steering-custom/authentication.md` | Optional authentication steering template. |
| `steering-custom/database.md` | Optional database steering template. |
| `steering-custom/deployment.md` | Optional deployment steering template. |
| `steering-custom/error-handling.md` | Optional error-handling steering template. |
| `steering-custom/security.md` | Optional security steering template. |
| `steering-custom/testing.md` | Optional testing steering template. |

## Shared rules

The rule sources live under `tools/specbind/templates/shared/settings/rules/`. The current CLI copies each referenced rule into the consuming skill's `rules/` directory:

- Claude Code: `.claude/skills/<skill>/rules/`
- Codex: `.agents/skills/<skill>/rules/`

The current CLI does not create `{{KIRO_DIR}}/settings/rules/`.

| Rule | Installed for |
| --- | --- |
| `design-discovery-full.md` | `kiro-spec-design` |
| `design-discovery-light.md` | `kiro-spec-design` |
| `design-principles.md` | `kiro-spec-design` |
| `design-review-gate.md` | `kiro-spec-design` |
| `design-review.md` | `kiro-validate-design` |
| `design-synthesis.md` | `kiro-spec-design` |
| `ears-format.md` | `kiro-spec-requirements` |
| `gap-analysis.md` | `kiro-validate-gap` |
| `requirements-review-gate.md` | `kiro-spec-requirements` |
| `steering-principles.md` | `kiro-steering`, `kiro-steering-custom` |
| `tasks-generation.md` | `kiro-spec-tasks` |
| `tasks-parallel-analysis.md` | `kiro-spec-tasks` |

## Steering artifacts

Skills create or update these persistent project-level files under `{{KIRO_DIR}}/steering/`.

| Path | Current role | Created or updated by |
| --- | --- | --- |
| `product.md` | Product purpose, value, and capabilities. | `kiro-steering` |
| `tech.md` | Technology choices and engineering conventions. | `kiro-steering` |
| `structure.md` | Repository organization and naming patterns. | `kiro-steering` |
| `roadmap.md` | Multi-spec scope, ordering, and progress. | `kiro-discovery`, `kiro-spec-batch` |
| `<name>.md` | Specialized project guidance. | `kiro-steering-custom` |

## Spec artifacts

Skills create or update these feature-level files under `{{KIRO_DIR}}/specs/<feature>/`.

| Path | Current role | Created or updated by |
| --- | --- | --- |
| `brief.md` | Discovery context, scope, and candidate boundaries. | `kiro-discovery` |
| `spec.json` | Spec metadata, phase, language, and approval state. | `kiro-spec-init`, `kiro-spec-requirements`, `kiro-spec-design`, `kiro-spec-tasks`, `kiro-spec-quick` |
| `requirements.md` | Initial project description and approved requirements. | `kiro-spec-init`, `kiro-spec-requirements`, `kiro-spec-quick` |
| `research.md` | Gap analysis, discovery findings, and design decisions. | `kiro-validate-gap`, `kiro-spec-design` |
| `design.md` | Approved technical design. | `kiro-spec-design`, `kiro-spec-quick` |
| `tasks.md` | Approved implementation tasks, progress, and implementation notes. | `kiro-spec-tasks`, `kiro-impl`, `kiro-spec-quick` |

`kiro-spec-batch` orchestrates the normal spec phase skills for multiple features, so their outputs are represented by the same spec rows above.

## Sources of truth

- Settings templates and shared rule sources: `tools/specbind/templates/shared/settings/`
- Skill output behavior: `tools/specbind/templates/agents/{claude-code-skills,codex-skills}/skills/`
- Installation manifests: `tools/specbind/templates/manifests/{claude-code-skills,codex-skills}.json`
- Shared-rule expansion: `tools/specbind/src/plan/sharedRules.ts` and `tools/specbind/src/plan/fileOperations.ts`

When installed paths or skill outputs change, update this index together with the templates, manifests, and skill instructions so it remains a useful migration checklist.
