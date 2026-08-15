import { describe, it, expect } from 'vitest';
import { processManifest } from '../src/manifest/processor';
import type { AgentType } from '../src/resolvers/agentLayout';
import type { OSType } from '../src/resolvers/os';
import { buildTemplateContext } from '../src/template/context';

describe('OS filtering in processManifest', () => {
  const agent: AgentType = 'claude-code-skills';
  const ctx = buildTemplateContext({ agent, lang: 'en' });

  it('includes artifacts with no OS condition', () => {
    const manifest = {
      version: 1,
      artifacts: [
        {
          id: 'common_commands',
          source: {
            type: 'templateDir' as const,
            fromDir: 'templates/agents/{{AGENT}}/commands/os-mac',
            toDir: '{{AGENT_COMMANDS_DIR}}',
          },
        },
      ],
    };

    const macResult = processManifest(manifest, agent, ctx, 'mac');
    const windowsResult = processManifest(manifest, agent, ctx, 'windows');
    const linuxResult = processManifest(manifest, agent, ctx, 'linux');

    expect(macResult).toHaveLength(1);
    expect(windowsResult).toHaveLength(1);
    expect(linuxResult).toHaveLength(1);
  });

  it('includes artifacts when OS matches single condition', () => {
    const manifest = {
      version: 1,
      artifacts: [
        {
          id: 'mac_commands',
          source: {
            type: 'templateDir' as const,
            fromDir: 'templates/agents/{{AGENT}}/commands/os-mac',
            toDir: '{{AGENT_COMMANDS_DIR}}',
          },
          when: { os: 'mac' as OSType },
        },
      ],
    };

    const macResult = processManifest(manifest, agent, ctx, 'mac');
    const windowsResult = processManifest(manifest, agent, ctx, 'windows');

    expect(macResult).toHaveLength(1);
    expect(windowsResult).toHaveLength(0);
  });

  it('includes artifacts when OS matches array condition', () => {
    const manifest = {
      version: 1,
      artifacts: [
        {
          id: 'unix_commands',
          source: {
            type: 'templateDir' as const,
            fromDir: 'templates/agents/{{AGENT}}/commands/os-unix',
            toDir: '{{AGENT_COMMANDS_DIR}}',
          },
          when: { os: ['mac', 'linux'] as OSType[] },
        },
      ],
    };

    const macResult = processManifest(manifest, agent, ctx, 'mac');
    const linuxResult = processManifest(manifest, agent, ctx, 'linux');
    const windowsResult = processManifest(manifest, agent, ctx, 'windows');

    expect(macResult).toHaveLength(1);
    expect(linuxResult).toHaveLength(1);
    expect(windowsResult).toHaveLength(0);
  });

  it('combines agent and OS filtering correctly', () => {
    const manifest = {
      version: 1,
      artifacts: [
        {
          id: 'claude_mac_skills',
          source: {
            type: 'templateDir' as const,
            fromDir: 'templates/agents/{{AGENT}}/commands/os-mac',
            toDir: '{{AGENT_COMMANDS_DIR}}',
          },
          when: { agent: 'claude-code-skills' as AgentType, os: 'mac' as OSType },
        },
        {
          id: 'codex_mac_skills',
          source: {
            type: 'templateDir' as const,
            fromDir: 'templates/agents/{{AGENT}}/commands/os-mac',
            toDir: '{{AGENT_COMMANDS_DIR}}',
          },
          when: { agent: 'codex-skills' as AgentType, os: 'mac' as OSType },
        },
      ],
    };

    const claudeMacResult = processManifest(manifest, 'claude-code-skills', ctx, 'mac');
    const claudeWindowsResult = processManifest(manifest, 'claude-code-skills', ctx, 'windows');
    const codexMacResult = processManifest(manifest, 'codex-skills', buildTemplateContext({ agent: 'codex-skills', lang: 'en' }), 'mac');

    expect(claudeMacResult).toHaveLength(1);
    expect(claudeMacResult[0].id).toBe('claude_mac_skills');
    expect(claudeWindowsResult).toHaveLength(0);
    expect(codexMacResult).toHaveLength(1);
    expect(codexMacResult[0].id).toBe('codex_mac_skills');
  });

  it('excludes artifacts when OS does not match', () => {
    const manifest = {
      version: 1,
      artifacts: [
        {
          id: 'windows_only',
          source: {
            type: 'templateFile' as const,
            from: 'templates/windows/powershell.tpl.md',
            toDir: '{{AGENT_COMMANDS_DIR}}',
          },
          when: { os: 'windows' as OSType },
        },
      ],
    };

    const macResult = processManifest(manifest, agent, ctx, 'mac');
    const windowsResult = processManifest(manifest, agent, ctx, 'windows');

    expect(macResult).toHaveLength(0);
    expect(windowsResult).toHaveLength(1);
  });
});
