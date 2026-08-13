# SpecBind Repository Guidelines

## Repository Purpose

- This repository develops SpecBind itself. It is not a consumer project using SpecBind to deliver an application.
- SpecBind was bootstrapped from `gotalab/cc-sdd` and has been detached from the GitHub fork network.
- Treat inherited cc-sdd code and documentation as migration inputs, not as the final SpecBind interface.
- Prefer SpecBind-specific commands, workflows, terminology, and compatibility decisions over drop-in cc-sdd compatibility unless compatibility is explicitly required.

## Source Layout

- `tools/specbind/src/` — CLI implementation
- `tools/specbind/test/` — automated tests
- `tools/specbind/templates/` — files installed into consumer projects
- `docs/guides/` — current and historical user-facing guides
- `docs/specbind/plans/` — repository migration and implementation plans
- `.kiro/settings/` and `.kiro/specs/` — inherited fixtures and test data unless a task explicitly establishes otherwise

Do not treat this repository's `.kiro/` contents as project steering or as active specifications for developing SpecBind itself. In particular, do not require `/kiro-*` workflows merely because those files exist.

## Development Workflow

- Follow the user's requested scope and make changes directly unless they explicitly request a SpecBind specification workflow.
- Keep changes narrow and preserve unrelated work in the worktree.
- When changing installed behavior, update the relevant source, templates, tests, and documentation together.
- Keep Claude Code and Codex templates aligned where they implement the same contract, while preserving platform-specific invocation syntax and capabilities.

## Validation

Run commands from `tools/specbind/`:

```sh
npm test
npm run build
```

- Add or update focused tests for behavior changes.
- Before reporting completion, inspect the final diff and confirm generated or installed templates still match their intended consumer environment.

## Language

- Respond to the user in Japanese unless they request another language.
- Preserve the language of an existing document unless the task requires translation.
- English and Japanese are the only officially supported product languages during the current stabilization phase.
