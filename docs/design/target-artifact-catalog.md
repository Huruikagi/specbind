# Target artifact catalog

This document records the accepted lifecycle and implementation status for the
files the SpecBind workflow installs, creates, maintains, or removes. The
current concise surface is indexed in the
[current generated artifact index](../current-artifact-index.md).

The catalog is intentionally separate from the [current generated artifact index](../current-artifact-index.md):

- The current index records what the CLI and skills produce today.
- This catalog records accepted ownership, lifecycle, and remaining open questions.

Related documents:

- [Target skill catalog](./target-skill-catalog.md)
- [Target workflows](./target-workflows.md)
- [Active spec lifecycle](./active-spec-lifecycle.md)
- [Spec state machine](./spec-state-machine.md)
- [Milestone state machine](./milestone-state-machine.md)
- [Decision 0002: project release adapter](./decisions/0002-project-release-adapter.md)
- [Decision 0003: active requirement set](./decisions/0003-active-requirement-set.md)
- [Decision 0004: release history layout](./decisions/0004-release-history-layout.md)
- [Decision 0005: active change abandonment](./decisions/0005-active-change-abandonment.md)
- [Decision 0007: configurable `.specbind` root](./decisions/0007-spec-root.md)
- [Decision 0008: shared settings customization](./decisions/0008-customization-surface.md)
- [Decision 0011: cross-spec contract manifest](./decisions/0011-cross-spec-contract.md)
- [Decision 0012: delegated approval](./decisions/0012-delegated-approval.md)
- [Decision 0013: structured task artifact](./decisions/0013-structured-task-artifact.md)
- [Decision 0014: structured spec metadata](./decisions/0014-structured-spec-metadata.md)
- [Decision 0045: OKF Markdown artifacts](./decisions/0045-okf-markdown-artifacts.md)
- [Decision 0046: grouped roadmap work items](./decisions/0046-roadmap-work-items.md)
- [Decision 0047: sparse direct-change status](./decisions/0047-sparse-direct-change-status.md)
- [Decision 0048: OKF per-spec log](./decisions/0048-okf-spec-log.md)
- [Decision 0049: concise OKF authoring rule](./decisions/0049-okf-authoring-rule.md)
- [Decision 0057: type-based artifact discovery](./decisions/0057-type-based-artifact-discovery.md)
- [Decision 0059: OKF artifact templates](./decisions/0059-okf-artifact-templates.md)
- [Decision 0060: Requirement ID and heading mapping](./decisions/0060-requirement-id-and-heading-mapping.md)
- [Decision 0061: Design Requirement traceability](./decisions/0061-design-requirement-traceability.md)
- [Decision 0062: minimal active brief profile](./decisions/0062-minimal-active-brief-profile.md)
- [Decision 0063: free-form release adapter profile](./decisions/0063-free-form-release-adapter-profile.md)
- [Decision 0064: path-scoped release finalization guard](./decisions/0064-path-scoped-release-finalization-guard.md)
- [Decision 0066: agent-judged release and CLI log insertion](./decisions/0066-agent-judged-release-and-cli-log-insertion.md)
- [Decision 0067: text-first English CLI results](./decisions/0067-text-first-english-cli-results.md)
- [Decision 0068: release log summary input](./decisions/0068-release-log-summary-input.md)
- [Decision 0078: contract-first review between Design and Tasks](./decisions/0078-contract-first-review-between-design-and-tasks.md)
- [Decision 0079: milestone-local Research](./decisions/0079-milestone-local-research.md)
- [Decision 0080: v1 Task, Contract, and completion details](./decisions/0080-v1-task-contract-and-completion-details.md)
- [Decision 0081: v1 release, Git, path, and CLI safety](./decisions/0081-v1-release-git-path-and-cli-safety.md)
- [Decision 0082: derived milestone state machine](./decisions/0082-derived-milestone-state-machine.md)
- [Decision 0085: Rust wire-model schema generation](./decisions/0085-rust-wire-model-schema-generation.md)
- [Decision 0069: stateless release preflight](./decisions/0069-stateless-release-preflight.md)
- [Decision 0070: derived release readiness](./decisions/0070-derived-release-readiness.md)
- [Decision 0071: no partial milestone release](./decisions/0071-no-partial-milestone-release.md)
- [Decision 0072: explicit release rebinding](./decisions/0072-explicit-release-rebinding.md)
- [Decision 0073: portable release version](./decisions/0073-portable-release-version.md)
- [Decision 0074: defer JSON CLI output](./decisions/0074-defer-json-cli-output.md)
- [Decision 0092: template and skill authoring boundary](./decisions/0092-template-skill-authoring-boundary.md)
- [Decision 0093: default shared-rule set](./decisions/0093-default-shared-rule-set.md)
- [Decision 0094: embedded product protocols](./decisions/0094-embedded-product-protocols.md)
- [Decision 0101: project adapter directory and Git workflow](./decisions/0101-project-adapter-directory-and-git-workflow.md)

Status: Accepted and implemented for the v1 artifact set

`{{SPEC_DIR}}` is configurable and defaults to `.specbind` for new installations. Existing explicitly configured roots remain valid; migration from the inherited `.kiro` default must be deliberate and guarded.

## Lifecycle principles

- Specs are active specifications of the product, not disposable delivery plans.
- Existing specs should be maintained as the represented behavior changes.
- A milestone groups the work intended for an active release cycle.
- Every milestone has a `roadmap.md`, including a milestone containing only one spec change.
- At most one active milestone is represented by `roadmap.md`.
- Machine-generated milestone identity is separate from its optional, later-bound release version.
- A concrete release version is mandatory before release execution begins.
- Release completion ends the active milestone but does not retire the specs it changed.
- Abandoning an unreleased milestone does not create release history by default.
- The configured spec root is an OKF v0.2 Knowledge Bundle. Its managed Markdown artifacts use YAML frontmatter plus free-form Markdown under Decision 0045.
- Every OKF profile permits and preserves unknown top-level Front Matter extensions under Decision 0045, while known fields and nested SpecBind-owned structures retain their profile-specific validation.

## Project-level artifacts

| Artifact or target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/steering/roadmap.md` | Required for every active milestone, including single-spec work. | Intent confirmed by `specbind-discovery`, whose first creation uses the Decision 0145 project-owned body template; persisted by Rust CLI milestone operations; archived by `specbind-release`. | Accepted | An OKF concept with `type: SpecBind Roadmap`; its frontmatter holds a branch-safe UUID v7 milestone ID, the Decision 0054 branch-local Git baseline, an initially null Decision 0073 portable release-version binding, and grouped work items. Its free-form body preserves the milestone-wide request and rationale. It represents current state only under Decision 0051 and carries no detailed contract review evidence. An explicitly abandoned unreleased roadmap is removed rather than release-archived. |
| `{{SPEC_DIR}}/steering/<name>.md` collection (`artifact_id` required) | Optional durable project guidance that outlives any milestone. | Authored by `specbind-steering`; consumed by applicable skills through `specbind steering list/read`. | Accepted | An OKF concept with `type: SpecBind Steering` under Decision 0098, discovered recursively below `steering/` and identified by `artifact_id`, which is also its selector. There is no core-versus-custom split. It is never fingerprinted, never gate evidence, and never a freshness input, so guidance that materially changed a decision is recorded in a Spec Brief, the Roadmap body, or the later skill's owning canonical artifact. |
| `{{SPEC_DIR}}/adoption/reverse-discovery.yaml` | Exists only while the initial brownfield-adoption workflow is establishing new Specs from a committed implementation baseline. | Authored and retired by the existing-implementation references of `specbind-discovery`; committed through project Git policy so another session can resume. | Accepted | A transient versioned investigation ledger under Decisions 0143 and 0175, not lifecycle state or a strict wire model. It records the source revision, selected scope, boundary candidates, evidence-backed observations, and dispositions. It is deleted after all accepted candidates have complete Brief and Research handoffs, while Git retains its history. |
| `{{SPEC_DIR}}/state/contract-review.md` | Exists only while a milestone with Spec-backed items has a current accepted global contract review. | `specbind-contract-review` authors a candidate judgment; guarded Rust CLI operations persist it. | Accepted | OKF project-state concept under Decision 0078. Frontmatter retains milestone identity, pass time, and contract-first input revisions; the body preserves the free-form AI-authored judgment. It has no classifications or Markdown-profile schema version. |
| `{{SPEC_DIR}}/releases/<version>-roadmap.md` | Persists as the released milestone-wide scope and dependency record. | `specbind-release`. | Accepted | Each release adds the final active-roadmap snapshot as a new flat file after verified publication; archive collisions must not overwrite history. |
| `{{SPEC_DIR}}/releases/<version>-contract-review.md` | Persists the final accepted milestone-wide contract review evidence and judgment for a Spec-backed release. | `specbind-release`. | Accepted | Companion archive moved from `state/` before the Roadmap completion marker. It is absent for Direct-only releases. |

## Settings artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/settings/templates/` | Requirements, Design, and milestone Roadmap body defaults are installed, then maintained and version-controlled by the project; other embedded scaffolds may be overridden deliberately. | Project maintainers; consumed by authoring skills. | Accepted | Supported customization surface for generated document structure and format. Spec Markdown templates are final-form OKF prototypes under Decision 0059. Decision 0151 permits project-defined body variables; each distinct name is bound to one `create` instruction, may be referenced repeatedly, and is resolved by the authoring agent. Unfilled default scaffolds are not necessarily valid live artifacts. The Decision 0145 Roadmap template owns only milestone-wide prose while the CLI owns live Front Matter. Updates must not silently overwrite local changes or reconcile existing artifacts. |
| `{{SPEC_DIR}}/settings/rules/` | Seven selectors are accepted. Six language-neutral defaults are installed for every project, while the Japanese `language-style` default is offered only for `ja`; project copies are then maintained and version-controlled by the project. | Project maintainers; consumed by the explicit owning skills, with `language-style` consumed by every product Skill. | Accepted | Supported customization surface for shared judgment, generation, and prose preferences. Decisions 0093, 0152, and 0169 fix the paths, language-aware installation, consumers, absence behavior, and cc-sdd disposition; v1 does not recursively auto-load additional rule files. Non-customizable product protocols remain embedded under Decision 0094. |
| `{{SPEC_DIR}}/settings/adapters/release.md` | Installed as a scaffold and maintained as project configuration. | Project maintainers; consumed by `specbind-release`. | Accepted | Free-form OKF project guidance under Decisions 0063 and 0101. Its only known field is `type`; headings are not machine syntax and an empty body means no adapter-specific actions. It cannot override core release gates or evidence requirements. |
| `{{SPEC_DIR}}/settings/adapters/git.md` | Installed with active local-checkpoint defaults and maintained as optional project configuration. | Project maintainers; consumed by skills that may create Git checkpoints or push. | Accepted | Free-form OKF project guidance under Decisions 0101 and 0137. Each eligible workflow unit is committed locally by default; absence, an empty body, or a legacy instruction scaffold means no adapter-directed commit or push. The phase request authorizes only the narrow local checkpoint, never push or history rewriting. |
| `{{SPEC_DIR}}/settings/adapters/deferred.md` | Installed with a working default and maintained as project configuration. | Project maintainers; consumed by the reviewing and authoring skills that produce findings. | Accepted | Free-form OKF project guidance under Decisions 0101 and 0122. It names where a review finding that does not hold a gate is recorded. Its default destination prevents reviewers from escalating every real finding merely because no non-blocking destination was configured. The destination is written to, never read as a source of work. |
| `{{SPEC_DIR}}/deferred.md` | Created on first use by the working default in the deferred adapter. | Review workflows append; people decide whether an entry returns through the Roadmap. | Accepted | Project-wide `Deferred Findings` OKF concept under Decision 0131. It is outside SpecBind lifecycle management: no gate, fingerprint, archive handling, or scope authority. |

## Spec artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/specs/<feature>/` | Persists across milestones and releases while the represented capability remains active. | Spec authoring and maintenance skills. | Accepted | A release must not delete a spec merely because its milestone completed. |
| `SpecBind Brief` singleton (`brief.md` by default) | Exists only for one active milestone change. | `specbind-discovery`. | Accepted | Minimal free-form OKF input under Decision 0062. Its only known field is `type`; the CLI does not parse its body or fingerprint it for gate evidence. Same-milestone deltas merge into it, and successful release finalization removes it. |
| `SpecBind Requirements` singleton (`requirements.md` by default) | Holds the complete currently valid requirements across releases. | Requirements workflow. | Accepted | Discovered by OKF type. Front Matter maps the two customizable structural heading labels, while Requirement IDs derive from explicit group number plus Acceptance Criteria list position under Decision 0060. The active requirement set is a separate milestone-scoped concept. |
| `SpecBind Design` collection (`design.md`, `artifact_id: main` by default) | Holds the complete currently valid design across one or more focused documents. | Design workflow. | Accepted | Discovered by OKF type plus stable `artifact_id`; revised in place for an active change. Its stable v1 Front Matter contract is `type`, `artifact_id`, and non-empty `requirement_ids`; unknown project metadata is allowed but ignored by SpecBind. The mapping set exactly matches its italic `_Requirements: ..._` body markers under Decision 0061. |
| `contract.yaml` singleton | Holds the current minimal cross-spec seam manifest across releases. | Design and contract review workflows. | Accepted | Fixed strict versioned artifact under Decision 0155. Contains stable Owns, Exports, Consumes, Invariants, and File Ownership entries; never an internal-design summary. |
| `SpecBind Research` singleton (`research.md` by default) | Optionally preserves current brownfield gap-analysis findings for one active milestone. | `specbind-gap-analysis`; consumed by Requirements and Design. | Accepted | Free-form, non-authoritative, and excluded from gate fingerprints. Persistent artifacts must remain self-contained. Release, scope removal, and abandonment delete it; idle Specs must not retain it. |
| `SpecBind Implementation Notes` collection (`implementation-notes.md`, `artifact_id: main` by default) | Optionally persists spec-scoped implementation knowledge across milestones and releases. | Task generation, implementation, debugging, review, and implementation-validation workflows. | Accepted | The known Front Matter fields are `type` and stable `artifact_id`. Live bodies are non-empty free-form Markdown; absence represents no durable notes. Gate fingerprints exclude the collection, release preserves it, and project-wide knowledge should be promoted to `steering/`. |
| `tasks.yaml` | Exists only for the active milestone's structured task plan and execution state. | Task and implementation workflows. | Accepted | The only canonical task artifact; starts fresh between milestones and is removed by successful release finalization. No parallel `tasks.md` view is maintained. |
| `log.md` | Persists per spec as the OKF reserved update log for released changes. | Release finalization workflow. | Accepted | Has no frontmatter. For a Spec-backed milestone, AI supplies one delivered-change summary per participating spec and the CLI inserts its canonical entry under newest-first ISO dates. Direct-only milestones do not update per-Spec logs. Release version is the human-facing label, milestone ID is secondary trace metadata, and abandoned work is omitted. |
| `spec.yaml` | Represents lifecycle, active-change metadata, active Requirement IDs, and gate evidence. | Spec lifecycle workflows. | Accepted | The only canonical per-spec metadata artifact; its target states and events are defined in the spec state machine and must represent released / no-active-change without requiring a brief artifact or `tasks.yaml`. |

## Tracked follow-up questions

- Superseded or removed product capabilities and durable retired identity are
  tracked by [Issue #7](https://github.com/Huruikagi/specbind/issues/7).
- An optional audit artifact for abandoned, unreleased milestones remains an
  open design choice within
  [Issue #8](https://github.com/Huruikagi/specbind/issues/8); it is not part of
  the default release archive.
- Expanding canonical Spec identity from one portable segment to an OKF-aligned
  namespace such as `commerce/checkout`, including placement of fixed
  structured artifacts, is tracked by
  [Issue #14](https://github.com/Huruikagi/specbind/issues/14).
