# Changelog

All notable changes to SpecBind will be documented in this file.

## Unreleased

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
