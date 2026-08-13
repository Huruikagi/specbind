import { describe, it, expect } from 'vitest';
import { resolveAgentLayout } from '../src/resolvers/agentLayout';

describe('resolveAgentLayout', () => {
  it('returns Claude Code Skills defaults', () => {
    expect(resolveAgentLayout('claude-code-skills')).toEqual({
      commandsDir: '.claude/skills',
      agentDir: '.claude',
      docFile: 'CLAUDE.md',
    });
  });

  it('returns Codex Skills defaults', () => {
    expect(resolveAgentLayout('codex-skills')).toEqual({
      commandsDir: '.agents/skills',
      agentDir: '.agents',
      docFile: 'AGENTS.md',
    });
  });

  it('applies config override for commandsDir', () => {
    expect(resolveAgentLayout('claude-code-skills', {
      agentLayouts: {
        'claude-code-skills': { commandsDir: '.custom/skills' },
      },
    })).toEqual({
      commandsDir: '.custom/skills',
      agentDir: '.claude',
      docFile: 'CLAUDE.md',
    });
  });
});
