# SpecBind

SpecBind keeps durable software specifications bound to agent-assisted delivery,
from intent through release.

AI coding agents can implement quickly, but the reasoning around a change is easy to lose: requirements become one-off prompts, design decisions drift away from the code, and a later agent has to reconstruct what the product is supposed to do. SpecBind gives that reasoning a maintained home and makes it part of the delivery lifecycle.

## What SpecBind does

SpecBind combines agent skills with a deterministic CLI:

- **Skills own judgment.** Agents help discover the right scope, author requirements and designs, review contracts, plan tasks, implement changes, and evaluate results.
- **The CLI owns invariants.** It validates artifacts and traceability, records approvals and task progress, detects stale downstream work, and guards lifecycle and release transitions.
- **Specs stay alive.** A Spec describes a product capability across milestones and releases. Later changes update the same durable requirements, design, and external contract instead of starting from a disposable plan.
- **Milestones make delivery explicit.** A Roadmap groups the work intended for a release, including dependencies across Specs and smaller Direct changes that do not need their own Spec.
- **Contracts expose cross-Spec seams early.** A contract-first review happens before task planning, so ownership conflicts, dependency cycles, and integration assumptions surface before implementation.

SpecBind is not a gate on every repository edit. Work enters the workflow when it belongs to a tracked delivery, changes behavior or boundaries owned by a Spec, or creates a new durable responsibility. Unrelated maintenance can remain ordinary work.

## The lifecycle

The deliberate path is:

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

Projects can adapt document templates, shared rules, and Git or release guidance while keeping the product's validation and state transitions consistent. SpecBind is developed and tested with Codex and Claude Code, and provides shared Agent Skills and `AGENTS.md` integration for other compatible agents. English and Japanese are the v1 artifact languages.

## Get started

### Install the CLI

The [latest stable release](https://github.com/Huruikagi/specbind/releases/latest)
supports Windows x64, Linux x64, and macOS ARM64.
The latest stable release can be installed without choosing a version.

With [mise](https://mise.jdx.dev/), on any supported platform:

```sh
mise use github:Huruikagi/specbind
```

This installs the latest stable version eligible under your mise settings and
records it in the mise configuration selected for the current directory. mise
applies a minimum release age to `latest` by default. If a newly published
stable release is not eligible yet, select that version explicitly with
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
location.

Confirm the installed version:

```sh
specbind --version
```

### Install SpecBind into a project

From the root of a Git repository with at least one commit, install the Codex
integration and English artifact defaults:

```sh
specbind install --agent codex --language en --project-instructions
```

Use `claude-code` instead of `codex` for Claude Code, and `ja` instead of `en`
for Japanese artifacts. The command installs the product-managed Skills and
creates project-owned templates, rules, and adapter guidance under
`.specbind/settings/`.

Then choose the route that matches the repository:

- [Start a new project](./docs/en/guide/start-new-project.md) before application
  implementation has begun.
- [Start with an existing project](./docs/en/guide/start-existing-project.md)
  when code or tests already exist.

The [Getting Started guide](./docs/en/guide/getting-started.md) explains both
routes and their prerequisites. The
[Japanese Getting Started guide](./docs/ja/guide/getting-started.md) covers the
same workflow in Japanese.

## Learn more

- [Documentation site](https://huruikagi.github.io/specbind/) is the published entry point for the user guide and current reference pages.
- [English user guide](./docs/en/index.md) and [Japanese user guide](./docs/ja/index.md) cover installation, delivery, customization, and removal.
- [Target workflows](./docs/design/target-workflows.md) describes the intended user journeys and responsibility boundaries.
- [Target artifact catalog](./docs/design/target-artifact-catalog.md) explains which records persist and who owns them.
- [CLI and agent boundary](./docs/design/cli-agent-boundary.md) explains why judgment belongs to agents while deterministic operations belong to the CLI.
- [Generated skill index](./docs/en/reference/current-skill-index.md) and [generated artifact index](./docs/en/reference/current-artifact-index.md) are concise snapshots of the current interface.
- [Repository map](./docs/repository-map.md) indexes the source layout, design documents, and decision record.

## Repository layout

- `tools/specbind/` — canonical Rust workspace for the `specbind` executable
- `tools/cc-sdd/` — inherited TypeScript migration oracle
- `docs/design/` — target workflows, lifecycle models, and accepted design decisions

## Development

The workspace pins Rust 1.97.1, Rustfmt, and Clippy through [`rust-toolchain.toml`](./tools/specbind/rust-toolchain.toml). Install [Rustup](https://rustup.rs/) before running Cargo commands. Windows development with the default MSVC target also requires Visual Studio Build Tools with the **Desktop development with C++** workload and a Windows SDK.

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

`Cargo.lock` is committed because SpecBind distributes an application binary. The same checks run on Windows and Linux in [the Rust workflow](./.github/workflows/rust.yml).

The versioned Rust DTOs under [`src/schema/`](./tools/specbind/src/schema/) are the structural source of truth for structured artifacts. After changing them, regenerate the checked-in Draft 2020-12 schemas and review the resulting diff:

```sh
cargo run --example generate_schemas
```

Embedded skills also have behavioral verification that cannot run in CI. Build its fixture project and follow [the forward-test procedure](./docs/skill-forward-tests.md):

```sh
sh tools/specbind/scripts/forward-test-fixture.sh /tmp/specbind-fixture en
```

The inherited TypeScript oracle retains its own verification commands:

```sh
cd tools/cc-sdd
npm test
npm run build
```

## Language support

SpecBind v1 officially supports English (`en`) and Japanese (`ja`). Other
languages are not currently part of the supported product contract.

## Upstream and attribution

SpecBind began from the source code of [cc-sdd](https://github.com/gotalab/cc-sdd) by gotalab, which itself inherited from Kiro. We are grateful to Kiro, the original project, and their contributors for the foundation.

SpecBind is an independent project and is not affiliated with or endorsed by gotalab. The original copyright and MIT license notice are retained in [LICENSE](./LICENSE).

## License

MIT License. See [LICENSE](./LICENSE).
