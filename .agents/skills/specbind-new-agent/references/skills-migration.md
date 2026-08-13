# Skills Migration Guide for Existing Agents

Use this guide when the target agent is already supported in `specbind` and now offers a native Skills mechanism.

## Decision Rule

Default to additive migration:
- Keep existing command/agent integration
- Add a new `{agent-id}-skills` variant
- Add dedicated alias flags (for example `--foo-skills`)

Use in-place migration only when the user explicitly asks for a breaking change.

## Migration Checklist

1. Confirm official Skills format
- Verify skill directory path
- Verify skill file format (`SKILL.md` schema and optional metadata files)
- Verify invocation style differences from command mode

2. Compare current implementation
- Check `tools/specbind/src/agents/registry.ts` for current `agent-id`
- Check current manifest in `tools/specbind/templates/manifests/`
- Check current templates in `tools/specbind/templates/agents/{agent-id}/`
- Check existing real-manifest test coverage in `tools/specbind/test/`

3. Define migration target
- New variant id: `{agent-id}-skills`
- New templates root: `tools/specbind/templates/agents/{agent-id}-skills/`
- New manifest: `tools/specbind/templates/manifests/{agent-id}-skills.json`
- New test: `tools/specbind/test/realManifest{AgentName}Skills.test.ts`

4. Implement skills-specific behavior
- Registry `layout.commandsDir` points to skills directory
- Registry `commands.*` examples use Skills invocation syntax
- Manifest artifact switches from `commands` to `skills` templateDir when needed
- Template payload contains Skill folders with `SKILL.md` files

5. Verify backward compatibility
- Existing `--{agent-id}` flow still works
- New `--{agent-id}-skills` flow works
- Dry-run for both outputs expected artifact destinations
- Real manifest tests exist for both variants

## Recommended Base Example

Use these files as concrete reference for skills-mode implementation:
- `tools/specbind/src/agents/registry.ts` (`claude-code-skills` entry)
- `tools/specbind/templates/manifests/claude-code-skills.json`
- `tools/specbind/templates/agents/claude-code-skills/`
- `tools/specbind/test/realManifestClaudeCodeSkills.test.ts`

## Review Output Requirement

Always include a short migration impact section in the plan/result:
- Compatibility impact (`none` for additive)
- CLI flag changes
- Documentation updates required
- Rollback path (remove new variant only)
