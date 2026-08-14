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
