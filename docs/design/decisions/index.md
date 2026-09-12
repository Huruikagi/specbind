# Decision index

Accepted decisions are authoritative. A superseded decision is retained for
history; follow the decision that replaced it. Run
`python scripts/check_decisions.py` from the repository root to verify that
Decision filenames, headings, identifiers, and this index remain consistent.

| Decision | Status | Summary |
| --- | --- | --- |
| [0001](./0001-skill-naming.md) | Superseded by 0075 | Replacement of inherited skill naming |
| [0002](./0002-project-release-adapter.md) | Accepted | Core-plus-project-adapter release direction |
| [0003](./0003-active-requirement-set.md) | Accepted | Storage contract for current milestone Requirement IDs |
| [0004](./0004-release-history-layout.md) | Accepted | Per-spec changelog and roadmap archive layout |
| [0005](./0005-active-change-abandonment.md) | Accepted | Scope removal, abandonment, and rollback boundaries |
| [0006](./0006-rust-cli.md) | Accepted | Direction to reimplement the complete SpecBind CLI in Rust |
| [0007](./0007-spec-root.md) | Accepted | Configurable spec root with `.specbind` as the new-project default |
| [0008](./0008-customization-surface.md) | Accepted | Shared templates and rules as the stable project customization surface |
| [0009](./0009-milestone-cli-boundary.md) | Accepted | Discovery-plus-Rust-CLI milestone responsibility boundary |
| [0010](./0010-release-execution-boundary.md) | Accepted | AI adapter execution and Rust CLI release-finalization boundary |
| [0011](./0011-cross-spec-contract.md) | Accepted | Persistent contract manifest and contract-first contract review direction |
| [0012](./0012-delegated-approval.md) | Accepted | Explicit-versus-delegated gate approval and non-interactive execution boundary |
| [0013](./0013-structured-task-artifact.md) | Accepted | Structured `tasks.yaml` source-of-truth direction |
| [0014](./0014-structured-spec-metadata.md) | Accepted | Structured `spec.yaml` source-of-truth direction |
| [0015](./0015-runtime-schema-layout.md) | Accepted | Versioned runtime-schema location and validation layers |
| [0016](./0016-fingerprint-value-format.md) | Accepted | Tagged lowercase SHA-256 fingerprint representation |
| [0017](./0017-requirements-gate-inputs.md) | Accepted | Requirements-gate fingerprint boundary excluding `brief.md` |
| [0018](./0018-gate-input-comparison.md) | Accepted | Markdown normalization, Requirement ID snapshot, and task-plan projection boundaries |
| [0019](./0019-task-ordering-and-dependencies.md) | Accepted | Ordered task execution with sparse dependency exceptions |
| [0020](./0020-positional-task-ids.md) | Accepted | cc-sdd-style positional numeric Task IDs |
| [0021](./0021-optional-completion-criteria.md) | Accepted | Dedicated, conditionally optional task completion criteria |
| [0022](./0022-no-optional-tasks.md) | Accepted | Removal of inherited optional tasks from `tasks.yaml` v1 |
| [0023](./0023-sparse-task-plan-shape.md) | Accepted | Strict sparse group and executable-task plan objects |
| [0024](./0024-sparse-task-execution-state.md) | Accepted | Sparse persisted completed and blocked task states |
| [0025](./0025-task-read-model.md) | Accepted | Derived CLI projections for task status and detail |
| [0026](./0026-runtime-implementation-notes.md) | Accepted | Persistent free-form implementation memory for later agents |
| [0027](./0027-spec-local-task-dependencies.md) | Accepted | Local-only Task IDs with roadmap and contract routing for cross-spec dependencies |
| [0028](./0028-task-plan-fingerprint.md) | Accepted | Normalized typed-plan projection and canonical fingerprint algorithm |
| [0029](./0029-completion-validation-handshake.md) | Accepted | Clean Git revision handshake for guarded completion validation |
| [0030](./0030-persist-only-accepted-completion-evidence.md) | Accepted | Current-state-only storage for successful completion evidence |
| [0031](./0031-project-scoped-revision-format.md) | Accepted | Scalar Git implementation revision interpreted from project context |
| [0032](./0032-gate-local-freshness-chain.md) | Accepted | Gate-local revision ownership and cascading freshness semantics |
| [0033](./0033-completion-mechanical-checks.md) | Accepted | Concise categorized command evidence for successful completion validation |
| [0034](./0034-do-not-persist-semantic-pass-flags.md) | Accepted | Semantic validation protocol without redundant persisted pass flags |
| [0035](./0035-roadmap-owned-cross-spec-review.md) | Superseded by 0078 | Roadmap ownership for contract-impact and downstream-review evidence |
| [0036](./0036-rfc3339-gate-timestamps.md) | Accepted | Timezone-qualified RFC 3339 format for gate timestamps |
| [0037](./0037-minimal-completion-evidence-shape.md) | Accepted | Strict three-field completion evidence object |
| [0038](./0038-design-gate-inputs.md) | Accepted | Design and contract fingerprint inputs for the design gate |
| [0039](./0039-minimal-tasks-gate-evidence.md) | Accepted | Minimal approval evidence for the normalized task-plan projection |
| [0040](./0040-state-gate-evidence-invariants.md) | Accepted | Sparse cumulative evidence and semantic state-to-gate invariants |
| [0041](./0041-no-per-spec-change-id.md) | Accepted | Milestone-plus-spec identity without a separate per-spec change ID |
| [0042](./0042-sequential-milestone-id.md) | Superseded by 0043 | Project-sequential milestone ID |
| [0043](./0043-uuidv7-milestone-id.md) | Accepted | Branch-safe UUID v7 milestone IDs |
| [0044](./0044-minimal-spec-root.md) | Accepted | Minimal strict `spec.yaml` root and active-change object |
| [0045](./0045-okf-markdown-artifacts.md) | Accepted | OKF Front Matter profile for every managed Markdown artifact |
| [0046](./0046-roadmap-work-items.md) | Accepted | Grouped Roadmap work items for new Specs, Spec updates, and Direct changes |
| [0047](./0047-sparse-direct-change-status.md) | Accepted | Sparse persisted completed status for Direct changes |
| [0048](./0048-okf-spec-log.md) | Accepted | Canonical OKF `log.md` for per-spec release history |
| [0049](./0049-okf-authoring-rule.md) | Superseded by 0094 | Concise installed OKF authoring rule |
| [0050](./0050-global-cross-spec-review.md) | Accepted | One global accepted contract review per milestone |
| [0051](./0051-current-state-roadmap.md) | Accepted | Current-state-only active Roadmap |
| [0052](./0052-project-state-artifacts.md) | Accepted | Project-wide machine state separated from steering |
| [0053](./0053-minimal-cross-spec-review-state.md) | Superseded by 0078 | Structured classifications paired with an AI-authored review |
| [0054](./0054-milestone-baseline-revision.md) | Accepted | Milestone baseline revision as the contract-diff anchor |
| [0055](./0055-cross-spec-review-inputs.md) | Accepted | Contract-first review inputs |
| [0056](./0056-canonical-contract-markdown.md) | Superseded by 0155 | Canonical five-section Markdown contract manifests |
| [0057](./0057-type-based-artifact-discovery.md) | Accepted | Type-based OKF artifact discovery |
| [0058](./0058-artifact-inventory-read-model.md) | Accepted | Artifact inventory separated from raw content reads |
| [0059](./0059-okf-artifact-templates.md) | Accepted | Final-form OKF documents as artifact templates |
| [0060](./0060-requirement-id-and-heading-mapping.md) | Accepted | Requirement IDs derived from mapped headings and list position |
| [0061](./0061-design-requirement-traceability.md) | Accepted | Explicit Design traceability in Front Matter and body markers |
| [0062](./0062-minimal-active-brief-profile.md) | Accepted | Minimal free-form active brief profile |
| [0063](./0063-free-form-release-adapter-profile.md) | Accepted | Free-form agent-interpreted release adapter profile |
| [0064](./0064-path-scoped-release-finalization-guard.md) | Accepted | Path-scoped release finalization Git guard |
| [0065](./0065-forceable-release-target-check.md) | Superseded by 0081 | Guarded release finalization with a narrow force override |
| [0066](./0066-agent-judged-release-and-cli-log-insertion.md) | Accepted | Agent-judged release success with CLI-inserted spec logs |
| [0067](./0067-text-first-english-cli-results.md) | Accepted | Concise, text-first, English-only CLI results |
| [0068](./0068-release-log-summary-input.md) | Accepted | Strict JSON per-spec summaries as release-finalization input |
| [0069](./0069-stateless-release-preflight.md) | Accepted | Stateless read-only release preflight |
| [0070](./0070-derived-release-readiness.md) | Accepted | Derived release readiness without a new evidence artifact |
| [0071](./0071-no-partial-milestone-release.md) | Accepted | No partially released milestone representation |
| [0072](./0072-explicit-release-rebinding.md) | Accepted | Explicit operation required for release rebinding |
| [0073](./0073-portable-release-version.md) | Accepted | Opaque portable release-version label |
| [0074](./0074-defer-json-cli-output.md) | Accepted | General JSON CLI output deferred beyond two command-specific status exceptions |
| [0075](./0075-v1-skill-and-orchestration-scope.md) | Accepted | Fixed v1 skill and orchestration scope |
| [0076](./0076-project-global-artifact-language.md) | Accepted | One project-global artifact language |
| [0077](./0077-v1-installation-distribution-and-migration.md) | Accepted | v1 installation, distribution, and cc-sdd migration contract |
| [0078](./0078-contract-first-review-between-design-and-tasks.md) | Accepted | One free-form contract-first review between Design and Tasks |
| [0079](./0079-milestone-local-research.md) | Accepted | Optional research as a milestone-local singleton |
| [0080](./0080-v1-task-contract-and-completion-details.md) | Accepted | Fixed v1 Task, Contract, and completion details |
| [0081](./0081-v1-release-git-path-and-cli-safety.md) | Accepted | Tightened v1 release, Git, path, and CLI safety |
| [0082](./0082-derived-milestone-state-machine.md) | Accepted | Derived milestone state and phase-relative dependency waves |
| [0083](./0083-json-schema-structural-authority.md) | Superseded by 0085 | JSON Schema authoritative over Rust artifact models |
| [0084](./0084-rust-dependency-strategy.md) | Accepted | Focused Rust dependencies behind SpecBind-owned boundaries |
| [0085](./0085-rust-wire-model-schema-generation.md) | Accepted | JSON Schema generated from versioned Rust wire models |
| [0086](./0086-completion-cli-handshake.md) | Accepted | Spec and Direct completion CLI handshake |
| [0087](./0087-milestone-review-cli.md) | Accepted | Milestone-owned contract review commands |
| [0088](./0088-gate-approval-cli.md) | Accepted | Spec gate approval and invalidation commands |
| [0089](./0089-milestone-creation-cli.md) | Accepted | Milestone creation, scope, and rebaseline commands |
| [0090](./0090-standalone-check-cli.md) | Accepted | Standalone traceability and contract check commands |
| [0091](./0091-installed-template-surface.md) | Accepted | Embedded scaffold set separated from the installed customization surface |
| [0092](./0092-template-skill-authoring-boundary.md) | Accepted | Artifact scaffold guidance separated from authoring workflow policy |
| [0093](./0093-default-shared-rule-set.md) | Accepted | Narrow installed shared-rule set and explicit skill consumers |
| [0094](./0094-embedded-product-protocols.md) | Accepted | Immutable shared semantic protocols exposed through the CLI |
| [0095](./0095-task-progress-cli.md) | Accepted | Guarded task execution progress commands |
| [0096](./0096-skill-asset-layout.md) | Accepted | One agent-neutral source per product-managed skill |
| [0097](./0097-discovery-routing-and-read-models.md) | Accepted | Discovery routing contract and the read models it requires |
| [0098](./0098-steering-read-surface.md) | Accepted | Steering documents identified by OKF type and read through the CLI |
| [0099](./0099-project-instruction-block.md) | Accepted | Marked SpecBind block maintained in root agent instruction files |
| [0100](./0100-requirements-skill-contract.md) | Accepted | Active selection, approval, review loop, and invalidation for the requirements skill |
| [0101](./0101-project-adapter-directory-and-git-workflow.md) | Accepted | Project adapter directory and free-form Git workflow guidance |
| [0102](./0102-workflow-entry-condition.md) | Accepted | When a request enters the SpecBind workflow at all |
| [0103](./0103-schema-read-surface.md) | Accepted | Embedded artifact and command-input schemas readable through the CLI |
| [0104](./0104-design-skill-contract.md) | Accepted | Reads, Contract update timing, approval, and rewind for the design skill |
| [0105](./0105-tasks-skill-contract.md) | Accepted | Review ordering, schema-driven authoring, renumbering safety, and approval for the tasks skill |
| [0106](./0106-contract-review-naming.md) | Accepted | Rename of the cross-spec review to the contract review |
| [0107](./0107-spec-status-contract-review-barrier.md) | Accepted | Contract-review barrier reported in Spec status from the tasks state onward |
| [0108](./0108-contract-review-skill-contract.md) | Accepted | Reads, baseline comparison, deep-input discipline, remediation, and acceptance for the contract review skill |
| [0109](./0109-subagent-dispatch-contract.md) | Accepted | Fresh-context subagent dispatch, its neutral expression, and the structured return |
| [0110](./0110-implement-skill-contract.md) | Accepted | Item selection, per-task dispatch cycle, bounded failure routing, and where the implement run stops |
| [0111](./0111-review-task-and-debug-skill-contracts.md) | Accepted | Two moments, the read-only boundary, and unfresh-context honesty for the review and debug skills |
| [0112](./0112-validate-implementation-skill-contract.md) | Accepted | Completion-verification protocol, the three verdicts, run-not-assembled evidence, and the multi-Spec metadata commit |
| [0113](./0113-verify-completion-skill-contract.md) | Accepted | Claim-shaped subject, distinct verdicts, and the consequence-free boundary for the claim verification skill |
| [0114](./0114-validate-design-skill-contract.md) | Accepted | Two verdicts with no inconclusive escape, the deletion test, and no self-initiated rewind for design validation |
| [0115](./0115-release-skill-contract.md) | Accepted | Binding order, confirmed publication, verification as a completion claim, and delivered-change summaries |
| [0116](./0116-spec-status-delegated-gates.md) | Accepted | Delegated gates and their workflow reported in Spec status |
| [0117](./0117-steering-authoring-contract.md) | Accepted | Steering authoring, in-place synchronization, and the steering template scope |
| [0118](./0118-gap-analysis-skill-contract.md) | Accepted | Gap analysis before Requirements, the request-mediated influence path, and marked conclusions |
| [0119](./0119-writing-while-a-completion-stands.md) | Accepted | One statement of what writing costs once a Spec holds accepted completion |
| [0120](./0120-quick-and-batch-orchestration-contracts.md) | Superseded by 0153 | Quick-plan and batch-plan orchestration, phase-specific dependency shape, and retry classification |
| [0121](./0121-requirements-coverage-is-not-slots.md) | Accepted | Requirements coverage bounded to what the Spec owes |
| [0122](./0122-finding-disposition-and-deferred-destination.md) | Accepted | Finding disposition and the project-named destination for deferred findings |
| [0123](./0123-reverse-traceability-and-unconsumed-seams.md) | Accepted | Reverse task-scope traceability and unconsumed exported seams |
| [0124](./0124-pre-1.0-binary-release-line.md) | Accepted | Pre-1.0 public binary release and distribution contract |
| [0125](./0125-agent-assisted-cc-sdd-migration.md) | Accepted | Agent-assisted cc-sdd migration and GitHub Pages handoff contract |
| [0126](./0126-cli-owned-cc-sdd-migration-resolution.md) | Accepted | CLI-owned accepted cc-sdd migration resolution and freshness contract |
| [0127](./0127-retire-cc-sdd-source-at-final-cutover.md) | Accepted | Git-guarded retirement of cc-sdd sources at final cutover |
| [0128](./0128-plan-orchestrator-names.md) | Superseded by 0153 | Planning orchestrator names expose their Tasks-approval stopping point |
| [0129](./0129-agent-role-capability-adapters.md) | Accepted | Agent-role capability adapters with project model overrides |
| [0130](./0130-mise-github-backend-installation.md) | Accepted | Installation through mise's GitHub backend over the existing release assets |
| [0131](./0131-okf-deferred-destination-and-adapter-state.md) | Accepted | Exact OKF-conformant deferred destination and visible adapter state |
| [0132](./0132-target-aware-template-resolution.md) | Accepted | Target-aware Spec template path and provenance resolution |
| [0133](./0133-phase-relative-spec-status.md) | Accepted | Phase-relative Spec health, expected work, and workflow action |
| [0134](./0134-phase-relative-milestone-review-health.md) | Accepted | Absent milestone review treated as expected workflow work |
| [0135](./0135-phase-relative-requirements-status.md) | Accepted | Absent Requirements treated as expected phase work |
| [0136](./0136-phase-relative-worktree-blocker.md) | Accepted | Worktree cleanliness reported only when it blocks current progress |
| [0137](./0137-active-default-git-checkpoints.md) | Accepted | Active local Git checkpoints for newly installed projects |
| [0138](./0138-dedicated-adapter-scaffold-marker.md) | Accepted | Dedicated exact marker for inactive adapter scaffolds |
| [0139](./0139-scoped-artifact-instructions.md) | Accepted | Lifecycle-scoped create, maintain, and consume instructions |
| [0140](./0140-release-adapter-bootstrap-and-finalization-checkpoint.md) | Accepted | One-time Release adapter bootstrap and post-finalization metadata checkpoint |
| [0141](./0141-guarded-agent-removal-and-project-uninstall.md) | Accepted | Exact planned agent removal and explicit durable-knowledge uninstall policy |
| [0142](./0142-bilingual-documentation-authoring-and-publishing.md) | Accepted | Japanese-first documentation authoring and English-default bilingual publishing hierarchy |
| [0143](./0143-existing-implementation-adoption.md) | Superseded by 0175 | Steering-first adoption of existing implementations through evidence-backed reverse discovery |
| [0144](./0144-major-version-compatibility-and-migration.md) | Accepted | Executable-major compatibility boundary and required migration route between majors |
| [0145](./0145-customizable-roadmap-body-template.md) | Accepted | Installed project-owned template for milestone-wide Roadmap prose |
| [0146](./0146-sequential-v1-tasks-and-per-task-checkpoints.md) | Accepted | Sequential v1 Tasks and one default checkpoint per completed Task |
| [0147](./0147-generic-agent-shared-surfaces.md) | Accepted | Generic `.agents/skills` and `AGENTS.md` integration with remaining-Agent shared ownership |
| [0148](./0148-cli-feedback-and-issue-forms.md) | Accepted | Offline product feedback routing to bilingual public Issue Forms |
| [0149](./0149-bound-spec-template-rendering-variable.md) | Superseded by 0151 | Canonical Spec identity rendering bound to explicit create guidance |
| [0150](./0150-attributable-fail-closed-default-scaffolds.md) | Accepted | Collection identity rendering and fail-closed default scaffolds |
| [0151](./0151-agent-bound-template-variables.md) | Superseded by 0167 | Project-defined template variables resolved through bound agent instructions |
| [0152](./0152-rule-selected-design-template-set.md) | Accepted | Project-rule selection of required, conditional, and disabled Design templates |
| [0153](./0153-unified-quick-plan-orchestrator.md) | Superseded by 0161 | One quick-plan orchestrator with explicit named and all-Spec scope modes |
| [0154](./0154-guided-configuration-workflow.md) | Accepted | Completing guided configuration workflow, summary command, and aftercare |
| [0155](./0155-versioned-yaml-contract-artifact.md) | Accepted | Versioned strict YAML Contract artifact and semantic fingerprint |
| [0156](./0156-derived-contract-graph-reads.md) | Accepted | Read-only direct Contract graph and reverse-consumer projections |
| [0157](./0157-command-specific-spec-status-json.md) | Accepted | Minimal command-specific JSON projection for Spec status |
| [0158](./0158-command-specific-milestone-status-json.md) | Accepted | Minimal command-specific JSON projection for Milestone status |
| [0159](./0159-forward-test-usability-boundaries.md) | Accepted | Forward-test usability, authority, status, and recovery boundaries |
| [0160](./0160-tracked-routing-and-selector-precedence.md) | Accepted | Tracked delivery routing and declared Design selector precedence |
| [0161](./0161-default-plan-and-phase-skill-namespace.md) | Accepted | Default Plan entry point and explicit plan-phase Skill namespace |
| [0162](./0162-forward-test-record-lifecycle.md) | Accepted | Separate forward-test run history, current dashboard, and findings lifecycle |
| [0163](./0163-macos-arm64-release-target.md) | Accepted | Native macOS ARM64 release, preflight, installer, and runtime verification |
| [0164](./0164-local-discovery-source-collections.md) | Accepted | Provider-neutral Discovery source semantics with a Git-backed local-files provider |
| [0165](./0165-release-binding-preserves-completion.md) | Accepted | Completion-preserving active-Roadmap release binding and rebinding |
| [0166](./0166-single-english-cc-sdd-migration-guide-url.md) | Accepted | One English cc-sdd migration guide URL for every CLI handoff |
| [0167](./0167-named-template-creation-outputs.md) | Accepted | Agent-produced named outputs with mechanically validated template references |
| [0168](./0168-milestone-drive-orchestrator.md) | Accepted | Milestone-wide drive orchestration with branch-local attention and reachable-work continuation |
| [0169](./0169-language-aware-writing-style-rule.md) | Accepted | Japanese-default shared prose policy consumed by every product Skill |
| [0170](./0170-deferred-findings-in-design-validation-handoff.md) | Accepted | Adapter-bound deferred findings carried through the unapproved Design validation handoff |
| [0171](./0171-project-local-plan-dispatch-environment.md) | Accepted | Project-local execution environment carried into fresh Plan dispatches |
| [0172](./0172-requirements-preservation-preflight.md) | Accepted | Existing Requirements preservation proved before approval |
| [0173](./0173-mechanical-requirement-retirement-guard.md) | Accepted | Established Requirement ID removal rejected mechanically |
| [0174](./0174-plan-phase-procedures-as-references.md) | Accepted | Requirements, Design, and Tasks procedures packaged as references of the single Plan Skill |
| [0175](./0175-existing-adoption-as-discovery-references.md) | Superseded by 0188 | Existing-implementation adoption procedures packaged as references of Discovery |
| [0176](./0176-skill-namespace-separation.md) | Accepted | `sb-*` Skill namespace separated from the `specbind` CLI |
| [0177](./0177-steering-scaffold-conformance-check.md) | Accepted | Mechanical verification of a materialized Steering scaffold |
| [0178](./0178-github-milestone-discovery-source-provider.md) | Accepted | GitHub Milestone Discovery source provider |
| [0179](./0179-spec-local-one-off-design-supplements.md) | Accepted | Evidence-backed Spec-local one-off Design supplements |
| [0180](./0180-delegate-binary-updates-to-installation-clients.md) | Accepted | Binary updates delegated to installation clients and separated from project-asset refresh |
| [0181](./0181-reverse-spec-establishment.md) | Accepted | Reverse Specs established from a fixed implementation revision as a non-release baseline |
| [0182](./0182-project-validation-adapter.md) | Accepted | Project-specific final implementation validation adapter |
| [0183](./0183-codex-skill-interface-metadata.md) | Accepted | Branded Codex interface metadata for product-managed Skills |
| [0184](./0184-agent-executable-update-workflow.md) | Accepted | Explicit installation-client updates routed through `sb-configure` with post-refresh package reload |
| [0185](./0185-reverse-deferred-finding-checkpoint.md) | Accepted | Reverse deferred findings recorded after the clean milestone baseline exists |
| [0186](./0186-reverse-design-contract-preflight.md) | Accepted | Phase-relative Contract preflight for dependency-ordered reverse Design |
| [0187](./0187-forward-test-routing-and-read-projections.md) | Accepted | Forward-test routing, active Requirement scope, and Contract graph status projections |
| [0188](./0188-retire-legacy-staged-adoption.md) | Accepted | Legacy staged adoption retired in favor of one reverse-establishment route |
| [0189](./0189-active-adapter-consumption-projection.md) | Accepted | Active-only adapter consumption with raw configuration reads preserved |
| [0190](./0190-file-ownership-path-projection.md) | Accepted | Concrete project paths resolved against Contract File Ownership declarations |
| [0191](./0191-resumable-reverse-establishment.md) | Accepted | Reverse establishment resumed from verified durable state and explicit authority |
| [0192](./0192-typed-milestone-action-handlers.md) | Accepted | Typed Milestone actions projected with Drive-consumable handlers |
| [0193](./0193-progressive-discovery-procedures.md) | Accepted | Progressive loading of ordinary, reverse, and provider-specific Discovery procedures |
| [0194](./0194-per-spec-design-revision-budget.md) | Accepted | Per-Spec Design remediation budget with validator-owned finding continuity |
| [0195](./0195-review-scope-recovery.md) | Accepted | Bounded fresh re-review after a diagnosed Task review scope defect |
| [0196](./0196-optional-temporary-adoption-skill.md) | Accepted | Optional temporary adoption Skill and guarded retirement after reverse finalization |
| [0197](./0197-task-verification-prerequisites.md) | Accepted | Task verification inputs and state available at completion, with targeted cross-Spec checks |
| [0198](./0198-inline-requirement-retirement.md) | Accepted | Inline Requirement retirement with reserved identities and ordinary delivery coverage |
| [0199](./0199-drive-replan-authority.md) | Accepted | Optional Drive delegation of in-scope Design, Contract, and Tasks recovery |
| [0200](./0200-milestone-task-blocker-projection.md) | Accepted | Milestone Task progress and blocker projection without dirty-state regression |
| [0201](./0201-design-scoped-traceability-projection.md) | Accepted | Design-only traceability projection without reading retained downstream Tasks |
| [0202](./0202-renew-contract-review-over-retained-delivery-tasks.md) | Accepted | Renewed delivery Contract Review without reading or deleting retained Tasks |
| [0203](./0203-carry-cli-owned-design-rewind-through-validation.md) | Accepted | Proven CLI-owned Design rewind carried through validation and approval checkpoint |
| [0204](./0204-close-or-transfer-lifecycle-mutations-before-clean-successors.md) | Accepted | Lifecycle mutations closed or transferred before clean-gated successors |
| [0205](./0205-verification-dimensions-do-not-require-invented-commands.md) | Accepted | Verification dimensions satisfied without inventing one command per dimension |
| [0206](./0206-retained-task-progress-must-reconcile-to-active-obligations.md) | Accepted | Retained Task progress explicitly reconciled to active obligations |
| [0207](./0207-non-current-task-plan-status-projection.md) | Accepted | Non-current Task plan authority and Tasks-phase recovery projected separately from health |
| [0208](./0208-linux-arm64-release-target.md) | Accepted | Native Linux ARM64 release archives, installation, and runtime verification |
| [0209](./0209-project-shared-contract.md) | Accepted | Project shared resources without dedicated Specs and reviewed Direct changes |
| [0210](./0210-durable-artifact-descriptions.md) | Accepted | Durable Requirements, Design, and Steering descriptions in inventories |
| [0211](./0211-version-range-project-migration-plans.md) | Accepted | Read-only version-range migration plans and project-specific reconciliation |
| [0212](./0212-shared-okf-bundle-index.md) | Accepted | Shared bundle-root OKF declaration and product-managed navigation block |
| [0213](./0213-plan-dispatch-capacity-recovery.md) | Accepted | Plan dispatch-capacity recovery without collapsing author and validator roles |
| [0214](./0214-drive-dispatch-capacity-handoffs.md) | Accepted | Lossless Drive handoffs for nested and pre-owner dispatch-capacity stops |
| [0215](./0215-focused-skill-selection-and-verification.md) | Accepted | Focused Skill selection, consistent claim routing, and proportionate Task verification |
