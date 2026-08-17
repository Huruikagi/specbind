# 0094: Expose immutable product protocols through the CLI

Status: Accepted

## Context

Decision 0092 separates user-owned templates and shared rules from product-managed skills and deterministic CLI behavior. Decision 0093 then classifies the inherited cc-sdd rule files, placing user-adjustable authoring preferences in `settings/rules/` and moving procedural material toward the owning skills.

That four-layer split still leaves a duplication problem. Codex and Claude Code skills often need the same substantial semantic checklist, and several SpecBind skills may share one authoring or review protocol. Copying that content into every product-managed skill keeps it non-customizable but creates multiple sources that can drift. Installing it as a project rule deduplicates it but incorrectly makes non-waivable product behavior user-owned.

The CLI already provides the versioned, cross-agent product boundary and embeds schemas, templates, and distribution assets. It can also expose immutable Markdown protocols for agents and humans to read without treating those documents as executable validation or project configuration.

## Decision

### Five-layer authoring model

SpecBind uses five distinct carriers:

1. **CLI validation and guarded operations** own deterministic invariants and mutation safety.
2. **Embedded product protocols** own substantial shared semantic baselines that projects must not weaken.
3. **Product-managed skills** own orchestration, authority checks, user interaction, retries, and skill-local instructions.
4. **Project rules** own repository-specific authoring and judgment preferences.
5. **Templates** own artifact structure, presentation, and scaffold-local guidance.

An embedded protocol is not an enforcement boundary. A skill can fail to read or follow prose, so every reliably machine-decidable invariant still belongs in CLI validation. Protocols are for semantic obligations that require agent judgment but must remain consistent across supported agents and project customization.

### CLI surface

The Rust CLI exposes:

```text
specbind protocol list
specbind protocol read <selector>
```

- Protocols are compiled into the binary and are available without a project root, `.specbind.json`, or installation.
- `protocol list` reports every selector and concise purpose using the standard text-first result contract.
- `protocol read` accepts exactly one selector and writes its raw Markdown body to stdout without a wrapper, matching the single-content read convention.
- Unknown selectors and unreadable embedded assets fail with stable protocol-specific result codes and keep content off stdout.
- Protocol commands never write files, materialize artifacts, inspect project overrides, or accept an alternate protocol root.
- V1 has no `protocol install`, `protocol edit`, project override, organization layer, remote source, or runtime plugin mechanism.

The user-facing term is **protocol**, not **rule**. A rule is project-owned policy; a protocol is versioned product behavior.

### Initial protocol set

V1 embeds these selectors:

| Selector | Product-owned semantic responsibility | Primary consumers |
| --- | --- | --- |
| `okf-authoring` | Concise OKF v0.2 authoring baseline, reserved-document behavior, relationship form, extension preservation, and the boundary between OKF metadata and SpecBind authority. | Every skill that creates or rewrites managed Markdown. |
| `requirements-review` | Complete-current-contract quality, observable scope, testability, ambiguity handling, and semantic readiness before Requirements approval. | `specbind-requirements`. |
| `design-discovery` | Selection and escalation of repository investigation needed before a self-contained Design can be authored. | `specbind-design`. |
| `design-authoring` | Non-waivable synthesis, simplification, owned-boundary, self-containment, and Requirement/Contract realization baseline. | `specbind-design`. |
| `design-validation` | Semantic Design review baseline shared by pre-approval authoring review and independent validation. | `specbind-design`, `specbind-validate-design`. |
| `gap-analysis` | Evidence gathering, option analysis, uncertainty handling, and the boundary between milestone-local Research and authoritative artifacts. | `specbind-gap-analysis`. |
| `task-planning` | Coverage, executability, dependency, completion-detail, boundary, and safe-parallel judgment over the structured Task contract. | `specbind-tasks`. |
| `cross-spec-review` | Contract-first compatibility, external-consumer impact, scope-expansion, and unresolved-finding baseline. | `specbind-cross-spec-review`. |

`specbind-quick` and `specbind-batch` consume the same protocols through the phase contracts they orchestrate. V1 defines no quick-specific or batch-specific protocol variants.

This initial set is deliberately bounded to substantial semantic material that was previously duplicated, mixed into cc-sdd rules, or newly shared by SpecBind's Contract workflow. Short instructions used by only one skill remain in that skill. Adding a protocol later is low-impact because it creates no project file, but still requires an explicit selector, consumer mapping, tests, and release-note treatment.

### Protocol document contract

- Protocol sources are English-authored UTF-8 Markdown under `tools/specbind/assets/protocols/<selector>.md`, embedded as product assets outside the consumer project's OKF bundle.
- They are not `SpecBind Rule` concepts and require no OKF Front Matter, `artifact_id`, schema version, lifecycle metadata, or user-selected language variant.
- The selector is a stable lowercase kebab-case product identifier declared by the embedded registry, not inferred from an installed path.
- Protocol content is versioned with the SpecBind binary. It may evolve with product behavior and is never merged into project files.
- Protocols may cite accepted decisions and CLI commands but do not duplicate complete schemas, state machines, or command help.
- Protocols contain semantic criteria and reusable reasoning procedures. Skill invocation order, retry limits, confirmation prompts, mutation calls, and platform-specific syntax remain in skills.

Skills author user-facing artifacts and reports in the configured project language after applying an English protocol. CLI protocol output itself remains English-only under Decision 0067.

### Loading and precedence

A product-managed skill names each required selector explicitly and reads it at most once per invocation. It does not discover protocols by listing first. `protocol list` exists for human inspection, diagnostics, and development.

Precedence is:

1. CLI invariants and guarded-operation results cannot be weakened by any prose carrier.
2. Product protocols and product-managed skill contracts are non-waivable and must be mutually consistent.
3. Project rules may strengthen or specialize the product baseline but cannot relax it.
4. Templates control valid structure and placement without redefining semantics.

When a project rule contradicts a product protocol, the skill follows the product baseline, reports the conflicting project policy, and requests clarification if the requested stronger behavior cannot be determined. It never silently treats the protocol as optional or replaces the project file.

Failure to read a required selector indicates incompatible product assets, normally an older binary used with newer installed skills. The skill stops and instructs the user to align the binary and refresh product-managed skills with `specbind install`; it does not fall back to a bundled copy inside the skill or to a project rule.

Reading a protocol is run-scoped context. Protocol content and versions are not copied into artifact Front Matter, fingerprints, gate evidence, or release records. Accepted artifacts and Git history remain the durable record of the applied judgment.

Updating a protocol therefore does not mechanically stale existing approvals. When a protocol change requires existing projects or accepted artifacts to be revisited, the SpecBind release must define an explicit migration, validation, or rewind action; ordinary protocol wording changes do not trigger invisible project-wide invalidation.

### cc-sdd and OKF disposition

The following inherited procedural material becomes product protocol content after removing orchestration and obsolete syntax:

- `requirements-review-gate.md` -> `requirements-review`
- `design-discovery-full.md` plus `design-discovery-light.md` -> `design-discovery`
- `design-synthesis.md` plus non-customizable authoring baseline from `design-principles.md` -> `design-authoring`
- `design-review-gate.md` plus `design-review.md` -> `design-validation`
- `gap-analysis.md` -> `gap-analysis`
- `tasks-parallel-analysis.md` plus non-customizable planning baseline from `tasks-generation.md` -> `task-planning`

Skill-owned control flow is removed from those documents and remains in the owning skill.

This decision supersedes Decision 0049's placement and mutability choice: its concise OKF content moves from the installed `settings/rules/okf-artifacts.md` file to the immutable `okf-authoring` protocol. The targeted OKF version and authoring baseline remain accepted. Projects may express additional OKF-related preferences in ordinary steering, but cannot replace the product protocol.

`cross-spec-review` has no cc-sdd source because persistent typed Contracts are a SpecBind capability.

## Consequences

- Shared product judgment has one implementation-independent source across Codex and Claude Code.
- Project rules become genuinely optional customization rather than a mixture of policy and required workflow.
- The installed rule set shrinks from six files to five because `okf-artifacts.md` is no longer user-owned.
- Skills stay focused on orchestration while the CLI remains the only mechanical enforcement boundary.
- Protocol content automatically matches the executing binary but requires explicit handling of stale installed skills.
- Humans can inspect the exact product protocol without locating generated agent assets.

## Implementation status

The CLI surface is implemented. `protocol list` and `protocol read <selector>` resolve from an explicit embedded registry, take no project path at all, and therefore work without a project root, `.specbind.json`, or installation. A read writes the raw Markdown body to stdout with no wrapper; an unknown selector returns `ERROR PROTOCOL_SELECTOR_NOT_FOUND`, keeps stdout empty, and lists the available selectors.

One of the eight accepted protocol documents is authored: `okf-authoring`. The remaining seven selectors are not yet registered, so `protocol list` currently reports one entry. Authoring them, referencing them from cross-agent skills, and adding the version-mismatch and customization-cannot-override tests remain separate increments.
