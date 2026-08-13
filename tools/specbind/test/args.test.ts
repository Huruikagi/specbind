import { describe, it, expect } from 'vitest';
import { parseArgs, type ParsedArgs } from '../src/cli/args';

describe('parseArgs', () => {
  it('parses basic flags with explicit values', () => {
    const args = parseArgs([
      '--agent', 'claude-code-skills',
      '--lang', 'ja',
      '--os', 'auto',
      '--overwrite', 'prompt',
      '--kiro-dir', '.kiro',
    ]);
    const expected: ParsedArgs = {
      agent: 'claude-code-skills',
      lang: 'ja',
      os: 'auto',
      overwrite: 'prompt',
      kiroDir: '.kiro',
    };
    expect(args).toEqual(expected);
  });

  it('supports boolean flags and short aliases', () => {
    const args = parseArgs(['--dry-run', '-y']);
    expect(args.dryRun).toBe(true);
    expect(args.yes).toBe(true);
  });

  it('parses additional languages', () => {
    expect(parseArgs(['--lang', 'es']).lang).toBe('es');
    expect(parseArgs(['--lang', 'ko']).lang).toBe('ko');
  });

  it('parses backup with and without value', () => {
    expect(parseArgs(['--backup']).backup).toBe(true);
    expect(parseArgs(['--backup', '.specbind.backup']).backup).toBe('.specbind.backup');
    expect(parseArgs(['--backup=.specbind.backup/custom']).backup).toBe('.specbind.backup/custom');
  });

  it('supports agent alias flags and detects conflicts', () => {
    expect(parseArgs(['--claude-skills']).agent).toBe('claude-code-skills');
    expect(parseArgs(['--claude-code-skills']).agent).toBe('claude-code-skills');
    expect(parseArgs(['--codex-skills']).agent).toBe('codex-skills');

    expect(() => parseArgs(['--agent', 'codex-skills', '--claude-skills'])).toThrowError(/agent.*conflict/i);
    expect(() => parseArgs(['--claude-skills', '--codex-skills'])).toThrowError(/agent.*conflict/i);
    expect(() => parseArgs(['--gemini-skills'])).toThrowError(/unknown flag/i);
  });

  it('validates enum values for os/lang/overwrite/agent', () => {
    expect(() => parseArgs(['--os', 'macos'])).toThrowError(/os.*invalid/i);
    expect(() => parseArgs(['--lang', 'jp'])).toThrowError(/lang.*invalid/i);
    expect(() => parseArgs(['--overwrite', 'replace'])).toThrowError(/overwrite.*invalid/i);
    expect(() => parseArgs(['--agent', 'unknown'])).toThrowError(/agent.*invalid/i);
  });

  it('rejects unknown flags', () => {
    expect(() => parseArgs(['--unknown-flag'])).toThrowError(/unknown flag/i);
  });
});
