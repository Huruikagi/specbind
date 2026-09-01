# Changelog

All notable changes to SpecBind will be documented in this file.

## Unreleased

## 1.2.0 - 2026-09-02

- Added complete, fail-closed GitHub Milestone discovery through the external
  `sb-discovery` provider, without adding network or credential state to the
  SpecBind CLI.
- Made configuration Steering-first and added automatic, Spec-local one-off
  Design supplements for independently justified design decisions.
- Consolidated the installed product Skill surface under the `sb-*` namespace,
  including unified planning, milestone driving, and Discovery-owned adoption
  procedures.
- Expanded the English and Japanese public guides, templates, and shared
  writing guidance while preserving the established 1.x CLI compatibility
  surface and diagnostics.

## 1.1.0 - 2026-09-01

- Promoted `1.1.0-rc.1` to the stable `1.1.0` release without product changes
  after the full native build, forward-test, archive, checksum, installer, and
  mise verification gates completed. The documented 1.x compatibility surface
  remains unchanged.

## 1.1.0-rc.1 - 2026-09-01

- Expanded the English and Japanese Requirements, Design, UI, research,
  implementation-notes, Roadmap, and Steering templates with stronger
  creation and maintenance guidance, while keeping project-owned templates
  opt-in and future-materialization oriented.
- Added `specbind-drive` to advance an active milestone across planning,
  implementation, and validation through the existing owning Skills and their
  guarded boundaries.
- Reframed template placeholders as named creation outputs, including Markdown
  fragments for repeatable structures. Templates published with `1.0.0` that
  use `create bind=<name>` remain valid throughout 1.x, and their established
  diagnostic codes remain stable.
- Added a Japanese language-style Rule and improved the readability of the
  Japanese public guide without changing exact product or machine identifiers.
- Added complete English and Japanese guides for step-by-step implementation
  and plan-and-drive delivery.

## 1.0.0 - 2026-08-31

- Published the first stable SpecBind release after the complete `1.0.0-rc.3`
  release candidate passed the Rust, inherited TypeScript, Decision, strict
  documentation, behavioral forward-test, archive, installer, and mise gates.
- Established the executable 1.x forward-compatibility contract. Migration from
  0.x is not applicable because no external adopter depends on 0.x project
  state; the maintainer accepts responsibility for maintainer-owned local
  conversion.

## 1.0.0-rc.3 - 2026-08-31

- Replaced the prose Contract artifact with a strict versioned YAML model and
  added direct dependency and reverse-consumer graph reads.
- Added guided project configuration, generic agent integration, attributable
  template variables, rule-selected Design sets, and fail-closed default
  scaffolds.
- Unified planning under `specbind-plan`, with explicit single-phase entry
  points and an explicit scope choice before planning begins.
- Added local Discovery source collections, completion-preserving release
  binding, command-specific JSON status projections, and sequential Task
  checkpoints.
- Added native macOS ARM64 release archives, checksum verification, shell and
  mise installation smoke tests, and a non-publishing manual release preflight.
- Added an offline `specbind feedback` entry point and bilingual GitHub Issue
  Forms for structured CLI, Skill, integration, and documentation bug reports
  and improvement proposals.
- Added a complete bilingual public guide and one canonical English cc-sdd
  migration URL for every CLI handoff.

## 1.0.0-rc.2 - 2026-08-25

- Added project-owned Rule discovery and read commands with raw, maintainer,
  and consumer projections for safe agent customization.
- Added customizable Roadmap body templates while keeping lifecycle metadata
  and state transitions owned by SpecBind.

## 1.0.0-rc.1 - 2026-08-24

- Added a Steering-first existing-project adoption workflow with a clean Git
  evidence preflight, revision-pinned reverse-discovery dossier, confirmed Spec
  boundary handoff, and normal Requirements/Design lifecycle continuation.
- Added guarded removal of one installed agent and explicit project uninstall
  modes that retain or remove durable SpecBind knowledge.
- Normalized the Japanese public documentation hierarchy and established
  Japanese-first authoring with an English-default final publication structure.
- Defined forward-upgrade compatibility within an executable major version and
  required a documented migration route for future breaking major releases.

## 0.2.0 - 2026-08-22

- Added supported installation through mise's GitHub backend, including
  release-workflow smoke tests on Windows and Linux.
- Made milestone and Spec status diagnostics phase-aware so expected future
  work is distinguished from actionable blockers.
- Added lifecycle-scoped artifact instructions, target-aware template
  resolution, and clearer steering and deferred-finding ownership.
- Enabled safe local Git checkpoints by default for eligible workflow units,
  while keeping push, branch changes, and unrelated work outside that policy.
- Added Release adapter bootstrapping, explicit empty-policy handling, complete
  release-procedure discovery, and guarded finalization checkpoints.
- Strengthened Discovery, Requirements, Design, Tasks, implementation, and
  validation skills with cleaner handoffs and independently verifiable evidence.
- Split the forward-test handbook into focused scenario and results documents,
  and added end-to-end journey fixtures for release-workflow verification.
- Tightened the `scope/v1` generated schema so its version field accepts only
  schema version 1.

## 0.1.0 - 2026-08-21

- Published the first stable pre-1.0 binary release for Windows x64 and Linux
  x64, with a tag-driven GitHub Release pipeline,
  checksum-verified PowerShell and shell installers, and release operations
  documentation.
- Added the Rust CLI for installing SpecBind, validating artifacts and
  traceability, recording lifecycle approvals and task progress, inspecting
  milestone status, and finalizing releases.
- Added product-managed Codex and Claude Code skills, project-owned templates,
  shared rules, release and Git adapters, and configurable agent-role adapters.
- Added guarded deterministic and agent-assisted migration from cc-sdd,
  including final retirement of migrated legacy sources.
- Preserved the independent v1 product contract and version-one artifact
  schemas while starting executable SemVer at the pre-1.0 release line.
- Added a Japanese Preview user guide covering Preview installation, the
  first Spec-backed workflow, core lifecycle concepts, customization, and
  migration from cc-sdd.
- Fixed `specbind spec list` to report an empty project immediately after
  installation, before the first Spec directory exists.
- Added a MkDocs Material documentation site and GitHub Pages deployment for
  the Japanese Preview guide and current reference pages.

## Upstream history

SpecBind was bootstrapped from
[gotalab/cc-sdd](https://github.com/gotalab/cc-sdd).

For changes made before SpecBind became an independent project, see the
[cc-sdd changelog](https://github.com/gotalab/cc-sdd/blob/main/CHANGELOG.md)
and [cc-sdd releases](https://github.com/gotalab/cc-sdd/releases).
