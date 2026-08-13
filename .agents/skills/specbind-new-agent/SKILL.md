---
name: specbind-new-agent
description: Add or extend coding-agent support in specbind by executing the SOP in docs/specbind/sop-new-agent.md end-to-end. Use when introducing a new agent, adding a subagent-capable variant, or evaluating migration of an existing supported agent to skills-based templates.
---

# SpecBind New Agent Integration

## Goal

Add a production-ready agent integration to `tools/specbind` with complete research, implementation, and verification artifacts.
Use the existing SOP and templates in this repository instead of inventing a new process.

## Default Operation Mode

Use `plan-first` as the default:
- First deliverable: `docs/specbind/plans/agent-plan-{agent-id}.md`
- Second deliverable (after plan approval): implementation changes in `tools/specbind/...`

Do not skip the plan document unless the user explicitly requests direct implementation.

## Collect Inputs First

Collect the minimum inputs before editing files:
- Target agent display name and agent id (kebab-case)
- Official documentation URL for command/agent/skills format
- Integration scope: `commands-only`, `commands+agents`, or `skills`
- Whether work type is `new-agent` or `migration`

If any item is unclear, resolve it during Phase 1 research before creating implementation files.

## Execution Workflow

### 1) Run Phase 1 Research

Read `docs/specbind/sop-new-agent.md` and extract all required spec details from official docs:
- commands directory
- agent directory (if supported)
- documentation filename
- command invocation format
- YAML frontmatter schema for commands and agents (or skills)
- placeholders/arguments format

Record exact values (not assumptions) in the plan document.

### 2) Create Plan Document

Copy and fill:
- `docs/specbind/templates/agent-plan-template.md`

Include:
- Phase 1 research summary table
- Differences vs nearest existing agent pattern
- 5-step implementation plan
- changed-file list
- verification plan

Write the completed plan to:
- `docs/specbind/plans/agent-plan-{agent-id}.md`

Before implementation, ensure the plan explicitly states:
- integration scope (`commands-only`, `commands+agents`, `skills`)
- whether this is `new-agent` or `migration`
- compatibility impact and rollback approach

### 3) Choose Base Pattern and Implement

Pick the nearest existing implementation and copy from it:
- `commands-only`: start from `tools/specbind/templates/agents/codex` or another commands-only agent
- `commands+agents`: start from `tools/specbind/templates/agents/claude-code-agent` or `tools/specbind/templates/agents/opencode-agent`
- `skills`: start from `tools/specbind/templates/agents/claude-code-skills`

Start implementation only after the plan file exists and the user has confirmed to proceed.

Apply the standard 5 implementation steps from SOP:
1. Update `tools/specbind/src/agents/registry.ts`
2. Add `tools/specbind/templates/manifests/{agent-id}.json`
3. Add `tools/specbind/templates/agents/{agent-id}/...`
4. Convert YAML frontmatter to target-agent format (keep body unless incompatibility is confirmed)
5. Add `tools/specbind/test/realManifest{AgentName}.test.ts`

For `skills` integration, generate `SKILL.md`-based command packages under `templates/agents/{agent-id}/skills/`.

### 4) Handle Migration Cases

If the request is migration (or migration may be needed), read `references/skills-migration.md` and decide strategy before editing:
- Recommended default: additive migration (`{agent-id}-skills` as a new option)
- Use in-place replacement only when explicitly requested

Always document compatibility impact and CLI flag impact in the plan.

### 5) Verify Before Reporting

Run verification in `tools/specbind`:
- `npm test`
- `npm run build && node dist/index.js --agent {agent-id} --dry-run`
- local temp-directory apply test with `--overwrite=force`
- language checks (`--lang ja`, `--lang en`)
- generated file count checks

If tests cannot run, explicitly report which step is blocked and why.

## Completion Criteria

- Plan file exists at `docs/specbind/plans/agent-plan-{agent-id}.md`
- Registry entry exists and routes to the new manifest id
- Manifest artifacts match target integration scope
- Templates render to expected target directories
- Real manifest test exists and passes
- Dry-run output matches expected artifact plan
- Plan document is updated with research evidence and verification notes

## References

- SOP: `docs/specbind/sop-new-agent.md`
- Plan template: `docs/specbind/templates/agent-plan-template.md`
- Migration guide: `references/skills-migration.md`
- Plan checklist: `references/plan-output-checklist.md`
