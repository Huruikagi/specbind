import { describe, it, expect } from 'vitest';
import { contextFromResolved } from '../src/template/fromResolved';
import { mergeConfigAndArgs } from '../src/cli/config';
import { parseArgs } from '../src/cli/args';

const runtimeDarwin = { platform: 'darwin' } as const;

describe('contextFromResolved', () => {
  it('creates template context from resolved config with default values', () => {
    const resolved = mergeConfigAndArgs(parseArgs([]), {}, runtimeDarwin);
    const ctx = contextFromResolved(resolved);

    expect(ctx.LANG_CODE).toBe('en');
    expect(ctx.KIRO_DIR).toBe('.kiro');
    expect(ctx.AGENT_DIR).toBe('.claude');
    expect(ctx.AGENT_DOC).toBe('CLAUDE.md');
    expect(ctx.AGENT_COMMANDS_DIR).toBe('.claude/skills');
  });

  it('creates a customized Codex context', () => {
    const args = parseArgs(['--lang', 'ja', '--kiro-dir', 'docs/kiro', '--agent', 'codex-skills']);
    const config = {
      agentLayouts: {
        'codex-skills': { commandsDir: '.custom/skills' },
      },
    };
    const ctx = contextFromResolved(mergeConfigAndArgs(args, config, runtimeDarwin));

    expect(ctx.LANG_CODE).toBe('ja');
    expect(ctx.KIRO_DIR).toBe('docs/kiro');
    expect(ctx.AGENT_DIR).toBe('.agents');
    expect(ctx.AGENT_DOC).toBe('AGENTS.md');
    expect(ctx.AGENT_COMMANDS_DIR).toBe('.custom/skills');
    expect(ctx.DEV_GUIDELINES).toContain('generate responses in Japanese');
  });

  it('preserves all layout properties correctly', () => {
    const args = parseArgs(['--kiro-dir', 'custom-kiro']);
    const config = {
      agentLayouts: {
        'claude-code-skills': {
          commandsDir: '.custom/skills/path',
          agentDir: '.custom-agent',
          docFile: 'CUSTOM-DOC.md',
        },
      },
    };
    const ctx = contextFromResolved(mergeConfigAndArgs(args, config, runtimeDarwin));

    expect(ctx.KIRO_DIR).toBe('custom-kiro');
    expect(ctx.AGENT_DIR).toBe('.custom-agent');
    expect(ctx.AGENT_DOC).toBe('CUSTOM-DOC.md');
    expect(ctx.AGENT_COMMANDS_DIR).toBe('.custom/skills/path');
  });
});
