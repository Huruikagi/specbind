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

## Guides

1. [Getting Started](./guide/getting-started.md) — choose the route for a new or existing project
   - [Start a new project](./guide/start-new-project.md)
   - [Start with an existing project](./guide/start-existing-project.md)
2. Implement changes — choose the route that fits the review granularity and Milestone size
   - [Plan and implement one item at a time](./guide/implement-step-by-step.md)
   - [Plan and Drive a Milestone](./guide/implement-with-plan-and-drive.md)
   - [Release a milestone](./guide/release.md)
3. [Update SpecBind](./guide/update.md) — update the binary and refresh product-managed project files
4. [Core concepts](./guide/concepts.md) — understand the model behind the workflow
5. [Customize SpecBind](./guide/customization.md) — adapt supported project-owned surfaces
6. [Report bugs and suggest improvements](./guide/feedback.md)
7. [Remove an Agent or uninstall](./guide/uninstall.md)

## Reference

- [Current generated skill index](./reference/current-skill-index.md)
- [Current generated artifact index](./reference/current-artifact-index.md)

## If you use cc-sdd

- [Migrate from cc-sdd](./guide/migrate-from-cc-sdd.md)

---

[Continue to Getting Started](./guide/getting-started.md) | [GitHub repository](https://github.com/Huruikagi/specbind)
