# Current generated artifact index

This page indexes the files the current SpecBind CLI installs and the artifacts
the current CLI and product-managed skills maintain. `{{SPEC_DIR}}` is configured
in `.specbind.json` and defaults to `.specbind` for a new installation.

For the workflows that own these files, see the
[current generated skill index](./current-skill-index.md). For their design
history and detailed lifecycle, see the
[target artifact catalog](https://github.com/Huruikagi/specbind/blob/main/docs/design/target-artifact-catalog.md).

## Installation surface

`specbind install` creates absent project-owned settings, refreshes
product-managed skills, and optionally maintains a marked block in each selected
agent's root instruction file. Existing project-owned settings are kept.

`specbind remove-agent` plans by default and removes only one selected agent's
exact product-managed Skills, role files, marked instruction block, and config
entries when rerun with `--apply`. `specbind uninstall --knowledge retain|remove`
likewise plans before applying and requires an explicit choice to retain or
remove the configured complete `{{SPEC_DIR}}` knowledge bundle. Both operations
use `.specbind.json` as the final completion marker and never uninstall the
machine-level binary.

| Target | Current behavior |
| --- | --- |
| `.specbind.json` | Versioned project configuration containing the Spec root, artifact language, selected agents, optional project-instruction integration, and optional agent-role capability overrides. |
| `{{SPEC_DIR}}/settings/templates/specs/requirements.md` | Project-owned Requirements structure and authoring scaffold. |
| `{{SPEC_DIR}}/settings/templates/specs/design.md` | Project-owned Design structure and authoring scaffold. |
| `{{SPEC_DIR}}/settings/rules/ears-format.md` | Project Requirements style preferences. |
| `{{SPEC_DIR}}/settings/rules/design-principles.md` | Project Design preferences. |
| `{{SPEC_DIR}}/settings/rules/contract-principles.md` | Project seam and compatibility policy. |
| `{{SPEC_DIR}}/settings/rules/tasks-generation.md` | Project task-decomposition preferences. |
| `{{SPEC_DIR}}/settings/rules/steering-principles.md` | Project steering-authoring preferences. |
| `{{SPEC_DIR}}/settings/adapters/release.md` | Project-owned release preparation, publication, verification, and cleanup guidance. |
| `{{SPEC_DIR}}/settings/adapters/git.md` | Active default policy that commits each eligible workflow unit locally, without pushing or rewriting history. |
| `{{SPEC_DIR}}/settings/adapters/deferred.md` | Project destination for real review findings that do not hold a gate. |
| `.claude/skills/<skill>/SKILL.md` | Product-managed Claude Code rendering of each of the 18 embedded skills. |
| `.agents/skills/<skill>/SKILL.md` | Product-managed Codex rendering of each of the 18 embedded skills. |
| `.codex/agents/specbind-*.toml` | Product-managed Codex role adapters for planning, implementation, review, diagnosis, and bounded research; model capability may be overridden through `.specbind.json`. |
| `.claude/agents/specbind-*.md` | Product-managed Claude Code role adapters for the same five roles; model capability may be overridden through `.specbind.json`. |
| `CLAUDE.md` / `AGENTS.md` marked block | Optional product-managed project instruction block; surrounding project text is preserved. |

The binary also embeds six Spec scaffolds (`brief`, `research`, `requirements`,
`design/main`, `contract`, and `implementation-notes/main`) and four Steering
scaffolds (`product`, `tech`, `structure`, and author-identified `document`) in
English and Japanese. `template list/read` exposes all of them. Only Requirements
and Design are installed by default; a project can override any selector under
`settings/templates/` deliberately. `template resolve spec <spec> <selector>`
reports the selected source and exact SpecBind-root-relative target path without
writing it.

Every template instruction explicitly names `create`, `maintain`, or `consume`.
Materialization removes `create` and carries the two durable scopes into the
live artifact. `artifact read` and `steering read` preserve exact raw Markdown
by default and accept `--for maintain` or `--for consume` to omit the unrelated
durable instruction scope. `rule list/read` expose the five fixed project-owned
rule selectors without scanning the directory; rule reads provide the same raw,
maintain, and consume modes and reject live `create` instructions.

Twelve immutable product protocols and the versioned structured-artifact and
command-input schemas are binary-owned read surfaces exposed by
`protocol list/read` and `schema list/read`; they are not installed as project
settings.

## Project-level lifecycle artifacts

| Artifact or path | Current lifecycle and owner |
| --- | --- |
| `{{SPEC_DIR}}/adoption/reverse-discovery.yaml` | Temporary Git-tracked evidence and reconciliation ledger created by `specbind-adopt-existing`, then deleted after every accepted Spec has a complete Brief and Research handoff. |
| `{{SPEC_DIR}}/deferred.md` | Optional project-wide OKF concept created by the default deferred adapter when the first non-blocking finding is recorded. It is not a gate, fingerprint input, lifecycle artifact, or source of work. |
| `{{SPEC_DIR}}/steering/roadmap.md` | CLI-owned current active-milestone scope, dependency, baseline, release-binding, and Direct-status record; discovery confirms its authored scope. |
| `{{SPEC_DIR}}/steering/<path>.md` | Optional durable `SpecBind Steering` collection authored by `specbind-steering` and selected by `artifact_id`. |
| `{{SPEC_DIR}}/state/contract-review.md` | Current accepted milestone-wide Contract review for a Spec-backed milestone; authored by `specbind-contract-review` and persisted by the CLI. |
| `{{SPEC_DIR}}/releases/<version>-roadmap.md` | Final released Roadmap archive written by release finalization. |
| `{{SPEC_DIR}}/releases/<version>-contract-review.md` | Final accepted Contract-review archive for a Spec-backed release. |

## Per-Spec artifacts

The canonical Spec directory is `{{SPEC_DIR}}/specs/<spec>/`.

| Artifact | Current lifecycle and owner |
| --- | --- |
| `spec.yaml` | Persistent structured lifecycle, active-change, Requirement-selection, gate, and completion state maintained only through guarded CLI operations. |
| `requirements.md` | Persistent complete current Requirements maintained by `specbind-requirements`. |
| `design.md` or another `SpecBind Design` document | Persistent Design collection maintained by `specbind-design`; `artifact_id` is its stable selector. |
| `contract.md` | Persistent canonical five-section Contract maintained with Design and reviewed milestone-wide. |
| `implementation-notes.md` or another `SpecBind Implementation Notes` document | Optional persistent implementation memory collection. |
| `brief.md` | Active-milestone input authored by discovery and removed by successful release finalization. |
| `research.md` | Optional active-milestone gap-analysis result replaced by `specbind-gap-analysis` and removed by finalization. |
| `tasks.yaml` | Canonical active-milestone task plan and sparse execution state; authored by `specbind-tasks`, progressed by implementation, and removed by finalization. |
| `log.md` | Persistent newest-first release history maintained by release finalization for Spec-backed milestones. |

Markdown artifacts are discovered by their OKF type and, for collections, their
`artifact_id`; the default filenames above are materialization paths rather than
general semantic identity. `spec.yaml`, `tasks.yaml`, `roadmap.md`, and the
Contract-review state keep their accepted fixed structured paths.

## Sources of truth

- Installation plan and ownership behavior: `tools/specbind/src/installation/install.rs`
- Agent removal and project uninstall behavior: `tools/specbind/src/installation/removal.rs`
- Embedded templates, rules, adapters, protocols, and skills: `tools/specbind/assets/`
- Artifact discovery and lifecycle I/O: `tools/specbind/src/artifacts.rs`
- Structured wire models and generated schemas: `tools/specbind/src/schema/` and `tools/specbind/schemas/`
- Guarded lifecycle mutations: `tools/specbind/src/lifecycle/approval.rs`, `tools/specbind/src/lifecycle/milestone/`, `tools/specbind/src/lifecycle/completion/`, and `tools/specbind/src/lifecycle/release_finalize.rs`
