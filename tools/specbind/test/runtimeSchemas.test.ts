import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const schemaKinds = ['spec', 'tasks'] as const;

interface RuntimeSchema {
  $schema?: unknown;
  type?: unknown;
  required?: unknown;
  properties?: {
    schema_version?: unknown;
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
});
