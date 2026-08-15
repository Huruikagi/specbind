import { describe, it, expect } from 'vitest';
import { processManifest } from '../src/manifest/processor';
import type { AgentType } from '../src/resolvers/agentLayout';
import type { OSType } from '../src/resolvers/os';
import { buildTemplateContext } from '../src/template/context';

const manifest = {
  version: 1,
  artifacts: [
    {
      id: 'commands_static_all',
      source: {
        type: 'staticDir' as const,
        from: 'templates/agents/{{AGENT}}/skills',
        toDir: '{{AGENT_COMMANDS_DIR}}',
      },
      when: { agent: ['claude-code-skills'] as AgentType[] },
    },
  ],
};

describe('processManifest', () => {
  it('filters by when.agent and resolves placeholders in from/toDir', () => {
    const agent: AgentType = 'claude-code-skills';
    const ctx = buildTemplateContext({ agent, lang: 'en' });
    const result = processManifest(manifest, agent, ctx, 'mac' as OSType);
    expect(result).toHaveLength(1);
    const art = result[0] as any;
    expect(art.id).toBe('commands_static_all');
    expect(art.source.from).toBe('templates/agents/claude-code-skills/skills');
    expect(art.source.toDir).toBe('.claude/skills');
  });

  it('supports templateDir planning with placeholders', () => {
    const m = {
      version: 1,
      artifacts: [
        {
          id: 'dir_tpl',
          source: {
            type: 'templateDir' as const,
            fromDir: 'templates/agents/{{AGENT}}/docs',
            toDir: '{{AGENT_DIR}}',
          },
        },
      ],
    };
    const agent: AgentType = 'claude-code-skills';
    const ctx = buildTemplateContext({ agent, lang: 'en' });
    const result = processManifest(m, agent, ctx, 'mac' as OSType);
    expect(result).toHaveLength(1);
    const dir = result[0] as any;
    expect(dir.source.type).toBe('templateDir');
    expect(dir.source.fromDir).toBe('templates/agents/claude-code-skills/docs');
    expect(dir.source.toDir).toBe('.claude');
  });

  it('excludes artifacts when agent does not match', () => {
    const agent: AgentType = 'codex-skills';
    const ctx = buildTemplateContext({ agent, lang: 'en' });
    const result = processManifest(manifest, agent, ctx, 'mac' as OSType);
    expect(result).toHaveLength(0);
  });

  it('supports templateFile with rename override', () => {
    const m = {
      version: 1,
      artifacts: [
        {
          id: 'doc_tpl',
          source: {
            type: 'templateFile' as const,
            from: 'templates/docs/README.tpl.md',
            toDir: '{{AGENT_DIR}}',
            rename: '{{AGENT_DOC}}',
          },
          when: { agent: 'claude-code-skills' as AgentType },
        },
      ],
    };
    const agent: AgentType = 'claude-code-skills';
    const ctx = buildTemplateContext({ agent, lang: 'en' });
    const result = processManifest(m, agent, ctx, 'mac' as OSType);
    expect(result).toHaveLength(1);
    const art = result[0] as any;
    expect(art.source.type).toBe('templateFile');
    expect(art.source.from).toBe('templates/docs/README.tpl.md');
    expect(art.source.toDir).toBe('.claude');
    expect(art.source.outFile).toBe('CLAUDE.md');
  });

  it('derives output filename from .tpl.md/.tpl.json when rename is missing', () => {
    const m = {
      version: 1,
      artifacts: [
        {
          id: 'json_tpl',
          source: {
            type: 'templateFile' as const,
            from: 'templates/meta/config.tpl.json',
            toDir: '{{KIRO_DIR}}',
          },
        },
        {
          id: 'md_tpl',
          source: {
            type: 'templateFile' as const,
            from: 'templates/docs/README.tpl.md',
            toDir: '{{AGENT_DIR}}',
          },
        },
      ],
    };
    const agent: AgentType = 'claude-code-skills';
    const ctx = buildTemplateContext({ agent, lang: 'en' });
    const result = processManifest(m, agent, ctx, 'mac' as OSType);
    expect(result).toHaveLength(2);
    const json = result.find((a: any) => a.id === 'json_tpl') as any;
    expect(json.source.toDir).toBe('.kiro');
    expect(json.source.outFile).toBe('config.json');
    const md = result.find((a: any) => a.id === 'md_tpl') as any;
    expect(md.source.toDir).toBe('.claude');
    expect(md.source.outFile).toBe('README.md');
  });
});
