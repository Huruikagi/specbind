# Changelog

All notable changes to SpecBind will be documented in this file.

## Unreleased

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
