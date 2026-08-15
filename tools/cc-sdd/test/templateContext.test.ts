import { describe, it, expect } from 'vitest';
import { buildTemplateContext } from '../src/template/context';
import type { CCSddConfig } from '../src/resolvers/agentLayout';

describe('buildTemplateContext', () => {
  it('includes LANG_CODE and KIRO_DIR (default)', () => {
    const ctx = buildTemplateContext({ agent: 'claude-code-skills', lang: 'ja' });
    expect(ctx.LANG_CODE).toBe('ja');
    expect(ctx.KIRO_DIR).toBe('.kiro');
    expect(ctx.DEV_GUIDELINES).toBe(
      '- Think in English, generate responses in Japanese. All Markdown content written to project files (e.g., requirements.md, design.md, tasks.md, research.md, validation reports) MUST be written in the target language configured for this specification (see spec.json.language).',
    );
  });

  it('uses kiro-dir flag when provided', () => {
    const ctx = buildTemplateContext({ agent: 'claude-code-skills', lang: 'en', kiroDir: { flag: 'docs/kiro' } });
    expect(ctx.KIRO_DIR).toBe('docs/kiro');
  });

  it('includes Claude Code Skills layout variables', () => {
    const ctx = buildTemplateContext({ agent: 'claude-code-skills', lang: 'en' });
    expect(ctx.AGENT_DIR).toBe('.claude');
    expect(ctx.AGENT_DOC).toBe('CLAUDE.md');
    expect(ctx.AGENT_COMMANDS_DIR).toBe('.claude/skills');
  });

  it('includes Codex Skills layout variables', () => {
    const ctx = buildTemplateContext({ agent: 'codex-skills', lang: 'en' });
    expect(ctx.AGENT_DIR).toBe('.agents');
    expect(ctx.AGENT_DOC).toBe('AGENTS.md');
    expect(ctx.AGENT_COMMANDS_DIR).toBe('.agents/skills');
  });

  it('respects agentLayouts override', () => {
    const config: CCSddConfig = {
      agentLayouts: {
        'claude-code-skills': { commandsDir: '.custom/skills' },
      },
    };
    const ctx = buildTemplateContext({ agent: 'claude-code-skills', lang: 'en', config });
    expect(ctx.AGENT_COMMANDS_DIR).toBe('.custom/skills');
    expect(ctx.AGENT_DIR).toBe('.claude');
    expect(ctx.AGENT_DOC).toBe('CLAUDE.md');
  });

  it('provides guidelines for all supported languages', () => {
    const langs = ['en', 'ja'] as const;
    for (const lang of langs) {
      const ctx = buildTemplateContext({ agent: 'claude-code-skills', lang });
      expect(ctx.DEV_GUIDELINES.length).toBeGreaterThan(0);
    }
  });
});
