# 0093: Install a narrow default shared-rule set

Status: Accepted

Decision 0152 expands the closed set with the required
`design-template-selection.md` routing Rule. Unlike the original five optional
preference Rules, its absence fails because the Design candidate set would be
unclassified.

[Decision 0146](./0146-sequential-v1-tasks-and-per-task-checkpoints.md)
removes parallelization preferences from the retained task-generation rule.

The requirement to preserve the original `.kiro` tree after final cutover is
superseded by Decision 0127. The classification and conversion boundaries in
this decision remain accepted.

## Context

Decision 0008 preserves cc-sdd's project-owned `settings/rules/` customization surface. Decision 0092 separates user-customizable rules from non-waivable product behavior, and Decision 0094 exposes shared immutable semantic baselines as CLI-readable product protocols. The exact project-owned default rule set still needs to exclude that protocol content.

The inherited cc-sdd tree contains twelve rule files. Some are genuine project-wide authoring principles, but others are product workflow procedures, review loops, output protocols, or restatements of machine structure. Installing all twelve would expose product behavior as if it were user policy, duplicate the future SpecBind skills, and invite unsupported weakening through customization.

Installing no project rules would move too far in the other direction. Requirements style, design preferences, task decomposition, and steering granularity are useful repository-level choices that should remain consistent across Codex and Claude Code and survive replacement of product-managed skills.

SpecBind therefore needs a narrow default set that follows cc-sdd where the rule responsibility remains valid, adds the new cross-spec contract concern, and moves non-customizable content into product protocols and owning skills.

## Decision

### Installed defaults

`specbind install` originally provided the following five preference files
under `{{SPEC_DIR}}/settings/rules/`; Decision 0152 adds the sixth
`design-template-selection.md` routing Rule:

| Default file | Origin | Customizable responsibility |
| --- | --- | --- |
| `ears-format.md` | Retained from cc-sdd | Preferred EARS patterns, subject choice, localization-aware phrasing, and testability style for Requirements. It does not define Requirement heading grammar, IDs, approval, or active-scope selection. |
| `design-principles.md` | Retained from cc-sdd | Project-adjustable architecture, interface, data-model, error-handling, diagram, documentation, and level-of-detail preferences. It does not define discovery modes, mandatory review loops, Design traceability syntax, or gate behavior. |
| `design-template-selection.md` | New for SpecBind | Required, conditional, or disabled classification and project applicability conditions for every Design template. |
| `contract-principles.md` | New for SpecBind | Project policy for shared ownership, public seams, compatibility posture, generated boundaries, dependency direction, and when warnings deserve stricter review. It does not define canonical Contract syntax, required graph validity, or cross-spec-review lifecycle. |
| `tasks-generation.md` | Retained from cc-sdd | Project preferences for task sizing, decomposition, completion-detail style, test-work grouping, and conservative parallelization. It does not define `tasks.yaml`, positional IDs, dependency semantics, required coverage, approval, or execution state. |
| `steering-principles.md` | Retained from cc-sdd | Project preferences for durable steering granularity, examples, preservation, and avoiding transient or obvious content. It does not define steering discovery, identity, installation, or synchronization workflow. |

The retained filenames preserve the useful cc-sdd customization vocabulary and make deliberate migration review easier. Their contents are rewritten for the Decision 0092 boundary; SpecBind does not copy the inherited files verbatim.

All official default rules are English-authored under Decision 0076. They are ordinary UTF-8 OKF concept documents with first-line Front Matter containing `type: SpecBind Rule`. They have no SpecBind-owned `schema_version`, `artifact_id`, applicability list, priority, or enablement field in v1. Unknown top-level OKF extension fields carry no routing semantics.

### Content boundary

An installed rule expresses a preference that a project may reasonably strengthen, relax, replace, or remove without changing the SpecBind lifecycle. It may include examples and review questions, but it does not own commands, phase order, retry counts, approval decisions, mutation authority, required reads, or machine-readable artifact grammar.

Project-specific operational guidance therefore does not become another shared
rule. Decision 0101 places release and Git workflow instructions under
`settings/adapters/`, where their owning skills interpret them without treating
them as product authority.

Exact CLI contracts may be summarized in a rule only to explain the boundary around a preference. Such summaries are non-authoritative and should link to or name the owning contract instead of duplicating a complete schema or protocol.

Product protocols and product-managed skills retain the semantic minimum even if every rule is absent. A customized rule can strengthen review or authoring policy, but cannot make a skill skip the product baseline. The CLI remains authoritative for every deterministic invariant.

### Skill loading

V1 uses explicit known-path loading rather than scanning every Markdown file or introducing a rule manifest:

| Rule | Owning consumers |
| --- | --- |
| `ears-format.md` | `specbind-requirements`. |
| `design-principles.md` | `specbind-design`, `specbind-validate-design`, and `specbind-gap-analysis`. |
| `design-template-selection.md` | `specbind-design` and `specbind-validate-design`. |
| `contract-principles.md` | `specbind-design`, `specbind-validate-design`, `specbind-gap-analysis` when boundaries are relevant, and `specbind-contract-review`. |
| `tasks-generation.md` | `specbind-tasks`. |
| `steering-principles.md` | `specbind-steering`. |

`specbind-quick-plan` and `specbind-batch-plan` use the same phase contracts and therefore the same applicable rules when they perform Requirements, Design, and Tasks work. They do not define separate quick-plan or batch-plan rule variants. Implementation, task review, completion validation, release, status, and debug use their product-managed contracts plus current project artifacts and steering; v1 installs no extra shared rule merely because one of those skills exists.

A skill reads each applicable file at most once per invocation. It does not silently substitute an embedded rule when a project file is absent, because the installed file is the user-owned policy source. Absence means that no project customization from that rule is applied; core protocol, skill, and CLI behavior remains intact.

Skills resolve those project files through the read-only CLI surface rather
than by joining `settings/rules/` paths themselves:

```text
specbind rule list
specbind rule read <selector>
specbind rule read <selector> --for maintain
specbind rule read <selector> --for consume
```

Selectors are the six accepted filenames without `.md`. `list` enumerates
that closed set rather than scanning the directory and reports each rule's
type, path, and project presence. `read` returns the project's exact raw UTF-8
Markdown when no purpose is supplied. Absence is the successful `NO_CHANGE
RULE_ABSENT` result, leaving the owning skill to apply the semantics above; an
unknown selector is never made meaningful by a similarly named file.

Rules are live managed Markdown under Decision 0139. They may carry durable
`maintain` and `consume` instructions, but not template-only `create`
instructions. Purpose projection preserves ordinary Markdown and the requested
instruction scope exactly while omitting the other durable scope. Invalid
instruction syntax, a `create` leak, a link-like or non-regular target, and
non-UTF-8 content fail the read rather than returning partial policy.

V1 does not recursively load additional `settings/rules/**/*.md` files.
Arbitrary automatic loading would make relevance and conflict precedence depend
on filenames or directory order. Projects customize the six known files and
use the Design-template selection Rule to classify arbitrary project-defined
Design selectors; ordinary steering artifacts remain the destination for other
additional durable project guidance.

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
| `design-principles.md` | Retain and rewrite as a default rule. | User-customizable technical and documentation preferences after moving the product baseline to `design-authoring`. |
| `tasks-generation.md` | Retain and rewrite as a default rule. | User-customizable task decomposition preferences after moving the product baseline to `task-planning` and removing schema and gate behavior. |
| `steering-principles.md` | Retain and rewrite as a default rule. | User-customizable durable-guidance preferences. |
| `design-discovery-full.md` | Merge its semantic criteria into `design-discovery`; keep branching in `specbind-design`. | Shared investigation baseline separated from workflow control. |
| `design-discovery-light.md` | Merge its semantic criteria into `design-discovery`; keep escalation in `specbind-design`. | Shared investigation baseline separated from workflow control. |
| `design-synthesis.md` | Move to `design-authoring`. | Required shared design reasoning, not a customization surface. |
| `design-review-gate.md` | Move semantic criteria to `design-validation`; keep approval orchestration in `specbind-design`. | Shared review baseline separated from gate control. |
| `design-review.md` | Move semantic criteria to `design-validation`; keep result orchestration in `specbind-validate-design`. | One validation baseline shared across agents and skills. |
| `requirements-review-gate.md` | Move semantic criteria to `requirements-review`; keep approval and repair flow in `specbind-requirements`. | Shared minimum separated from workflow control. |
| `gap-analysis.md` | Move semantic criteria to `gap-analysis`; keep artifact lifecycle in `specbind-gap-analysis`. | Shared investigation baseline separated from workflow control. |
| `tasks-parallel-analysis.md` | Move to `task-planning`. | Product baseline for safe parallel judgment over structured task semantics. |

`contract-principles.md` has no direct cc-sdd counterpart because SpecBind adds persistent typed Contracts. The former `okf-artifacts.md` installed rule is replaced by Decision 0094's immutable `okf-authoring` protocol.

`specbind migrate cc-sdd` does not copy inherited rule files verbatim into the SpecBind default paths. The migration plan identifies legacy rule files that differ from their known cc-sdd defaults and reports them for manual policy review. The original `.kiro` tree remains intact during planning and semantic resolution, then Decision 0127 retires it only at the explicit final cutover. New SpecBind defaults are written only through the ordinary absent-target settings behavior, so a procedural cc-sdd file never silently becomes user-owned SpecBind policy.

## Consequences

- A new project receives six purposeful rule files rather than the complete inherited process library.
- Four familiar cc-sdd customization topics remain available under familiar filenames.
- Contract authoring receives a SpecBind-native project-policy surface, while OKF authoring uses an immutable product protocol.
- Product protocols and skill implementations have concrete, separate destinations for semantic guidance and workflow control removed from templates and inherited rules.
- Rule loading is predictable and bounded without a premature manifest or applicability schema.
- Adding another official default later is a visible installation-surface decision because refresh creates that file in existing projects.

## Implementation status

All six default rules are authored as embedded installation assets under
`tools/specbind/assets/rules/`, and `specbind install --dry-run` plans them as
create-or-keep entries alongside the Decision 0091 and 0152 templates. Each is
a `SpecBind Rule` OKF concept with no `schema_version`, `artifact_id`,
applicability, priority, or enablement Front Matter field, and the one English
set serves both configured artifact languages. `rule list/read` expose the
fixed selector set and project copies, including the Decision 0139 purpose
projections; current consuming skills request `--for consume` and never resolve
the settings path themselves. Decision 0152 gives the selection Rule a narrow
machine-readable Markdown section contract and requires its presence.

The contents are rewritten for the Decision 0092 boundary rather than copied from cc-sdd: each file states that the project owns it, names the CLI contract or protocol that stays authoritative, and carries preferences plus review questions instead of workflow control. The inherited files under `tools/cc-sdd/templates/shared/settings/rules/` remain migration inputs.

The task-generation default makes its test-grouping preference actionable:
tests stay with the behavior task unless verification spans several earlier
tasks or a separately reviewable system boundary. Projects may replace that
preference, but a fresh installation no longer asks the planner to invent which
of two unnamed conventions the project chose.

Applying an installation, the v1 skills that load these rules, and the tests proving absence and customization cannot weaken the product baseline are separate increments.
