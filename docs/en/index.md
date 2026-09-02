# SpecBind user guide

SpecBind helps you develop software with AI coding agents while keeping
specifications and implementation connected over time. Instead of leaving
requirements and design decisions in one-off prompts, it preserves them as
maintained project artifacts.

SpecBind combines Agent Skills with the `specbind` CLI. Skills make semantic
decisions, author and review artifacts, implement changes, and explain results.
The CLI validates structure and consistency and records approvals, progress,
and lifecycle transitions.

SpecBind started from [gotalab/cc-sdd](https://github.com/gotalab/cc-sdd/tree/main),
which itself inherited from Kiro. It retains many concepts from cc-sdd v3,
reorganized around SpecBind's current contracts. We are grateful to Kiro,
cc-sdd, and their contributors for that foundation.

## Supported environments

SpecBind v1 release binaries target Windows x64, Linux x64, and macOS ARM64.
Linux x64 is verified on WSL2, and macOS ARM64 is verified on Apple Silicon CI.

## Get started

Start here: choose a route and install SpecBind.

- [Choose a route](./guide/getting-started.md) — new project or existing project
- [Install SpecBind](./guide/install.md) — shared by both routes
- [Start a new project](./guide/start-new-project.md)
- [Start with an existing project](./guide/start-existing-project.md)

## Work with SpecBind

The everyday workflow after installation.

- [Core concepts](./guide/concepts.md) — Spec, Milestone, Gate, and the model behind the workflow
- [Plan and implement one item at a time](./guide/implement-step-by-step.md) — inspect every artifact and Gate
- [Plan and Drive a Milestone](./guide/implement-with-plan-and-drive.md) — advance all safely reachable work
- [Establish Specs from an existing implementation](./guide/adopt-existing.md) — make current code the baseline
- [Release a milestone](./guide/release.md) — close a Milestone as one release

## Configure and maintain

- [Customize SpecBind](./guide/customization.md) — templates, Rules, adapters, Steering, per-role models
- [Update SpecBind](./guide/update.md) — update the binary and product-managed project files
- [Remove an Agent or uninstall](./guide/uninstall.md)
- [Migrate from cc-sdd](./guide/migrate-from-cc-sdd.md)
- [Report bugs and suggest improvements](./guide/feedback.md)

## Reference

- [Current generated skill index](./reference/current-skill-index.md)
- [Current generated artifact index](./reference/current-artifact-index.md)

---

[Choose a route](./guide/getting-started.md) | [GitHub repository](https://github.com/Huruikagi/specbind)
