# 0093: Install a narrow default shared-rule set

Status: Accepted

## Context

Decision 0008 preserves cc-sdd's project-owned `settings/rules/` customization surface, and Decision 0049 already requires the SpecBind-specific `okf-artifacts.md` rule. Decision 0092 then separates user-customizable rules from non-waivable skill behavior and deterministic CLI contracts. The exact default rule set is still undefined.

The inherited cc-sdd tree contains twelve rule files. Some are genuine project-wide authoring principles, but others are product workflow procedures, review loops, output protocols, or restatements of machine structure. Installing all twelve would expose product behavior as if it were user policy, duplicate the future SpecBind skills, and invite unsupported weakening through customization.

Installing only `okf-artifacts.md` would move too far in the other direction. Requirements style, design preferences, task decomposition, and steering granularity are useful repository-level choices that should remain consistent across Codex and Claude Code and survive replacement of product-managed skills.

SpecBind therefore needs a narrow default set that follows cc-sdd where the rule responsibility remains valid, adds the new cross-spec contract concern, and moves procedural content into the owning skills.

## Decision

### Installed defaults

`specbind install` provides exactly these six default files under `{{SPEC_DIR}}/settings/rules/`:

| Default file | Origin | Customizable responsibility |
| --- | --- | --- |
| `okf-artifacts.md` | SpecBind | Concise OKF v0.2 authoring reminders, relationship style, extension-field preservation, and the boundary between OKF metadata and SpecBind lifecycle authority accepted by Decision 0049. |
| `ears-format.md` | Retained from cc-sdd | Preferred EARS patterns, subject choice, localization-aware phrasing, and testability style for Requirements. It does not define Requirement heading grammar, IDs, approval, or active-scope selection. |
| `design-principles.md` | Retained from cc-sdd | Project-adjustable architecture, interface, data-model, error-handling, diagram, documentation, and level-of-detail preferences. It does not define discovery modes, mandatory review loops, Design traceability syntax, or gate behavior. |
| `contract-principles.md` | New for SpecBind | Project policy for shared ownership, public seams, compatibility posture, generated boundaries, dependency direction, and when warnings deserve stricter review. It does not define canonical Contract syntax, required graph validity, or cross-spec-review lifecycle. |
| `tasks-generation.md` | Retained from cc-sdd | Project preferences for task sizing, decomposition, completion-detail style, test-work grouping, and conservative parallelization. It does not define `tasks.yaml`, positional IDs, dependency semantics, required coverage, approval, or execution state. |
| `steering-principles.md` | Retained from cc-sdd | Project preferences for durable steering granularity, examples, preservation, and avoiding transient or obvious content. It does not define steering discovery, identity, installation, or synchronization workflow. |

The retained filenames preserve the useful cc-sdd customization vocabulary and make deliberate migration review easier. Their contents are rewritten for the Decision 0092 boundary; SpecBind does not copy the inherited files verbatim.

All official default rules are English-authored under Decision 0076. They are ordinary UTF-8 OKF concept documents with first-line Front Matter containing `type: SpecBind Rule`. They have no SpecBind-owned `schema_version`, `artifact_id`, applicability list, priority, or enablement field in v1. Unknown top-level OKF extension fields carry no routing semantics.

### Content boundary

An installed rule expresses a preference that a project may reasonably strengthen, relax, replace, or remove without changing the SpecBind lifecycle. It may include examples and review questions, but it does not own commands, phase order, retry counts, approval decisions, mutation authority, required reads, or machine-readable artifact grammar.

Exact CLI contracts may be summarized in a rule only to explain the boundary around a preference. Such summaries are non-authoritative and should link to or name the owning contract instead of duplicating a complete schema or protocol.

Product-managed skills retain the semantic minimum even if every rule is absent. A customized rule can strengthen review or authoring policy, but cannot make a skill skip its baseline checks. The CLI remains authoritative for every deterministic invariant.

### Skill loading

V1 uses explicit known-path loading rather than scanning every Markdown file or introducing a rule manifest:

| Rule | Owning consumers |
| --- | --- |
| `okf-artifacts.md` | Every skill that creates or rewrites managed Markdown. Read-only skills load it only when their review checks OKF authoring quality beyond CLI diagnostics. |
| `ears-format.md` | `specbind-requirements`. |
| `design-principles.md` | `specbind-design`, `specbind-validate-design`, and `specbind-gap-analysis`. |
| `contract-principles.md` | `specbind-design`, `specbind-validate-design`, `specbind-gap-analysis` when boundaries are relevant, and `specbind-cross-spec-review`. |
| `tasks-generation.md` | `specbind-tasks`. |
| `steering-principles.md` | `specbind-steering`. |

`specbind-quick` and `specbind-batch` use the same phase contracts and therefore the same applicable rules when they perform Requirements, Design, and Tasks work. They do not define separate quick or batch rule variants. Implementation, task review, completion validation, release, status, and debug use their product-managed contracts plus current project artifacts and steering; v1 installs no extra shared rule merely because one of those skills exists.

A skill reads each applicable file at most once per invocation. It does not silently substitute an embedded rule when a project file is absent, because the installed file is the user-owned policy source. Absence means that no project customization from that rule is applied; core skill and CLI behavior remains intact.

V1 does not recursively load additional `settings/rules/**/*.md` files. Arbitrary automatic loading would make relevance and conflict precedence depend on filenames or directory order. A future extensible rule profile may add stable IDs, applicability selectors, and deterministic ordering through a separate decision. Until then, projects customize the six known files and use ordinary steering artifacts for additional durable project guidance.

### Installation and refresh

- Official defaults are embedded in the Rust binary as installation assets under Decision 0077.
- Initial installation writes every missing default rule and never overwrites an existing path.
- A later `specbind install` refresh creates newly introduced or otherwise missing default files as uncommitted changes for review, following the common settings contract. It does not merge updated official prose into a project-owned file.
- Skills read only project files after installation; embedded copies are not runtime fallback policy.
- Deleting a rule is valid between installs and leaves the core workflow defined. A later refresh may offer the missing default again under Decision 0077, and the project may remove that uncommitted addition before committing.
- Installed rules use one English default set for both configured artifact languages. Projects may localize or rewrite their copies; rule language is not machine-validated.

### cc-sdd disposition

The inherited files are classified as follows:

| cc-sdd file | SpecBind v1 disposition | Reason |
| --- | --- | --- |
| `ears-format.md` | Retain and rewrite as a default rule. | User-customizable Requirements writing style. |
| `design-principles.md` | Retain and rewrite as a default rule. | User-customizable technical and documentation preferences. |
| `tasks-generation.md` | Retain and rewrite as a default rule. | User-customizable task decomposition preferences after removing schema and gate behavior. |
| `steering-principles.md` | Retain and rewrite as a default rule. | User-customizable durable-guidance preferences. |
| `design-discovery-full.md` | Move to `specbind-design`. | Workflow branching and required investigation procedure. |
| `design-discovery-light.md` | Move to `specbind-design`. | Workflow branching and escalation behavior. |
| `design-synthesis.md` | Move its semantic baseline to `specbind-design`. | Required design reasoning, not a standalone customization surface. |
| `design-review-gate.md` | Move to `specbind-design`. | Product-owned pre-approval review behavior. |
| `design-review.md` | Move to `specbind-validate-design`. | Product-owned validation protocol and result semantics. |
| `requirements-review-gate.md` | Move to `specbind-requirements`. | Product-owned semantic minimum before approval. |
| `gap-analysis.md` | Move to `specbind-gap-analysis`. | Investigation workflow and output lifecycle. |
| `tasks-parallel-analysis.md` | Move to `specbind-tasks`. | Product baseline for safe parallel judgment over structured task semantics. |

`contract-principles.md` and `okf-artifacts.md` have no direct cc-sdd counterpart. They exist because SpecBind adds persistent typed Contracts and an OKF bundle contract.

`specbind migrate cc-sdd` does not copy inherited rule files verbatim into the SpecBind default paths. The migration plan identifies legacy rule files that differ from their known cc-sdd defaults and reports them for manual policy review. The original `.kiro` tree remains intact. New SpecBind defaults are written only through the ordinary absent-target settings behavior, so a procedural cc-sdd file never silently becomes user-owned SpecBind policy.

## Consequences

- A new project receives six purposeful rule files rather than the complete inherited process library.
- Four familiar cc-sdd customization topics remain available under familiar filenames.
- Contract and OKF authoring receive explicit SpecBind-native policy surfaces.
- Skill implementations have a concrete destination for the workflow guidance removed from templates and inherited rules.
- Rule loading is predictable and bounded without a premature manifest or applicability schema.
- Adding another official default later is a visible installation-surface decision because refresh creates that file in existing projects.

## Implementation status

This decision defines the target installed set and loading contract. The Rust installer and v1 SpecBind skills do not yet install or consume these six rules. The inherited files under `tools/cc-sdd/templates/shared/settings/rules/` remain migration inputs until the rewritten embedded defaults, skill-owned procedures, installation planning, and absence/customization tests are implemented together.
