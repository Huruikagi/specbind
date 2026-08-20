# Changelog

All notable changes to SpecBind will be documented in this file.

## Unreleased

SpecBind has not published a binary release yet.

- Added the pre-1.0 GitHub Release pipeline for Windows x64 and Linux x64,
  checksum-verified PowerShell and shell installers, and release operations
  documentation.
- Set the first release candidate version to `0.1.0-rc.1` while preserving the
  independent v1 product contract and version-one artifact schemas.
- Added a Japanese Preview user guide covering Preview installation, the
  first Spec-backed workflow, and the core lifecycle concepts.
- Fixed `specbind spec list` to report an empty project immediately after
  installation, before the first Spec directory exists.

## Upstream history

SpecBind was bootstrapped from
[gotalab/cc-sdd](https://github.com/gotalab/cc-sdd).

For changes made before SpecBind became an independent project, see the
[cc-sdd changelog](https://github.com/gotalab/cc-sdd/blob/main/CHANGELOG.md)
and [cc-sdd releases](https://github.com/gotalab/cc-sdd/releases).
