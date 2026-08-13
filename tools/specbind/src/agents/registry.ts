export interface AgentLayoutDefaults {
  commandsDir: string;
  agentDir: string;
  docFile: string;
}

export interface AgentCommandHints {
  spec: string;
  steering: string;
  steeringCustom: string;
}

export interface AgentCompletionGuide {
  prependSteps?: string[];
  appendSteps?: string[];
}

export interface AgentDefinition {
  label: string;
  description: string;
  aliasFlags: string[];
  recommendedModels?: string[];
  layout: AgentLayoutDefaults;
  commands: AgentCommandHints;
  manifestId?: string;
  completionGuide?: AgentCompletionGuide;
  templateFallbacks?: Record<string, string>;
}

export const agentDefinitions = {
  'claude-code-skills': {
    label: 'Claude Code Skills',
    description:
      'Installs kiro skills in `.claude/skills/kiro-*/`, shared settings in `{{KIRO_DIR}}/settings/`, and a CLAUDE.md quickstart.',
    aliasFlags: ['--claude-code-skills', '--claude-skills'],
    recommendedModels: ['Planning / review: Claude Opus 4.6 or newer', 'Implementation: Claude Sonnet 4.6 or newer'],
    layout: {
      commandsDir: '.claude/skills',
      agentDir: '.claude',
      docFile: 'CLAUDE.md',
    },
    commands: {
      spec: '`/kiro-spec-init <what-to-build>`',
      steering: '`/kiro-steering`',
      steeringCustom: '`/kiro-steering-custom <what-to-create-custom-steering-document>`',
    },
    completionGuide: {
      prependSteps: [
        'If you are not sure whether the work should become one spec, many specs, or no spec at all, start with `/kiro-discovery <idea>`.',
      ],
      appendSteps: [
        'Use `/kiro-spec-quick <what-to-build> [--auto]` only when you intentionally want the fast path for a single spec.',
      ],
    },
    templateFallbacks: {
      'CLAUDE.md': '../../CLAUDE.md',
    },
    manifestId: 'claude-code-skills',
  },
  'codex-skills': {
    label: 'Codex Skills',
    description:
      'Installs kiro skills in `.agents/skills/kiro-*/`, shared settings in `{{KIRO_DIR}}/settings/`, and an AGENTS.md quickstart.',
    aliasFlags: ['--codex-skills'],
    recommendedModels: ['Planning / review: gpt-5.4 high or xhigh', 'Implementation: gpt-5.4'],
    layout: {
      commandsDir: '.agents/skills',
      agentDir: '.agents',
      docFile: 'AGENTS.md',
    },
    commands: {
      spec: '`$kiro-spec-init <what-to-build>`',
      steering: '`$kiro-steering`',
      steeringCustom: '`$kiro-steering-custom <what-to-create-custom-steering-document>`',
    },
    completionGuide: {
      prependSteps: [
        'If you are not sure whether the work should become one spec, many specs, or no spec at all, start with `$kiro-discovery <idea>`.',
      ],
      appendSteps: [
        'Use `$kiro-spec-quick <what-to-build> [--auto]` only when you intentionally want the fast path for a single spec.',
      ],
    },
    manifestId: 'codex-skills',
  },
} as const satisfies Record<string, AgentDefinition>;

export type AgentType = keyof typeof agentDefinitions;

export const getAgentDefinition = (agent: AgentType): AgentDefinition => {
  const definition = agentDefinitions[agent];
  if (!definition) {
    throw new Error(`Unknown agent: ${agent as string}`);
  }
  return definition as AgentDefinition;
};

export const agentList = Object.keys(agentDefinitions) as AgentType[];
