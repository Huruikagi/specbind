# Getting Started

How you start with SpecBind depends on whether your project already has an
implementation. Choose the route that matches its current state.

!!! info "Terminology"
    Terms used in these guides, including Spec, Steering, Milestone, and Gate,
    are explained in [Core concepts](./concepts.md). Read that page first or
    refer to it as the terms appear.

## Start a new project

Use this route before application implementation has begun. Establish and
commit the project foundation and first release scope, then let Discovery
classify the supplied material into durable responsibility boundaries.

This route assumes that the product direction is already reasonably clear. If
you are still exploring what to build, prototype without SpecBind first.

[Start a new project](./start-new-project.md)

## Start with an existing project

Use this route when the repository already contains code or tests. You can
either use SpecBind for the next change or establish Specs from the existing
implementation.

[Start with an existing project](./start-existing-project.md)

## Requirements for either route

- A target Git repository. A new repository needs an initial baseline commit;
  an existing repository needs at least one commit.
- A coding agent.
- Windows x64, Linux x64 on WSL2, or macOS ARM64.

### Supported coding agents

SpecBind is developed and tested with Codex and Claude Code. Other coding
agents should work when they support Agent Skills and `AGENTS.md`, including
Cursor, GitHub Copilot, and Devin, but those integrations have not received the
same verification. If something fails, use the
[feedback guide](./feedback.md).

## What SpecBind does not do

SpecBind is not an application scaffolding tool. It maintains specifications,
designs, implementation plans, and delivery lifecycle state through coding
agents and the CLI. Initialize the project and select frameworks using your
normal workflow.

---

[Guide home](../index.md) | [Core concepts](./concepts.md)
