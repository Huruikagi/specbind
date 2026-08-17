# 0077: Define the v1 installation, distribution, and cc-sdd migration contract

Status: Accepted

## Context

The inherited TypeScript installer exposes compatibility aliases, manifests, overwrite policies, backups, profiles, and operating-system switches. SpecBind v1 is primarily invoked through agent skills after installation and needs a smaller, testable distribution surface.

## Decision

### Product distribution

- The first public Rust release is SpecBind `1.0.0`; it does not continue the private inherited TypeScript package version.
- V1 ships GitHub Release binaries only for Windows x64 and Linux x64 tested under WSL2. Each release documents the exact locally tested environments.
- macOS ARM64, macOS Intel, and Linux ARM64 are post-v1 targets.
- PowerShell and shell installer scripts download the selected binary. `--version` pins an explicit release; omission selects the latest stable release, and prereleases require an explicit version.
- Every binary has an entry in `SHA256SUMS`, and installer scripts fail closed when the checksum manifest is absent or verification fails. Platform code signing and notarization are post-v1.
- Default binary locations are `%LOCALAPPDATA%\SpecBind\bin\specbind.exe` on Windows and `$HOME/.local/bin/specbind` on Linux. `--install-dir` may override them. Scripts do not edit shell profiles or user PATH automatically; they print an exact follow-up command when the directory is not on PATH.
- The Rust binary embeds official schemas, templates, read-only product protocols, installable rules, skill assets, and defaults.

### Project installation

- `specbind install` is the initial project installer and the idempotent product-asset refresh command. The name `update` remains available for a future binary self-update operation.
- V1 install inputs are agent selection, `en|ja` language, `specDir` with `.specbind` as the default, optional project-instruction integration, and `--dry-run`.
- V1 removes public manifest, OS, profile, overwrite, backup, `--yes`, `--kiro-dir`, and other inherited compatibility options.
- Agent selection is additive and may contain Codex, Claude Code, or both. Agent removal and uninstall are post-v1.
- `.specbind.json` is version-controlled and contains `schemaVersion`, `specDir`, `language`, `agents`, and optional `projectInstructions: true`. False project-instruction state may be represented by absence.
- Product-managed agent skills are replaced with the current embedded versions when their target paths are Git-clean. Direct skill edits are not a supported customization API; Git remains recovery.
- Existing project-owned settings are never overwritten. Missing embedded default settings are created automatically and left uncommitted for review; users may remove unwanted additions before committing.
- Decision 0093 fixes the five default shared-rule paths; install and refresh treat them as project-owned settings rather than product-managed skill assets. Decision 0094 protocols remain binary-owned and are never installed as project files.
- When project instructions are enabled, the installer maintains only a marked SpecBind block in the selected agents' root `AGENTS.md` or `CLAUDE.md`. Existing surrounding content is preserved, malformed or duplicate markers stop the operation, and the selection persists through `.specbind.json`.
- Initial installation may create new files in a Git repository that has no commit. Any operation that replaces, moves, or deletes an existing file requires a commit and a clean repository first. The installer never commits project changes.

### Explicit cc-sdd migration

- Normal installation does not detect or interpret `.kiro`. Migration is invoked explicitly as `specbind migrate cc-sdd`.
- `specbind migrate cc-sdd` is a read-only plan. `specbind migrate cc-sdd --apply` recomputes and applies that plan after agent or human confirmation and Git-clean validation.
- Migration creates converted SpecBind artifacts without deleting the original `.kiro` tree. It imports only evidence and lifecycle state that can be proved; ambiguous active state returns to normal approval rather than receiving invented evidence.
- Known `kiro-*` agent asset directories are removed after successful conversion because concurrent old and new workflows are unsafe.
- An exact known legacy quickstart block may be removed from root `AGENTS.md` or `CLAUDE.md`. Edited, mixed, or ambiguous legacy instructions stop migration for manual cleanup; migration never deletes text by guessing from `kiro` words.
- The inherited TypeScript implementation moves temporarily from `tools/specbind` to `tools/cc-sdd` as a comparison and migration oracle. The Rust CLI is developed in `tools/specbind`; the temporary source is removed when Rust v1 meets its cutover gates.

## Implementation status

`specbind install --dry-run` is implemented as a read-only planner. It resolves the effective configuration from any existing `.specbind.json` merged with additive agent selection, requires an explicit language and at least one agent for an initial installation, refuses an unsupported `specDir` change, and reports each target as create, replace, or keep. Project-owned settings are reported as keep and never replaced. A plan containing any replacement enforces the accepted repository guard: at least one commit and a clean worktree.

`specbind install` applies that plan. Assets are written before the configuration, so a project only claims to be installed once the files its skills read exist, and an interrupted run converges on the next invocation because missing defaults are created and existing project files are kept. Each write revalidates the planned state and fails closed when the target changed after planning. An installation whose targets are all current returns `NO_CHANGE INSTALL_UP_TO_DATE`. The installer never commits.

Both paths currently cover `.specbind.json`, the Decision 0091 installed template set, the Decision 0093 shared-rule set, and the Decision 0096 agent skill assets rendered per selected agent. The project-instruction block specified by [Decision 0099](./0099-project-instruction-block.md) is planned and applied for each selected agent when the setting is enabled. TTY prompting for missing inputs is not implemented, so an initial installation requires explicit agent and language values.

## Consequences

- V1 has a small installation contract that can be tested in the environments the project actually controls.
- Git replaces bespoke backup directories and overwrite prompts.
- Project customization survives product updates, while generated agent resources can reliably advance.
- Migration remains explicit, reviewable, and non-destructive toward the original `.kiro` artifacts.
