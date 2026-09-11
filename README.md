# SpecBind

SpecBind keeps durable software specifications bound to agent-assisted delivery,
from intent through release.

**[Read the user guide](https://huruikagi.github.io/specbind/)** ·
[日本語のユーザーガイド](https://huruikagi.github.io/specbind/ja/) ·
[Choose a route](https://huruikagi.github.io/specbind/guide/getting-started/) ·
[Core concepts](https://huruikagi.github.io/specbind/guide/concepts/)

AI coding agents can implement quickly, but the reasoning around a change is easy to lose: requirements become one-off prompts, design decisions drift away from the code, and a later agent has to reconstruct what the product is supposed to do. SpecBind gives that reasoning a maintained home and makes it part of the delivery lifecycle.

## What SpecBind does

SpecBind combines agent skills with a deterministic CLI:

- **Skills own judgment.** Agents help discover the right scope, author requirements and designs, review contracts, plan tasks, implement changes, and evaluate results.
- **The CLI owns invariants.** It validates artifacts and traceability, records approvals and task progress, detects stale downstream work, and guards lifecycle and release transitions.
- **Specs stay alive.** A Spec describes a product capability across milestones and releases. Later changes update the same durable requirements, design, and external contract instead of starting from a disposable plan.
- **Existing products can establish a baseline.** Evidence-backed reverse discovery can establish durable Specs from a fixed implementation revision without presenting existing behavior as a new release.
- **Milestones make delivery explicit.** A Roadmap groups the work intended for a release, including dependencies across Specs and smaller Direct changes that do not need their own Spec.
- **Contracts expose cross-Spec seams early.** A contract-first review happens before task planning, so ownership conflicts, dependency cycles, and integration assumptions surface before implementation.

SpecBind is not a gate on every repository edit. Work enters the workflow when it belongs to a tracked delivery, changes behavior or boundaries owned by a Spec, or creates a new durable responsibility. Unrelated maintenance can remain ordinary work.

## The lifecycle

```text
discover scope
  -> requirements
  -> design and contract
  -> contract review
  -> tasks
  -> implementation and verification
  -> release
```

Approvals bind each phase to the exact inputs that were reviewed. If an upstream artifact changes, SpecBind marks the affected downstream evidence stale rather than letting an agent silently continue from an obsolete plan. Faster orchestration can reuse the same artifacts and guards without defining a weaker workflow.

Projects can adapt document templates, shared rules, and Git, release, or final-validation guidance while keeping the product's validation and state transitions consistent. SpecBind is developed and tested with Codex and Claude Code, and provides shared Agent Skills and `AGENTS.md` integration for other compatible agents. English and Japanese are the officially supported artifact languages.

## Get started

### Install the CLI

The [latest stable release](https://github.com/Huruikagi/specbind/releases/latest)
supports Windows x64, Linux x64, macOS ARM64, and Linux ARM64 GNU.

With [mise](https://mise.jdx.dev/), on any supported platform:

```sh
mise use github:Huruikagi/specbind
mise lock
```

`mise use` records the latest eligible stable version in the mise configuration
for the current directory, and `mise lock` records its checksum so the team uses
the same release. mise applies a minimum release age to `latest` by default; to
take a release that is not yet eligible, select it explicitly with
`github:Huruikagi/specbind@<version>`.

Without mise, use the platform installer.

Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/Huruikagi/specbind/main/install.ps1 | iex
```

WSL2/Linux or Apple Silicon macOS:

```sh
curl -fsSL https://raw.githubusercontent.com/Huruikagi/specbind/main/install.sh | sh
```

Both installers verify the release archive against `SHA256SUMS`, install to the
platform default, and leave persistent `PATH` changes to the user. Use
`-InstallDir` on PowerShell or `--install-dir` on Linux/macOS to choose another
location. Confirm the installation with `specbind --version`.

### Install SpecBind into a project

From the root of a Git repository with at least one commit:

```sh
specbind install --agent codex --language en --project-instructions
```

Use `claude-code` instead of `codex` for Claude Code, or `generic` for another
host that supports Agent Skills and `AGENTS.md`. Repeat `--agent` for more than
one integration, and use `ja` instead of `en` for Japanese artifacts. Review and
commit the installed files, then reopen the coding-agent session so it discovers
the new Skills.

### Choose a route

- [Start a new project](https://huruikagi.github.io/specbind/guide/start-new-project/)
  before application implementation has begun.
- [Start with an existing project](https://huruikagi.github.io/specbind/guide/start-existing-project/)
  when code or tests already exist.

Codex invokes Skills as `$sb-*` and Claude Code as `/sb-*`. Ordinary changes
start with `sb-discovery`, then `sb-plan` and `sb-drive` advance the Milestone.
Use `sb-configure` whenever project settings need review.

## Reference

- [Skill index](https://huruikagi.github.io/specbind/reference/current-skill-index/) — every installed Skill and when to use it
- [Artifact index](https://huruikagi.github.io/specbind/reference/current-artifact-index/) — installed files and maintained artifacts, with their owners
- [Lifecycle states](https://huruikagi.github.io/specbind/reference/lifecycle-states/) — Spec states and Milestone stages reported by the CLI
- [CLI commands](https://huruikagi.github.io/specbind/reference/cli-commands/) — `specbind` commands grouped by purpose

## Development

This repository develops SpecBind itself:

- `tools/specbind/` — canonical Rust workspace for the `specbind` executable
- `tools/cc-sdd/` — inherited TypeScript migration oracle
- `docs/` — public user guide (`en/`, `ja/`), design documents, and Decisions

The [repository map](./docs/repository-map.md) indexes the source layout and
design documents, and the [Decision index](./docs/design/decisions/index.md)
lists every accepted and superseded decision.

The workspace uses Rust 1.98.1, Rustfmt, and Clippy for development through [`rust-toolchain.toml`](./tools/specbind/rust-toolchain.toml), while [`Cargo.toml`](./tools/specbind/Cargo.toml) retains Rust 1.97.1 as the minimum supported Rust version. Install [Rustup](https://rustup.rs/) before running Cargo commands. Windows development with the default MSVC target also requires Visual Studio Build Tools with the **Desktop development with C++** workload and a Windows SDK.

Run the current CLI from the Rust workspace:

```sh
cd tools/specbind
cargo run -- --help
```

Run the complete Rust verification set before committing Rust changes:

```sh
cargo fmt --all -- --check
cargo run --example generate_schemas -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
```

`Cargo.lock` is committed because SpecBind distributes an application binary.
The ordinary [Rust workflow](./.github/workflows/rust.yml) runs these checks on
Linux. The [release workflow](./.github/workflows/release.yml) repeats them while
building the supported native archives on Windows, Linux, and macOS.

The versioned Rust DTOs under [`src/schema/`](./tools/specbind/src/schema/) are the structural source of truth for structured artifacts. After changing them, regenerate the checked-in Draft 2020-12 schemas and review the resulting diff:

```sh
cargo run --example generate_schemas
```

Embedded skills also have behavioral verification that cannot run in CI. Build its fixture project and follow [the forward-test procedure](./docs/skill-forward-tests.md):

```sh
sh tools/specbind/scripts/forward-test-fixture.sh /tmp/specbind-fixture en
```

To build the documentation site locally, install `requirements-docs.txt` and run
`python -m mkdocs build --strict` from the repository root.

The inherited TypeScript oracle is excluded from routine verification. Its
checks remain available when changing `tools/cc-sdd/` or when executable
evidence about inherited behavior is needed; passing them verifies only the
reference tree, not the current SpecBind product contract:

```sh
cd tools/cc-sdd
npm test
npm run build
```

## Upstream and attribution

SpecBind began from the source code of [cc-sdd](https://github.com/gotalab/cc-sdd) by gotalab, which itself inherited from Kiro. We are grateful to Kiro, the original project, and their contributors for the foundation.

SpecBind is an independent project and is not affiliated with or endorsed by gotalab.

## License

MIT License. The original copyright and license notice are retained in [LICENSE](./LICENSE).
