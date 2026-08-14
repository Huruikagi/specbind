import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const schemaKinds = ['spec', 'tasks'] as const;

interface RuntimeSchema {
  $schema?: unknown;
  $defs?: Record<string, unknown>;
  type?: unknown;
  required?: unknown;
  properties?: {
    schema_version?: unknown;
    language?: unknown;
    active_change?: unknown;
    plan?: unknown;
    execution?: unknown;
  };
  additionalProperties?: unknown;
}

describe('runtime schema scaffolds', () => {
  for (const kind of schemaKinds) {
    it(`${kind} v1 declares the accepted strict version envelope`, async () => {
      const filePath = join(process.cwd(), 'schemas', kind, 'v1.schema.json');
      const schema = JSON.parse(await readFile(filePath, 'utf8')) as RuntimeSchema;

      expect(schema.$schema).toBe('https://json-schema.org/draft/2020-12/schema');
      expect(schema.type).toBe('object');
      expect(schema.required).toContain('schema_version');
      expect(schema.properties.schema_version).toEqual({ const: 1 });
      expect(schema.additionalProperties).toBe(false);
    });
  }

  it('includes runtime schemas in the TypeScript package surface', async () => {
    const packageJson = JSON.parse(
      await readFile(join(process.cwd(), 'package.json'), 'utf8'),
    ) as { files?: string[] };

    expect(packageJson.files).toContain('schemas');
  });

  it('defines the accepted tagged lowercase SHA-256 fingerprint value', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;

    expect(schema.$defs?.fingerprint).toEqual({
      type: 'string',
      pattern: '^sha256:[0-9a-f]{64}$',
    });
  });

  it('defines canonical UUID v7 milestone IDs', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;

    expect(schema.$defs?.milestoneId).toEqual({
      type: 'string',
      description: 'Canonical lowercase hyphenated UUID v7 milestone identity.',
      pattern:
        '^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$',
    });
  });

  it('wires the minimal strict spec root and active change', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const activeChange = schema.$defs?.activeChange as {
      required?: string[];
      properties?: Record<string, unknown>;
      additionalProperties?: unknown;
    };

    expect(schema.required).toEqual(['schema_version', 'language', 'active_change']);
    expect(schema.properties).toEqual({
      schema_version: { const: 1 },
      language: { $ref: '#/$defs/specLanguage' },
      active_change: {
        oneOf: [{ type: 'null' }, { $ref: '#/$defs/activeChange' }],
      },
    });
    expect(schema.$defs?.specLanguage).toEqual({ enum: ['en', 'ja'] });
    expect(activeChange.required).toEqual([
      'milestone_id',
      'state',
      'requirement_ids',
    ]);
    expect(activeChange.properties).toHaveProperty('gate_evidence', {
      $ref: '#/$defs/gateEvidence',
    });
    expect(activeChange.additionalProperties).toBe(false);
    expect(JSON.stringify(schema)).not.toContain('feature_name');
    expect(JSON.stringify(schema)).not.toContain('created_at');
    expect(JSON.stringify(schema)).not.toContain('updated_at');
    expect(JSON.stringify(schema)).not.toContain('ready_for_implementation');
  });

  it('defines timezone-qualified RFC 3339 gate timestamps', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const requirementsEvidence = schema.$defs?.requirementsGateEvidence as {
      properties?: Record<string, unknown>;
    };

    expect(schema.$defs?.passedAt).toEqual({
      type: 'string',
      description: 'RFC 3339 date-time with an explicit UTC or numeric-offset timezone.',
      format: 'date-time',
      pattern: '(?:[Zz]|[+-][0-9]{2}:[0-9]{2})$',
    });
    expect(requirementsEvidence.properties?.passed_at).toEqual({
      $ref: '#/$defs/passedAt',
    });
  });

  it('defines a project-scoped full Git implementation revision', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;

    expect(schema.$defs?.implementationRevision).toEqual({
      type: 'string',
      description:
        'Full lowercase Git commit object ID; semantic validation requires the form used by the current project repository.',
      pattern: '^(?:[0-9a-f]{40}|[0-9a-f]{64})$',
    });
  });

  it('defines concise successful mechanical completion checks', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const check = schema.$defs?.mechanicalCheck as {
      required?: string[];
      properties?: Record<string, unknown>;
      additionalProperties?: unknown;
    };

    expect(check.required).toEqual(['kind', 'command', 'exit_code']);
    expect(check.properties?.kind).toEqual({
      enum: ['test', 'build', 'smoke', 'lint', 'typecheck', 'custom'],
    });
    expect(check.properties?.exit_code).toEqual({ const: 0 });
    expect(check.additionalProperties).toBe(false);
    expect(schema.$defs?.mechanicalCheckList).toMatchObject({
      type: 'array',
      minItems: 1,
    });
  });

  it('defines strict minimal completion gate evidence', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const evidence = schema.$defs?.completionGateEvidence as {
      required?: string[];
      properties?: Record<string, unknown>;
      additionalProperties?: unknown;
    };

    expect(evidence.required).toEqual([
      'passed_at',
      'implementation_revision',
      'mechanical_checks',
    ]);
    expect(evidence.properties).toEqual({
      passed_at: { $ref: '#/$defs/passedAt' },
      implementation_revision: { $ref: '#/$defs/implementationRevision' },
      mechanical_checks: { $ref: '#/$defs/mechanicalCheckList' },
    });
    expect(evidence.additionalProperties).toBe(false);
  });

  it('defines readable requirements gate evidence without a brief fingerprint', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const evidence = schema.$defs?.requirementsGateEvidence as {
      required?: string[];
      properties?: Record<string, unknown>;
    };

    expect(evidence.required).toContain('approved_requirement_ids');
    expect(evidence.properties).toHaveProperty('approved_requirement_ids');
    expect(evidence.properties).toHaveProperty('input_revisions');
    expect(JSON.stringify(evidence)).toContain('requirements.md');
    expect(JSON.stringify(evidence)).not.toContain('brief.md');
    expect(schema.$defs?.requirementIdList).toMatchObject({
      type: 'array',
      minItems: 1,
      uniqueItems: true,
    });
  });

  it('defines design gate evidence over required design and contract files', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const evidence = schema.$defs?.designGateEvidence as {
      required?: string[];
      properties?: {
        input_revisions?: {
          required?: string[];
          properties?: Record<string, unknown>;
          additionalProperties?: unknown;
        };
      };
      additionalProperties?: unknown;
    };

    expect(evidence.required).toEqual(['passed_at', 'approval_mode', 'input_revisions']);
    expect(evidence.properties?.input_revisions?.required).toEqual([
      'design.md',
      'contract.md',
    ]);
    expect(evidence.properties?.input_revisions?.properties).toEqual({
      'design.md': { $ref: '#/$defs/fingerprint' },
      'contract.md': { $ref: '#/$defs/fingerprint' },
    });
    expect(evidence.properties?.input_revisions?.additionalProperties).toBe(false);
    expect(evidence.additionalProperties).toBe(false);
    expect(JSON.stringify(evidence)).not.toContain('requirements.md');
  });

  it('defines minimal tasks gate evidence over only the normalized plan', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const evidence = schema.$defs?.tasksGateEvidence as {
      required?: string[];
      properties?: {
        input_revisions?: {
          required?: string[];
          properties?: Record<string, unknown>;
          additionalProperties?: unknown;
        };
      };
      additionalProperties?: unknown;
    };

    expect(evidence.required).toEqual(['passed_at', 'approval_mode', 'input_revisions']);
    expect(evidence.properties?.input_revisions?.required).toEqual([
      'tasks.yaml#plan',
    ]);
    expect(evidence.properties?.input_revisions?.properties).toEqual({
      'tasks.yaml#plan': { $ref: '#/$defs/fingerprint' },
    });
    expect(evidence.properties?.input_revisions?.additionalProperties).toBe(false);
    expect(evidence.additionalProperties).toBe(false);
    expect(JSON.stringify(evidence)).not.toContain('execution');
    expect(JSON.stringify(evidence)).not.toContain('requirement_ids');
  });

  it('defines a non-empty sparse gate evidence container', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'spec', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;

    expect(schema.$defs?.gateEvidence).toEqual({
      type: 'object',
      minProperties: 1,
      properties: {
        requirements: { $ref: '#/$defs/requirementsGateEvidence' },
        design: { $ref: '#/$defs/designGateEvidence' },
        tasks: { $ref: '#/$defs/tasksGateEvidence' },
        completion: { $ref: '#/$defs/completionGateEvidence' },
      },
      additionalProperties: false,
    });
  });

  it('defines sparse scheduling fields for executable tasks', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'tasks', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const task = schema.$defs?.executableTask as {
      required?: string[];
      properties?: Record<string, unknown>;
    };

    expect(task.required).not.toContain('parallel');
    expect(task.required).not.toContain('depends_on');
    expect(task.properties).toHaveProperty('parallel', { const: true });
    expect(task.properties).toHaveProperty('depends_on');
    expect(JSON.stringify(task)).not.toContain('optional');
  });

  it('defines spec-local one- or two-level positional numeric task references', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'tasks', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;

    expect(schema.$defs?.taskReference).toEqual({
      type: 'string',
      description:
        'Unqualified positional Task ID resolved only within this tasks.yaml artifact.',
      pattern: '^[1-9][0-9]*(?:\\.[1-9][0-9]*)?$',
    });
  });

  it('defines explicit completion criteria as a non-empty list when present', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'tasks', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;

    expect(schema.$defs?.completionCriteria).toEqual({
      $ref: '#/$defs/nonEmptyStringList',
    });
    expect(schema.$defs?.nonEmptyStringList).toMatchObject({
      description:
        'Ordered non-empty string sequence; array order is preserved in the task-plan fingerprint.',
    });
    expect(schema.$defs?.uniqueNonEmptyStringList).toMatchObject({
      description:
        'Non-empty string set; values are sorted during task-plan fingerprint normalization.',
    });
  });

  it('wires the accepted task plan into the root schema', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'tasks', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const group = schema.$defs?.taskGroup as {
      properties?: {
        tasks?: {
          minItems?: number;
        };
      };
    };

    expect(schema.required).toEqual(['schema_version', 'plan']);
    expect(schema.properties.plan).toEqual({ $ref: '#/$defs/taskPlan' });
    expect(group.properties?.tasks?.minItems).toBe(2);
  });

  it('defines sparse durable task execution states', async () => {
    const schema = JSON.parse(
      await readFile(join(process.cwd(), 'schemas', 'tasks', 'v1.schema.json'), 'utf8'),
    ) as RuntimeSchema;
    const executionText = JSON.stringify(schema.$defs?.taskExecutionState);

    expect(schema.properties.execution).toEqual({ $ref: '#/$defs/taskExecution' });
    expect(executionText).toContain('completedTaskState');
    expect(executionText).toContain('blockedTaskState');
    expect(executionText).not.toContain('pending');
    expect(executionText).not.toContain('in_progress');
    expect(executionText).not.toContain('skipped');
    expect(schema.$defs?.completedTaskState).toEqual({
      type: 'object',
      required: ['status'],
      properties: {
        status: { const: 'completed' },
      },
      additionalProperties: false,
    });
    expect(JSON.stringify(schema.$defs?.taskExecution)).not.toContain(
      'implementation_notes',
    );
  });

});
