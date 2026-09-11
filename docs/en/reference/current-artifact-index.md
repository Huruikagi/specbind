# Artifact index

This page lists the files the current SpecBind CLI installs into a project and
the artifacts its CLI and Skills maintain. For the workflows that own them, see
the [skill index](./current-skill-index.md).

Paths below use the default Spec root, `.specbind/`. If your `.specbind.json`
sets a different `specDir`, read `.specbind/` as that directory.

## Who owns each file

| Owner | Meaning |
| --- | --- |
| **Project** | Yours to edit. `specbind install` never overwrites it. |
| **Skill** | Project content that you keep current through its owning Skill rather than by hand. |
| **CLI** | Structured state written only by guarded CLI operations. Never edit by hand. |
| **Product** | Product-managed. `specbind install` replaces it with the embedded version; do not edit. |

## At a glance

```text
.specbind.json                          Project   Project configuration
.specbind/
├─ index.md                             Project   Entry point; the marked block is Product
├─ settings/
│  ├─ templates/                        Project   Artifact scaffolds
│  ├─ rules/                            Project   Shared judgment rules (7 fixed files)
│  └─ adapters/                         Project   Operational guidance (4 fixed files)
├─ steering/
│  ├─ roadmap.md                        CLI       Active Milestone
│  └─ <path>.md                         Skill     Steering documents
├─ specs/
│  ├─ shared-contract.yaml              Skill     Shared Contract (optional)
│  └─ <spec>/                                     Per-Spec artifacts (see below)
├─ state/                               CLI       Accepted Contract review
├─ releases/                            CLI       Release archives
├─ baselines/                           CLI       Spec establishment archives
├─ deferred.md                          Skill     Deferred findings (optional)
└─ adoption/                            Skill     Present only while establishing Specs
.agents/skills/sb-*/                    Product   Skills for Codex and generic agents
.claude/skills/sb-*/                    Product   Skills for Claude Code
.codex/agents/specbind-*.toml           Product   Codex role definitions
.claude/agents/specbind-*.md            Product   Claude Code role definitions
AGENTS.md / CLAUDE.md (marked block)    Product   Project instructions (optional)
```

## Project configuration

| File | Owner | Contents |
| --- | --- | --- |
| `.specbind.json` | Project | Spec root, artifact language, selected Agents, project-instruction integration, and optional `agentRoles` overrides. `specDir` is fixed at installation; change Agents and language through `specbind install`. |
| `.specbind/index.md` | Project | Shared entry point for all artifacts. SpecBind refreshes only its localized marked block; add project links outside the markers. |

## Settings: `.specbind/settings/`

Created by the first installation and never overwritten afterward. See
[Customize SpecBind](../guide/customization.md) for how to edit them.

### `templates/`

| File | Contents |
| --- | --- |
| `specs/requirements.md` | Requirements structure and authoring scaffold |
| `specs/design.md` | Main Design scaffold, required for every Spec |
| `specs/ui.md` | Conditional screen-design scaffold |
| `roadmap.md` | Body of the Milestone Roadmap; Front Matter stays CLI-owned |

Other scaffolds are embedded in the binary and can be listed with
`specbind template list`. Copy one to its reported `template_path` only when
you want to override it.

### `rules/`

| File | Contents |
| --- | --- |
| `ears-format.md` | Requirements style |
| `design-principles.md` | Design preferences |
| `design-template-selection.md` | Whether each Design template is required, conditional, or disabled |
| `contract-principles.md` | Ownership, seams, and compatibility policy |
| `tasks-generation.md` | Task decomposition preferences |
| `steering-principles.md` | Steering authoring preferences |
| `language-style.md` | Prose style; installed only for Japanese |

### `adapters/`

| File | Contents |
| --- | --- |
| `release.md` | Release preparation, publication, verification, and cleanup |
| `git.md` | Commit policy; the default commits each workflow unit locally without pushing |
| `deferred.md` | Where non-blocking review findings go |
| `validation.md` | Extra project-specific checks for final implementation validation |

## Milestone and project state

| File | Owner | Lifetime and contents |
| --- | --- | --- |
| `steering/roadmap.md` | CLI | Active Milestone scope, dependencies, target release, and Direct item status. Discovery authors the body. Moved to `releases/` at release. |
| `steering/<path>.md` | Skill | Durable Steering documents, maintained by `sb-steering`. |
| `state/contract-review.md` | CLI | Current accepted Milestone-wide Contract review, authored by `sb-contract-review`. |
| `state/cc-sdd-migration.yaml` | CLI | Present only during a cc-sdd migration. |
| `releases/<version>-roadmap.md` | CLI | Archived Roadmap of a released Milestone. |
| `releases/<version>-contract-review.md` | CLI | Archived Contract review of a released Milestone. |
| `baselines/<version>-roadmap.md` / `baselines/<version>-contract-review.md` | CLI | Archived Roadmap and Contract review from establishing Specs with `sb-adopt`. Not a release record. |
| `specs/shared-contract.yaml` | Skill | Optional shared Contract for resources used by several features. Survives release. See [Customize SpecBind](../guide/customization.md#shared-contract). |
| `deferred.md` | Skill | Created by the default deferred adapter when the first non-blocking finding is recorded. Not a work queue. |
| `adoption/reverse-discovery.yaml` | Skill | Temporary evidence ledger of `sb-adopt`, deleted when Spec establishment completes. |

## Per-Spec artifacts: `.specbind/specs/<spec>/`

| File | Owner | Lifetime | Contents |
| --- | --- | --- | --- |
| `spec.yaml` | CLI | Durable | Lifecycle, Gate, and completion state |
| `requirements.md` | Skill | Durable | Complete current Requirements (`sb-plan`) |
| `design.md` and other Design documents | Skill | Durable | Design collection (`sb-plan`) |
| `contract.yaml` | Skill | Durable | Contract, reviewed Milestone-wide |
| `log.md` | CLI | Durable | Release history, newest first |
| `implementation-notes.md` | Skill | Durable, optional | Implementation memory |
| `brief.md` | Skill | Active Milestone | Discovery's summary and relevant sources |
| `research.md` | Skill | Active Milestone, optional | Gap-analysis result (`sb-gap-analysis`) |
| `tasks.yaml` | Skill and CLI | Active Milestone | Task plan (`sb-plan`) and execution progress (CLI) |

Active-Milestone artifacts are removed when the release is finalized. Markdown
artifacts are identified by their type and `artifact_id`, so the filenames above
are defaults rather than required names.

## Embedded in the binary

These are read through the CLI and never installed as project files:

- Spec, Steering, and Roadmap templates: `specbind template list` / `template read`
- Product protocols: `specbind protocol list` / `protocol read`
- Structured artifact and command-input schemas: `specbind schema list` / `schema read`

See [Customize SpecBind](../guide/customization.md) for template instructions
and overrides.
