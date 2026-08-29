# 0157: Add command-specific JSON output for Spec status

Status: Accepted

## Context

[Decision 0074](./0074-defer-json-cli-output.md) defers a general JSON output
mode until after v1 so the initial release does not need a common envelope,
response schemas, and duplicate rendering coverage for every command. That
restraint remains useful.

SpecBind can nevertheless provide a small integration foothold for scripts,
editor extensions, dashboards, and other locally invoked tools. `spec status`
is the representative command because it already composes lifecycle,
freshness, coverage, task progress, and diagnostics through one authoritative
read model. Without a typed projection, a surrounding tool must parse the
human-oriented text layout or reconstruct that model from artifacts.

The initial surrounding tools are expected to evolve with SpecBind. A general
response protocol, standalone response schemas, and speculative abstractions
for integrations that do not yet exist would add more contract than the
demonstrated use case requires.

## Decision

### Command surface

V1 adds one command-specific option:

```text
specbind spec status <spec> --json
```

- `--json` applies only to `spec status`. It is not a global option and does
  not imply JSON support for another command.
- Omitting `--json` preserves the existing Decision 0067 text output
  byte-for-byte.
- The command resolves the same `SpecStatusModel` for both renderings. JSON is
  an alternate projection, not a separate status calculation.

### Minimal response

A successful invocation writes one UTF-8 JSON document followed by a newline
to stdout:

```json
{"status":"ok","code":"SPEC_STATUS_REPORTED","data":{}}
```

The `data` object reports the named Spec, declared state, milestone, health,
four gate freshness values, next action, expected Requirements or Design work,
Contract Review status, delegated gates, task progress and blockers,
Requirement coverage, and diagnostics. JSON field names use `camelCase`;
established lifecycle values and stable codes remain the same lowercase or
uppercase ASCII machine tokens used by the text projection.

A command failure requested with `--json` writes this minimal shape to stdout:

```json
{"status":"error","code":"SPEC_STATUS_FAILED","message":"Cannot report status for spec checkout.","details":[]}
```

- `status` is exactly `ok` or `error` for this read-only command.
- `code` retains the existing stable result or diagnostic code.
- `message` and `details` appear on failure and remain explanatory text. A
  consumer branches on `status`, `code`, typed `data`, and process exit status
  rather than parsing those strings.
- Command-level JSON results keep stderr empty. Success exits zero and failure
  remains nonzero. Failures outside command execution, such as an inability to
  write stdout, are not JSON response guarantees.

### Compatibility boundary

This command-specific response follows the executable compatibility policy in
[Decision 0144](./0144-major-version-compatibility-and-migration.md). It has no
independent `schemaVersion` and no published JSON Schema. Additive fields may
be introduced within the executable major version; removing or repurposing an
established field is breaking. Consumers must ignore unknown fields.

Decision 0074 continues to defer a global `--json` or `--format` option, a
common cross-command envelope, response-schema distribution, and JSON support
for every other command. A later concrete integration may extend support one
command at a time or justify a broader protocol.

## Consequences

- A surrounding tool can consume the richest per-Spec read model without text
  parsing or direct artifact interpretation.
- V1 gains one exercised JSON contract without refactoring every CLI result.
- Text-first output remains the default user and Agent surface.
- Future JSON work is driven by a concrete consumer rather than an assumed
  ecosystem design.

## Implementation status

Implemented. `spec status --json` serializes the existing composed read model,
routes success and command failure as a single stdout JSON document, preserves
the existing exit status, and leaves default text output unchanged.
