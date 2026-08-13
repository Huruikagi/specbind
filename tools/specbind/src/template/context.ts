import { resolveKiroDir, type KiroDirOptions } from '../resolvers/kiroDir.js';
import { resolveAgentLayout, type AgentLayout, type AgentType, type CCSddConfig } from '../resolvers/agentLayout.js';
import type { SupportedLanguage } from '../constants/languages.js';

export interface BuildTemplateContextOptions {
  agent: AgentType;
  lang: SupportedLanguage;
  kiroDir?: KiroDirOptions;
  config?: CCSddConfig;
}

export type TemplateContext = {
  LANG_CODE: string;
  DEV_GUIDELINES: string;
  KIRO_DIR: string;
  AGENT_DIR: string;
  AGENT_DOC: string;
  AGENT_COMMANDS_DIR: string;
};

const guidelinesMap: Record<SupportedLanguage, string> = {
  en: '- Think in English, generate responses in English. All Markdown content written to project files (e.g., requirements.md, design.md, tasks.md, research.md, validation reports) MUST be written in the target language configured for this specification (see spec.json.language).',
  ja: '- Think in English, generate responses in Japanese. All Markdown content written to project files (e.g., requirements.md, design.md, tasks.md, research.md, validation reports) MUST be written in the target language configured for this specification (see spec.json.language).',
};

export const getDevGuidelines = (lang: SupportedLanguage): string => guidelinesMap[lang];

export const createTemplateContext = (
  lang: SupportedLanguage,
  kiroDir: string,
  layout: AgentLayout,
): TemplateContext => ({
  LANG_CODE: lang,
  DEV_GUIDELINES: getDevGuidelines(lang),
  KIRO_DIR: kiroDir,
  AGENT_DIR: layout.agentDir,
  AGENT_DOC: layout.docFile,
  AGENT_COMMANDS_DIR: layout.commandsDir,
});

export const buildTemplateContext = (opts: BuildTemplateContextOptions): TemplateContext => {
  const kiro = resolveKiroDir(opts.kiroDir ?? {});
  const layout = resolveAgentLayout(opts.agent, opts.config);
  return createTemplateContext(opts.lang, kiro, layout);
};
